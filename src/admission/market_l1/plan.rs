use super::validation::{validate_manifest, validate_pointer, validate_report};
use crate::error::AppResult;
use crate::models::market::{
    MarketL1IndexPointer, MarketL1Manifest, MarketL1ReadPlan, MarketL1Report,
};

pub fn build_market_l1_read_plan(
    pointer: &MarketL1IndexPointer,
    manifest: &MarketL1Manifest,
    report: &MarketL1Report,
    manifest_key: &str,
    requested_start_ms: i64,
    requested_end_ms: i64,
) -> AppResult<MarketL1ReadPlan> {
    validate_pointer(pointer, requested_start_ms, requested_end_ms)?;
    validate_manifest(
        pointer,
        manifest,
        manifest_key,
        requested_start_ms,
        requested_end_ms,
    )?;
    validate_report(report, manifest, manifest_key)?;
    Ok(MarketL1ReadPlan {
        l1_run_id: manifest.l1_run_id.clone(),
        manifest_key: manifest_key.to_owned(),
        report_key: manifest.report_key.clone(),
        output_object_keys: manifest.output_object_keys.clone(),
        market_data_quality_summary_key: manifest.market_data_quality_summary_key.clone(),
        market_feature_delta_key: manifest.market_feature_delta_key.clone(),
        market_feature_delta_summary_key: manifest.market_feature_delta_summary_key.clone(),
        market_regime_context_key: manifest.market_regime_context_key.clone(),
        symbol_universe_snapshot_key: manifest.symbol_universe_snapshot_key.clone(),
        input_time_range_start_ms: manifest.input_time_range_start_ms,
        input_time_range_end_ms: manifest.input_time_range_end_ms,
    })
}
