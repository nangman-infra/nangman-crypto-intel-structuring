use crate::cli::CliArgs;
use intel_structuring_app::models::constants::STRUCTURED_PACKET_SCHEMA_VERSION;
use intel_structuring_app::time::time_part;

const HOUR_MS: i64 = 3_600_000;

pub(crate) fn input_prefixes(cli: &CliArgs, timestamp_ms: i64) -> Vec<String> {
    let mut prefixes = Vec::new();
    for prefix in &cli.structured_prefixes {
        if !prefixes.contains(prefix) {
            prefixes.push(prefix.clone());
        }
    }
    if let Some(hours) = cli.recent_hours {
        for prefix in recent_structured_packet_prefixes(timestamp_ms, hours) {
            if !prefixes.contains(&prefix) {
                prefixes.push(prefix);
            }
        }
    }
    prefixes
}

fn recent_structured_packet_prefixes(timestamp_ms: i64, recent_hours: usize) -> Vec<String> {
    let mut prefixes = Vec::new();
    for offset in 0..recent_hours {
        let timestamp = timestamp_ms.saturating_sub(offset as i64 * HOUR_MS);
        let part = time_part(timestamp);
        let prefix = format!(
            "structured-intel-packet/schema={STRUCTURED_PACKET_SCHEMA_VERSION}/dt={}/hour={:02}/",
            part.event_date, part.hour
        );
        if !prefixes.contains(&prefix) {
            prefixes.push(prefix);
        }
    }
    prefixes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::parse_cli_args;

    #[test]
    fn dedupes_explicit_and_recent_prefixes() {
        let timestamp_ms = 1779647400000;
        let mut cli = parse_cli_args(
            [
                "--recent-hours".to_owned(),
                "2".to_owned(),
                "--structured-prefix".to_owned(),
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=18/"
                    .to_owned(),
            ]
            .into_iter(),
        )
        .unwrap();
        cli.structured_prefixes.push(
            "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=17/"
                .to_owned(),
        );

        assert_eq!(
            input_prefixes(&cli, timestamp_ms),
            vec![
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=18/"
                    .to_owned(),
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=17/"
                    .to_owned()
            ]
        );
    }

    #[test]
    fn recent_prefixes_walk_back_across_day_boundary() {
        let prefixes = recent_structured_packet_prefixes(3_600_000, 3);

        assert_eq!(
            prefixes,
            vec![
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=1970-01-01/hour=01/"
                    .to_owned(),
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=1970-01-01/hour=00/"
                    .to_owned(),
                "structured-intel-packet/schema=structured_intel_packet_v1/dt=1969-12-31/hour=23/"
                    .to_owned(),
            ]
        );
    }
}
