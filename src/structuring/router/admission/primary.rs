use super::quality::{
    is_derivatives_snapshot_hint, is_derivatives_snapshot_source, is_numeric_market_snapshot,
};
use crate::ai::contract::ModelStructuringResponse;
use crate::models::market::MarketContextSnapshot;
use crate::models::output::{EventType, TerminalDecision};
use crate::models::raw::RawIntelEvent;

pub(super) fn is_low_value_terminal(response: &ModelStructuringResponse) -> bool {
    matches!(
        response.terminal_decision,
        TerminalDecision::GeneralMarketContext
            | TerminalDecision::IrrelevantOrNoise
            | TerminalDecision::UnsupportedOrWeak
    )
}

pub(in crate::structuring::router) fn numeric_snapshot_can_stop_at_primary(
    event: &RawIntelEvent,
    market_context: &MarketContextSnapshot,
    response: &ModelStructuringResponse,
) -> bool {
    if !is_numeric_market_snapshot(event) {
        return false;
    }
    if is_single_numeric_funding_snapshot_response(event, response) {
        return true;
    }
    if market_context.status.is_any_available() {
        return false;
    }
    matches!(
        response.terminal_decision,
        TerminalDecision::LowConfidenceStructured
            | TerminalDecision::UnsupportedOrWeak
            | TerminalDecision::GeneralMarketContext
            | TerminalDecision::IrrelevantOrNoise
    ) && response.confidence_score <= 0.75
}

fn is_single_numeric_funding_snapshot_response(
    event: &RawIntelEvent,
    response: &ModelStructuringResponse,
) -> bool {
    is_numeric_market_snapshot(event)
        && (matches!(response.event_type, EventType::FundingShift)
            || event
                .event_category_hint
                .as_deref()
                .is_some_and(is_derivatives_snapshot_hint)
            || is_derivatives_snapshot_source(event))
}
