use super::ObjectStore;
use crate::error::{AppError, AppResult};
use aws_sdk_s3::primitives::ByteStream;
use aws_smithy_types::error::metadata::ProvideErrorMetadata;
use serde::Serialize;

impl ObjectStore {
    pub async fn put_json_if_absent<T: Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> AppResult<Vec<u8>> {
        let bytes = serde_json::to_vec_pretty(value)?;
        self.put_bytes_guarded(key, bytes.clone(), "application/json", true)
            .await?;
        Ok(bytes)
    }

    pub async fn put_json_idempotent<T: Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> AppResult<Vec<u8>> {
        let bytes = serde_json::to_vec_pretty(value)?;
        self.put_bytes_idempotent(key, bytes.clone(), "application/json")
            .await?;
        Ok(bytes)
    }

    pub async fn put_jsonl_if_absent<T: Serialize>(
        &self,
        key: &str,
        records: &[T],
    ) -> AppResult<Vec<u8>> {
        let (bytes, _) = crate::jsonl::build_jsonl_chunk(records)?;
        self.put_bytes_guarded(key, bytes.clone(), "application/x-ndjson", true)
            .await?;
        Ok(bytes)
    }

    pub async fn put_jsonl_idempotent<T: Serialize>(
        &self,
        key: &str,
        records: &[T],
    ) -> AppResult<Vec<u8>> {
        let (bytes, _) = crate::jsonl::build_jsonl_chunk(records)?;
        self.put_bytes_idempotent(key, bytes.clone(), "application/x-ndjson")
            .await?;
        Ok(bytes)
    }

    pub async fn put_bytes_if_absent(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &'static str,
    ) -> AppResult<()> {
        self.put_bytes_guarded(key, bytes, content_type, true).await
    }

    pub async fn put_bytes_idempotent(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &'static str,
    ) -> AppResult<()> {
        match self
            .put_bytes_guarded(key, bytes.clone(), content_type, true)
            .await
        {
            Ok(()) => Ok(()),
            Err(AppError::Validation(message)) if message.contains("object already exists") => {
                let existing = self.get_bytes(key).await?;
                if existing == bytes {
                    Ok(())
                } else {
                    Err(AppError::validation(format!(
                        "idempotency conflict bucket={} key={key}",
                        self.bucket
                    )))
                }
            }
            Err(error) => Err(error),
        }
    }

    async fn put_bytes_guarded(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &'static str,
        if_absent: bool,
    ) -> AppResult<()> {
        let mut request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(ByteStream::from(bytes));
        if if_absent {
            request = request.if_none_match("*");
        }
        request.send().await.map_err(|error| {
            let code = error.code().map(str::to_owned);
            let message = error.to_string();
            if if_absent && is_precondition_failure(code.as_deref(), &message) {
                AppError::validation(format!(
                    "object already exists bucket={} key={key}",
                    self.bucket
                ))
            } else {
                AppError::aws(format!(
                    "put_object bucket={} key={} error={message}",
                    self.bucket, key
                ))
            }
        })?;
        Ok(())
    }
}

pub(super) fn is_precondition_failure(code: Option<&str>, message: &str) -> bool {
    matches!(code, Some("PreconditionFailed"))
        || message.contains("PreconditionFailed")
        || message.contains("precondition")
        || message.contains("412")
}
