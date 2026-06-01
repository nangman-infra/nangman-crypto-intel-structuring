use crate::error::{AppError, AppResult};
use async_nats::jetstream::consumer::DeliverPolicy;

pub(super) fn raw_deliver_policy(value: &str) -> AppResult<DeliverPolicy> {
    match value {
        "all" => Ok(DeliverPolicy::All),
        "new" => Ok(DeliverPolicy::New),
        "last" => Ok(DeliverPolicy::Last),
        "last_per_subject" => Ok(DeliverPolicy::LastPerSubject),
        other => Err(AppError::config(format!(
            "unsupported INTEL_L1_RAW_DELIVER_POLICY: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::raw_deliver_policy;
    use async_nats::jetstream::consumer::DeliverPolicy;

    #[test]
    fn parses_supported_deliver_policies() {
        assert!(matches!(
            raw_deliver_policy("all").unwrap(),
            DeliverPolicy::All
        ));
        assert!(matches!(
            raw_deliver_policy("new").unwrap(),
            DeliverPolicy::New
        ));
        assert!(matches!(
            raw_deliver_policy("last").unwrap(),
            DeliverPolicy::Last
        ));
        assert!(matches!(
            raw_deliver_policy("last_per_subject").unwrap(),
            DeliverPolicy::LastPerSubject
        ));
    }

    #[test]
    fn rejects_unknown_deliver_policy() {
        assert!(raw_deliver_policy("oldest").is_err());
    }
}
