use super::admission::{
    escalation_admission_allows, is_high_impact_event, numeric_snapshot_can_stop_at_primary,
    raw_quality_requires_escalation, raw_quality_requires_model, within_escalation_budget,
};
use crate::ai::contract::ModelStructuringResponse;
use crate::config::ModelPolicyConfig;
use crate::models::market::MarketContextSnapshot;
use crate::models::output::{ConfidenceBand, TerminalDecision};
use crate::models::raw::RawIntelEvent;
use crate::structuring::rule::RuleAssessment;

pub(super) fn rule_is_sufficient(
    event: &RawIntelEvent,
    rule: &RuleAssessment,
    market_context: &MarketContextSnapshot,
) -> bool {
    if raw_quality_requires_model(event) {
        return false;
    }
    let gate_supported = rule
        .evidence_sentences
        .iter()
        .all(|sentence| !sentence.trim().is_empty());
    rule.confidence_score >= 0.82
        && gate_supported
        && !rule.high_risk
        && !matches!(
            rule.symbol_confidence_band,
            ConfidenceBand::Weak | ConfidenceBand::Low
        )
        && !market_context.status.is_pending_or_unavailable()
}

pub(super) fn should_escalate_from_model(
    event: &RawIntelEvent,
    market_context: &MarketContextSnapshot,
    rule: &RuleAssessment,
    response: &ModelStructuringResponse,
    policy: &ModelPolicyConfig,
) -> bool {
    if !escalation_admission_allows(event, market_context, rule, Some(response)) {
        return false;
    }
    if rule.high_risk || matches!(response.terminal_decision, TerminalDecision::Conflicted) {
        return true;
    }
    if numeric_snapshot_can_stop_at_primary(event, market_context, response) {
        return false;
    }
    let high_impact = is_high_impact_event(&rule.event_type)
        || is_high_impact_event(&response.event_type)
        || event.source_category.contains("exchange");
    let weak_raw_claim = raw_quality_requires_escalation(event)
        && !matches!(
            response.terminal_decision,
            TerminalDecision::UnsupportedOrWeak
                | TerminalDecision::IrrelevantOrNoise
                | TerminalDecision::GeneralMarketContext
        );
    let weak_model_signal = response.confidence_score < policy.escalate_if_confidence_below
        || matches!(
            response.terminal_decision,
            TerminalDecision::UnsupportedOrWeak
        )
        || matches!(
            response.confidence_band,
            ConfidenceBand::Low | ConfidenceBand::Weak
        );
    let safety_escalation = high_impact && weak_model_signal;
    let audit_escalation =
        response.confidence_score < (policy.escalate_if_confidence_below + 0.10).min(1.0);
    let noncritical_escalation = weak_raw_claim || safety_escalation || audit_escalation;
    noncritical_escalation
        && within_escalation_budget(&event.event_id, policy.escalation_budget_ratio)
}
