#!/bin/sh
set -eu

repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"

: "${TEKES_KERNEL_FIXTURES:=$repo_dir/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice13.sh
python3 scripts/check-web-search-provider-fixtures.py "$TEKES_KERNEL_FIXTURES"
python3 scripts/check-web-tools-fixtures.py "$TEKES_KERNEL_FIXTURES"

cargo test -p tools --locked --test slice14e_web \
  slice14e_gate_109_web_fetch_extraction_chain -- --exact
cargo test -p engine --locked --test slice14e_web \
  slice14e_gate_109_web_fetch_result_is_closed_and_drops_raw_body -- --exact
cargo test -p tools --locked --test slice14e_web \
  slice14e_gate_110_web_network_policy_and_bounds -- --exact
cargo test -p tekes-worker --locked \
  provider_context_tests::slice14e_gate_111_web_search_exact_wire_and_negative_matrix -- --exact
cargo test -p tekes-worker --locked \
  provider_context_tests::slice14e_gate_112_web_parity_retirement_and_secret_lifecycle -- --exact

cargo test -p tools --locked --test slice14e_web
cargo test -p engine --locked --test slice14e_web
cargo test -p tekes-worker --locked web_search_
cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
