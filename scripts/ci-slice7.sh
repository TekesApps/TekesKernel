#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_root"

"$repo_root/scripts/ci-slice6.sh"

for test_name in \
  slice7_gate_53_provider_dialect_parity_oracle \
  slice7_gate_54_provider_stream_framing_bounds \
  slice7_gate_57_provider_terminal_normalization \
  slice7_gate_55_provider_send_recovery_matrix \
  slice7_gate_56_provider_http_classification_and_secrets \
  slice7_gate_58_provider_overflow_compact_retry
do
  cargo test -p provider --locked "$test_name" -- --exact
done

cargo test -p provider --locked credential::tests::live_rotation_delivers_revocation_before_control_returns -- --exact
cargo test -p provider --locked credential::tests::live_store_revocation_delivers_revocation_before_control_returns -- --exact
cargo test -p tekes-supervisor --locked --test secret_store
cargo test -p tekes-worker --locked slice7_credential_descriptor_launch -- --exact
cargo test -p tekes-worker --locked slice7_worker_provider_loop_commits_outcome_before_release -- --exact
cargo test -p tekes-worker --locked slice7_provider_reconcile_closes_unpaired_attempt -- --exact
cargo test -p tekes-worker --locked slice7_provider_reconcile_completes_adopt_from_asset_only -- --exact
cargo test -p tekes-worker --locked slice7_ordinary_recovery_closes_old_attempt_before_resend -- --exact
cargo test -p tekes-worker --locked provider_context_tests::stateless_projection_replays_sealed_baseline_and_only_new_admits -- --exact
cargo test -p tekes-worker --locked provider_context_tests::server_managed_projection_uses_continuation_and_only_pending_host_facts -- --exact
cargo test -p tekes-worker --locked provider_context_tests::overflow_compaction_is_checkpointed_and_changes_the_retry_projection -- --exact
cargo test -p tekes-worker --locked provider_context_tests::context_overflow_closes_compacts_and_retries_without_resetting_the_turn -- --exact
cargo test -p tekes-worker --locked provider_context_tests::child_seed_snapshot_projects_normalized_payload_without_parent_baseline -- --exact
