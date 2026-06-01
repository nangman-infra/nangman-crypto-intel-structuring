use super::super::super::revision::build_revision_write_plan;
use super::super::fixtures::packet_with_market_status;
use crate::models::market::{MarketContextSnapshot, MarketContextStatus};
use crate::models::output::EvidenceQualityReason;

#[test]
fn terminal_revision_plan_keeps_missing_context_reason() {
    let packet = packet_with_market_status(MarketContextStatus::Pending);
    let plan = build_revision_write_plan(
        &packet,
        MarketContextSnapshot::unavailable("terminal_missing_market_context", "published_at_ms"),
        Some("terminal_missing_market_context".to_owned()),
        3_600_000,
        "test-output-bucket",
        "policy_v1",
    )
    .unwrap();

    assert_eq!(plan.manifest.status, "terminal_missing_market_context");
    assert_eq!(
        plan.revised_packet.market_context_status,
        MarketContextStatus::Unavailable
    );
    assert_eq!(
        plan.revised_packet
            .market_context_terminal_reason
            .as_deref(),
        Some("terminal_missing_market_context")
    );
    assert!(
        plan.revised_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::MarketContextMissing)
    );
}
