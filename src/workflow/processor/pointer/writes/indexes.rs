use super::super::super::IntelStructuringProcessor;
use super::super::super::outputs::PacketObjectKeys;
use super::super::super::support::{IndexBuildInput, build_index};
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::models::output::PacketRevisionIndex;
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::PacketSet;
use crate::workflow::keys;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn write_revision_index(
        &self,
        raw_event: &RawIntelEvent,
        packet_set: &PacketSet,
        object_keys: &PacketObjectKeys,
        observed_at_ms: i64,
    ) -> AppResult<()> {
        let revision_index = PacketRevisionIndex {
            schema_version: PacketRevisionIndex::schema(),
            packet_family_id: packet_set.structured_packet.packet_family_id.clone(),
            raw_event_id: raw_event.event_id.clone(),
            latest_revision: packet_set.structured_packet.revision,
            latest_packet_id: packet_set.structured_packet.packet_id.clone(),
            latest_structured_key: object_keys.structured_key.clone(),
            market_context_status: packet_set.structured_packet.market_context_status.clone(),
            updated_at_ms: observed_at_ms,
        };
        self.output_store
            .put_json_idempotent(
                &keys::packet_revision_index_key(
                    &packet_set.structured_packet.packet_family_id,
                    packet_set.structured_packet.revision,
                ),
                &revision_index,
            )
            .await?;
        Ok(())
    }

    pub(super) async fn write_prepared_index(
        &self,
        raw_event: &RawIntelEvent,
        packet_set: &PacketSet,
        object_keys: &PacketObjectKeys,
        manifest_key: &str,
        finished_at_ms: i64,
    ) -> AppResult<()> {
        let index_input = IndexBuildInput {
            packet_id: &packet_set.structured_packet.packet_id,
            raw_event_id: &raw_event.event_id,
            manifest_key,
            structured_key: &object_keys.structured_key,
            flag_key: object_keys.flag_key.as_deref(),
            finished_at_ms,
            policy_version: &self.config.structuring_policy_version,
        };
        let prepared_index = build_index("prepared", &index_input);
        self.output_store
            .put_json_idempotent(
                &keys::prepared_index_key(
                    &raw_event.event_id,
                    &self.config.structuring_policy_version,
                ),
                &prepared_index,
            )
            .await?;
        Ok(())
    }
}
