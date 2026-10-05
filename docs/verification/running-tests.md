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
Slice 10 has process and external owners, so it does not pretend that every
gate is one Rust test. These are the exact commands owned by
`scripts/ci-slice10.sh` after the complete Slice-9 job:

| order | gate | owner | exact command/evidence transition |
|---:|---:|---|---|
| 1 | 71 | supervisor process integration | `cargo test -p tekes-supervisor --locked --test slice10_daemon slice10_gate_71_deployment_filesystem_and_ownership -- --exact` |
| 2 | 72 | supervisor support plus signed external UAT | `cargo test -p tekes-supervisor --locked --test slice10_daemon slice10_gate_72_launchd_crash_recovery -- --exact`; release evidence additionally follows the signed continuation below |
| 3 | 73 | selector publication integration | `cargo test -p tekes-selector --locked --test slice10_selector slice10_gate_73_install_upgrade_publication_crash_matrix -- --exact` |
| 4 | 74 | resident selector process harness | `cargo test -p tekes-selector --locked --test slice10_selector slice10_gate_74_crash_loop_external_rollback -- --exact` |
| 5 | 75 | production-path composition | the exact five-command block in `ci-slice10.sh`: Gate 74's selector rollback/ready errors; `process_host::tests::production_observability_projection_and_corruption_are_inert_and_redacted`; `secret_authority::tests::rotation_and_revocation_advance_floor_before_restart`; transport Gate 66's real HTTP rejection; `slice10_support_bundle_command_and_schema` |
| 6 | 76 | signed external production UAT | **no Rust test**: `prepare` → real reboot and target-user login → `verify` → `consume` → `acknowledge`, using the commands below |

Gate 75's in-repository credential test owns the durable generation/digest
logic; only Gate 76 may claim the signed Keychain ACL and OS persistence facts.
Gate 72's in-repository test is supporting process evidence; only its signed
continuation may claim launchd restart, selector SIGKILL and reboot/login.
Neither portable fixtures nor a Rust test with a Gate-76-shaped name can make
those external facts green.

`ci-slice12.sh` owns Gates 81–85 in the `plugins` crate. It first checks the
hand-authored lifecycle and historical carrier-compatibility fixtures, then runs formatting,
warnings-as-errors clippy and the complete package/lifecycle tests. When an
explicit signed `TEKES_COMPUTER_USE_ARCHIVE` is available it additionally
expands, validates and signature-checks that archive, enables only the inert
receipt, and resolves its executable
through the generic component relation without launching it. This creates no
current Computer Use plugin binding. The hermetic run says
only `carrier gates: passed` and prints `NOT QUALIFIED` for the real reference;
qualification CI sets `TEKES_SLICE12_REQUIRE_REFERENCE=1`, making a missing
private archive or unavailable macOS `codesign` a nonzero result.

`plugins::PluginStore::manage` is the supervisor-owned typed API that later
Client exposure composes through the additive `ProductionEndpointRoutes`
extension validator/execute seam. Slice 12 does not edit `TEKES_V2_ROUTES`;
14F must name and validate each optional public plugin method explicitly.
For Slice 13 runtime composition, `PluginComponentReference` is exactly the
`(plugin_id, component_id)` relation and
`PluginStore::resolve_executable_component` returns the immutable executable,
plugin data path and package-digest `plugin_generation`. The resolver launches
nothing and contains no package-specific mapping.

Slice 13 adds one production crate, `mcp`, and the following exact mappings.
During isolated parallel development the script starts from the published
Slice-10 baseline; before publication it rebases onto merged Slice 12 and
invokes `ci-slice12.sh` first.

| order | gate | Rust test function |
|---:|---:|---|
| 1 | 86 | `slice13_gate_86_protocol_handshake_and_framing` |
| 2 | 87 | `slice13_gate_87_operations_and_cancellation_shapes` |
| 3 | 88 | `slice13_gate_88_stdio_and_daemon_pool_ownership` |
| 4 | 89 | `slice13_gate_89_http_oauth_authorization_partition` |
| 5 | 90 | `slice13_gate_90_catalog_projection_and_collision` |
| 6 | 91 | `slice13_gate_91_management_publication_and_recovery` |
| 7 | 92 | `slice13_gate_92_closed_recovery_matrix` |

`scripts/ci-slice13.sh` runs these names one by one with `--exact`, verifies
the complete `fixtures/mcp-runtime/` inventory, and then runs the `mcp`,
supervisor dynamic-route, worker catalog/search, and endpoint management
suites. HTTP tests bind loopback only; no real secret enters CI.

Slice 14B's `safepoint` crate and Gates 97–100 were removed on 2026-09-06
(simplification item 8); `scripts/ci-slice14b.sh` now runs only the retained
Slice-14C/14D/14E lanes, the worker suite and the workspace check/clippy.

Gate 90 additionally runs
`slice13_gate_90_plugin_bridge_resolves_immutable_slice12_generation` in the
supervisor crate: it installs a generic MCP plugin through Slice 12, resolves
the immutable executable/data path and package-digest generation, then
performs a real handshake. Gate 91 additionally runs
`slice13_gate_91_production_assembly_mounts_and_executes_management`: the
daemon assembly must advertise all seven additive methods and dispatch closed
save/list requests through the same production composite. The MCP crate fault
matrix covers every registry/reference/receipt boundary.

Slice 14D adds the `thread-search` crate and keeps Client exposure deferred to
14F. Its exact mappings are:

| order | gate | Rust test function |
|---:|---:|---|
| 1 | 105 | `slice14d_gate_105_contract_oracle_and_stable_identity` |
| 2 | 106 | `slice14d_gate_106_paging_archive_visibility_and_membership_lock` |
| 3 | 107 | `slice14d_gate_107_rebuild_and_stale_corrupt_fallback` |
| 4 | 108 | `slice14d_gate_108_fail_closed_and_semantic_non_mutation` |

`scripts/ci-slice14d.sh` runs the complete Slice-13 prerequisite, validates
the hand-authored `fixtures/thread-search/` corpus, executes Gates 105–108 in
order, and finishes with workspace check/clippy. The production seam is
`thread_search::ThreadSearchAuthority`; no endpoint/supervisor source may
register `thread.search`, `thread/search`, or `session.search` in this slice.

Slice 14E remains in the existing `tools`, `engine`, and `tekes-worker`
crates; a Web-specific daemon or endpoint crate is forbidden. Its exact
mappings are:

| order | gate | Rust test function |
|---:|---:|---|
| 1 | 109 | `slice14e_gate_109_web_fetch_extraction_chain` plus `slice14e_gate_109_web_fetch_result_is_closed_and_drops_raw_body` |
| 2 | 110 | `slice14e_gate_110_web_network_policy_and_bounds` |
| 3 | 111 | `slice14e_gate_111_web_search_exact_wire_and_negative_matrix` |
| 4 | 112 | `slice14e_gate_112_web_parity_retirement_and_secret_lifecycle` |

`scripts/ci-slice14e.sh` runs the complete Slice-13 prerequisite, validates
both Web corpora, executes the production extractor/result, network and
Tavily credential paths, and finishes with workspace check/clippy. The only
network implementation is `tools::BoundedHttpClient`; the only search
provider in format 1 is the worker's brokered `TavilySearchProvider`.

For implementation integration before a production release, action
`preflight` runs the complete Slice-9 prerequisite, repository-owned gates
71–75, full workspace suite, signed bundle/coordinator builds, final-path
signature verification, real selector CMS/profile parsing, and signed Client
admission without installing, switching users, or rebooting. A green preflight
permits the implementation commit to merge but does **not** mark the external
halves of Gate 72 or Gate 76 green; both remain mandatory before a production
tag or distribution.

After constructing the profiled supervisor app, `ci-slice10.sh` also runs the
ignored-on-ordinary-CI `macos_signature` integration test with explicit app,
requirement, team, identifier and access-group inputs. That test invokes the
real Darwin `codesign`, CMS and provisioning-profile parser used by selector;
it is required supporting evidence for Gate 73, but it does not replace the
external Gate 72/76 reboot lane.

The signed continuation is one durable operation, resumed explicitly:

```sh
# Development integration: no install, account switch, or reboot.
TEKES_SLICE10_PRODUCTION_ACTION=preflight \
TEKES_SLICE10_SIGNING_IDENTITY="$identity" \
TEKES_SLICE10_TEAM_ID="$team" \
TEKES_SLICE10_CLIENT_REQUIREMENT="$client_requirement" \
TEKES_SLICE10_CLIENT_UAT="$client_uat" \
scripts/ci-slice10.sh

# Phase 1: build/sign, install the resume job, and return operation UUID.
TEKES_SLICE10_PRODUCTION_ACTION=prepare \
TEKES_SLICE10_PRODUCTION_GATE=76 \
TEKES_SLICE10_SIGNING_IDENTITY="$identity" \
TEKES_SLICE10_TEAM_ID="$team" \
TEKES_SLICE10_CLIENT_REQUIREMENT="$client_requirement" \
TEKES_SLICE10_CLIENT_UAT="$client_uat" \
scripts/ci-slice10.sh

# Phase 2 is a real reboot followed by target-user login. Then use the exact
# operation UUID printed by prepare for every remaining phase.
TEKES_SLICE10_PRODUCTION_ACTION=verify \
TEKES_SLICE10_PRODUCTION_GATE=76 \
TEKES_SLICE10_PRODUCTION_OPERATION="$operation" scripts/ci-slice10.sh
TEKES_SLICE10_PRODUCTION_ACTION=consume \
TEKES_SLICE10_PRODUCTION_GATE=76 \
TEKES_SLICE10_PRODUCTION_OPERATION="$operation" scripts/ci-slice10.sh
TEKES_SLICE10_PRODUCTION_ACTION=acknowledge \
TEKES_SLICE10_PRODUCTION_GATE=76 \
TEKES_SLICE10_PRODUCTION_OPERATION="$operation" scripts/ci-slice10.sh
```

Gate 72 uses the same four-phase sequence with
`TEKES_SLICE10_PRODUCTION_GATE=72`. `verify` cannot substitute for `consume`,
and only `acknowledge` may remove the retained operation key after evidence has
been archived. The selector is never a supervisor self-update API.

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
# Integration lane. Release qualification replaces preflight with the
# prepare/reboot/verify/consume/acknowledge sequence above for Gates 72 and 76.
TEKES_SLICE10_PRODUCTION_ACTION=preflight \
TEKES_KERNEL_FIXTURES="$PWD/fixtures" scripts/ci-slice10.sh
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
