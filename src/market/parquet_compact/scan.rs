use crate::error::{AppError, AppResult};
use crate::models::market::MarketSymbolSummary;
use bytes::Bytes;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use super::batch::extract_batch_summaries;
use super::wanted::WantedSymbols;

pub(super) fn scan_parquet_bytes(
    bytes: Bytes,
    wanted: &WantedSymbols,
) -> AppResult<Vec<MarketSymbolSummary>> {
    let reader = ParquetRecordBatchReaderBuilder::try_new(bytes)
        .map_err(|error| AppError::parquet(error.to_string()))?
        .with_batch_size(2048)
        .build()
        .map_err(|error| AppError::parquet(error.to_string()))?;

    let mut output = Vec::new();
    for batch in reader {
        let batch = batch.map_err(|error| AppError::parquet(error.to_string()))?;
        output.extend(extract_batch_summaries(&batch, wanted)?);
    }
    Ok(output)
}
