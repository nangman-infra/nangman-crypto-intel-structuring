pub(in crate::storage::object_store) fn is_precondition_failure(
    code: Option<&str>,
    message: &str,
) -> bool {
    matches!(code, Some("PreconditionFailed"))
        || message.contains("PreconditionFailed")
        || message.contains("precondition")
        || message.contains("412")
}
