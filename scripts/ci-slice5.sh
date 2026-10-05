#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$(pwd)/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice4.sh

run_gate() {
  cargo test -p conformance --locked --test slice5_gates "$1" -- --exact
}

run_gate slice5_gate_15_redact_rewrite_closure
run_gate slice5_gate_16_rewrite_publication_crash_matrix
run_gate slice5_gate_24_staging_ownership_gc
