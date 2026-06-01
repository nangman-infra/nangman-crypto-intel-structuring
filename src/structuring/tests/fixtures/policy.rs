use crate::config::ModelPolicyConfig;
use crate::models::constants::{DEFAULT_ESCALATION_MODEL_ID, DEFAULT_PRIMARY_MODEL_ID};

pub(in crate::structuring::tests) fn policy(enable_bedrock: bool) -> ModelPolicyConfig {
    policy_with_escalation_budget(enable_bedrock, 0.15)
}

pub(in crate::structuring::tests) fn policy_with_escalation_budget(
    enable_bedrock: bool,
    escalation_budget_ratio: f64,
) -> ModelPolicyConfig {
    ModelPolicyConfig {
        primary_model_id: DEFAULT_PRIMARY_MODEL_ID.to_owned(),
        escalation_model_id: DEFAULT_ESCALATION_MODEL_ID.to_owned(),
        escalate_if_confidence_below: 0.65,
        escalation_budget_ratio,
        enable_bedrock,
    }
}
