use crate::error::AppError;
use crate::hash::stable_short_id;
use crate::models::output::{IntelL1IndexPointer, OutputObjectRef};
use crate::models::raw::RawIntelEvent;
use crate::time::run_id;

pub(super) fn object_ref(
    family: &str,
    key: &str,
    record_count: usize,
    bytes: &[u8],
) -> OutputObjectRef {
    OutputObjectRef {
        object_family: family.to_owned(),
        key: key.to_owned(),
        record_count,
        byte_count: bytes.len(),
    }
}

pub(super) struct IndexBuildInput<'a> {
    pub(super) packet_id: &'a str,
    pub(super) raw_event_id: &'a str,
    pub(super) manifest_key: &'a str,
    pub(super) structured_key: &'a str,
    pub(super) flag_key: Option<&'a str>,
    pub(super) finished_at_ms: i64,
    pub(super) policy_version: &'a str,
}

pub(super) fn build_index(status: &str, input: &IndexBuildInput<'_>) -> IntelL1IndexPointer {
    IntelL1IndexPointer {
        schema_version: crate::models::constants::INDEX_POINTER_SCHEMA_VERSION.to_owned(),
        packet_id: input.packet_id.to_owned(),
        raw_event_id: input.raw_event_id.to_owned(),
        status: status.to_owned(),
        manifest_key: input.manifest_key.to_owned(),
        structured_packet_keys: vec![input.structured_key.to_owned()],
        context_flag_keys: input
            .flag_key
            .map(|key| vec![key.to_owned()])
            .unwrap_or_default(),
        finished_at_ms: input.finished_at_ms,
        structuring_policy_version: input.policy_version.to_owned(),
    }
}

pub(super) fn policy_scoped_run_id(policy_version: &str, observed_at_ms: i64) -> String {
    let policy_id = stable_short_id("policy", &[policy_version]);
    run_id(&format!("intel-l1-{policy_id}"), observed_at_ms)
}

pub(super) fn is_permanent_failure(error: &AppError) -> bool {
    matches!(error, AppError::Validation(_))
}

pub(super) fn is_numeric_market_snapshot(event: &RawIntelEvent) -> bool {
    event.source_quality_or_unknown() == "market_snapshot"
        || event.content_quality_or_unknown() == "numeric_observation"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_scoped_run_id_changes_with_policy_version() {
        let timestamp_ms = 1_779_658_292_837;
        let first = policy_scoped_run_id("policy_v1", timestamp_ms);
        let second = policy_scoped_run_id("policy_v2", timestamp_ms);

        assert_ne!(first, second);
        assert!(first.starts_with("intel-l1-policy_"));
        assert!(first.ends_with("-20260524T213132837Z"));
    }
}
