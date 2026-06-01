use serde::{Deserialize, Serialize};

use super::super::common::ConfidenceBand;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolResolutionTrace {
    pub raw_mentions: Vec<String>,
    pub resolved_project: Option<String>,
    pub resolved_asset: Option<String>,
    pub canonical_symbol: Option<String>,
    pub venue_symbols: Vec<String>,
    pub mapping_confidence: ConfidenceBand,
    pub ambiguity_reason: Option<String>,
}
