#!/usr/bin/env bash

send_pipeline_alert() {
  local priority="$1"
  local title="$2"
  local text="$3"
  if [[ -z "$PIPELINE_ALERT_S3_BUCKET" ]]; then
    die "NANGMAN_PIPELINE_ALERT_S3_BUCKET or INTEL_STRUCTURING_PIPELINE_ALERT_S3_BUCKET is required"
  fi
  local now_ms dt hour event_id key payload
  now_ms="$(date -u +%s000)"
  dt="$(date -u +%Y-%m-%d)"
  hour="$(date -u +%H)"
  event_id="pipeline_alert_intel_structuring_${now_ms}_$$"
  key="${PIPELINE_ALERT_S3_PREFIX%/}/dt=${dt}/hour=${hour}/app=${APP_NAME}/priority=${priority}/${event_id}.json"
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

  local payload_file put_status
  payload_file="$(mktemp)"
  printf '%s\n' "$payload" > "$payload_file"
  put_status=0
  aws s3api put-object \
    --region "$AWS_REGION" \
    --bucket "$PIPELINE_ALERT_S3_BUCKET" \
    --key "$key" \
    --body "$payload_file" \
    --content-type application/json >/dev/null || put_status=$?
  rm -f "$payload_file"
  return "$put_status"
}
