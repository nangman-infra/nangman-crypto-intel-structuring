use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceQualityReason {
    BaselineMissing,
    SingleNumericSnapshot,
    SingleSourceOnly,
    TitleOnly,
    SymbolAmbiguous,
    MarketContextMissing,
    DuplicateOrSyndicatedSource,
}
