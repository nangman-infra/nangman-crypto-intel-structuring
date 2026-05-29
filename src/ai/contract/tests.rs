use super::*;
use crate::models::output::{ConfidenceBand, EventType, RelevanceDecayHint, TerminalDecision};

#[test]
fn rejects_model_response_with_execution_only_terminal() {
    let mut response = response();
    response.terminal_decision = TerminalDecision::QuarantineOnly;

    assert!(response.validate_evidence_gate().is_err());
}

#[test]
fn rejects_invalid_symbol_band_for_model_contract() {
    let mut response = response();
    response.symbol_confidence_band = ConfidenceBand::High;

    assert!(response.validate_evidence_gate().is_err());
}

#[test]
fn rejects_noncanonical_symbol() {
    let mut response = response();
    response.normalized_symbols = vec!["btc-usd".to_owned()];

    assert!(response.validate_evidence_gate().is_err());
}

fn response() -> ModelStructuringResponse {
    ModelStructuringResponse {
        event_type: EventType::Incident,
        normalized_symbols: vec!["BTC".to_owned()],
        symbol_confidence_band: ConfidenceBand::Strong,
        topic_summary: "Incident confirmed".to_owned(),
        stance_summary: "Evidence supports an incident classification".to_owned(),
        risk_summary: "Operational risk is present".to_owned(),
        regime_hint: "event_driven".to_owned(),
        scenario_hint: "watch_only".to_owned(),
        confidence_band: ConfidenceBand::High,
        confidence_score: 0.9,
        novelty_score: 0.8,
        relevance_decay_hint: RelevanceDecayHint::MultiDay,
        contradiction_flags: Vec::new(),
        evidence_ids: Vec::new(),
        evidence_sentences: vec!["Evidence sentence".to_owned()],
        terminal_decision: TerminalDecision::HighConfidenceStructured,
    }
}

#[test]
fn hydrates_evidence_ids_from_pack() {
    let mut response = response();
    response.evidence_ids = vec!["E1".to_owned()];
    response.evidence_sentences.clear();
    response
        .hydrate_evidence_sentences(&[EvidenceSnippet {
            id: "E1".to_owned(),
            text: "Exact source sentence".to_owned(),
        }])
        .unwrap();

    assert_eq!(response.evidence_sentences, vec!["Exact source sentence"]);
}
