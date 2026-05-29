use super::MarketL1Reader;
use crate::error::AppResult;
use crate::time::{floor_window, time_part};
use std::collections::BTreeSet;

const LATEST_BEFORE_MAX_INDEX_KEYS_PER_HOUR: usize = 5_000;
const HOUR_MS: i64 = 3_600_000;

impl MarketL1Reader {
    pub(super) async fn candidate_window_starts(&self, basis_window_start_ms: i64) -> Vec<i64> {
        let mut ordered = Vec::new();
        let mut seen = BTreeSet::new();
        for offset in -self.radius_windows..=self.radius_windows {
            push_unique(
                &mut ordered,
                &mut seen,
                basis_window_start_ms + offset * self.window_ms,
            );
        }
        if let Ok(Some(latest_before)) =
            self.latest_before_window_start(basis_window_start_ms).await
        {
            push_unique(&mut ordered, &mut seen, latest_before);
        }
        ordered
    }

    async fn latest_before_window_start(
        &self,
        basis_window_start_ms: i64,
    ) -> AppResult<Option<i64>> {
        let earliest = basis_window_start_ms.saturating_sub(self.latest_before_lookback_ms);
        let mut latest = None;
        for prefix in index_prefixes(self.window_ms, earliest, basis_window_start_ms) {
            for key in self
                .store
                .list_keys(&prefix, LATEST_BEFORE_MAX_INDEX_KEYS_PER_HOUR)
                .await?
            {
                let Some(window_start_ms) = parse_window_start_ms(&key) else {
                    continue;
                };
                if window_start_ms > basis_window_start_ms || window_start_ms < earliest {
                    continue;
                }
                latest =
                    Some(latest.map_or(window_start_ms, |value: i64| value.max(window_start_ms)));
            }
        }
        Ok(latest)
    }
}

fn push_unique(values: &mut Vec<i64>, seen: &mut BTreeSet<i64>, value: i64) {
    if seen.insert(value) {
        values.push(value);
    }
}

fn index_prefixes(window_ms: i64, earliest_ms: i64, latest_ms: i64) -> Vec<String> {
    let mut prefixes = Vec::new();
    let mut current = floor_window(earliest_ms.min(latest_ms), HOUR_MS);
    let latest_hour = floor_window(latest_ms.max(earliest_ms), HOUR_MS);
    while current <= latest_hour {
        let part = time_part(current);
        prefixes.push(format!(
            "l1_index/window_ms={window_ms}/event_date={}/hour={:02}/",
            part.event_date, part.hour
        ));
        let next = current.saturating_add(HOUR_MS);
        if next <= current {
            break;
        }
        current = next;
    }
    prefixes
}

fn parse_window_start_ms(key: &str) -> Option<i64> {
    key.strip_suffix(".json")?
        .rsplit_once("window_start_ms=")?
        .1
        .parse()
        .ok()
}

pub fn index_pointer_key(window_ms: i64, window_start_ms: i64) -> String {
    let part = time_part(window_start_ms);
    format!(
        "l1_index/window_ms={window_ms}/event_date={}/hour={:02}/window_start_ms={window_start_ms}.json",
        part.event_date, part.hour
    )
}

#[cfg(test)]
mod tests {
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
}
