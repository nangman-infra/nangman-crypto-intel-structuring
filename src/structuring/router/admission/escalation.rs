use super::primary::is_low_value_terminal;
use super::quality::{
    is_derivatives_snapshot_hint, is_derivatives_snapshot_source, is_low_quality_broad_scan,
    is_numeric_market_snapshot, is_official_or_trusted_notice,
};
use crate::ai::contract::ModelStructuringResponse;
use crate::models::market::MarketContextSnapshot;
use crate::models::output::EventType;
use crate::models::raw::RawIntelEvent;
use crate::structuring::rule::RuleAssessment;

pub(in crate::structuring::router) fn escalation_admission_allows(
    event: &RawIntelEvent,
    market_context: &MarketContextSnapshot,
    rule: &RuleAssessment,
    primary_response: Option<&ModelStructuringResponse>,
) -> bool {
    if is_single_numeric_funding_snapshot(event, rule, primary_response) {
        return false;
    }

    if is_numeric_market_snapshot(event) && market_context.status.is_pending_or_unavailable() {
        return false;
    }

    if is_numeric_market_snapshot(event)
        && !market_context.status.supports_numeric_snapshot_escalation()
    {
        return false;
    }

    if is_low_quality_broad_scan(event) && !rule.high_risk && !is_official_or_trusted_notice(event)
    {
        return false;
    }

    if let Some(response) = primary_response
        && is_low_value_terminal(response)
        && !rule.high_risk
        && !is_official_or_trusted_notice(event)
    {
        return false;
    }

    true
}

fn is_single_numeric_funding_snapshot(
    event: &RawIntelEvent,
    rule: &RuleAssessment,
    primary_response: Option<&ModelStructuringResponse>,
) -> bool {
    is_numeric_market_snapshot(event)
        && (matches!(rule.event_type, EventType::FundingShift)
            || primary_response
                .is_some_and(|response| matches!(response.event_type, EventType::FundingShift))
            || event
                .event_category_hint
                .as_deref()
                .is_some_and(is_derivatives_snapshot_hint)
            || is_derivatives_snapshot_source(event))
}
