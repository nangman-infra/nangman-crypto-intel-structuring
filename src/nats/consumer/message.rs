use super::RawIntelMessage;
use crate::error::{AppError, AppResult};

impl RawIntelMessage {
    pub fn payload(&self) -> &[u8] {
        &self.inner.payload
    }

    pub async fn ack(self) -> AppResult<()> {
        self.inner
            .double_ack()
            .await
            .map_err(|error| AppError::nats(format!("raw double ack failed: {error}")))
    }
}
