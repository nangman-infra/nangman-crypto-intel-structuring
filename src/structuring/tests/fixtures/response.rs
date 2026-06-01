use crate::ai::contract::ModelStructuringResponse;
use crate::models::output::{
    ConfidenceBand, ContradictionFlag, EventType, RelevanceDecayHint, TerminalDecision,
};

pub(in crate::structuring::tests) fn response(
    confidence: f64,
    terminal_decision: TerminalDecision,
) -> ModelStructuringResponse {
    response_with_evidence(
        confidence,
        terminal_decision,
        "Protocol exploit investigation expands after the team confirmed the incident",
    )
}

pub(in crate::structuring::tests) fn response_with_evidence(
    confidence: f64,
    terminal_decision: TerminalDecision,
    evidence_sentence: &str,
) -> ModelStructuringResponse {
    ModelStructuringResponse {
        event_type: EventType::Incident,
        normalized_symbols: vec!["ABC".to_owned()],
        symbol_confidence_band: ConfidenceBand::Strong,
        topic_summary: "Incident confirmed".to_owned(),
        stance_summary: "Evidence supports an incident classification".to_owned(),
        risk_summary: "Operational risk is present".to_owned(),
        regime_hint: "event_driven".to_owned(),
        scenario_hint: "watch_only".to_owned(),
        confidence_band: if confidence >= 0.8 {
            ConfidenceBand::High
        } else {
            ConfidenceBand::Low
        },
        confidence_score: confidence,
        novelty_score: 0.8,
        relevance_decay_hint: RelevanceDecayHint::MultiDay,
        contradiction_flags: Vec::<ContradictionFlag>::new(),
        evidence_ids: Vec::new(),
        evidence_sentences: vec![evidence_sentence.to_owned()],
        terminal_decision,
    }
}
