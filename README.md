# intel-structuring-app

`intel-structuring-app` is the INTEL-L1 stateless worker.

Repository: `git@github.com:nangman-infra/nangman-crypto-intel-structuring.git`

It consumes `RAW_INTEL` pointer messages from NATS JetStream, recovers raw evidence from AWS S3 Raw Intel L0, reads Market-L1 only through the `l1_index -> manifest -> report -> output_object_keys` contract path, structures the event, writes INTEL-L1 objects to S3, publishes structured pointers to NATS, and only then acknowledges the original RAW_INTEL message.

## Runtime contract

```text
RAW_INTEL durable pull consumer
  -> Raw Intel L0 S3 recovery and sha verification
  -> Market-L1 admission
  -> L0 source/content quality admission
  -> rule/NLP/NLI
  -> deterministic evidence_pack with stable evidence IDs
  -> Llama 4 Scout primary extraction through Bedrock Converse
  -> primary repair for evidence/schema gate misses
  -> Llama 4 Maverick escalation for hard/high-risk/high-impact cases
  -> immutable story_member write
  -> refreshed story_cluster merge
  -> S3 JSONL outputs, manifest, and prepared index
  -> NATS structured pointer publish ack
  -> success index write
  -> RAW_INTEL double ack
```

The app is stateless. Local disk is used only for temporary parquet scanning and can be lost at any time.
Story state is reconstructed from S3 `story-members/` objects, so ECS Spot restart does not depend on container memory or local files.

Market-L1 context lookup checks exact/radius windows first, then scans every
hour in the configured latest-before lookback range. It must not jump from the
current hour to only the oldest lookback hour, because that can attach stale L1
manifests and empty universe snapshots even when fresher success pointers exist
in the middle of the lookback window.

## Semantic routing

L0 quality metadata from `intel-crawl-app` is part of the L1 decision input:

```text
content_kind
content_quality
content_quality_score
source_quality
source_relevance_scope
```

Rule-only output is blocked when the raw item is community reaction, market snapshot, title-only, or metadata fallback evidence. Direct, high-quality community reaction can finish on the primary model. Weak global symbol-scan evidence can still call the primary model first, but any structured claim is escalated only when the escalation admission contract allows it.

Single numeric derivatives snapshots are never allowed to use escalation. `stale_but_usable` market context is preserved in the packet as audit context, but it is not strong enough to open expensive model escalation for numeric snapshots. Non-critical safety escalation also respects `INTEL_L1_ESCALATION_BUDGET_RATIO` as a hard budget.

`intel-l1-rehydration-worker` rechecks `pending` and `stale_but_usable` packets against newer Market-L1 windows. When a fresher non-stale market context exists, it writes a new structured packet revision and republishes the structured pointer so downstream candidate scoring can rerun without mutating the original packet. Packets already closed as `terminal_missing_market_context` stay closed by default; operators must pass `--include-terminal-missing-market-context` for bounded backfill runs that intentionally reopen them after Market-L1 coverage has caught up. Use repeated `--structured-prefix structured-intel-packet/.../hour=HH/` arguments for narrow, checkpointed rehydration of known stale partitions instead of forcing a broad recent-hour scan.

Llama 4 Scout and Maverick are intentionally used as low-cost generation models. Bedrock Structured Outputs are not assumed for these models, so the app treats the local Rust contract as the source of truth: prompt schema, JSON extraction, serde validation, evidence-ID hydration, deterministic repair, admission gates, and fallback/quarantine logic all run outside the model.

## Local quality and ECS runtime

```bash
git clone git@github.com:nangman-infra/nangman-crypto-intel-structuring.git
cd nangman-crypto-intel-structuring
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
docker buildx build --platform linux/arm64 -t intel-structuring-app:local --load .
```

Runtime deployment is ECS/Fargate Spot. Operational values come from the ECS
task definition, task role, and private runtime context. Local development
harnesses are not the deployment source of truth. Set
`INTEL_L1_ENABLE_BEDROCK=true` only after the ECS task role or local AWS
credentials can invoke the configured Bedrock inference profiles.

## Quality gate

```bash
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
docker buildx build --platform linux/arm64 -t intel-structuring-app:local --load .
```

The GitHub Actions workflow runs formatting, tests, clippy, coverage generation, SonarQube scan, and SonarQube Quality Gate on `main`.

## Machine-readable contracts

The stable INTEL-L1 boundary is checked into the repository:

```text
schemas/*.schema.json
asyncapi/nats.asyncapi.json
```

JSON Schema Draft 2020-12 files define AWS S3 payload contracts for raw pointers, structured packets, context flags, story clusters, health events, manifests, index pointers, packet revision indexes, and structured object pointers.

The AsyncAPI 3.0 contract defines the NATS subjects this app consumes and publishes. NATS is only a pointer/event bus; AWS S3 remains the canonical durable store. Contract tests run with `cargo test --all-targets`, so schema version drift, missing subjects, or accidental trading-decision semantics fail CI.

The raw pointer schema still accepts the legacy
`storage_ref.kind = rustfs_jsonl_record` value for compatibility with existing
RAW_INTEL messages. Runtime Raw L0 reads use the AWS SDK S3/IAM path.

## Runtime prerequisites

```text
NATS:
- RAW_INTEL stream exists
- app can create or access STRUCTURED_INTEL

Raw Intel L0 AWS S3:
- INTEL_L1_RAW_S3_BUCKET
- INTEL_L1_RAW_S3_REGION
- read access to the Raw Intel L0 bucket

AWS:
- read access to Market-L1 bucket
- write/read access to INTEL-L1 output bucket
- Bedrock invoke permission when INTEL_L1_ENABLE_BEDROCK=true
```

The worker handles both Ctrl-C and Linux `SIGTERM`. If ECS Spot stops the task, the current message is only acknowledged after the complete success path; otherwise JetStream redelivers it.

The app emits CloudWatch Embedded Metric Format JSON to stdout. ECS should route stdout to CloudWatch Logs; alarms belong to the ECS/service infrastructure layer.
