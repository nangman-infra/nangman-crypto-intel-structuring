use super::admission::{
    critical_rule_escalation_path, escalation_admission_allows, within_escalation_budget,
};
use super::decision::StructuringDecision;
use super::request::model_request;
use crate::ai::contract::{ModelProvider, ModelStage};
use crate::config::ModelPolicyConfig;
use crate::error::AppResult;
use crate::models::market::MarketContextSnapshot;
use crate::models::raw::RawIntelEvent;
use crate::structuring::nli::verify_model_response;
use crate::structuring::rule::RuleAssessment;

pub(super) async fn try_escalation_or_fallback<P: ModelProvider>(
    provider: &P,
    policy: &ModelPolicyConfig,
    event: &RawIntelEvent,
    market_context: &MarketContextSnapshot,
    rule: RuleAssessment,
    primary_invocations: usize,
) -> AppResult<StructuringDecision> {
    if !escalation_admission_allows(event, market_context, &rule, None) {
        return Ok(StructuringDecision::fallback(rule, primary_invocations, 0));
    }
    if !critical_rule_escalation_path(&rule)
        && !within_escalation_budget(&event.event_id, policy.escalation_budget_ratio)
    {
        return Ok(StructuringDecision::fallback(rule, primary_invocations, 0));
    }

    let request = model_request(event, market_context, &rule);
    match provider.structure(ModelStage::Escalation, &request).await {
        Ok(response) => {
            let gate = verify_model_response(event, &response);
            if !gate.supported {
                Ok(StructuringDecision::fallback(rule, primary_invocations, 1))
            } else {
                Ok(StructuringDecision::escalation(
                    rule,
                    response,
                    primary_invocations,
                ))
            }
        }
        Err(_) => Ok(StructuringDecision::fallback(rule, primary_invocations, 1)),
    }
}
