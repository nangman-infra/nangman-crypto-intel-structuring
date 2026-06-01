use crate::error::{AppError, AppResult};
use aws_sdk_s3::primitives::ByteStream;

pub(super) async fn collect_body_bytes(
    body: ByteStream,
    error_context: String,
) -> AppResult<Vec<u8>> {
    Ok(body
        .collect()
        .await
        .map_err(|error| AppError::aws(format!("{error_context}: {error}")))?
        .into_bytes()
        .to_vec())
}
