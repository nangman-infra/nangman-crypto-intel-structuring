mod process;
mod publish;
mod revision;
mod revision_index;
mod scan;
mod status;

#[cfg(test)]
mod tests;

use crate::config::ProcessingConfig;
use crate::market::reader::MarketL1Reader;
use crate::nats::publisher::StructuredPublisher;
use crate::storage::object_store::ObjectStore;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MarketContextRehydrationOptions {
    pub include_terminal_missing_market_context: bool,
}

pub struct MarketContextRehydrator {
    output_store: ObjectStore,
    market_reader: MarketL1Reader,
    publisher: StructuredPublisher,
    config: ProcessingConfig,
    rehydration_options: MarketContextRehydrationOptions,
}

impl MarketContextRehydrator {
    pub fn new(
        output_store: ObjectStore,
        market_reader: MarketL1Reader,
        publisher: StructuredPublisher,
        config: ProcessingConfig,
        rehydration_options: MarketContextRehydrationOptions,
    ) -> Self {
        Self {
            output_store,
            market_reader,
            publisher,
            config,
            rehydration_options,
        }
    }
}
