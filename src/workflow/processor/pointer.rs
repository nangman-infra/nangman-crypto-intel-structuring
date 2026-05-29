mod input;
mod packet;
mod publish;
mod success;
mod writes;

use super::IntelStructuringProcessor;
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::models::raw::RawIntelEventCreatedPointer;

use super::support::policy_scoped_run_id;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn process_pointer(
        &self,
        pointer: RawIntelEventCreatedPointer,
    ) -> AppResult<()> {
        let raw_event = self.read_verified_raw_event(&pointer).await?;
        let observed_at_ms = raw_event.observed_at_ms;
        let run_id = policy_scoped_run_id(&self.config.structuring_policy_version, observed_at_ms);
        let (packet_set, story_merge) = self
            .build_validated_packet_set(&raw_event, observed_at_ms)
            .await?;
        let stored_outputs = self
            .write_success_outputs(
                &raw_event,
                &packet_set,
                &story_merge,
                &run_id,
                observed_at_ms,
            )
            .await?;
        self.publish_success_outputs(&raw_event, &packet_set, &stored_outputs)
            .await?;
        self.finalize_success(&raw_event, &packet_set, &stored_outputs)
            .await?;
        Ok(())
    }
}
