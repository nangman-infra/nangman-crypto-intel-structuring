use super::super::ObjectStore;
use super::super::validation::validate_object_prefix;
use crate::error::{AppError, AppResult};

const S3_LIST_OBJECTS_V2_MAX_KEYS: usize = 1_000;

impl ObjectStore {
    pub async fn list_keys(&self, prefix: &str, max_keys: usize) -> AppResult<Vec<String>> {
        if max_keys == 0 {
            return Ok(Vec::new());
        }
        validate_object_prefix(prefix, "S3 list prefix")?;
        let mut keys = Vec::new();
        let mut continuation_token = None;
        while keys.len() < max_keys {
            let mut request = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix)
                .max_keys(page_limit(keys.len(), max_keys));
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

fn page_limit(current_len: usize, max_keys: usize) -> i32 {
    max_keys
        .saturating_sub(current_len)
        .min(S3_LIST_OBJECTS_V2_MAX_KEYS) as i32
}

#[cfg(test)]
mod tests {
    use super::page_limit;

    #[test]
    fn page_limit_caps_to_s3_max_keys_type() {
        assert_eq!(page_limit(3, 10), 7);
        assert_eq!(page_limit(0, usize::MAX), 1_000);
    }
}
