use super::super::super::{event, market_context, pending_market_context, response};
use crate::models::output::{EventType, EvidenceQualityReason, ModelTierUsed, TerminalDecision};
use crate::structuring::packet::build_packet_set;
use crate::structuring::router::StructuringDecision;
use crate::structuring::rule::assess;

#[test]
fn numeric_snapshot_records_metric_guard_reasons() {
    let mut event = event();
    event.source_id = "derivatives_binance_usdm_open_interest_rest".to_owned();
    event.source_category = "funding".to_owned();
    event.body = r#"{"symbol":"BTCUSDT","open_interest":"1042","event_time_ms":1}"#.to_owned();
    event.content_quality = Some("numeric_observation".to_owned());
    event.source_quality = Some("market_snapshot".to_owned());
    event.event_category_hint = Some("open_interest_snapshot".to_owned());
    let mut model = response(0.7, TerminalDecision::LowConfidenceStructured);
    model.event_type = EventType::FundingShift;
    model.normalized_symbols = vec!["ABC".to_owned()];
    let decision = StructuringDecision {
        rule: assess(&event),
        model_response: Some(model),
        model_tier_used: ModelTierUsed::Primary,
        fallback_count: 0,
        primary_invocations: 1,
        escalation_invocations: 0,
    };

    let packet_set = build_packet_set(
        &event,
        &decision,
        pending_market_context(),
        "policy-v1",
        1234,
        300_000,
        21_600_000,
    );

    assert_eq!(packet_set.structured_packet.metric_evidence.len(), 1);
    assert_eq!(
        packet_set.structured_packet.metric_evidence[0].value,
        Some(1042.0)
    );
    assert!(
        packet_set
            .structured_packet
            .source_independence_summary
            .official_source_present
    );
    assert!(
        packet_set
            .structured_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::SingleNumericSnapshot)
    );
    assert!(
        packet_set
            .structured_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::BaselineMissing)
    );
    assert!(
        packet_set
            .structured_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::MarketContextMissing)
    );
    assert_eq!(
        packet_set.structured_packet.market_context_retry_after_ms,
        Some(301_234)
    );
    assert_eq!(
        packet_set.structured_packet.market_context_expire_at_ms,
        Some(21_601_234)
    );
}

#[test]
fn duplicate_or_syndicated_source_records_content_hash_guard() {
    let mut event = event();
    event.source_quality = Some("syndicated_duplicate".to_owned());
    let decision = StructuringDecision {
        rule: assess(&event),
        model_response: None,
        model_tier_used: ModelTierUsed::RuleOnly,
        fallback_count: 0,
        primary_invocations: 0,
        escalation_invocations: 0,
    };

    let packet_set = build_packet_set(
        &event,
        &decision,
        market_context(),
        "policy-v1",
        1234,
        300_000,
        21_600_000,
    );

    assert_eq!(
        packet_set
            .structured_packet
            .source_independence_summary
            .duplicate_content_hashes,
        vec![event.content_hash]
    );
    assert!(
        packet_set
            .structured_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::DuplicateOrSyndicatedSource)
    );
}
