use serde_json::json;

pub(super) fn model_response_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "event_type": {
                "type": "string",
                "enum": [
                    "listing",
                    "delisting",
                    "deposit_withdrawal",
                    "incident",
                    "partnership",
                    "token_unlock",
                    "governance",
                    "funding_shift",
                    "macro_event",
                    "regulatory",
                    "social_backlash",
                    "social_hype",
                    "other"
                ]
            },
            "normalized_symbols": {
                "type": "array",
                "items": {"type": "string"}
            },
            "symbol_confidence_band": {
                "type": "string",
                "enum": ["weak", "moderate", "strong"]
            },
            "topic_summary": {"type": "string"},
            "stance_summary": {"type": "string"},
            "risk_summary": {"type": "string"},
            "regime_hint": {"type": "string"},
            "scenario_hint": {"type": "string"},
            "confidence_band": {
                "type": "string",
                "enum": ["low", "medium", "high"]
            },
            "confidence_score": {"type": "number"},
            "novelty_score": {"type": "number"},
            "relevance_decay_hint": {
                "type": "string",
                "enum": ["minutes", "hours", "day", "multi_day", "structural"]
            },
            "contradiction_flags": {
                "type": "array",
                "items": {
                    "type": "string",
                    "enum": [
                        "time_mismatch",
                        "symbol_ambiguity",
                        "source_claim_conflict",
                        "rumor_vs_official",
                        "title_body_mismatch",
                        "evidence_weak"
                    ]
                }
            },
            "evidence_ids": {
                "type": "array",
                "items": {"type": "string"}
            },
            "terminal_decision": {
                "type": "string",
                "enum": [
                    "high_confidence_structured",
                    "low_confidence_structured",
                    "general_market_context",
                    "conflicted",
                    "unsupported_or_weak",
                    "irrelevant_or_noise"
                ]
            }
        },
        "required": [
            "event_type",
            "normalized_symbols",
            "symbol_confidence_band",
            "topic_summary",
            "stance_summary",
            "risk_summary",
            "regime_hint",
            "scenario_hint",
            "confidence_band",
            "confidence_score",
            "novelty_score",
            "relevance_decay_hint",
            "contradiction_flags",
            "evidence_ids",
            "terminal_decision"
        ]
    })
}
