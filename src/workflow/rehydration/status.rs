use super::MarketContextRehydrationOptions;
use crate::error::AppError;
use crate::models::market::MarketContextStatus;
use crate::models::output::StructuredIntelPacket;

pub(super) const TERMINAL_MISSING_MARKET_CONTEXT: &str = "terminal_missing_market_context";

pub(super) fn should_attempt_market_context_refresh(
    packet: &StructuredIntelPacket,
    options: &MarketContextRehydrationOptions,
) -> bool {
    matches!(
        packet.market_context_status,
        MarketContextStatus::Pending | MarketContextStatus::StaleButUsable
    ) || is_terminal_missing_market_context_reopen_candidate(packet, options)
}

pub(super) fn is_terminal_missing_market_context_reopen_candidate(
    packet: &StructuredIntelPacket,
    options: &MarketContextRehydrationOptions,
) -> bool {
    options.include_terminal_missing_market_context
        && packet.market_context_status == MarketContextStatus::Unavailable
        && packet.market_context_terminal_reason.as_deref() == Some(TERMINAL_MISSING_MARKET_CONTEXT)
}

pub(super) fn refreshed_context_warrants_revision(
    current: &MarketContextStatus,
    refreshed: &MarketContextStatus,
    terminal_reopen: bool,
) -> bool {
    if !refreshed.is_any_available() {
        return false;
    }
    if terminal_reopen {
        return true;
    }
    match current {
        MarketContextStatus::Pending => true,
        MarketContextStatus::StaleButUsable => {
            !matches!(refreshed, MarketContextStatus::StaleButUsable)
        }
        _ => false,
    }
}

pub(super) fn is_record_level_rehydration_error(error: &AppError) -> bool {
    matches!(error, AppError::Json(_) | AppError::Validation(_))
}
