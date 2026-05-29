use crate::models::constants::STORY_CLUSTER_SCHEMA_VERSION;
use crate::models::output::{ConflictLevel, ContradictionFlag, EventType, StoryCluster};
use crate::models::raw::RawIntelEvent;

pub(super) struct StoryClusterInput<'a> {
    pub(super) event: &'a RawIntelEvent,
    pub(super) observed_at_ms: i64,
    pub(super) story_hint_key: String,
    pub(super) cluster_id: String,
    pub(super) event_type: &'a EventType,
    pub(super) normalized_symbols: &'a [String],
    pub(super) novelty_score: f64,
    pub(super) contradiction_flags: &'a [ContradictionFlag],
}

pub(super) fn build_story_cluster(input: StoryClusterInput<'_>) -> StoryCluster {
    StoryCluster {
        cluster_id: input.cluster_id,
        source_event_ids: vec![input.event.event_id.clone()],
        story_hint_key: input.story_hint_key,
        primary_topic: format!("{:?}", input.event_type),
        secondary_topics: Vec::new(),
        related_symbols: input.normalized_symbols.to_vec(),
        source_count: 1,
        trust_mix: input.event.trust_tier.clone(),
        first_published_at_ms: input.event.published_at_ms,
        last_updated_at_ms: input.observed_at_ms,
        novelty_score: input.novelty_score,
        conflict_level: if input.contradiction_flags.is_empty() {
            ConflictLevel::None
        } else {
            ConflictLevel::Medium
        },
        conflicting_source_ids: Vec::new(),
        resolution_summary: "single source story".to_owned(),
        schema_version: STORY_CLUSTER_SCHEMA_VERSION.to_owned(),
    }
}
