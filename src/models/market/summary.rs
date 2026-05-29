use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketSymbolSummary {
    pub symbol: String,
    pub venue: String,
    pub window_start_ms: i64,
    pub window_end_ms: i64,
    pub mid_price: Option<f64>,
    pub spread_bps: Option<f64>,
    pub trade_count: i64,
    pub trade_volume: f64,
    pub slice_completeness: String,
}
