use crate::models::constants::HEALTH_EVENT_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuringHealthEvent {
    pub health_event_id: String,
    pub observed_at_ms: i64,
    pub input_event_count: usize,
    pub cluster_count: usize,
    pub structured_packet_count: usize,
    pub flag_packet_count: usize,
    pub model_l0_invocations: usize,
    pub model_l1_invocations: usize,
    pub fallback_count: usize,
    pub conflict_high_count: usize,
    pub health_level: HealthLevel,
    pub reason: Option<String>,
    pub schema_version: String,
}

impl StructuringHealthEvent {
    pub fn schema() -> String {
        HEALTH_EVENT_SCHEMA_VERSION.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HealthLevel {
    Healthy,
    Degraded,
    FallbackOnly,
    Blocked,
}
