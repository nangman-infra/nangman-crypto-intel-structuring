use super::support::is_permanent_failure;
use super::{AckDecision, IntelStructuringProcessor};
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::models::raw::RawIntelEventCreatedPointer;
use crate::nats::consumer::RawIntelMessage;
use crate::workflow::keys;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub async fn process_nats_message(&self, message: &RawIntelMessage) -> AppResult<AckDecision> {
        let pointer = match RawIntelEventCreatedPointer::parse(message.payload()) {
            Ok(pointer) => pointer,
            Err(error) => {
                self.write_quarantine(None, "invalid_pointer", false, error.to_string())
                    .await?;
                return Ok(AckDecision::Ack);
            }
        };

        if self
            .output_store
            .object_exists(&keys::index_key(
                &pointer.event_id,
                &self.config.structuring_policy_version,
            ))
            .await?
        {
            return Ok(AckDecision::Ack);
        }

        match self.process_pointer(pointer.clone()).await {
            Ok(()) => Ok(AckDecision::Ack),
            Err(error) if is_permanent_failure(&error) => {
                self.write_quarantine(
                    Some(pointer.event_id.as_str()),
                    "permanent_input_failure",
                    false,
                    error.to_string(),
                )
                .await?;
                Ok(AckDecision::Ack)
            }
            Err(error) => {
                eprintln!(
                    "{{\"level\":\"error\",\"raw_event_id\":\"{}\",\"ack\":\"no\",\"error\":{}}}",
                    pointer.event_id,
                    serde_json::to_string(&error.to_string())?
                );
                Ok(AckDecision::DoNotAck)
            }
        }
    }
}
