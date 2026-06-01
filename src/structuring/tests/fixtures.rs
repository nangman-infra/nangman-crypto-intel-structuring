#[path = "fixtures/event.rs"]
mod event;
#[path = "fixtures/market.rs"]
mod market;
#[path = "fixtures/policy.rs"]
mod policy;
#[path = "fixtures/response.rs"]
mod response;

pub(in crate::structuring::tests) use event::{event, numeric_snapshot_event};
pub(in crate::structuring::tests) use market::{
    market_context, pending_market_context, stale_market_context,
};
pub(in crate::structuring::tests) use policy::{policy, policy_with_escalation_budget};
pub(in crate::structuring::tests) use response::{response, response_with_evidence};
