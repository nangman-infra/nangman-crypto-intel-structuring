use crate::hash::sha256_prefixed;
use crate::models::constants::{CONTEXT_FLAG_SCHEMA_VERSION, STRUCTURED_PACKET_SCHEMA_VERSION};
use crate::models::output::{S3ObjectPointer, StructuredPointer};
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::PacketSet;

pub(in crate::workflow::processor) fn structured_pointer(
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

pub(in crate::workflow::processor) fn context_flag_pointer(
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
