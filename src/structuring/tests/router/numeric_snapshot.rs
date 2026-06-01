use super::super::*;
use super::support::ScriptedProvider;
use crate::structuring::router::ModelRouter;

#[tokio::test]
async fn numeric_market_snapshot_with_pending_context_bypasses_models() {
    let event = numeric_snapshot_event();
    let mut primary = numeric_snapshot_response();
    primary.event_type = EventType::FundingShift;
    primary.normalized_symbols = vec!["BTC".to_owned()];
    primary.symbol_confidence_band = ConfidenceBand::Moderate;
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(primary),
            primary_repair: None,
            escalation: Some(response(0.95, TerminalDecision::HighConfidenceStructured)),
        },
        policy(true),
    );

    let decision = router
        .decide(&event, &pending_market_context())
        .await
        .unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::RuleOnly);
    assert_eq!(decision.primary_invocations, 0);
    assert_eq!(decision.escalation_invocations, 0);
}

#[tokio::test]
async fn numeric_market_snapshot_with_stale_context_stays_on_primary() {
    let event = numeric_snapshot_event();
    let mut primary = numeric_snapshot_response();
    primary.event_type = EventType::FundingShift;
    primary.normalized_symbols = vec!["BTC".to_owned()];
    primary.symbol_confidence_band = ConfidenceBand::Moderate;
    let router = ModelRouter::new(
        ScriptedProvider {
            primary: Some(primary),
            primary_repair: None,
            escalation: Some(response(0.95, TerminalDecision::HighConfidenceStructured)),
        },
        policy(true),
    );

    let decision = router
        .decide(&event, &stale_market_context())
        .await
        .unwrap();

    assert_eq!(decision.model_tier_used, ModelTierUsed::Primary);
    assert_eq!(decision.primary_invocations, 1);
    assert_eq!(decision.escalation_invocations, 0);
}

#[tokio::test]
async fn single_numeric_funding_snapshot_never_uses_escalation_even_with_context() {
    let event = numeric_snapshot_event();
    let mut primary = numeric_snapshot_response();
    primary.event_type = EventType::FundingShift;
    primary.normalized_symbols = vec!["BTC".to_owned()];
    primary.symbol_confidence_band = ConfidenceBand::Moderate;
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

fn numeric_snapshot_response() -> ModelStructuringResponse {
    response_with_evidence(
        0.62,
        TerminalDecision::LowConfidenceStructured,
        r#"{"symbol":"BTCUSDT","open_interest":"1042","event_time_ms":1}"#,
    )
}
