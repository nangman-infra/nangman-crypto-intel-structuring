#[test]
fn rejects_manifest_key_that_does_not_match_pointer() {
    let error = build_market_l1_read_plan(
        &fixtures::pointer(),
        &fixtures::manifest(),
        &fixtures::report(),
        "runs/run_id=other/manifest.json",
        1_000,
        2_000,
    )
    .expect_err("manifest key must match pointer canonical manifest key")
    .to_string();

    assert!(error.contains("Market-L1 manifest key mismatch"));
}

#[test]
fn builds_read_plan_for_window_inside_run_range() {
    let plan = build_market_l1_read_plan(
        &fixtures::pointer(),
        &fixtures::manifest(),
        &fixtures::report(),
        "runs/run_id=r/manifest.json",
        1_000,
        2_000,
    )
    .unwrap();

    assert_eq!(plan.l1_run_id, "r");
    assert_eq!(
        plan.output_object_keys,
        vec!["normalized_market_slice/a.parquet"]
    );
    assert_eq!(
        plan.market_data_quality_summary_key,
        Some("market_data_quality_summary/run_id=r/summary.json".to_owned())
    );
    assert_eq!(
        plan.market_feature_delta_key,
        Some("market_feature_delta/run_id=r/delta.json".to_owned())
    );
    assert_eq!(
        plan.market_feature_delta_summary_key,
        Some("market_feature_delta_summary/run_id=r/summary.json".to_owned())
    );
    assert_eq!(
        plan.market_regime_context_key,
        Some("market_regime_context/run_id=r/context.json".to_owned())
    );
    assert_eq!(
        plan.symbol_universe_snapshot_key,
        Some("symbol_universe_snapshot/run_id=r/snapshot.json".to_owned())
    );
}
