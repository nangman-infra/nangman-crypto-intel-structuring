use crate::error::AppResult;
use crate::models::output::StoryMember;
use crate::models::raw::RawIntelEvent;
use crate::storage::object_store::ObjectStore;
use crate::structuring::packet::PacketSet;
use crate::workflow::keys;

use super::apply::apply_story_cluster;
use super::merge::merge_story_members;

pub struct StoryMergeManager {
    store: ObjectStore,
    member_scan_limit: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoryMergeResult {
    pub story_member_key: String,
    pub story_member_bytes: Vec<u8>,
    pub member_count: usize,
}

impl StoryMergeManager {
    pub fn new(store: ObjectStore, member_scan_limit: usize) -> Self {
        Self {
            store,
            member_scan_limit: member_scan_limit.max(1),
        }
    }

    pub async fn merge_current_event(
        &self,
        event: &RawIntelEvent,
        packet_set: &mut PacketSet,
        policy_version: &str,
        observed_at_ms: i64,
    ) -> AppResult<StoryMergeResult> {
        let current_member =
            StoryMember::from_packet_set(event, packet_set, policy_version, observed_at_ms);
        let story_member_key = keys::story_member_key(
            &current_member.story_hint_key,
            policy_version,
            &current_member.raw_event_id,
        );
        let story_member_bytes = serde_json::to_vec_pretty(&current_member)?;

        let mut members = self
            .load_existing_members(&current_member, policy_version)
            .await?;
        members.push(current_member);

        let merged_cluster = merge_story_members(&packet_set.story_cluster, members);
        apply_story_cluster(packet_set, merged_cluster);

        Ok(StoryMergeResult {
            story_member_key,
            story_member_bytes,
            member_count: packet_set.story_cluster.source_event_ids.len(),
        })
    }

    async fn load_existing_members(
        &self,
        current_member: &StoryMember,
        policy_version: &str,
    ) -> AppResult<Vec<StoryMember>> {
        let prefix = keys::story_member_prefix(&current_member.story_hint_key, policy_version);
        let mut members = Vec::new();
        for key in self
            .store
            .list_keys(&prefix, self.member_scan_limit)
            .await?
        {
            let member = self.store.get_json::<StoryMember>(&key).await?;
            if member.raw_event_id == current_member.raw_event_id {
                continue;
            }
            if self
                .store
                .object_exists(&keys::index_key(&member.raw_event_id, policy_version))
                .await?
            {
                members.push(member);
            }
        }
        Ok(members)
    }
}
