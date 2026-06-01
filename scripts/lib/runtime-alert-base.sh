#!/usr/bin/env bash

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
