mod apply;
mod identity;
mod manager;
mod member;
mod merge;

pub use identity::{story_cluster_id, story_hint_key};
pub use manager::{StoryMergeManager, StoryMergeResult};
pub use merge::merge_story_members;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::constants::{STORY_CLUSTER_SCHEMA_VERSION, STORY_MEMBER_SCHEMA_VERSION};
    use crate::models::output::{
        ConfidenceBand, ConflictLevel, EventType, StoryCluster, StoryMember,
    };
    use crate::models::raw::RawIntelEvent;

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

    fn story_cluster() -> StoryCluster {
        StoryCluster {
            cluster_id: "story_1".to_owned(),
            source_event_ids: vec!["raw1".to_owned()],
            story_hint_key: "hint".to_owned(),
            primary_topic: "incident".to_owned(),
            secondary_topics: Vec::new(),
            related_symbols: vec!["ABC".to_owned()],
            source_count: 1,
            trust_mix: "T1=1".to_owned(),
            first_published_at_ms: Some(1),
            last_updated_at_ms: 1,
            novelty_score: 0.5,
            conflict_level: ConflictLevel::None,
            conflicting_source_ids: Vec::new(),
            resolution_summary: "single source story".to_owned(),
            schema_version: STORY_CLUSTER_SCHEMA_VERSION.to_owned(),
        }
    }

    fn member(raw_event_id: &str, source_id: &str, event_type: EventType) -> StoryMember {
        StoryMember {
            schema_version: STORY_MEMBER_SCHEMA_VERSION.to_owned(),
            story_hint_key: "hint".to_owned(),
            cluster_id: "story_1".to_owned(),
            raw_event_id: raw_event_id.to_owned(),
            source_id: source_id.to_owned(),
            source_category: "news".to_owned(),
            normalized_symbols: vec!["ABC".to_owned()],
            event_type,
            confidence_band: ConfidenceBand::Medium,
            contradiction_flags: Vec::new(),
            trust_tier: "T1".to_owned(),
            published_at_ms: Some(1),
            observed_at_ms: 1,
            novelty_score: 0.5,
            structuring_policy_version: "policy".to_owned(),
        }
    }

    fn raw_event(event_id: &str, source_id: &str, title: &str) -> RawIntelEvent {
        RawIntelEvent {
            event_id: event_id.to_owned(),
            source_id: source_id.to_owned(),
            source_category: "news".to_owned(),
            source_name: "News".to_owned(),
            fetched_at_ms: 1,
            published_at_ms: Some(1),
            observed_at_ms: 1,
            language: "en".to_owned(),
            title: title.to_owned(),
            body: title.to_owned(),
            url: "https://example.com".to_owned(),
            author_or_channel: None,
            trust_tier: "T1".to_owned(),
            cadence_tier: "low".to_owned(),
            content_hash: "h".to_owned(),
            dedup_key: event_id.to_owned(),
            symbol_candidates: vec!["ABC".to_owned()],
            event_category_hint: None,
            top50_relevance: "relevant".to_owned(),
            content_kind: Some("news_article".to_owned()),
            content_quality: Some("full_text".to_owned()),
            content_quality_score: Some(80),
            source_quality: Some("trusted_symbol_match".to_owned()),
            source_relevance_scope: Some("symbol_alias_match".to_owned()),
            direct_asset_count: Some(0),
            matched_asset_count: Some(1),
            historical_source_depth: None,
            backfill_window_start_ms: None,
            backfill_window_end_ms: None,
            source_time_range_verified: None,
            schema_version: "raw_intel_event_v1".to_owned(),
        }
    }
}
