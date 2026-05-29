use crate::hash::stable_short_id;
use crate::models::output::EventType;
use crate::models::raw::RawIntelEvent;
use crate::time::time_part;
use std::collections::BTreeSet;

pub fn story_hint_key(event: &RawIntelEvent, event_type: &EventType, symbols: &[String]) -> String {
    let basis_ms = event.published_at_ms.unwrap_or(event.fetched_at_ms);
    let date = time_part(basis_ms).event_date;
    let symbol_signature = symbol_signature(symbols);
    let topic_signature = if symbol_signature == "general" {
        title_signature(&event.title)
    } else {
        event_type_label(event_type).to_owned()
    };
    stable_short_id(
        "story_hint",
        &[
            &date,
            event_type_label(event_type),
            &symbol_signature,
            &topic_signature,
        ],
    )
}

pub fn story_cluster_id(story_hint_key: &str, policy_version: &str) -> String {
    stable_short_id("story", &[story_hint_key, policy_version])
}

pub(super) fn event_type_label(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::Listing => "listing",
        EventType::Delisting => "delisting",
        EventType::DepositWithdrawal => "deposit_withdrawal",
        EventType::Incident => "incident",
        EventType::Partnership => "partnership",
        EventType::TokenUnlock => "token_unlock",
        EventType::Governance => "governance",
        EventType::FundingShift => "funding_shift",
        EventType::MacroEvent => "macro_event",
        EventType::Regulatory => "regulatory",
        EventType::SocialBacklash => "social_backlash",
        EventType::SocialHype => "social_hype",
        EventType::Other => "other",
    }
}

fn symbol_signature(symbols: &[String]) -> String {
    let signature = symbols
        .iter()
        .map(|symbol| symbol.trim().to_ascii_uppercase())
        .filter(|symbol| !symbol.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(",");
    if signature.is_empty() {
        "general".to_owned()
    } else {
        signature
    }
}

fn title_signature(title: &str) -> String {
    let normalized_title = title.to_ascii_lowercase();
    let tokens = normalized_title
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| token.len() >= 4)
        .filter(|token| !STOPWORDS.contains(token))
        .take(8)
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        "untitled".to_owned()
    } else {
        tokens.join("_")
    }
}

const STOPWORDS: &[&str] = &[
    "about", "after", "amid", "from", "into", "over", "that", "this", "with", "will",
];
