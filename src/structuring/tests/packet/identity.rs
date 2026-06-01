use super::super::{event, market_context};
use crate::models::market::MarketContextStatus;
use crate::models::output::{EvidenceQualityReason, ModelTierUsed};
use crate::structuring::packet::build_packet_set;
use crate::structuring::router::StructuringDecision;
use crate::structuring::rule::assess;

#[test]
fn packet_set_is_deterministic_for_redelivery_inputs() {
    let event = event();
    let decision = StructuringDecision {
        rule: assess(&event),
        model_response: None,
        model_tier_used: ModelTierUsed::RuleOnly,
        fallback_count: 0,
        primary_invocations: 0,
        escalation_invocations: 0,
    };

    let first = build_packet_set(
        &event,
        &decision,
        market_context(),
        "policy-v1",
        1234,
        300_000,
        21_600_000,
    );
    let second = build_packet_set(
        &event,
        &decision,
        market_context(),
        "policy-v1",
        1234,
        300_000,
        21_600_000,
    );

    assert_eq!(first, second);
    assert_eq!(
        first.structured_packet.source_quality_summary,
        "T1 source news_test freshness_ms=1233 content_quality=full_text score=80 source_quality=trusted_symbol_match relevance_scope=symbol_alias_match"
    );
    assert_eq!(first.structured_packet.published_at_ms, Some(1));
    assert_eq!(first.structured_packet.fetched_at_ms, 1);
    assert_eq!(first.structured_packet.raw_event_id, "intel_evt_test");
    assert_eq!(first.structured_packet.event_timestamp_ms, 1);
    assert_eq!(first.structured_packet.revision, 0);
    assert_eq!(first.structured_packet.structured_at_ms, 1234);
    assert_eq!(first.structured_packet.decision_available_at_ms, 1234);
    assert_eq!(
        first.structured_packet.market_context_status,
        MarketContextStatus::AvailableSymbolContext
    );
    assert_eq!(
        first
            .structured_packet
            .source_independence_summary
            .independent_source_count,
        1
    );
    assert_eq!(
        first.structured_packet.symbol_resolution_trace[0].canonical_symbol,
        Some("ABC".to_owned())
    );
    assert!(!first.structured_packet.text_evidence.is_empty());
    assert!(
        first
            .structured_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::SingleSourceOnly)
    );
    assert_eq!(first.context_flag_packet, second.context_flag_packet);
}
