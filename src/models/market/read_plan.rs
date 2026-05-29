#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketL1ReadPlan {
    pub l1_run_id: String,
    pub manifest_key: String,
    pub report_key: String,
    pub output_object_keys: Vec<String>,
    pub market_data_quality_summary_key: Option<String>,
    pub market_feature_delta_key: Option<String>,
    pub market_feature_delta_summary_key: Option<String>,
    pub market_regime_context_key: Option<String>,
    pub symbol_universe_snapshot_key: Option<String>,
    pub input_time_range_start_ms: i64,
    pub input_time_range_end_ms: i64,
}
