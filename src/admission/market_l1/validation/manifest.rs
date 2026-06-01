use super::time_range::validate_time_range;
use crate::error::{AppError, AppResult};
use crate::models::constants::{MARKET_L1_MANIFEST_SCHEMA_VERSION, MARKET_L1_SLICE_SCHEMA_VERSION};
use crate::models::market::{MarketL1IndexPointer, MarketL1Manifest};

pub(in crate::admission::market_l1) fn validate_manifest(
    pointer: &MarketL1IndexPointer,
    manifest: &MarketL1Manifest,
    manifest_key: &str,
    requested_start_ms: i64,
    requested_end_ms: i64,
) -> AppResult<()> {
    if manifest_key != pointer.canonical_manifest_key {
        return Err(AppError::validation("Market-L1 manifest key mismatch"));
    }
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
