use super::ObjectStore;
use super::validation::validate_object_key;
use crate::error::{AppError, AppResult};

mod body;
mod exists;
mod list;
mod range;

use body::collect_body_bytes;
pub(super) use range::byte_range_header;

impl ObjectStore {
    pub async fn head_bucket(&self) -> AppResult<()> {
        self.client
            .head_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .map_err(|error| AppError::aws(format!("head_bucket {}: {error}", self.bucket)))?;
        Ok(())
    }

    pub async fn get_bytes(&self, key: &str) -> AppResult<Vec<u8>> {
        validate_object_key(key, "S3 object key")?;
        let output = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|error| {
                AppError::aws(format!(
                    "get_object bucket={} key={} error={error}",
                    self.bucket, key
                ))
            })?;
        collect_body_bytes(output.body, format!("collect body key={key}")).await
    }

    pub async fn get_byte_range(
        &self,
        key: &str,
        offset: usize,
        length: usize,
    ) -> AppResult<Vec<u8>> {
        validate_object_key(key, "S3 object key")?;
        let range = byte_range_header(offset, length)?;
        let output = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .range(&range)
            .send()
            .await
            .map_err(|error| {
                AppError::aws(format!(
                    "get_object_range bucket={} key={} range={} error={error}",
                    self.bucket, key, range
                ))
            })?;
        collect_body_bytes(output.body, format!("collect ranged body key={key}")).await
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> AppResult<T> {
        let bytes = self.get_bytes(key).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
