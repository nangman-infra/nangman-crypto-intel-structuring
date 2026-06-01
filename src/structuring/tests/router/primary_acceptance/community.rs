use super::*;

#[tokio::test]
async fn community_reaction_uses_primary_without_escalation_when_evidence_is_direct() {
    let mut event = event();
    event.source_id = "social_hackernews_solana_rss".to_owned();
    event.source_category = "social".to_owned();
    event.title = "SOL developer discussion gains attention".to_owned();
    event.body = "SOL developer discussion gains attention from the community.".to_owned();
    event.symbol_candidates = vec!["SOL".to_owned()];
    event.event_category_hint = Some("community_reaction".to_owned());
    event.content_kind = Some("community_reaction".to_owned());
    event.content_quality = Some("full_text".to_owned());
    event.content_quality_score = Some(80);
    event.source_quality = Some("community_reaction".to_owned());
    event.source_relevance_scope = Some("direct_asset".to_owned());
    event.direct_asset_count = Some(1);
    event.matched_asset_count = Some(1);
    let mut primary = response_with_evidence(
        0.86,
        TerminalDecision::HighConfidenceStructured,
        "SOL developer discussion gains attention from the community",
    );
    primary.event_type = EventType::SocialHype;
    primary.normalized_symbols = vec!["SOL".to_owned()];
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(primary),
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
