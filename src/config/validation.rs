use super::Args;
use crate::error::{AppError, AppResult};

impl Args {
    pub(super) fn validate(&self) -> AppResult<()> {
        validate_non_empty(&self.nats.url, "NATS_URL")?;
        validate_real_config_value(&self.output_store.bucket, "INTEL_L1_OUTPUT_S3_BUCKET")?;
        validate_real_config_value(&self.market_l1_store.bucket, "INTEL_L1_MARKET_L1_BUCKET")?;
        validate_real_config_value(&self.raw_l0_store.bucket, "INTEL_L1_RAW_S3_BUCKET")?;
        if self.market_l1_window_ms <= 0 {
            return Err(AppError::config(
                "INTEL_L1_MARKET_WINDOW_MS must be positive",
            ));
        }
        if self.nats.ack_wait_secs == 0 {
            return Err(AppError::config(
                "INTEL_L1_RAW_ACK_WAIT_SECS must be positive",
            ));
        }
        if self.nats.max_deliver <= 0 {
            return Err(AppError::config(
                "INTEL_L1_RAW_MAX_DELIVER must be positive",
            ));
        }
        if self.nats.batch_size == 0 {
            return Err(AppError::config("INTEL_L1_RAW_BATCH_SIZE must be positive"));
        }
        validate_deliver_policy(&self.nats.raw_deliver_policy)?;
        if self.nats.output_stream_max_age_secs == 0 {
            return Err(AppError::config(
                "INTEL_L1_OUTPUT_STREAM_MAX_AGE_SECS must be positive",
            ));
        }
        if self.nats.output_stream_duplicate_window_secs == 0 {
            return Err(AppError::config(
                "INTEL_L1_OUTPUT_STREAM_DUPLICATE_WINDOW_SECS must be positive",
            ));
        }
        validate_ratio(
            self.model_policy.escalate_if_confidence_below,
            "INTEL_L1_ESCALATE_IF_CONFIDENCE_BELOW",
        )?;
        validate_ratio(
            self.model_policy.escalation_budget_ratio,
            "INTEL_L1_ESCALATION_BUDGET_RATIO",
        )?;
        if self.processing.chunk_max_records == 0 {
            return Err(AppError::config(
                "INTEL_L1_CHUNK_MAX_RECORDS must be positive",
            ));
        }
        if self.processing.story_member_scan_limit == 0 {
            return Err(AppError::config(
                "INTEL_L1_STORY_MEMBER_SCAN_LIMIT must be positive",
            ));
        }
        if self.processing.market_context_latest_before_lookback_ms <= 0 {
            return Err(AppError::config(
                "INTEL_L1_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS must be positive",
            ));
        }
        if self.processing.market_context_stale_after_ms <= 0 {
            return Err(AppError::config(
                "INTEL_L1_MARKET_CONTEXT_STALE_AFTER_MS must be positive",
            ));
        }
        if self.processing.market_context_retry_interval_ms <= 0 {
            return Err(AppError::config(
                "INTEL_L1_MARKET_CONTEXT_RETRY_INTERVAL_MS must be positive",
            ));
        }
        if self.processing.market_context_expire_after_ms <= 0 {
            return Err(AppError::config(
                "INTEL_L1_MARKET_CONTEXT_EXPIRE_AFTER_MS must be positive",
            ));
        }
        Ok(())
    }
}

fn validate_non_empty(value: &str, name: &str) -> AppResult<()> {
    if value.trim().is_empty() {
        Err(AppError::config(format!("{name} must not be empty")))
    } else {
        Ok(())
    }
}

fn validate_real_config_value(value: &str, name: &str) -> AppResult<()> {
    validate_non_empty(value, name)?;
    if value.contains('<') || value.contains('>') {
        return Err(AppError::config(format!(
            "{name} must be set to a real value, not a public-doc placeholder"
        )));
    }
    Ok(())
}

fn validate_ratio(value: f64, name: &str) -> AppResult<()> {
    if !(0.0..=1.0).contains(&value) {
        Err(AppError::config(format!("{name} must be between 0 and 1")))
    } else {
        Ok(())
    }
}

fn validate_deliver_policy(value: &str) -> AppResult<()> {
    match value {
        "all" | "new" | "last" | "last_per_subject" => Ok(()),
        other => Err(AppError::config(format!(
            "INTEL_L1_RAW_DELIVER_POLICY must be one of all,new,last,last_per_subject, got {other}"
        ))),
    }
}
