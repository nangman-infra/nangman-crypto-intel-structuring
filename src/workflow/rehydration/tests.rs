use super::MarketContextRehydrationOptions;
use super::revision::{
    build_revision_write_plan, effective_packet_family_id, effective_raw_event_id,
    parse_revision_from_key,
};
use super::status::{
    is_record_level_rehydration_error, refreshed_context_warrants_revision,
    should_attempt_market_context_refresh,
};
use crate::error::AppError;
use crate::models::market::{MarketContextSnapshot, MarketContextStatus};
use crate::models::output::{
    ConfidenceBand, ContradictionFlag, EventType, EvidenceQualityReason, ModelTierUsed,
    RelevanceDecayHint, SourceIndependenceSummary, StructuredIntelPacket, TerminalDecision,
    TimeRelevanceWindow,
};
use crate::structuring::packet::{market_context_ref, revised_packet_id};

#[test]
fn parses_revision_index_key() {
    assert_eq!(
        parse_revision_from_key(
            "packet-revision-index/schema=intel_l1_packet_revision_index_v1/packet_family_id=family_1/revision=0000000007.json"
        ),
        Some(7)
    );
}

#[test]
fn refresh_candidates_include_pending_and_stale_but_usable() {
    let options = MarketContextRehydrationOptions::default();
    let pending = packet_with_market_status(MarketContextStatus::Pending);
    let stale = packet_with_market_status(MarketContextStatus::StaleButUsable);
    let available = packet_with_market_status(MarketContextStatus::AvailableSymbolContext);

    assert!(should_attempt_market_context_refresh(&pending, &options));
    assert!(should_attempt_market_context_refresh(&stale, &options));
    assert!(!should_attempt_market_context_refresh(&available, &options));
}

#[test]
fn terminal_missing_context_rehydration_requires_explicit_opt_in() {
    let mut packet = packet_with_market_status(MarketContextStatus::Unavailable);
    packet.market_context_terminal_reason = Some("terminal_missing_market_context".to_owned());

    assert!(!should_attempt_market_context_refresh(
        &packet,
        &MarketContextRehydrationOptions::default()
    ));
    assert!(should_attempt_market_context_refresh(
        &packet,
        &MarketContextRehydrationOptions {
            include_terminal_missing_market_context: true
        }
    ));
}

#[test]
fn unrelated_terminal_context_is_not_reopened() {
    let mut packet = packet_with_market_status(MarketContextStatus::Unavailable);
    packet.market_context_terminal_reason = Some("source_contract_terminal".to_owned());

    assert!(!should_attempt_market_context_refresh(
        &packet,
        &MarketContextRehydrationOptions {
            include_terminal_missing_market_context: true
        }
    ));
}

#[test]
fn pending_context_accepts_any_available_refresh() {
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::Pending,
        &MarketContextStatus::StaleButUsable,
        false
    ));
    assert!(!refreshed_context_warrants_revision(
        &MarketContextStatus::Pending,
        &MarketContextStatus::Unavailable,
        false
    ));
}

#[test]
fn stale_context_requires_non_stale_available_refresh() {
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::StaleButUsable,
        &MarketContextStatus::NearestAvailable,
        false
    ));
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::StaleButUsable,
        &MarketContextStatus::AvailableSymbolContext,
        false
    ));
    assert!(!refreshed_context_warrants_revision(
        &MarketContextStatus::StaleButUsable,
        &MarketContextStatus::StaleButUsable,
        false
    ));
}

#[test]
fn terminal_reopen_accepts_any_available_refresh() {
    assert!(refreshed_context_warrants_revision(
        &MarketContextStatus::Unavailable,
        &MarketContextStatus::StaleButUsable,
        true
    ));
    assert!(!refreshed_context_warrants_revision(
        &MarketContextStatus::Unavailable,
        &MarketContextStatus::Unavailable,
        true
    ));
}

#[test]
fn record_level_rehydration_errors_are_skippable() {
    let json_error = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    assert!(is_record_level_rehydration_error(&AppError::Json(
        json_error
    )));
    assert!(is_record_level_rehydration_error(&AppError::validation(
        "legacy packet"
    )));
    assert!(!is_record_level_rehydration_error(&AppError::aws(
        "object store unavailable"
    )));
}

#[test]
fn revision_plan_rewrites_packet_manifest_index_and_pointer() {
    let packet = packet_with_market_status(MarketContextStatus::StaleButUsable);
    let market_context = available_market_context();
    let plan = build_revision_write_plan(
        &packet,
        market_context,
        None,
        3_600_000,
        "test-output-bucket",
        "policy_v1",
    )
    .unwrap();
    let expected_packet_id = revised_packet_id("family_1", 3);

    assert_eq!(plan.revised_packet.packet_family_id, "family_1");
    assert_eq!(plan.revised_packet.raw_event_id, "raw_1");
    assert_eq!(plan.revised_packet.packet_id, expected_packet_id);
    assert_eq!(plan.revised_packet.revision, 3);
    assert_eq!(
        plan.revised_packet.supersedes_packet_id.as_deref(),
        Some("packet_1")
    );
    assert_eq!(
        plan.revised_packet.market_context_status,
        MarketContextStatus::AvailableSymbolContext
    );
    assert_eq!(plan.revised_packet.market_context_retry_after_ms, None);
    assert_eq!(plan.revised_packet.market_context_expire_at_ms, None);
    assert_eq!(plan.revised_packet.market_context_terminal_reason, None);
    assert!(
        !plan
            .revised_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::MarketContextMissing)
    );
    assert!(
        plan.revised_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::SingleSourceOnly)
    );

    assert!(plan.structured_key.contains("raw_event_id=raw_1"));
    assert!(plan.structured_key.contains(&plan.revised_packet.packet_id));
    assert_eq!(plan.manifest.status, "rehydrated_market_context");
    assert_eq!(plan.manifest.raw_event_id, "raw_1");
    assert_eq!(plan.manifest.structuring_policy_version, "policy_v1");
    assert_eq!(plan.manifest.output_object_count, 1);
    assert_eq!(
        plan.manifest.output_objects[0].byte_count,
        plan.structured_bytes.len()
    );
    assert_eq!(plan.manifest.output_objects[0].key, plan.structured_key);
    assert_eq!(plan.manifest_key, plan.pointer.manifest_key);
    assert_eq!(plan.revision_index.packet_family_id, "family_1");
    assert_eq!(plan.revision_index.latest_revision, 3);
    assert_eq!(
        plan.revision_index.market_context_status,
        MarketContextStatus::AvailableSymbolContext
    );
    assert_eq!(plan.pointer.storage_ref.bucket, "test-output-bucket");
    assert_eq!(plan.pointer.storage_ref.key, plan.structured_key);
    assert!(
        plan.pointer
            .storage_ref
            .content_sha256
            .starts_with("sha256:")
    );
}

#[test]
fn terminal_revision_plan_keeps_missing_context_reason() {
    let packet = packet_with_market_status(MarketContextStatus::Pending);
    let plan = build_revision_write_plan(
        &packet,
        MarketContextSnapshot::unavailable("terminal_missing_market_context", "published_at_ms"),
        Some("terminal_missing_market_context".to_owned()),
        3_600_000,
        "test-output-bucket",
        "policy_v1",
    )
    .unwrap();

    assert_eq!(plan.manifest.status, "terminal_missing_market_context");
    assert_eq!(
        plan.revised_packet.market_context_status,
        MarketContextStatus::Unavailable
    );
    assert_eq!(
        plan.revised_packet
            .market_context_terminal_reason
            .as_deref(),
        Some("terminal_missing_market_context")
    );
    assert!(
        plan.revised_packet
            .evidence_quality_reasons
            .contains(&EvidenceQualityReason::MarketContextMissing)
    );
}

#[test]
fn effective_ids_fall_back_to_source_event_then_packet_id() {
    let mut packet = packet_with_market_status(MarketContextStatus::Pending);
    packet.packet_family_id.clear();
    packet.raw_event_id.clear();

    assert_eq!(effective_packet_family_id(&packet), "source_1");
    assert_eq!(effective_raw_event_id(&packet), "source_1");

    packet.source_event_ids.clear();

    assert_eq!(effective_packet_family_id(&packet), "packet_1");
    assert_eq!(effective_raw_event_id(&packet), "packet_1");
}

fn packet_with_market_status(status: MarketContextStatus) -> StructuredIntelPacket {
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

fn available_market_context() -> MarketContextSnapshot {
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
