mod indexes;
mod objects;

use super::super::IntelStructuringProcessor;
use super::super::outputs::{PacketObjectKeys, PacketOutputRefsInput};
use super::super::outputs::{packet_object_keys, packet_output_refs};
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::models::raw::RawIntelEvent;
use crate::structuring::packet::{ManifestBuildInput, PacketSet, build_manifest};
use crate::structuring::story::StoryMergeResult;
use crate::workflow::keys;

pub(super) struct StoredPacketOutputs {
    pub(super) object_keys: PacketObjectKeys,
    pub(super) structured_bytes: Vec<u8>,
    pub(super) flag_bytes: Option<Vec<u8>>,
    pub(super) manifest_key: String,
    pub(super) manifest_bytes: Vec<u8>,
    pub(super) finished_at_ms: i64,
}

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn write_success_outputs(
        &self,
        raw_event: &RawIntelEvent,
        packet_set: &PacketSet,
        story_merge: &StoryMergeResult,
        run_id: &str,
        observed_at_ms: i64,
    ) -> AppResult<StoredPacketOutputs> {
        let object_keys = packet_object_keys(observed_at_ms, raw_event, packet_set);
        self.write_story_member(story_merge).await?;
        let story_bytes = self.write_story_cluster(packet_set, &object_keys).await?;
        let structured_bytes = self
            .write_structured_packet(packet_set, &object_keys)
            .await?;
        self.write_revision_index(raw_event, packet_set, &object_keys, observed_at_ms)
            .await?;
        let flag_bytes = self.write_context_flag(packet_set, &object_keys).await?;
        let health_bytes = self.write_health(packet_set, &object_keys).await?;

        let output_objects = packet_output_refs(PacketOutputRefsInput {
            story_member_key: &story_merge.story_member_key,
            story_member_bytes: &story_merge.story_member_bytes,
            keys: &object_keys,
            story_bytes: &story_bytes,
            structured_bytes: &structured_bytes,
            flag_bytes: flag_bytes.as_deref(),
            health_bytes: &health_bytes,
        });
        let finished_at_ms = observed_at_ms;
        let manifest_key = keys::manifest_key(observed_at_ms, &raw_event.event_id, run_id);
        let manifest = build_manifest(
            ManifestBuildInput {
                run_id: run_id.to_owned(),
                raw_event_id: raw_event.event_id.clone(),
                status: "success".to_owned(),
                started_at_ms: observed_at_ms,
                finished_at_ms,
                policy_version: self.config.structuring_policy_version.clone(),
                output_objects,
            },
            packet_set,
        );
        let manifest_bytes = self
            .output_store
            .put_json_idempotent(&manifest_key, &manifest)
            .await?;
        self.write_prepared_index(
            raw_event,
            packet_set,
            &object_keys,
            &manifest_key,
            finished_at_ms,
        )
        .await?;

        Ok(StoredPacketOutputs {
            object_keys,
            structured_bytes,
            flag_bytes,
            manifest_key,
            manifest_bytes,
            finished_at_ms,
        })
    }
}
