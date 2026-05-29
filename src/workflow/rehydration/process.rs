use super::MarketContextRehydrator;
use super::status::{
    TERMINAL_MISSING_MARKET_CONTEXT, is_terminal_missing_market_context_reopen_candidate,
    refreshed_context_warrants_revision, should_attempt_market_context_refresh,
};
use crate::error::AppResult;
use crate::models::market::{MarketContextSnapshot, MarketContextStatus};
use crate::models::output::StructuredIntelPacket;
use crate::time::now_ms;

impl MarketContextRehydrator {
    pub(super) async fn try_rehydrate_key(&self, key: &str) -> AppResult<bool> {
        let bytes = self.output_store.get_bytes(key).await?;
        let packet: StructuredIntelPacket = serde_json::from_slice(&bytes)?;
        let terminal_reopen =
            is_terminal_missing_market_context_reopen_candidate(&packet, &self.rehydration_options);
        if !should_attempt_market_context_refresh(&packet, &self.rehydration_options) {
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
}
