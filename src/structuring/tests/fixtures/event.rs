use crate::models::raw::RawIntelEvent;

pub(in crate::structuring::tests) fn event() -> RawIntelEvent {
    RawIntelEvent {
        event_id: "intel_evt_test".to_owned(),
        source_id: "news_test".to_owned(),
        source_category: "news".to_owned(),
        source_name: "News".to_owned(),
        fetched_at_ms: 1,
        published_at_ms: Some(1),
        observed_at_ms: 1,
        language: "en".to_owned(),
        title: "Protocol exploit investigation expands".to_owned(),
        body: "Protocol exploit investigation expands after the team confirmed the incident."
            .to_owned(),
        url: "https://example.com".to_owned(),
        author_or_channel: None,
        trust_tier: "T1".to_owned(),
        cadence_tier: "low".to_owned(),
        content_hash: "h".to_owned(),
        dedup_key: "d".to_owned(),
        symbol_candidates: vec!["ABC".to_owned()],
        event_category_hint: None,
        top50_relevance: "relevant".to_owned(),
        content_kind: Some("news_article".to_owned()),
        content_quality: Some("full_text".to_owned()),
        content_quality_score: Some(80),
        source_quality: Some("trusted_symbol_match".to_owned()),
        source_relevance_scope: Some("symbol_alias_match".to_owned()),
        direct_asset_count: Some(0),
        matched_asset_count: Some(1),
        historical_source_depth: None,
        backfill_window_start_ms: None,
        backfill_window_end_ms: None,
        source_time_range_verified: None,
        schema_version: "raw_intel_event_v1".to_owned(),
    }
}

pub(in crate::structuring::tests) fn numeric_snapshot_event() -> RawIntelEvent {
    let mut event = event();
    event.source_id = "derivatives_binance_usdm_open_interest_rest".to_owned();
    event.source_category = "funding".to_owned();
    event.title = "Binance USD-M open interest BTCUSDT".to_owned();
    event.body = r#"{"symbol":"BTCUSDT","open_interest":"1042","event_time_ms":1}"#.to_owned();
    event.symbol_candidates = vec!["BTC".to_owned()];
    event.event_category_hint = Some("open_interest_snapshot".to_owned());
    event.content_kind = Some("derivatives_snapshot".to_owned());
    event.content_quality = Some("numeric_observation".to_owned());
    event.content_quality_score = Some(63);
    event.source_quality = Some("market_snapshot".to_owned());
    event.source_relevance_scope = Some("symbol_alias_match".to_owned());
    event
}
