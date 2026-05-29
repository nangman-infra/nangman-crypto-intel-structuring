use crate::admission::market_l1::build_market_l1_read_plan;
use crate::error::AppResult;
use crate::models::market::{
    MarketContextSnapshot, MarketL1IndexPointer, MarketL1Manifest, MarketL1Report,
};
use crate::storage::object_store::ObjectStore;
use crate::time::floor_window;

mod selection;
mod status;

pub use selection::index_pointer_key;
use status::{context_status, merge_snapshots};

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

    async fn read_contexts(
        &self,
        basis_timestamp_ms: i64,
        basis_kind: &str,
        symbols: &[String],
    ) -> AppResult<MarketContextSnapshot> {
        let basis_window_start_ms = floor_window(basis_timestamp_ms, self.window_ms);
        let mut snapshots = Vec::new();
        let mut last_error = None;
        for window_start_ms in self.candidate_window_starts(basis_window_start_ms).await {
            match self
                .read_single_context(
                    basis_timestamp_ms,
                    basis_kind,
                    symbols,
                    basis_window_start_ms,
                    window_start_ms,
                )
                .await
            {
                Ok(snapshot) => snapshots.push(snapshot),
                Err(error) => last_error = Some(error),
            }
        }
        merge_snapshots(snapshots).ok_or_else(|| {
            last_error.unwrap_or_else(|| {
                crate::error::AppError::validation("Market-L1 no usable windows")
            })
        })
    }

    async fn read_single_context(
        &self,
        basis_timestamp_ms: i64,
        basis_kind: &str,
        symbols: &[String],
        basis_window_start_ms: i64,
        window_start_ms: i64,
    ) -> AppResult<MarketContextSnapshot> {
        let window_end_ms = window_start_ms + self.window_ms;
        let pointer_key = index_pointer_key(self.window_ms, window_start_ms);
        let pointer = self
            .store
            .get_json::<MarketL1IndexPointer>(&pointer_key)
            .await?;
        let manifest = self
            .store
            .get_json::<MarketL1Manifest>(&pointer.canonical_manifest_key)
            .await?;
        let report = self
            .store
            .get_json::<MarketL1Report>(&manifest.report_key)
            .await?;
        let plan = build_market_l1_read_plan(
            &pointer,
            &manifest,
            &report,
            &pointer.canonical_manifest_key,
            window_start_ms,
            window_end_ms,
        )?;

        let symbol_summaries = if symbols.is_empty() {
            Vec::new()
        } else {
            crate::market::parquet_compact::read_symbol_summaries(&self.store, &plan, symbols)
                .await?
        };

        Ok(MarketContextSnapshot {
            status: context_status(
                symbols,
                &symbol_summaries,
                window_start_ms,
                basis_window_start_ms,
                self.stale_after_ms,
            ),
            basis_timestamp_ms: Some(basis_timestamp_ms),
            basis_kind: basis_kind.to_owned(),
            window_start_ms: Some(window_start_ms),
            window_end_ms: Some(window_end_ms),
            manifest_key: Some(plan.manifest_key),
            output_object_keys: plan.output_object_keys,
            market_data_quality_summary_key: plan.market_data_quality_summary_key,
            market_feature_delta_key: plan.market_feature_delta_key,
            market_feature_delta_summary_key: plan.market_feature_delta_summary_key,
            market_regime_context_key: plan.market_regime_context_key,
            symbol_universe_snapshot_key: plan.symbol_universe_snapshot_key,
            symbol_summaries,
            unavailable_reason: None,
        })
    }
}
