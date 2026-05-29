use intel_structuring_app::error::{AppError, AppResult};

use super::prefix::normalize_structured_prefix;
use super::{CliArgs, DEFAULT_MAX_PACKETS};

const HELP_TEXT: &str = "intel-l1-rehydration-worker [--max-packets <positive integer>] [--recent-hours <positive integer>] [--structured-prefix <s3 key prefix>] [--include-terminal-missing-market-context]";

pub(crate) fn parse_cli_args(values: impl Iterator<Item = String>) -> AppResult<CliArgs> {
    let mut parsed = ParsedCliArgs::default();
    let mut values = values.peekable();

    while let Some(arg) = values.next() {
        match arg.as_str() {
            "--max-packets" => {
                parsed.max_packets =
                    parse_usize_arg("--max-packets", next_value(&mut values, "--max-packets")?)?;
            }
            "--recent-hours" => {
                parsed.recent_hours = Some(parse_usize_arg(
                    "--recent-hours",
                    next_value(&mut values, "--recent-hours")?,
                )?);
            }
            "--structured-prefix" => {
                let prefix =
                    normalize_structured_prefix(&next_value(&mut values, "--structured-prefix")?)?;
                parsed.structured_prefixes.push(prefix);
            }
            "--include-terminal-missing-market-context" => {
                parsed.include_terminal_missing_market_context = true;
            }
            "--help" | "-h" => {
                return Err(AppError::config(HELP_TEXT));
            }
            other => {
                return Err(AppError::config(format!(
                    "unknown rehydration argument: {other}"
                )));
            }
        }
    }

    parsed.validate()
}

#[derive(Debug)]
struct ParsedCliArgs {
    max_packets: usize,
    recent_hours: Option<usize>,
    structured_prefixes: Vec<String>,
    include_terminal_missing_market_context: bool,
}

impl ParsedCliArgs {
    fn validate(self) -> AppResult<CliArgs> {
        if self.max_packets == 0 {
            return Err(AppError::config("--max-packets must be positive"));
        }
        if self.recent_hours.is_some_and(|hours| hours == 0) {
            return Err(AppError::config("--recent-hours must be positive"));
        }

        Ok(CliArgs {
            max_packets: self.max_packets,
            recent_hours: self.recent_hours,
            structured_prefixes: self.structured_prefixes,
            include_terminal_missing_market_context: self.include_terminal_missing_market_context,
        })
    }
}

impl Default for ParsedCliArgs {
    fn default() -> Self {
        Self {
            max_packets: DEFAULT_MAX_PACKETS,
            recent_hours: None,
            structured_prefixes: Vec::new(),
            include_terminal_missing_market_context: false,
        }
    }
}

fn next_value(
    values: &mut impl Iterator<Item = String>,
    argument_name: &'static str,
) -> AppResult<String> {
    values
        .next()
        .ok_or_else(|| AppError::config(format!("{argument_name} requires a value")))
}

fn parse_usize_arg(argument_name: &'static str, value: String) -> AppResult<usize> {
    value
        .parse::<usize>()
        .map_err(|error| AppError::config(format!("invalid {argument_name}: {error}")))
}
