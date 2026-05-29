use crate::error::{AppError, AppResult};
use crate::hash::sha256_prefixed;
use crate::models::constants::RAW_EVENT_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

use super::pointer::RawIntelEventCreatedPointer;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RawIntelEvent {
    pub event_id: String,
    pub source_id: String,
    pub source_category: String,
    pub source_name: String,
    pub fetched_at_ms: i64,
    pub published_at_ms: Option<i64>,
    pub observed_at_ms: i64,
    pub language: String,
    pub title: String,
    pub body: String,
    pub url: String,
    pub author_or_channel: Option<String>,
    pub trust_tier: String,
    pub cadence_tier: String,
    pub content_hash: String,
    pub dedup_key: String,
    #[serde(default)]
    pub symbol_candidates: Vec<String>,
    pub event_category_hint: Option<String>,
    pub top50_relevance: String,
    #[serde(default)]
    pub content_kind: Option<String>,
    #[serde(default)]
    pub content_quality: Option<String>,
    #[serde(default)]
    pub content_quality_score: Option<u8>,
    #[serde(default)]
    pub source_quality: Option<String>,
    #[serde(default)]
    pub source_relevance_scope: Option<String>,
    #[serde(default)]
    pub direct_asset_count: Option<usize>,
    #[serde(default)]
    pub matched_asset_count: Option<usize>,
    #[serde(default)]
    pub historical_source_depth: Option<String>,
    #[serde(default)]
    pub backfill_window_start_ms: Option<i64>,
    #[serde(default)]
    pub backfill_window_end_ms: Option<i64>,
    #[serde(default)]
    pub source_time_range_verified: Option<bool>,
    pub schema_version: String,
}

impl RawIntelEvent {
    pub fn parse_verified(bytes: &[u8], pointer: &RawIntelEventCreatedPointer) -> AppResult<Self> {
        let actual_sha = sha256_prefixed(bytes);
        if actual_sha != pointer.storage_ref.content_sha256 {
            return Err(AppError::validation(format!(
                "raw record sha mismatch for {}",
                pointer.event_id
            )));
        }
        let event: Self = serde_json::from_slice(bytes)?;
        event.validate_against_pointer(pointer)?;
        Ok(event)
    }

    fn validate_against_pointer(&self, pointer: &RawIntelEventCreatedPointer) -> AppResult<()> {
        if self.schema_version != RAW_EVENT_SCHEMA_VERSION {
            return Err(AppError::validation(format!(
                "raw event schema mismatch: expected {RAW_EVENT_SCHEMA_VERSION}, got {}",
                self.schema_version
            )));
        }
        if self.event_id != pointer.event_id {
            return Err(AppError::validation(format!(
                "raw event id mismatch: pointer={} raw={}",
                pointer.event_id, self.event_id
            )));
        }
        if self.content_hash != pointer.content_hash {
            return Err(AppError::validation(format!(
                "raw content_hash mismatch for {}",
                self.event_id
            )));
        }
        Ok(())
    }

    pub fn evidence_text(&self, max_body_chars: usize) -> String {
        let body = if self.body.chars().count() > max_body_chars {
            self.body.chars().take(max_body_chars).collect::<String>()
        } else {
            self.body.clone()
        };
        format!("{}\n\n{}", self.title, body)
    }

    pub fn content_kind_or_unknown(&self) -> &str {
        self.content_kind.as_deref().unwrap_or("unknown")
    }

    pub fn content_quality_or_unknown(&self) -> &str {
        self.content_quality.as_deref().unwrap_or("unknown")
    }

    pub fn content_quality_score_label(&self) -> String {
        self.content_quality_score
            .map(|score| score.to_string())
            .unwrap_or_else(|| "unknown".to_owned())
    }

    pub fn source_quality_or_unknown(&self) -> &str {
        self.source_quality.as_deref().unwrap_or("unknown")
    }

    pub fn source_relevance_scope_or_unknown(&self) -> &str {
        self.source_relevance_scope.as_deref().unwrap_or("unknown")
    }
}
