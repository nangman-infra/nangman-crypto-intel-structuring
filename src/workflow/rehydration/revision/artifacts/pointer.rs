use crate::hash::sha256_prefixed;
use crate::models::constants::STRUCTURED_PACKET_SCHEMA_VERSION;
use crate::models::output::{S3ObjectPointer, StructuredIntelPacket, StructuredPointer};

pub(super) struct PointerBuildInput<'a> {
    pub(super) packet_id: String,
    pub(super) raw_event_id: String,
    pub(super) revised_packet: &'a StructuredIntelPacket,
    pub(super) bucket: &'a str,
    pub(super) structured_key: &'a str,
    pub(super) structured_bytes: &'a [u8],
    pub(super) manifest_key: &'a str,
    pub(super) created_at_ms: i64,
}

pub(super) fn build_pointer(input: PointerBuildInput<'_>) -> StructuredPointer {
    StructuredPointer {
        schema_version: StructuredPointer::schema(),
        packet_id: input.packet_id,
        raw_event_id: input.raw_event_id,
        terminal_decision: input.revised_packet.terminal_decision.clone(),
        storage_ref: S3ObjectPointer {
            bucket: input.bucket.to_owned(),
            key: input.structured_key.to_owned(),
            content_sha256: sha256_prefixed(input.structured_bytes),
            schema_version: STRUCTURED_PACKET_SCHEMA_VERSION.to_owned(),
        },
        manifest_key: input.manifest_key.to_owned(),
        created_at_ms: input.created_at_ms,
    }
}
