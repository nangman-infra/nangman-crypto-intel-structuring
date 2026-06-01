use crate::models::market::MarketContextStatus;
use serde::{Deserialize, Serialize};

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
