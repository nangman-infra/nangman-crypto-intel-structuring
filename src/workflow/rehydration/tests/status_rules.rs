use super::super::MarketContextRehydrationOptions;
use super::super::status::{
    is_record_level_rehydration_error, refreshed_context_warrants_revision,
    should_attempt_market_context_refresh,
};
use super::fixtures::packet_with_market_status;
use crate::error::AppError;
use crate::models::market::MarketContextStatus;

#[test]
fn refresh_candidates_include_pending_and_stale_but_usable() {
    let options = MarketContextRehydrationOptions::default();
    let pending = packet_with_market_status(MarketContextStatus::Pending);
    let stale = packet_with_market_status(MarketContextStatus::StaleButUsable);
    let available = packet_with_market_status(MarketContextStatus::AvailableSymbolContext);

    assert!(should_attempt_market_context_refresh(&pending, &options));
    assert!(should_attempt_market_context_refresh(&stale, &options));
    assert!(!should_attempt_market_context_refresh(&available, &options));
}

#[test]
fn terminal_missing_context_rehydration_requires_explicit_opt_in() {
    let mut packet = packet_with_market_status(MarketContextStatus::Unavailable);
    packet.market_context_terminal_reason = Some("terminal_missing_market_context".to_owned());

    assert!(!should_attempt_market_context_refresh(
        &packet,
        &MarketContextRehydrationOptions::default()
    ));
    assert!(should_attempt_market_context_refresh(
        &packet,
        &MarketContextRehydrationOptions {
            include_terminal_missing_market_context: true
        }
    ));
}

#[test]
fn unrelated_terminal_context_is_not_reopened() {
    let mut packet = packet_with_market_status(MarketContextStatus::Unavailable);
    packet.market_context_terminal_reason = Some("source_contract_terminal".to_owned());

    assert!(!should_attempt_market_context_refresh(
        &packet,
        &MarketContextRehydrationOptions {
            include_terminal_missing_market_context: true
        }
    ));
}

#[test]
fn pending_context_accepts_any_available_refresh() {
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::Pending,
        &MarketContextStatus::StaleButUsable,
        false
    ));
    assert!(!refreshed_context_warrants_revision(
        &MarketContextStatus::Pending,
        &MarketContextStatus::Unavailable,
        false
    ));
}

#[test]
fn stale_context_requires_non_stale_available_refresh() {
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::StaleButUsable,
        &MarketContextStatus::NearestAvailable,
        false
    ));
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::StaleButUsable,
        &MarketContextStatus::AvailableSymbolContext,
        false
    ));
    assert!(!refreshed_context_warrants_revision(
        &MarketContextStatus::StaleButUsable,
        &MarketContextStatus::StaleButUsable,
        false
    ));
}

#[test]
fn terminal_reopen_accepts_any_available_refresh() {
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::Unavailable,
        &MarketContextStatus::StaleButUsable,
        true
    ));
    assert!(!refreshed_context_warrants_revision(
        &MarketContextStatus::Unavailable,
        &MarketContextStatus::Unavailable,
        true
    ));
}

#[test]
fn record_level_rehydration_errors_are_skippable() {
    let json_error = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    assert!(is_record_level_rehydration_error(&AppError::Json(
        json_error
    )));
    assert!(is_record_level_rehydration_error(&AppError::validation(
        "legacy packet"
    )));
    assert!(!is_record_level_rehydration_error(&AppError::aws(
        "object store unavailable"
    )));
}
