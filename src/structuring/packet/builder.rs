use crate::models::constants::CONTEXT_FLAG_SCHEMA_VERSION;
use crate::models::market::MarketContextSnapshot;
use crate::models::raw::RawIntelEvent;
use crate::structuring::router::StructuringDecision;
use crate::structuring::story::{story_cluster_id, story_hint_key};

use super::ids::{flag_packet_id, initial_packet_id, packet_family_id};
use super::types::PacketSet;
use context_flag::build_context_flag_packet;
use health::build_health_event;
use story_cluster::StoryClusterInput;
use structured_packet::StructuredPacketInput;
use time::{decision_available_at_ms, pending_market_context_schedule, time_window};

mod context_flag;
mod health;
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
    let model = decision.model_response.as_ref();
    let packet_family_id = packet_family_id(&event.event_id, policy_version);
    let packet_id = initial_packet_id(&event.event_id, policy_version);
    let flag_packet_id = flag_packet_id(&packet_id, CONTEXT_FLAG_SCHEMA_VERSION, policy_version);
    let normalized_symbols = model
        .map(|value| value.normalized_symbols.clone())
        .unwrap_or_else(|| decision.rule.normalized_symbols.clone());
    let confidence_band = model
        .map(|value| value.confidence_band.clone())
        .unwrap_or_else(|| decision.rule.confidence_band.clone());
    let event_type = model
        .map(|value| value.event_type.clone())
        .unwrap_or_else(|| decision.rule.event_type.clone());
    let story_hint_key = story_hint_key(event, &event_type, &normalized_symbols);
    let cluster_id = story_cluster_id(&story_hint_key, policy_version);
    let contradiction_flags = model
        .map(|value| value.contradiction_flags.clone())
        .unwrap_or_else(|| decision.rule.contradiction_flags.clone());
    let terminal_decision = model
        .map(|value| value.terminal_decision.clone())
        .unwrap_or_else(|| decision.rule.terminal_decision.clone());
    let evidence_sentences = model
        .map(|value| value.evidence_sentences.clone())
        .unwrap_or_else(|| decision.rule.evidence_sentences.clone());
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
    let relevance_decay_hint = model
        .map(|value| value.relevance_decay_hint.clone())
        .unwrap_or_else(|| decision.rule.relevance_decay_hint.clone());
    let time_relevance_window = time_window(
        event.published_at_ms.unwrap_or(event.fetched_at_ms),
        relevance_decay_hint,
    );

    let story_cluster = story_cluster::build_story_cluster(StoryClusterInput {
        event,
        observed_at_ms,
        story_hint_key,
        cluster_id: cluster_id.clone(),
        event_type: &event_type,
        normalized_symbols: &normalized_symbols,
        novelty_score: model
            .map(|value| value.novelty_score)
            .unwrap_or(decision.rule.novelty_score),
        contradiction_flags: &contradiction_flags,
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
        normalized_symbols,
        symbol_confidence_band: model
            .map(|value| value.symbol_confidence_band.clone())
            .unwrap_or_else(|| decision.rule.symbol_confidence_band.clone()),
        event_type,
        topic_summary: model
            .map(|value| value.topic_summary.clone())
            .unwrap_or_else(|| decision.rule.topic_summary.clone()),
        stance_summary: model
            .map(|value| value.stance_summary.clone())
            .unwrap_or_else(|| decision.rule.stance_summary.clone()),
        risk_summary: model
            .map(|value| value.risk_summary.clone())
            .unwrap_or_else(|| decision.rule.risk_summary.clone()),
        regime_hint: model
            .map(|value| value.regime_hint.clone())
            .unwrap_or_else(|| decision.rule.regime_hint.clone()),
        scenario_hint: model
            .map(|value| value.scenario_hint.clone())
            .unwrap_or_else(|| decision.rule.scenario_hint.clone()),
        confidence_band: confidence_band.clone(),
        novelty_score: model
            .map(|value| value.novelty_score)
            .unwrap_or(decision.rule.novelty_score),
        time_relevance_window: time_relevance_window.clone(),
        contradiction_flags,
        terminal_decision,
        evidence_sentences,
        market_context_retry_after_ms,
        market_context_expire_at_ms,
    });

    let context_flag_packet = build_context_flag_packet(
        &structured_packet,
        flag_packet_id,
        packet_id,
        cluster_id,
        time_relevance_window,
        &confidence_band,
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
