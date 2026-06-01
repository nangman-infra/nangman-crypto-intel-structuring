use super::super::ObjectStore;
use super::super::validation::validate_object_key;
use super::precondition::is_precondition_failure;
use crate::error::{AppError, AppResult};
use aws_sdk_s3::primitives::ByteStream;
use aws_smithy_types::error::metadata::ProvideErrorMetadata;

pub(super) enum PutOutcome {
    Stored,
    AlreadyExists,
}

impl ObjectStore {
    pub(super) async fn put_bytes_if_none_match(
        &self,
        key: &str,
        bytes: Vec<u8>,
        content_type: &'static str,
    ) -> AppResult<PutOutcome> {
        validate_object_key(key, "S3 object key")?;
        let request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .body(ByteStream::from(bytes))
            .if_none_match("*");

        match request.send().await {
            Ok(_) => Ok(PutOutcome::Stored),
            Err(error) => {
                let code = error.code().map(str::to_owned);
                let message = error.to_string();
                if is_precondition_failure(code.as_deref(), &message) {
                    Ok(PutOutcome::AlreadyExists)
                } else {
                    Err(AppError::aws(format!(
                        "put_object bucket={} key={} error={message}",
                        self.bucket, key
                    )))
                }
            }
        }
    }

    pub(super) async fn ensure_existing_bytes_match(
        &self,
        key: &str,
        expected_bytes: &[u8],
    ) -> AppResult<()> {
        let existing = self.get_bytes(key).await?;
        if existing == expected_bytes {
            Ok(())
        } else {
            Err(AppError::validation(format!(
                "idempotency conflict bucket={} key={key}",
                self.bucket
            )))
        }
    }

    pub(super) fn object_already_exists_error(&self, key: &str) -> AppError {
        AppError::validation(format!(
            "object already exists bucket={} key={key}",
            self.bucket
        ))
    }
}
