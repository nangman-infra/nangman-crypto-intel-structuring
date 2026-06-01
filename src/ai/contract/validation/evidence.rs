use super::super::types::{EvidenceSnippet, ModelStructuringResponse};
use crate::error::{AppError, AppResult};
use crate::models::output::{ConfidenceBand, TerminalDecision};
use std::collections::BTreeMap;

pub(super) fn hydrate_evidence_sentences(
    response: &mut ModelStructuringResponse,
    evidence_pack: &[EvidenceSnippet],
) -> AppResult<()> {
    if response.evidence_ids.is_empty() {
        return Ok(());
    }
    let by_id = evidence_pack
        .iter()
        .map(|snippet| (snippet.id.as_str(), snippet.text.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut sentences = Vec::new();
    for evidence_id in &response.evidence_ids {
        let Some(sentence) = by_id.get(evidence_id.as_str()) else {
            return Err(AppError::validation(format!(
                "model returned unknown evidence_id {evidence_id}"
            )));
        };
        sentences.push((*sentence).to_owned());
    }
    response.evidence_sentences = sentences;
    Ok(())
}

pub(super) fn validate_evidence_requirements(
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
