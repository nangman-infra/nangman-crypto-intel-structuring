use super::text::normalize_whitespace;

#[derive(Debug, Clone)]
pub(super) struct Candidate {
    pub(super) text: String,
    pub(super) score: i32,
    pub(super) order: usize,
}

pub(super) fn push_candidate(
    candidates: &mut Vec<Candidate>,
    text: &str,
    score: i32,
    order: &mut usize,
) {
    let normalized = normalize_whitespace(text);
    if normalized.chars().count() < 12 {
        return;
    }
    candidates.push(Candidate {
        text: normalized,
        score,
        order: *order,
    });
    *order += 1;
}
