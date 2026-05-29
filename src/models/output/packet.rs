use crate::models::constants::STRUCTURED_PACKET_SCHEMA_VERSION;
use crate::models::market::{MarketContextSnapshot, MarketContextStatus};
use serde::{Deserialize, Serialize};

use super::common::{
    ConfidenceBand, ContradictionFlag, EventType, ModelTierUsed, TerminalDecision,
    TimeRelevanceWindow,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolResolutionTrace {
    pub raw_mentions: Vec<String>,
    pub resolved_project: Option<String>,
    pub resolved_asset: Option<String>,
    pub canonical_symbol: Option<String>,
    pub venue_symbols: Vec<String>,
    pub mapping_confidence: ConfidenceBand,
    pub ambiguity_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceIndependenceSummary {
    pub source_event_count: usize,
    pub independent_source_count: usize,
    pub official_source_present: bool,
    pub duplicate_content_hashes: Vec<String>,
    pub syndicated_from: Option<String>,
    pub original_source_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextEvidence {
    pub evidence_text: String,
    pub source_event_id: String,
    pub source_id: String,
    pub published_at_ms: Option<i64>,
    pub evidence_kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricEvidence {
    pub metric_name: String,
    pub symbol: Option<String>,
    pub venue: Option<String>,
    pub value: Option<f64>,
    pub previous_value: Option<f64>,
    pub delta_pct: Option<f64>,
    pub window_ms: Option<i64>,
    pub observed_at_ms: i64,
    pub source_event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceQualityReason {
    BaselineMissing,
    SingleNumericSnapshot,
    SingleSourceOnly,
    TitleOnly,
    SymbolAmbiguous,
    MarketContextMissing,
    DuplicateOrSyndicatedSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketContextRef {
    pub status: MarketContextStatus,
    pub basis_timestamp_ms: Option<i64>,
    pub basis_kind: String,
    pub window_start_ms: Option<i64>,
    pub window_end_ms: Option<i64>,
    pub manifest_key: Option<String>,
    pub output_object_keys: Vec<String>,
    pub market_data_quality_summary_key: Option<String>,
    pub market_feature_delta_key: Option<String>,
    pub market_feature_delta_summary_key: Option<String>,
    pub market_regime_context_key: Option<String>,
    pub symbol_universe_snapshot_key: Option<String>,
}
