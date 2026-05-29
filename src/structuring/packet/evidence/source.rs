use crate::models::output::SourceIndependenceSummary;
use crate::models::raw::RawIntelEvent;

pub(in crate::structuring::packet) fn source_independence_summary(
    event: &RawIntelEvent,
) -> SourceIndependenceSummary {
    let source_quality = event.source_quality_or_unknown();
    let duplicate_content_hashes =
        if source_quality.contains("duplicate") || source_quality.contains("syndicated") {
            vec![event.content_hash.clone()]
        } else {
            Vec::new()
        };
    let syndicated_from = source_quality
        .contains("syndicated")
        .then(|| event.source_id.clone());
    SourceIndependenceSummary {
        source_event_count: 1,
        independent_source_count: 1,
        official_source_present: official_source_present(event),
        duplicate_content_hashes,
        syndicated_from,
        original_source_ids: vec![event.source_id.clone()],
    }
}

fn official_source_present(event: &RawIntelEvent) -> bool {
    let source_text = format!(
        "{} {} {}",
        event.source_id,
        event.source_category,
        event.source_quality_or_unknown()
    )
    .to_ascii_lowercase();
    source_text.contains("official")
        || source_text.contains("exchange")
        || source_text.contains("binance")
        || source_text.contains("upbit")
        || source_text.contains("bithumb")
        || source_text.contains("project")
        || source_text.contains("notice")
}

pub(in crate::structuring::packet) fn source_quality_summary(
    event: &RawIntelEvent,
    observed_at_ms: i64,
) -> String {
    format!(
        "{} source {} freshness_ms={} content_quality={} score={} source_quality={} relevance_scope={}",
        event.trust_tier,
        event.source_id,
        observed_at_ms.saturating_sub(event.fetched_at_ms),
        event.content_quality_or_unknown(),
        event.content_quality_score_label(),
        event.source_quality_or_unknown(),
        event.source_relevance_scope_or_unknown()
    )
}
