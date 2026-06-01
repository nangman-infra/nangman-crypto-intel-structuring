mod fixtures;

use crate::models::output::{ConflictLevel, EventType};

use super::{merge_story_members, story_hint_key};
use fixtures::{member, raw_event, story_cluster};

#[test]
fn same_symbol_event_type_and_day_share_story_hint() {
    let first = raw_event("raw1", "src1", "ABC exploit confirmed");
    let second = raw_event("raw2", "src2", "Protocol incident update for ABC");

    assert_eq!(
        story_hint_key(&first, &EventType::Incident, &["ABC".to_owned()]),
        story_hint_key(&second, &EventType::Incident, &["ABC".to_owned()])
    );
}

#[test]
fn merge_preserves_sources_and_conflict() {
    let base = story_cluster();
    let merged = merge_story_members(
        &base,
        vec![
            member("raw1", "src1", EventType::Incident),
            member("raw2", "src2", EventType::Regulatory),
        ],
    );

    assert_eq!(merged.source_event_ids, vec!["raw1", "raw2"]);
    assert_eq!(merged.source_count, 2);
    assert_eq!(merged.conflict_level, ConflictLevel::High);
    assert_eq!(merged.conflicting_source_ids, vec!["src1", "src2"]);
    assert!(merged.secondary_topics.contains(&"regulatory".to_owned()));
}

#[test]
fn source_count_counts_unique_sources_not_member_count() {
    let base = story_cluster();
    let merged = merge_story_members(
        &base,
        vec![
            member("raw1", "src1", EventType::Incident),
            member("raw2", "src1", EventType::Incident),
        ],
    );

    assert_eq!(merged.source_event_ids, vec!["raw1", "raw2"]);
    assert_eq!(merged.source_count, 1);
    assert_eq!(merged.conflict_level, ConflictLevel::None);
}
