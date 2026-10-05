# Simplification program (2026-09-05, Frank's rulings on the redundancy list)

[Alignment audit record](crate-readme-semantic-audit-2026-09-05.md)

Frank's rulings on the twelve "possibly redundant" items, the design each
ruling resolves to, the DSH reference consulted where the ruling left room,
the blast radius, and the execution order. General rule from the rulings:
**no version numbers in names** (files, specs, protocols, tools) — git is the
version history; a compatibility integer may live *inside* a header or
handshake (DSH keeps `version` inside the session file header and nowhere in
names). Undecidable points are listed at the end for joint review.

| # | Item | Ruling | Resolution |
|---|---|---|---|
| 1 | second durable endpoint carrier | agree; keep one `endpoint.jsonl`, no version suffix | journal = projection of the ledger only (no chunk persistence; chunks stay transient); `endpoint-requests-*.jsonl` removed (holds come from the ledger); files renamed without version suffix, old names migrated on open |
| 2 | two leases per attempt | merge | one attempt-keyed admission handshake; the credential material is delivered on the credential descriptor when admission is granted and dropped at `attempt_settled`; the broker's separate lease id and revocation-monitor thread go away (DSH: resolve once per request, rotation reaches the next request) |
| 3 | `usage` as a standalone event | redesign under the thread structure | `usage` becomes a field of the settling `output`/`error` (`usage: {availability, figures…}`); constraint 2's usage-adjacency rules and the orphan-usage machinery disappear; readers (endpoint projection, `usage.summary`/`cacheAttribution`, rewrite) read the field |
| 4 | `dynamic-helper-v1` local-tool lane | remove | integration `protocol:"dynamic-helper-v1"`, `supervisor::dynamic_bindings`, the discovery spec and fixtures go; local tools are MCP stdio servers |
| 5 | three-way compaction summary | propose the best approach | DSH: one model summary per compaction, logged with the call, tool-result pruning first, `pressure`/`context-overflow` triggers. Kernel: model summary (today's `promote`) becomes the only summary; `settings.compaction_shadow` and `shadow` mode removed; the deterministic quoted history survives only as the fallback when the summary request fails; triggers stay preflight (pressure) + overflow |
| 6 | Brief V1/V2 | drop V1, no version field | one `brief` schema (the strict atoms/sources shape), `schema_version` removed from the tool, `policy.brief_schema_version` removed, source alias table always rendered |
| 7 | worker-control v1 | delete; if anything stays, no version numbers | the supervisor `main` stdin shell and the v1 message set go; one `worker-control` protocol (today's v2 content) with the hello negotiation kept as a compatibility integer; `worker-control-v1.md`+`v2.md` → `worker-control.md` |
| 8 | Safepoint shadow git | delete if it should be | DSH has no file snapshots (relies on the user's VCS) → delete the crate, the pre-mutation seam, the `safepoints.*` client routes, `workspaces/<id>/safepoints/`, spec and fixtures; the Tekes client does not call these routes |
| 9 | accelerated checkpoints | delete if unused | measured: the largest real ledger folds in 0.11 ms (10 events; ≈10 µs/event). Delete the acceleration (state asset, digest preimages, seeded replay, `checkpoint-v1`); keep the plain `checkpoint` marker (summary, coverage, `key_floor`) that compaction and origin-key retention rely on. DSH keeps acceleration only as an optional projection cache outside the log |
| 10 | post-turn memory merge | notes are recorded any time; memorize runs only when notes exist | remove the extraction model request and `settings.memory_merge`; at every completed root turn, staged `note` candidates of the thread that are still unconsumed are memorized deterministically (no model call); nothing runs when no candidate exists |
| 11 | four endpoint specs, one boundary | explain, merge, optimize code, no version numbers | `session-endpoint.md` absorbs projection/transport/management (sections: routes and streams, journal projection, transport listener, management authority and recovery); `endpoint::v3` module and `endpoint-management-v2/` root directory renamed without version; the wire `protocolVersion` integer stays inside the handshake |
| 12 | duplicate title caches; `note` vs `state` | merge | thread-search reads the supervisor's `manifest.json` instead of keeping `.thread-search-v1.json` + lock; `note` is dropped from the core kind table (unknown kinds already render generically; `state` subkinds are the one extension mechanism) |

## Order

6 → 10 → 5 → 12 → 4 → 8 → 7 → 2 → 3 → 9 → 1+11. Small and self-contained
first; the schema-shape changes (3, 9) and the carrier/spec consolidation
(1+11) last because they touch fixtures, gates and on-disk layout together.
Each item: implement, workspace regression green, live proof where the item
has one, commit, push.

## Progress

| # | Status | Evidence |
|---|---|---|
| 6 | **done** (2026-09-05) | One `brief` schema in the registry (`atoms/goal/completion`, no `schema_version`); `fixed_schema_for`/`validate_fixed_arguments_for`/`brief_v2_schema`, `CatalogContext.brief_schema_version`, `policy.brief_schema_version` and `EffectivePolicy.brief_schema_version` removed; `read_brief_v2` → `read_brief`, `BriefContractV2` → `BriefContract`; the worker renders the source alias table whenever a turn admits inputs and `brief` is callable (a ledger written before the single schema still contributes its `canonical_user_goal` as an antecedent). Workspace regression: 116 targets, 767 passed, 0 failed, 46 ignored (live gates). Scripted text flow passed. Live `run-public-flow.py --live --case text --brief-sources` passed on the Cloudflare OpenAI route (turn-2 brief: intent + constraint citing `s1`, two facts citing `s2`, goal over all four, explicit completion empty, effective completion `a1,a2`). |
| 10 | **done** (2026-09-05) | The model-extraction request, its system prompt, `settings.memory_merge` and the memory-merge catalog role (`role_memory_merge`, `CatalogRole::MemoryMerge`, the model-visible `memorize` tool and its schema rules) are gone; `note` candidates carry `key, kind, fact, why, scope` (no `commit`/`supersedes`); after every root turn that settles `completed` the supervisor memorizes the thread's unconsumed candidates in one host-side merge (`memory-merge-<turn>-memorize`, `WorkflowBackend::memorize` is host-only), writes nothing when there is none. Fixed catalog is 26 tools. Workspace regression: 115 targets, 766 passed, 0 failed, 46 ignored (live gates). Live `run-public-flow.py --live --case text --memory` passed on the Cloudflare OpenAI route: the model called `note` (key `single_lowercase_word_no_punctuation`, scope global) and the host merged it after the turn with no model request. |
| 5 | **done** (2026-09-06) | One compaction summary: the compactor-role `summary_artifact` request runs for every auto-compaction (preflight or overflow, inside the attempt lease) and every supervisor-authored manual compact; an admitted continuation is the compact `summary`, the deterministic quoted history is the fallback when the request fails, the artifact is rejected or the plan moved. Removed: `settings.compaction_shadow`, `ShadowMode` (off/shadow/promote), `run-compaction-shadow.py` A/B wrapper, the profile setting test. Renamed: `engine::compaction_shadow` → `compaction_summary` (`CompactionSummary`, `admit_summary_artifact`, `SUMMARY_SYSTEM`), `provider::prepare_summary_request`/`summary_completion_artifact`, worker `run_compaction_summary`, supervisor `summary_for_manual_compaction`, event field `compact.shadow` → `compact.summary_request` (no `mode`, no duplicated continuation), runner flag `--compaction`. Workspace regression: 114 targets, 766 passed, 0 failed, 46 ignored (live gates). Live `run-public-flow.py --live --case text --compaction` passed on the Cloudflare OpenAI route: artifact accepted (8 evidence refs inside a 47 KB bundle), 366-byte model summary written, 4/4 private codes recalled after the manual compact. |
| 12 | **done** (2026-09-06) | Correction to the resolution: the "supervisor-owned `manifest.json`" existed only in `docs/data/storage.md`, never in code, so there was one real title cache. Kept that one (the thread-search projection) and removed the phantom from the docs; the cache file is now `.thread-search.json` (no version suffix; a legacy `.thread-search-v1.json` is deleted on sight, the cache is rebuildable). `note` left the core kind table (`EventKind::Note` removed from schema, store barrier list and rewrite materialization): it is an ordinary extension kind — runtime visibility, not turn-bound, generic rendering — and event-v1 says so; the turn-after-settle invalid fixture now uses a `state` event. Workspace regression: 115 targets, 765 passed, 0 failed, 46 ignored (live gates); one timing flake (`capacity_release_starts_a_pending_queue_transaction_on_a_settled_tail`, worker-admission timeout under full load) passed 3/3 alone and on the full rerun. No live gate applies. |
| 4 | **done** (2026-09-06) | The `dynamic-helper-v1` lane is gone: `Integration.protocol` and its validation, `supervisor::dynamic_bindings` discovery (`production_helper_discovery`, `HelperDynamicCatalogDiscovery`, catalog decoding), `engine::DynamicHelperBackend`, `DynamicToolBackend`/`DynamicExecBinding`/`DynamicTool.exec`, `DynamicToolSourceKind::External`, the discovery spec and fixtures, the three helper scripts. Every dynamic tool is a plugin or MCP tool executed through supervisor control; `resolve_worker_launch_bindings` binds the MCP catalog only. Local tools are MCP stdio servers: the fixture MCP server gained a `local-tools` scenario (uat_marker, get_shipping_eta, record_left, record_right) and the deferred-tool and parallel-tools live gates run against it as `mcp__local__*`. Workspace regression: 113 targets, 761 passed, 0 failed, 46 ignored (live gates). Live: `run-live-process.py --scenario tool-search` (two turns, search → deferred MCP tool, exact MCP result envelope) and `run-public-flow.py --live --parallel-tools` (eager parallel fan-in audit) passed on the Cloudflare OpenAI route. Leftovers recorded below: `config/integrations.json` now has no consumer; `scripts/check-client-extension-fixtures.py` reports a pre-existing "twenty-method base" drift. |
| 8 | **done** (2026-09-06) | The `safepoint` crate, `spec/safepoint-v1.md`, the engine pre-mutation seam (`PreMutationAuthority`, `dispatch_with_pre_mutation`, `workspace_mutating`), the worker `WorkerSafepoint`, the `safepoint.v1` client capability and its three routes/DTOs/error classes, the `workspaces/<id>/safepoints/` directory creation and installer migration entry, the safepoint fixtures, `check-safepoint-fixtures.py` and Gates 97–100 are deleted (gate numbers retained as retired). The workspace quiescence lock stays: workspace relocation still takes it exclusively. Client extensions are now nine capability groups / 40 methods; the legacy `safepoint/list|restore` disposition is `retired`. The Tekes client referenced safepoints only through the legacy AppServer methods, never the Kernel routes. Workspace regression: 112 targets, 751 passed, 0 failed, 46 ignored (live gates). No live gate applies. |
| 7 | **done** (2026-09-06) | One `worker-control` protocol: the version is the `hello`/`selected` integer (2) and nothing else carries a version — `worker_control::v2` became `worker_control::durable` (re-exported at the crate root; `DurableControlError`, `require_version`), `MIN_PROTOCOL_VERSION`/`hello()`/`require_v2` and every `version >= 2` gate are gone, the supervisor `main` stdin shell and its tests are deleted (`tekes-supervisor` without `--install-root` prints usage), `spec/worker-control-v1.md` + `v2.md` merged into `spec/worker-control.md` (all cross-references relinked), `fixtures/wire-v2/` moved to `fixtures/wire/durable/` (the `v1-fallback` and duplicate negotiation cases dropped; base `hello-ok`/`hello-disjoint` at version 2/3), `check-wire-v2-fixtures.py` → `check-worker-control-fixtures.py`. The deployment authority registry now names `spec/worker-control.md`, so its digest and the bundle-manifest digest chain were re-pinned (daemon/selector constants, deployment fixtures, packaging probe, Swift UAT runner). Workspace regression: 112 targets, 750 passed, 0 failed, 46 ignored (live gates); deployment and worker-control fixture checks pass. No live gate applies. |
| 2 | **done** (2026-09-06) | The credential channel is request/response only: one `credential_get` keyed by the attempt (or `tool:<call-id>`) answers `credential {request_id, material, generation}` or `credential_error`; there is no lease id, no `credential_release`, no `credential_revoked` push, no revocation-monitor thread. Removed: `CredentialRelease`, `CredentialRevoked`, `CredentialMonitor`, `CredentialClient::{release, monitor_revocation}`, the broker's `leases`/`next_lease` maps and `BrokerError::UnknownLease`, the `revoked` flag threaded through the Tavily transport, and the `credential_revoked` classification of a cancelled provider request (a cancel with no recorded cause is now `transport`/`cancelled`, recoverable). Renamed: `CredentialLease` → `CredentialMaterial` (zeroed on drop; dropped where the attempt settles), `BrokerDecision::Lease` → `Material`. `CredentialBrokerControl::{rotate, revoke}` still drive the supervisor's authority-refresh propagation, and (DSH: resolve per request) they reach the next `credential_get` — an answered request keeps what it holds until it settles. Deviation from the resolution column: the admission `lease_request`/`lease` handshake stays on the supervisor control pipe, because material may only travel on the private descriptor; the merge is that one attempt id keys both and the second identity (the broker lease id) is gone. `spec/credential-broker-v1.md` → `spec/credential-broker.md` (cross-references relinked; the `tekes-credential-v1\0` request-id hash preimage is unchanged and documented as a constant, not a name); `fixtures/credential-broker/` corpus rewritten to 10 cases (`revoked-after-dispatch` dropped, `rotation` = second get returns the new generation, `success` = one get); gate 56 and gate 112 updated (`web_search_revocation_reaches_the_next_call`). Workspace regression: 112 targets, 747 passed, 0 failed, 46 ignored (live gates); web-tools, secret-store and web-search-provider fixture checks pass. No live gate applies (rotation/revocation are unit-exercised over a real socket pair). |
| 9 | **done** (2026-09-06; taken before item 3 so the usage change does not rebuild seeded checkpoint fixtures twice) | Checkpoint acceleration is gone: `checkpoint` is `{covers, summary}` — a plain covering marker (the boundary a later `compact` may cover), always a barrier, never a replay seed. Removed: `spec/checkpoint-v1.md`, `schema::checkpoint` (`CheckpointStateV1` and its record types, `CheckpointStateRef`, `checkpoint_state_ref`), `LedgerValidator::{checkpoint_state, restore_from_checkpoint}`, `CheckpointStatus`/`LedgerProjection::{checkpoints, key_floor}` and the checkpoint-invalidation and origin-key expiry (`key_floor`) folds, `store::scan_valid_prefix_seeded` (33 call sites now use `scan_valid_prefix`), `CheckpointReceipt`/`replay_checkpoint`, the state-asset publication in `create_checkpoint(timestamp, summary) -> seq`, `fixtures/checkpoint/` and its state asset, `events/checkpoint-state`, `invalid/checkpoint-keys-future`, the `checkpoint-assets` deployment authority (registry and bundle-manifest digest chain re-pinned across selector/daemon/packaging/UAT runner), gates 38–41 (retired, numbers kept), CI slice scripts. Older ledgers whose checkpoints still carry digests/keys/state stay valid: unknown payload fields are ignored, so no on-open migration is needed. Gate 42 is now `slice3_gate_42_checkpoint_marker_barrier`, gate 30 `slice1_gate_30_checkpoint_marker_replay`. Workspace regression: 112 targets, 744 passed, 0 failed, 46 ignored (live gates); deployment fixture check passes. No live gate applies. |
| 3 | **done** (2026-09-06) | `usage` is no longer an event: the `output` or `error{attempt}` that settles an attempt carries `usage: {availability, figures...}` (required on every attempt outcome, forbidden on a turn-level error) — the predecessor's placement (usage rides on the per-step assistant message). Removed: `EventKind::Usage`, `AttemptStage::Usage`, constraint 2's adjacency/orphan rules (`counted_usage_pairs`), the engine `OutcomePhase::UsageAppended`/`MissingUsage` step, the worker's `append_usage` (eight sites now build `usage_object(...)` into the outcome), the projection's usage map (the assistant message reads the output's `usage`), the `usage.summary`/`cacheAttribution` "effective pair" scan (reads outcome usage directly), the three `events/usage-*` fixtures; `Event::usage()` accessor added. Every fixture ledger and inline test ledger was rewritten by a transformer (usage line merged into its outcome, seqs renumbered, references remapped); the endpoint expected journal, torn corpus, invalid corpus expectations and the seven attempt-outcome event fixtures updated. Answer to Frank's question: responses are persisted (normalized content + sealed native fragments on `output`); rendered requests are not (only `attempt.wire_digest`; bytes only under `TEKES_KERNEL_LIVE_ARTIFACT`) — recorded below as undecided. Older ledgers with standalone `usage` events fold as unknown extension kinds and their outputs carry no usage; not migrated (UAT-era data only). Workspace regression: 113 targets, 744 passed, 0 failed, 46 ignored (live gates); endpoint-authority, web-tools, secret-store and deployment fixture checks pass. No live gate applies. |
| 1+11 | **done** (2026-09-06, four slices) | (a) done: the endpoint journal is `endpoint.jsonl` (+ `endpoint.lock`), a pure projection of the semantic ledger — provider chunks are transient (`session/transient` only) and never journaled; removed the journal's stream carrier records (`StreamCarrier`, `append_stream*`, `next_stream_frame`, per-attempt frame ordering, `StreamOrder`/`StreamForkAnchor`), the projector's chunk provenance on `assistant/message`, `Projector::append_stream_frame`, and the supervisor's background stream-persistence worker (`StreamPersistenceCommand`, `flush_stream_persistence`); the frame ordinal is an in-memory presentation counter. A journal under the earlier `endpoint-v2.jsonl` name is retired on open and rebuilt from the ledger (endpoint seqs reassigned; clients re-baseline). Gate 49 is now `chunks_are_transient_and_never_journaled`. Workspace regression: 112 targets, 753 passed, 0 failed (one worker-spawn timing flake `worker_early_eof_is_reaped_before_spawn_returns` passed alone), 46 ignored. Note: `scripts/check-endpoint-transport-fixtures.py` reports a pre-existing management spec/route mismatch (`workspace.relocate` registered but not in the spec table) — recorded below. (b) done: the request journal (`endpoint-requests-v2.jsonl`/`.lock`, `RequestJournal`, its records and `RespondLifecycle::commit`) is gone — an answerable request is an `approval_request` hold read from the semantic ledger (`endpoint::requests`: `PendingRequest::derive`, `RequestState::resolved`; child ids hash the `child-request` components), its resolution is the ledger's later `approval_response` for the call (or the aborting `tool_result`), and the carrier derives pending/resolved requests per line (`line_requests`/`session_requests`) for locate, author, actionable baselines and mux replay, publishing requested frames when a line's pending set changes; `RespondAuthority::locate` now returns the derived `RequestState`; legacy request files are retired on open. The `endpoint-request-journals` deployment authority is dropped (digest chain re-pinned); `management-request-journal` fixture became the ledger-derived `management-requests` oracle; the redact-carrier fixture models the projection journal only; `audit-public-task-approvals.py` and `task-approval-websocket-client.py` derive request ids from holds. Workspace regression: 114 targets, 753 passed, 0 failed, 46 ignored (live gates); deployment fixture check passes; the request oracle and redact-carrier fixtures pass the transport checker's validators (the checker itself still stops at the pre-existing `workspace.relocate` drift). (c) done: the root management directory is `endpoint-management/` (`store::endpoint_management_root` renames a legacy `endpoint-management-v2/` in place on first open; two populated layouts fail closed; daemon layout, installer migration list and the macOS UAT runner follow), and the public stream protocol module is `endpoint::mux` with no version in any name (`SessionSyncFrame`, `SessionStreamTarget`, `SessionMuxClientFrame`, `MuxHostDescription`, `MuxProtocolError`, `TEKES_UNARY_ROUTES`, `open_mux_stream`, `mux_respond_actionable`, … — 30 renamed symbols; the wire `protocolVersion: 3` integer is unchanged); test files `session_endpoint.rs`, fixture corpus `fixtures/endpoint-mux/`. Workspace regression: 114 targets, 753 passed, 0 failed, 46 ignored (live gates); deployment fixture check passes. (d) done: `spec/session-endpoint.md` absorbs `session-endpoint-v3.md`, `endpoint-projection-v2.md`, `endpoint-transport-v2.md` and `endpoint-management-v2.md` under the four sections the ruling named (Routes and streams, Journal projection, Transport listener, Management authority and recovery); the retired public routes stay listed only as carrier registrations; every cross-reference in spec/docs/README/crate docs/scripts is relinked (29 files), the two deployment authorities that named the old specs point at `session-endpoint.md` (digest chain re-pinned), the transport checker reads the merged registration table. Workspace regression: 112 targets, 752 passed, 0 failed, 46 ignored (live gates); deployment fixture check passes; the transport checker reads the merged registration table (still stopping at the pre-existing `workspace.relocate` drift). Item 1+11 is complete. |

## Recorded for joint review (not decided by the rulings)

1. **Persist rendered requests?** Today the ledger keeps the request digest
   (`attempt.wire_digest`) and the response's sealed native fragments; the
   request body is re-renderable and is written to disk only under
   `TEKES_KERNEL_LIVE_ARTIFACT`. DSH keeps the routed request durably (its
   token meter replays from it). Storing every request as an asset costs
   context-size × attempts per turn.
2. **Version suffixes on the other spec files** (`*-v1.md`): the rulings name
   endpoint, worker-control and brief; applying "no version numbers" to all 36
   specs is a link-churn decision.
3. **`control-receipts/` (D-70)**: exact-retry receipts for supervisor-side
   tool effects — kept under item 1 because they serve idempotency rule 3,
   not presentation.
4. **Tekes client Safepoint UI**: client-side only today; item 8 removes the
   Kernel routes the client never called.
5. **`config/integrations.json` after item 4**: the dynamic-helper lane was its
   only consumer; the file, `IntegrationsConfig` and the `integrations`
   revision in the RevisionVector are now inert configuration. Removing them
   touches config-v1, the config routes and the snapshot digest — proposed as
   part of the 1+11 consolidation.
6. **`scripts/check-endpoint-transport-fixtures.py`** reports "management
   spec/route registry mismatch extra=['workspace.relocate']" on the committed
   tree (the transport registers `workspace.relocate`; the management spec's
   registration table does not list it); pre-existing, not part of the
   workspace regression, to be folded into the item-11 spec merge.
7. **`session-endpoint-v2` origin `client` namespace**: the fixed origin-tuple
   client string carried by endpoint-authored ledger events, fixtures and
   checkers. It is durable dedup identity data, not a file/spec/protocol name,
   so it was left alone; renaming it would change retry identity for existing
   ledgers.
8. **`scripts/check-client-extension-fixtures.py`** fails on the committed tree
   with "the frozen twenty-method base drifted" (its frozen method list is
   behind the catalog); not part of the workspace regression, not touched.

## Rulings on the joint-review list (2026-09-06)

Frank ruled on every item above. Tekes-client leftovers (item 4) are out of
scope: this program only touches TekesKernel.

| # | ruling | status |
|---|---|---|
| 1 | persist rendered provider requests (the predecessor keeps the routed request) | **done** — the worker publishes the exact transmitted body (`PreparedRequest.body`, secret-free by the prepare contract) as a content-addressed thread asset before appending `attempt`, and `attempt.request = {asset, bytes}` references it (event §attempt; provider-runtime §Send ordering step 3). Optional in the schema so ledgers written before the field exist stay valid; a worker always writes it; shape is closed (`asset` + positive `bytes`). Rewrites drop `attempt` rows, so the asset never enters a fork/redact destination and a redact scan that finds forbidden bytes in it poisons it like any other asset. `TEKES_KERNEL_LIVE_ARTIFACT` capture is unchanged (it still captures responses). Tests: `attempt_request.rs` (schema), `provider_attempt_persists_the_exact_request_body_as_a_thread_asset` (worker), `events/attempt-request` fixture (corpus 86). Workspace regression: 113 targets, 756 passed, 0 failed, 46 ignored (live gates). |
| 2 | remove the `-v1` suffix from the remaining spec files | **done** — the 23 remaining `spec/*-v1.md` files are `spec/<name>.md` (builtin-tools, client-extensions, command-catalog, config, deployment, event, exec-helper, instruction-snapshot, launch-bindings, mcp-runtime, plugin-package, provider-dialect-profiles, provider-runtime, rewrite-publication, sandbox-profile, schedule, secret-store, skill-package, thread-search, tool-hook, tool-runtime, web-search-provider, web-tools); every reference in spec/docs/crate READMEs/scripts/packaging/fixture READMEs and code comments follows (147 files; `docs/history` and the audits keep their historical links, as with the earlier merges). Names that were also fixture file names lost the suffix (`fixtures/tools/builtin-tools*`, `first-party-tools`, `builtin-tool-schemas`, `builtin-tool-migration`, `fixtures/deployment/selector`), the fixed tool schema revision is `builtin-tools/schema-1`, the scheduler origin principal/client is `schedule`, the helper dependency label is `exec-helper`. Preimage constants (`tekes-provider-request-v1\0` …) and the legacy cache name `.thread-search-v1.json` are unchanged. The deployment authority registry names the new contract paths (digest chain re-pinned). Workspace regression: 113 targets, 755 passed, 0 failed, 46 ignored (live gates); all fixture checkers pass. Still versioned on-disk names, not covered by this ruling and each implying an on-open migration of installed data: `threads/<id>/session-settings-v1.json`, `credential-state/provider-secret-generations-v1.json`, `<storage>/schedules-v1.jsonl` + `.schedules-v1.lock`, the `tekes-kernel-product-installer-v1` wrapper name, the `tekes-client-resource-v1` origin client, the `idempotency-reconcile-v1` authority label, the `# tekes-endpoint-transport-v1` fixture magic — recorded for a ruling. |
| 3 | `control-receipts/`: merge | **done** — the two per-thread exact-retry carriers (`control-intents/` = request tuple before an external effect may begin, `control-receipts/` = tuple + exact response) are one record per request id at `control/<pp>/<request_id>.json` (`ControlRecord {request, response?}`; the intent is the record without a response, settling replaces it in place under one lock). Record bytes are unchanged, so a pre-merge thread folder is folded into `control/` under the lock on first use (receipt wins over a stale intent; test `a_pre_merge_receipt_layout_is_folded_into_the_control_directory_on_first_use`). The deployment authority is `tool-control-records` (digest chain re-pinned). The management operation carrier (`endpoint-management/operations/`, root-scoped, rpcId-keyed, multi-phase) and the MCP registry receipts stay separate: different scope, key and lifecycle. Workspace regression: 113 targets, 756 passed, 0 failed, 46 ignored (live gates); deployment and worker-control fixture checks pass. |
| 4 | Tekes client Safepoint UI: ignore | closed |
| 5 | delete `config/integrations.json` and its inert config surface | **done** — `Integration`, `IntegrationsConfig`, `validate_integrations`, `ConfigRepository::publish_integrations`, the snapshot member and its revision, the enabled-integration privilege-reduction comparison, the two config fixtures and the spec rules (integration transport shape, `credential_env`) are gone; config-v1 lists two global files. A snapshot published before the removal still decodes: `ConfigSnapshot`/`RevisionVector` read and ignore an `integrations` member (`legacy_integrations: ()`), never write it (unit test `a_snapshot_published_with_the_integrations_file_still_decodes`). Workspace regression: 114 targets, 755 passed, 0 failed, 46 ignored (live gates); deployment and secret-store fixture checks pass. |
| 6 | `workspace.relocate` drift: delete if unused | **done** — the route is served (`endpoint_host::relocate_workspace`) and the Tekes client calls it (`UserControlsView.relocateWorkspace`), so it stays; the transport checker's registration map, the closed error table (`workspace-busy`) and the per-route error table gained the route, and the `management/workspace-relocate` transcript triplet (success + `workspace-busy`) was added (54 registered cases). The checker passes on the tree again. |
| 7 | `session-endpoint-v2` origin `client` namespace: no version number | **done** — the namespace is `session-endpoint` (`endpoint::ORIGIN_CLIENT`); every emitter, checker, fixture ledger and the spec use it. Ledgers and operation records written before the rename carry `session-endpoint-v2`; `endpoint::is_endpoint_origin_client` accepts both when recovery re-checks a session-create genesis against its intent, and nothing writes the old value. Retry identity of pre-rename requests is untouched (their events keep their own tuple). |
| 8 | `check-client-extension-fixtures.py` twenty-method drift: merge/delete | **done** — the checker's frozen Python copy of the base method list is deleted; the catalog's `base` is bound to `TEKES_UNARY_ROUTES` + `remote.mux` by conformance gate 113, and the checker only keeps it closed, unique and disjoint from the extension methods. The three specs that still described a "frozen twenty-route Session Endpoint v2" base now say "Session Endpoint base registrations". |

Workspace regression after 6+7+8: 112 targets, 753 passed, 0 failed, 46
ignored (live gates); endpoint-transport, client-extension and worker-control
fixture checks pass.

## Version suffixes removed from every remaining name (2026-09-06)

Frank's ruling: remove the version number from all names. Done in three
commits, each with a green workspace regression.

**Durable file and directory names** (renamed in place on first open by
`store::retire_legacy_name`; two populated names fail closed):
`session-settings-v1.json` → `session-settings.json`,
`provider-secret-generations-v1.json` → `provider-secret-generations.json`,
`schedules-v1.jsonl` → `schedules.jsonl` (the stateless legacy lock file is
removed), `config-admin-v1/` → `config-admin/` (both names are now product
installer migration candidates).

**Identifiers, protocols and labels**: origin client `tekes-client-resource`,
authority label `idempotency-reconcile`, installer handshake
`tekes-kernel-product-installer` (the legacy string is still accepted from a
product assembled before the rename), UAT protocol
`tekes-kernel-production-uat`, sandbox backends `darwin-seatbelt` and
`linux-landlock-seccomp`, transport fixture magic `# tekes-endpoint-transport`.

**Rust item names**: `BuiltinManifest::compiled_v1` → `compiled`,
`BUILTIN_V1_DESCRIPTORS` → `BUILTIN_DESCRIPTORS`, `v3_journal_page` →
`journal_page`, the nine capability route tables `*_V1` → `*_METHODS`, and the
three test functions still prefixed with a protocol version.

### Deliberately unchanged, with the reason

- ~~Keychain service attributes~~ **done** (Frank ruled: do the real
  migration, no version in the service). `com.tekes.kernel.provider-secret`
  and `com.tekes.kernel.endpoint`. A record still under the superseded name is
  moved on the read that precedes any decision about it: the exact bytes are
  added under the current service, then the old item is deleted, and a failed
  move is an error rather than a silent fallback (the next read completes it).
  Provider secrets migrate in `provider::secret_store` (new `SecItemDelete`
  binding) and in the installer's `migrate_credential`; the loopback bearer
  migrates in the signed installer's `bearer_read`, and both the daemon's
  read-only lookup and the selector's `security(1)` adapter fall back to the
  superseded service. The generation floor accepts one authority recorded with
  the old service and republishes it under the new one (unit test
  `a_floor_recorded_under_the_superseded_keychain_service_is_accepted_once`).
  Uninstall deletes both names in the selector adapter and in the UAT runner.
  **Not yet proven on a real machine**: the migration paths are Security.framework
  and `security(1)` calls that no unit test can exercise. Gate 71-76 install UAT
  on a real machine is required before shipping this.
- **Hash preimage constants** (`tekes-provider-request-v1\0`,
  `tekes-credential-v1\0`, `tekes-mcp-continuation-v1\0`,
  `tekes-tool-continuation-v1\0`, `tekes-provider-query-v1\0`,
  `tekes-oauth-refresh-grant-v1`): byte constants inside a digest preimage, not
  names; changing one changes every derived id in existing data.
- **Negotiated Client capability ids** (`resources.v1`, `tools.v1`, `mcp.v1`,
  `plugins.v1`, `schedule.v1`, `threadSearch.v1`, `usage.v1`,
  `providerAdmin.v1`, `workspacePolicy.v1`): the integer is the negotiated
  capability version — the in-handshake compatibility integer the ruling
  explicitly allows.
- **Legacy names kept for migration** (`endpoint-v2.jsonl`,
  `endpoint-management-v2/`, `control-receipts/`, `control-intents/`,
  `.thread-search-v1.json`, `session-endpoint-v2`, and the new
  `*-v1` file constants above): they name data written before a rename.
- **External wire identities**: provider dialect ids (`openai_responses_v1`,
  `anthropic_messages_v1`, `tavily_v1`, …), model ids and evidence revisions.

## Keychain rename: what was proven on a real machine (2026-09-06)

The release UAT (gates 72/76) is runner-owned and could not run here: it needs a
distribution signing identity, matching supervisor/installer provisioning
profiles, a signed Client UAT app, a fresh dedicated user account and a real
hardware reboot. This machine has Apple Development identities only. What was
run instead, against this machine's own installed data:

- **Every durable rename, over a copy of the live `~/.agents`** (317 files):
  `endpoint-management-v2/` → `endpoint-management/`, `config-admin-v1/` →
  `config-admin/`, `provider-secret-generations-v1.json` → the current name
  (7 real generation floors carried over unchanged, service normalized),
  5 real `session-settings-v1.json` documents renamed with identical bytes and
  still decoding. Re-opening is a no-op. Every other file was byte-identical
  after the run.
- **The provider secret migration, against the real login keychain** under
  scratch names: staged under the superseded service, the production
  `keychain_read_migrating` returned the exact bytes, the record then existed
  only under the current service, a second read was an ordinary hit, and the
  probe removed what it created.

Two defects were found and fixed by that exercise:

1. `drive_select_model` read its staged candidate at the new name only, so a
   select-model operation staged before the rename and recovered after it could
   not complete. This machine has eight such payloads (all from completed
   operations, so none is stuck). The read now retires the superseded name;
   regression test `a_select_model_payload_staged_before_the_rename_is_read_on_recovery`
   fails when the fix is reverted.
2. The `security(1)` adapter cannot write a credential from a non-interactive
   context: `add-generic-password -w` takes the value on a controlling
   terminal, not stdin, so a write stores an empty password (verified twice on
   this machine). The migration first added to that adapter would therefore
   have emptied the bearer and deleted the original. That adapter now falls
   back read-only; the signed installer owns the move in-process.

The installer's two keychain paths are now one function,
`keychain::read_migrating`, with `protection` and the access group as
parameters instead of hardcoded values, and both `bearer_read` and
`migrate_credential` call it. That makes the move testable in ordinary Rust:
`a_credential_under_the_superseded_service_moves_and_answers` drives the
production function against the login keychain (file-based, no access group)
and the matching provider test does the same for `keychain_read_migrating`.
Both are `#[ignore]`d because they touch the developer's login keychain, use a
unique service pair per run and delete what they created from a `Drop` guard:
an item left by an earlier build would otherwise make the next run wait on a
keychain access prompt, which is exactly what happened once during this work
(a rebuilt test binary blocked for eighteen minutes on `SecurityAgent`).

What no unsigned process can reach, in any language: the same calls against the
*data-protection* keychain with a team-prefixed access group. That combination
needs the restricted `keychain-access-groups` entitlement, which macOS honours
only from an embedded provisioning profile. Measured on this machine: a
dev-signed binary carrying the entitlement without a profile is SIGKILLed at
launch, and the same binary without the entitlement gets
`errSecMissingEntitlement` (-34018) from `SecItemAdd`. The local signed-install
path that would supply a profile is suspended -- `Tekes/scripts/prepare-kernel-signing.py`
and `build-sibling-kernel-product.sh` are commented out in full, and this
machine holds no Kernel provisioning profile.

Still owed before release: gates 72/76 on a real machine, which is where the
entitled data-protection variant gets exercised.
