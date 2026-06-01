use crate::error::AppResult;
use crate::models::market::{MarketL1ReadPlan, MarketSymbolSummary};
use crate::storage::object_store::ObjectStore;
use bytes::Bytes;
use std::collections::BTreeMap;

use super::scan::scan_parquet_bytes;
use super::wanted::wanted_symbols;

pub async fn read_symbol_summaries(
    store: &ObjectStore,
    plan: &MarketL1ReadPlan,
    symbols: &[String],
) -> AppResult<Vec<MarketSymbolSummary>> {
    let wanted = wanted_symbols(symbols);
    if wanted.is_empty() {
        return Ok(Vec::new());
    }

    let mut summaries = BTreeMap::<String, MarketSymbolSummary>::new();
    for key in &plan.output_object_keys {
        let bytes = Bytes::from(store.get_bytes(key).await?);
        for summary in scan_parquet_bytes(bytes, &wanted)? {
            summaries
                .entry(format!("{}:{}", summary.symbol, summary.venue))
                .or_insert(summary);
        }
    }
    Ok(summaries.into_values().collect())
}
