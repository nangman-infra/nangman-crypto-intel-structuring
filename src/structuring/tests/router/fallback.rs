use super::super::*;
use super::support::ScriptedProvider;
use crate::structuring::router::ModelRouter;

#[tokio::test]
async fn falls_back_when_models_disabled() {
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: None,
            primary_repair: None,
            escalation: None,
        },
        policy(false),
    );
    let decision = router.decide(&event(), &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::FallbackOnly);
    assert_eq!(decision.fallback_count, 1);
}

#[tokio::test]
async fn noncritical_high_impact_escalation_respects_escalation_budget() {
    let mut event = event();
    event.title = "ABC listing expands to a new venue".to_owned();
    event.body = "ABC listing expands to a new venue with limited supporting detail.".to_owned();
    event.symbol_candidates.clear();
    let mut primary = response_with_evidence(
        0.5,
        TerminalDecision::LowConfidenceStructured,
        "ABC listing expands to a new venue with limited supporting detail",
    );
    primary.event_type = EventType::Listing;
    primary.normalized_symbols = vec!["ABC".to_owned()];
    primary.symbol_confidence_band = ConfidenceBand::Moderate;
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(primary),
            primary_repair: None,
            escalation: Some(response(0.95, TerminalDecision::HighConfidenceStructured)),
        },
        policy_with_escalation_budget(true, 0.0),
    );

    let decision = router.decide(&event, &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::Primary);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 0);
}

#[tokio::test]
async fn noncritical_escalation_fallback_respects_escalation_budget() {
    let mut event = event();
    event.title = "ABC listing expands to a new venue".to_owned();
    event.body = "ABC listing expands to a new venue with limited supporting detail.".to_owned();
    event.symbol_candidates.clear();
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: None,
            primary_repair: None,
            escalation: Some(response(0.95, TerminalDecision::HighConfidenceStructured)),
        },
        policy_with_escalation_budget(true, 0.0),
    );

    let decision = router.decide(&event, &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::FallbackOnly);
    assert_eq!(decision.fallback_count, 1);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 0);
}
