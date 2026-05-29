use crate::models::constants::{STORY_CLUSTER_SCHEMA_VERSION, STORY_MEMBER_SCHEMA_VERSION};
use serde::{Deserialize, Serialize};

use super::common::{ConfidenceBand, ContradictionFlag, EventType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryCluster {
    pub cluster_id: String,
    pub source_event_ids: Vec<String>,
    pub story_hint_key: String,
    pub primary_topic: String,
    pub secondary_topics: Vec<String>,
    pub related_symbols: Vec<String>,
    pub source_count: usize,
    pub trust_mix: String,
    pub first_published_at_ms: Option<i64>,
    pub last_updated_at_ms: i64,
    pub novelty_score: f64,
    pub conflict_level: ConflictLevel,
    pub conflicting_source_ids: Vec<String>,
    pub resolution_summary: String,
    pub schema_version: String,
}

impl StoryCluster {
    pub fn schema() -> String {
        STORY_CLUSTER_SCHEMA_VERSION.to_owned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictLevel {
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryMember {
    pub schema_version: String,
    pub story_hint_key: String,
    pub cluster_id: String,
    pub raw_event_id: String,
    pub source_id: String,
    pub source_category: String,
    pub normalized_symbols: Vec<String>,
    pub event_type: EventType,
    pub confidence_band: ConfidenceBand,
    pub contradiction_flags: Vec<ContradictionFlag>,
    pub trust_tier: String,
    pub published_at_ms: Option<i64>,
    pub observed_at_ms: i64,
    pub novelty_score: f64,
    pub structuring_policy_version: String,
}

impl StoryMember {
    pub fn schema() -> String {
        STORY_MEMBER_SCHEMA_VERSION.to_owned()
    }
}
