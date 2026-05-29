use crate::ai::evidence::build_evidence_pack_with_limits;
use crate::models::output::TextEvidence;
use crate::models::raw::RawIntelEvent;

const FALLBACK_TEXT_EVIDENCE_MAX_ITEMS: usize = 2;
const FALLBACK_TEXT_EVIDENCE_MAX_CHARS: usize = 360;

pub(in crate::structuring::packet) fn text_evidence(
    event: &RawIntelEvent,
    evidence_sentences: &[String],
) -> Vec<TextEvidence> {
    let explicit = evidence_sentences
        .iter()
        .filter(|sentence| !sentence.trim().is_empty())
        .map(|sentence| TextEvidence {
            evidence_text: sentence.clone(),
            source_event_id: event.event_id.clone(),
            source_id: event.source_id.clone(),
            published_at_ms: event.published_at_ms,
            evidence_kind: "source_sentence".to_owned(),
        })
        .collect::<Vec<_>>();
    if !explicit.is_empty() {
        return explicit;
    }

    fallback_text_evidence(event)
}

fn fallback_text_evidence(event: &RawIntelEvent) -> Vec<TextEvidence> {
    if !fallback_text_evidence_allowed(event) {
        return Vec::new();
    }

    build_evidence_pack_with_limits(
        event,
        FALLBACK_TEXT_EVIDENCE_MAX_ITEMS,
        FALLBACK_TEXT_EVIDENCE_MAX_CHARS,
    )
    .into_iter()
    .filter(|snippet| !snippet.text.trim().is_empty())
    .map(|snippet| TextEvidence {
        evidence_text: snippet.text,
        source_event_id: event.event_id.clone(),
        source_id: event.source_id.clone(),
        published_at_ms: event.published_at_ms,
        evidence_kind: "source_excerpt".to_owned(),
    })
    .collect()
}

fn fallback_text_evidence_allowed(event: &RawIntelEvent) -> bool {
    if event.symbol_candidates.len() != 1 {
        return false;
    }
    if event.source_relevance_scope_or_unknown() != "direct_asset" {
        return false;
    }
    if event.direct_asset_count.unwrap_or_default() != 1 {
        return false;
    }
    if matches!(
        event.content_quality_or_unknown(),
        "title_only" | "metadata_fallback" | "unknown"
    ) {
        return false;
    }
    if event.content_quality_score.is_some_and(|score| score < 45) {
        return false;
    }
    event.body.split_whitespace().count() >= 8
}
