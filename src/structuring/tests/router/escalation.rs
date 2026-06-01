use super::super::*;
use super::support::ScriptedProvider;
use crate::structuring::router::ModelRouter;

#[tokio::test]
async fn escalates_from_weak_primary_to_escalation() {
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(response(0.4, TerminalDecision::UnsupportedOrWeak)),
            primary_repair: None,
            escalation: Some(response(0.9, TerminalDecision::HighConfidenceStructured)),
        },
        policy(true),
    );
    let event = event();
    let decision = router.decide(&event, &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::Escalation);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 1);
}

#[tokio::test]
async fn drops_unsupported_escalation_output_instead_of_using_weak_model_content() {
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(response(0.4, TerminalDecision::UnsupportedOrWeak)),
            primary_repair: None,
            escalation: Some(response_with_evidence(
                0.9,
                TerminalDecision::HighConfidenceStructured,
                "Unsupported sentence from another source",
            )),
        },
        policy(true),
    );
    let decision = router.decide(&event(), &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::FallbackOnly);
    assert!(decision.model_response.is_none());
    assert_eq!(decision.fallback_count, 1);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 1);
}

#[tokio::test]
async fn escalation_failure_does_not_reuse_weak_primary_content() {
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(response(0.4, TerminalDecision::UnsupportedOrWeak)),
            primary_repair: None,
            escalation: None,
        },
        policy(true),
    );
    let decision = router.decide(&event(), &market_context()).await.unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::FallbackOnly);
    assert!(decision.model_response.is_none());
    assert_eq!(decision.fallback_count, 1);
}
