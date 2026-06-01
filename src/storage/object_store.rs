use crate::error::AppResult;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Builder as S3ConfigBuilder;
use aws_types::region::Region;

mod config;
mod read;
mod validation;
mod write;

#[cfg(test)]
mod tests;

use config::validate_config;
pub use validation::{validate_object_key, validate_object_prefix};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStoreConfig {
    pub bucket: String,
    pub region: String,
    pub profile: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

#[derive(Clone)]
pub struct ObjectStore {
    client: Client,
    bucket: String,
}

impl ObjectStore {
    pub async fn connect(config: ObjectStoreConfig) -> AppResult<Self> {
        validate_config(&config)?;
        let mut loader =
            aws_config::defaults(BehaviorVersion::latest()).region(Region::new(config.region));
        if let Some(profile) = config.profile {
            loader = loader.profile_name(profile);
        }
        let sdk_config = loader.load().await;
        let mut s3_builder = S3ConfigBuilder::from(&sdk_config);
        if let (Some(access_key_id), Some(secret_access_key)) =
            (config.access_key_id, config.secret_access_key)
        {
            s3_builder = s3_builder.credentials_provider(Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "intel-structuring-app-explicit-object-store",
            ));
        }
        let s3_config = s3_builder.build();
        let store = Self {
            client: Client::from_conf(s3_config),
            bucket: config.bucket,
        };
        store.head_bucket().await?;
        Ok(store)
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }
}
