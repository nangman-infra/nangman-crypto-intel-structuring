use crate::models::constants::{
    INDEX_POINTER_SCHEMA_VERSION, PACKET_REVISION_INDEX_SCHEMA_VERSION,
};

use super::segments::path_segment;

pub fn index_key(raw_event_id: &str, policy_version: &str) -> String {
    format!(
        "intel-l1-index/schema={INDEX_POINTER_SCHEMA_VERSION}/raw_event_id={}/policy={}.json",
        path_segment(raw_event_id),
        path_segment(policy_version)
    )
}

pub fn prepared_index_key(raw_event_id: &str, policy_version: &str) -> String {
    format!(
        "intel-l1-index/status=prepared/schema={INDEX_POINTER_SCHEMA_VERSION}/raw_event_id={}/policy={}.json",
        path_segment(raw_event_id),
        path_segment(policy_version)
    )
}

pub fn packet_revision_index_prefix(packet_family_id: &str) -> String {
    format!(
        "packet-revision-index/schema={PACKET_REVISION_INDEX_SCHEMA_VERSION}/packet_family_id={}/",
        path_segment(packet_family_id)
    )
}

pub fn packet_revision_index_key(packet_family_id: &str, revision: u32) -> String {
    format!(
        "{}revision={:010}.json",
        packet_revision_index_prefix(packet_family_id),
        revision
    )
}
