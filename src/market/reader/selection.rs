use super::MarketL1Reader;
use crate::error::AppResult;
use crate::time::{floor_window, time_part};
use std::collections::BTreeSet;

const LATEST_BEFORE_MAX_INDEX_KEYS_PER_HOUR: usize = 5_000;
const HOUR_MS: i64 = 3_600_000;

#[cfg(test)]
mod tests;

impl MarketL1Reader {
    pub(super) async fn candidate_window_starts(&self, basis_window_start_ms: i64) -> Vec<i64> {
        let mut ordered = Vec::new();
        let mut seen = BTreeSet::new();
        for offset in -self.radius_windows..=self.radius_windows {
            push_unique(
                &mut ordered,
                &mut seen,
                offset_window_start_ms(basis_window_start_ms, offset, self.window_ms),
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
                latest =
                    latest_window_start_from_key(latest, &key, earliest, basis_window_start_ms);
            }
        }
        Ok(latest)
    }
}

fn offset_window_start_ms(basis_window_start_ms: i64, offset: i64, window_ms: i64) -> i64 {
    basis_window_start_ms.saturating_add(offset.saturating_mul(window_ms))
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

fn latest_window_start_from_key(
    latest: Option<i64>,
    key: &str,
    earliest_ms: i64,
    basis_window_start_ms: i64,
) -> Option<i64> {
    let window_start_ms = parse_window_start_ms(key)?;
    if window_start_ms > basis_window_start_ms || window_start_ms < earliest_ms {
        return latest;
    }
    Some(latest.map_or(window_start_ms, |value| value.max(window_start_ms)))
}

pub fn index_pointer_key(window_ms: i64, window_start_ms: i64) -> String {
    let part = time_part(window_start_ms);
    format!(
        "l1_index/window_ms={window_ms}/event_date={}/hour={:02}/window_start_ms={window_start_ms}.json",
        part.event_date, part.hour
    )
}
