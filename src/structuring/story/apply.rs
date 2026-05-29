use crate::models::output::{ConflictLevel, ContextFlagPacket, HealthLevel, StoryCluster};
use crate::structuring::packet::PacketSet;

pub(super) fn apply_story_cluster(packet_set: &mut PacketSet, cluster: StoryCluster) {
    packet_set.structured_packet.cluster_id = cluster.cluster_id.clone();
    packet_set.structured_packet.source_event_ids = cluster.source_event_ids.clone();
    packet_set.structured_packet.normalized_symbols = cluster.related_symbols.clone();
    if let Some(context_flag_packet) = packet_set.context_flag_packet.as_mut() {
        update_context_flag_for_cluster(context_flag_packet, &cluster);
    }
    packet_set.story_cluster = cluster;
    packet_set.health_event.conflict_high_count =
        usize::from(packet_set.story_cluster.conflict_level == ConflictLevel::High);
    packet_set.health_event.flag_packet_count =
        usize::from(packet_set.context_flag_packet.is_some());
    if packet_set.health_event.fallback_count > 0 {
        packet_set.health_event.health_level = HealthLevel::FallbackOnly;
    } else if packet_set.story_cluster.conflict_level == ConflictLevel::High {
        packet_set.health_event.health_level = HealthLevel::Degraded;
    }
}

fn update_context_flag_for_cluster(
    context_flag_packet: &mut ContextFlagPacket,
    cluster: &StoryCluster,
) {
    context_flag_packet.cluster_id = cluster.cluster_id.clone();
    context_flag_packet.normalized_symbols = cluster.related_symbols.clone();
}
