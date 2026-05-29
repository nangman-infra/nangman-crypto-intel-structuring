use crate::models::raw::RawIntelEvent;

pub(super) fn score_text(event: &RawIntelEvent, text: &str) -> i32 {
    let lower = text.to_ascii_lowercase();
    let mut score = 0;
    for keyword in [
        "listing",
        "delisting",
        "remove",
        "suspend",
        "withdrawal",
        "deposit",
        "hack",
        "exploit",
        "incident",
        "security",
        "regulatory",
        "sec",
        "unlock",
        "governance",
        "proposal",
        "partnership",
        "funding",
        "open interest",
    ] {
        if lower.contains(keyword) {
            score += 4;
        }
    }
    for symbol in &event.symbol_candidates {
        let symbol = symbol.trim();
        if !symbol.is_empty()
            && text
                .to_ascii_uppercase()
                .contains(&symbol.to_ascii_uppercase())
        {
            score += 3;
        }
    }
    if event.source_category.contains("notice") || event.source_category.contains("exchange") {
        score += 2;
    }
    if event.trust_tier == "T1" {
        score += 1;
    }
    score
}
