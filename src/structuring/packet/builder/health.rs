use crate::hash::stable_short_id;
use crate::models::constants::HEALTH_EVENT_SCHEMA_VERSION;
use crate::models::output::{
    ConflictLevel, ContextFlagPacket, HealthLevel, StoryCluster, StructuringHealthEvent,
};
use crate::models::raw::RawIntelEvent;
use crate::structuring::router::StructuringDecision;

pub(super) fn build_health_event(
    event: &RawIntelEvent,
    decision: &StructuringDecision,
    story_cluster: &StoryCluster,
    context_flag_packet: &Option<ContextFlagPacket>,
    observed_at_ms: i64,
    policy_version: &str,
) -> StructuringHealthEvent {
    StructuringHealthEvent {
        health_event_id: stable_short_id("intel_l1_health", &[&event.event_id, policy_version]),
        observed_at_ms,
        input_event_count: 1,
        cluster_count: 1,
        structured_packet_count: 1,
        flag_packet_count: usize::from(context_flag_packet.is_some()),
        model_l0_invocations: decision.primary_invocations,
        model_l1_invocations: decision.escalation_invocations,
        fallback_count: decision.fallback_count,
        conflict_high_count: usize::from(story_cluster.conflict_level == ConflictLevel::High),
        health_level: if decision.fallback_count > 0 {
            HealthLevel::FallbackOnly
        } else {
            HealthLevel::Healthy
        },
        reason: None,
        schema_version: HEALTH_EVENT_SCHEMA_VERSION.to_owned(),
    }
}
