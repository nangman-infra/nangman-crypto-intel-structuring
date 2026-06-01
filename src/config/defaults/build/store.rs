use super::super::super::env::{env_opt, env_or};
use super::super::super::{
    DEFAULT_MARKET_L1_BUCKET, DEFAULT_OUTPUT_BUCKET, DEFAULT_RAW_S3_BUCKET, DEFAULT_RAW_S3_REGION,
    ObjectStoreConfig,
};

pub(in crate::config::defaults) fn raw_l0_store() -> ObjectStoreConfig {
    ObjectStoreConfig {
        bucket: env_or("INTEL_L1_RAW_S3_BUCKET", DEFAULT_RAW_S3_BUCKET),
        region: env_or("INTEL_L1_RAW_S3_REGION", DEFAULT_RAW_S3_REGION),
        profile: env_opt("AWS_PROFILE"),
        access_key_id: None,
        secret_access_key: None,
    }
}

pub(in crate::config::defaults) fn output_store(aws_region: &str) -> ObjectStoreConfig {
    ObjectStoreConfig {
        bucket: env_or("INTEL_L1_OUTPUT_S3_BUCKET", DEFAULT_OUTPUT_BUCKET),
        region: env_or("INTEL_L1_OUTPUT_S3_REGION", aws_region),
        profile: env_opt("AWS_PROFILE"),
        access_key_id: None,
        secret_access_key: None,
    }
}

pub(in crate::config::defaults) fn market_l1_store(aws_region: &str) -> ObjectStoreConfig {
    ObjectStoreConfig {
        bucket: env_or("INTEL_L1_MARKET_L1_BUCKET", DEFAULT_MARKET_L1_BUCKET),
        region: env_or("INTEL_L1_MARKET_S3_REGION", aws_region),
        profile: env_opt("AWS_PROFILE"),
        access_key_id: None,
        secret_access_key: None,
    }
}
