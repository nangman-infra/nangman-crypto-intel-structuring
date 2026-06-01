use serde::{Deserialize, Serialize};

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
