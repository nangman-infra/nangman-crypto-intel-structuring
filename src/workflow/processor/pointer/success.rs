use super::super::IntelStructuringProcessor;
use super::super::support::{IndexBuildInput, build_index};
use super::super::telemetry::{emit_success_metric, print_success_log};
use super::writes::StoredPacketOutputs;
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::PacketSet;
use crate::workflow::keys;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn finalize_success(
        &self,
        raw_event: &RawIntelEvent,
        packet_set: &PacketSet,
        outputs: &StoredPacketOutputs,
    ) -> AppResult<()> {
        let index_input = IndexBuildInput {
            packet_id: &packet_set.structured_packet.packet_id,
            raw_event_id: &raw_event.event_id,
            manifest_key: &outputs.manifest_key,
            structured_key: &outputs.object_keys.structured_key,
            flag_key: outputs.object_keys.flag_key.as_deref(),
            finished_at_ms: outputs.finished_at_ms,
            policy_version: &self.config.structuring_policy_version,
        };
        let index = build_index("success", &index_input);
        self.output_store
            .put_json_idempotent(
                &keys::index_key(&raw_event.event_id, &self.config.structuring_policy_version),
                &index,
            )
            .await?;

        emit_success_metric(raw_event, packet_set)?;
        print_success_log(raw_event, packet_set, &outputs.manifest_bytes)?;
        Ok(())
    }
}
