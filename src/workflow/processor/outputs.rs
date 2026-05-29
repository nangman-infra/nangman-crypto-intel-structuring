use super::support::object_ref;
use crate::hash::sha256_prefixed;
use crate::models::constants::{CONTEXT_FLAG_SCHEMA_VERSION, STRUCTURED_PACKET_SCHEMA_VERSION};
use crate::models::output::{OutputObjectRef, S3ObjectPointer, StructuredPointer};
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::PacketSet;
use crate::workflow::keys;

pub(super) struct PacketObjectKeys {
    pub(super) structured_key: String,
    pub(super) flag_key: Option<String>,
    pub(super) story_key: String,
    pub(super) health_key: String,
}

pub(super) fn packet_object_keys(
    observed_at_ms: i64,
    raw_event: &RawIntelEvent,
    packet_set: &PacketSet,
) -> PacketObjectKeys {
    PacketObjectKeys {
        structured_key: keys::structured_packet_key(
            observed_at_ms,
            &raw_event.event_id,
            &packet_set.structured_packet.packet_id,
        ),
        flag_key: packet_set
            .context_flag_packet
            .as_ref()
            .map(|context_flag_packet| {
                keys::context_flag_key(
                    observed_at_ms,
                    &raw_event.event_id,
                    &context_flag_packet.flag_packet_id,
                )
            }),
        story_key: keys::story_cluster_key(
            observed_at_ms,
            &raw_event.event_id,
            &packet_set.story_cluster.cluster_id,
        ),
        health_key: keys::health_key(
            observed_at_ms,
            &raw_event.event_id,
            &packet_set.health_event.health_event_id,
        ),
    }
}

pub(super) struct PacketOutputRefsInput<'a> {
    pub(super) story_member_key: &'a str,
    pub(super) story_member_bytes: &'a [u8],
    pub(super) keys: &'a PacketObjectKeys,
    pub(super) story_bytes: &'a [u8],
    pub(super) structured_bytes: &'a [u8],
    pub(super) flag_bytes: Option<&'a [u8]>,
    pub(super) health_bytes: &'a [u8],
}

pub(super) fn packet_output_refs(input: PacketOutputRefsInput<'_>) -> Vec<OutputObjectRef> {
    let mut output_objects = vec![
        object_ref(
            "story_member",
            input.story_member_key,
            1,
            input.story_member_bytes,
        ),
        object_ref("story_cluster", &input.keys.story_key, 1, input.story_bytes),
        object_ref(
            "structured_intel_packet",
            &input.keys.structured_key,
            1,
            input.structured_bytes,
        ),
        object_ref(
            "structuring_health_event",
            &input.keys.health_key,
            1,
            input.health_bytes,
        ),
    ];
    if let (Some(flag_key), Some(flag_bytes)) = (&input.keys.flag_key, input.flag_bytes) {
        output_objects.push(object_ref("context_flag_packet", flag_key, 1, flag_bytes));
    }
    output_objects
}

pub(super) fn structured_pointer(
    packet_set: &PacketSet,
    raw_event: &RawIntelEvent,
    output_bucket: &str,
    structured_key: &str,
    structured_bytes: &[u8],
    manifest_key: &str,
    finished_at_ms: i64,
) -> StructuredPointer {
    StructuredPointer {
        schema_version: StructuredPointer::schema(),
        packet_id: packet_set.structured_packet.packet_id.clone(),
        raw_event_id: raw_event.event_id.clone(),
        terminal_decision: packet_set.structured_packet.terminal_decision.clone(),
        storage_ref: S3ObjectPointer {
            bucket: output_bucket.to_owned(),
            key: structured_key.to_owned(),
            content_sha256: sha256_prefixed(structured_bytes),
            schema_version: STRUCTURED_PACKET_SCHEMA_VERSION.to_owned(),
        },
        manifest_key: manifest_key.to_owned(),
        created_at_ms: finished_at_ms,
    }
}

pub(super) fn context_flag_pointer(
    packet_set: &PacketSet,
    raw_event: &RawIntelEvent,
    output_bucket: &str,
    flag_key: Option<&str>,
    flag_bytes: Option<&[u8]>,
    manifest_key: &str,
    finished_at_ms: i64,
) -> Option<StructuredPointer> {
    let (Some(context_flag_packet), Some(flag_key), Some(flag_bytes)) =
        (&packet_set.context_flag_packet, flag_key, flag_bytes)
    else {
        return None;
    };
    Some(StructuredPointer {
        schema_version: StructuredPointer::schema(),
        packet_id: context_flag_packet.flag_packet_id.clone(),
        raw_event_id: raw_event.event_id.clone(),
        terminal_decision: packet_set.structured_packet.terminal_decision.clone(),
        storage_ref: S3ObjectPointer {
            bucket: output_bucket.to_owned(),
            key: flag_key.to_owned(),
            content_sha256: sha256_prefixed(flag_bytes),
            schema_version: CONTEXT_FLAG_SCHEMA_VERSION.to_owned(),
        },
        manifest_key: manifest_key.to_owned(),
        created_at_ms: finished_at_ms,
    })
}
