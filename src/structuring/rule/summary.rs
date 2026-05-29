use crate::models::output::{EventType, RelevanceDecayHint};
use crate::models::raw::RawIntelEvent;

pub(super) fn topic_summary(event: &RawIntelEvent, event_type: &EventType) -> String {
    format!("{}: {}", event_type_label(event_type), event.title)
}

pub(super) fn stance_summary(event_type: &EventType) -> String {
    match event_type {
        EventType::Listing | EventType::Partnership => {
            "원문 기준 긍정적 해석 가능성은 있으나 매매 판단은 보류".to_owned()
        }
        EventType::Delisting | EventType::Incident | EventType::Regulatory => {
            "원문 기준 headline/event risk가 존재하며 관찰 대상으로 분류".to_owned()
        }
        _ => "원문 기반 일반 시장 정보로 분류".to_owned(),
    }
}

pub(super) fn risk_summary(event_type: &EventType) -> String {
    match event_type {
        EventType::Delisting => "상장폐지 또는 거래쌍 제거 관련 유동성/심리 리스크".to_owned(),
        EventType::Incident => "보안/운영 사고 관련 신뢰도 및 변동성 리스크".to_owned(),
        EventType::Regulatory => "규제/법적 불확실성 리스크".to_owned(),
        EventType::DepositWithdrawal => {
            "입출금 운영 이벤트로 인한 단기 거래 불편 리스크".to_owned()
        }
        _ => "직접적인 고위험 신호는 rule layer에서 확인되지 않음".to_owned(),
    }
}

pub(super) fn regime_hint(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::MacroEvent | EventType::Regulatory => "risk_off",
        EventType::SocialHype => "social_mania",
        EventType::Other => "uncertain",
        _ => "event_driven",
    }
}

pub(super) fn scenario_hint(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::Incident | EventType::Regulatory | EventType::Delisting => {
            "structural_break_possible"
        }
        EventType::Other => "noise_only",
        _ => "watch_only",
    }
}

pub(super) fn relevance_decay_hint(event_type: &EventType) -> RelevanceDecayHint {
    match event_type {
        EventType::Incident | EventType::Regulatory => RelevanceDecayHint::MultiDay,
        EventType::Listing | EventType::Delisting | EventType::DepositWithdrawal => {
            RelevanceDecayHint::Hours
        }
        EventType::MacroEvent => RelevanceDecayHint::Day,
        _ => RelevanceDecayHint::Hours,
    }
}

pub(super) fn novelty_score(event_type: &EventType, event: &RawIntelEvent) -> f64 {
    if event.source_category.contains("official") || event.source_category.contains("project") {
        0.72
    } else if matches!(event_type, EventType::Other) {
        0.35
    } else {
        0.58
    }
}

fn event_type_label(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::Listing => "listing",
        EventType::Delisting => "delisting",
        EventType::DepositWithdrawal => "deposit_withdrawal",
        EventType::Incident => "incident",
        EventType::Partnership => "partnership",
        EventType::TokenUnlock => "token_unlock",
        EventType::Governance => "governance",
        EventType::FundingShift => "funding_shift",
        EventType::MacroEvent => "macro_event",
        EventType::Regulatory => "regulatory",
        EventType::SocialBacklash => "social_backlash",
        EventType::SocialHype => "social_hype",
        EventType::Other => "other",
    }
}
