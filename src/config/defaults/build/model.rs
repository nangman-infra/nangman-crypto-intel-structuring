use super::super::super::env::{env_f32, env_f64, env_i32, env_opt, env_or, env_usize};
use super::super::super::{BedrockConfig, DEFAULT_BEDROCK_REGION, ModelPolicyConfig};
use crate::error::{AppError, AppResult};

pub(in crate::config::defaults) fn bedrock_config(
    enabled: bool,
    primary_model_id: &str,
    escalation_model_id: &str,
) -> AppResult<BedrockConfig> {
    Ok(BedrockConfig {
        enabled,
        region: env_or("BEDROCK_REGION", DEFAULT_BEDROCK_REGION),
        profile: env_opt("AWS_PROFILE"),
        primary_model_id: primary_model_id.to_owned(),
        escalation_model_id: escalation_model_id.to_owned(),
        max_input_chars: env_usize("INTEL_L1_MODEL_MAX_INPUT_CHARS", 12_000)?,
        max_output_tokens: env_i32("INTEL_L1_MODEL_MAX_OUTPUT_TOKENS", 1200)?,
        temperature: env_f32("INTEL_L1_MODEL_TEMPERATURE", 0.0)?,
    })
}

pub(in crate::config::defaults) fn model_policy_config(
    enable_bedrock: bool,
    primary_model_id: String,
    escalation_model_id: String,
) -> AppResult<ModelPolicyConfig> {
    Ok(ModelPolicyConfig {
        primary_model_id,
        escalation_model_id,
        escalate_if_confidence_below: env_f64("INTEL_L1_ESCALATE_IF_CONFIDENCE_BELOW", 0.65)?,
        escalation_budget_ratio: env_f64("INTEL_L1_ESCALATION_BUDGET_RATIO", 0.15)?,
        enable_bedrock,
    })
}

pub(in crate::config::defaults) fn max_messages() -> AppResult<Option<usize>> {
    env_opt("INTEL_L1_MAX_MESSAGES")
        .map(|value| {
            value.parse::<usize>().map_err(|_| {
                AppError::config(format!(
                    "INTEL_L1_MAX_MESSAGES invalid: {value} has invalid type"
                ))
            })
        })
        .transpose()
}
