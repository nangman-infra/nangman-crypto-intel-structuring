use super::build_market_l1_read_plan;
use super::validation::validate_pointer;
use crate::models::constants::{
    MARKET_L1_MANIFEST_SCHEMA_VERSION, MARKET_L1_POINTER_SCHEMA_VERSION,
    MARKET_L1_REPORT_SCHEMA_VERSION, MARKET_L1_SLICE_SCHEMA_VERSION,
};
use crate::models::market::{MarketL1IndexPointer, MarketL1Manifest, MarketL1Report};

fn pointer() -> MarketL1IndexPointer {
    MarketL1IndexPointer {
        schema_version: MARKET_L1_POINTER_SCHEMA_VERSION.to_owned(),
        canonical_manifest_key: "runs/run_id=r/manifest.json".to_owned(),
        l1_run_id: "r".to_owned(),
        status: "success".to_owned(),
        finished_at_ms: 3,
        input_time_range_start_ms: 0,
        input_time_range_end_ms: 900_000,
        indexed_window_start_ms: Some(1_000),
        indexed_window_end_ms: Some(2_000),
        schema_version_emitted: MARKET_L1_SLICE_SCHEMA_VERSION.to_owned(),
    }
}

fn manifest() -> MarketL1Manifest {
    MarketL1Manifest {
        schema_version: MARKET_L1_MANIFEST_SCHEMA_VERSION.to_owned(),
        l1_run_id: "r".to_owned(),
        status: "success".to_owned(),
        input_time_range_start_ms: 0,
        input_time_range_end_ms: 900_000,
        schema_version_emitted: MARKET_L1_SLICE_SCHEMA_VERSION.to_owned(),
        report_key: "normalization_report/run_id=r/report.json".to_owned(),
        output_object_keys: vec!["normalized_market_slice/a.parquet".to_owned()],
        market_data_quality_summary_key: Some(
            "market_data_quality_summary/run_id=r/summary.json".to_owned(),
        ),
        market_feature_delta_key: Some("market_feature_delta/run_id=r/delta.json".to_owned()),
        market_feature_delta_summary_key: Some(
            "market_feature_delta_summary/run_id=r/summary.json".to_owned(),
        ),
        market_regime_context_key: Some("market_regime_context/run_id=r/context.json".to_owned()),
        symbol_universe_snapshot_key: Some(
            "symbol_universe_snapshot/run_id=r/snapshot.json".to_owned(),
        ),
        output_record_count: 1,
        slice_count_total: 1,
        finished_at_ms: 3,
    }
}

fn report() -> MarketL1Report {
    MarketL1Report {
        schema_version: MARKET_L1_REPORT_SCHEMA_VERSION.to_owned(),
        l1_run_id: "r".to_owned(),
        input_time_range_start_ms: 0,
        input_time_range_end_ms: 900_000,
        run_mode: "LIVE".to_owned(),
        fallback_alert: false,
        input_schema_versions: vec!["raw_market_event_v2".to_owned()],
        input_local_object_count: 0,
        input_s3_object_count: 1,
        input_object_keys: vec!["raw_market_event/a.parquet".to_owned()],
        input_record_count: 1,
        duplicate_event_count: 0,
        invalid_event_count: 0,
        payload_hash_mismatch_count: 0,
        slice_count_total: 1,
        slice_count_complete: 1,
        slice_count_partial: 0,
        slice_count_incomplete: 0,
        slice_count_reference_only: 0,
        output_object_keys: vec!["normalized_market_slice/a.parquet".to_owned()],
        market_data_quality_summary_key: Some(
            "market_data_quality_summary/run_id=r/summary.json".to_owned(),
        ),
        market_feature_delta_key: Some("market_feature_delta/run_id=r/delta.json".to_owned()),
        market_feature_delta_summary_key: Some(
            "market_feature_delta_summary/run_id=r/summary.json".to_owned(),
        ),
        market_regime_context_key: Some("market_regime_context/run_id=r/context.json".to_owned()),
        symbol_universe_snapshot_key: Some(
            "symbol_universe_snapshot/run_id=r/snapshot.json".to_owned(),
        ),
        status: "success".to_owned(),
        failure_reason: None,
        manifest_key: "runs/run_id=r/manifest.json".to_owned(),
        started_at_ms: 1,
        finished_at_ms: 3,
        runner_git_sha: "test".to_owned(),
        runner_git_dirty: false,
        runner_build_profile: "debug".to_owned(),
        schema_version_emitted: MARKET_L1_SLICE_SCHEMA_VERSION.to_owned(),
    }
}

#[test]
fn blocks_non_success_pointer() {
    let mut pointer = pointer();
    pointer.status = "partial".to_owned();
    assert!(validate_pointer(&pointer, 0, 1000).is_err());
}

#[test]
fn accepts_pointer_covering_requested_window() {
    assert!(validate_pointer(&pointer(), 1_000, 2_000).is_ok());
}

#[test]
fn rejects_pointer_outside_requested_window() {
    assert!(validate_pointer(&pointer(), 900_000, 901_000).is_err());
}

#[test]
fn builds_read_plan_for_window_inside_run_range() {
    let plan = build_market_l1_read_plan(
        &pointer(),
        &manifest(),
        &report(),
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
