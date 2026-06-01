use crate::models::output::StoryMember;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn dedupe_members_by_event_id(members: Vec<StoryMember>) -> Vec<StoryMember> {
    let mut by_event_id = BTreeMap::<String, StoryMember>::new();
    for member in members {
        by_event_id
            .entry(member.raw_event_id.clone())
            .or_insert(member);
    }
    by_event_id.into_values().collect()
}

pub(super) fn source_event_ids(members: &[StoryMember]) -> Vec<String> {
    members
        .iter()
        .map(|member| member.raw_event_id.clone())
        .collect()
}

pub(super) fn related_symbols(members: &[StoryMember]) -> Vec<String> {
    members
        .iter()
        .flat_map(|member| member.normalized_symbols.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn source_ids(members: &[StoryMember]) -> BTreeSet<String> {
    members
        .iter()
        .map(|member| member.source_id.clone())
        .collect()
}

pub(super) fn first_published_at_ms(members: &[StoryMember]) -> Option<i64> {
    members
        .iter()
        .filter_map(|member| member.published_at_ms)
        .min()
}

pub(super) fn trust_mix(members: &[StoryMember]) -> String {
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
