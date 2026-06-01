use super::super::super::env::{env_bool, env_i64, env_or, env_u64, env_usize};
use super::super::super::{
    DEFAULT_CONTEXT_FLAG_SUBJECT, DEFAULT_HEALTH_SUBJECT, DEFAULT_NATS_URL, DEFAULT_RAW_CONSUMER,
    DEFAULT_RAW_DELIVER_POLICY, DEFAULT_RAW_STREAM, DEFAULT_RAW_SUBJECT,
    DEFAULT_STRUCTURED_PACKET_SUBJECT, DEFAULT_STRUCTURED_STREAM, NatsConfig,
};
use crate::error::AppResult;

pub(in crate::config::defaults) fn nats_config() -> AppResult<NatsConfig> {
    Ok(NatsConfig {
        url: env_or("NATS_URL", DEFAULT_NATS_URL),
        raw_stream: env_or("INTEL_L1_RAW_NATS_STREAM", DEFAULT_RAW_STREAM),
        raw_subject: env_or("INTEL_L1_RAW_NATS_SUBJECT", DEFAULT_RAW_SUBJECT),
        raw_consumer: env_or("INTEL_L1_RAW_NATS_CONSUMER", DEFAULT_RAW_CONSUMER),
        raw_deliver_policy: env_or("INTEL_L1_RAW_DELIVER_POLICY", DEFAULT_RAW_DELIVER_POLICY),
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
        ensure_output_stream: env_bool("INTEL_L1_ENSURE_OUTPUT_STREAM", true)?,
        output_stream_max_age_secs: env_u64(
            "INTEL_L1_OUTPUT_STREAM_MAX_AGE_SECS",
            14 * 24 * 60 * 60,
        )?,
        output_stream_duplicate_window_secs: env_u64(
            "INTEL_L1_OUTPUT_STREAM_DUPLICATE_WINDOW_SECS",
            24 * 60 * 60,
        )?,
        ack_wait_secs: env_u64("INTEL_L1_RAW_ACK_WAIT_SECS", 300)?,
        max_deliver: env_i64("INTEL_L1_RAW_MAX_DELIVER", 20)?,
        batch_size: env_usize("INTEL_L1_RAW_BATCH_SIZE", 1)?,
    })
}
