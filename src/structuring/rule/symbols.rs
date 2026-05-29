use std::collections::BTreeSet;

pub(super) fn normalize_symbols(symbols: &[String]) -> Vec<String> {
    symbols
        .iter()
        .map(|symbol| symbol.trim().to_ascii_uppercase())
        .filter(|symbol| {
            !symbol.is_empty()
                && symbol.len() <= 12
                && symbol.chars().all(|ch| ch.is_ascii_alphanumeric())
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
