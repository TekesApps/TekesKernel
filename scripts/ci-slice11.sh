#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$(pwd)/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice9.sh

cargo test -p conformance --locked --test slice11_gates \
  slice11_gate_77_skill_package_migration_and_precedence -- --exact
cargo test -p conformance --locked --test slice11_gates \
  slice11_gate_78_command_catalog_expansion_and_negative_corpus -- --exact
cargo test -p conformance --locked --test slice11_gates \
  slice11_gate_79_resource_capabilities_and_keyed_command_submission -- --exact
cargo test -p conformance --locked --test slice11_gates \
  slice11_gate_80_production_input_seam_and_frozen_v2_authority -- --exact

# Focused support suites keep the package parser, model projection, carrier
# seam and collision-free extension composition live around the four gates.
cargo test -p profile --locked resources
cargo test -p tekes-supervisor --locked resource_capability
cargo test -p tekes-worker --locked skill_catalog
