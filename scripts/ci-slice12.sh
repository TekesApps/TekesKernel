#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root"

python3 scripts/check-plugin-fixtures.py
cargo fmt --all -- --check
cargo clippy -p plugins --all-targets --locked -- -D warnings
cargo test -p plugins --locked
echo "Slice 12 hermetic carrier gates: passed; real reference is not qualified by these gates"

if [ -z "${TEKES_COMPUTER_USE_ARCHIVE:-}" ]; then
  echo "Slice 12 real-reference qualification: NOT QUALIFIED (TEKES_COMPUTER_USE_ARCHIVE is unset)" >&2
  if [ "${TEKES_SLICE12_REQUIRE_REFERENCE:-0}" = "1" ]; then
    exit 78
  fi
  exit 0
fi

if [ "$(uname -s)" != "Darwin" ] || ! command -v codesign >/dev/null 2>&1; then
  echo "Slice 12 real-reference qualification: NOT QUALIFIED (macOS codesign is unavailable)" >&2
  exit 78
fi

TEKES_COMPUTER_USE_ARCHIVE="$TEKES_COMPUTER_USE_ARCHIVE" \
  cargo test -p plugins --locked --test slice12_gates \
    slice12_reference_qualification_external_archive_without_launch \
    -- --ignored --exact
echo "Slice 12 real-reference qualification: QUALIFIED"
