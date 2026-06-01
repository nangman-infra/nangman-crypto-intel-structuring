use crate::error::AppResult;
use serde::Serialize;

pub(super) const JSON_CONTENT_TYPE: &str = "application/json";
pub(super) const JSONL_CONTENT_TYPE: &str = "application/x-ndjson";

pub(super) fn json_bytes<T: Serialize>(value: &T) -> AppResult<Vec<u8>> {
    Ok(serde_json::to_vec_pretty(value)?)
}

pub(super) fn jsonl_bytes<T: Serialize>(records: &[T]) -> AppResult<Vec<u8>> {
    let (bytes, _) = crate::jsonl::build_jsonl_chunk(records)?;
    Ok(bytes)
}
