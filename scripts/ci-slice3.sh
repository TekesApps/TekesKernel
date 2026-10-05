#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$(pwd)/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice2.sh

run_gate() {
  cargo test -p conformance --locked "$1" -- --exact
}

run_gate slice3_gate_42_checkpoint_marker_barrier
