#!/usr/bin/env bash

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
