use crate::error::{AppError, AppResult};
use arrow_array::{Array, Float64Array, Int64Array, RecordBatch, StringArray};

pub(super) fn string_col<'a>(batch: &'a RecordBatch, name: &str) -> AppResult<&'a StringArray> {
    batch
        .column_by_name(name)
        .ok_or_else(|| AppError::parquet(format!("missing column {name}")))?
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| AppError::parquet(format!("column {name} is not StringArray")))
}

pub(super) fn i64_col<'a>(batch: &'a RecordBatch, name: &str) -> AppResult<&'a Int64Array> {
    batch
        .column_by_name(name)
        .ok_or_else(|| AppError::parquet(format!("missing column {name}")))?
        .as_any()
        .downcast_ref::<Int64Array>()
        .ok_or_else(|| AppError::parquet(format!("column {name} is not Int64Array")))
}

pub(super) fn f64_col<'a>(batch: &'a RecordBatch, name: &str) -> AppResult<&'a Float64Array> {
    batch
        .column_by_name(name)
        .ok_or_else(|| AppError::parquet(format!("missing column {name}")))?
        .as_any()
        .downcast_ref::<Float64Array>()
        .ok_or_else(|| AppError::parquet(format!("column {name} is not Float64Array")))
}

pub(super) fn optional_f64_col<'a>(
    batch: &'a RecordBatch,
    name: &str,
) -> AppResult<Option<&'a Float64Array>> {
    match batch.column_by_name(name) {
        Some(array) => Ok(Some(
            array
                .as_any()
                .downcast_ref::<Float64Array>()
                .ok_or_else(|| AppError::parquet(format!("column {name} is not Float64Array")))?,
        )),
        None => Ok(None),
    }
}

pub(super) fn nullable_value(array: Option<&Float64Array>, index: usize) -> Option<f64> {
    let array = array?;
    if array.is_null(index) {
        None
    } else {
        Some(array.value(index))
    }
}
