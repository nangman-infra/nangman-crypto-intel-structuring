use super::env::{env_bool, env_i64, env_or};
use super::{Args, DEFAULT_AWS_REGION, DEFAULT_MARKET_L1_WINDOW_MS};
use crate::error::AppResult;
use crate::models::constants::{DEFAULT_ESCALATION_MODEL_ID, DEFAULT_PRIMARY_MODEL_ID};

mod build;

impl Args {
    pub(in crate::config) fn from_env() -> AppResult<Self> {
        let aws_region = env_or("AWS_REGION", DEFAULT_AWS_REGION);
        let enable_bedrock = env_bool("INTEL_L1_ENABLE_BEDROCK", false)?;
        let primary_model_id = env_or("INTEL_L1_PRIMARY_MODEL_ID", DEFAULT_PRIMARY_MODEL_ID);
        let escalation_model_id =
            env_or("INTEL_L1_ESCALATION_MODEL_ID", DEFAULT_ESCALATION_MODEL_ID);
        let bedrock =
            build::bedrock_config(enable_bedrock, &primary_model_id, &escalation_model_id)?;

        Ok(Self {
            nats: build::nats_config()?,
            raw_l0_store: build::raw_l0_store(),
            output_store: build::output_store(&aws_region),
            market_l1_store: build::market_l1_store(&aws_region),
            market_l1_window_ms: env_i64("INTEL_L1_MARKET_WINDOW_MS", DEFAULT_MARKET_L1_WINDOW_MS)?,
            bedrock,
            model_policy: build::model_policy_config(
                enable_bedrock,
                primary_model_id,
                escalation_model_id,
            )?,
            processing: build::processing_config()?,
            max_messages: build::max_messages()?,
            exit_on_idle: env_bool("INTEL_L1_EXIT_ON_IDLE", false)?,
        })
    }
}
