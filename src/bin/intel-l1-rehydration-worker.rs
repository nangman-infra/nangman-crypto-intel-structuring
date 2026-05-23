use intel_structuring_app::config::Args;
use intel_structuring_app::error::{AppError, AppResult};
use intel_structuring_app::market::reader::MarketL1Reader;
use intel_structuring_app::models::constants::STRUCTURED_PACKET_SCHEMA_VERSION;
use intel_structuring_app::nats::publisher::StructuredPublisher;
use intel_structuring_app::storage::object_store::ObjectStore;
use intel_structuring_app::time::{now_ms, time_part};
use intel_structuring_app::workflow::rehydration::MarketContextRehydrator;

const DEFAULT_MAX_PACKETS: usize = 512;
const HOUR_MS: i64 = 3_600_000;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliArgs {
    max_packets: usize,
    recent_hours: Option<usize>,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse(std::iter::once("intel-l1-rehydration-worker".to_owned()))?;
    let cli = parse_cli_args(std::env::args().skip(1))?;
    let output_store = ObjectStore::connect(args.output_store.clone()).await?;
    let market_store = ObjectStore::connect(args.market_l1_store.clone()).await?;
    let market_reader = MarketL1Reader::new(
        market_store,
        args.market_l1_window_ms,
        args.processing.market_context_window_radius,
        args.processing.market_context_latest_before_lookback_ms,
        args.processing.market_context_stale_after_ms,
    );
    let publisher = StructuredPublisher::connect(&args.nats).await?;
    let rehydrator =
        MarketContextRehydrator::new(output_store, market_reader, publisher, args.processing);
    let input_prefixes = cli
        .recent_hours
        .map(|hours| recent_structured_packet_prefixes(now_ms(), hours))
        .unwrap_or_default();
    let published = if input_prefixes.is_empty() {
        rehydrator.run_once(cli.max_packets).await?
    } else {
        rehydrator
            .run_prefixes_once(&input_prefixes, cli.max_packets)
            .await?
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "mode": "market_context_rehydration",
            "max_packets_per_prefix": cli.max_packets,
            "recent_hours": cli.recent_hours,
            "input_prefixes": input_prefixes,
            "published_revisions": published,
        }))?
    );
    Ok(())
}

fn parse_cli_args(values: impl Iterator<Item = String>) -> AppResult<CliArgs> {
    let mut max_packets = DEFAULT_MAX_PACKETS;
    let mut recent_hours = None;
    let mut values = values.peekable();
    while let Some(arg) = values.next() {
        match arg.as_str() {
            "--max-packets" => {
                let Some(value) = values.next() else {
                    return Err(AppError::config("--max-packets requires a value"));
                };
                max_packets = value
                    .parse::<usize>()
                    .map_err(|error| AppError::config(format!("invalid --max-packets: {error}")))?;
            }
            "--recent-hours" => {
                let Some(value) = values.next() else {
                    return Err(AppError::config("--recent-hours requires a value"));
                };
                recent_hours = Some(value.parse::<usize>().map_err(|error| {
                    AppError::config(format!("invalid --recent-hours: {error}"))
                })?);
            }
            "--help" | "-h" => {
                return Err(AppError::config(
                    "intel-l1-rehydration-worker [--max-packets <positive integer>] [--recent-hours <positive integer>]",
                ));
            }
            other => {
                return Err(AppError::config(format!(
                    "unknown rehydration argument: {other}"
                )));
            }
        }
    }
    if max_packets == 0 {
        return Err(AppError::config("--max-packets must be positive"));
    }
    if recent_hours.is_some_and(|hours| hours == 0) {
        return Err(AppError::config("--recent-hours must be positive"));
    }
    Ok(CliArgs {
        max_packets,
        recent_hours,
    })
}

fn recent_structured_packet_prefixes(timestamp_ms: i64, recent_hours: usize) -> Vec<String> {
    let mut prefixes = Vec::new();
    for offset in 0..recent_hours {
        let timestamp = timestamp_ms.saturating_sub(offset as i64 * HOUR_MS);
        let part = time_part(timestamp);
        let prefix = format!(
            "structured-intel-packet/schema={STRUCTURED_PACKET_SCHEMA_VERSION}/dt={}/hour={:02}/",
            part.event_date, part.hour
        );
        if !prefixes.contains(&prefix) {
            prefixes.push(prefix);
        }
    }
    prefixes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_recent_hours_with_max_packets() {
        let args = parse_cli_args(
            [
                "--max-packets".to_owned(),
                "20".to_owned(),
                "--recent-hours".to_owned(),
                "3".to_owned(),
            ]
            .into_iter(),
        )
        .unwrap();

        assert_eq!(
            args,
            CliArgs {
                max_packets: 20,
                recent_hours: Some(3)
            }
        );
    }

    #[test]
    fn rejects_zero_recent_hours() {
        let error =
            parse_cli_args(["--recent-hours".to_owned(), "0".to_owned()].into_iter()).unwrap_err();

        assert!(error.to_string().contains("--recent-hours"));
    }

    #[test]
    fn recent_prefixes_walk_back_across_day_boundary() {
        let prefixes = recent_structured_packet_prefixes(3_600_000, 3);

        assert_eq!(
            prefixes,
            vec![
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=1970-01-01/hour=01/"
                    .to_owned(),
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=1970-01-01/hour=00/"
                    .to_owned(),
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=1969-12-31/hour=23/"
                    .to_owned(),
            ]
        );
    }
}
