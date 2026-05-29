use super::*;

#[test]
fn builds_ranked_stable_evidence_pack() {
    let event = RawIntelEvent {
        event_id: "e".to_owned(),
        source_id: "s".to_owned(),
        source_category: "exchange_notice".to_owned(),
        source_name: "S".to_owned(),
        fetched_at_ms: 1,
        published_at_ms: Some(1),
        observed_at_ms: 1,
        language: "en".to_owned(),
        title: "ABC listing notice".to_owned(),
        body: "Noise sentence. ABC deposits will open tomorrow. ABC deposits will open tomorrow."
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
        content_kind: Some("exchange_notice".to_owned()),
        content_quality: Some("full_text".to_owned()),
        content_quality_score: Some(90),
        source_quality: Some("trusted_symbol_match".to_owned()),
        source_relevance_scope: Some("direct_asset".to_owned()),
        direct_asset_count: Some(1),
        matched_asset_count: Some(1),
        historical_source_depth: None,
        backfill_window_start_ms: None,
        backfill_window_end_ms: None,
        source_time_range_verified: None,
        schema_version: "raw_intel_event_v1".to_owned(),
    };

    let pack = build_evidence_pack_with_limits(&event, 3, 80);

    assert_eq!(pack[0].id, "E1");
    assert!(pack[0].text.contains("ABC"));
    assert_eq!(pack.len(), 3);
}
