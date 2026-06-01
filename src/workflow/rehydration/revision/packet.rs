use crate::models::market::MarketContextSnapshot;
use crate::models::output::{EvidenceQualityReason, StructuredIntelPacket};
use crate::structuring::packet::market_context_ref;

pub(super) fn build_revised_packet(
    packet: &StructuredIntelPacket,
    market_context: MarketContextSnapshot,
    terminal_reason: Option<String>,
    revision: u32,
    packet_family_id: String,
    raw_event_id: String,
    packet_id: String,
) -> StructuredIntelPacket {
    let mut revised_packet = packet.clone();
    revised_packet.packet_family_id = packet_family_id;
    revised_packet.raw_event_id = raw_event_id;
    revised_packet.packet_id = packet_id;
    revised_packet.revision = revision;
    revised_packet.supersedes_packet_id = Some(packet.packet_id.clone());
    revised_packet.market_context_status = market_context.status.clone();
    revised_packet.market_context = market_context.clone();
    revised_packet.market_context_ref = market_context_ref(&market_context);
    revised_packet.market_context_retry_after_ms = None;
    revised_packet.market_context_expire_at_ms = None;
    revised_packet.market_context_terminal_reason = terminal_reason;
    if market_context.status.is_any_available() {
        revised_packet
            .evidence_quality_reasons
            .retain(|reason| !matches!(reason, EvidenceQualityReason::MarketContextMissing));
    }
    revised_packet
}
