use super::time_range::validate_time_range;
use crate::error::{AppError, AppResult};
use crate::models::constants::{MARKET_L1_POINTER_SCHEMA_VERSION, MARKET_L1_SLICE_SCHEMA_VERSION};
use crate::models::market::MarketL1IndexPointer;

pub(in crate::admission::market_l1) fn validate_pointer(
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
