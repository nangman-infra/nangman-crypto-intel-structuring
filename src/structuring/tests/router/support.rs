use crate::ai::contract::{
    ModelProvider, ModelStage, ModelStructuringRequest, ModelStructuringResponse,
};
use crate::error::AppResult;
use async_trait::async_trait;

pub(super) struct ScriptedProvider {
    pub(super) primary: Option<ModelStructuringResponse>,
    pub(super) primary_repair: Option<ModelStructuringResponse>,
    pub(super) escalation: Option<ModelStructuringResponse>,
}

#[async_trait]
impl ModelProvider for ScriptedProvider {
    async fn structure(
        &self,
        stage: ModelStage,
        _request: &ModelStructuringRequest,
    ) -> AppResult<ModelStructuringResponse> {
        match stage {
            ModelStage::Primary => self
                .primary
                .clone()
                .ok_or_else(|| crate::error::AppError::bedrock("primary unavailable")),
            ModelStage::PrimaryRepair => self
                .primary_repair
                .clone()
                .ok_or_else(|| crate::error::AppError::bedrock("primary repair unavailable")),
            ModelStage::Escalation => self
                .escalation
                .clone()
                .ok_or_else(|| crate::error::AppError::bedrock("escalation unavailable")),
        }
    }
}
