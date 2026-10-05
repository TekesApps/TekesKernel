# Crate README semantic audit — supervisor, worker, engine, provider, mcp (2026-09-05)

[Verification entry point](../README.md)

Scope: every behavioral claim in the five crates' `docs/README.md` (module
rows, entry points, call chains, boundary paragraphs) checked against the
worktree source at `888384a`. Method: symbol/phrase existence per crate
(`grep`), then reading the code behind each claim that names a rule rather than
a symbol. Static reading only; no runtime evidence is claimed here.

| Crate | Claims checked | Result | Change |
|---|---|---|---|
| supervisor | entry points and `run_production → run_daemon → run_daemon_inner` chain; `execute_outcome`/`continue_task`; sweep through `ensure_action_at` with `durable_wait_until` (engine); `merge_turn_memory`; `locked_compact`; allowed `mcp__*` tools (`LaunchBindings::bind` checks the effective names including the dynamic catalog; `mcp_failure_is_required`); helper cwd `HelperPath { root: workspace, path: "" }`; secret policy on pending states and continuation responses (`production_tool_control`); dependency admission after a locked approval | all hold | added the compaction v5 shadow to the `locked_compact` paragraph and the `oauth` binding exchange |
| worker | entry chain `main → run → load_profiles / LockedLedger::open → run_decision → run_provider_turn(_inner)`; `exchange_tool_control` pub-in-binary; admission wait `1 << min(n, 8)` s clamped 1–30 s (`PROVIDER_ADMISSION_FLOOR/CEILING`); `state{provider_admission}.next_attempt_at`; `poll_after` parking; version gate exit 76 (`EX_PROTOCOL`, `StoreError::VersionGate` is a protocol failure); toolchain PATH from frozen `toolchain_roots`; subagent `report`; adoption call-identity checks | all hold; one wording | `NativeDeferredMode` is the provider capability type reached through `native_deferred_tools()` (reworded); added compaction v5 shadow and Brief V2 source table |
| engine | entry points; `dispatch_with_pre_mutation` chain; `ToolPipeline::has_causal_offer` (lives in `tools`, shared as stated); `ensure_action_at` / `durable_wait_until`; `first_post_compact_attempt` in `compact_gate`; `read_brief_v2`; `admit_shadow_artifact` | one inaccuracy | `automatic_memory_merge` memorizes every unconsumed candidate of the thread in the log, not only the turn's own (reworded) |
| provider | entry points; `prepare_with_native_deferred_tools`, `native_deferred_tools`, empty `tool_search_output` replay; `wholesale_limited` 402 → `RateLimited`; `Usage.cache_miss`; Kimi `choices[0].usage` fallback without aggregation; `ToolCallReady`/`mark_call_ready`; `arguments_complete` (worker-control) and `argumentsComplete`/`assistantFrameId` (endpoint); `uniqueItems`; `SecretMutationAuthority`, `OAuthTokenExchange` | all hold | none |
| mcp | entry points; `taskSupport`, `call_tool_augmented`, fixture `task-augmented`; SEP-1686 adoption (`ttlMs`, `pollIntervalMs`, `tasks/result`); `-32600` textual downgrade evidence; `parameter_headers` (`mcp-param-*`), `subscription` (`subscriptions/listen` correlation filter) | all hold | none |

Second pass (same day): endpoint, tools, profile, store, schema.

| Crate | Claims checked | Result | Change |
|---|---|---|---|
| endpoint | entry points; unary/worker-event paths; `tests/legacy_replay.rs` + `TEKES_LEGACY_REPLAY_EXPORT`; `NativeEndpoint::author_keyed_with_ledger` requiring `origin.target == session` (`service.rs`) | all hold | none |
| tools | entry points; `HelperClient::sandboxed_with_scratch` (0700 scratch shared by clones, worker exports it as `TMPDIR`); toolchain roots as read-only bin paths; `BackendTerminal::Pending`, `append_continuation_step`, `complete_continuation` | all hold | added `fixed_schema_for`/Brief V2 selection and `summary_artifact` seq evidence refs |
| profile | entry points; toolchain roots canonicalized/sorted/deduplicated and never writable (`config.rs` resolution + snapshot check) | all hold | added `brief_schema_version`, `compaction_shadow` validation and the 4096-byte description cap |
| store | entry points; reader/writer minima (`WRITER_VERSION = 1`, `VersionGate` before append and on checkpoint replay/tail repair); thread creation and rewrite gate through `LockedLedger::open`/`scan_valid_prefix_seeded`/`validate_ledger` at reader 1 (the same gate, reached through the scan rather than a separate check); `RewriteKind::{Fork, Redact}`; auto-compaction as worker checkpoint + `compact` (no rewrite); `append_keyed_with_ledger_if` | all hold | none |
| schema | entry points; `push` order (seq, reader gate, historical references, turn allocation, kind relations, supersedes, origin); `durable_wait_until` = max of the two waits; `terminal_tail` caveat | all hold | added `compact.shadow` validation |

Third pass (same day): the remaining eleven crates.

| Crate | Claims checked | Result |
|---|---|---|
| transport | entry points; `router → unary_inner → EndpointCarrierHost::unary`, `remote_mux_loop`; `/api/remote.mux` vs `/web/remote.mux` | all hold |
| selector | entry points; `main → parse_args/run_command → Selector`; crash-loop disposition and `rollback`; no Cargo dependency on supervisor/worker | all hold |
| product-installer | `Arguments`/`Operation`/`execute`; lib + bin + `build.rs` targets; `migration` | all hold |
| plugins | `PluginStore`, `PluginArchive`, `Manifest`, `ComponentProjection`, `SignaturePolicy` | all hold |
| worker-control | `Hello`/`SupervisorMessage`/`WorkerMessage`/`Frame`/`encode_line`/`v2::ToolControl`; `Frame.ledger_seq`; `tool_control_result` `pending` arm (`PendingContinuation`) and `tool_continuation(_result)`; supervisor `main` still negotiates v1 (`negotiate(&hello, 1, 1)`) | all hold |
| conformance | one-line `lib.rs`; gates under `tests/` | holds |
| deployment-tests | `workspace_root`, `run_fixture_checker`, `configured_binary` | all hold |
| safepoint | capture is `git add -A -- .` into a private index; nested repositories rejected (`SafepointError::NestedRepository`); `begin_tool_mutation` lease flow | all hold |
| schedule | `ScheduleAuthority`, `CronSchedule`, `ScheduleDefinition`, `LaunchClaim`; `poll_due`/`recover`/`record_status` | all hold |
| thread-search | `ThreadSearchAuthority`, `SearchRequest`, `SearchResult`, `ArchiveVisibility`, cursors | all hold |
| test-support | `FixtureRoot`, `FixtureError`; `publish = false` | all hold |

Every crate README has now been read claim by claim against the worktree at
`4d62dc3`; two wordings were corrected in total (engine memory merge, worker
native deferred routing), and the 2026-09-05 behaviors were added where a crate
implements them. Not covered here: any claim about runtime outcomes, which the
live inventory carries.

## Narrative docs pass (2026-09-05, night): flows → runtime → data

Same method over `docs/flows`, `docs/runtime`, `docs/data`, `docs/concepts`
and `docs/interfaces` (2646 lines). Spec-normative statements were taken as
owned by their gates; every statement that names a code behavior was checked.

| Document | Finding | Resolution |
|---|---|---|
| runtime/worker.md §Budgets | D-39 preflight compaction ("before each send … estimate against the config threshold") was not implemented: auto-compaction fired only on a provider context-overflow error and `compact_trigger_tokens` was validated but never consulted (config-v1, provider-adapter and the decision log all promised it) | **implemented** in the worker (`preflight_compaction_due`: the prepared candidate's byte size — provider-runtime-v1's one-token-per-byte `candidate_tokens` bound — vs `compact_trigger_tokens`, after the credential lease and before the admission lease, shadow request under the same lease, one compaction per turn); unit test `preflight_compaction_precedes_the_send_when_the_candidate_exceeds_the_trigger`; doc rewritten to the implemented rule |
| runtime/worker.md §Signals | `SIGTERM` row described a graceful settle path; the worker installs no signal handler and the supervisor's `terminate_worker` sends SIGTERM then SIGKILL after one second | row split: the graceful path is the `stop` control message; SIGTERM behaves as a crash |
| runtime/worker.md | `run_start {run_id …}` and `tool_result {outcome: aborted_by_crash}` | field names aligned to the ledger (`run`, `{aborted: "crash"}`) |
| runtime/tools.md, runtime/supervisor.md | "stdio MCP servers are ordinary child processes of the worker / workers own those directly", "daemon socket … unix socket" | every MCP peer is supervisor-owned and reached through `supervisor_control` (mcp-runtime-v1; the worker does not link `mcp`); rewritten |
| runtime/supervisor.md §Sweep, data/storage.md | asset GC with a grace period and the `orphans/` quarantine move are described as running | marked as designed but not implemented (`gc_rewrite_debris` is the only collector; orphan child files are left in place) |
| flows/turn.md | "Fifteen action types" (review-era count) | named the current action enums; added the shadow/preflight clauses |
| data/context.md | `aborted_by_crash` shape | ledger shape |
| flows/delegation.md, data/events.md, data/durability.md, data/idempotency.md, data/storage.md (rest), concepts/thread.md, interfaces/client.md, interfaces/web.md | durable vocabulary (`delegation` state, `spawn_id`, `{"bounded":3}`, `validation-<seq>`, `validator_for`, `resolved_inputs`, `input_sources`), retry budget 3, admission backoff 1 s doubling to 30 s, exit 76 = `EX_PROTOCOL`, five-strike restart backoff to 8 s, `O_CLOEXEC`, `F_FULLFSYNC`, diagnostics reserve, `LOCK_NB`, ports 7347/7357, `--web-listen`, web manifest resource, 14 V3 unary routes, storage names (`control-receipts`, `.thread-search-v1.json`, `endpoint-requests-v2`, `.create-staging`, `.rewrite-trash`, `.thread-catalog.lock`, `credential-state`, `tool-state/artifact-versions`, `goals/`, `memory/`, `endpoint-management-v2/`, `staging/`, `cache/`), `.agents/` with `.agent/` fallback | all hold |

Not verified by reading: the parts of these documents that restate spec
invariants (tail-lifecycle predicates, event-v1 constraints, checkpoint and
rewrite phase machines); their evidence is the conformance and live gates.

## Spec invariants pass (2026-09-05, night): all 36 specs

Method: (1) every closed-vocabulary token in every spec (snake/kebab/dotted
identifiers in backticks, 1,300 tokens) checked for presence in the source
tree under its own, CamelCase and camelCase spellings; (2) every numeric bound
in the specs checked against the code constant; (3) the cross-event
constraints of event-v1 and the state table, action matrix and invariants of
tail-lifecycle mapped rule by rule to the validator and lifecycle code.

| Area | Checked | Result |
|---|---|---|
| event-v1 constraints 1–9 | `schema::fold`: one final settle, turn-bound-after-settle, attempt chain (`attempt id reused`, dispatched/recovery ordering, usage-then-outcome exactly once), call/spawn/approval pairing, `admits` model-visible + eligibility + backward, origin-tuple mapping, backward references, turn allocation/prefix-completeness/queue_edit consumption, `recovery_ordinal` | all enforced in one fold |
| event-v1 visibility defaults, tail framing | `EventKind::default_visibility` (model / dialect-dependent / runtime / never); `store::tail` longest valid prefix with the lock holder truncating (`set_len`) | hold |
| tail-lifecycle | states 1–7 in that order (`engine::lifecycle::classify`), `LOCK_EX \| LOCK_NB` with EWOULDBLOCK/EAGAIN as lock loss, archive allowed only for `settled`/`parked_hold` (`archive_action`), `answered_hold` ordinary resume, `stop-active`/`archived`/`session-running` typed rejections, backoff 1/2/4/8 s five strikes; predicate names (`lock_held_by_caller`, `resume_permits`) are spec prose for `held_by_caller` / `resume_policy.permits` | hold |
| checkpoint-v1, rewrite-publication-v1 | digest preimages, `key_floor` bounds; phases `prepared→built→validated→published(→source_retired→source_removed)`, closed = record removed, `staging/*/op.json` sweep | hold |
| worker-control v1/v2 | every table message name is a codec name; `Frame.ledger_seq`; `tool_control(_result)`, `tool_continuation(_result)`, `queue_transaction(_result)`, `launch_child` | hold |
| provider-runtime-v1 | frame 8 MiB, terminal 64 MiB, finish reasons, closure rules; `candidate_tokens` = body bytes (D-39, now consumed by the worker preflight) | **fixed**: the failure union and the status table omitted `rate_limited` (HTTP 429 and the Cloudflare 402 wholesale limit are `rate_limited` → `error{rate_limit}` + durable admission wait, per event-v1 and the code) |
| instruction-snapshot-v1 | typed error classes | **fixed**: the spec listed `invalid_utf8`/`invalid_settings`; the crate's closed set is `ProfileError` (`invalid_bytes`, `invalid_schema`, `invalid_reference`, `stale_revision`, `store`, …) — spec now names it |
| mcp-runtime-v1 | 15 s handshake, 30 s catalog/resource, 10 min tool/task, 4 MiB frame, 16 KiB error body, 4096-byte descriptions | **fixed wording**: an untrusted project entry is omitted from resolution (`registry.resolve`), not launched, not advertised, and fails reconnect when trust is revoked; the spec's `blocked_by_project_trust` label was never a surfaced code |
| config-v1 rules 1–7 | folders/legacy cwd, canonical writable roots inside a cwd, unique ids, `compact_trigger_tokens < context_window_tokens`, dialect-profile acceptance, stdio/http shape, `credential_env` names, enabled references, sorted-unique lists | hold (`profile::config` validation messages) |
| builtin-tools-v1, tool-runtime-v1 | 27 names (fixture oracle), read scan 32 MiB, shell 120 s default / 600 s ceiling, `job` actions, `skill_explorer` limit 1–10; the computer-use table documents the external reference server's tools, not Kernel schemas | hold |
| secret-store-v1, safepoint-v1, schedule-v1, thread-search-v1, session-endpoint-v3 (14 unary methods), endpoint-*-v2, deployment-v1, plugin-package-v1, skill-package-v1, command-catalog-v1, dynamic-catalog-discovery-v1, exec-helper-v1, sandbox-profile-v1, credential-broker, launch-bindings-v1, tool-hook-v1, client-extensions-v1, web-tools-v1, web-search-provider-v1, provider-adapter, provider-dialect-profiles-v1, worker-control | vocabulary and bounds | hold (error-class names appear as CamelCase enum variants; `index_diagnostics` is the internal hit/missing/stale/corrupt counter, not a DTO, as the spec says) |

Not verified: prose that restates a gate's behavior in words (the gates are the
evidence), and vocabulary that only appears in fixtures the gates load.
