use crate::models::output::{
    IntelL1Manifest, PacketRevisionIndex, StructuredIntelPacket, StructuredPointer,
};

#[derive(Debug, Clone)]
pub(in crate::workflow::rehydration) struct RevisionWritePlan {
    pub(in crate::workflow::rehydration) revised_packet: StructuredIntelPacket,
    pub(in crate::workflow::rehydration) structured_key: String,
    pub(in crate::workflow::rehydration) structured_bytes: Vec<u8>,
    pub(in crate::workflow::rehydration) manifest: IntelL1Manifest,
    pub(in crate::workflow::rehydration) manifest_key: String,
    pub(in crate::workflow::rehydration) revision_index: PacketRevisionIndex,
    pub(in crate::workflow::rehydration) revision_index_key: String,
    pub(in crate::workflow::rehydration) pointer: StructuredPointer,
}
