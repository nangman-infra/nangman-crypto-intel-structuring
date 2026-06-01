use super::super::ObjectStore;
use super::super::validation::validate_object_key;
use crate::error::{AppError, AppResult};

impl ObjectStore {
    pub async fn object_exists(&self, key: &str) -> AppResult<bool> {
        validate_object_key(key, "S3 object key")?;
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
                if message_looks_not_found(&message) {
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
}

fn message_looks_not_found(message: &str) -> bool {
    message.contains("NotFound") || message.contains("404") || message.contains("NoSuchKey")
}

#[cfg(test)]
mod tests {
    use super::message_looks_not_found;

    #[test]
    fn recognizes_s3_not_found_fallback_messages() {
        assert!(message_looks_not_found("service returned NotFound"));
        assert!(message_looks_not_found("status code 404"));
        assert!(message_looks_not_found("NoSuchKey"));
        assert!(!message_looks_not_found("AccessDenied"));
    }
}
