use super::RawIntelConsumer;
use super::deliver_policy::raw_deliver_policy;
use crate::error::{AppError, AppResult};
use crate::nats::config::NatsConfig;
use async_nats::jetstream;
use async_nats::jetstream::consumer::AckPolicy;
use std::time::Duration;

impl RawIntelConsumer {
    pub async fn connect(config: &NatsConfig) -> AppResult<Self> {
        let client = async_nats::connect(&config.url)
            .await
            .map_err(|error| AppError::nats(format!("connect {}: {error}", config.url)))?;
        let jetstream = jetstream::new(client);
        let stream = jetstream
            .get_stream(&config.raw_stream)
            .await
            .map_err(|error| {
                AppError::nats(format!("get raw stream {}: {error}", config.raw_stream))
            })?;
        let consumer = stream
            .get_or_create_consumer(&config.raw_consumer, raw_consumer_config(config)?)
            .await
            .map_err(|error| {
                AppError::nats(format!(
                    "get/create raw consumer {} on stream {}: {error}",
                    config.raw_consumer, config.raw_stream
                ))
            })?;
        Ok(Self {
            consumer,
            batch_size: config.batch_size.max(1),
        })
    }
}

fn raw_consumer_config(config: &NatsConfig) -> AppResult<jetstream::consumer::pull::Config> {
    Ok(jetstream::consumer::pull::Config {
        durable_name: Some(config.raw_consumer.clone()),
        filter_subject: config.raw_subject.clone(),
        ack_policy: AckPolicy::Explicit,
        ack_wait: Duration::from_secs(config.ack_wait_secs),
        max_deliver: config.max_deliver,
        max_ack_pending: config.batch_size as i64,
        deliver_policy: raw_deliver_policy(&config.raw_deliver_policy)?,
        ..Default::default()
    })
}
