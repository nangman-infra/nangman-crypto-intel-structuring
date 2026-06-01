use super::super::super::{event, market_context};
use crate::models::output::ModelTierUsed;
use crate::structuring::packet::build_packet_set;
use crate::structuring::router::StructuringDecision;
use crate::structuring::rule::assess;

#[test]
fn direct_asset_full_text_preserves_source_excerpt_when_rule_has_no_event_evidence() {
    let mut event = event();
    event.event_id = "intel_evt_pepe_direct_html".to_owned();
    event.source_id = "project_pepe_official_html".to_owned();
    event.source_category = "project_notice".to_owned();
    event.source_name = "PEPE".to_owned();
    event.title = "PEPE".to_owned();
    event.body = "$PEPE is a coin for the people, forever. Fueled by pure memetic power, let $PEPE show you the way.".to_owned();
    event.symbol_candidates = vec!["PEPE".to_owned()];
    event.content_quality = Some("full_text".to_owned());
    event.content_quality_score = Some(88);
    event.source_quality = Some("trusted_symbol_match".to_owned());
    event.source_relevance_scope = Some("direct_asset".to_owned());
    event.direct_asset_count = Some(1);
    event.matched_asset_count = Some(1);
    let rule = assess(&event);
    assert!(rule.evidence_sentences.is_empty());
    let decision = StructuringDecision {
        rule,
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
        packet_set.structured_packet.text_evidence[0].evidence_kind,
        "source_excerpt"
    );
    assert!(
        packet_set.structured_packet.text_evidence[0]
            .evidence_text
            .contains("$PEPE")
    );
    assert!(packet_set.structured_packet.evidence_sentences.is_empty());
}

#[test]
fn title_only_direct_asset_without_explicit_evidence_does_not_create_source_excerpt() {
    let mut event = event();
    event.event_id = "intel_evt_title_only_direct_html".to_owned();
    event.source_id = "project_pepe_title_only".to_owned();
    event.source_category = "project_notice".to_owned();
    event.title = "PEPE".to_owned();
    event.body =
        "$PEPE is a coin for the people, forever, but this crawler only trusted the title."
            .to_owned();
    event.symbol_candidates = vec!["PEPE".to_owned()];
    event.content_quality = Some("title_only".to_owned());
    event.content_quality_score = Some(30);
    event.source_quality = Some("trusted_symbol_match".to_owned());
    event.source_relevance_scope = Some("direct_asset".to_owned());
    event.direct_asset_count = Some(1);
    event.matched_asset_count = Some(1);
    let rule = assess(&event);
    assert!(rule.evidence_sentences.is_empty());
    let decision = StructuringDecision {
        rule,
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

    assert!(packet_set.structured_packet.text_evidence.is_empty());
}
