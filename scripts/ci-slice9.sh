#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$(pwd)/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice8.sh
python3 scripts/check-endpoint-transport-fixtures.py

# Production assembly proof precedes carrier-mechanics and Client-driver
# checks: exact 20-route registration, recovery-gated readiness, structural
# 400 boundary, shared-admission respond, and archive mux/host lifecycle.
cargo test -p tekes-supervisor --locked --test slice9_endpoint_carrier
cargo test -p tekes-supervisor --locked --test slice9_endpoint_host

cargo test -p transport --locked --test slice9_transport \
  slice9_gate_65_endpoint_transport_authority -- --exact
cargo test -p transport --locked --test slice9_transport \
  slice9_gate_66_endpoint_http_error_and_idempotency -- --exact
cargo test -p conformance --locked --test slice9_gates \
  slice9_gate_67_endpoint_subscribe_atomicity -- --exact
cargo test -p conformance --locked --test slice9_gates \
  slice9_gate_68_endpoint_raw_history_reconnect -- --exact
cargo test -p conformance --locked --test slice9_gates \
  slice9_gate_69_endpoint_drain_backpressure_security -- --exact
cargo test -p conformance --locked --test slice9_gates \
  slice9_gate_70_endpoint_archive_client_rematerialization -- --exact

# Supporting transport tests keep the complete HTTP/WebSocket carrier surface
# live in addition to the six named release gates.
cargo test -p transport --locked --test slice9_transport
