use crate::error::{AppError, AppResult};

pub(in crate::storage::object_store) fn byte_range_header(
    offset: usize,
    length: usize,
) -> AppResult<String> {
    if length == 0 {
        return Err(AppError::validation("invalid byte range"));
    }
    let end = offset
        .checked_add(length)
        .and_then(|value| value.checked_sub(1))
        .ok_or_else(|| AppError::validation("invalid byte range"))?;
    Ok(format!("bytes={offset}-{end}"))
}

#[cfg(test)]
mod tests {
    use super::byte_range_header;

    #[test]
    fn rejects_overflowing_byte_range() {
        let error = byte_range_header(usize::MAX, 2).unwrap_err();
        assert!(error.to_string().contains("invalid byte range"));
    }
}
