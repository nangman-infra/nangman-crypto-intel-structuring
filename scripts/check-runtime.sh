#!/usr/bin/env bash
set -euo pipefail

APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="${1:-$APP_DIR/.env}"

if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "$ENV_FILE"
  set +a
fi

: "${NATS_URL:?NATS_URL is required}"
: "${INTEL_L1_OUTPUT_S3_BUCKET:?INTEL_L1_OUTPUT_S3_BUCKET is required}"
: "${INTEL_L1_MARKET_L1_BUCKET:?INTEL_L1_MARKET_L1_BUCKET is required}"
: "${AWS_REGION:=ap-northeast-2}"
: "${BEDROCK_REGION:=us-east-1}"
: "${INTEL_L1_PRIMARY_MODEL_ID:=us.meta.llama4-scout-17b-instruct-v1:0}"
: "${INTEL_L1_ESCALATION_MODEL_ID:=us.meta.llama4-maverick-17b-instruct-v1:0}"

echo "[1/4] NATS RAW_INTEL stream"
docker run --rm natsio/nats-box:0.17.0 \
  nats --server "$NATS_URL" stream info "${INTEL_L1_RAW_NATS_STREAM:-RAW_INTEL}" >/dev/null

echo "[2/4] Market-L1 bucket"
aws s3api head-bucket \
  --bucket "$INTEL_L1_MARKET_L1_BUCKET" \
  --region "${INTEL_L1_MARKET_S3_REGION:-$AWS_REGION}" >/dev/null

echo "[3/4] INTEL-L1 output bucket"
aws s3api head-bucket \
  --bucket "$INTEL_L1_OUTPUT_S3_BUCKET" \
  --region "${INTEL_L1_OUTPUT_S3_REGION:-$AWS_REGION}" >/dev/null

echo "[4/4] Bedrock inference profiles"
profile_count="$(aws bedrock list-inference-profiles \
  --region "$BEDROCK_REGION" \
  --query "length(inferenceProfileSummaries[?inferenceProfileId==\`$INTEL_L1_PRIMARY_MODEL_ID\` || inferenceProfileId==\`$INTEL_L1_ESCALATION_MODEL_ID\`])" \
  --output text)"
if [[ "$profile_count" != "2" ]]; then
  aws bedrock list-inference-profiles \
    --region "$BEDROCK_REGION" \
    --query 'inferenceProfileSummaries[?contains(inferenceProfileId, `llama4`)].[inferenceProfileId,status]' \
    --output table
  echo "required Bedrock inference profiles were not both found" >&2
  exit 1
fi

echo "runtime checks passed"
