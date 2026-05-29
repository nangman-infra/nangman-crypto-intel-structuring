use super::types::{EvidenceSnippet, ModelStructuringResponse};
use crate::error::{AppError, AppResult};
use crate::models::output::{ConfidenceBand, TerminalDecision};
use std::collections::BTreeMap;

impl ModelStructuringResponse {
    pub fn hydrate_evidence_sentences(
        &mut self,
        evidence_pack: &[EvidenceSnippet],
    ) -> AppResult<()> {
        if self.evidence_ids.is_empty() {
            return Ok(());
        }
        let by_id = evidence_pack
            .iter()
            .map(|snippet| (snippet.id.as_str(), snippet.text.as_str()))
            .collect::<BTreeMap<_, _>>();
        let mut sentences = Vec::new();
        for evidence_id in &self.evidence_ids {
            let Some(sentence) = by_id.get(evidence_id.as_str()) else {
                return Err(AppError::validation(format!(
                    "model returned unknown evidence_id {evidence_id}"
                )));
            };
            sentences.push((*sentence).to_owned());
        }
        self.evidence_sentences = sentences;
        Ok(())
    }

    pub fn validate_evidence_gate(&self) -> AppResult<()> {
        validate_confidence_bands(&self.symbol_confidence_band, &self.confidence_band)?;
        validate_evidence_requirements(
            &self.confidence_band,
            &self.terminal_decision,
            self.evidence_sentences.is_empty(),
        )?;
        validate_score_range(self.confidence_score, "model confidence_score must be 0..1")?;
        validate_score_range(self.novelty_score, "model novelty_score must be 0..1")?;
        validate_text_fields(self)?;
        for symbol in &self.normalized_symbols {
            validate_symbol(symbol)?;
        }
        Ok(())
    }
}

fn validate_confidence_bands(
    symbol_band: &ConfidenceBand,
    confidence_band: &ConfidenceBand,
) -> AppResult<()> {
    if !matches!(
        symbol_band,
        ConfidenceBand::Weak | ConfidenceBand::Moderate | ConfidenceBand::Strong
    ) {
        return Err(AppError::validation(
            "model symbol_confidence_band must be weak/moderate/strong",
        ));
    }
    if !matches!(
        confidence_band,
        ConfidenceBand::Low | ConfidenceBand::Medium | ConfidenceBand::High
    ) {
        return Err(AppError::validation(
            "model confidence_band must be low/medium/high",
        ));
    }
    Ok(())
}

fn validate_evidence_requirements(
    confidence_band: &ConfidenceBand,
    terminal_decision: &TerminalDecision,
    evidence_is_empty: bool,
) -> AppResult<()> {
    if matches!(
        confidence_band,
        ConfidenceBand::High | ConfidenceBand::Strong
    ) && evidence_is_empty
    {
        return Err(AppError::validation(
            "model high confidence without evidence",
        ));
    }
    if matches!(
        terminal_decision,
        TerminalDecision::HighConfidenceStructured | TerminalDecision::Conflicted
    ) && evidence_is_empty
    {
        return Err(AppError::validation(
            "model terminal decision requires evidence",
        ));
    }
    if matches!(terminal_decision, TerminalDecision::QuarantineOnly) {
        return Err(AppError::validation("model must not emit quarantine_only"));
    }
    Ok(())
}

fn validate_score_range(score: f64, message: &'static str) -> AppResult<()> {
    if !(0.0..=1.0).contains(&score) {
        return Err(AppError::validation(message));
    }
    Ok(())
}

fn validate_text_fields(response: &ModelStructuringResponse) -> AppResult<()> {
    for field in [
        &response.topic_summary,
        &response.stance_summary,
        &response.risk_summary,
        &response.regime_hint,
        &response.scenario_hint,
    ] {
        validate_text_field(field)?;
    }
    Ok(())
}

fn validate_text_field(field: &str) -> AppResult<()> {
    if field.trim().is_empty() {
        return Err(AppError::validation("model text fields must not be empty"));
    }
    if field.chars().count() > 512 {
        return Err(AppError::validation(
            "model text fields must be <=512 chars",
        ));
    }
    Ok(())
}

fn validate_symbol(symbol: &str) -> AppResult<()> {
    let trimmed = symbol.trim();
    if trimmed.is_empty() {
        return Err(AppError::validation(
            "model normalized symbol must not be empty",
        ));
    }
    if trimmed.len() > 16 {
        return Err(AppError::validation(
            "model normalized symbol must be <=16 chars",
        ));
    }
    if trimmed != trimmed.to_ascii_uppercase()
        || !trimmed
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
    {
        return Err(AppError::validation(
            "model normalized symbol must be uppercase ASCII alphanumeric",
        ));
    }
    Ok(())
}
