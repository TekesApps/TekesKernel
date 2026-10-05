#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$PWD/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice5.sh
scripts/check-endpoint-authority.py

run_gate() {
  cargo test -p endpoint --locked --test slice6_gates "$1" -- --exact
}

run_gate slice6_gate_47_endpoint_authority_byte_sync
run_gate slice6_gate_48_stable_projection_journal
run_gate slice6_gate_49_chunk_history_live_identity
run_gate slice6_gate_50_endpoint_mutation_and_archive_contract
run_gate slice6_gate_51_inventory_and_model_readiness_are_independent
run_gate slice6_gate_52_stitch_gap_overlap_and_unknown_fail_closed
