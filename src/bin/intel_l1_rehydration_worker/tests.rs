use super::*;

#[test]
fn parses_recent_hours_with_max_packets() {
    let args = parse_cli_args(
        [
            "--max-packets".to_owned(),
            "20".to_owned(),
            "--recent-hours".to_owned(),
            "3".to_owned(),
        ]
        .into_iter(),
    )
    .unwrap();

    assert_eq!(
        args,
        CliArgs {
            max_packets: 20,
            recent_hours: Some(3),
            structured_prefixes: Vec::new(),
            include_terminal_missing_market_context: false
        }
    );
}

#[test]
fn parses_terminal_missing_rehydration_opt_in() {
    let args = parse_cli_args(
        [
            "--max-packets".to_owned(),
            "20".to_owned(),
            "--include-terminal-missing-market-context".to_owned(),
        ]
        .into_iter(),
    )
    .unwrap();

    assert_eq!(
        args,
        CliArgs {
            max_packets: 20,
            recent_hours: None,
            structured_prefixes: Vec::new(),
            include_terminal_missing_market_context: true
        }
    );
}

#[test]
fn parses_explicit_structured_prefixes() {
    let args = parse_cli_args(
        [
            "--structured-prefix".to_owned(),
            "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=10"
                .to_owned(),
            "--structured-prefix".to_owned(),
            "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=11/"
                .to_owned(),
        ]
        .into_iter(),
    )
    .unwrap();

    assert_eq!(
        args.structured_prefixes,
        vec![
            "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=10/"
                .to_owned(),
            "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=11/"
                .to_owned()
        ]
    );
}

#[test]
fn rejects_non_structured_prefix() {
    let error = parse_cli_args(
        [
            "--structured-prefix".to_owned(),
            "candidate-evidence-bundle/priority=p2/".to_owned(),
        ]
        .into_iter(),
    )
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("--structured-prefix must start with structured-intel-packet/")
    );
}

#[test]
fn rejects_s3_uri_prefix_case_insensitively() {
    let error = parse_cli_args(
        [
            "--structured-prefix".to_owned(),
            "S3://bucket/structured-intel-packet/".to_owned(),
        ]
        .into_iter(),
    )
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("--structured-prefix must be an object key prefix")
    );
}

#[test]
fn rejects_structured_prefix_with_unsafe_key_shape() {
    for prefix in [
        "structured-intel-packet/schema=structured_intel_packet_v1/../",
        "structured-intel-packet/schema=structured_intel_packet_v1//dt=2026-05-24/",
        "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24?bad=true",
        "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24#bad",
        "structured-intel-packet/schema=structured_intel_packet_v1/dt=2026-05-24/hour=10\\bad",
    ] {
        let error =
            parse_cli_args(["--structured-prefix".to_owned(), prefix.to_owned()].into_iter())
                .unwrap_err()
                .to_string();

        assert!(
            error.contains("--structured-prefix"),
            "expected structured-prefix validation error for {prefix:?}, got {error}"
        );
    }
}

#[test]
fn rejects_zero_recent_hours() {
    let error =
        parse_cli_args(["--recent-hours".to_owned(), "0".to_owned()].into_iter()).unwrap_err();

    assert!(error.to_string().contains("--recent-hours"));
}
