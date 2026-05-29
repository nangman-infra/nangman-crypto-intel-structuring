use crate::models::output::{
    ConfidenceBand, ContradictionFlag, EventType, RelevanceDecayHint, TerminalDecision,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelStage {
    Primary,
    PrimaryRepair,
    Escalation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceSnippet {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelStructuringRequest {
    pub raw_event_id: String,
    pub source_id: String,
    pub source_category: String,
    pub title: String,
    pub body: String,
    pub url: String,
    pub symbol_candidates: Vec<String>,
    pub event_category_hint: Option<String>,
    pub top50_relevance: String,
    pub content_kind: String,
    pub content_quality: String,
    pub content_quality_score: String,
    pub source_quality: String,
    pub source_relevance_scope: String,
    pub rule_event_type: EventType,
    pub rule_confidence: f64,
    pub evidence_candidates: Vec<String>,
    pub evidence_pack: Vec<EvidenceSnippet>,
    pub market_context_status: String,
    pub market_context_summary: String,
    #[serde(default)]
    pub repair_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelStructuringResponse {
    pub event_type: EventType,
    pub normalized_symbols: Vec<String>,
    pub symbol_confidence_band: ConfidenceBand,
    pub topic_summary: String,
    pub stance_summary: String,
    pub risk_summary: String,
    pub regime_hint: String,
    pub scenario_hint: String,
    pub confidence_band: ConfidenceBand,
    pub confidence_score: f64,
    pub novelty_score: f64,
    pub relevance_decay_hint: RelevanceDecayHint,
    pub contradiction_flags: Vec<ContradictionFlag>,
    #[serde(default)]
    pub evidence_ids: Vec<String>,
    #[serde(default)]
    pub evidence_sentences: Vec<String>,
    pub terminal_decision: TerminalDecision,
}
