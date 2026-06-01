#!/usr/bin/env bash

die() {
  printf 'runtime check failed: %s\n' "$*" >&2
  exit 1
}

assert_equals() {
  local actual="$1"
  local expected="$2"
  if [[ "$actual" != "$expected" ]]; then
    die "self-test expected '$expected' but got '$actual'"
  fi
}
