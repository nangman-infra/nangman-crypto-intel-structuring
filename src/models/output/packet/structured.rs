use crate::models::constants::STRUCTURED_PACKET_SCHEMA_VERSION;
use crate::models::market::{MarketContextSnapshot, MarketContextStatus};
use serde::{Deserialize, Serialize};

use super::super::common::{
    ConfidenceBand, ContradictionFlag, EventType, ModelTierUsed, TerminalDecision,
    TimeRelevanceWindow,
};
use super::{
    EvidenceQualityReason, MarketContextRef, MetricEvidence, SourceIndependenceSummary,
    SymbolResolutionTrace, TextEvidence,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuredIntelPacket {
    pub packet_id: String,
    #[serde(default)]
    pub packet_family_id: String,
    #[serde(default)]
    pub raw_event_id: String,
    #[serde(default)]
    pub event_timestamp_ms: i64,
    #[serde(default)]
    pub revision: u32,
    #[serde(default)]
    pub supersedes_packet_id: Option<String>,
    pub cluster_id: String,
    pub source_event_ids: Vec<String>,
    pub published_at_ms: Option<i64>,
    pub fetched_at_ms: i64,
    pub structured_at_ms: i64,
    pub decision_available_at_ms: i64,
    pub normalized_symbols: Vec<String>,
    pub symbol_confidence_band: ConfidenceBand,
    pub symbol_resolution_trace: Vec<SymbolResolutionTrace>,
    pub event_type: EventType,
    pub topic_summary: String,
    pub stance_summary: String,
    pub risk_summary: String,
    pub regime_hint: String,
    pub scenario_hint: String,
    pub confidence_band: ConfidenceBand,
    pub novelty_score: f64,
    pub time_relevance_window: TimeRelevanceWindow,
    pub contradiction_flags: Vec<ContradictionFlag>,
    pub source_quality_summary: String,
    pub source_independence_summary: SourceIndependenceSummary,
    pub text_evidence: Vec<TextEvidence>,
    pub metric_evidence: Vec<MetricEvidence>,
    pub evidence_quality_reasons: Vec<EvidenceQualityReason>,
    pub market_context_status: MarketContextStatus,
    #[serde(default)]
    pub market_context_retry_after_ms: Option<i64>,
    #[serde(default)]
    pub market_context_expire_at_ms: Option<i64>,
    #[serde(default)]
    pub market_context_terminal_reason: Option<String>,
    pub market_context_ref: MarketContextRef,
    pub model_tier_used: ModelTierUsed,
    pub terminal_decision: TerminalDecision,
    pub evidence_sentences: Vec<String>,
    pub market_context: MarketContextSnapshot,
    pub schema_version: String,
}

impl StructuredIntelPacket {
    pub fn schema() -> String {
        STRUCTURED_PACKET_SCHEMA_VERSION.to_owned()
    }
}
