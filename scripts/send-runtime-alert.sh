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

log() {
  printf '%s\n' "$*"
}

die() {
  printf 'intel structuring runtime alert failed: %s\n' "$*" >&2
  exit 1
}

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    die "missing required command: $1"
  fi
}

is_true() {
  case "$1" in
    1 | true | TRUE | yes | YES) return 0 ;;
    *) return 1 ;;
  esac
}

send_pipeline_alert() {
  local priority="$1"
  local title="$2"
  local text="$3"
  if [[ -z "$PIPELINE_ALERT_S3_BUCKET" ]]; then
    die "NANGMAN_PIPELINE_ALERT_S3_BUCKET or INTEL_STRUCTURING_PIPELINE_ALERT_S3_BUCKET is required"
  fi
  local now_ms dt hour event_id key payload_file
  now_ms="$(date -u +%s000)"
  dt="$(date -u +%Y-%m-%d)"
  hour="$(date -u +%H)"
  event_id="pipeline_alert_intel_structuring_${now_ms}_$$"
  key="${PIPELINE_ALERT_S3_PREFIX%/}/dt=${dt}/hour=${hour}/app=${APP_NAME}/priority=${priority}/${event_id}.json"
  payload_file="$(mktemp)"
  local payload
  payload="$(jq -nc \
    --arg event_id "$event_id" \
    --arg dedupe_key "${APP_NAME}:${priority}:${title}" \
    --arg app "$APP_NAME" \
    --arg env "$ALERT_ENV" \
    --arg priority "$priority" \
    --arg title "$title" \
    --arg rendered_text "$text" \
    --argjson created_at_ms "$now_ms" \
    '{schema_version:"pipeline_alert_event_v1",event_id:$event_id,dedupe_key:$dedupe_key,app:$app,environment:$env,priority:$priority,title:$title,conclusion:"Runtime wrapper emitted a pipeline alert.",rendered_text:$rendered_text,current_state:["pre-rendered runtime alert"],reasons:[],next_actions:[],safety:["paper/live/order execution unchanged"],created_at_ms:$created_at_ms}')"
  printf '%s\n' "$payload" > "$payload_file"
  aws s3api put-object \
    --region "$AWS_REGION" \
    --bucket "$PIPELINE_ALERT_S3_BUCKET" \
    --key "$key" \
    --body "$payload_file" \
    --content-type application/json >/dev/null
  rm -f "$payload_file"
}

compact_tail() {
  local file="$1"
  local lines="${2:-12}"
  tail -n "$lines" "$file" | sed -E 's/[0-9]{12}/<aws-account-id>/g; s/[[:space:]]+$//'
}

failure_message() {
  local status="$1"
  local output_file="$2"
  local now_kst
  now_kst="$(TZ=Asia/Seoul date '+%Y-%m-%d %H:%M:%S KST')"
  cat <<EOF
[P1][intel-structuring-app] runtime check failed

결론:
Intel-L1 structuring runtime check가 실패했습니다. RAW_INTEL이 STRUCTURED_INTEL로 변환되지 못할 수 있습니다.

현재 상태:
- env: ${ALERT_ENV}
- check: scripts/check-runtime.sh
- exit_status: ${status}
- app_dir: ${APP_DIR}

주요 원인:
$(compact_tail "$output_file" 10 | sed 's/^/- /')

다음 행동:
- RAW_INTEL JetStream 접근 가능 여부 확인
- Market-L1 bucket과 INTEL-L1 output bucket 권한 확인
- Bedrock inference profile과 task role invoke 권한 확인
- 새 task log stream에 structured packet 처리 로그가 생기는지 확인

안전 상태:
- 이 알림은 intel structuring 상태 알림입니다.
- paper/live/order execution을 변경하지 않습니다.

발송 시각: ${now_kst}
EOF
}

success_message() {
  local output_file="$1"
  local now_kst
  now_kst="$(TZ=Asia/Seoul date '+%Y-%m-%d %H:%M:%S KST')"
  cat <<EOF
[P3][intel-structuring-app] runtime check summary

결론:
Intel-L1 structuring runtime check가 통과했습니다.

현재 상태:
- env: ${ALERT_ENV}
- check: scripts/check-runtime.sh

요약:
$(compact_tail "$output_file" 8 | sed 's/^/- /')

다음 행동:
- 일반 성공 알림은 기본적으로 끕니다.
- INTEL_STRUCTURING_ALERT_INCLUDE_SUCCESS=true일 때만 이 요약을 보냅니다.

발송 시각: ${now_kst}
EOF
}

self_test() {
  require_command jq
  local tmp
  tmp="$(mktemp)"
  cat > "$tmp" <<'EOF'
[1/4] NATS RAW_INTEL stream
nats: no servers available for connection
runtime checks failed before Bedrock profile validation
EOF
  local message
  message="$(failure_message 1 "$tmp")"
  [[ "$message" == *"[P1][intel-structuring-app]"* ]] || die "self-test expected P1 title"
  [[ "$message" == *"RAW_INTEL"* ]] || die "self-test expected RAW_INTEL context"
  [[ "$message" == *"다음 행동:"* ]] || die "self-test expected next actions"
  rm -f "$tmp"
  log "send-runtime-alert self-test passed"
}

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
    send_pipeline_alert P1 "runtime check failed" "$(failure_message "$status" "$output_file")"
    rm -f "$output_file"
    return "$status"
  fi

  if is_true "$INCLUDE_SUCCESS"; then
    send_pipeline_alert P3 "runtime check summary" "$(success_message "$output_file")"
  fi
  rm -f "$output_file"
}

main "$@"
