use crate::models::output::{ConfidenceBand, ContradictionFlag, EventType, TerminalDecision};

pub(super) fn rule_confidence(
    event_type: &EventType,
    symbol_band: &ConfidenceBand,
    evidence: &[String],
    contradictions: &[ContradictionFlag],
) -> f64 {
    let mut score: f64 = match event_type {
        EventType::Other => 0.35,
        EventType::Listing | EventType::Delisting | EventType::DepositWithdrawal => 0.72,
        EventType::Incident | EventType::Regulatory => 0.62,
        _ => 0.55,
    };
    if !evidence.is_empty() {
        score += 0.12;
    }
    if matches!(symbol_band, ConfidenceBand::Strong) {
        score += 0.08;
    }
    if !contradictions.is_empty() {
        score -= 0.2;
    }
    score.clamp(0.0, 1.0)
}

pub(super) fn confidence_band(score: f64) -> ConfidenceBand {
    if score >= 0.8 {
        ConfidenceBand::High
    } else if score >= 0.55 {
        ConfidenceBand::Medium
    } else {
        ConfidenceBand::Low
    }
}

pub(super) fn terminal_decision(
    score: f64,
    no_symbols: bool,
    conflicted: bool,
    high_risk: bool,
) -> TerminalDecision {
    if conflicted {
        TerminalDecision::Conflicted
    } else if score >= 0.8 {
        TerminalDecision::HighConfidenceStructured
    } else if no_symbols && score >= 0.45 {
        TerminalDecision::GeneralMarketContext
    } else if score >= 0.55 {
        TerminalDecision::LowConfidenceStructured
    } else if high_risk {
        TerminalDecision::UnsupportedOrWeak
    } else {
        TerminalDecision::IrrelevantOrNoise
    }
}
