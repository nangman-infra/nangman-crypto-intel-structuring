use super::super::support::object_ref;
use super::keys::PacketObjectKeys;
use crate::models::output::OutputObjectRef;

pub(in crate::workflow::processor) struct PacketOutputRefsInput<'a> {
    pub(in crate::workflow::processor) story_member_key: &'a str,
    pub(in crate::workflow::processor) story_member_bytes: &'a [u8],
    pub(in crate::workflow::processor) keys: &'a PacketObjectKeys,
    pub(in crate::workflow::processor) story_bytes: &'a [u8],
    pub(in crate::workflow::processor) structured_bytes: &'a [u8],
    pub(in crate::workflow::processor) flag_bytes: Option<&'a [u8]>,
    pub(in crate::workflow::processor) health_bytes: &'a [u8],
}

pub(in crate::workflow::processor) fn packet_output_refs(
    input: PacketOutputRefsInput<'_>,
) -> Vec<OutputObjectRef> {
    let mut output_objects = vec![
        object_ref(
            "story_member",
            input.story_member_key,
            1,
            input.story_member_bytes,
        ),
        object_ref("story_cluster", &input.keys.story_key, 1, input.story_bytes),
        object_ref(
            "structured_intel_packet",
            &input.keys.structured_key,
            1,
            input.structured_bytes,
        ),
        object_ref(
            "structuring_health_event",
            &input.keys.health_key,
            1,
            input.health_bytes,
        ),
    ];
    if let (Some(flag_key), Some(flag_bytes)) = (&input.keys.flag_key, input.flag_bytes) {
        output_objects.push(object_ref("context_flag_packet", flag_key, 1, flag_bytes));
    }
    output_objects
}
