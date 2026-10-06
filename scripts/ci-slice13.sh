#!/bin/sh
set -eu

repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"

scripts/ci-slice12.sh

python3 scripts/check-mcp-runtime-fixtures.py

for gate in \
  slice13_gate_86_protocol_handshake_and_framing \
  slice13_gate_87_operations_and_cancellation_shapes \
  slice13_gate_88_stdio_and_daemon_pool_ownership \
  slice13_gate_89_http_oauth_authorization_partition \
  slice13_gate_90_catalog_projection_and_collision \
  slice13_gate_91_management_publication_and_recovery \
  slice13_gate_92_closed_recovery_matrix
do
  cargo test -p mcp --locked --test slice13_gates "$gate" -- --exact
done

cargo test -p tekes-supervisor --locked --test slice13_management \
  slice13_gate_88_standalone_mutations_close_the_bound_generation -- --exact
cargo test -p tekes-supervisor --locked --test slice13_management \
  slice13_gate_89_pending_oauth_requires_exact_bound_ownership -- --exact
cargo test -p tekes-supervisor --locked --test slice13_management \
  slice13_gate_90_plugin_bridge_resolves_immutable_slice12_generation -- --exact
cargo test -p tekes-supervisor --locked --test slice13_management \
  slice13_gate_91_production_assembly_mounts_and_executes_management -- --exact
cargo test -p tekes-supervisor --locked --test slice13_management \
  slice13_gate_91_nested_credential_values_fail_closed -- --exact

cargo test -p mcp --locked
cargo test -p tekes-supervisor --locked
cargo check --workspace --locked
