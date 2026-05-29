use super::ObjectStore;
use crate::error::{AppError, AppResult};

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

    pub async fn object_exists(&self, key: &str) -> AppResult<bool> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(error) => {
                if error
                    .as_service_error()
                    .map(|service_error| service_error.is_not_found())
                    == Some(true)
                {
                    return Ok(false);
                }
                let message = error.to_string();
                if message.contains("NotFound")
                    || message.contains("404")
                    || message.contains("NoSuchKey")
                {
                    Ok(false)
                } else {
                    Err(AppError::aws(format!(
                        "head_object bucket={} key={} error={message}",
                        self.bucket, key
                    )))
                }
            }
        }
    }

    pub async fn get_bytes(&self, key: &str) -> AppResult<Vec<u8>> {
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
        Ok(output
            .body
            .collect()
            .await
            .map_err(|error| AppError::aws(format!("collect body key={key}: {error}")))?
            .into_bytes()
            .to_vec())
    }

    pub async fn get_byte_range(
        &self,
        key: &str,
        offset: usize,
        length: usize,
    ) -> AppResult<Vec<u8>> {
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
        Ok(output
            .body
            .collect()
            .await
            .map_err(|error| AppError::aws(format!("collect ranged body key={key}: {error}")))?
            .into_bytes()
            .to_vec())
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> AppResult<T> {
        let bytes = self.get_bytes(key).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_keys(&self, prefix: &str, max_keys: usize) -> AppResult<Vec<String>> {
        if max_keys == 0 {
            return Ok(Vec::new());
        }
        let mut keys = Vec::new();
        let mut continuation_token = None;
        while keys.len() < max_keys {
            let remaining = max_keys.saturating_sub(keys.len()).min(i32::MAX as usize) as i32;
            let mut request = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix)
                .max_keys(remaining);
            if let Some(token) = continuation_token {
                request = request.continuation_token(token);
            }
            let output = request.send().await.map_err(|error| {
                AppError::aws(format!(
                    "list_objects_v2 bucket={} prefix={} error={error}",
                    self.bucket, prefix
                ))
            })?;
            for object in output.contents() {
                if let Some(key) = object.key() {
                    keys.push(key.to_owned());
                    if keys.len() >= max_keys {
                        break;
                    }
                }
            }
            continuation_token = output.next_continuation_token().map(ToOwned::to_owned);
            if continuation_token.is_none() {
                break;
            }
        }
        Ok(keys)
    }
}

pub(super) fn byte_range_header(offset: usize, length: usize) -> AppResult<String> {
    if length == 0 {
        return Err(AppError::validation("invalid byte range"));
    }
    let end = offset
        .checked_add(length)
        .and_then(|value| value.checked_sub(1))
        .ok_or_else(|| AppError::validation("invalid byte range"))?;
    Ok(format!("bytes={offset}-{end}"))
}
