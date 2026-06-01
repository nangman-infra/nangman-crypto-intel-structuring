use crate::error::{AppError, AppResult};

pub(super) fn validate_time_range(
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
