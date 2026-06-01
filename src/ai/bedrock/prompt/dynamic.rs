use crate::ai::contract::ModelStructuringRequest;

pub(in crate::ai::bedrock) fn build_dynamic_prompt(
    request: &ModelStructuringRequest,
    max_input_chars: usize,
) -> String {
    let body_section = body_section(&request.body, max_input_chars);
    let repair_context = repair_context_section(request.repair_context.as_deref());
    format!(
        r#"Input:
raw_event_id: {}
source_id: {}
source_category: {}
url: {}
symbol_candidates: {:?}
event_category_hint: {:?}
top50_relevance: {}
content_kind: {}
content_quality: {}
content_quality_score: {}
source_quality: {}
source_relevance_scope: {}
rule_event_type: {:?}
rule_confidence: {}
market_context_status: {}
market_context_summary: {}
evidence_candidates: {:?}
evidence_pack: {:?}
title: {}
{}{}"#,
        request.raw_event_id,
        request.source_id,
        request.source_category,
        request.url,
        request.symbol_candidates,
        request.event_category_hint,
        request.top50_relevance,
        request.content_kind,
        request.content_quality,
        request.content_quality_score,
        request.source_quality,
        request.source_relevance_scope,
        request.rule_event_type,
        request.rule_confidence,
        request.market_context_status,
        request.market_context_summary,
        request.evidence_candidates,
        request.evidence_pack,
        request.title,
        body_section,
        repair_context
    )
}

fn body_section(body: &str, max_input_chars: usize) -> String {
    let body = truncate_body(body, max_input_chars);
    if body.trim().is_empty() {
        String::new()
    } else {
        format!("body_excerpt: {body}\n")
    }
}

fn truncate_body(body: &str, max_input_chars: usize) -> String {
    if body.chars().count() > max_input_chars {
        body.chars().take(max_input_chars).collect::<String>()
    } else {
        body.to_owned()
    }
}

fn repair_context_section(repair_context: Option<&str>) -> String {
    repair_context
        .map(|value| format!("repair_context: {value}\n"))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{body_section, repair_context_section};

    #[test]
    fn body_section_truncates_on_character_boundary() {
        assert_eq!(body_section("가나다abc", 4), "body_excerpt: 가나다a\n");
    }

    #[test]
    fn body_section_omits_empty_excerpt_after_trimming() {
        assert_eq!(body_section("   ", 10), "");
    }

    #[test]
    fn repair_context_section_is_optional() {
        assert_eq!(repair_context_section(None), "");
        assert_eq!(
            repair_context_section(Some("fix evidence")),
            "repair_context: fix evidence\n"
        );
    }
}
