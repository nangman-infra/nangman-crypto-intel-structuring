use super::*;

#[tokio::test]
async fn accepts_primary_for_low_value_unsupported_outputs() {
    let mut event = event();
    event.source_category = "news".to_owned();
    event.symbol_candidates.clear();
    event.title = "General market commentary continues".to_owned();
    event.body = "General market commentary continues without a specific coin catalyst.".to_owned();
    let mut primary = response_with_evidence(
        0.5,
        TerminalDecision::UnsupportedOrWeak,
        "General market commentary continues without a specific coin catalyst",
    );
    primary.event_type = EventType::Other;
    primary.normalized_symbols.clear();
    primary.symbol_confidence_band = ConfidenceBand::Weak;
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(primary),
            primary_repair: None,
            escalation: Some(response(0.9, TerminalDecision::HighConfidenceStructured)),
        },
        policy(true),
    );

    let decision = router.decide(&event, &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::Primary);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 0);
}
