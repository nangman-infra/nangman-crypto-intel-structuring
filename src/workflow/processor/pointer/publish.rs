use super::super::IntelStructuringProcessor;
use super::super::outputs::{context_flag_pointer, structured_pointer};
use super::writes::StoredPacketOutputs;
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::PacketSet;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn publish_success_outputs(
        &self,
        raw_event: &RawIntelEvent,
        packet_set: &PacketSet,
        outputs: &StoredPacketOutputs,
    ) -> AppResult<()> {
        let structured_pointer = structured_pointer(
            packet_set,
            raw_event,
            self.output_store.bucket(),
            &outputs.object_keys.structured_key,
            &outputs.structured_bytes,
            &outputs.manifest_key,
            outputs.finished_at_ms,
        );
        let flag_pointer = context_flag_pointer(
            packet_set,
            raw_event,
            self.output_store.bucket(),
            outputs.object_keys.flag_key.as_deref(),
            outputs.flag_bytes.as_deref(),
            &outputs.manifest_key,
            outputs.finished_at_ms,
        );

        self.publisher
            .publish_structured_pointer(&packet_set.structured_packet, &structured_pointer)
            .await?;
        if let (Some(context_flag_packet), Some(flag_pointer)) =
            (&packet_set.context_flag_packet, &flag_pointer)
        {
            self.publisher
                .publish_context_flag_pointer(context_flag_packet, flag_pointer)
                .await?;
        }
        self.publisher
            .publish_health(&packet_set.health_event)
            .await?;
        self.publisher.flush().await?;
        Ok(())
    }
}
