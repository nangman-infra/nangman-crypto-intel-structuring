use super::summary::MarketSymbolSummary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketContextSnapshot {
    pub status: MarketContextStatus,
    pub basis_timestamp_ms: Option<i64>,
    pub basis_kind: String,
    pub window_start_ms: Option<i64>,
    pub window_end_ms: Option<i64>,
    pub manifest_key: Option<String>,
    pub output_object_keys: Vec<String>,
    pub market_data_quality_summary_key: Option<String>,
    pub market_feature_delta_key: Option<String>,
    pub market_feature_delta_summary_key: Option<String>,
    pub market_regime_context_key: Option<String>,
    pub symbol_universe_snapshot_key: Option<String>,
    pub symbol_summaries: Vec<MarketSymbolSummary>,
    pub unavailable_reason: Option<String>,
}

impl MarketContextSnapshot {
    pub fn unavailable(reason: impl Into<String>, basis_kind: impl Into<String>) -> Self {
        Self {
            status: MarketContextStatus::Unavailable,
            basis_timestamp_ms: None,
            basis_kind: basis_kind.into(),
            window_start_ms: None,
            window_end_ms: None,
            manifest_key: None,
            output_object_keys: Vec::new(),
            market_data_quality_summary_key: None,
            market_feature_delta_key: None,
            market_feature_delta_summary_key: None,
            market_regime_context_key: None,
            symbol_universe_snapshot_key: None,
            symbol_summaries: Vec::new(),
            unavailable_reason: Some(reason.into()),
        }
    }

    pub fn pending(reason: impl Into<String>, basis_timestamp_ms: i64, basis_kind: &str) -> Self {
        Self {
            status: MarketContextStatus::Pending,
            basis_timestamp_ms: Some(basis_timestamp_ms),
            basis_kind: basis_kind.to_owned(),
            window_start_ms: None,
            window_end_ms: None,
            manifest_key: None,
            output_object_keys: Vec::new(),
            market_data_quality_summary_key: None,
            market_feature_delta_key: None,
            market_feature_delta_summary_key: None,
            market_regime_context_key: None,
            symbol_universe_snapshot_key: None,
            symbol_summaries: Vec::new(),
            unavailable_reason: Some(reason.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MarketContextStatus {
    Available,
    AvailableSymbolContext,
    AvailableGeneralContext,
    NearestAvailable,
    SymbolContextOnly,
    StaleButUsable,
    Pending,
    Unavailable,
}

impl MarketContextStatus {
    pub fn is_any_available(&self) -> bool {
        matches!(
            self,
            Self::Available
                | Self::AvailableSymbolContext
                | Self::AvailableGeneralContext
                | Self::NearestAvailable
                | Self::SymbolContextOnly
                | Self::StaleButUsable
        )
    }

    pub fn is_symbol_usable(&self) -> bool {
        matches!(
            self,
            Self::Available
                | Self::AvailableSymbolContext
                | Self::NearestAvailable
                | Self::SymbolContextOnly
                | Self::StaleButUsable
        )
    }

    pub fn is_pending_or_unavailable(&self) -> bool {
        matches!(self, Self::Pending | Self::Unavailable)
    }

    pub fn is_stale_but_usable(&self) -> bool {
        matches!(self, Self::StaleButUsable)
    }

    pub fn supports_numeric_snapshot_escalation(&self) -> bool {
        matches!(
            self,
            Self::Available
                | Self::AvailableSymbolContext
                | Self::NearestAvailable
                | Self::SymbolContextOnly
        )
    }
}
