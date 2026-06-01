use super::ObjectStore;
use crate::error::AppResult;
use serde::Serialize;

mod payload;
mod precondition;
mod request;

use payload::{JSON_CONTENT_TYPE, JSONL_CONTENT_TYPE, json_bytes, jsonl_bytes};
#[cfg(test)]
pub(super) use precondition::is_precondition_failure;
use request::PutOutcome;

impl ObjectStore {
    pub async fn put_json_if_absent<T: Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> AppResult<Vec<u8>> {
        let bytes = json_bytes(value)?;
        self.put_bytes_if_absent(key, bytes.clone(), JSON_CONTENT_TYPE)
            .await?;
        Ok(bytes)
    }

    pub async fn put_json_idempotent<T: Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> AppResult<Vec<u8>> {
        let bytes = json_bytes(value)?;
        self.put_bytes_idempotent(key, bytes.clone(), JSON_CONTENT_TYPE)
            .await?;
        Ok(bytes)
    }

    pub async fn put_jsonl_if_absent<T: Serialize>(
        &self,
        key: &str,
        records: &[T],
    ) -> AppResult<Vec<u8>> {
        let bytes = jsonl_bytes(records)?;
        self.put_bytes_if_absent(key, bytes.clone(), JSONL_CONTENT_TYPE)
            .await?;
        Ok(bytes)
    }

    pub async fn put_jsonl_idempotent<T: Serialize>(
        &self,
        key: &str,
        records: &[T],
    ) -> AppResult<Vec<u8>> {
        let bytes = jsonl_bytes(records)?;
        self.put_bytes_idempotent(key, bytes.clone(), JSONL_CONTENT_TYPE)
            .await?;
        Ok(bytes)
    }

    pub async fn put_bytes_if_absent(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &'static str,
    ) -> AppResult<()> {
        match self
            .put_bytes_if_none_match(key, bytes, content_type)
            .await?
        {
            PutOutcome::Stored => Ok(()),
            PutOutcome::AlreadyExists => Err(self.object_already_exists_error(key)),
        }
    }

    pub async fn put_bytes_idempotent(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &'static str,
    ) -> AppResult<()> {
        let expected_bytes = bytes.clone();
        match self
            .put_bytes_if_none_match(key, bytes, content_type)
            .await?
        {
            PutOutcome::Stored => Ok(()),
            PutOutcome::AlreadyExists => {
                self.ensure_existing_bytes_match(key, &expected_bytes).await
            }
        }
    }
}
