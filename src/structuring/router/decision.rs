use crate::ai::contract::ModelStructuringResponse;
use crate::models::output::ModelTierUsed;
use crate::structuring::rule::RuleAssessment;

#[derive(Debug, Clone, PartialEq)]
pub struct StructuringDecision {
    pub rule: RuleAssessment,
    pub model_response: Option<ModelStructuringResponse>,
    pub model_tier_used: ModelTierUsed,
    pub fallback_count: usize,
    pub primary_invocations: usize,
    pub escalation_invocations: usize,
}

impl StructuringDecision {
    pub(super) fn rule_only(rule: RuleAssessment) -> Self {
        Self {
            rule,
            model_response: None,
            model_tier_used: ModelTierUsed::RuleOnly,
            fallback_count: 0,
            primary_invocations: 0,
            escalation_invocations: 0,
        }
    }

    pub(super) fn fallback(
        rule: RuleAssessment,
        primary_invocations: usize,
        escalation_invocations: usize,
    ) -> Self {
        Self {
            rule,
            model_response: None,
            model_tier_used: ModelTierUsed::FallbackOnly,
            fallback_count: 1,
            primary_invocations,
            escalation_invocations,
        }
    }

    pub(super) fn primary(
        rule: RuleAssessment,
        response: ModelStructuringResponse,
        primary_invocations: usize,
    ) -> Self {
        Self {
            rule,
            model_response: Some(response),
            model_tier_used: ModelTierUsed::Primary,
            fallback_count: 0,
            primary_invocations,
            escalation_invocations: 0,
        }
    }

    pub(super) fn escalation(
        rule: RuleAssessment,
        response: ModelStructuringResponse,
        primary_invocations: usize,
    ) -> Self {
        Self {
            rule,
            model_response: Some(response),
            model_tier_used: ModelTierUsed::Escalation,
            fallback_count: 0,
            primary_invocations,
            escalation_invocations: 1,
        }
    }
}
