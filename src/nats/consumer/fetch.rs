use super::{RawIntelConsumer, RawIntelMessage};
use crate::error::{AppError, AppResult};
use futures_util::StreamExt;
use std::time::Duration;

impl RawIntelConsumer {
    pub async fn next_message(&mut self) -> AppResult<Option<RawIntelMessage>> {
        let mut messages = self
            .consumer
            .fetch()
            .max_messages(self.batch_size)
            .expires(Duration::from_secs(5))
            .messages()
            .await
            .map_err(|error| AppError::nats(format!("fetch raw messages: {error}")))?;
        match messages.next().await {
            Some(Ok(message)) => Ok(Some(RawIntelMessage { inner: message })),
            Some(Err(error)) => Err(AppError::nats(format!("read raw message: {error}"))),
            None => Ok(None),
        }
    }
}
