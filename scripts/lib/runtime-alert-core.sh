#!/usr/bin/env bash

RUNTIME_ALERT_LIB_DIR="${RUNTIME_ALERT_LIB_DIR:-$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)}"

# shellcheck source=scripts/lib/runtime-alert-base.sh
source "$RUNTIME_ALERT_LIB_DIR/runtime-alert-base.sh"
# shellcheck source=scripts/lib/runtime-alert-pipeline.sh
source "$RUNTIME_ALERT_LIB_DIR/runtime-alert-pipeline.sh"
# shellcheck source=scripts/lib/runtime-alert-message.sh
source "$RUNTIME_ALERT_LIB_DIR/runtime-alert-message.sh"
# shellcheck source=scripts/lib/runtime-alert-self-test.sh
source "$RUNTIME_ALERT_LIB_DIR/runtime-alert-self-test.sh"
