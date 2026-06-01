use super::super::super::revision::parse_revision_from_key;

#[test]
fn parses_revision_index_key() {
    assert_eq!(
        parse_revision_from_key(
            "packet-revision-index/schema=intel_l1_packet_revision_index_v1/packet_family_id=family_1/revision=0000000007.json"
        ),
        Some(7)
    );
}

#[test]
fn rejects_non_terminal_revision_markers() {
    assert_eq!(
        parse_revision_from_key(
            "packet-revision-index/schema=intel_l1_packet_revision_index_v1/packet_family_id=family_1/revision=0000000007/marker.json"
        ),
        None
    );
    assert_eq!(
        parse_revision_from_key(
            "packet-revision-index/schema=intel_l1_packet_revision_index_v1/packet_family_id=family_1/not_revision=0000000007.json"
        ),
        None
    );
}
