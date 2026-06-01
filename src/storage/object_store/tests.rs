use super::config::validate_config;
use super::read::byte_range_header;
use super::validation::{validate_object_key, validate_object_prefix};
use super::write::is_precondition_failure;
use super::*;

#[test]
fn rejects_public_doc_bucket_placeholder() {
    let config = ObjectStoreConfig {
        bucket: "nangman-crypto-dev-intel-structuring-l1-<account-suffix>".to_owned(),
        region: "ap-northeast-2".to_owned(),
        profile: None,
        access_key_id: None,
        secret_access_key: None,
    };
    let err = validate_config(&config).unwrap_err();
    assert!(err.to_string().contains("public-doc placeholder"));
}

#[test]
fn builds_valid_byte_range_header() {
    assert_eq!(byte_range_header(5, 10).unwrap(), "bytes=5-14");
}

#[test]
fn rejects_zero_length_byte_range() {
    let err = byte_range_header(5, 0).unwrap_err();
    assert!(err.to_string().contains("invalid byte range"));
}

#[test]
fn validates_safe_object_key_shapes() {
    validate_object_key(
        "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=10/raw_event_id=raw_1/packet_id=pkt_1/part-000001.jsonl",
        "test object key",
    )
    .unwrap();
}

#[test]
fn rejects_unsafe_object_key_shapes() {
    let oversized_key = format!("prefix/{}", "a".repeat(1_024));
    for key in [
        "",
        " leading/object.json",
        "/absolute/object.json",
        "s3://bucket/object.json",
        "prefix/../object.json",
        "prefix/./object.json",
        "prefix//object.json",
        "prefix/object.json/",
        "prefix\\object.json",
        "prefix/object\n.json",
        "prefix/object name.json",
        "prefix/object.json?version=1",
        "prefix/object.json#fragment",
        oversized_key.as_str(),
    ] {
        let error = validate_object_key(key, "test object key")
            .expect_err("unsafe key must be rejected")
            .to_string();
        assert!(
            error.contains("test object key"),
            "expected object key label for {key:?}, got {error}"
        );
    }
}

#[test]
fn validates_safe_object_prefix_shapes() {
    validate_object_prefix(
        "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=10/",
        "test list prefix",
    )
    .unwrap();
    validate_object_prefix("structured-intel-packet", "test list prefix").unwrap();
}

#[test]
fn rejects_unsafe_object_prefix_shapes() {
    for prefix in [
        "",
        "/structured-intel-packet/",
        "s3://bucket/structured-intel-packet/",
        "structured-intel-packet/../",
        "structured-intel-packet//schema=x/",
        "structured-intel-packet/schema=x?bad=true",
        "structured-intel-packet/schema=x#bad",
        "structured-intel-packet\\schema=x",
        "structured-intel-packet/schema =x/",
    ] {
        let error = validate_object_prefix(prefix, "test list prefix")
            .expect_err("unsafe prefix must be rejected")
            .to_string();
        assert!(
            error.contains("test list prefix"),
            "expected prefix label for {prefix:?}, got {error}"
        );
    }
}

#[test]
fn detects_precondition_failure_from_metadata_code() {
    assert!(is_precondition_failure(
        Some("PreconditionFailed"),
        "service error"
    ));
}

#[test]
fn detects_precondition_failure_from_message_fallback() {
    assert!(is_precondition_failure(None, "S3 returned 412"));
}
