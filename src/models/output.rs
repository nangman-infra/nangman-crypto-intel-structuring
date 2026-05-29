mod common;
mod lifecycle;
mod packet;
mod story;

pub use common::{
    ConfidenceBand, ContradictionFlag, EventType, ModelTierUsed, RelevanceDecayHint,
    TerminalDecision, TimeRelevanceWindow,
};
pub use lifecycle::{
    ContextFlagPacket, HealthLevel, IntelL1IndexPointer, IntelL1Manifest, OutputObjectRef,
    PacketRevisionIndex, QuarantineEvent, S3ObjectPointer, StructuredPointer,
    StructuringHealthEvent,
};
pub use packet::{
    EvidenceQualityReason, MarketContextRef, MetricEvidence, SourceIndependenceSummary,
    StructuredIntelPacket, SymbolResolutionTrace, TextEvidence,
};
pub use story::{ConflictLevel, StoryCluster, StoryMember};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structured_pointer_has_own_schema_version() {
        assert_eq!(StructuredPointer::schema(), "structured_pointer_v1");
    }

    #[test]
    fn model_tier_accepts_legacy_bedrock_names_but_serializes_current_contract() {
        assert_eq!(
            serde_json::from_str::<ModelTierUsed>("\"haiku\"").unwrap(),
            ModelTierUsed::Primary
        );
        assert_eq!(
            serde_json::from_str::<ModelTierUsed>("\"sonnet\"").unwrap(),
            ModelTierUsed::Escalation
        );
        assert_eq!(
            serde_json::to_string(&ModelTierUsed::Primary).unwrap(),
            "\"primary\""
        );
        assert_eq!(
            serde_json::to_string(&ModelTierUsed::Escalation).unwrap(),
            "\"escalation\""
        );
    }
}
