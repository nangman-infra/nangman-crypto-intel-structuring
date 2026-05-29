use crate::ai::bedrock::prompt::{build_dynamic_prompt, build_static_prompt};
use crate::ai::bedrock::response::{extract_converse_text, extract_json_object};
use crate::ai::contract::{
    ModelProvider, ModelStage, ModelStructuringRequest, ModelStructuringResponse,
};
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::Client;
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ConversationRole, InferenceConfiguration, Message, SystemContentBlock,
};
use aws_types::region::Region;

mod prompt;
mod response;
mod schema;

#[derive(Debug, Clone)]
pub struct BedrockConfig {
    pub enabled: bool,
    pub region: String,
    pub profile: Option<String>,
    pub primary_model_id: String,
    pub escalation_model_id: String,
    pub max_input_chars: usize,
    pub max_output_tokens: i32,
    pub temperature: f32,
}

pub struct BedrockModelProvider {
    config: BedrockConfig,
    client: Option<Client>,
}

impl BedrockModelProvider {
    pub async fn new(config: BedrockConfig) -> AppResult<Self> {
        if !config.enabled {
            return Ok(Self {
                config,
                client: None,
            });
        }
        let mut loader = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(config.region.clone()));
        if let Some(profile) = &config.profile {
            loader = loader.profile_name(profile);
        }
        let sdk_config = loader.load().await;
        Ok(Self {
            config,
            client: Some(Client::new(&sdk_config)),
        })
    }

    fn model_id(&self, stage: ModelStage) -> &str {
        match stage {
            ModelStage::Primary | ModelStage::PrimaryRepair => &self.config.primary_model_id,
            ModelStage::Escalation => &self.config.escalation_model_id,
        }
    }
}

#[async_trait]
impl ModelProvider for BedrockModelProvider {
    async fn structure(
        &self,
        stage: ModelStage,
        request: &ModelStructuringRequest,
    ) -> AppResult<ModelStructuringResponse> {
        let Some(client) = &self.client else {
            return Err(AppError::bedrock("Bedrock model provider disabled"));
        };
        let static_prompt = build_static_prompt(stage);
        let dynamic_prompt = build_dynamic_prompt(request, self.config.max_input_chars);
        let max_tokens = match stage {
            ModelStage::Primary | ModelStage::PrimaryRepair => {
                self.config.max_output_tokens.min(800)
            }
            ModelStage::Escalation => self.config.max_output_tokens,
        };
        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(dynamic_prompt))
            .build()
            .map_err(|error| AppError::bedrock(format!("build Converse message: {error}")))?;
        let inference_config = InferenceConfiguration::builder()
            .max_tokens(max_tokens)
            .temperature(self.config.temperature)
            .build();
        let output = client
            .converse()
            .model_id(self.model_id(stage))
            .system(SystemContentBlock::Text(static_prompt))
            .messages(message)
            .inference_config(inference_config)
            .send()
            .await
            .map_err(|error| {
                AppError::bedrock(format!("converse {}: {error}", self.model_id(stage)))
            })?;
        let text = extract_converse_text(&output)?;
        let mut parsed: ModelStructuringResponse =
            serde_json::from_str(extract_json_object(&text)?)?;
        parsed.hydrate_evidence_sentences(&request.evidence_pack)?;
        parsed.validate_evidence_gate()?;
        Ok(parsed)
    }
}
