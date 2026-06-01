use crate::models::constants::MANIFEST_SCHEMA_VERSION;
use crate::models::output::{IntelL1Manifest, OutputObjectRef};

pub(super) fn build_manifest(
    run_id: &str,
    raw_event_id: &str,
    structured_key: &str,
    structured_bytes_len: usize,
    terminal_missing_market_context: bool,
    created_at_ms: i64,
    policy_version: &str,
) -> IntelL1Manifest {
    IntelL1Manifest {
        schema_version: MANIFEST_SCHEMA_VERSION.to_owned(),
        run_id: run_id.to_owned(),
        raw_event_id: raw_event_id.to_owned(),
        status: if terminal_missing_market_context {
            "terminal_missing_market_context".to_owned()
        } else {
            "rehydrated_market_context".to_owned()
        },
        started_at_ms: created_at_ms,
        finished_at_ms: created_at_ms,
        structuring_policy_version: policy_version.to_owned(),
        output_object_count: 1,
        output_objects: vec![OutputObjectRef {
            object_family: "structured_intel_packet".to_owned(),
            key: structured_key.to_owned(),
            record_count: 1,
            byte_count: structured_bytes_len,
        }],
        structured_packet_count: 1,
        context_flag_packet_count: 0,
        story_cluster_count: 0,
        health_event_count: 0,
    }
}
