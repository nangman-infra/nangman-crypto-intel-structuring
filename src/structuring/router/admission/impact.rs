use crate::models::output::EventType;
use crate::structuring::rule::RuleAssessment;

pub(in crate::structuring::router) fn is_high_impact_event(event_type: &EventType) -> bool {
    matches!(
        event_type,
        EventType::Listing
            | EventType::Delisting
            | EventType::DepositWithdrawal
            | EventType::Incident
            | EventType::TokenUnlock
            | EventType::FundingShift
            | EventType::Regulatory
    )
}

pub(in crate::structuring::router) fn critical_rule_escalation_path(rule: &RuleAssessment) -> bool {
    rule.high_risk
}
