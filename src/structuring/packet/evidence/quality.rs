use crate::models::market::MarketContextSnapshot;
use crate::models::output::EvidenceQualityReason;
use crate::models::raw::RawIntelEvent;
use std::collections::BTreeSet;

pub(in crate::structuring::packet) fn evidence_quality_reasons(
    event: &RawIntelEvent,
    normalized_symbols: &[String],
    market_context: &MarketContextSnapshot,
) -> Vec<EvidenceQualityReason> {
    let mut reasons = BTreeSet::new();
    reasons.insert(EvidenceQualityReason::SingleSourceOnly);
    if matches!(event.content_quality.as_deref(), Some("title_only")) {
        reasons.insert(EvidenceQualityReason::TitleOnly);
    }
    if normalized_symbols.len() > 1 || event.symbol_candidates.len() > 1 {
        reasons.insert(EvidenceQualityReason::SymbolAmbiguous);
    }
    if !market_context.status.is_symbol_usable() {
        reasons.insert(EvidenceQualityReason::MarketContextMissing);
    }
    if event.source_quality_or_unknown() == "market_snapshot"
        || event.content_quality_or_unknown() == "numeric_observation"
    {
        reasons.insert(EvidenceQualityReason::SingleNumericSnapshot);
        reasons.insert(EvidenceQualityReason::BaselineMissing);
    }
    if event.source_quality_or_unknown().contains("syndicated")
        || event.source_quality_or_unknown().contains("duplicate")
    {
        reasons.insert(EvidenceQualityReason::DuplicateOrSyndicatedSource);
    }
    reasons.into_iter().collect()
}
