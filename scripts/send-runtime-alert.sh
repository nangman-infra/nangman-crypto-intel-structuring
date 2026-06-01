#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
APP_DIR="$(cd -- "$SCRIPT_DIR/.." && pwd -P)"
ENV_FILE="${INTEL_STRUCTURING_ENV_FILE:-$APP_DIR/.env}"

APP_NAME="intel-structuring-app"
AWS_REGION="${AWS_REGION:-ap-northeast-2}"
ALERT_ENV="${NANGMAN_ALERT_ENV:-dev}"
INCLUDE_SUCCESS="${INTEL_STRUCTURING_ALERT_INCLUDE_SUCCESS:-false}"
PIPELINE_ALERT_S3_BUCKET="${NANGMAN_PIPELINE_ALERT_S3_BUCKET:-${INTEL_STRUCTURING_PIPELINE_ALERT_S3_BUCKET:-}}"
PIPELINE_ALERT_S3_PREFIX="${NANGMAN_PIPELINE_ALERT_S3_PREFIX:-pipeline-alert-event/schema=pipeline_alert_event_v1}"

# shellcheck source=scripts/lib/runtime-alert-core.sh
source "$SCRIPT_DIR/lib/runtime-alert-core.sh"

main() {
  if is_true "${INTEL_STRUCTURING_ALERT_SELF_TEST:-false}"; then
    self_test
    return
  fi

  require_command aws
  require_command jq
  require_command tail
  require_command sed

  local output_file
  output_file="$(mktemp)"
  set +e
  "$SCRIPT_DIR/check-runtime.sh" "$ENV_FILE" > "$output_file" 2>&1
  local status=$?
  set -e

  if [[ "$status" -ne 0 ]]; then
    local alert_status=0
    send_pipeline_alert P1 "runtime check failed" "$(failure_message "$status" "$output_file")" || alert_status=$?
    rm -f "$output_file"
    if [[ "$alert_status" -ne 0 ]]; then
      return "$alert_status"
    fi
    return "$status"
  fi

  if is_true "$INCLUDE_SUCCESS"; then
    local alert_status=0
    send_pipeline_alert P3 "runtime check summary" "$(success_message "$output_file")" || alert_status=$?
    rm -f "$output_file"
    return "$alert_status"
  fi
  rm -f "$output_file"
}

main "$@"
