#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$(pwd)/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice3.sh


run_gate() {
  cargo test -p tools --locked --test slice4_gates "$1" -- --exact
}

run_gate slice4_gate_43_builtin_tool_manifest_parity
run_gate slice4_gate_44_exec_helper_process_protocol
run_gate slice4_gate_45_sandbox_probe_and_approval_binding
run_gate slice4_gate_46_hook_process_limits_and_correlation
run_gate slice4_gate_07_mutated_tool_write_ahead
run_gate slice4_gate_10_descriptor_first_file_open
run_gate slice4_gate_11_sandbox_backend_probe
run_gate slice4_gate_08_ask_user_fast_answer
cargo test -p tekes-worker --locked --test shell \
  slice4_gate_09_spawn_validator_ordering -- --exact

# Supporting contract oracles required by the Slice-4 profile.
run_gate tool_contract_oracles
run_gate spawn_seed_contract_oracle

cargo test -p tekes-worker --locked --test shell \
  worker_tool_control_integration -- --exact
