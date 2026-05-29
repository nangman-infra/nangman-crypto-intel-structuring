use super::MarketContextRehydrator;
use super::revision::build_revision_write_plan;
use crate::error::AppResult;
use crate::models::market::MarketContextSnapshot;
use crate::models::output::StructuredIntelPacket;
use crate::time::now_ms;

impl MarketContextRehydrator {
    pub(super) async fn publish_revision(
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
