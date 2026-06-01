use crate::models::market::MarketContextSnapshot;
use crate::storage::object_store::ObjectStore;

mod read;
mod selection;
mod status;

pub use selection::index_pointer_key;

#[derive(Clone)]
pub struct MarketL1Reader {
    store: ObjectStore,
    window_ms: i64,
    radius_windows: i64,
    latest_before_lookback_ms: i64,
    stale_after_ms: i64,
}

impl MarketL1Reader {
    pub fn new(
        store: ObjectStore,
        window_ms: i64,
        radius_windows: i64,
        latest_before_lookback_ms: i64,
        stale_after_ms: i64,
    ) -> Self {
        Self {
            store,
            window_ms,
            radius_windows: radius_windows.max(0),
            latest_before_lookback_ms: latest_before_lookback_ms.max(window_ms),
            stale_after_ms: stale_after_ms.max(window_ms),
        }
    }

    pub async fn context_for(
        &self,
        published_at_ms: Option<i64>,
        fetched_at_ms: i64,
        symbols: &[String],
    ) -> MarketContextSnapshot {
        let (basis_timestamp_ms, basis_kind) = match published_at_ms {
            Some(value) => (value, "published_at_ms"),
            None => (fetched_at_ms, "fetched_at_ms"),
        };
        match self
            .read_contexts(basis_timestamp_ms, basis_kind, symbols)
            .await
        {
            Ok(snapshot) => snapshot,
            Err(error) => MarketContextSnapshot::pending(
                format!("Market-L1 unavailable: {error}"),
                basis_timestamp_ms,
                basis_kind,
            ),
        }
    }
}
