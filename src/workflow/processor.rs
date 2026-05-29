use crate::config::ProcessingConfig;
use crate::market::reader::MarketL1Reader;
use crate::nats::publisher::StructuredPublisher;
use crate::storage::object_store::ObjectStore;

mod ack;
mod message;
mod outputs;
mod pointer;
mod quarantine;
mod support;
mod telemetry;

pub use ack::AckDecision;

pub struct IntelStructuringProcessor<P>
where
    P: crate::ai::contract::ModelProvider,
{
    raw_l0_store: ObjectStore,
    output_store: ObjectStore,
    market_reader: MarketL1Reader,
    router: crate::structuring::router::ModelRouter<P>,
    publisher: StructuredPublisher,
    config: ProcessingConfig,
}

impl<P> IntelStructuringProcessor<P>
where
    P: crate::ai::contract::ModelProvider,
{
    pub fn new(
        raw_l0_store: ObjectStore,
        output_store: ObjectStore,
        market_reader: MarketL1Reader,
        router: crate::structuring::router::ModelRouter<P>,
        publisher: StructuredPublisher,
        config: ProcessingConfig,
    ) -> Self {
        Self {
            raw_l0_store,
            output_store,
            market_reader,
            router,
            publisher,
            config,
        }
    }
}
