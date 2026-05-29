use super::decision::StructuringDecision;
use crate::models::output::ConfidenceBand;
use crate::models::raw::RawIntelEvent;
use crate::structuring::nli::verify_rule_evidence;

pub fn force_rule_evidence_floor(event: &RawIntelEvent, decision: &mut StructuringDecision) {
    if decision.model_response.is_none() {
        let gate = verify_rule_evidence(event, &decision.rule.evidence_sentences);
        if !gate.supported && matches!(decision.rule.confidence_band, ConfidenceBand::High) {
            decision.rule.confidence_band = ConfidenceBand::Medium;
        }
    }
}
