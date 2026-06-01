#[path = "tests/fixtures.rs"]
mod fixtures;

#[path = "tests/router.rs"]
mod router;

#[path = "tests/packet.rs"]
mod packet;

use crate::ai::contract::ModelStructuringResponse;
use crate::models::output::{ConfidenceBand, EventType, ModelTierUsed, TerminalDecision};
use fixtures::*;
