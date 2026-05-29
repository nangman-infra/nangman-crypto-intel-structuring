use crate::models::output::{
    ContextFlagPacket, StoryCluster, StructuredIntelPacket, StructuringHealthEvent,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PacketSet {
    pub story_cluster: StoryCluster,
    pub structured_packet: StructuredIntelPacket,
    pub context_flag_packet: Option<ContextFlagPacket>,
    pub health_event: StructuringHealthEvent,
}
