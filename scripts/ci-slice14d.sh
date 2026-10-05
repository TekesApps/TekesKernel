#!/bin/sh
set -eu

repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"

scripts/ci-slice13.sh
python3 scripts/check-thread-search-fixtures.py

for gate in \
  slice14d_gate_105_contract_oracle_and_stable_identity \
  slice14d_gate_106_paging_archive_visibility_and_membership_lock \
  slice14d_gate_107_rebuild_and_stale_corrupt_fallback \
  slice14d_gate_108_fail_closed_and_semantic_non_mutation
do
  cargo test -p thread-search --locked --test slice14d_gates "$gate" -- --exact
done

cargo test -p thread-search --locked
cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
