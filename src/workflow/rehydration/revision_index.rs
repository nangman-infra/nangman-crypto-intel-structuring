use super::MarketContextRehydrator;
use super::revision::{effective_packet_family_id, parse_revision_from_key};
use crate::error::AppResult;
use crate::models::output::{PacketRevisionIndex, StructuredIntelPacket};
use crate::workflow::keys;

const REVISION_INDEX_MAX_KEYS: usize = 256;

impl MarketContextRehydrator {
    pub(super) async fn is_not_latest_revision(
        &self,
        packet: &StructuredIntelPacket,
    ) -> AppResult<bool> {
        let Some(index) = self
            .latest_revision_index(effective_packet_family_id(packet))
            .await?
        else {
            return Ok(false);
        };
        Ok(packet.revision < index.latest_revision)
    }

    pub(super) async fn latest_revision_index(
        &self,
        packet_family_id: &str,
    ) -> AppResult<Option<PacketRevisionIndex>> {
        let mut latest: Option<(u32, String)> = None;
        for key in self
            .output_store
            .list_keys(
                &keys::packet_revision_index_prefix(packet_family_id),
                REVISION_INDEX_MAX_KEYS,
            )
            .await?
        {
            let Some(revision) = parse_revision_from_key(&key) else {
                continue;
            };
            let replace = latest
                .as_ref()
                .is_none_or(|(current_revision, _)| revision > *current_revision);
            if replace {
                latest = Some((revision, key));
            }
        }
        let Some((_, key)) = latest else {
            return Ok(None);
        };
        self.output_store.get_json(&key).await.map(Some)
    }
}
