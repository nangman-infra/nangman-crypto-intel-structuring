use crate::models::market::{MarketContextSnapshot, MarketContextStatus};

pub(in crate::structuring::tests) fn market_context() -> MarketContextSnapshot {
    MarketContextSnapshot {
        status: MarketContextStatus::AvailableSymbolContext,
        basis_timestamp_ms: Some(1),
        basis_kind: "published_at_ms".to_owned(),
        window_start_ms: Some(0),
        window_end_ms: Some(1000),
        manifest_key: Some("m".to_owned()),
        output_object_keys: vec!["o".to_owned()],
        market_data_quality_summary_key: Some("q".to_owned()),
        market_feature_delta_key: Some("d".to_owned()),
        market_feature_delta_summary_key: Some("ds".to_owned()),
        market_regime_context_key: Some("r".to_owned()),
        symbol_universe_snapshot_key: Some("u".to_owned()),
        symbol_summaries: Vec::new(),
        unavailable_reason: None,
    }
}

pub(in crate::structuring::tests) fn pending_market_context() -> MarketContextSnapshot {
    MarketContextSnapshot {
        status: MarketContextStatus::Pending,
        basis_timestamp_ms: Some(1),
        basis_kind: "published_at_ms".to_owned(),
        window_start_ms: None,
        window_end_ms: None,
        manifest_key: None,
        output_object_keys: Vec::new(),
        market_data_quality_summary_key: None,
        market_feature_delta_key: None,
        market_feature_delta_summary_key: None,
        market_regime_context_key: None,
        symbol_universe_snapshot_key: None,
        symbol_summaries: Vec::new(),
        unavailable_reason: Some("fixture pending".to_owned()),
    }
}

pub(in crate::structuring::tests) fn stale_market_context() -> MarketContextSnapshot {
    let mut context = market_context();
    context.status = MarketContextStatus::StaleButUsable;
    context.window_start_ms = Some(0);
    context.window_end_ms = Some(1000);
    context
}
