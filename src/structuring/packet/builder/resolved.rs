use crate::models::output::{
    ConfidenceBand, ContradictionFlag, EventType, RelevanceDecayHint, TerminalDecision,
};
use crate::structuring::router::StructuringDecision;

pub(super) struct ResolvedPacketFields {
    pub(super) normalized_symbols: Vec<String>,
    pub(super) symbol_confidence_band: ConfidenceBand,
    pub(super) event_type: EventType,
    pub(super) topic_summary: String,
    pub(super) stance_summary: String,
    pub(super) risk_summary: String,
    pub(super) regime_hint: String,
    pub(super) scenario_hint: String,
    pub(super) confidence_band: ConfidenceBand,
    pub(super) novelty_score: f64,
    pub(super) relevance_decay_hint: RelevanceDecayHint,
    pub(super) contradiction_flags: Vec<ContradictionFlag>,
    pub(super) terminal_decision: TerminalDecision,
    pub(super) evidence_sentences: Vec<String>,
}

impl ResolvedPacketFields {
    pub(super) fn from_decision(decision: &StructuringDecision) -> Self {
        let model = decision.model_response.as_ref();
        Self {
            normalized_symbols: model
                .map(|value| value.normalized_symbols.clone())
                .unwrap_or_else(|| decision.rule.normalized_symbols.clone()),
            symbol_confidence_band: model
                .map(|value| value.symbol_confidence_band.clone())
                .unwrap_or_else(|| decision.rule.symbol_confidence_band.clone()),
            event_type: model
                .map(|value| value.event_type.clone())
                .unwrap_or_else(|| decision.rule.event_type.clone()),
            topic_summary: model
                .map(|value| value.topic_summary.clone())
                .unwrap_or_else(|| decision.rule.topic_summary.clone()),
            stance_summary: model
                .map(|value| value.stance_summary.clone())
                .unwrap_or_else(|| decision.rule.stance_summary.clone()),
            risk_summary: model
                .map(|value| value.risk_summary.clone())
                .unwrap_or_else(|| decision.rule.risk_summary.clone()),
            regime_hint: model
                .map(|value| value.regime_hint.clone())
                .unwrap_or_else(|| decision.rule.regime_hint.clone()),
            scenario_hint: model
                .map(|value| value.scenario_hint.clone())
                .unwrap_or_else(|| decision.rule.scenario_hint.clone()),
            confidence_band: model
                .map(|value| value.confidence_band.clone())
                .unwrap_or_else(|| decision.rule.confidence_band.clone()),
            novelty_score: model
                .map(|value| value.novelty_score)
                .unwrap_or(decision.rule.novelty_score),
            relevance_decay_hint: model
                .map(|value| value.relevance_decay_hint.clone())
                .unwrap_or_else(|| decision.rule.relevance_decay_hint.clone()),
            contradiction_flags: model
                .map(|value| value.contradiction_flags.clone())
                .unwrap_or_else(|| decision.rule.contradiction_flags.clone()),
            terminal_decision: model
                .map(|value| value.terminal_decision.clone())
                .unwrap_or_else(|| decision.rule.terminal_decision.clone()),
            evidence_sentences: model
                .map(|value| value.evidence_sentences.clone())
                .unwrap_or_else(|| decision.rule.evidence_sentences.clone()),
        }
    }
}
