use crate::error::{AppError, AppResult};
use crate::models::constants::{
    MARKET_L1_MANIFEST_SCHEMA_VERSION, MARKET_L1_POINTER_SCHEMA_VERSION,
    MARKET_L1_REPORT_SCHEMA_VERSION, MARKET_L1_SLICE_SCHEMA_VERSION,
};
use crate::models::market::{MarketL1IndexPointer, MarketL1Manifest, MarketL1Report};

pub(super) fn validate_pointer(
    pointer: &MarketL1IndexPointer,
    requested_start_ms: i64,
    requested_end_ms: i64,
) -> AppResult<()> {
    if pointer.schema_version != MARKET_L1_POINTER_SCHEMA_VERSION {
        return Err(AppError::validation(format!(
            "Market-L1 pointer schema mismatch: {}",
            pointer.schema_version
        )));
    }
    if pointer.status != "success" {
        return Err(AppError::validation(format!(
            "Market-L1 pointer status is not success: {}",
            pointer.status
        )));
    }
    if pointer.schema_version_emitted != MARKET_L1_SLICE_SCHEMA_VERSION {
        return Err(AppError::validation(format!(
            "Market-L1 pointer emitted schema mismatch: {}",
            pointer.schema_version_emitted
        )));
    }
    validate_time_range(
        pointer.input_time_range_start_ms,
        pointer.input_time_range_end_ms,
        requested_start_ms,
        requested_end_ms,
        "pointer",
    )
}

pub(super) fn validate_manifest(
    pointer: &MarketL1IndexPointer,
    manifest: &MarketL1Manifest,
    requested_start_ms: i64,
    requested_end_ms: i64,
) -> AppResult<()> {
    if manifest.schema_version != MARKET_L1_MANIFEST_SCHEMA_VERSION {
        return Err(AppError::validation(format!(
            "Market-L1 manifest schema mismatch: {}",
            manifest.schema_version
        )));
    }
    if manifest.l1_run_id != pointer.l1_run_id {
        return Err(AppError::validation("Market-L1 manifest run id mismatch"));
    }
    if manifest.status != "success" {
        return Err(AppError::validation(format!(
            "Market-L1 manifest status is not success: {}",
            manifest.status
        )));
    }
    if manifest.schema_version_emitted != MARKET_L1_SLICE_SCHEMA_VERSION {
        return Err(AppError::validation(format!(
            "Market-L1 manifest emitted schema mismatch: {}",
            manifest.schema_version_emitted
        )));
    }
    validate_time_range(
        manifest.input_time_range_start_ms,
        manifest.input_time_range_end_ms,
        requested_start_ms,
        requested_end_ms,
        "manifest",
    )?;
    if manifest.output_object_keys.is_empty() {
        return Err(AppError::validation(
            "Market-L1 manifest output_object_keys empty",
        ));
    }
    if manifest.output_record_count != manifest.slice_count_total {
        return Err(AppError::validation(
            "Market-L1 manifest output count mismatch",
        ));
    }
    Ok(())
}

pub(super) fn validate_report(
    report: &MarketL1Report,
    manifest: &MarketL1Manifest,
    manifest_key: &str,
) -> AppResult<()> {
    if report.schema_version != MARKET_L1_REPORT_SCHEMA_VERSION {
        return Err(AppError::validation(format!(
            "Market-L1 report schema mismatch: {}",
            report.schema_version
        )));
    }
    if report.l1_run_id != manifest.l1_run_id {
        return Err(AppError::validation("Market-L1 report run id mismatch"));
    }
    if report.status != manifest.status {
        return Err(AppError::validation("Market-L1 report status mismatch"));
    }
    if report.input_time_range_start_ms != manifest.input_time_range_start_ms
        || report.input_time_range_end_ms != manifest.input_time_range_end_ms
    {
        return Err(AppError::validation("Market-L1 report time range mismatch"));
    }
    if report.schema_version_emitted != manifest.schema_version_emitted {
        return Err(AppError::validation(
            "Market-L1 report emitted schema mismatch",
        ));
    }
    if report.manifest_key != manifest_key {
        return Err(AppError::validation(
            "Market-L1 report manifest key mismatch",
        ));
    }
    if report.output_object_keys != manifest.output_object_keys {
        return Err(AppError::validation(
            "Market-L1 report output_object_keys mismatch",
        ));
    }
    if report.market_data_quality_summary_key != manifest.market_data_quality_summary_key
        || report.market_feature_delta_key != manifest.market_feature_delta_key
        || report.market_feature_delta_summary_key != manifest.market_feature_delta_summary_key
        || report.market_regime_context_key != manifest.market_regime_context_key
        || report.symbol_universe_snapshot_key != manifest.symbol_universe_snapshot_key
    {
        return Err(AppError::validation(
            "Market-L1 report projection object key mismatch",
        ));
    }
    Ok(())
}

fn validate_time_range(
    actual_start_ms: i64,
    actual_end_ms: i64,
    expected_start_ms: i64,
    expected_end_ms: i64,
    label: &str,
) -> AppResult<()> {
    if actual_start_ms <= expected_start_ms && actual_end_ms >= expected_end_ms {
        Ok(())
    } else {
        Err(AppError::validation(format!(
            "Market-L1 {label} time range mismatch actual={actual_start_ms}-{actual_end_ms} expected={expected_start_ms}-{expected_end_ms}"
        )))
    }
}
