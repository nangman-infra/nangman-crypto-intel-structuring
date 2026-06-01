use super::*;

#[tokio::test]
async fn weak_global_symbol_scan_claim_stays_on_primary() {
    let mut event = event();
    event.title = "ABC market headline circulates".to_owned();
    event.body = "ABC market headline circulates in a broad market digest.".to_owned();
    event.content_quality = Some("title_only".to_owned());
    event.content_quality_score = Some(30);
    event.source_quality = Some("global_symbol_scan".to_owned());
    event.source_relevance_scope = Some("global_symbol_scan".to_owned());
    event.direct_asset_count = Some(0);
    event.matched_asset_count = Some(1);
    let primary = response_with_evidence(
        0.88,
        TerminalDecision::HighConfidenceStructured,
        "ABC market headline circulates in a broad market digest",
    );
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(primary),
            primary_repair: None,
            escalation: Some(response_with_evidence(
                0.82,
                TerminalDecision::LowConfidenceStructured,
                "ABC market headline circulates in a broad market digest",
            )),
        },
        policy(true),
    );

    let decision = router.decide(&event, &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::Primary);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 0);
}

#[tokio::test]
async fn unsupported_low_quality_broad_scan_does_not_use_escalation() {
    let mut event = event();
    event.title = "ABC market headline circulates".to_owned();
    event.body = "ABC market headline circulates in a broad market digest.".to_owned();
    event.content_quality = Some("title_only".to_owned());
    event.content_quality_score = Some(30);
    event.source_quality = Some("global_symbol_scan".to_owned());
    event.source_relevance_scope = Some("global_symbol_scan".to_owned());
    event.direct_asset_count = Some(0);
    event.matched_asset_count = Some(1);
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(response_with_evidence(
                0.35,
                TerminalDecision::UnsupportedOrWeak,
                "ABC market headline circulates in a broad market digest",
            )),
            primary_repair: None,
            escalation: Some(response(0.95, TerminalDecision::HighConfidenceStructured)),
        },
        policy(true),
    );

    let decision = router.decide(&event, &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::Primary);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 0);
}
