pub fn path_segment(value: &str) -> String {
    let segment = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();

    if matches!(segment.as_str(), "" | "." | "..") {
        "_".to_owned()
    } else {
        segment
    }
}
