use crate::models::constants::STORY_MEMBER_SCHEMA_VERSION;

use super::segments::path_segment;

pub fn story_member_prefix(story_hint_key: &str, policy_version: &str) -> String {
    format!(
        "story-members/schema={STORY_MEMBER_SCHEMA_VERSION}/story_hint_key={}/policy={}/",
        path_segment(story_hint_key),
        path_segment(policy_version)
    )
}

pub fn story_member_key(story_hint_key: &str, policy_version: &str, raw_event_id: &str) -> String {
    format!(
        "{}raw_event_id={}.json",
        story_member_prefix(story_hint_key, policy_version),
        path_segment(raw_event_id)
    )
}
