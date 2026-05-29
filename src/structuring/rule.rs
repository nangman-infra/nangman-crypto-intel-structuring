mod classify;
mod evidence;
mod scoring;
mod summary;
mod symbols;
#[cfg(test)]
mod tests;

use crate::models::output::{
    ConfidenceBand, ContradictionFlag, EventType, RelevanceDecayHint, TerminalDecision,
};
use crate::models::raw::RawIntelEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct RuleAssessment {
    pub event_type: EventType,
    pub normalized_symbols: Vec<String>,
    pub symbol_confidence_band: ConfidenceBand,
    pub confidence_score: f64,
    pub confidence_band: ConfidenceBand,
    pub evidence_sentences: Vec<String>,
    pub contradiction_flags: Vec<ContradictionFlag>,
    pub terminal_decision: TerminalDecision,
    pub high_risk: bool,
    pub topic_summary: String,
    pub stance_summary: String,
    pub risk_summary: String,
    pub regime_hint: String,
    pub scenario_hint: String,
    pub relevance_decay_hint: RelevanceDecayHint,
    pub novelty_score: f64,
}

pub fn assess(event: &RawIntelEvent) -> RuleAssessment {
    let text = format!("{} {}", event.title, event.body).to_ascii_lowercase();
    let event_type = classify::classify_event_type(&text);
    let high_risk = matches!(
        event_type,
        EventType::Delisting
            | EventType::Incident
            | EventType::Regulatory
            | EventType::DepositWithdrawal
    );
    let normalized_symbols = symbols::normalize_symbols(&event.symbol_candidates);
    let symbol_confidence_band = if normalized_symbols.is_empty() {
        ConfidenceBand::Weak
    } else if event.source_category.contains("project") {
        ConfidenceBand::Strong
    } else {
        ConfidenceBand::Moderate
    };
    let evidence_sentences = evidence::evidence_candidates(event, &text);
    let contradiction_flags = evidence::contradiction_flags(event, &text, &evidence_sentences);
    let confidence_score = scoring::rule_confidence(
        &event_type,
        &symbol_confidence_band,
        &evidence_sentences,
        &contradiction_flags,
    );
    let confidence_band = scoring::confidence_band(confidence_score);
    let terminal_decision = scoring::terminal_decision(
        confidence_score,
        normalized_symbols.is_empty(),
        !contradiction_flags.is_empty(),
        high_risk,
    );

    RuleAssessment {
        topic_summary: summary::topic_summary(event, &event_type),
        stance_summary: summary::stance_summary(&event_type),
        risk_summary: summary::risk_summary(&event_type),
        regime_hint: summary::regime_hint(&event_type).to_owned(),
        scenario_hint: summary::scenario_hint(&event_type).to_owned(),
        relevance_decay_hint: summary::relevance_decay_hint(&event_type),
        novelty_score: summary::novelty_score(&event_type, event),
        event_type,
        normalized_symbols,
        symbol_confidence_band,
        confidence_score,
        confidence_band,
        evidence_sentences,
        contradiction_flags,
        terminal_decision,
        high_risk,
    }
}
