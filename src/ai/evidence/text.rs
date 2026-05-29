pub(super) fn split_sentences(line: &str) -> Vec<&str> {
    let mut sentences = Vec::new();
    let mut start = 0;
    for (index, ch) in line.char_indices() {
        if matches!(ch, '.' | '!' | '?' | '\u{3002}') {
            let end = index + ch.len_utf8();
            if start < end {
                sentences.push(&line[start..end]);
            }
            start = end;
        }
    }
    if start < line.len() {
        sentences.push(&line[start..]);
    }
    sentences
}

pub(super) fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(super) fn normalize_for_dedup(text: &str) -> String {
    normalize_whitespace(text).to_ascii_lowercase()
}

pub(super) fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_owned();
    }
    text.chars().take(max_chars).collect::<String>()
}
