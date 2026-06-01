use super::*;

#[test]
fn builds_market_l1_index_key() {
    assert_eq!(
        index_pointer_key(1_000, 0),
        "l1_index/window_ms=1000/event_date=1970-01-01/hour=00/window_start_ms=0.json"
    );
}

#[test]
fn parses_window_start_from_index_key() {
    assert_eq!(
        parse_window_start_ms(
            "l1_index/window_ms=1000/event_date=2026-05-08/hour=12/window_start_ms=1778242444000.json"
        ),
        Some(1_778_242_444_000)
    );
}

#[test]
fn ignores_out_of_range_index_keys_when_selecting_latest_window() {
    let latest = latest_window_start_from_key(
        Some(1_000),
        "l1_index/window_ms=1000/event_date=1970-01-01/hour=00/window_start_ms=500.json",
        1_000,
        3_000,
    );
    assert_eq!(latest, Some(1_000));

    let latest = latest_window_start_from_key(
        Some(1_000),
        "l1_index/window_ms=1000/event_date=1970-01-01/hour=00/window_start_ms=4000.json",
        1_000,
        3_000,
    );
    assert_eq!(latest, Some(1_000));
}

#[test]
fn keeps_largest_in_range_index_window() {
    let latest = latest_window_start_from_key(
        Some(1_000),
        "l1_index/window_ms=1000/event_date=1970-01-01/hour=00/window_start_ms=3000.json",
        1_000,
        3_000,
    );
    assert_eq!(latest, Some(3_000));
}

#[test]
fn offset_window_start_saturates_at_i64_bounds() {
    assert_eq!(offset_window_start_ms(i64::MAX - 1, 5, 1_000), i64::MAX);
    assert_eq!(offset_window_start_ms(i64::MIN + 1, -5, 1_000), i64::MIN);
}

#[test]
fn builds_previous_and_current_hour_prefixes_for_cross_hour_lookback() {
    assert_eq!(
        index_prefixes(1_000, 3_599_000, 3_600_000),
        vec![
            "l1_index/window_ms=1000/event_date=1970-01-01/hour=00/".to_owned(),
            "l1_index/window_ms=1000/event_date=1970-01-01/hour=01/".to_owned(),
        ]
    );
}

#[test]
fn builds_all_hour_prefixes_for_multi_hour_latest_before_lookback() {
    assert_eq!(
        index_prefixes(1_000, 0, 14_400_000),
        vec![
            "l1_index/window_ms=1000/event_date=1970-01-01/hour=00/".to_owned(),
            "l1_index/window_ms=1000/event_date=1970-01-01/hour=01/".to_owned(),
            "l1_index/window_ms=1000/event_date=1970-01-01/hour=02/".to_owned(),
            "l1_index/window_ms=1000/event_date=1970-01-01/hour=03/".to_owned(),
            "l1_index/window_ms=1000/event_date=1970-01-01/hour=04/".to_owned(),
        ]
    );
}
