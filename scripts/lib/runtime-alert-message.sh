#!/usr/bin/env bash

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
