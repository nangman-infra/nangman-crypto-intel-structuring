use crate::ai::contract::{ModelProvider, ModelStage};
use crate::config::ModelPolicyConfig;
use crate::error::AppResult;
use crate::models::market::MarketContextSnapshot;
use crate::models::raw::RawIntelEvent;
use crate::structuring::nli::verify_model_response;
use crate::structuring::rule::assess;

mod admission;
mod decision;
mod escalation;
mod evidence_floor;
mod gates;
mod request;

use admission::should_bypass_models_for_cost;
pub use decision::StructuringDecision;
use escalation::try_escalation_or_fallback;
pub use evidence_floor::force_rule_evidence_floor;
use gates::{rule_is_sufficient, should_escalate_from_model};
use request::{model_repair_request, model_request};

pub struct ModelRouter<P: ModelProvider> {
    provider: P,
    policy: ModelPolicyConfig,
}

impl<P: ModelProvider> ModelRouter<P> {
    pub fn new(provider: P, policy: ModelPolicyConfig) -> Self {
        Self { provider, policy }
    }

    pub async fn decide(
        &self,
        event: &RawIntelEvent,
        market_context: &MarketContextSnapshot,
    ) -> AppResult<StructuringDecision> {
        let rule = assess(event);
        if rule_is_sufficient(event, &rule, market_context) {
            return Ok(StructuringDecision::rule_only(rule));
        }

        if should_bypass_models_for_cost(event, market_context, &rule) {
            return Ok(StructuringDecision::rule_only(rule));
        }

        if !self.policy.enable_bedrock {
            return Ok(StructuringDecision::fallback(rule, 0, 0));
        }

        let request = model_request(event, market_context, &rule);
        let primary = self.provider.structure(ModelStage::Primary, &request).await;
        let Ok(primary_response) = primary else {
            return try_escalation_or_fallback(
                &self.provider,
                &self.policy,
                event,
                market_context,
                rule,
                1,
            )
            .await;
        };
        let mut primary_invocations = 1;
        let primary_gate = verify_model_response(event, &primary_response);
        if primary_gate.supported
            && !should_escalate_from_model(
                event,
                market_context,
                &rule,
                &primary_response,
                &self.policy,
            )
        {
            return Ok(StructuringDecision::primary(rule, primary_response, 1));
        }

        if !primary_gate.supported && !rule.high_risk {
            primary_invocations += 1;
            let repair_request = model_repair_request(request, &primary_response, &primary_gate);
            if let Ok(repaired_response) = self
                .provider
                .structure(ModelStage::PrimaryRepair, &repair_request)
                .await
            {
                let repaired_gate = verify_model_response(event, &repaired_response);
                if repaired_gate.supported
                    && !should_escalate_from_model(
                        event,
                        market_context,
                        &rule,
                        &repaired_response,
                        &self.policy,
                    )
                {
                    return Ok(StructuringDecision::primary(rule, repaired_response, 2));
                }
            }
        }

        try_escalation_or_fallback(
            &self.provider,
            &self.policy,
            event,
            market_context,
            rule,
            primary_invocations,
        )
        .await
    }
}
