use crate::error::AppResult;
use crate::models::market::MarketSymbolSummary;
use arrow_array::RecordBatch;

use super::columns::{f64_col, i64_col, nullable_value, optional_f64_col, string_col};
use super::wanted::WantedSymbols;

pub(super) fn extract_batch_summaries(
    batch: &RecordBatch,
    wanted: &WantedSymbols,
) -> AppResult<Vec<MarketSymbolSummary>> {
    let base_asset = string_col(batch, "base_asset")?;
    let symbol_canonical = string_col(batch, "symbol_canonical")?;
    let venue = string_col(batch, "venue")?;
    let slice_completeness = string_col(batch, "slice_completeness")?;
    let window_start_ms = i64_col(batch, "window_start_ms")?;
    let window_end_ms = i64_col(batch, "window_end_ms")?;
    let trade_count = i64_col(batch, "trade_count")?;
    let trade_volume = f64_col(batch, "trade_volume")?;
    let mid_price = optional_f64_col(batch, "mid_price")?;
    let spread_bps = optional_f64_col(batch, "spread_bps")?;

    let mut summaries = Vec::new();
    for index in 0..batch.num_rows() {
        let symbol = base_asset.value(index).to_ascii_uppercase();
        let canonical = symbol_canonical.value(index).to_ascii_uppercase();
        if !wanted.contains(&symbol) && !wanted.contains(&canonical) {
            continue;
        }
        summaries.push(MarketSymbolSummary {
            symbol,
            venue: venue.value(index).to_owned(),
            window_start_ms: window_start_ms.value(index),
            window_end_ms: window_end_ms.value(index),
            mid_price: nullable_value(mid_price, index),
            spread_bps: nullable_value(spread_bps, index),
            trade_count: trade_count.value(index),
            trade_volume: trade_volume.value(index),
            slice_completeness: slice_completeness.value(index).to_owned(),
        });
    }
    Ok(summaries)
}
