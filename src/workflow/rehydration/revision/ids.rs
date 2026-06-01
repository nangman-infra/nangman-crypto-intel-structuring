use crate::models::output::StructuredIntelPacket;

pub(in crate::workflow::rehydration) fn parse_revision_from_key(key: &str) -> Option<u32> {
    let file_name = key.rsplit_once('/').map_or(key, |(_, file_name)| file_name);
    file_name
        .strip_suffix(".json")?
        .strip_prefix("revision=")?
        .parse()
        .ok()
}

pub(in crate::workflow::rehydration) fn effective_packet_family_id(
    packet: &StructuredIntelPacket,
) -> &str {
    if !packet.packet_family_id.trim().is_empty() {
        packet.packet_family_id.as_str()
    } else if !packet.raw_event_id.trim().is_empty() {
        packet.raw_event_id.as_str()
    } else if let Some(source_event_id) = first_non_empty_source_event_id(packet) {
        source_event_id
    } else {
        packet.packet_id.as_str()
    }
}

pub(in crate::workflow::rehydration) fn effective_raw_event_id(
    packet: &StructuredIntelPacket,
) -> &str {
    if !packet.raw_event_id.trim().is_empty() {
        packet.raw_event_id.as_str()
    } else if let Some(source_event_id) = first_non_empty_source_event_id(packet) {
        source_event_id
    } else {
        packet.packet_id.as_str()
    }
}

fn first_non_empty_source_event_id(packet: &StructuredIntelPacket) -> Option<&str> {
    packet
        .source_event_ids
        .iter()
        .map(String::as_str)
        .find(|source_event_id| !source_event_id.trim().is_empty())
}
