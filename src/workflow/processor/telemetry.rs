use super::support::is_numeric_market_snapshot;
use crate::error::AppResult;
use crate::hash::sha256_prefixed;
use crate::models::raw::RawIntelEvent;
use crate::observability::{ProcessingMetric, emit_processing_metric};
use crate::structuring::packet::PacketSet;

pub(super) fn emit_success_metric(
    raw_event: &RawIntelEvent,
    packet_set: &PacketSet,
) -> AppResult<()> {
    emit_processing_metric(&ProcessingMetric {
        raw_event_id: raw_event.event_id.clone(),
        packet_id: packet_set.structured_packet.packet_id.clone(),
        model_tier_used: packet_set.structured_packet.model_tier_used.clone(),
        terminal_decision: packet_set.structured_packet.terminal_decision.clone(),
        market_context_status: packet_set.structured_packet.market_context_status.clone(),
        ack_ready: true,
        fallback_count: packet_set.health_event.fallback_count,
        conflict_count: packet_set.structured_packet.contradiction_flags.len(),
        primary_invocation_count: packet_set.health_event.model_l0_invocations,
        escalation_invocation_count: packet_set.health_event.model_l1_invocations,
        numeric_snapshot_count: usize::from(is_numeric_market_snapshot(raw_event)),
        stale_market_context_count: usize::from(
            packet_set
                .structured_packet
                .market_context_status
                .is_stale_but_usable(),
        ),
        escalation_on_numeric_snapshot_count: usize::from(
            is_numeric_market_snapshot(raw_event)
                && packet_set.health_event.model_l1_invocations > 0,
        ),
    })
}

pub(super) fn print_success_log(
    raw_event: &RawIntelEvent,
    packet_set: &PacketSet,
    manifest_bytes: &[u8],
) -> AppResult<()> {
    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({
            "raw_event_id": raw_event.event_id,
            "packet_id": packet_set.structured_packet.packet_id,
            "terminal_decision": packet_set.structured_packet.terminal_decision,
            "model_tier_used": packet_set.structured_packet.model_tier_used,
            "market_context_status": packet_set.structured_packet.market_context_status,
            "evidence_quality_reasons": packet_set.structured_packet.evidence_quality_reasons,
            "primary_invocations": packet_set.health_event.model_l0_invocations,
            "escalation_invocations": packet_set.health_event.model_l1_invocations,
            "manifest_sha256": sha256_prefixed(manifest_bytes),
            "ack_ready": true
        }))?
    );
    Ok(())
}
