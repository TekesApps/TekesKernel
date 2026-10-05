#!/bin/sh
set -eu

repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"

scripts/ci-slice13.sh
python3 scripts/check-schedule-fixtures.py

for gate in \
  slice14c_gate_101_contract_oracle_cron_and_timezone \
  slice14c_gate_102_keyed_management_durability_and_dedup \
  slice14c_gate_103_claim_redrive_restart_and_missed_policy \
  slice14c_gate_104_fail_closed_nonmutation_and_internal_boundary
do
  cargo test -p schedule --locked --test slice14c_gates "$gate" -- --exact
done

cargo test -p tekes-supervisor --locked \
  process_host::tests::slice14c_gate_103_production_claim_uses_management_and_delivery_authorities -- --exact

cargo test -p schedule --locked
cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
