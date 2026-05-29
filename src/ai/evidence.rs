mod candidate;
mod scoring;
mod text;

use self::candidate::push_candidate;
use self::scoring::score_text;
use self::text::{normalize_for_dedup, split_sentences, truncate_chars};
use crate::ai::contract::EvidenceSnippet;
use crate::models::raw::RawIntelEvent;
use std::collections::BTreeSet;

const DEFAULT_MAX_ITEMS: usize = 10;
const DEFAULT_MAX_TEXT_CHARS: usize = 420;

pub fn build_evidence_pack(event: &RawIntelEvent) -> Vec<EvidenceSnippet> {
    build_evidence_pack_with_limits(event, DEFAULT_MAX_ITEMS, DEFAULT_MAX_TEXT_CHARS)
}

pub fn build_evidence_pack_with_limits(
    event: &RawIntelEvent,
    max_items: usize,
    max_text_chars: usize,
) -> Vec<EvidenceSnippet> {
    let mut candidates = Vec::new();
    let mut order = 0;

    push_candidate(
        &mut candidates,
        event.title.trim(),
        score_text(event, event.title.trim()) + 10,
        &mut order,
    );

    for line in event.body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        for sentence in split_sentences(line) {
            let sentence = sentence.trim();
            if sentence.is_empty() {
                continue;
            }
            push_candidate(
                &mut candidates,
                sentence,
                score_text(event, sentence),
                &mut order,
            );
        }
    }

    let mut seen = BTreeSet::new();
    candidates.retain(|candidate| {
        let key = normalize_for_dedup(&candidate.text);
        if key.is_empty() || seen.contains(&key) {
            return false;
        }
        seen.insert(key);
        true
    });
    candidates.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.order.cmp(&right.order))
    });

    candidates
        .into_iter()
        .take(max_items)
        .enumerate()
        .map(|(index, candidate)| EvidenceSnippet {
            id: format!("E{}", index + 1),
            text: truncate_chars(&candidate.text, max_text_chars),
        })
        .collect()
}

#[cfg(test)]
mod tests;
