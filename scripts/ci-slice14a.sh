#!/bin/sh
set -eu

repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"

python3 scripts/check-mcp-reference-qualification-fixtures.py
cargo fmt --all -- --check
cargo clippy -p tekes-supervisor --all-targets --locked -- -D warnings

for gate in \
  slice14a_gate_117_generic_local_mcp_configuration \
  slice14a_gate_118_twelve_tool_discovery_and_call \
  slice14a_gate_119_hermetic_tcc_denial_grant_and_prompt_suppression \
  slice14a_gate_120_stop_restart_crash_recovery_and_no_special_cases
do
  cargo test -p tekes-supervisor --locked --test slice14a_reference_package "$gate" -- --exact
done

if [ ! -f crates/conformance/tests/slice14f_gates.rs ]; then
  echo "Slice 14A prerequisite: NOT CONVERGED (Slice 14F implementation gates are not merged)" >&2
  if [ "${TEKES_SLICE14A_REQUIRE_CONVERGED:-0}" = "1" ]; then
    exit 78
  fi
fi

if [ -z "${TEKES_SLICE14A_REFERENCE_EXECUTABLE:-}" ] || \
   [ -z "${TEKES_SLICE14A_DENIED_REFERENCE_EXECUTABLE:-}" ] || \
   [ -z "${TEKES_SLICE14A_REFERENCE_REVISION:-}" ]; then
  echo "Slice 14A real reference: NOT QUALIFIED (production executable, denial-canary executable and locked source revision are required)" >&2
  if [ "${TEKES_SLICE14A_REQUIRE_REFERENCE:-0}" = "1" ]; then
    exit 78
  fi
  exit 0
fi

if [ "$(uname -s)" != "Darwin" ] || ! command -v codesign >/dev/null 2>&1; then
  echo "Slice 14A real reference: NOT QUALIFIED (macOS codesign is unavailable)" >&2
  exit 78
fi

cargo test -p tekes-supervisor --locked --test slice14a_reference_package \
  slice14a_real_reference_mcp_safe_preflight -- --ignored --exact
echo "Slice 14A real reference: production lifecycle and qualification-only denied path passed; interactive TCC grant/call remains manual UAT"
