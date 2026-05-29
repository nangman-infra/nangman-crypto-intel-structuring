use crate::models::constants::CONTEXT_FLAG_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

use super::super::common::{ConfidenceBand, ModelTierUsed, TimeRelevanceWindow};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContextFlagPacket {
    pub flag_packet_id: String,
    pub packet_id: String,
    pub cluster_id: String,
    pub normalized_symbols: Vec<String>,
    pub observe_only: bool,
    pub block_new_entries: bool,
    pub reduce_only: bool,
    pub paper_only: bool,
    pub context_flag: String,
    pub risk_flag: String,
    pub regime_flag: String,
    pub scenario_flag: String,
    pub time_relevance_window: TimeRelevanceWindow,
    pub flag_confidence_band: ConfidenceBand,
    pub reason_summary: String,
    pub model_tier_used: ModelTierUsed,
    pub schema_version: String,
}

impl ContextFlagPacket {
    pub fn schema() -> String {
        CONTEXT_FLAG_SCHEMA_VERSION.to_owned()
    }
}
