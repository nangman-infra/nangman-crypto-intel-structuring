use crate::models::constants::QUARANTINE_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuarantineEvent {
    pub schema_version: String,
    pub quarantine_id: String,
    pub raw_event_id: Option<String>,
    pub observed_at_ms: i64,
    pub failure_class: String,
    pub retryable: bool,
    pub reason: String,
}

impl QuarantineEvent {
    pub fn new(
        quarantine_id: String,
        raw_event_id: Option<String>,
        observed_at_ms: i64,
        failure_class: impl Into<String>,
        retryable: bool,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: QUARANTINE_SCHEMA_VERSION.to_owned(),
            quarantine_id,
            raw_event_id,
            observed_at_ms,
            failure_class: failure_class.into(),
            retryable,
            reason: reason.into(),
        }
    }
}
