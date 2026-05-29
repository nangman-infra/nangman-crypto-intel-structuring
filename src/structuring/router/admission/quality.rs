use crate::models::raw::RawIntelEvent;

pub(super) fn is_numeric_market_snapshot(event: &RawIntelEvent) -> bool {
    event.source_quality_or_unknown() == "market_snapshot"
        || event.content_quality_or_unknown() == "numeric_observation"
}

pub(super) fn is_derivatives_snapshot_hint(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    value.contains("funding")
        || value.contains("open_interest")
        || value.contains("open interest")
        || value.contains("liquidation")
}

pub(super) fn is_derivatives_snapshot_source(event: &RawIntelEvent) -> bool {
    let source_id = event.source_id.to_ascii_lowercase();
    let content_kind = event.content_kind_or_unknown().to_ascii_lowercase();
    source_id.contains("funding")
        || source_id.contains("open_interest")
        || source_id.contains("open-interest")
        || source_id.contains("liquidation")
        || content_kind.contains("derivatives")
}

pub(super) fn is_low_quality_broad_scan(event: &RawIntelEvent) -> bool {
    matches!(
        event.content_quality_or_unknown(),
        "title_only" | "metadata_fallback"
    ) || matches!(
        event.source_quality_or_unknown(),
        "global_symbol_scan" | "metadata_fallback"
    ) || matches!(
        event.source_relevance_scope.as_deref(),
        Some("global_symbol_scan")
    )
}

pub(super) fn is_official_or_trusted_notice(event: &RawIntelEvent) -> bool {
    event.trust_tier == "T0"
        || event.source_id.contains("official")
        || event.source_id.contains("exchange")
        || event.source_category.contains("official")
        || event.source_category.contains("exchange")
        || event.source_category.contains("project")
        || matches!(
            event.source_quality.as_deref(),
            Some("official_source")
                | Some("official_notice")
                | Some("exchange_notice")
                | Some("project_notice")
                | Some("trusted_symbol_match")
        )
}

pub(in crate::structuring::router) fn raw_quality_requires_model(event: &RawIntelEvent) -> bool {
    matches!(
        event.source_quality_or_unknown(),
        "community_reaction" | "market_snapshot"
    ) || matches!(
        event.content_quality_or_unknown(),
        "title_only" | "metadata_fallback"
    )
}

pub(in crate::structuring::router) fn raw_quality_requires_escalation(
    event: &RawIntelEvent,
) -> bool {
    event.content_quality_score.is_some_and(|score| score < 45)
        || matches!(
            event.source_relevance_scope.as_deref(),
            Some("global_symbol_scan")
        )
}
