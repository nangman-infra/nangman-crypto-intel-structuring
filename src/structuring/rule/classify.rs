use crate::models::output::EventType;

pub(super) fn classify_event_type(text: &str) -> EventType {
    let rules = [
        (
            EventType::Delisting,
            ["delist", "remove trading pair", "trading pair removal"].as_slice(),
        ),
        (
            EventType::Listing,
            ["list", "listing", "new trading pair"].as_slice(),
        ),
        (
            EventType::DepositWithdrawal,
            [
                "deposit",
                "withdrawal",
                "suspend deposits",
                "suspend withdrawals",
            ]
            .as_slice(),
        ),
        (
            EventType::Incident,
            ["exploit", "hack", "breach", "incident", "outage", "halt"].as_slice(),
        ),
        (
            EventType::Regulatory,
            [
                "sec",
                "cftc",
                "lawsuit",
                "regulator",
                "regulatory",
                "sanction",
            ]
            .as_slice(),
        ),
        (EventType::TokenUnlock, ["unlock", "vesting"].as_slice()),
        (
            EventType::Governance,
            ["governance", "proposal", "vote"].as_slice(),
        ),
        (
            EventType::Partnership,
            ["partnership", "integrates", "collaboration"].as_slice(),
        ),
        (
            EventType::FundingShift,
            ["funding rate", "open interest", "liquidation"].as_slice(),
        ),
        (
            EventType::SocialBacklash,
            ["backlash", "criticism", "controversy"].as_slice(),
        ),
        (
            EventType::SocialHype,
            ["hype", "viral", "surge in mentions"].as_slice(),
        ),
        (
            EventType::MacroEvent,
            ["fomc", "inflation", "cpi", "rate cut", "fed"].as_slice(),
        ),
    ];
    rules
        .iter()
        .find_map(|(event_type, keywords)| {
            keywords
                .iter()
                .any(|keyword| text.contains(keyword))
                .then_some(event_type.clone())
        })
        .unwrap_or(EventType::Other)
}
