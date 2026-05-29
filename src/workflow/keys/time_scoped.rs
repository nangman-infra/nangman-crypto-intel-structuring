use crate::models::constants::{
    CONTEXT_FLAG_SCHEMA_VERSION, HEALTH_EVENT_SCHEMA_VERSION, MANIFEST_SCHEMA_VERSION,
    QUARANTINE_SCHEMA_VERSION, STORY_CLUSTER_SCHEMA_VERSION, STRUCTURED_PACKET_SCHEMA_VERSION,
};
use crate::time::time_part;

use super::segments::path_segment;

pub fn structured_packet_key(timestamp_ms: i64, raw_event_id: &str, packet_id: &str) -> String {
    let part = time_part(timestamp_ms);
    format!(
        "structured-intel-packet/schema={STRUCTURED_PACKET_SCHEMA_VERSION}/dt={}/hour={:02}/raw_event_id={}/packet_id={}/part-000001.jsonl",
        part.event_date,
        part.hour,
        path_segment(raw_event_id),
        path_segment(packet_id)
    )
}

pub fn context_flag_key(timestamp_ms: i64, raw_event_id: &str, flag_packet_id: &str) -> String {
    let part = time_part(timestamp_ms);
    format!(
        "context-flag-packet/schema={CONTEXT_FLAG_SCHEMA_VERSION}/dt={}/hour={:02}/raw_event_id={}/flag_packet_id={}/part-000001.jsonl",
        part.event_date,
        part.hour,
        path_segment(raw_event_id),
        path_segment(flag_packet_id)
    )
}

pub fn story_cluster_key(timestamp_ms: i64, raw_event_id: &str, cluster_id: &str) -> String {
    let part = time_part(timestamp_ms);
    format!(
        "story-cluster/schema={STORY_CLUSTER_SCHEMA_VERSION}/dt={}/hour={:02}/raw_event_id={}/cluster_id={}/part-000001.jsonl",
        part.event_date,
        part.hour,
        path_segment(raw_event_id),
        path_segment(cluster_id)
    )
}

pub fn health_key(timestamp_ms: i64, raw_event_id: &str, health_event_id: &str) -> String {
    let part = time_part(timestamp_ms);
    format!(
        "structuring-health/schema={HEALTH_EVENT_SCHEMA_VERSION}/dt={}/hour={:02}/raw_event_id={}/health_event_id={}/part-000001.jsonl",
        part.event_date,
        part.hour,
        path_segment(raw_event_id),
        path_segment(health_event_id)
    )
}

pub fn manifest_key(timestamp_ms: i64, raw_event_id: &str, run_id: &str) -> String {
    let part = time_part(timestamp_ms);
    format!(
        "manifests/schema={MANIFEST_SCHEMA_VERSION}/dt={}/hour={:02}/raw_event_id={}/run_id={}.json",
        part.event_date,
        part.hour,
        path_segment(raw_event_id),
        path_segment(run_id)
    )
}

pub fn quarantine_key(
    timestamp_ms: i64,
    raw_event_id: Option<&str>,
    quarantine_id: &str,
) -> String {
    let part = time_part(timestamp_ms);
    format!(
        "quarantine/schema={QUARANTINE_SCHEMA_VERSION}/dt={}/hour={:02}/raw_event_id={}/quarantine_id={}.json",
        part.event_date,
        part.hour,
        path_segment(raw_event_id.unwrap_or("unknown")),
        path_segment(quarantine_id)
    )
}
