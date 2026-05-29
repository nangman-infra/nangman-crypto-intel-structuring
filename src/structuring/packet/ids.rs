use crate::hash::stable_short_id;
use crate::models::constants::STRUCTURED_PACKET_SCHEMA_VERSION;
use crate::models::market::MarketContextSnapshot;
use crate::models::output::MarketContextRef;

pub fn revised_packet_id(packet_family_id: &str, revision: u32) -> String {
    stable_short_id(
        "intel_pkt",
        &[packet_family_id, "revision", &revision.to_string()],
    )
}

pub fn market_context_ref(market_context: &MarketContextSnapshot) -> MarketContextRef {
    MarketContextRef {
        status: market_context.status.clone(),
        basis_timestamp_ms: market_context.basis_timestamp_ms,
        basis_kind: market_context.basis_kind.clone(),
        window_start_ms: market_context.window_start_ms,
        window_end_ms: market_context.window_end_ms,
        manifest_key: market_context.manifest_key.clone(),
        output_object_keys: market_context.output_object_keys.clone(),
        market_data_quality_summary_key: market_context.market_data_quality_summary_key.clone(),
        market_feature_delta_key: market_context.market_feature_delta_key.clone(),
        market_feature_delta_summary_key: market_context.market_feature_delta_summary_key.clone(),
        market_regime_context_key: market_context.market_regime_context_key.clone(),
        symbol_universe_snapshot_key: market_context.symbol_universe_snapshot_key.clone(),
    }
}

pub(super) fn packet_family_id(event_id: &str, policy_version: &str) -> String {
    stable_short_id(
        "intel_pkt_family",
        &[event_id, STRUCTURED_PACKET_SCHEMA_VERSION, policy_version],
    )
}

pub(super) fn initial_packet_id(event_id: &str, policy_version: &str) -> String {
    stable_short_id(
        "intel_pkt",
        &[event_id, STRUCTURED_PACKET_SCHEMA_VERSION, policy_version],
    )
}

pub(super) fn flag_packet_id(
    packet_id: &str,
    flag_schema_version: &str,
    policy_version: &str,
) -> String {
    stable_short_id(
        "intel_flag",
        &[packet_id, flag_schema_version, policy_version],
    )
}
