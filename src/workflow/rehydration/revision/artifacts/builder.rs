use super::super::ids::{effective_packet_family_id, effective_raw_event_id};
use super::super::packet::build_revised_packet;
use super::RevisionWritePlan;
use super::index::build_revision_index;
use super::manifest::build_manifest;
use super::pointer::{PointerBuildInput, build_pointer};
use crate::error::AppResult;
use crate::jsonl::build_jsonl_chunk;
use crate::models::market::MarketContextSnapshot;
use crate::models::output::StructuredIntelPacket;
use crate::structuring::packet::revised_packet_id;
use crate::time::run_id;
use crate::workflow::keys;

pub(in crate::workflow::rehydration) fn build_revision_write_plan(
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
    let terminal_missing_market_context = terminal_reason.is_some();
    let revised_packet = build_revised_packet(
        packet,
        market_context,
        terminal_reason,
        revision,
        packet_family_id.clone(),
        raw_event_id.clone(),
        packet_id.clone(),
    );

    let structured_key = keys::structured_packet_key(created_at_ms, &raw_event_id, &packet_id);
    let (structured_bytes, _) = build_jsonl_chunk(std::slice::from_ref(&revised_packet))?;
    let run_id = run_id("intel-l1-rehydration", created_at_ms);
    let manifest = build_manifest(
        &run_id,
        &raw_event_id,
        &structured_key,
        structured_bytes.len(),
        terminal_missing_market_context,
        created_at_ms,
        policy_version,
    );
    let manifest_key = keys::manifest_key(created_at_ms, &raw_event_id, &run_id);
    let revision_index = build_revision_index(
        &packet_family_id,
        &raw_event_id,
        revision,
        &packet_id,
        &structured_key,
        &revised_packet,
        created_at_ms,
    );
    let revision_index_key = keys::packet_revision_index_key(&packet_family_id, revision);
    let pointer = build_pointer(PointerBuildInput {
        packet_id,
        raw_event_id,
        revised_packet: &revised_packet,
        bucket,
        structured_key: &structured_key,
        structured_bytes: &structured_bytes,
        manifest_key: &manifest_key,
        created_at_ms,
    });

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
