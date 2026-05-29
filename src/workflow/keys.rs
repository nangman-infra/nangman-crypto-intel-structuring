mod indexes;
mod segments;
mod story;
mod time_scoped;

pub use indexes::{
    index_key, packet_revision_index_key, packet_revision_index_prefix, prepared_index_key,
};
pub use segments::path_segment;
pub use story::{story_member_key, story_member_prefix};
pub use time_scoped::{
    context_flag_key, health_key, manifest_key, quarantine_key, story_cluster_key,
    structured_packet_key,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_structured_key() {
        assert_eq!(
            structured_packet_key(0, "raw:e", "pkt/1"),
            "structured-intel-packet/schema=structured_intel_packet_v1/dt=1970-01-01/hour=00/raw_event_id=raw_e/packet_id=pkt_1/part-000001.jsonl"
        );
    }

    #[test]
    fn index_key_is_deterministic_for_redelivery() {
        assert_eq!(
            index_key("raw:e", "policy/1"),
            index_key("raw:e", "policy/1")
        );
        assert_ne!(
            index_key("raw:e", "policy/1"),
            index_key("raw:e", "policy/2")
        );
    }

    #[test]
    fn prepared_and_success_index_keys_are_separate() {
        assert_ne!(
            prepared_index_key("raw:e", "policy/1"),
            index_key("raw:e", "policy/1")
        );
        assert!(prepared_index_key("raw:e", "policy/1").contains("status=prepared"));
    }

    #[test]
    fn story_member_key_groups_by_hint_and_policy() {
        assert_eq!(
            story_member_key("hint/1", "policy/1", "raw:e"),
            "story-members/schema=story_member_v1/story_hint_key=hint_1/policy=policy_1/raw_event_id=raw_e.json"
        );
    }

    #[test]
    fn packet_revision_index_key_uses_contract_schema_version() {
        assert_eq!(
            packet_revision_index_key("family/1", 7),
            "packet-revision-index/schema=intel_l1_packet_revision_index_v1/packet_family_id=family_1/revision=0000000007.json"
        );
    }

    #[test]
    fn path_segment_blocks_period_only_segments() {
        assert_eq!(path_segment("."), "_");
        assert_eq!(path_segment(".."), "_");
        assert_eq!(
            story_member_key("..", "policy/1", "."),
            "story-members/schema=story_member_v1/story_hint_key=_/policy=policy_1/raw_event_id=_.json"
        );
    }

    #[test]
    fn path_segment_preserves_safe_dot_values() {
        assert_eq!(path_segment("policy.v1.2"), "policy.v1.2");
    }
}
