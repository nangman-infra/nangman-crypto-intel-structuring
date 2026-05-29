use super::quality::{
    is_low_quality_broad_scan, is_numeric_market_snapshot, is_official_or_trusted_notice,
};
use crate::models::market::MarketContextSnapshot;
use crate::models::output::TerminalDecision;
use crate::models::raw::RawIntelEvent;
use crate::structuring::rule::RuleAssessment;

pub(in crate::structuring::router) fn should_bypass_models_for_cost(
    event: &RawIntelEvent,
    market_context: &MarketContextSnapshot,
    rule: &RuleAssessment,
) -> bool {
    if rule.high_risk || is_official_or_trusted_notice(event) {
        return false;
    }

    if is_numeric_market_snapshot(event) && market_context.status.is_pending_or_unavailable() {
        return true;
    }

    let weak_general_item = rule.normalized_symbols.is_empty()
        && matches!(
            rule.terminal_decision,
            TerminalDecision::GeneralMarketContext
                | TerminalDecision::IrrelevantOrNoise
                | TerminalDecision::UnsupportedOrWeak
        );
    weak_general_item && is_low_quality_broad_scan(event)
}
