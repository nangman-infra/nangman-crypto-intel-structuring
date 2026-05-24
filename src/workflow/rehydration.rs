use crate::config::ProcessingConfig;
use crate::error::{AppError, AppResult};
use crate::hash::sha256_prefixed;
use crate::jsonl::build_jsonl_chunk;
use crate::market::reader::MarketL1Reader;
use crate::models::constants::{MANIFEST_SCHEMA_VERSION, STRUCTURED_PACKET_SCHEMA_VERSION};
use crate::models::market::{MarketContextSnapshot, MarketContextStatus};
use crate::models::output::{
    IntelL1Manifest, OutputObjectRef, PacketRevisionIndex, S3ObjectPointer, StructuredIntelPacket,
    StructuredPointer,
};
use crate::nats::publisher::StructuredPublisher;
use crate::storage::object_store::ObjectStore;
use crate::structuring::packet::{market_context_ref, revised_packet_id};
use crate::time::{now_ms, run_id};
use crate::workflow::keys;
use std::collections::BTreeSet;

const STRUCTURED_PACKET_PREFIX: &str = "structured-intel-packet/schema=structured_intel_packet_v1/";
const REVISION_INDEX_MAX_KEYS: usize = 256;
const TERMINAL_MISSING_MARKET_CONTEXT: &str = "terminal_missing_market_context";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MarketContextRehydrationOptions {
    pub include_terminal_missing_market_context: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MarketContextRehydrationSummary {
    pub scanned_keys: usize,
    pub published_revisions: usize,
    pub skipped_record_errors: usize,
}

pub struct MarketContextRehydrator {
    output_store: ObjectStore,
    market_reader: MarketL1Reader,
    publisher: StructuredPublisher,
    config: ProcessingConfig,
}

impl MarketContextRehydrator {
    pub fn new(
        output_store: ObjectStore,
        market_reader: MarketL1Reader,
        publisher: StructuredPublisher,
        config: ProcessingConfig,
    ) -> Self {
        Self {
            output_store,
            market_reader,
            publisher,
            config,
        }
    }

    pub async fn run_once(&self, max_packets: usize) -> AppResult<usize> {
        Ok(self
            .run_once_with_options(max_packets, MarketContextRehydrationOptions::default())
            .await?
            .published_revisions)
    }

    pub async fn run_once_with_options(
        &self,
        max_packets: usize,
        options: MarketContextRehydrationOptions,
    ) -> AppResult<MarketContextRehydrationSummary> {
        self.run_prefixes_once_with_options(
            &[STRUCTURED_PACKET_PREFIX.to_owned()],
            max_packets,
            options,
        )
        .await
    }

    pub async fn run_prefixes_once(
        &self,
        prefixes: &[String],
        max_packets_per_prefix: usize,
    ) -> AppResult<usize> {
        Ok(self
            .run_prefixes_once_with_options(
                prefixes,
                max_packets_per_prefix,
                MarketContextRehydrationOptions::default(),
            )
            .await?
            .published_revisions)
    }

    pub async fn run_prefixes_once_with_options(
        &self,
        prefixes: &[String],
        max_packets_per_prefix: usize,
        options: MarketContextRehydrationOptions,
    ) -> AppResult<MarketContextRehydrationSummary> {
        let keys = self
            .list_rehydration_keys(prefixes, max_packets_per_prefix)
            .await?;
        let mut summary = MarketContextRehydrationSummary {
            scanned_keys: keys.len(),
            ..MarketContextRehydrationSummary::default()
        };
        for key in keys {
            match self.try_rehydrate_key(&key, options).await {
                Ok(true) => summary.published_revisions += 1,
                Ok(false) => {}
                Err(error) if is_record_level_rehydration_error(&error) => {
                    summary.skipped_record_errors += 1;
                    eprintln!("market context rehydration skipped key={key}: {error}");
                }
                Err(error) => return Err(error),
            }
        }
        Ok(summary)
    }

    async fn list_rehydration_keys(
        &self,
        prefixes: &[String],
        max_packets_per_prefix: usize,
    ) -> AppResult<Vec<String>> {
        let mut keys = BTreeSet::new();
        for prefix in prefixes {
            for key in self
                .output_store
                .list_keys(prefix, max_packets_per_prefix)
                .await?
            {
                keys.insert(key);
            }
        }
        Ok(keys.into_iter().collect())
    }

    async fn try_rehydrate_key(
        &self,
        key: &str,
        options: MarketContextRehydrationOptions,
    ) -> AppResult<bool> {
        let bytes = self.output_store.get_bytes(key).await?;
        let packet: StructuredIntelPacket = serde_json::from_slice(&bytes)?;
        let terminal_reopen =
            is_terminal_missing_market_context_reopen_candidate(&packet, &options);
        if !should_attempt_market_context_refresh(&packet, &options) {
            return Ok(false);
        }
        if packet.market_context_terminal_reason.is_some() && !terminal_reopen {
            return Ok(false);
        }
        if packet
            .market_context_retry_after_ms
            .is_some_and(|retry_after_ms| retry_after_ms > now_ms())
        {
            return Ok(false);
        }
        if self.is_not_latest_revision(&packet).await? {
            return Ok(false);
        }

        let refreshed_context = self
            .market_reader
            .context_for(
                packet.published_at_ms,
                packet.fetched_at_ms,
                &packet.normalized_symbols,
            )
            .await;
        if refreshed_context_warrants_revision(
            &packet.market_context_status,
            &refreshed_context.status,
            terminal_reopen,
        ) {
            self.publish_revision(packet, refreshed_context, None)
                .await?;
            return Ok(true);
        }
        if packet.market_context_status == MarketContextStatus::Pending
            && packet
                .market_context_expire_at_ms
                .or_else(|| {
                    Some(
                        packet
                            .decision_available_at_ms
                            .saturating_add(self.config.market_context_expire_after_ms),
                    )
                })
                .is_some_and(|expire_at_ms| expire_at_ms <= now_ms())
        {
            let basis_kind = if packet.published_at_ms.is_some() {
                "published_at_ms"
            } else {
                "fetched_at_ms"
            };
            let terminal_context =
                MarketContextSnapshot::unavailable(TERMINAL_MISSING_MARKET_CONTEXT, basis_kind);
            self.publish_revision(
                packet,
                terminal_context,
                Some(TERMINAL_MISSING_MARKET_CONTEXT.to_owned()),
            )
            .await?;
            return Ok(true);
        }
        Ok(false)
    }

    async fn is_not_latest_revision(&self, packet: &StructuredIntelPacket) -> AppResult<bool> {
        let Some(index) = self
            .latest_revision_index(effective_packet_family_id(packet))
            .await?
        else {
            return Ok(false);
        };
        Ok(packet.revision < index.latest_revision)
    }

    async fn latest_revision_index(
        &self,
        packet_family_id: &str,
    ) -> AppResult<Option<PacketRevisionIndex>> {
        let mut latest: Option<(u32, String)> = None;
        for key in self
            .output_store
            .list_keys(
                &keys::packet_revision_index_prefix(packet_family_id),
                REVISION_INDEX_MAX_KEYS,
            )
            .await?
        {
            let Some(revision) = parse_revision_from_key(&key) else {
                continue;
            };
            let replace = latest
                .as_ref()
                .is_none_or(|(current_revision, _)| revision > *current_revision);
            if replace {
                latest = Some((revision, key));
            }
        }
        let Some((_, key)) = latest else {
            return Ok(None);
        };
        self.output_store.get_json(&key).await.map(Some)
    }

    async fn publish_revision(
        &self,
        packet: StructuredIntelPacket,
        market_context: MarketContextSnapshot,
        terminal_reason: Option<String>,
    ) -> AppResult<()> {
        let plan = build_revision_write_plan(
            &packet,
            market_context,
            terminal_reason,
            now_ms(),
            self.output_store.bucket(),
            &self.config.structuring_policy_version,
        )?;
        self.output_store
            .put_bytes_idempotent(
                &plan.structured_key,
                plan.structured_bytes.clone(),
                "application/x-ndjson",
            )
            .await?;
        self.output_store
            .put_json_idempotent(&plan.manifest_key, &plan.manifest)
            .await?;
        self.output_store
            .put_json_idempotent(&plan.revision_index_key, &plan.revision_index)
            .await?;
        self.publisher
            .publish_structured_pointer(&plan.revised_packet, &plan.pointer)
            .await?;
        self.publisher.flush().await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct RevisionWritePlan {
    revised_packet: StructuredIntelPacket,
    structured_key: String,
    structured_bytes: Vec<u8>,
    manifest: IntelL1Manifest,
    manifest_key: String,
    revision_index: PacketRevisionIndex,
    revision_index_key: String,
    pointer: StructuredPointer,
}

fn build_revision_write_plan(
    packet: &StructuredIntelPacket,
    market_context: MarketContextSnapshot,
    terminal_reason: Option<String>,
    created_at_ms: i64,
    bucket: &str,
    policy_version: &str,
) -> AppResult<RevisionWritePlan> {
    let revision = packet.revision.saturating_add(1);
    let packet_family_id = effective_packet_family_id(packet).to_owned();
    let raw_event_id = effective_raw_event_id(packet).to_owned();
    let packet_id = revised_packet_id(&packet_family_id, revision);
    let mut revised_packet = packet.clone();
    revised_packet.packet_family_id = packet_family_id.clone();
    revised_packet.raw_event_id = raw_event_id.clone();
    revised_packet.packet_id = packet_id.clone();
    revised_packet.revision = revision;
    revised_packet.supersedes_packet_id = Some(packet.packet_id.clone());
    revised_packet.market_context_status = market_context.status.clone();
    revised_packet.market_context = market_context.clone();
    revised_packet.market_context_ref = market_context_ref(&market_context);
    revised_packet.market_context_retry_after_ms = None;
    revised_packet.market_context_expire_at_ms = None;
    revised_packet.market_context_terminal_reason = terminal_reason.clone();
    if market_context.status.is_any_available() {
        revised_packet.evidence_quality_reasons.retain(|reason| {
            !matches!(
                reason,
                crate::models::output::EvidenceQualityReason::MarketContextMissing
            )
        });
    }

    let structured_key = keys::structured_packet_key(created_at_ms, &raw_event_id, &packet_id);
    let (structured_bytes, _) = build_jsonl_chunk(std::slice::from_ref(&revised_packet))?;
    let run_id = run_id("intel-l1-rehydration", created_at_ms);
    let manifest = IntelL1Manifest {
        schema_version: MANIFEST_SCHEMA_VERSION.to_owned(),
        run_id: run_id.clone(),
        raw_event_id: raw_event_id.clone(),
        status: if terminal_reason.is_some() {
            "terminal_missing_market_context".to_owned()
        } else {
            "rehydrated_market_context".to_owned()
        },
        started_at_ms: created_at_ms,
        finished_at_ms: created_at_ms,
        structuring_policy_version: policy_version.to_owned(),
        output_object_count: 1,
        output_objects: vec![OutputObjectRef {
            object_family: "structured_intel_packet".to_owned(),
            key: structured_key.clone(),
            record_count: 1,
            byte_count: structured_bytes.len(),
        }],
        structured_packet_count: 1,
        context_flag_packet_count: 0,
        story_cluster_count: 0,
        health_event_count: 0,
    };
    let manifest_key = keys::manifest_key(created_at_ms, &raw_event_id, &run_id);
    let revision_index = PacketRevisionIndex {
        schema_version: PacketRevisionIndex::schema(),
        packet_family_id: packet_family_id.clone(),
        raw_event_id: raw_event_id.clone(),
        latest_revision: revision,
        latest_packet_id: packet_id.clone(),
        latest_structured_key: structured_key.clone(),
        market_context_status: revised_packet.market_context_status.clone(),
        updated_at_ms: created_at_ms,
    };
    let revision_index_key = keys::packet_revision_index_key(&packet_family_id, revision);
    let pointer = StructuredPointer {
        schema_version: StructuredPointer::schema(),
        packet_id,
        raw_event_id,
        terminal_decision: revised_packet.terminal_decision.clone(),
        storage_ref: S3ObjectPointer {
            bucket: bucket.to_owned(),
            key: structured_key.clone(),
            content_sha256: sha256_prefixed(&structured_bytes),
            schema_version: STRUCTURED_PACKET_SCHEMA_VERSION.to_owned(),
        },
        manifest_key: manifest_key.clone(),
        created_at_ms,
    };

    Ok(RevisionWritePlan {
        revised_packet,
        structured_key,
        structured_bytes,
        manifest,
        manifest_key,
        revision_index,
        revision_index_key,
        pointer,
    })
}

fn should_attempt_market_context_refresh(
    packet: &StructuredIntelPacket,
    options: &MarketContextRehydrationOptions,
) -> bool {
    matches!(
        packet.market_context_status,
        MarketContextStatus::Pending | MarketContextStatus::StaleButUsable
    ) || is_terminal_missing_market_context_reopen_candidate(packet, options)
}

fn is_terminal_missing_market_context_reopen_candidate(
    packet: &StructuredIntelPacket,
    options: &MarketContextRehydrationOptions,
) -> bool {
    options.include_terminal_missing_market_context
        && packet.market_context_status == MarketContextStatus::Unavailable
        && packet.market_context_terminal_reason.as_deref() == Some(TERMINAL_MISSING_MARKET_CONTEXT)
}

fn refreshed_context_warrants_revision(
    current: &MarketContextStatus,
    refreshed: &MarketContextStatus,
    terminal_reopen: bool,
) -> bool {
    if !refreshed.is_any_available() {
        return false;
    }
    if terminal_reopen {
        return true;
    }
    match current {
        MarketContextStatus::Pending => true,
        MarketContextStatus::StaleButUsable => {
            !matches!(refreshed, MarketContextStatus::StaleButUsable)
        }
        _ => false,
    }
}

fn is_record_level_rehydration_error(error: &AppError) -> bool {
    matches!(error, AppError::Json(_) | AppError::Validation(_))
}

fn parse_revision_from_key(key: &str) -> Option<u32> {
    key.strip_suffix(".json")?
        .rsplit_once("revision=")?
        .1
        .parse()
        .ok()
}

fn effective_packet_family_id(packet: &StructuredIntelPacket) -> &str {
    if !packet.packet_family_id.trim().is_empty() {
        packet.packet_family_id.as_str()
    } else if !packet.raw_event_id.trim().is_empty() {
        packet.raw_event_id.as_str()
    } else if let Some(source_event_id) = packet.source_event_ids.first() {
        source_event_id.as_str()
    } else {
        packet.packet_id.as_str()
    }
}

fn effective_raw_event_id(packet: &StructuredIntelPacket) -> &str {
    if !packet.raw_event_id.trim().is_empty() {
        packet.raw_event_id.as_str()
    } else if let Some(source_event_id) = packet.source_event_ids.first() {
        source_event_id.as_str()
    } else {
        packet.packet_id.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MarketContextRehydrationOptions, build_revision_write_plan, effective_packet_family_id,
        effective_raw_event_id, is_record_level_rehydration_error, parse_revision_from_key,
        refreshed_context_warrants_revision, should_attempt_market_context_refresh,
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
            MarketContextSnapshot::unavailable(
                "terminal_missing_market_context",
                "published_at_ms",
            ),
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
}
