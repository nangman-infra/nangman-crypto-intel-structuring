use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::PacketSet;
use crate::workflow::keys;

pub(in crate::workflow::processor) struct PacketObjectKeys {
    pub(in crate::workflow::processor) structured_key: String,
    pub(in crate::workflow::processor) flag_key: Option<String>,
    pub(in crate::workflow::processor) story_key: String,
    pub(in crate::workflow::processor) health_key: String,
}

pub(in crate::workflow::processor) fn packet_object_keys(
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
