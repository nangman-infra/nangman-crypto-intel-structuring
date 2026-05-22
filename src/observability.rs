use crate::error::AppResult;
use crate::models::market::MarketContextStatus;
use crate::models::output::{ModelTierUsed, TerminalDecision};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub struct ProcessingMetric {
    pub raw_event_id: String,
    pub packet_id: String,
    pub model_tier_used: ModelTierUsed,
    pub terminal_decision: TerminalDecision,
    pub market_context_status: MarketContextStatus,
    pub ack_ready: bool,
    pub fallback_count: usize,
    pub conflict_count: usize,
    pub primary_invocation_count: usize,
    pub escalation_invocation_count: usize,
    pub numeric_snapshot_count: usize,
    pub stale_market_context_count: usize,
    pub escalation_on_numeric_snapshot_count: usize,
}

pub fn emit_processing_metric(metric: &ProcessingMetric) -> AppResult<()> {
    let document = json!({
        "_aws": {
            "Timestamp": crate::time::now_ms(),
            "CloudWatchMetrics": [{
                "Namespace": "NangmanCrypto/IntelL1",
                "Dimensions": [["Service", "ModelTier"]],
                "Metrics": [
                    {"Name": "ProcessedEventCount", "Unit": "Count"},
                    {"Name": "AckReadyCount", "Unit": "Count"},
                    {"Name": "FallbackCount", "Unit": "Count"},
                    {"Name": "ConflictCount", "Unit": "Count"},
                    {"Name": "PrimaryInvocationCount", "Unit": "Count"},
                    {"Name": "EscalationInvocationCount", "Unit": "Count"},
                    {"Name": "NumericSnapshotCount", "Unit": "Count"},
                    {"Name": "StaleMarketContextCount", "Unit": "Count"},
                    {"Name": "EscalationOnNumericSnapshotCount", "Unit": "Count"}
                ]
            }]
        },
        "Service": "intel-structuring-app",
        "ModelTier": format!("{:?}", metric.model_tier_used),
        "TerminalDecision": format!("{:?}", metric.terminal_decision),
        "MarketContextStatus": format!("{:?}", metric.market_context_status),
        "ProcessedEventCount": 1,
        "AckReadyCount": usize::from(metric.ack_ready),
        "FallbackCount": metric.fallback_count,
        "ConflictCount": metric.conflict_count,
        "PrimaryInvocationCount": metric.primary_invocation_count,
        "EscalationInvocationCount": metric.escalation_invocation_count,
        "NumericSnapshotCount": metric.numeric_snapshot_count,
        "StaleMarketContextCount": metric.stale_market_context_count,
        "EscalationOnNumericSnapshotCount": metric.escalation_on_numeric_snapshot_count,
        "raw_event_id": metric.raw_event_id,
        "packet_id": metric.packet_id
    });
    println!("{}", serde_json::to_string(&document)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_primary_and_escalation_metrics() {
        let metric = ProcessingMetric {
            raw_event_id: "raw-1".to_owned(),
            packet_id: "packet-1".to_owned(),
            model_tier_used: ModelTierUsed::Escalation,
            terminal_decision: TerminalDecision::HighConfidenceStructured,
            market_context_status: MarketContextStatus::Available,
            ack_ready: true,
            fallback_count: 0,
            conflict_count: 0,
            primary_invocation_count: 1,
            escalation_invocation_count: 1,
            numeric_snapshot_count: 0,
            stale_market_context_count: 0,
            escalation_on_numeric_snapshot_count: 0,
        };

        emit_processing_metric(&metric).unwrap();
    }
}
