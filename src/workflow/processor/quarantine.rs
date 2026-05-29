use super::IntelStructuringProcessor;
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::hash::stable_short_id;
use crate::models::output::QuarantineEvent;
use crate::structuring::validation::{redact_forbidden_output_terms, validate_no_forbidden_output};
use crate::time::now_ms;
use crate::workflow::keys;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn write_quarantine(
        &self,
        raw_event_id: Option<&str>,
        failure_class: &str,
        retryable: bool,
        reason: String,
    ) -> AppResult<()> {
        let observed_at_ms = now_ms();
        let sanitized_reason = redact_forbidden_output_terms(&reason);
        let quarantine_id = stable_short_id(
            "intel_l1_quarantine",
            &[
                raw_event_id.unwrap_or("unknown"),
                failure_class,
                &sanitized_reason,
            ],
        );
        let event = QuarantineEvent::new(
            quarantine_id.clone(),
            raw_event_id.map(ToOwned::to_owned),
            observed_at_ms,
            failure_class,
            retryable,
            sanitized_reason,
        );
        validate_no_forbidden_output(&event)?;
        self.output_store
            .put_json_idempotent(
                &keys::quarantine_key(observed_at_ms, raw_event_id, &quarantine_id),
                &event,
            )
            .await?;
        Ok(())
    }
}
