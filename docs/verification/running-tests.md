# Running tests and checks

[Verification · documentation home](../README.md) · [Verification entry point](README.md) · [Next: conformance gates by topic](gates/README.md)

## Fixtures and test mapping

`test_support::FixtureRoot` resolves:

1. `TEKES_KERNEL_FIXTURES` when absolute and present;
2. otherwise walks upward from `env!("CARGO_MANIFEST_DIR")` until both the
   workspace `Cargo.toml` and `fixtures/manifest.json` exist;
3. otherwise returns a test error listing every checked path.

The conformance crate uses stable function names
`slice1_gate_NN_<snake_case_name>`. The exact mapping is:

| order | gate | Rust test function |
|---:|---|---|
| 1 | 28 | `slice1_gate_28_canonical_envelope_fixtures` |
| 2 | 17 | `slice1_gate_17_downgrade_evolution` |
| 3 | 32 | `slice1_gate_32_resume_policy_encoding` |
| 4 | 18 | `slice1_gate_18_upgrade_gate_barrier` |
| 5 | 29 | `slice1_gate_29_torn_tail_fault_injection` |
| 6 | 30 | `slice1_gate_30_checkpoint_marker_replay` |
| 7 | 31 | `slice1_gate_31_protocol_negotiation_reject` |
| 8 | 4 | `slice1_gate_04_attempt_lease_commit` |
| 9 | 5 | `slice1_gate_05_outcome_usage_crash_window` |
| 10 | 6 | `slice1_gate_06_completed_turn_usage_exit` |
| 11 | 33 | `slice1_gate_33_run_mode_arbitration` |
| 12 | 34 | `slice1_gate_34_slow_hold_lifecycle` |
| 13 | 1 | `slice1_gate_01_cold_boot_create_retry` |
| 14 | 2 | `slice1_gate_02_dead_target_keyed_delivery` |
| 15 | 3 | `slice1_gate_03_alive_target_receipt_correlation` |
| 16 | 13 | `slice1_gate_13_busy_unknown_worker` |
| 17 | 19 | `slice1_gate_19_canary_per_run_attribution` |
| 18 | 12 | `slice1_gate_12_supervisor_total_failure` |
| 19 | 14 | `slice1_gate_14_stop_mid_grandchild_crash` |

`scripts/ci-slice1.sh` invokes these names one by one in table order using
`cargo test -p conformance --locked <exact-name> -- --exact`.
Cargo's parallel/default test order is not used as the gate order.

Slice 2 adds this ordered mapping after the complete Slice-1 job:

| order | gate | Rust test function |
|---:|---|---|
| 1 | 35 | `slice2_gate_35_config_contract_fixtures` |
| 2 | 37 | `slice2_gate_37_instruction_oracle_and_rejections` |
| 3 | 36 | `slice2_gate_36_config_atomic_publish_and_revocation` |
| 4 | 20 | `slice2_gate_20_instruction_snapshot_race` |
| 5 | 21 | `slice2_gate_21_instruction_next_launch_only` |

`scripts/ci-slice2.sh` first invokes `ci-slice1.sh`, then the five names above
one at a time, then the real `supervisor_passes_immutable_profile_descriptors`
process test. It is the authoritative combined gate for the implemented
Slice-2 surface.

Slice 3 adds this ordered mapping after the complete Slice-2 job:

| order | gate | Rust test function |
|---:|---|---|
| 1 | 42 | `slice3_gate_42_checkpoint_marker_barrier` |

Gates 38–41 (checkpoint acceleration) are retired. `scripts/ci-slice3.sh`
first invokes `ci-slice2.sh`, then the name above. It is the authoritative combined gate for the implemented
surface.

Slice 4 adds the following ordered mapping after the complete Slice-3 job:

| order | gate | Rust test function |
|---:|---|---|
| 1 | 43 | `slice4_gate_43_builtin_tool_manifest_parity` |
| 2 | 44 | `slice4_gate_44_exec_helper_process_protocol` |
| 3 | 45 | `slice4_gate_45_sandbox_probe_and_approval_binding` |
| 4 | 46 | `slice4_gate_46_hook_process_limits_and_correlation` |
| 5 | 7 | `slice4_gate_07_mutated_tool_write_ahead` |
| 6 | 10 | `slice4_gate_10_descriptor_first_file_open` |
| 7 | 11 | `slice4_gate_11_sandbox_backend_probe` |
| 8 | 8 | `slice4_gate_08_ask_user_fast_answer` |
| 9 | 9 | `slice4_gate_09_spawn_validator_ordering` |

`scripts/ci-slice4.sh` must first invoke `ci-slice3.sh`, then run these nine
names one at a time. (The migration parity check against the legacy
repositories needs private checkouts and is not part of the public CI.) This mapping is
normative for Slice 4; the script is not considered present or green until
every listed test exists.

Slice 5 adds the following ordered mapping after the complete Slice-4 job:

| order | gate | Rust test function |
|---:|---|---|
| 1 | 15 | `slice5_gate_15_redact_rewrite_closure` |
| 2 | 16 | `slice5_gate_16_rewrite_publication_crash_matrix` |
| 3 | 24 | `slice5_gate_24_staging_ownership_gc` |

`scripts/ci-slice5.sh` first invokes `ci-slice4.sh`, then these three names one
at a time. The implementation lives in `store::rewrite`; it owns the closed
operation type, deterministic projection, carrier scan, source-prefix binding,
phase recovery, retirement, and rewrite-only GC.

Slice 6 adds the following ordered mapping after the complete Slice-5 job:

| order | gate | Rust test function |
|---:|---|---|
| 1 | 47 | `slice6_gate_47_endpoint_authority_byte_sync` |
| 2 | 48 | `slice6_gate_48_stable_projection_journal` |
| 3 | 49 | `slice6_gate_49_chunk_history_live_identity` |
| 4 | 50 | `slice6_gate_50_endpoint_mutation_and_archive_contract` |
| 5 | 51 | `slice6_gate_51_inventory_and_model_readiness_are_independent` |
| 6 | 52 | `slice6_gate_52_stitch_gap_overlap_and_unknown_fail_closed` |

`scripts/ci-slice6.sh` first invokes `ci-slice5.sh`, verifies the pinned
Client-owned corpus, then runs these six names one at a time in `endpoint`.
The workspace parity lane adds `--require-source`; standalone CI verifies the
committed SHA-256 lock without assuming a sibling checkout.

Slices 7–10 add these mappings; every script first invokes the preceding slice
script and then runs its six names one at a time:

| slice | order | gate | Rust test function |
|---:|---:|---:|---|
| 7 | 1 | 53 | `slice7_gate_53_provider_dialect_parity_oracle` |
| 7 | 2 | 54 | `slice7_gate_54_provider_stream_framing_bounds` |
| 7 | 3 | 57 | `slice7_gate_57_provider_terminal_normalization` |
| 7 | 4 | 55 | `slice7_gate_55_provider_send_recovery_matrix` |
| 7 | 5 | 56 | `slice7_gate_56_provider_http_classification_and_secrets` |
| 7 | 6 | 58 | `slice7_gate_58_provider_overflow_compact_retry` |
| 8 | 1 | 59 | `slice8_gate_59_tool_dispatcher_catalog_coverage` |
| 8 | 2 | 60 | `slice8_gate_60_tool_write_ahead_crash_matrix` |
| 8 | 3 | 61 | `slice8_gate_61_tool_hold_stop_resume` |
| 8 | 4 | 62 | `slice8_gate_62_supervisor_tool_control_dedup` |
| 8 | 5 | 63 | `slice8_gate_63_job_and_helper_lifetime` |
| 8 | 6 | 64 | `slice8_gate_64_tool_policy_secret_and_network` |
| 9 | 1 | 65 | `slice9_gate_65_endpoint_transport_authority` |
| 9 | 2 | 66 | `slice9_gate_66_endpoint_http_error_and_idempotency` |
| 9 | 3 | 67 | `slice9_gate_67_endpoint_subscribe_atomicity` |
| 9 | 4 | 68 | `slice9_gate_68_endpoint_raw_history_reconnect` |
| 9 | 5 | 69 | `slice9_gate_69_endpoint_drain_backpressure_security` |
| 9 | 6 | 70 | `slice9_gate_70_endpoint_archive_client_rematerialization` |
| 12 | 1 | 81 | `slice12_gate_81_hermetic_package_carrier_compatibility` |
| 12 | 2 | 82 | `slice12_gate_82_hermetic_signatures_trust_and_grants_fail_closed` |
| 12 | 3 | 83 | `slice12_gate_83_install_update_remove_enable_and_collision_ownership` |
| 12 | 4 | 84 | `slice12_gate_84_transaction_recovery_is_commit_or_rollback` |
| 12 | 5 | 85 | `slice12_gate_85_typed_management_is_inert_and_deterministic` |

`ci-slice7.sh` completes and checks every provider fixture/source lock first.
All six named gates run in `provider`; supporting worker tests exercise the
real recovery/context assembly for 55/58, while Gate 56 additionally runs the
supervisor SecretStore readiness suite and worker descriptor path.
`ci-slice8.sh` verifies tool
registries in both directions: gate 59 spans tools+engine, 60/61/64 run through
engine+worker, 62 spans worker-control+supervisor, and 63 runs the real helper
and job broker. `ci-slice9.sh` runs 65/66 in transport, 67/69 in conformance
over the supervisor host seam. Client-library and application acceptance belongs to their
own repositories and is not source-coupled into the Kernel test graph.
Slice 10 (Gates 71–76) covered the launchd deployment, which has been removed
together with `scripts/ci-slice10.sh`; `ci-slice11.sh` now follows the
Slice-9 job directly.

## Rust CI

From the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo build --workspace --locked
cargo build --workspace --release --locked
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice1.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice2.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice3.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice4.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice5.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice6.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice7.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice8.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice9.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice11.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice12.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice13.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice14d.sh
TEKES_KERNEL_FIXTURES="$PWD/fixtures" cargo test --workspace --locked
git diff --exit-code -- fixtures spec docs/verification/gates/README.md
```

CI disables external network after dependency fetch while permitting isolated
loopback fixture servers and the real local Client transport harness, and uses a fresh temporary storage
root per crash case, sets a 10-minute timeout per gate, and fails if any child
process or file lock remains after teardown. The fixture manifest is compared
with disk in both directions before any test runs.
