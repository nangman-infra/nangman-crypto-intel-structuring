use super::*;

#[test]
fn parses_cli_overrides() {
    let args = Args::parse(
        [
            "intel-structuring-app",
            "--max-messages",
            "1",
            "--exit-on-idle",
            "true",
            "--enable-bedrock",
            "false",
            "--raw-s3-bucket",
            "test-raw-intel-l0",
            "--output-bucket",
            "test-intel-structuring-l1",
            "--market-l1-bucket",
            "test-market-l1",
        ]
        .into_iter()
        .map(str::to_owned),
    )
    .unwrap();

    assert_eq!(args.max_messages, Some(1));
    assert!(args.exit_on_idle);
    assert!(!args.model_policy.enable_bedrock);
    assert_eq!(
        args.processing.market_context_latest_before_lookback_ms,
        DEFAULT_MARKET_CONTEXT_LATEST_BEFORE_LOOKBACK_MS
    );
    assert_eq!(
        args.processing.market_context_stale_after_ms,
        DEFAULT_MARKET_CONTEXT_STALE_AFTER_MS
    );
}

#[test]
fn separates_aws_region_from_bedrock_region() {
    let args = Args::parse(
        [
            "intel-structuring-app",
            "--aws-region",
            "ap-northeast-2",
            "--bedrock-region",
            "us-east-1",
            "--raw-s3-bucket",
            "test-raw-intel-l0",
            "--output-bucket",
            "test-intel-structuring-l1",
            "--market-l1-bucket",
            "test-market-l1",
        ]
        .into_iter()
        .map(str::to_owned),
    )
    .unwrap();

    assert_eq!(args.output_store.region, "ap-northeast-2");
    assert_eq!(args.market_l1_store.region, "ap-northeast-2");
    assert_eq!(args.bedrock.region, "us-east-1");
}

#[test]
fn rejects_public_doc_bucket_placeholder() {
    let err = Args::parse(
        [
            "intel-structuring-app",
            "--output-bucket",
            DEFAULT_OUTPUT_BUCKET,
            "--raw-s3-bucket",
            "test-raw-intel-l0",
            "--market-l1-bucket",
            "test-market-l1",
        ]
        .into_iter()
        .map(str::to_owned),
    )
    .unwrap_err();

    assert!(err.to_string().contains("INTEL_L1_OUTPUT_S3_BUCKET"));
    assert!(err.to_string().contains("public-doc placeholder"));
}

#[test]
fn rejects_default_raw_bucket_placeholder() {
    let err = Args::parse(
        [
            "intel-structuring-app",
            "--output-bucket",
            "test-intel-structuring-l1",
            "--market-l1-bucket",
            "test-market-l1",
        ]
        .into_iter()
        .map(str::to_owned),
    )
    .unwrap_err();

    assert!(err.to_string().contains("INTEL_L1_RAW_S3_BUCKET"));
}
