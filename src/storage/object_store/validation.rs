use crate::error::{AppError, AppResult};

const MAX_S3_OBJECT_KEY_BYTES: usize = 1024;

pub fn validate_object_key(key: &str, label: &str) -> AppResult<()> {
    validate_key_shape(key, label, false)
}

pub fn validate_object_prefix(prefix: &str, label: &str) -> AppResult<()> {
    validate_key_shape(prefix, label, true)
}

fn validate_key_shape(value: &str, label: &str, allow_trailing_slash: bool) -> AppResult<()> {
    if value.is_empty() {
        return Err(AppError::validation(format!("{label} must not be empty")));
    }
    if value.trim() != value {
        return Err(AppError::validation(format!(
            "{label} must not include leading or trailing whitespace"
        )));
    }
    if value.len() > MAX_S3_OBJECT_KEY_BYTES {
        return Err(AppError::validation(format!(
            "{label} must be at most {MAX_S3_OBJECT_KEY_BYTES} bytes"
        )));
    }
    if value.starts_with('/') || value.to_ascii_lowercase().starts_with("s3://") {
        return Err(AppError::validation(format!(
            "{label} must be an object key, not a URI or absolute path"
        )));
    }
    if value.contains('?') || value.contains('#') {
        return Err(AppError::validation(format!(
            "{label} must not include query or fragment markers"
        )));
    }
    if value
        .chars()
        .any(|ch| ch.is_control() || ch.is_whitespace() || ch == '\\')
    {
        return Err(AppError::validation(format!(
            "{label} must not contain control characters, whitespace, or backslashes"
        )));
    }

    let normalized = if allow_trailing_slash {
        value.strip_suffix('/').unwrap_or(value)
    } else {
        value
    };
    if normalized.is_empty() || normalized.split('/').any(str::is_empty) {
        return Err(AppError::validation(format!(
            "{label} must not contain empty path segments"
        )));
    }
    if normalized
        .split('/')
        .any(|segment| matches!(segment, "." | ".."))
    {
        return Err(AppError::validation(format!(
            "{label} must not contain period-only path segments"
        )));
    }
    Ok(())
}
