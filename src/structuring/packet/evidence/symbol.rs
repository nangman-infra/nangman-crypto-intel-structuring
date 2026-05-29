use crate::models::output::{ConfidenceBand, SymbolResolutionTrace};
use crate::models::raw::RawIntelEvent;
use std::collections::BTreeSet;

pub(in crate::structuring::packet) fn symbol_resolution_trace(
    event: &RawIntelEvent,
    normalized_symbols: &[String],
    mapping_confidence: &ConfidenceBand,
) -> Vec<SymbolResolutionTrace> {
    let raw_mentions = event
        .symbol_candidates
        .iter()
        .map(|symbol| symbol.trim().to_ascii_uppercase())
        .filter(|symbol| !symbol.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if normalized_symbols.is_empty() {
        return vec![SymbolResolutionTrace {
            raw_mentions,
            resolved_project: None,
            resolved_asset: None,
            canonical_symbol: None,
            venue_symbols: Vec::new(),
            mapping_confidence: ConfidenceBand::Weak,
            ambiguity_reason: Some("no_resolved_symbol".to_owned()),
        }];
    }
    let ambiguity_reason = if normalized_symbols.len() > 1 {
        Some("multiple_candidate_symbols".to_owned())
    } else if matches!(
        mapping_confidence,
        ConfidenceBand::Weak | ConfidenceBand::Low
    ) {
        Some("weak_mapping_confidence".to_owned())
    } else {
        None
    };
    normalized_symbols
        .iter()
        .map(|symbol| SymbolResolutionTrace {
            raw_mentions: raw_mentions.clone(),
            resolved_project: Some(symbol.clone()),
            resolved_asset: Some(symbol.clone()),
            canonical_symbol: Some(symbol.clone()),
            venue_symbols: venue_symbols(symbol),
            mapping_confidence: mapping_confidence.clone(),
            ambiguity_reason: ambiguity_reason.clone(),
        })
        .collect()
}

fn venue_symbols(symbol: &str) -> Vec<String> {
    vec![
        format!("{symbol}USDT"),
        format!("{symbol}USD"),
        format!("KRW-{symbol}"),
    ]
}
