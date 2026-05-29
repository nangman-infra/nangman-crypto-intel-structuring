mod parse;
mod prefix;

#[cfg(test)]
mod tests;

pub(crate) use parse::parse_cli_args;

const DEFAULT_MAX_PACKETS: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CliArgs {
    pub(crate) max_packets: usize,
    pub(crate) recent_hours: Option<usize>,
    pub(crate) structured_prefixes: Vec<String>,
    pub(crate) include_terminal_missing_market_context: bool,
}
