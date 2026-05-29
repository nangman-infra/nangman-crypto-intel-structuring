use crate::models::output::{ConfidenceBand, EventType, StructuredIntelPacket, TerminalDecision};

pub(super) fn context_flag(packet: &StructuredIntelPacket) -> String {
    match packet.event_type {
        EventType::MacroEvent => "macro_uncertainty",
        EventType::SocialBacklash | EventType::SocialHype => "social_attention_spike",
        EventType::FundingShift => "funding_stress",
        EventType::Other => "project_event",
        _ => "exchange_operational_event",
    }
    .to_owned()
}

pub(super) fn risk_flag(packet: &StructuredIntelPacket) -> String {
    match packet.event_type {
        EventType::Incident => "operational_risk",
        EventType::Delisting | EventType::Regulatory => "headline_risk",
        EventType::FundingShift => "volatility_risk",
        _ => "rumor_risk",
    }
    .to_owned()
}

pub(super) fn should_emit_context_flag(packet: &StructuredIntelPacket) -> bool {
    if packet.normalized_symbols.is_empty() {
        return false;
    }
    if matches!(
        packet.symbol_confidence_band,
        ConfidenceBand::Weak | ConfidenceBand::Low
    ) {
        return false;
    }
    if !matches!(
        packet.terminal_decision,
        TerminalDecision::HighConfidenceStructured | TerminalDecision::GeneralMarketContext
    ) {
        return false;
    }
    if matches!(packet.event_type, EventType::FundingShift)
        && !packet.market_context.status.is_symbol_usable()
    {
        return false;
    }
    true
}

pub(super) fn flag_confidence(confidence: &ConfidenceBand) -> ConfidenceBand {
    match confidence {
        ConfidenceBand::High | ConfidenceBand::Strong => ConfidenceBand::High,
        ConfidenceBand::Medium | ConfidenceBand::Moderate => ConfidenceBand::Medium,
        _ => ConfidenceBand::Low,
    }
}
