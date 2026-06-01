#!/usr/bin/env bash

self_test_env_file_loader() {
  local tmp_dir
  local env_path
  local invalid_env_path
  tmp_dir="$(mktemp -d)"
  INTEL_STRUCTURING_RUNTIME_SELF_TEST_TMP_DIR="$tmp_dir"
  trap 'rm -rf "$INTEL_STRUCTURING_RUNTIME_SELF_TEST_TMP_DIR"' EXIT
  env_path="$tmp_dir/runtime.env"
  invalid_env_path="$tmp_dir/invalid-runtime.env"
  cat > "$env_path" <<'EOF'
# comments and blank lines are ignored
export INTEL_STRUCTURING_ENV_SELF_TEST_REGION=ap-northeast-2
INTEL_STRUCTURING_ENV_SELF_TEST_QUOTED="quoted value"
INTEL_STRUCTURING_ENV_SELF_TEST_LITERAL="$(touch "$tmp_dir/env-loader-executed")"
EOF

  (
    load_env_file "$env_path"
    assert_equals "$INTEL_STRUCTURING_ENV_SELF_TEST_REGION" "ap-northeast-2"
    assert_equals "$INTEL_STRUCTURING_ENV_SELF_TEST_QUOTED" "quoted value"
    assert_equals \
      "$INTEL_STRUCTURING_ENV_SELF_TEST_LITERAL" \
      '$(touch "$tmp_dir/env-loader-executed")'
  )
  if [[ -e "$tmp_dir/env-loader-executed" ]]; then
    die "self-test env loader executed shell syntax from env file"
  fi

  printf 'BAD-KEY=value\n' > "$invalid_env_path"
  if (load_env_file "$invalid_env_path") >/dev/null 2>&1; then
    die "self-test expected invalid env file key to fail"
  fi
  rm -rf "$tmp_dir"
  trap - EXIT
  echo "check-runtime env loader self-test passed"
}
