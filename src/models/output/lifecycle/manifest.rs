use crate::models::constants::MANIFEST_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputObjectRef {
    pub object_family: String,
    pub key: String,
    pub record_count: usize,
    pub byte_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntelL1Manifest {
    pub schema_version: String,
    pub run_id: String,
    pub raw_event_id: String,
    pub status: String,
    pub started_at_ms: i64,
    pub finished_at_ms: i64,
    pub structuring_policy_version: String,
    pub output_object_count: usize,
    pub output_objects: Vec<OutputObjectRef>,
    pub structured_packet_count: usize,
    pub context_flag_packet_count: usize,
    pub story_cluster_count: usize,
    pub health_event_count: usize,
}

impl IntelL1Manifest {
    pub fn schema() -> String {
        MANIFEST_SCHEMA_VERSION.to_owned()
    }
}
