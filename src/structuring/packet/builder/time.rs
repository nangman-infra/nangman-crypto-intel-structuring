use crate::models::market::MarketContextSnapshot;
use crate::models::output::{RelevanceDecayHint, TimeRelevanceWindow};
use crate::models::raw::RawIntelEvent;

pub(super) fn pending_market_context_schedule(
    market_context: &MarketContextSnapshot,
    decision_available_at_ms: i64,
    retry_interval_ms: i64,
    expire_after_ms: i64,
) -> (Option<i64>, Option<i64>) {
    if market_context.status.is_pending_or_unavailable() {
        (
            Some(decision_available_at_ms.saturating_add(retry_interval_ms.max(1))),
            Some(decision_available_at_ms.saturating_add(expire_after_ms.max(1))),
        )
    } else {
        (None, None)
    }
}

pub(super) fn decision_available_at_ms(event: &RawIntelEvent, structured_at_ms: i64) -> i64 {
    [
        event.published_at_ms,
        Some(event.fetched_at_ms),
        Some(event.observed_at_ms),
        Some(structured_at_ms),
    ]
    .into_iter()
    .flatten()
    .max()
    .unwrap_or(structured_at_ms)
}

pub(super) fn time_window(start_basis_ms: i64, decay: RelevanceDecayHint) -> TimeRelevanceWindow {
    let width_ms = match decay {
        RelevanceDecayHint::Minutes => 30 * 60 * 1000,
        RelevanceDecayHint::Hours => 6 * 60 * 60 * 1000,
        RelevanceDecayHint::Day => 24 * 60 * 60 * 1000,
        RelevanceDecayHint::MultiDay => 3 * 24 * 60 * 60 * 1000,
        RelevanceDecayHint::Structural => 14 * 24 * 60 * 60 * 1000,
    };
    TimeRelevanceWindow {
        start_ms: start_basis_ms,
        end_ms: start_basis_ms.saturating_add(width_ms),
        relevance_decay_hint: decay,
    }
}
