use crate::error::{AppError, AppResult};
use crate::models::constants::RAW_POINTER_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};

use super::RAW_STORAGE_KIND_AWS_S3_JSONL_RECORD;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawIntelEventCreatedPointer {
    pub schema_version: String,
    pub event_id: String,
    pub source_id: String,
    pub source_category: String,
    pub fetched_at_ms: i64,
    pub published_at_ms: Option<i64>,
    pub created_at_ms: i64,
    pub content_hash: String,
    pub dedup_key: String,
    #[serde(default)]
    pub symbol_candidates: Vec<String>,
    pub top50_relevance: String,
    pub storage_ref: RawIntelEventStorageRef,
}

impl RawIntelEventCreatedPointer {
    pub fn parse(bytes: &[u8]) -> AppResult<Self> {
        let pointer: Self = serde_json::from_slice(bytes)?;
        pointer.validate()?;
        Ok(pointer)
    }

    pub fn validate(&self) -> AppResult<()> {
        if self.schema_version != RAW_POINTER_SCHEMA_VERSION {
            return Err(AppError::validation(format!(
                "raw pointer schema mismatch: expected {RAW_POINTER_SCHEMA_VERSION}, got {}",
                self.schema_version
            )));
        }
        if self.event_id.trim().is_empty() {
            return Err(AppError::validation("raw pointer event_id is required"));
        }
        self.storage_ref.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RawIntelEventStorageRef {
    pub kind: String,
    pub endpoint_alias: String,
    pub bucket: String,
    pub key: String,
    pub line_number: usize,
    pub byte_offset: usize,
    pub byte_length: usize,
    pub content_sha256: String,
}

impl RawIntelEventStorageRef {
    pub fn validate(&self) -> AppResult<()> {
        if self.kind != RAW_STORAGE_KIND_AWS_S3_JSONL_RECORD {
            return Err(AppError::validation(format!(
                "unsupported raw storage kind: {}",
                self.kind
            )));
        }
        if self.bucket.trim().is_empty() || self.key.trim().is_empty() {
            return Err(AppError::validation("raw storage bucket/key are required"));
        }
        if self.byte_length == 0 {
            return Err(AppError::validation(
                "raw storage byte_length must be positive",
            ));
        }
        if !self.content_sha256.starts_with("sha256:") {
            return Err(AppError::validation(
                "raw storage content_sha256 must be sha256-prefixed",
            ));
        }
        Ok(())
    }
}
