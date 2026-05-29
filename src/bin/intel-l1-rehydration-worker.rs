#[path = "intel_l1_rehydration_worker/cli.rs"]
mod cli;
#[path = "intel_l1_rehydration_worker/prefixes.rs"]
mod prefixes;

use self::cli::parse_cli_args;
use self::prefixes::input_prefixes;
use intel_structuring_app::config::Args;
use intel_structuring_app::error::AppResult;
use intel_structuring_app::market::reader::MarketL1Reader;
use intel_structuring_app::nats::publisher::StructuredPublisher;
use intel_structuring_app::storage::object_store::ObjectStore;
use intel_structuring_app::time::now_ms;
use intel_structuring_app::workflow::rehydration::{
    MarketContextRehydrationOptions, MarketContextRehydrator,
};

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
    let rehydration_options = MarketContextRehydrationOptions {
        include_terminal_missing_market_context: cli.include_terminal_missing_market_context,
    };
    let rehydrator = MarketContextRehydrator::new(
        output_store,
        market_reader,
        publisher,
        args.processing,
        rehydration_options,
    );
    let input_prefixes = input_prefixes(&cli, now_ms());
    if !input_prefixes.is_empty() {
        eprintln!(
            "market context rehydration scanning {} structured prefixes with max_packets_per_prefix={}",
            input_prefixes.len(),
            cli.max_packets
        );
    }
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
            "include_terminal_missing_market_context": cli.include_terminal_missing_market_context,
            "published_revisions": published,
        }))?
    );
    Ok(())
}
