use crate::error::{AppError, AppResult};
use crate::models::constants::MARKET_L1_REPORT_SCHEMA_VERSION;
use crate::models::market::{MarketL1Manifest, MarketL1Report};

pub(in crate::admission::market_l1) fn validate_report(
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
