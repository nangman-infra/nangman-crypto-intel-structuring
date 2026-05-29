use crate::error::AppResult;
use crate::hash::sha256_prefixed;
use crate::jsonl::build_jsonl_chunk;
use crate::models::constants::{MANIFEST_SCHEMA_VERSION, STRUCTURED_PACKET_SCHEMA_VERSION};
use crate::models::market::MarketContextSnapshot;
use crate::models::output::{
    EvidenceQualityReason, IntelL1Manifest, OutputObjectRef, PacketRevisionIndex, S3ObjectPointer,
    StructuredIntelPacket, StructuredPointer,
};
use crate::structuring::packet::{market_context_ref, revised_packet_id};
use crate::time::run_id;
use crate::workflow::keys;

#[derive(Debug, Clone)]
pub(super) struct RevisionWritePlan {
    pub(super) revised_packet: StructuredIntelPacket,
    pub(super) structured_key: String,
    pub(super) structured_bytes: Vec<u8>,
    pub(super) manifest: IntelL1Manifest,
    pub(super) manifest_key: String,
    pub(super) revision_index: PacketRevisionIndex,
    pub(super) revision_index_key: String,
    pub(super) pointer: StructuredPointer,
}

pub(super) fn build_revision_write_plan(
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
        revised_packet
            .evidence_quality_reasons
            .retain(|reason| !matches!(reason, EvidenceQualityReason::MarketContextMissing));
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

pub(super) fn parse_revision_from_key(key: &str) -> Option<u32> {
    key.strip_suffix(".json")?
        .rsplit_once("revision=")?
        .1
        .parse()
        .ok()
}

pub(super) fn effective_packet_family_id(packet: &StructuredIntelPacket) -> &str {
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

pub(super) fn effective_raw_event_id(packet: &StructuredIntelPacket) -> &str {
    if !packet.raw_event_id.trim().is_empty() {
        packet.raw_event_id.as_str()
    } else if let Some(source_event_id) = packet.source_event_ids.first() {
        source_event_id.as_str()
    } else {
        packet.packet_id.as_str()
    }
}
