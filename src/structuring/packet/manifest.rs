use crate::models::output::{IntelL1Manifest, OutputObjectRef};

use super::types::PacketSet;

#[derive(Debug, Clone)]
pub struct ManifestBuildInput {
    pub run_id: String,
    pub raw_event_id: String,
    pub status: String,
    pub started_at_ms: i64,
    pub finished_at_ms: i64,
    pub policy_version: String,
    pub output_objects: Vec<OutputObjectRef>,
}

pub fn build_manifest(input: ManifestBuildInput, packet_set: &PacketSet) -> IntelL1Manifest {
    IntelL1Manifest {
        schema_version: crate::models::constants::MANIFEST_SCHEMA_VERSION.to_owned(),
        run_id: input.run_id,
        raw_event_id: input.raw_event_id,
        status: input.status,
        started_at_ms: input.started_at_ms,
        finished_at_ms: input.finished_at_ms,
        structuring_policy_version: input.policy_version,
        output_object_count: input.output_objects.len(),
        output_objects: input.output_objects,
        structured_packet_count: 1,
        context_flag_packet_count: usize::from(packet_set.context_flag_packet.is_some()),
        story_cluster_count: 1,
        health_event_count: usize::from(!packet_set.health_event.health_event_id.is_empty()),
    }
}
