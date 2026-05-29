use std::collections::BTreeSet;

use super::MarketContextRehydrator;
use super::status::is_record_level_rehydration_error;
use crate::error::AppResult;

const STRUCTURED_PACKET_PREFIX: &str = "structured-intel-packet/schema=structured_intel_packet_v1/";

impl MarketContextRehydrator {
    pub async fn run_once(&self, max_packets: usize) -> AppResult<usize> {
        self.run_prefixes_once(&[STRUCTURED_PACKET_PREFIX.to_owned()], max_packets)
            .await
    }

    pub async fn run_prefixes_once(
        &self,
        prefixes: &[String],
        max_packets_per_prefix: usize,
    ) -> AppResult<usize> {
        let keys = self
            .list_rehydration_keys(prefixes, max_packets_per_prefix)
            .await?;
        let mut published = 0usize;
        for key in keys {
            match self.try_rehydrate_key(&key).await {
                Ok(true) => published += 1,
                Ok(false) => {}
                Err(error) if is_record_level_rehydration_error(&error) => {
                    eprintln!("market context rehydration skipped key={key}: {error}");
                }
                Err(error) => return Err(error),
            }
        }
        Ok(published)
    }

    async fn list_rehydration_keys(
        &self,
        prefixes: &[String],
        max_packets_per_prefix: usize,
    ) -> AppResult<Vec<String>> {
        let mut keys = BTreeSet::new();
        for prefix in prefixes {
            for key in self
                .output_store
                .list_keys(prefix, max_packets_per_prefix)
                .await?
            {
                keys.insert(key);
            }
        }
        Ok(keys.into_iter().collect())
    }
}
