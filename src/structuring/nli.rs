use crate::ai::contract::ModelStructuringResponse;
use crate::models::output::{ConfidenceBand, ContradictionFlag, EventType, TerminalDecision};
use crate::models::raw::RawIntelEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceGateResult {
    pub supported: bool,
    pub contradiction_flags: Vec<ContradictionFlag>,
}

pub fn verify_rule_evidence(
    event: &RawIntelEvent,
    evidence_sentences: &[String],
) -> EvidenceGateResult {
    let source = event.evidence_text(50_000).to_ascii_lowercase();
    let mut flags = Vec::new();
    for sentence in evidence_sentences {
        if !source.contains(&sentence.to_ascii_lowercase()) {
            flags.push(ContradictionFlag::EvidenceWeak);
        }
    }
    EvidenceGateResult {
        supported: flags.is_empty(),
        contradiction_flags: flags,
    }
}

pub fn verify_model_response(
    event: &RawIntelEvent,
    response: &ModelStructuringResponse,
) -> EvidenceGateResult {
    let mut result = verify_rule_evidence(event, &response.evidence_sentences);
    if matches!(
        response.confidence_band,
        ConfidenceBand::High | ConfidenceBand::Strong
    ) && response.evidence_sentences.is_empty()
    {
        result.supported = false;
        result
            .contradiction_flags
            .push(ContradictionFlag::EvidenceWeak);
    }
    if response.normalized_symbols.is_empty()
        && matches!(response.symbol_confidence_band, ConfidenceBand::Strong)
    {
        result.supported = false;
        result
            .contradiction_flags
            .push(ContradictionFlag::SymbolAmbiguity);
    }
    let source = event.evidence_text(50_000).to_ascii_uppercase();
    let candidates = event
        .symbol_candidates
        .iter()
        .map(|symbol| symbol.trim().to_ascii_uppercase())
        .filter(|symbol| !symbol.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    for symbol in &response.normalized_symbols {
        let normalized = symbol.trim().to_ascii_uppercase();
        if !candidates.contains(&normalized) && !source.contains(&normalized) {
            result.supported = false;
            result
                .contradiction_flags
                .push(ContradictionFlag::SymbolAmbiguity);
        }
    }
    if is_single_numeric_snapshot(event)
        && matches!(response.event_type, EventType::FundingShift)
        && matches!(
            response.terminal_decision,
            TerminalDecision::HighConfidenceStructured | TerminalDecision::Conflicted
        )
    {
        result.supported = false;
        result
            .contradiction_flags
            .push(ContradictionFlag::EvidenceWeak);
    }
    result
}

fn is_single_numeric_snapshot(event: &RawIntelEvent) -> bool {
    event.source_quality_or_unknown() == "market_snapshot"
        || event.content_quality_or_unknown() == "numeric_observation"
}

#[cfg(test)]
mod tests;
