use super::super::super::env::{env_i64, env_or, env_usize};
use super::super::super::{
    DEFAULT_MARKET_CONTEXT_EXPIRE_AFTER_MS, DEFAULT_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS,
    DEFAULT_MARKET_CONTEXT_RETRY_INTERVAL_MS, DEFAULT_MARKET_CONTEXT_STALE_AFTER_MS,
    ProcessingConfig,
};
use crate::error::AppResult;
use crate::models::constants::STRUCTURING_POLICY_VERSION;

pub(in crate::config::defaults) fn processing_config() -> AppResult<ProcessingConfig> {
    Ok(ProcessingConfig {
        structuring_policy_version: env_or(
            "INTEL_L1_STRUCTURING_POLICY_VERSION",
            STRUCTURING_POLICY_VERSION,
        ),
        chunk_max_records: env_usize("INTEL_L1_CHUNK_MAX_RECORDS", 1000)?,
        market_context_window_radius: env_i64("INTEL_L1_MARKET_CONTEXT_RADIUS_WINDOWS", 1)?,
        market_context_latest_before_lookback_ms: env_i64(
            "INTEL_L1_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS",
            DEFAULT_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS,
        )?,
        market_context_stale_after_ms: env_i64(
            "INTEL_L1_MARKET_CONTEXT_STALE_AFTER_MS",
            DEFAULT_MARKET_CONTEXT_STALE_AFTER_MS,
        )?,
        market_context_retry_interval_ms: env_i64(
            "INTEL_L1_MARKET_CONTEXT_RETRY_INTERVAL_MS",
            DEFAULT_MARKET_CONTEXT_RETRY_INTERVAL_MS,
        )?,
        market_context_expire_after_ms: env_i64(
            "INTEL_L1_MARKET_CONTEXT_EXPIRE_AFTER_MS",
            DEFAULT_MARKET_CONTEXT_EXPIRE_AFTER_MS,
        )?,
        max_raw_body_chars: env_usize("INTEL_L1_MAX_RAW_BODY_CHARS", 20_000)?,
        story_member_scan_limit: env_usize("INTEL_L1_STORY_MEMBER_SCAN_LIMIT", 128)?,
    })
}
