use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeRelevanceWindow {
    pub start_ms: i64,
    pub end_ms: i64,
    pub relevance_decay_hint: RelevanceDecayHint,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelevanceDecayHint {
    Minutes,
    Hours,
    Day,
    MultiDay,
    Structural,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceBand {
    Weak,
    Low,
    Moderate,
    Medium,
    Strong,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Listing,
    Delisting,
    DepositWithdrawal,
    Incident,
    Partnership,
    TokenUnlock,
    Governance,
    FundingShift,
    MacroEvent,
    Regulatory,
    SocialBacklash,
    SocialHype,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ContradictionFlag {
    TimeMismatch,
    SymbolAmbiguity,
    SourceClaimConflict,
    RumorVsOfficial,
    TitleBodyMismatch,
    EvidenceWeak,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelTierUsed {
    RuleOnly,
    #[serde(alias = "haiku")]
    Primary,
    #[serde(alias = "sonnet")]
    Escalation,
    FallbackOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalDecision {
    HighConfidenceStructured,
    LowConfidenceStructured,
    GeneralMarketContext,
    Conflicted,
    UnsupportedOrWeak,
    IrrelevantOrNoise,
    QuarantineOnly,
}
