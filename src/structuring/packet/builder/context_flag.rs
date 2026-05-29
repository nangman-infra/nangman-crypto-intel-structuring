use crate::models::constants::CONTEXT_FLAG_SCHEMA_VERSION;
use crate::models::output::{
    ConfidenceBand, ContextFlagPacket, ModelTierUsed, StructuredIntelPacket, TimeRelevanceWindow,
};

use super::super::flags::{context_flag, flag_confidence, risk_flag, should_emit_context_flag};

pub(super) fn build_context_flag_packet(
    structured_packet: &StructuredIntelPacket,
    flag_packet_id: String,
    packet_id: String,
    cluster_id: String,
    time_relevance_window: TimeRelevanceWindow,
    confidence_band: &ConfidenceBand,
    model_tier_used: ModelTierUsed,
) -> Option<ContextFlagPacket> {
    should_emit_context_flag(structured_packet).then(|| ContextFlagPacket {
        flag_packet_id,
        packet_id,
        cluster_id,
        normalized_symbols: structured_packet.normalized_symbols.clone(),
        observe_only: true,
        block_new_entries: false,
        reduce_only: false,
        paper_only: true,
        context_flag: context_flag(structured_packet),
        risk_flag: risk_flag(structured_packet),
        regime_flag: structured_packet.regime_hint.clone(),
        scenario_flag: structured_packet.scenario_hint.clone(),
        time_relevance_window,
        flag_confidence_band: flag_confidence(confidence_band),
        reason_summary: structured_packet.risk_summary.clone(),
        model_tier_used,
        schema_version: CONTEXT_FLAG_SCHEMA_VERSION.to_owned(),
    })
}
