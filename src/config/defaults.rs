use super::env::{
    env_bool, env_f32, env_f64, env_i32, env_i64, env_opt, env_or, env_u64, env_usize,
};
use super::*;
use crate::models::constants::{
    DEFAULT_ESCALATION_MODEL_ID, DEFAULT_PRIMARY_MODEL_ID, STRUCTURING_POLICY_VERSION,
};

impl Args {
    pub(in crate::config) fn from_env() -> Self {
        let aws_region = env_or("AWS_REGION", DEFAULT_AWS_REGION);
        let enable_bedrock = env_bool("INTEL_L1_ENABLE_BEDROCK", false);
        let primary_model_id = env_or("INTEL_L1_PRIMARY_MODEL_ID", DEFAULT_PRIMARY_MODEL_ID);
        let escalation_model_id =
            env_or("INTEL_L1_ESCALATION_MODEL_ID", DEFAULT_ESCALATION_MODEL_ID);

        Self {
            nats: NatsConfig {
                url: env_or("NATS_URL", DEFAULT_NATS_URL),
                raw_stream: env_or("INTEL_L1_RAW_NATS_STREAM", DEFAULT_RAW_STREAM),
                raw_subject: env_or("INTEL_L1_RAW_NATS_SUBJECT", DEFAULT_RAW_SUBJECT),
                raw_consumer: env_or("INTEL_L1_RAW_NATS_CONSUMER", DEFAULT_RAW_CONSUMER),
                raw_deliver_policy: env_or(
                    "INTEL_L1_RAW_DELIVER_POLICY",
                    DEFAULT_RAW_DELIVER_POLICY,
                ),
                structured_stream: env_or("INTEL_L1_OUTPUT_NATS_STREAM", DEFAULT_STRUCTURED_STREAM),
                structured_packet_subject: env_or(
                    "INTEL_L1_STRUCTURED_PACKET_SUBJECT",
                    DEFAULT_STRUCTURED_PACKET_SUBJECT,
                ),
                context_flag_subject: env_or(
                    "INTEL_L1_CONTEXT_FLAG_SUBJECT",
                    DEFAULT_CONTEXT_FLAG_SUBJECT,
                ),
                health_subject: env_or("INTEL_L1_HEALTH_SUBJECT", DEFAULT_HEALTH_SUBJECT),
                ensure_output_stream: env_bool("INTEL_L1_ENSURE_OUTPUT_STREAM", true),
                output_stream_max_age_secs: env_u64(
                    "INTEL_L1_OUTPUT_STREAM_MAX_AGE_SECS",
                    14 * 24 * 60 * 60,
                ),
                output_stream_duplicate_window_secs: env_u64(
                    "INTEL_L1_OUTPUT_STREAM_DUPLICATE_WINDOW_SECS",
                    24 * 60 * 60,
                ),
                ack_wait_secs: env_u64("INTEL_L1_RAW_ACK_WAIT_SECS", 300),
                max_deliver: env_i64("INTEL_L1_RAW_MAX_DELIVER", 20),
                batch_size: env_usize("INTEL_L1_RAW_BATCH_SIZE", 1),
            },
            raw_l0_store: ObjectStoreConfig {
                bucket: env_or("INTEL_L1_RAW_S3_BUCKET", DEFAULT_RAW_S3_BUCKET),
                region: env_or("INTEL_L1_RAW_S3_REGION", DEFAULT_RAW_S3_REGION),
                profile: env_opt("AWS_PROFILE"),
                access_key_id: None,
                secret_access_key: None,
            },
            output_store: ObjectStoreConfig {
                bucket: env_or("INTEL_L1_OUTPUT_S3_BUCKET", DEFAULT_OUTPUT_BUCKET),
                region: env_or("INTEL_L1_OUTPUT_S3_REGION", &aws_region),
                profile: env_opt("AWS_PROFILE"),
                access_key_id: None,
                secret_access_key: None,
            },
            market_l1_store: ObjectStoreConfig {
                bucket: env_or("INTEL_L1_MARKET_L1_BUCKET", DEFAULT_MARKET_L1_BUCKET),
                region: env_or("INTEL_L1_MARKET_S3_REGION", &aws_region),
                profile: env_opt("AWS_PROFILE"),
                access_key_id: None,
                secret_access_key: None,
            },
            market_l1_window_ms: env_i64("INTEL_L1_MARKET_WINDOW_MS", DEFAULT_MARKET_L1_WINDOW_MS),
            bedrock: BedrockConfig {
                enabled: enable_bedrock,
                region: env_or("BEDROCK_REGION", DEFAULT_BEDROCK_REGION),
                profile: env_opt("AWS_PROFILE"),
                primary_model_id: primary_model_id.clone(),
                escalation_model_id: escalation_model_id.clone(),
                max_input_chars: env_usize("INTEL_L1_MODEL_MAX_INPUT_CHARS", 12_000),
                max_output_tokens: env_i32("INTEL_L1_MODEL_MAX_OUTPUT_TOKENS", 1200),
                temperature: env_f32("INTEL_L1_MODEL_TEMPERATURE", 0.0),
            },
            model_policy: ModelPolicyConfig {
                primary_model_id,
                escalation_model_id,
                escalate_if_confidence_below: env_f64(
                    "INTEL_L1_ESCALATE_IF_CONFIDENCE_BELOW",
                    0.65,
                ),
                escalation_budget_ratio: env_f64("INTEL_L1_ESCALATION_BUDGET_RATIO", 0.15),
                enable_bedrock,
            },
            processing: ProcessingConfig {
                structuring_policy_version: env_or(
                    "INTEL_L1_STRUCTURING_POLICY_VERSION",
                    STRUCTURING_POLICY_VERSION,
                ),
                chunk_max_records: env_usize("INTEL_L1_CHUNK_MAX_RECORDS", 1000),
                market_context_window_radius: env_i64("INTEL_L1_MARKET_CONTEXT_RADIUS_WINDOWS", 1),
                market_context_latest_before_lookback_ms: env_i64(
                    "INTEL_L1_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS",
                    DEFAULT_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS,
                ),
                market_context_stale_after_ms: env_i64(
                    "INTEL_L1_MARKET_CONTEXT_STALE_AFTER_MS",
                    DEFAULT_MARKET_CONTEXT_STALE_AFTER_MS,
                ),
                market_context_retry_interval_ms: env_i64(
                    "INTEL_L1_MARKET_CONTEXT_RETRY_INTERVAL_MS",
                    DEFAULT_MARKET_CONTEXT_RETRY_INTERVAL_MS,
                ),
                market_context_expire_after_ms: env_i64(
                    "INTEL_L1_MARKET_CONTEXT_EXPIRE_AFTER_MS",
                    DEFAULT_MARKET_CONTEXT_EXPIRE_AFTER_MS,
                ),
                max_raw_body_chars: env_usize("INTEL_L1_MAX_RAW_BODY_CHARS", 20_000),
                story_member_scan_limit: env_usize("INTEL_L1_STORY_MEMBER_SCAN_LIMIT", 128),
            },
            max_messages: env_opt("INTEL_L1_MAX_MESSAGES").and_then(|value| value.parse().ok()),
            exit_on_idle: env_bool("INTEL_L1_EXIT_ON_IDLE", false),
        }
    }
}
