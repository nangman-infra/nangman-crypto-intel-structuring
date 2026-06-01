mod connect;
mod deliver_policy;
mod fetch;
mod message;

use async_nats::jetstream::consumer::PullConsumer;

pub struct RawIntelConsumer {
    pub(super) consumer: PullConsumer,
    pub(super) batch_size: usize,
}

pub struct RawIntelMessage {
    pub(super) inner: async_nats::jetstream::Message,
}
