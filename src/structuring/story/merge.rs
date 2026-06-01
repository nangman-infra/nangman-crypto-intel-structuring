mod conflict;
mod members;
mod topics;

use crate::models::output::{StoryCluster, StoryMember};

pub fn merge_story_members(base: &StoryCluster, members: Vec<StoryMember>) -> StoryCluster {
    let members = members::dedupe_members_by_event_id(members);
    if members.is_empty() {
        return base.clone();
    }

    let source_event_ids = members::source_event_ids(&members);
    let related_symbols = members::related_symbols(&members);
    let source_ids = members::source_ids(&members);
    let event_types = topics::event_type_labels(&members);
    let contradiction_flags = conflict::merged_contradiction_flags(&members, event_types.len() > 1);
    let conflict_level = conflict::conflict_level(&contradiction_flags, source_ids.len());
    let conflicting_source_ids = conflict::conflicting_source_ids(&conflict_level, &source_ids);

    StoryCluster {
        cluster_id: base.cluster_id.clone(),
        source_event_ids,
        story_hint_key: base.story_hint_key.clone(),
        primary_topic: topics::primary_topic(&event_types),
        secondary_topics: event_types.into_iter().collect(),
        related_symbols,
        source_count: source_ids.len(),
        trust_mix: members::trust_mix(&members),
        first_published_at_ms: members::first_published_at_ms(&members),
        last_updated_at_ms: members
            .iter()
            .map(|member| member.observed_at_ms)
            .max()
            .unwrap_or(base.last_updated_at_ms),
        novelty_score: members
            .iter()
            .map(|member| member.novelty_score)
            .fold(base.novelty_score, f64::max),
        conflict_level,
        conflicting_source_ids,
        resolution_summary: conflict::resolution_summary(&contradiction_flags, source_ids.len()),
        schema_version: base.schema_version.clone(),
    }
}
