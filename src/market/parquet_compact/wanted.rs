use std::collections::BTreeSet;

pub(super) type WantedSymbols = BTreeSet<String>;

pub(super) fn wanted_symbols(symbols: &[String]) -> WantedSymbols {
    symbols
        .iter()
        .map(|symbol| symbol.trim().to_ascii_uppercase())
        .filter(|symbol| !symbol.is_empty())
        .collect()
}
