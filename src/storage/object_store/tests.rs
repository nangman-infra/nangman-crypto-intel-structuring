use super::config::validate_config;
use super::read::byte_range_header;
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
