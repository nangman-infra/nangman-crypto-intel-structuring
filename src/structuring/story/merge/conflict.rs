use crate::models::output::{ConflictLevel, ContradictionFlag, StoryMember};
use std::collections::BTreeSet;

pub(super) fn merged_contradiction_flags(
    members: &[StoryMember],
    event_type_conflict: bool,
) -> Vec<ContradictionFlag> {
    let mut flags = members
        .iter()
        .flat_map(|member| member.contradiction_flags.iter().cloned())
        .collect::<BTreeSet<_>>();
    if event_type_conflict {
        flags.insert(ContradictionFlag::SourceClaimConflict);
    }
    flags.into_iter().collect()
}

pub(super) fn conflict_level(flags: &[ContradictionFlag], source_count: usize) -> ConflictLevel {
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

pub(super) fn conflicting_source_ids(
    conflict_level: &ConflictLevel,
    source_ids: &BTreeSet<String>,
) -> Vec<String> {
    if matches!(
        conflict_level,
        &ConflictLevel::Medium | &ConflictLevel::High
    ) {
        source_ids.iter().cloned().collect()
    } else {
        Vec::new()
    }
}

pub(super) fn resolution_summary(flags: &[ContradictionFlag], source_count: usize) -> String {
    if flags.is_empty() && source_count <= 1 {
        "single source story".to_owned()
    } else if flags.is_empty() {
        "merged independent sources with no conflict detected".to_owned()
    } else {
        "merged sources with preserved conflict flags".to_owned()
    }
}
