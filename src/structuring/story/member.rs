use crate::models::constants::STORY_MEMBER_SCHEMA_VERSION;
use crate::models::output::StoryMember;
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::PacketSet;

impl StoryMember {
    pub fn from_packet_set(
        event: &RawIntelEvent,
        packet_set: &PacketSet,
        policy_version: &str,
        observed_at_ms: i64,
    ) -> Self {
        Self {
            schema_version: STORY_MEMBER_SCHEMA_VERSION.to_owned(),
            story_hint_key: packet_set.story_cluster.story_hint_key.clone(),
            cluster_id: packet_set.story_cluster.cluster_id.clone(),
            raw_event_id: event.event_id.clone(),
            source_id: event.source_id.clone(),
            source_category: event.source_category.clone(),
            normalized_symbols: packet_set.structured_packet.normalized_symbols.clone(),
            event_type: packet_set.structured_packet.event_type.clone(),
            confidence_band: packet_set.structured_packet.confidence_band.clone(),
            contradiction_flags: packet_set.structured_packet.contradiction_flags.clone(),
            trust_tier: event.trust_tier.clone(),
            published_at_ms: event.published_at_ms,
            observed_at_ms,
            novelty_score: packet_set.structured_packet.novelty_score,
            structuring_policy_version: policy_version.to_owned(),
        }
    }
}
