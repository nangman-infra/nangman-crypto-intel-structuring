use crate::models::output::StoryMember;
use std::collections::BTreeSet;

use super::super::identity::event_type_label;

pub(super) fn event_type_labels(members: &[StoryMember]) -> BTreeSet<String> {
    members
        .iter()
        .map(|member| event_type_label(&member.event_type).to_owned())
        .collect()
}

pub(super) fn primary_topic(event_types: &BTreeSet<String>) -> String {
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
