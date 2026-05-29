use super::classify::classify_event_type;
use super::symbols::normalize_symbols;
use crate::models::output::{ContradictionFlag, EventType};
use crate::models::raw::RawIntelEvent;

pub(super) fn evidence_candidates(event: &RawIntelEvent, text: &str) -> Vec<String> {
    let source = if event.body.trim().is_empty() {
        event.title.as_str()
    } else {
        event.body.as_str()
    };
    source
        .split(['.', '\n'])
        .map(str::trim)
        .filter(|sentence| sentence.len() >= 12)
        .filter(|sentence| {
            let lower = sentence.to_ascii_lowercase();
            text_keywords(text)
                .iter()
                .any(|keyword| lower.contains(keyword))
        })
        .take(3)
        .map(ToOwned::to_owned)
        .collect()
}

fn text_keywords(text: &str) -> Vec<&'static str> {
    [
        "list",
        "delist",
        "deposit",
        "withdraw",
        "hack",
        "exploit",
        "regulat",
        "proposal",
        "unlock",
        "funding",
        "partnership",
    ]
    .into_iter()
    .filter(|keyword| text.contains(keyword))
    .collect()
}

pub(super) fn contradiction_flags(
    event: &RawIntelEvent,
    text: &str,
    evidence: &[String],
) -> Vec<ContradictionFlag> {
    let mut flags = Vec::new();
    if event.title.to_ascii_lowercase().contains("rumor") || text.contains("unconfirmed") {
        flags.push(ContradictionFlag::RumorVsOfficial);
    }
    if !event.symbol_candidates.is_empty() && normalize_symbols(&event.symbol_candidates).len() > 3
    {
        flags.push(ContradictionFlag::SymbolAmbiguity);
    }
    if evidence.is_empty() && !matches!(classify_event_type(text), EventType::Other) {
        flags.push(ContradictionFlag::EvidenceWeak);
    }
    flags
}
