use crate::models::output::{PacketRevisionIndex, StructuredIntelPacket};

pub(super) fn build_revision_index(
    packet_family_id: &str,
    raw_event_id: &str,
    revision: u32,
    packet_id: &str,
    structured_key: &str,
    revised_packet: &StructuredIntelPacket,
    created_at_ms: i64,
) -> PacketRevisionIndex {
    PacketRevisionIndex {
        schema_version: PacketRevisionIndex::schema(),
        packet_family_id: packet_family_id.to_owned(),
        raw_event_id: raw_event_id.to_owned(),
        latest_revision: revision,
        latest_packet_id: packet_id.to_owned(),
        latest_structured_key: structured_key.to_owned(),
        market_context_status: revised_packet.market_context_status.clone(),
        updated_at_ms: created_at_ms,
    }
}
