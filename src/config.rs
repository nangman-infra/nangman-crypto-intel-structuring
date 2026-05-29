use crate::ai::bedrock::BedrockConfig;
use crate::nats::config::NatsConfig;
use crate::storage::object_store::ObjectStoreConfig;

mod defaults;
mod env;
mod parse;
#[cfg(test)]
mod tests;
mod validation;

pub const DEFAULT_NATS_URL: &str = "nats://127.0.0.1:4222";
pub const DEFAULT_RAW_STREAM: &str = "RAW_INTEL";
pub const DEFAULT_RAW_SUBJECT: &str = "raw_intel_event.created";
pub const DEFAULT_RAW_CONSUMER: &str = "intel-structuring-l1";
pub const DEFAULT_RAW_DELIVER_POLICY: &str = "all";
pub const DEFAULT_STRUCTURED_STREAM: &str = "STRUCTURED_INTEL";
pub const DEFAULT_STRUCTURED_PACKET_SUBJECT: &str = "structured_intel_packet.created";
pub const DEFAULT_CONTEXT_FLAG_SUBJECT: &str = "context_flag_packet.created";
pub const DEFAULT_HEALTH_SUBJECT: &str = "structuring_health_event.created";
pub const DEFAULT_RAW_S3_BUCKET: &str = "nangman-crypto-dev-intel-crawl-l0-<account-suffix>";
pub const DEFAULT_RAW_S3_REGION: &str = "ap-northeast-2";
pub const DEFAULT_OUTPUT_BUCKET: &str = "nangman-crypto-dev-intel-structuring-l1-<account-suffix>";
pub const DEFAULT_AWS_REGION: &str = "ap-northeast-2";
pub const DEFAULT_BEDROCK_REGION: &str = "us-east-1";
pub const DEFAULT_MARKET_L1_BUCKET: &str = "nangman-crypto-dev-market-ingest-l1-<account-suffix>";
pub const DEFAULT_MARKET_L1_WINDOW_MS: i64 = 1_000;
pub const DEFAULT_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS: i64 = 6 * 60 * 60 * 1_000;
pub const DEFAULT_MARKET_CONTEXT_STALE_AFTER_MS: i64 = 10 * 60 * 1_000;
pub const DEFAULT_MARKET_CONTEXT_RETRY_INTERVAL_MS: i64 = 5 * 60 * 1_000;
pub const DEFAULT_MARKET_CONTEXT_EXPIRE_AFTER_MS: i64 = 6 * 60 * 60 * 1_000;

#[derive(Debug, Clone)]
pub struct Args {
    pub nats: NatsConfig,
    pub raw_l0_store: ObjectStoreConfig,
    pub output_store: ObjectStoreConfig,
    pub market_l1_store: ObjectStoreConfig,
    pub market_l1_window_ms: i64,
    pub bedrock: BedrockConfig,
    pub model_policy: ModelPolicyConfig,
    pub processing: ProcessingConfig,
    pub max_messages: Option<usize>,
    pub exit_on_idle: bool,
}

#[derive(Debug, Clone)]
pub struct ModelPolicyConfig {
    pub primary_model_id: String,
    pub escalation_model_id: String,
    pub escalate_if_confidence_below: f64,
    pub escalation_budget_ratio: f64,
    pub enable_bedrock: bool,
}

#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    pub structuring_policy_version: String,
    pub chunk_max_records: usize,
    pub market_context_window_radius: i64,
    pub market_context_latest_before_lookback_ms: i64,
    pub market_context_stale_after_ms: i64,
    pub market_context_retry_interval_ms: i64,
    pub market_context_expire_after_ms: i64,
    pub max_raw_body_chars: usize,
    pub story_member_scan_limit: usize,
}
