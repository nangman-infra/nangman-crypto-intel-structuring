use crate::models::market::{MarketContextSnapshot, MarketContextStatus, MarketSymbolSummary};

pub(super) fn merge_snapshots(
    mut snapshots: Vec<MarketContextSnapshot>,
) -> Option<MarketContextSnapshot> {
    if snapshots.is_empty() {
        return None;
    }
    let mut first = snapshots.remove(0);
    for snapshot in snapshots {
        first.status = merge_status(&first.status, &snapshot.status);
        first.output_object_keys.extend(snapshot.output_object_keys);
        if first.market_data_quality_summary_key.is_none() {
            first.market_data_quality_summary_key = snapshot.market_data_quality_summary_key;
        }
        if first.market_feature_delta_key.is_none() {
            first.market_feature_delta_key = snapshot.market_feature_delta_key;
        }
        if first.market_feature_delta_summary_key.is_none() {
            first.market_feature_delta_summary_key = snapshot.market_feature_delta_summary_key;
        }
        if first.market_regime_context_key.is_none() {
            first.market_regime_context_key = snapshot.market_regime_context_key;
        }
        if first.symbol_universe_snapshot_key.is_none() {
            first.symbol_universe_snapshot_key = snapshot.symbol_universe_snapshot_key;
        }
        first.symbol_summaries.extend(snapshot.symbol_summaries);
    }
    Some(first)
}

pub(super) fn context_status(
    requested_symbols: &[String],
    symbol_summaries: &[MarketSymbolSummary],
    window_start_ms: i64,
    basis_window_start_ms: i64,
    stale_after_ms: i64,
) -> MarketContextStatus {
    if requested_symbols.is_empty() {
        return MarketContextStatus::AvailableGeneralContext;
    }
    if symbol_summaries.is_empty() {
        return MarketContextStatus::AvailableGeneralContext;
    }
    if window_start_ms == basis_window_start_ms {
        MarketContextStatus::AvailableSymbolContext
    } else if basis_window_start_ms.saturating_sub(window_start_ms) > stale_after_ms {
        MarketContextStatus::StaleButUsable
    } else {
        MarketContextStatus::NearestAvailable
    }
}

fn merge_status(left: &MarketContextStatus, right: &MarketContextStatus) -> MarketContextStatus {
    use MarketContextStatus::*;
    match (left, right) {
        (AvailableSymbolContext, _) | (_, AvailableSymbolContext) => AvailableSymbolContext,
        (NearestAvailable, _) | (_, NearestAvailable) => NearestAvailable,
        (SymbolContextOnly, _) | (_, SymbolContextOnly) => SymbolContextOnly,
        (StaleButUsable, _) | (_, StaleButUsable) => StaleButUsable,
        (Available, _) | (_, Available) => Available,
        (AvailableGeneralContext, _) | (_, AvailableGeneralContext) => AvailableGeneralContext,
        (Pending, _) | (_, Pending) => Pending,
        (Unavailable, Unavailable) => Unavailable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_old_symbol_context_as_stale_but_usable() {
        let status = context_status(
            &["SUI".to_owned()],
            &[MarketSymbolSummary {
                symbol: "SUI".to_owned(),
                venue: "binance".to_owned(),
                window_start_ms: 0,
                window_end_ms: 1_000,
                mid_price: Some(1.0),
                spread_bps: Some(1.0),
                trade_count: 1,
                trade_volume: 10.0,
                slice_completeness: "complete".to_owned(),
            }],
            0,
            3_600_000,
            600_000,
        );
        assert_eq!(status, MarketContextStatus::StaleButUsable);
    }

    #[test]
    fn marks_nearby_symbol_context_as_nearest_available() {
        let status = context_status(
            &["SUI".to_owned()],
            &[MarketSymbolSummary {
                symbol: "SUI".to_owned(),
                venue: "binance".to_owned(),
                window_start_ms: 0,
                window_end_ms: 1_000,
                mid_price: Some(1.0),
                spread_bps: Some(1.0),
                trade_count: 1,
                trade_volume: 10.0,
                slice_completeness: "complete".to_owned(),
            }],
            0,
            300_000,
            600_000,
        );
        assert_eq!(status, MarketContextStatus::NearestAvailable);
    }
}
