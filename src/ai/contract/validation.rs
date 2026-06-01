use super::types::{EvidenceSnippet, ModelStructuringResponse};
use crate::error::AppResult;

mod evidence;
mod fields;

use evidence::{hydrate_evidence_sentences, validate_evidence_requirements};
use fields::{
    validate_confidence_bands, validate_score_range, validate_symbol, validate_text_fields,
};

impl ModelStructuringResponse {
    pub fn hydrate_evidence_sentences(
        &mut self,
        evidence_pack: &[EvidenceSnippet],
    ) -> AppResult<()> {
        hydrate_evidence_sentences(self, evidence_pack)
    }

    pub fn validate_evidence_gate(&self) -> AppResult<()> {
        validate_confidence_bands(&self.symbol_confidence_band, &self.confidence_band)?;
        validate_evidence_requirements(
            &self.confidence_band,
            &self.terminal_decision,
            self.evidence_sentences.is_empty(),
        )?;
        validate_score_range(self.confidence_score, "model confidence_score must be 0..1")?;
        validate_score_range(self.novelty_score, "model novelty_score must be 0..1")?;
        validate_text_fields(self)?;
        for symbol in &self.normalized_symbols {
            validate_symbol(symbol)?;
        }
        Ok(())
    }
}
