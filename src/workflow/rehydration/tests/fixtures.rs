use crate::models::market::{MarketContextSnapshot, MarketContextStatus};
use crate::models::output::{
    ConfidenceBand, ContradictionFlag, EventType, EvidenceQualityReason, ModelTierUsed,
    RelevanceDecayHint, SourceIndependenceSummary, StructuredIntelPacket, TerminalDecision,
    TimeRelevanceWindow,
};
use crate::structuring::packet::market_context_ref;

pub(super) fn packet_with_market_status(status: MarketContextStatus) -> StructuredIntelPacket {
    let market_context = match &status {
        MarketContextStatus::Pending => {
            MarketContextSnapshot::pending("missing", 1, "published_at_ms")
        }
        MarketContextStatus::Unavailable => {
            MarketContextSnapshot::unavailable("missing", "published_at_ms")
        }
        other => {
            let mut context = available_market_context();
            context.status = other.clone();
            context
        }
    };
    let market_context_ref = market_context_ref(&market_context);
    StructuredIntelPacket {
        packet_id: "packet_1".to_owned(),
        packet_family_id: "family_1".to_owned(),
        raw_event_id: "raw_1".to_owned(),
        event_timestamp_ms: 1,
        revision: 2,
        supersedes_packet_id: None,
        cluster_id: "cluster_1".to_owned(),
        source_event_ids: vec!["source_1".to_owned()],
        published_at_ms: Some(1),
        fetched_at_ms: 1,
        structured_at_ms: 2,
        decision_available_at_ms: 2,
        normalized_symbols: vec!["BTC".to_owned()],
        symbol_confidence_band: ConfidenceBand::High,
        symbol_resolution_trace: Vec::new(),
        event_type: EventType::FundingShift,
        topic_summary: "funding changed".to_owned(),
        stance_summary: "neutral".to_owned(),
        risk_summary: "observe".to_owned(),
        regime_hint: "mixed".to_owned(),
        scenario_hint: "basis_watch".to_owned(),
        confidence_band: ConfidenceBand::High,
        novelty_score: 0.8,
        time_relevance_window: TimeRelevanceWindow {
            start_ms: 1,
            end_ms: 3_600_001,
            relevance_decay_hint: RelevanceDecayHint::Hours,
        },
        contradiction_flags: vec![ContradictionFlag::EvidenceWeak],
        source_quality_summary: "test source".to_owned(),
        source_independence_summary: SourceIndependenceSummary {
            source_event_count: 1,
            independent_source_count: 1,
            official_source_present: false,
            duplicate_content_hashes: Vec::new(),
            syndicated_from: None,
            original_source_ids: vec!["source_1".to_owned()],
        },
        text_evidence: Vec::new(),
        metric_evidence: Vec::new(),
        evidence_quality_reasons: vec![
            EvidenceQualityReason::MarketContextMissing,
            EvidenceQualityReason::SingleSourceOnly,
        ],
        market_context_status: status,
        market_context_retry_after_ms: Some(10),
        market_context_expire_at_ms: Some(20),
        market_context_terminal_reason: None,
        market_context_ref,
        model_tier_used: ModelTierUsed::RuleOnly,
        terminal_decision: TerminalDecision::LowConfidenceStructured,
        evidence_sentences: vec!["evidence".to_owned()],
        market_context,
        schema_version: StructuredIntelPacket::schema(),
    }
}

pub(super) fn available_market_context() -> MarketContextSnapshot {
    MarketContextSnapshot {
        status: MarketContextStatus::AvailableSymbolContext,
        basis_timestamp_ms: Some(1),
        basis_kind: "published_at_ms".to_owned(),
        window_start_ms: Some(0),
        window_end_ms: Some(3_600_000),
        manifest_key: Some("market/manifest.json".to_owned()),
        output_object_keys: vec!["market/data.parquet".to_owned()],
        market_data_quality_summary_key: Some("market/quality.json".to_owned()),
        market_feature_delta_key: Some("market/delta.parquet".to_owned()),
        market_feature_delta_summary_key: Some("market/delta-summary.json".to_owned()),
        market_regime_context_key: Some("market/regime.json".to_owned()),
        symbol_universe_snapshot_key: Some("market/universe.json".to_owned()),
        symbol_summaries: Vec::new(),
        unavailable_reason: None,
    }
}
