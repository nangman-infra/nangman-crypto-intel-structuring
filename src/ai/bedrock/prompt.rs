use super::schema::model_response_schema;
use crate::ai::contract::{ModelStage, ModelStructuringRequest};

pub(super) fn build_static_prompt(stage: ModelStage) -> String {
    let role = match stage {
        ModelStage::Primary => "Primary extractor. Be conservative and finish safe cases.",
        ModelStage::PrimaryRepair => {
            "Repair extractor. Fix only schema, evidence IDs, and confidence consistency."
        }
        ModelStage::Escalation => {
            "Escalation adjudicator. Resolve high impact ambiguity and produce a terminal decision."
        }
    };
    let schema = model_response_schema();
    format!(
        r#"You are INTEL-L1, a crypto market intelligence structuring worker.

Role:
{role}

Task:
- Convert one raw market-intelligence item into one JSON object.
- Use only supplied evidence IDs from evidence_pack.
- Return evidence_ids, not free-form source quotes.
- If a symbol is not directly supported by evidence, use an empty normalized_symbols list or weak symbol confidence.
- If the item is useful but not coin-specific, use general_market_context.
- If evidence is weak and the item is low impact, use unsupported_or_weak or low_confidence_structured.
- Use content_quality, source_quality, and source_relevance_scope as routing evidence quality hints.
- Use market_context_summary when available; if it is unavailable, do not invent cross-market confirmation.
- Never make a high-confidence structured claim from title_only, metadata_fallback, unknown, or global_symbol_scan evidence unless the evidence_pack directly supports it.
- Never make a high-confidence funding_shift claim from a single numeric snapshot without market_context_summary support.
- Never produce trading, execution, sizing, entry, exit, or live-readiness recommendations.
- Do not infer buy/sell direction.

Allowed event_type values:
listing, delisting, deposit_withdrawal, incident, partnership, token_unlock, governance,
funding_shift, macro_event, regulatory, social_backlash, social_hype, other

Allowed symbol_confidence_band values:
weak, moderate, strong

Allowed confidence_band values:
low, medium, high

Allowed relevance_decay_hint values:
minutes, hours, day, multi_day, structural

Allowed contradiction_flags values:
time_mismatch, symbol_ambiguity, source_claim_conflict, rumor_vs_official,
title_body_mismatch, evidence_weak

Allowed terminal_decision values:
high_confidence_structured, low_confidence_structured, general_market_context,
conflicted, unsupported_or_weak, irrelevant_or_noise

Output contract:
- Return exactly one JSON object.
- Do not wrap the JSON in markdown.
- Do not include commentary before or after the JSON.
- The JSON object must satisfy this schema:
{schema}
"#
    )
}

pub(super) fn build_dynamic_prompt(
    request: &ModelStructuringRequest,
    max_input_chars: usize,
) -> String {
    let body = if request.body.chars().count() > max_input_chars {
        request
            .body
            .chars()
            .take(max_input_chars)
            .collect::<String>()
    } else {
        request.body.clone()
    };
    let body_section = if body.trim().is_empty() {
        String::new()
    } else {
        format!("body_excerpt: {}\n", body)
    };
    let repair_context = request
        .repair_context
        .as_ref()
        .map(|value| format!("repair_context: {value}\n"))
        .unwrap_or_default();
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
