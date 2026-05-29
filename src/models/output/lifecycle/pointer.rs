use crate::models::constants::{
    INDEX_POINTER_SCHEMA_VERSION, PACKET_REVISION_INDEX_SCHEMA_VERSION,
    STRUCTURED_POINTER_SCHEMA_VERSION,
};
use crate::models::market::MarketContextStatus;
use serde::{Deserialize, Serialize};

use super::super::common::TerminalDecision;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntelL1IndexPointer {
    pub schema_version: String,
    pub packet_id: String,
    pub raw_event_id: String,
    pub status: String,
    pub manifest_key: String,
    pub structured_packet_keys: Vec<String>,
    pub context_flag_keys: Vec<String>,
    pub finished_at_ms: i64,
    pub structuring_policy_version: String,
}

impl IntelL1IndexPointer {
    pub fn schema() -> String {
        INDEX_POINTER_SCHEMA_VERSION.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PacketRevisionIndex {
    pub schema_version: String,
    pub packet_family_id: String,
    pub raw_event_id: String,
    pub latest_revision: u32,
    pub latest_packet_id: String,
    pub latest_structured_key: String,
    pub market_context_status: MarketContextStatus,
    pub updated_at_ms: i64,
}

impl PacketRevisionIndex {
    pub fn schema() -> String {
        PACKET_REVISION_INDEX_SCHEMA_VERSION.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuredPointer {
    pub schema_version: String,
    pub packet_id: String,
    pub raw_event_id: String,
    pub terminal_decision: TerminalDecision,
    pub storage_ref: S3ObjectPointer,
    pub manifest_key: String,
    pub created_at_ms: i64,
}

impl StructuredPointer {
    pub fn schema() -> String {
        STRUCTURED_POINTER_SCHEMA_VERSION.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct S3ObjectPointer {
    pub bucket: String,
    pub key: String,
    pub content_sha256: String,
    pub schema_version: String,
}
