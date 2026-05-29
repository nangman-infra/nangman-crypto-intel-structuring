use super::super::IntelStructuringProcessor;
use crate::ai::contract::ModelProvider;
use crate::error::{AppError, AppResult};
use crate::models::raw::{RawIntelEvent, RawIntelEventCreatedPointer};

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn read_verified_raw_event(
        &self,
        pointer: &RawIntelEventCreatedPointer,
    ) -> AppResult<RawIntelEvent> {
        if pointer.storage_ref.bucket != self.raw_l0_store.bucket() {
            return Err(AppError::validation(format!(
                "raw pointer bucket mismatch pointer={} configured={}",
                pointer.storage_ref.bucket,
                self.raw_l0_store.bucket()
            )));
        }
        let raw_bytes = self
            .raw_l0_store
            .get_byte_range(
                &pointer.storage_ref.key,
                pointer.storage_ref.byte_offset,
                pointer.storage_ref.byte_length,
            )
            .await?;
        RawIntelEvent::parse_verified(&raw_bytes, pointer)
    }
}
