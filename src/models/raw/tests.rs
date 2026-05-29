use crate::hash::sha256_prefixed;
use crate::models::constants::RAW_POINTER_SCHEMA_VERSION;

use super::*;

#[test]
fn rejects_wrong_pointer_schema() {
    let payload = br#"{
        "schema_version":"bad",
        "event_id":"e1",
        "source_id":"s",
        "source_category":"news",
        "fetched_at_ms":1,
        "published_at_ms":null,
        "created_at_ms":1,
        "content_hash":"h",
        "dedup_key":"d",
        "symbol_candidates":[],
        "top50_relevance":"unknown",
        "storage_ref":{"kind":"aws_s3_jsonl_record","endpoint_alias":"aws-s3-primary","bucket":"b","key":"k","line_number":1,"byte_offset":0,"byte_length":1,"content_sha256":"sha256:abc"}
    }"#;
    assert!(RawIntelEventCreatedPointer::parse(payload).is_err());
}

#[test]
fn verifies_raw_event_hash_and_pointer_identity() {
    let raw = br#"{"event_id":"e1","source_id":"s","source_category":"news","source_name":"S","fetched_at_ms":1,"published_at_ms":null,"observed_at_ms":1,"language":"en","title":"T","body":"B","url":"https://example.com","author_or_channel":null,"trust_tier":"T1","cadence_tier":"low","content_hash":"content-hash","dedup_key":"d","symbol_candidates":[],"event_category_hint":null,"top50_relevance":"unknown","schema_version":"raw_intel_event_v1"}"#;
    let pointer = RawIntelEventCreatedPointer {
        schema_version: RAW_POINTER_SCHEMA_VERSION.to_owned(),
        event_id: "e1".to_owned(),
        source_id: "s".to_owned(),
        source_category: "news".to_owned(),
        fetched_at_ms: 1,
        published_at_ms: None,
        created_at_ms: 1,
        content_hash: "content-hash".to_owned(),
        dedup_key: "d".to_owned(),
        symbol_candidates: Vec::new(),
        top50_relevance: "unknown".to_owned(),
        storage_ref: RawIntelEventStorageRef {
            kind: RAW_STORAGE_KIND_AWS_S3_JSONL_RECORD.to_owned(),
            endpoint_alias: "aws-s3-primary".to_owned(),
            bucket: "b".to_owned(),
            key: "k".to_owned(),
            line_number: 1,
            byte_offset: 0,
            byte_length: raw.len(),
            content_sha256: sha256_prefixed(raw),
        },
    };

    let parsed = RawIntelEvent::parse_verified(raw, &pointer).unwrap();
    assert_eq!(parsed.event_id, "e1");
}
