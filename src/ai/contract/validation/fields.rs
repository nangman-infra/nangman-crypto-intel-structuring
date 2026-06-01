use super::super::types::ModelStructuringResponse;
use crate::error::{AppError, AppResult};
use crate::models::output::ConfidenceBand;

pub(super) fn validate_confidence_bands(
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

pub(super) fn validate_score_range(score: f64, message: &'static str) -> AppResult<()> {
    if !(0.0..=1.0).contains(&score) {
        return Err(AppError::validation(message));
    }
    Ok(())
}

pub(super) fn validate_text_fields(response: &ModelStructuringResponse) -> AppResult<()> {
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

pub(super) fn validate_symbol(symbol: &str) -> AppResult<()> {
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
