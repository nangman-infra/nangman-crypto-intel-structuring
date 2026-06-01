#!/usr/bin/env bash
set -euo pipefail

APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="${1:-$APP_DIR/.env}"
SCRIPT_DIR="$APP_DIR/scripts"

# shellcheck source=scripts/lib/check-runtime-core.sh
source "$SCRIPT_DIR/lib/check-runtime-core.sh"
# shellcheck source=scripts/lib/check-runtime-env.sh
source "$SCRIPT_DIR/lib/check-runtime-env.sh"
# shellcheck source=scripts/lib/check-runtime-self-test.sh
source "$SCRIPT_DIR/lib/check-runtime-self-test.sh"
# shellcheck source=scripts/lib/check-runtime-checks.sh
source "$SCRIPT_DIR/lib/check-runtime-checks.sh"

if [[ "${INTEL_STRUCTURING_RUNTIME_SELF_TEST:-}" == "1" ]]; then
  self_test_env_file_loader
  exit 0
fi

load_env_file "$ENV_FILE"

load_runtime_defaults
run_runtime_checks

echo "runtime checks passed"
