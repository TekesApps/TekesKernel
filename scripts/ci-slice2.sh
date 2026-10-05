#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$(pwd)/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice1.sh

run_gate() {
  cargo test -p conformance --locked "$1" -- --exact
}

run_gate slice2_gate_35_config_contract_fixtures
run_gate slice2_gate_37_instruction_oracle_and_rejections
run_gate slice2_gate_36_config_atomic_publish_and_revocation
run_gate slice2_gate_20_instruction_snapshot_race
run_gate slice2_gate_21_instruction_next_launch_only

cargo test -p tekes-worker --locked --test shell \
  supervisor_passes_immutable_profile_descriptors -- --exact
