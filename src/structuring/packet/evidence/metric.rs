use crate::models::output::MetricEvidence;
use crate::models::raw::RawIntelEvent;

pub(in crate::structuring::packet) fn metric_evidence(
    event: &RawIntelEvent,
    normalized_symbols: &[String],
) -> Vec<MetricEvidence> {
    if event.source_quality_or_unknown() != "market_snapshot"
        && event.content_quality_or_unknown() != "numeric_observation"
    {
        return Vec::new();
    }
    let value = metric_value(event);
    let symbols = if normalized_symbols.is_empty() {
        vec![None]
    } else {
        normalized_symbols
            .iter()
            .map(|symbol| Some(symbol.clone()))
            .collect()
    };
    symbols
        .into_iter()
        .map(|symbol| MetricEvidence {
            metric_name: event
                .event_category_hint
                .clone()
                .unwrap_or_else(|| event.content_kind_or_unknown().to_owned()),
            symbol,
            venue: Some(event.source_id.clone()),
            value,
            previous_value: None,
            delta_pct: None,
            window_ms: None,
            observed_at_ms: event.observed_at_ms,
            source_event_id: event.event_id.clone(),
        })
        .collect()
}

fn metric_value(event: &RawIntelEvent) -> Option<f64> {
    let body: serde_json::Value = serde_json::from_str(&event.body).ok()?;
    metric_value_keys(event)
        .iter()
        .filter_map(|key| body.get(key))
        .find_map(json_number)
}

fn metric_value_keys(event: &RawIntelEvent) -> &'static [&'static str] {
    let metric_name = event
        .event_category_hint
        .as_deref()
        .unwrap_or_else(|| event.content_kind_or_unknown())
        .to_ascii_lowercase();
    let source_id = event.source_id.to_ascii_lowercase();

    if metric_name.contains("open_interest")
        || metric_name.contains("open interest")
        || source_id.contains("open_interest")
        || source_id.contains("open-interest")
    {
        &["open_interest", "openInterest", "value"]
    } else if metric_name.contains("funding") || source_id.contains("funding") {
        &[
            "last_funding_rate",
            "funding_rate",
            "lastFundingRate",
            "fundingRate",
            "value",
        ]
    } else if metric_name.contains("liquidation") || source_id.contains("liquidation") {
        &[
            "liquidation",
            "liquidations",
            "liquidation_value",
            "notional",
            "value",
        ]
    } else {
        &["metric_value", "value"]
    }
}

fn json_number(value: &serde_json::Value) -> Option<f64> {
    let parsed = match value {
        serde_json::Value::Number(number) => number.as_f64(),
        serde_json::Value::String(text) => text.trim().parse::<f64>().ok(),
        _ => None,
    }?;
    parsed.is_finite().then_some(parsed)
}
