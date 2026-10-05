#!/bin/sh
set -eu

: "${TEKES_KERNEL_FIXTURES:=$(pwd)/fixtures}"
export TEKES_KERNEL_FIXTURES

scripts/ci-slice7.sh
python3 scripts/check-web-search-provider-fixtures.py

cargo test -p tools --locked --test slice8_tool_runtime_oracle \
  slice8_gate_59_tool_dispatcher_catalog_coverage -- --exact
cargo test -p tekes-worker --locked \
  provider_context_tests::slice8_gate_60_tool_write_ahead_crash_matrix -- --exact
cargo test -p tekes-worker --locked \
  provider_context_tests::slice8_gate_61_tool_hold_stop_resume -- --exact
cargo test -p tekes-supervisor --locked --test shell \
  slice8_gate_62_supervisor_tool_control_dedup -- --exact
cargo test -p tools --locked --test slice8_runtime_backends \
  slice8_gate_63_job_and_helper_lifetime -- --exact
cargo test -p engine --locked --test slice8_system_tools \
  slice8_gate_64_tool_policy_secret_and_network -- --exact

# Supporting Slice-8 contract suites exercise every schema, backend, workflow
# authority and worker-control durable-control byte oracle in addition to the six named gates.
cargo test -p tools --locked --test slice8_schema_registry
cargo test -p tools --locked --test slice8_tool_runtime_oracle
cargo test -p tools --locked --test slice8_runtime_backends
cargo test -p engine --locked --test slice8_dispatcher
cargo test -p engine --locked --test slice8_system_tools
cargo test -p engine --locked --test slice8_workflow
cargo test -p worker-control --locked --test slice8_v2
cargo test -p profile --locked --test slice8_launch_bindings
cargo test -p schema --locked --test slice8_launch_bindings
cargo test -p engine --locked --test slice8_dynamic_catalog
cargo test -p tekes-worker --locked web_search_
cargo test -p tekes-worker --locked tavily_request_and_response_match_the_oracle
