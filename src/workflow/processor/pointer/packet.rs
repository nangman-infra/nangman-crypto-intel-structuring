use super::super::IntelStructuringProcessor;
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::{PacketSet, build_packet_set};
use crate::structuring::router::force_rule_evidence_floor;
use crate::structuring::story::{StoryMergeManager, StoryMergeResult};
use crate::structuring::validation::validate_no_forbidden_output;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn build_validated_packet_set(
        &self,
        raw_event: &RawIntelEvent,
        observed_at_ms: i64,
    ) -> AppResult<(PacketSet, StoryMergeResult)> {
        let market_context = self
            .market_reader
            .context_for(
                raw_event.published_at_ms,
                raw_event.fetched_at_ms,
                &raw_event.symbol_candidates,
            )
            .await;
        let mut decision = self.router.decide(raw_event, &market_context).await?;
        force_rule_evidence_floor(raw_event, &mut decision);
        let mut packet_set = build_packet_set(
            raw_event,
            &decision,
            market_context,
            &self.config.structuring_policy_version,
            observed_at_ms,
            self.config.market_context_retry_interval_ms,
            self.config.market_context_expire_after_ms,
        );
        let story_merge = StoryMergeManager::new(
            self.output_store.clone(),
            self.config.story_member_scan_limit,
        )
        .merge_current_event(
            raw_event,
            &mut packet_set,
            &self.config.structuring_policy_version,
            observed_at_ms,
        )
        .await?;
        validate_packet_set(&packet_set)?;
        Ok((packet_set, story_merge))
    }
}

fn validate_packet_set(packet_set: &PacketSet) -> AppResult<()> {
    validate_no_forbidden_output(&packet_set.story_cluster)?;
    validate_no_forbidden_output(&packet_set.structured_packet)?;
    if let Some(context_flag_packet) = &packet_set.context_flag_packet {
        validate_no_forbidden_output(context_flag_packet)?;
    }
    Ok(())
}
