use crate::models::output::{ConflictLevel, ContradictionFlag, StoryCluster, StoryMember};
use std::collections::{BTreeMap, BTreeSet};

use super::identity::event_type_label;

pub fn merge_story_members(base: &StoryCluster, members: Vec<StoryMember>) -> StoryCluster {
    let members = dedupe_members_by_event_id(members);
    if members.is_empty() {
        return base.clone();
    }

    let source_event_ids = members
        .iter()
        .map(|member| member.raw_event_id.clone())
        .collect::<Vec<_>>();
    let related_symbols = members
        .iter()
        .flat_map(|member| member.normalized_symbols.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let source_ids = members
        .iter()
        .map(|member| member.source_id.clone())
        .collect::<BTreeSet<_>>();
    let event_types = members
        .iter()
        .map(|member| event_type_label(&member.event_type).to_owned())
        .collect::<BTreeSet<_>>();
    let contradiction_flags = merged_contradiction_flags(&members, event_types.len() > 1);
    let conflict_level = conflict_level(&contradiction_flags, source_ids.len());
    let conflicting_source_ids =
        if matches!(conflict_level, ConflictLevel::Medium | ConflictLevel::High) {
            source_ids.iter().cloned().collect()
        } else {
            Vec::new()
        };

    StoryCluster {
        cluster_id: base.cluster_id.clone(),
        source_event_ids,
        story_hint_key: base.story_hint_key.clone(),
        primary_topic: primary_topic(&event_types),
        secondary_topics: event_types.into_iter().collect(),
        related_symbols,
        source_count: source_ids.len(),
        trust_mix: trust_mix(&members),
        first_published_at_ms: first_published_at_ms(&members),
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
        resolution_summary: resolution_summary(&contradiction_flags, source_ids.len()),
        schema_version: base.schema_version.clone(),
    }
}

fn dedupe_members_by_event_id(members: Vec<StoryMember>) -> Vec<StoryMember> {
    let mut by_event_id = BTreeMap::<String, StoryMember>::new();
    for member in members {
        by_event_id
            .entry(member.raw_event_id.clone())
            .or_insert(member);
    }
    by_event_id.into_values().collect()
}

fn merged_contradiction_flags(
    members: &[StoryMember],
    event_type_conflict: bool,
) -> Vec<ContradictionFlag> {
    let mut flags = members
        .iter()
        .flat_map(|member| member.contradiction_flags.clone())
        .collect::<BTreeSet<_>>();
    if event_type_conflict {
        flags.insert(ContradictionFlag::SourceClaimConflict);
    }
    flags.into_iter().collect()
}

fn conflict_level(flags: &[ContradictionFlag], source_count: usize) -> ConflictLevel {
    if flags.iter().any(|flag| {
        matches!(
            flag,
            ContradictionFlag::SourceClaimConflict | ContradictionFlag::RumorVsOfficial
        )
    }) {
        ConflictLevel::High
    } else if !flags.is_empty() {
        ConflictLevel::Medium
    } else if source_count > 1 {
        ConflictLevel::Low
    } else {
        ConflictLevel::None
    }
}

fn first_published_at_ms(members: &[StoryMember]) -> Option<i64> {
    members
        .iter()
        .filter_map(|member| member.published_at_ms)
        .min()
}

fn trust_mix(members: &[StoryMember]) -> String {
    let mut counts = BTreeMap::<String, usize>::new();
    for member in members {
        *counts.entry(member.trust_tier.clone()).or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(tier, count)| format!("{tier}={count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn primary_topic(event_types: &BTreeSet<String>) -> String {
    if event_types.len() == 1 {
        event_types
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| "other".to_owned())
    } else {
        "mixed_topic_conflict".to_owned()
    }
}

fn resolution_summary(flags: &[ContradictionFlag], source_count: usize) -> String {
    if flags.is_empty() && source_count <= 1 {
        "single source story".to_owned()
    } else if flags.is_empty() {
        "merged independent sources with no conflict detected".to_owned()
    } else {
        "merged sources with preserved conflict flags".to_owned()
    }
}
