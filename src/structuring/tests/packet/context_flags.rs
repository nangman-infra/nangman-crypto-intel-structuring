use super::super::{event, market_context, pending_market_context, response};
use crate::models::output::{ConfidenceBand, EventType, ModelTierUsed, TerminalDecision};
use crate::structuring::packet::build_packet_set;
use crate::structuring::router::StructuringDecision;
use crate::structuring::rule::assess;

#[test]
fn high_confidence_structured_packet_emits_context_flag() {
    let event = event();
    let decision = StructuringDecision {
        rule: assess(&event),
        model_response: Some(response(0.9, TerminalDecision::HighConfidenceStructured)),
        model_tier_used: ModelTierUsed::Primary,
        fallback_count: 0,
        primary_invocations: 1,
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

    assert!(packet_set.context_flag_packet.is_some());
    assert_eq!(packet_set.health_event.flag_packet_count, 1);
}

#[test]
fn low_confidence_structured_packet_does_not_emit_context_flag() {
    let event = event();
    let decision = StructuringDecision {
        rule: assess(&event),
        model_response: Some(response(0.7, TerminalDecision::LowConfidenceStructured)),
        model_tier_used: ModelTierUsed::Primary,
        fallback_count: 0,
        primary_invocations: 1,
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

    assert!(packet_set.context_flag_packet.is_none());
    assert_eq!(packet_set.health_event.flag_packet_count, 0);
}

#[test]
fn funding_shift_without_available_market_context_does_not_emit_context_flag() {
    let mut event = event();
    event.source_category = "funding".to_owned();
    event.content_quality = Some("numeric_observation".to_owned());
    event.source_quality = Some("market_snapshot".to_owned());
    let mut model = response(0.9, TerminalDecision::HighConfidenceStructured);
    model.event_type = EventType::FundingShift;
    model.normalized_symbols = vec!["ABC".to_owned()];
    let decision = StructuringDecision {
        rule: assess(&event),
        model_response: Some(model),
        model_tier_used: ModelTierUsed::Escalation,
        fallback_count: 0,
        primary_invocations: 1,
        escalation_invocations: 1,
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

    assert!(packet_set.context_flag_packet.is_none());
    assert_eq!(packet_set.health_event.flag_packet_count, 0);
}

#[test]
fn weak_symbol_packet_does_not_emit_context_flag() {
    let event = event();
    let mut weak_model = response(0.7, TerminalDecision::LowConfidenceStructured);
    weak_model.symbol_confidence_band = ConfidenceBand::Weak;
    let decision = StructuringDecision {
        rule: assess(&event),
        model_response: Some(weak_model),
        model_tier_used: ModelTierUsed::Primary,
        fallback_count: 0,
        primary_invocations: 1,
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

    assert!(packet_set.context_flag_packet.is_none());
    assert_eq!(packet_set.health_event.flag_packet_count, 0);
}
