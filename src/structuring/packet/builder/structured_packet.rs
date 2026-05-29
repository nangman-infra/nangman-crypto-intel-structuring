use crate::models::constants::STRUCTURED_PACKET_SCHEMA_VERSION;
use crate::models::market::MarketContextSnapshot;
use crate::models::output::{
    ConfidenceBand, ContradictionFlag, EventType, StructuredIntelPacket, TerminalDecision,
    TimeRelevanceWindow,
};
use crate::models::raw::RawIntelEvent;
use crate::structuring::router::StructuringDecision;

use super::super::evidence::{
    evidence_quality_reasons, metric_evidence, source_independence_summary, source_quality_summary,
    symbol_resolution_trace, text_evidence,
};
use super::super::ids::market_context_ref;

pub(super) struct StructuredPacketInput<'a> {
    pub(super) event: &'a RawIntelEvent,
    pub(super) decision: &'a StructuringDecision,
    pub(super) market_context: MarketContextSnapshot,
    pub(super) packet_id: String,
    pub(super) packet_family_id: String,
    pub(super) cluster_id: String,
    pub(super) event_timestamp_ms: i64,
    pub(super) structured_at_ms: i64,
    pub(super) decision_available_at_ms: i64,
    pub(super) normalized_symbols: Vec<String>,
    pub(super) symbol_confidence_band: ConfidenceBand,
    pub(super) event_type: EventType,
    pub(super) topic_summary: String,
    pub(super) stance_summary: String,
    pub(super) risk_summary: String,
    pub(super) regime_hint: String,
    pub(super) scenario_hint: String,
    pub(super) confidence_band: ConfidenceBand,
    pub(super) novelty_score: f64,
    pub(super) time_relevance_window: TimeRelevanceWindow,
    pub(super) contradiction_flags: Vec<ContradictionFlag>,
    pub(super) terminal_decision: TerminalDecision,
    pub(super) evidence_sentences: Vec<String>,
    pub(super) market_context_retry_after_ms: Option<i64>,
    pub(super) market_context_expire_at_ms: Option<i64>,
}

pub(super) fn build_structured_packet(input: StructuredPacketInput<'_>) -> StructuredIntelPacket {
    StructuredIntelPacket {
        packet_id: input.packet_id,
        packet_family_id: input.packet_family_id,
        raw_event_id: input.event.event_id.clone(),
        event_timestamp_ms: input.event_timestamp_ms,
        revision: 0,
        supersedes_packet_id: None,
        cluster_id: input.cluster_id,
        source_event_ids: vec![input.event.event_id.clone()],
        published_at_ms: input.event.published_at_ms,
        fetched_at_ms: input.event.fetched_at_ms,
        structured_at_ms: input.structured_at_ms,
        decision_available_at_ms: input.decision_available_at_ms,
        normalized_symbols: input.normalized_symbols.clone(),
        symbol_confidence_band: input.symbol_confidence_band.clone(),
        symbol_resolution_trace: symbol_resolution_trace(
            input.event,
            &input.normalized_symbols,
            &input.symbol_confidence_band,
        ),
        event_type: input.event_type,
        topic_summary: input.topic_summary,
        stance_summary: input.stance_summary,
        risk_summary: input.risk_summary,
        regime_hint: input.regime_hint,
        scenario_hint: input.scenario_hint,
        confidence_band: input.confidence_band,
        novelty_score: input.novelty_score,
        time_relevance_window: input.time_relevance_window,
        contradiction_flags: input.contradiction_flags,
        source_quality_summary: source_quality_summary(input.event, input.structured_at_ms),
        source_independence_summary: source_independence_summary(input.event),
        text_evidence: text_evidence(input.event, &input.evidence_sentences),
        metric_evidence: metric_evidence(input.event, &input.normalized_symbols),
        evidence_quality_reasons: evidence_quality_reasons(
            input.event,
            &input.normalized_symbols,
            &input.market_context,
        ),
        market_context_status: input.market_context.status.clone(),
        market_context_retry_after_ms: input.market_context_retry_after_ms,
        market_context_expire_at_ms: input.market_context_expire_at_ms,
        market_context_terminal_reason: None,
        market_context_ref: market_context_ref(&input.market_context),
        model_tier_used: input.decision.model_tier_used.clone(),
        terminal_decision: input.terminal_decision,
        evidence_sentences: input.evidence_sentences,
        market_context: input.market_context,
        schema_version: STRUCTURED_PACKET_SCHEMA_VERSION.to_owned(),
    }
}
