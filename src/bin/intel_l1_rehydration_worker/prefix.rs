use intel_structuring_app::error::{AppError, AppResult};
use intel_structuring_app::storage::object_store::validate_object_prefix;

pub(super) fn normalize_structured_prefix(value: &str) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::config("--structured-prefix must not be empty"));
    }
    if trimmed.starts_with('/') || trimmed.to_ascii_lowercase().starts_with("s3://") {
        return Err(AppError::config(
            "--structured-prefix must be an object key prefix, not a URI or absolute path",
        ));
    }
    if !trimmed.starts_with("structured-intel-packet/") {
        return Err(AppError::config(
            "--structured-prefix must start with structured-intel-packet/",
        ));
    }
    let normalized = trimmed.trim_end_matches('/').to_owned() + "/";
    validate_object_prefix(&normalized, "--structured-prefix")?;
    Ok(normalized)
}
