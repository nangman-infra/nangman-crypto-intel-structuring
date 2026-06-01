use super::super::*;
use super::support::ScriptedProvider;
use crate::structuring::router::ModelRouter;

#[path = "primary_acceptance/broad_scan.rs"]
mod broad_scan;
#[path = "primary_acceptance/community.rs"]
mod community;
#[path = "primary_acceptance/low_value.rs"]
mod low_value;
