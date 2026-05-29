use crate::ai::contract::{ModelStructuringRequest, ModelStructuringResponse};
use crate::ai::evidence::build_evidence_pack;
use crate::models::market::MarketContextSnapshot;
use crate::models::raw::RawIntelEvent;
use crate::structuring::nli::EvidenceGateResult;
use crate::structuring::rule::RuleAssessment;
use serde_json::json;
use std::collections::BTreeSet;

pub(super) fn model_request(
    event: &RawIntelEvent,
    market_context: &MarketContextSnapshot,
    rule: &RuleAssessment,
) -> ModelStructuringRequest {
    ModelStructuringRequest {
        raw_event_id: event.event_id.clone(),
        source_id: event.source_id.clone(),
        source_category: event.source_category.clone(),
        title: event.title.clone(),
        body: event.body.clone(),
        url: event.url.clone(),
        symbol_candidates: event.symbol_candidates.clone(),
        event_category_hint: event.event_category_hint.clone(),
        top50_relevance: event.top50_relevance.clone(),
        content_kind: event.content_kind_or_unknown().to_owned(),
        content_quality: event.content_quality_or_unknown().to_owned(),
        content_quality_score: event.content_quality_score_label(),
        source_quality: event.source_quality_or_unknown().to_owned(),
        source_relevance_scope: event.source_relevance_scope_or_unknown().to_owned(),
        rule_event_type: rule.event_type.clone(),
        rule_confidence: rule.confidence_score,
        evidence_candidates: rule.evidence_sentences.clone(),
        evidence_pack: build_evidence_pack(event),
        market_context_status: format!("{:?}", market_context.status),
        market_context_summary: market_context_summary(market_context, &event.symbol_candidates),
        repair_context: None,
    }
}

fn market_context_summary(market_context: &MarketContextSnapshot, symbols: &[String]) -> String {
    if !market_context.status.is_any_available() {
        return format!(
            "status={:?}; reason={}",
            market_context.status,
            market_context
                .unavailable_reason
                .as_deref()
                .unwrap_or("not_available")
        );
    }
    if market_context.symbol_summaries.is_empty() {
        return format!("status={:?}; symbol_summaries=empty", market_context.status);
    }

    let wanted = symbols
        .iter()
        .map(|symbol| symbol.trim().to_ascii_uppercase())
        .filter(|symbol| !symbol.is_empty())
        .collect::<BTreeSet<_>>();
    let mut summaries = Vec::new();
    for summary in &market_context.symbol_summaries {
        if !wanted.is_empty() && !wanted.contains(&summary.symbol.to_ascii_uppercase()) {
            continue;
        }
        summaries.push(format!(
            "symbol={} venue={} window={}..{} mid={} spread_bps={} trades={} volume={} completeness={}",
            summary.symbol,
            summary.venue,
            summary.window_start_ms,
            summary.window_end_ms,
            optional_f64(summary.mid_price),
            optional_f64(summary.spread_bps),
            summary.trade_count,
            summary.trade_volume,
            summary.slice_completeness
        ));
        if summaries.len() >= 8 {
            break;
        }
    }
    if summaries.is_empty() {
        return format!(
            "status={:?}; requested_symbols_missing_from_market_context",
            market_context.status
        );
    }
    summaries.join("; ")
}

fn optional_f64(value: Option<f64>) -> String {
    value
        .map(|number| format!("{number:.8}"))
        .unwrap_or_else(|| "null".to_owned())
}

pub(super) fn model_repair_request(
    mut request: ModelStructuringRequest,
    response: &ModelStructuringResponse,
    gate: &EvidenceGateResult,
) -> ModelStructuringRequest {
    request.repair_context = Some(
        json!({
            "previous_response": response,
            "gate_supported": gate.supported,
            "contradiction_flags": gate.contradiction_flags
        })
        .to_string(),
    );
    request
}
