use super::super::super::revision::build_revision_write_plan;
use super::super::fixtures::{available_market_context, packet_with_market_status};
use crate::models::market::MarketContextStatus;
use crate::models::output::EvidenceQualityReason;
use crate::structuring::packet::revised_packet_id;

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
