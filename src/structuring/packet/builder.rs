use crate::models::constants::CONTEXT_FLAG_SCHEMA_VERSION;
use crate::models::market::MarketContextSnapshot;
use crate::models::raw::RawIntelEvent;
use crate::structuring::router::StructuringDecision;
use crate::structuring::story::{story_cluster_id, story_hint_key};

use super::ids::{flag_packet_id, initial_packet_id, packet_family_id};
use super::types::PacketSet;
use context_flag::build_context_flag_packet;
use health::build_health_event;
use resolved::ResolvedPacketFields;
use story_cluster::StoryClusterInput;
use structured_packet::StructuredPacketInput;
use time::{decision_available_at_ms, pending_market_context_schedule, time_window};

mod context_flag;
mod health;
mod resolved;
mod story_cluster;
mod structured_packet;
mod time;

pub fn build_packet_set(
    event: &RawIntelEvent,
    decision: &StructuringDecision,
    market_context: MarketContextSnapshot,
    policy_version: &str,
    observed_at_ms: i64,
    market_context_retry_interval_ms: i64,
    market_context_expire_after_ms: i64,
) -> PacketSet {
    let packet_family_id = packet_family_id(&event.event_id, policy_version);
    let packet_id = initial_packet_id(&event.event_id, policy_version);
    let flag_packet_id = flag_packet_id(&packet_id, CONTEXT_FLAG_SCHEMA_VERSION, policy_version);
    let resolved = ResolvedPacketFields::from_decision(decision);
    let story_hint_key = story_hint_key(event, &resolved.event_type, &resolved.normalized_symbols);
    let cluster_id = story_cluster_id(&story_hint_key, policy_version);
    let structured_at_ms = observed_at_ms;
    let decision_available_at_ms = decision_available_at_ms(event, structured_at_ms);
    let event_timestamp_ms = event.published_at_ms.unwrap_or(event.fetched_at_ms);
    let (market_context_retry_after_ms, market_context_expire_at_ms) =
        pending_market_context_schedule(
            &market_context,
            decision_available_at_ms,
            market_context_retry_interval_ms,
            market_context_expire_after_ms,
        );
    let time_relevance_window = time_window(
        event.published_at_ms.unwrap_or(event.fetched_at_ms),
        resolved.relevance_decay_hint.clone(),
    );
    let context_flag_confidence_band = resolved.confidence_band.clone();

    let story_cluster = story_cluster::build_story_cluster(StoryClusterInput {
        event,
        observed_at_ms,
        story_hint_key,
        cluster_id: cluster_id.clone(),
        event_type: &resolved.event_type,
        normalized_symbols: &resolved.normalized_symbols,
        novelty_score: resolved.novelty_score,
        contradiction_flags: &resolved.contradiction_flags,
    });

    let structured_packet = structured_packet::build_structured_packet(StructuredPacketInput {
        event,
        decision,
        market_context,
        packet_id: packet_id.clone(),
        packet_family_id,
        cluster_id: cluster_id.clone(),
        event_timestamp_ms,
        structured_at_ms,
        decision_available_at_ms,
        normalized_symbols: resolved.normalized_symbols,
        symbol_confidence_band: resolved.symbol_confidence_band,
        event_type: resolved.event_type,
        topic_summary: resolved.topic_summary,
        stance_summary: resolved.stance_summary,
        risk_summary: resolved.risk_summary,
        regime_hint: resolved.regime_hint,
        scenario_hint: resolved.scenario_hint,
        confidence_band: resolved.confidence_band,
        novelty_score: resolved.novelty_score,
        time_relevance_window: time_relevance_window.clone(),
        contradiction_flags: resolved.contradiction_flags,
        terminal_decision: resolved.terminal_decision,
        evidence_sentences: resolved.evidence_sentences,
        market_context_retry_after_ms,
        market_context_expire_at_ms,
    });

    let context_flag_packet = build_context_flag_packet(
        &structured_packet,
        flag_packet_id,
        packet_id,
        cluster_id,
        time_relevance_window,
        &context_flag_confidence_band,
        decision.model_tier_used.clone(),
    );
    let health_event = build_health_event(
        event,
        decision,
        &story_cluster,
        &context_flag_packet,
        observed_at_ms,
        policy_version,
    );

    PacketSet {
        story_cluster,
        structured_packet,
        context_flag_packet,
        health_event,
    }
}
