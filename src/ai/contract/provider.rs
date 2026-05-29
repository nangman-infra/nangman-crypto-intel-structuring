use super::types::{ModelStage, ModelStructuringRequest, ModelStructuringResponse};
use crate::error::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ModelProvider: Send + Sync {
    async fn structure(
        &self,
        stage: ModelStage,
        request: &ModelStructuringRequest,
    ) -> AppResult<ModelStructuringResponse>;
}
