use super::super::super::IntelStructuringProcessor;
use super::super::super::outputs::PacketObjectKeys;
use crate::ai::contract::ModelProvider;
use crate::error::AppResult;
use crate::structuring::packet::PacketSet;
use crate::structuring::story::StoryMergeResult;

impl<P> IntelStructuringProcessor<P>
where
    P: ModelProvider,
{
    pub(super) async fn write_story_member(&self, story_merge: &StoryMergeResult) -> AppResult<()> {
        self.output_store
            .put_bytes_idempotent(
                &story_merge.story_member_key,
                story_merge.story_member_bytes.clone(),
                "application/json",
            )
            .await?;
        Ok(())
    }

    pub(super) async fn write_story_cluster(
        &self,
        packet_set: &PacketSet,
        object_keys: &PacketObjectKeys,
    ) -> AppResult<Vec<u8>> {
        self.output_store
            .put_jsonl_idempotent(
                &object_keys.story_key,
                std::slice::from_ref(&packet_set.story_cluster),
            )
            .await
    }

    pub(super) async fn write_structured_packet(
        &self,
        packet_set: &PacketSet,
        object_keys: &PacketObjectKeys,
    ) -> AppResult<Vec<u8>> {
        self.output_store
            .put_jsonl_idempotent(
                &object_keys.structured_key,
                std::slice::from_ref(&packet_set.structured_packet),
            )
            .await
    }

    pub(super) async fn write_context_flag(
        &self,
        packet_set: &PacketSet,
        object_keys: &PacketObjectKeys,
    ) -> AppResult<Option<Vec<u8>>> {
        if let (Some(flag_key), Some(context_flag_packet)) =
            (&object_keys.flag_key, &packet_set.context_flag_packet)
        {
            return self
                .output_store
                .put_jsonl_idempotent(flag_key, std::slice::from_ref(context_flag_packet))
                .await
                .map(Some);
        }
        Ok(None)
    }

    pub(super) async fn write_health(
        &self,
        packet_set: &PacketSet,
        object_keys: &PacketObjectKeys,
    ) -> AppResult<Vec<u8>> {
        self.output_store
            .put_jsonl_idempotent(
                &object_keys.health_key,
                std::slice::from_ref(&packet_set.health_event),
            )
            .await
    }
}
