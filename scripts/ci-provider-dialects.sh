#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_root"

python3 scripts/check-provider-dialect-fixtures.py
for test_name in \
  provider_dialect_gate_93_registry_closure \
  provider_dialect_gate_93_invalid_oracle_executes \
  provider_dialect_gate_94_exact_dialect_bytes \
  provider_dialect_gate_95_recovery_and_mismatch \
  provider_dialect_gate_96_session_controls_to_wire
do
  cargo test -p provider --locked --test dialect_profiles "$test_name" -- --exact
done

cargo test -p tekes-worker --locked \
  slice7_worker_provider_loop_commits_outcome_before_release -- --exact
cargo test -p tekes-worker --locked \
  provider_context_tests::deepseek_responses_ignores_family_continuation_and_replays_full_history -- --exact
cargo test -p tekes-supervisor --locked --test secret_store \
  provider_readiness_accepts_a_configured_gateway_the_kernel_has_never_seen -- --exact
cargo test -p tekes-supervisor --locked --test secret_store \
  provider_readiness_rejects_an_unknown_dialect_as_misconfigured -- --exact
