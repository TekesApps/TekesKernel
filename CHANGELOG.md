# Changelog

Notable changes per release. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow the
workspace version in `Cargo.toml`.

## Unreleased

## 0.5.0 — 2026-10-14

Toolchain moves to Rust 1.95.0. The MCP client additionally speaks the tasks
extension (SEP-2663) used by the official Rust SDK; every handshake now
declares it. No existing wire field changes meaning; SEP-1686 servers are
unaffected.

### Changed
- The pinned Rust toolchain moves from 1.85.1 to 1.95.0 (`rust-toolchain.toml`,
  workspace `rust-version`). Behavior is unchanged; the new compiler's lints are
  fixed in place (let chains for collapsible `if`s, a few rewrites, and
  `#[allow]`s with a stated reason where reshaping a public type or giving up a
  deliberate non-short-circuit would change behavior). `Cargo.lock` gains only
  the dev-dependency tree of the official Rust MCP SDK (see "Added").

### Added
- MCP tasks extension (SEP-2663, `io.modelcontextprotocol/tasks`), as implemented
  by the official Rust SDK `rmcp` 3.x, alongside the existing SEP-1686 support
  (no change for SEP-1686 servers). Contract: `spec/mcp-runtime.md` ("Tasks
  extension shapes").
  - Every MCP client handshake now declares
    `extensions["io.modelcontextprotocol/tasks"]: {}` (`initialize`
    capabilities, and the `clientCapabilities` request metadata of modern
    peers). A server that creates tasks only for declaring clients, such as
    `rmcp`, now returns them; before, such a server never created a task and
    answered `-32021` to task methods.
  - A flat `resultType:"task"` `tools/call` result becomes the Kernel creation
    shape `{"task":{...}}` and binds the durable continuation exactly like a
    SEP-1686 task. Flat `tasks/get` payloads (`ttlMs`, `pollIntervalMs`, inline
    `result`, `error`, `inputRequests`) are accepted; `tasks/result` is sent only
    to peers advertising the top-level `tasks` capability.
  - Task augmentation does not require `execution.taskSupport`: a peer that
    advertises only the extension has no request-side `task` member (the server
    decides per call), and any non-terminal task answer binds a continuation. A
    task from a peer that advertised no task capability fails closed
    (`unknown_effect`). `-32021` from a server that lost the declaration is a
    typed remote error and is not retried.
  - Fixtures `fixtures/mcp-runtime/tasks-extension-*.jsonl` (replayed byte for
    byte), and `crates/mcp/tests/rmcp_interop.rs`, which runs a real `rmcp`
    3.5.1 server (dev-dependency) over stdio in the legacy and modern protocols.

## 0.4.6 — 2026-10-13

Additive wire changes only (see "Wire" below); no existing field changes meaning.

### Fixed
- One MCP tool whose schema the dynamic-schema contract refuses (for example a
  `maxProperties` or `minProperties` keyword) no longer takes its whole server
  down. The tool is excluded alone; the server's other tools project, route and
  stay available, and the server stays `ready`. Before, preparation dropped the
  server (`mcp-server-projection-failed`), the policy naming its tools then
  failed, and the client saw only "Workspace policy exceeds an authority
  ceiling". Present since 0.2.0.
  - Per tool: a schema, description, projected name or external-effect
    declaration the launch validator refuses, and an effectful tool whose
    reconcile tool was refused. Per server (unchanged): spawn, handshake and
    `tools/list` failures, duplicate tool names in one listing, and route or
    pool collisions.
  - Each refusal is a bounded diagnostic (`server`, `tool`, `name`, `code`,
    JSON `pointer`, `keyword`, `reason`) in supervisor.log
    (`mcp-tool-projection-refused`), in an `mcp_tool` session notice per server
    at worker launch, and in `mcp.list`.
  - `workspace.policy.set` accepts a policy that names a refused tool, stores it
    as sent and reports it in `unavailableTools`; the tool is not in
    `tools/list`, `tools/resolve` answers `tool-not-found`, and it is not
    callable. An unknown name, an unavailable server and every ceiling are
    refused as before. The worker launch honours the same rule through the
    launch binding's optional `unavailable_tools`.
  - A refusal reports a projected `name` only when it is a valid launch tool name
    (at most 128 bytes, no ASCII control). A refused tool whose projected name is
    longer is an unknown tool at `workspace.policy.set` (`tool-not-installed`), and
    a refusal whose projected name is also an executable tool's (server `a` tool
    `b__c` against server `a__b` tool `c`) is not reported as unavailable. A policy
    that `set` accepts therefore always yields a valid launch binding, `tools/list`
    and worker launch.
  - The `mcp_tool` notice names at most eight skipped tools, those the workspace
    policy names first, and counts the rest.
  - A route reconnect re-projects the catalog per tool, so a sibling that stays
    refused is not reported as drift of an effectful route.
- `workspace.policy.set` failures say what failed. `policy-escalation` stays the
  code for everything that exceeds what is installed or granted, but `message`
  now differs by cause and `details.cause` is one of `authority-ceiling`,
  `tool-not-installed`, `mcp-server-unavailable`; `details.reason` is as
  precise as before and now lists the offending tool or server names. A policy
  that could not be judged because configuration, instructions or MCP
  preparation failed is reported as `internal` with
  `details.cause = "catalog-unavailable"` instead of as an escalation.

### Wire
- `workspace.policy.set` result: optional `unavailableTools` (at most 32 entries)
  and `unavailableToolCount`, present only when the policy names a refused tool.
- `workspace.policy.set` failure `details`: new `cause`.
- `mcp.list` row `readiness` for a ready server: optional `refusedToolCount` and
  `refusedTools` (at most 32 entries).
- Launch bindings v1: optional `unavailable_tools` (absent when empty, so bytes
  of a launch without refusals are unchanged). Host to worker only.
- New session notice classification `mcp_tool`.
- `mcp` crate: `project_catalog_partial`, `McpToolRefusal`; `project_catalog`
  keeps its all-or-nothing behavior. `profile` crate:
  `LaunchBindings::bind_with_unavailable`, `SchemaRefusal`.

### Known behavior changes
- Host and worker must come from the same release. An older worker refuses a
  launch whose bindings carry `unavailable_tools` (it rejects unknown fields);
  that only happens when a launch has refused tools, which 0.4.5 would have
  answered by dropping the whole server.
- An explicit empty `unavailable_tools` in launch bindings is rejected on decode
  (the host never emits it).
- A server whose only defect is a refused tool is now `ready` with zero or more
  tools instead of `refresh-failed`/`projection-failed`, and a policy naming
  only its refused tools is accepted instead of failing the whole policy.
- The `message` of `policy-escalation` is no longer always "Workspace policy
  exceeds an authority ceiling"; clients should match on `code` and
  `details.cause`.

## 0.4.5 — 2026-10-13

No wire-format change.

### Fixed
- A root could become unbootable: the endpoint exited with status 74 before
  ready, on every launch, with "workspace publication does not match prepared
  revision". The 30 s management-recovery tick staged a synthetic
  `workspace.rename` to repair workspace metadata; a `workspace.policy.set`
  published between that snapshot and its Carriers check, leaving the record
  stuck in `carriers`. `workspace.policy.set` racing `workspace.relocate`
  corrupted the same way. Present since 0.2.0.
  - The tick now heals workspace metadata in place under the management lock and
    stages no operation.
  - `workspace.policy.set` (and the replay of a journaled policy intent) publish
    under the management lock, held only around the publication. Lock order is
    management, then `config/.lock`.
  - A workspace operation that can never complete (`CorruptOperation`) is moved,
    with its payload, to `endpoint-management/quarantine/<digest>/` (`record.json`,
    `payload-root/`, `reason.txt`) and logged as `management-operation-quarantined`;
    recovery and boot continue. A record written by another record version (for
    example a newer Kernel's after a downgrade) is logged and left in place.
  - One failing record no longer stops recovery of the remaining records; the
    first failure is still returned afterwards.

### Known behavior changes
- `workspace.policy.set` can now wait behind a running workspace operation
  (create, rename, relocate, folder edit) for the management lock. If that
  operation changed the workspace meanwhile, the call fails as `stale-revision`.
- A root that is already stuck boots on its first 0.4.5 launch: the stuck record
  moves to `endpoint-management/quarantine/<digest>/` with its payload and
  reason, and boot continues. The interrupted operation (a synthetic rename) is
  not replayed; the workspace metadata is healed instead. No manual repair is
  needed.

## 0.4.4 — 2026-10-13

### Fixed
- Linux Landlock sandbox launches now require ABI 3 so truncation cannot modify
  files outside writable roots. Network-denied tools cannot redirect Unix
  socketpairs to outside endpoints, and seccomp refuses metadata mutations
  that Landlock does not mediate (also within writable roots).

### Added
- Linux bubblewrap/seccomp sandbox backend, requiring bubblewrap 0.4.0 or
  later and unprivileged namespaces but no Landlock support. Automatic
  selection probes bubblewrap first, falls back to Landlock when unavailable,
  and refuses execution when neither can confine the tool. The namespace
  backend permits filesystem metadata changes inside writable roots and
  isolates the network namespace for network-denied tools.
- Detached jobs authenticate their sandboxed runner and retain private host
  launch receipts for reconnecting and stopping. Bubblewrap jobs use verified
  pidfds to terminate the complete PID namespace. Running jobs from older
  versions without a private receipt fail verification after an upgrade;
  finish or stop them before upgrading.
- Native x86_64 and aarch64 Linux sandbox CI gates, including public job-launcher
  tests for filesystem and network isolation, descendants, and failed launches.
  Both backends are tested explicitly before automatic selection, preventing
  fallback from hiding a failed implementation.

## 0.4.3 — 2026-10-13

### Fixed
- DeepSeek Responses now accepts image inputs supported by its API. The dialect capability
  declaration previously rejected an image locally before the request could be sent. Historical
  images are also preserved when a later annotation or text input replays the conversation.
  Non-image file inputs remain unsupported by this dialect.
- This release builds on 0.4.2 and retains its steering, cancellation receipt, context projection
  replay, quarantine locking, and request publication fixes.

### Client compatibility
- Input-bubble image display is fixed separately in TekesClientKit: Kernel image asset references
  now resolve through the existing session-authorized attachment API, including historical inputs.


## 0.4.2 — 2026-10-13

### Fixed
- **Steering while a `subagent` or `task` runs no longer fails the turn (F146-D2).** A steer is
  delivered at the child-wait yield point, so it sat between the spawning call and its
  `child_result` and the provider-context projection rendered it between the `function_call` and
  its output (DeepSeek `http_400 ... No tool output found`; the turn failed and the steer was
  dropped). The projection now re-anchors a steer that falls inside an open call-to-result span
  to just after the last result of that batch (also for a mixed batch of a spawned call and
  ordinary calls); several steers keep their order and a steer outside a span keeps its place.
  The ledger is unchanged, so existing threads heal on their next request. Tests: the 0.4.1 fake
  DeepSeek pairing rule for `subagent`, `task`, a multi-call batch (the ordinary result before and
  after the steer), several steers, a later turn, and a control.
- **`session.cancel` no longer returns `internal` after a successful stop (D6).** When the worker
  had exited at the end of its turn, the stop write hit EPIPE and the route returned the I/O error,
  although the stop was durable. `cancel` now confirms the worker's exit (within a second, like the
  prompt, compact and respond paths) and returns the durable receipt of the locked author; a worker
  that is still running keeps the error. The failure message now carries the stage (`stop-write`,
  `receipt`, `cascade`) and the elapsed milliseconds, so `endpoint-route-error-folded` shows where a
  stop failed, and a `session-cancel-worker-exited` line records the fallback.
- **A thread reopened after a config change shows its new context usage (N1).** The journal
  snapshot labels its projections with `asOfSeq` clamped to the cut, which a client holding the newer
  control baseline does not take as an update. The open now follows the snapshot with
  `contextDetails` then `contextUsage` projection frames carrying their true clock sequence when it
  is beyond the cut (ClientKit 0.1.97 accepts and merges them; the client contract script decodes
  and validates them from a real process). `spec/session-endpoint.md` §session-journal is updated.
  This supersedes the 0.4.0 note that a reopened thread was not guaranteed to show a changed usage.
- **Quarantine decisions no longer rest on two unlocked reads 25 ms apart (O-final m3, n3).** A
  ledger whose unlocked read shows a complete invalid record is judged on a read under a
  non-blocking exclusive lock of the line: a free lock gives an authoritative read, a busy lock means
  a live holder owns the line and the decision is deferred. This covers the main-ledger fold, child
  lines and thread hydration.
- **A request frame refused at the 256-session limit is retried (O-final n4).** The published set is
  restored when the frame fails to go out, so the next reconcile announces it.

### Changed
- The actionables baseline no longer byte-reads unchanged child ledgers on every recompute (O-final
  n2): the prefilter answer is cached against the names and lengths of the child files.
  Threads whose child ledger holds a historical `approval_request` are still folded per recompute.

### Behaviour notes
- After a journal snapshot, clients now receive `contextDetails` and `contextUsage` projection
  frames at their true clock sequence when it is beyond the snapshot's cut (see N1 above); an older
  client treats them as ordinary live projection frames.

### Known issues
- A steer that a turn which failed under 0.4.0 or 0.4.1 already dropped from the model's context
  is not replayed later; only steers delivered with 0.4.2 are re-anchored (L1).
- A child hold written with no doorbell (a child worker dying between its durable append and its
  message) is still seen only through the child-file prefilter, after the next main-line fold (L2).
- Holding a ledger's lock to decide on damage can, for an instant, make a starting worker's
  non-blocking acquire fail with `Busy` (as `probe_line_lock` already does).
- The worker test `permission_mode_file_change_applies_to_the_next_tool_batch` remains
  environment-dependent (see 0.4.1).

## 0.4.1 — 2026-10-13

### Fixed
- **A completed `subagent`/`task` child no longer breaks the parent's next provider request.**
  The parent's durable `state{subkind:"delegation"}` record is model-visible by default and is
  written between the spawning `tool_call` and its `child_result`, so the provider-context
  projection rendered it as a user message between the call and its output. DeepSeek rejects
  that with `http_400 invalid_request_error: No tool output found for tool call <id>`, and the
  thread then failed every turn because each attempt re-rendered the same history. The
  projection now skips `delegation` state rows when it builds the parent's provider context.
  The child still receives its assignment from its seed snapshot (`seed_provider_items`), the
  ledger format and writer are unchanged, and threads already stuck on this shape recover on
  their next request because the context is rebuilt from the ledger. Present since 0.3.0.
  Tests: a fake DeepSeek pairing rule over the real spawn transaction for both `subagent` and
  `task`, a ledger already in the broken state, and an adjacency assertion in
  `production_task_creates_launches_waits_and_pairs_child_once`.

### Behaviour note
- The parent model no longer sees the host-resolved inputs (`resolved_inputs`) or any
  hook-rewritten effective invocation of a spawned `subagent`/`task` call, because the
  delegation record is no longer rendered in the parent's context. It keeps its own call
  arguments and the `child_result`. The child still receives its assignment, including the
  resolved inputs and the effective invocation, from its seed snapshot.

### Known issues
- If the user steers a queued message (the composer's Steer, or auto-steer) while a `subagent`
  or `task` runs, that turn can fail with the same `No tool output found for tool call <id>`
  400, because the steer input is rendered between the spawning call and its `child_result`.
  The thread recovers on the next turn, but the steer text is dropped from the model's context.
  This predates 0.4.1 (a separate cause from the delegation record) and is tracked for 0.4.2.
- The worker test `permission_mode_file_change_applies_to_the_next_tool_batch` depends on how the
  test binary is built: it fails under `cargo test -p tekes-worker` ("fixed tool is not
  available in this launch profile: shell"), also on v0.4.0, and passes in the
  `cargo test --workspace` run.

## 0.4.0 — 2026-10-12

### Changed
- **Lazy boot.** `ready` no longer reads any thread. The boot-time sweep that decoded every
  ledger and journal, repaired every tail, removed ephemeral threads and validated the
  endpoint journals is gone from the readiness path; on a 60-thread, 25 MB root `ready`
  drops from 7 s to about 1 s (Release) and from 47 s to about 4 s (Debug) and no longer
  grows with the number of threads (measured flat to 600). The 256-active-session check no
  longer runs at open: a root with more than 256 active folders boots, and every baseline
  lists all of them. A thread is verified (a torn tail repaired when no line lock is held,
  its ledger folded once, its journal projected) when something needs it: a baseline waits
  at most two seconds and lists the rest as placeholder rows followed by inventory upserts;
  opening a `session-journal` stream, answering a request and spawning a worker verify their
  thread first. Ephemeral-thread and tombstone cleanup, the schedule timer and a background
  card/upgrade pass run after `ready`. Sessions that existed before the process started stay
  deferred (no worker is started for them) until `sessions.recover`; a thread created, forked
  or unarchived by this process is admitted at once.
- **Inventory, workspace, control and actionables baselines** come from thread cards,
  persisted context projections and the request holds on the cards instead of folding ledgers.
- **All-session streams attach lazily.** A new `events.mux` generation attaches no session.
  A session is attached when it is created, forked or unarchived, or the first time a frame is
  published for it; `session/subscribed` is emitted at that attachment. Idle sessions are
  released least-recently-touched first beyond 256 attachments. Clients using only the
  `session-journal`, `session-control`, `actionables`, `session-inventory` and `workspace`
  streams are unaffected.
- Responding to a request finds it through an index (cards, then a byte prefilter of threads
  without a card and of the archive) and reads one thread. The actionables baseline lists pending
  requests of child lines too (verification folds the child lines; a doorbell fold marks them
  unverified and their bytes are prefiltered; live child requests come from the published set). Respond, actionables and
  registration reads use the valid prefix of a ledger instead of failing on a torn tail.
- A journal snapshot carries the thread's current `contextUsage`/`contextDetails` values, with
  `projections.asOfSeq` clamped to the snapshot's `throughSequence` (a label, not the context
  projection clock; ClientKit rejects a projection beyond the cut). The current value is not
  re-sent as a frame, and a client that already holds a higher sequence for the projection (from
  its control baseline) keeps its own value, so a reopened thread is not guaranteed to show a usage
  that changed while it was idle (no worse than before; it never shows a wrong value).
- `thread.search` skips a quarantined thread instead of failing the query; `usage` tolerates a
  torn tail; workspace-membership checks read the genesis line only; the management recovery
  retry runs every 30 s on its own thread and takes the management lock per record.

### Added
- `thread-card.json` in each active thread folder: a derived, content-checked summary of the
  verified ledger prefix (see `spec/session-endpoint.md` §Thread cards and lazy verification).
  Never authority; deleting every card is safe and an older Kernel ignores them. Written
  atomically by a paced background writer (8 fsyncs/s by default, `TEKES_KERNEL_CARD_MAX_PER_SEC`).
- Quarantine: a ledger with no valid genesis or a complete invalid record in the main or any
  child line keeps its inventory row; opening it fails with stream error
  `internal / stream-open-failed {"reason":"quarantined"}`, and it is never rewritten.
- `active-session-limit {limit:256}` as a stream error (`conflict`) when 256 pinned sessions
  already hold the live-event registrations.
- Diagnostics: `TEKES_KERNEL_BOOT_TRACE=1` (phase times and ledger/journal work counters on
  stderr), `TEKES_KERNEL_BOOT_PROGRESS=1` (opt-in `{"type":"boot"}` records before `ready`),
  `TEKES_KERNEL_LIST_COLD_WAIT_MS` (baseline verification budget, default 2000),
  `TEKES_KERNEL_CARD_DEBOUNCE_MS`, `TEKES_KERNEL_CARD_MAX_PER_SEC`.

### Removed
- `ManagementStore::open` / `open_at` from the public API (use `open_light` for requests and
  `open_at_with_queue_driver` for the recovering open at boot).
- The runtime provider route allow-list. The proof registry
  (`fixtures/provider-dialects/profiles.canonical.json`) is no longer compiled
  into the Kernel: a configured provider on a supported dialect runs with the
  endpoint, gateway and model it is given, and an upstream that rejects the
  route fails at request time. Credential headers come from the dialect, and
  the configured endpoint is no longer completed from a proved URL. Readiness
  drops `dialect-unproved` (an unknown dialect is `misconfigured`).
  `--models-available` now prints `[{dialect_id, protocol_family}]` instead of
  proof rows. Removed from the `provider` crate: `AdvertisedDialectProof`,
  `advertised_dialect_proofs`, `configured_route_is_verified`,
  `validate_endpoint`, and `ResolvedDialectProfile::{proof_verified,
  proved_endpoint}`; added `supported_dialects` and `validate_provider`.
  `scripts/add-provider-model-proof-alias.py` is removed.

## 0.3.0 — 2026-10-08

### Removed
- The launchd deployment. The host application launches the Kernel and
  supplies its endpoint token and credentials through the environment, as in
  [Application-owned launch](docs/builtin-launch.md). Removed: the `selector`,
  `product-installer` and `deployment-tests` crates, the signed release and UAT
  packaging, the deployment fixtures and `ci-slice10.sh`; and from
  `tekes-supervisor` the `--install-root` daemon mode, `support-bundle`,
  `--web-launch-url`, `--describe-web-client`, and the operational metrics,
  access log and support bundle that only the daemon wrote.
  `--describe-build` no longer reports `authority_registry_sha256`, and the
  storage preflight no longer creates `credential-state/`
  ([#28](https://github.com/TekesApps/TekesKernel/issues/28)).
- The `providerAdmin.v1` client-extension group (`providers.list`,
  `providers.verify`, `providers.connections`, `providers.connection.*` and
  `providers.profile*`). The launching application owns provider
  configuration; abandoned provider and settings operation records are ignored
  on recovery. The client-extension fixtures drop the group's catalog entry,
  method and negative cases, error codes and readiness value cases.

### Added
- `workspaceFolders.v1` client extension: `workspace.listFolders`,
  `workspace.addFolder` and `workspace.removeFolder` edit a workspace's
  ordered folder list through recoverable endpoint management operations.
  Removal refuses the last folder, a folder any session is bound to, and a
  running workspace; a second folder is refused while unbound legacy sessions
  exist. `workspace.create` now rejects a path that is already another
  workspace's folder. See [client extensions](spec/client-extensions.md#workspace-administration).
- Linux tool sandbox. The `linux-landlock-seccomp` backend now applies its
  plan: Landlock confines the filesystem to the policy roots and seccomp
  refuses sockets under `deny` network, process creation without
  `allow_process`, and namespace, mount, keyring, `io_uring` and
  terminal-injection interfaces. Shell, edit and job tools therefore run on
  Linux instead of being refused. `loopback` network is refused on Linux
  because neither mechanism can enforce it
  ([#25](https://github.com/TekesApps/TekesKernel/issues/25)).
- `web_listen` in the built-in launch document starts the browser Web Client on
  a loopback address and adds `webUrl` to the readiness record. It is off when
  omitted.

### Fixed
- Built-in launch now starts on Linux. The storage preflight required APFS on
  every platform; outside macOS it now relies on the local-filesystem probe.

## 0.2.2 — 2026-10-06

### Added
- Release builds record their Git commit. With `TEKES_SOURCE_REVISION` set at
  compile time, `tekes-supervisor --describe-build` adds `source_revision`.
  `build-signed-release.sh` requires `--source-revision` and rejects a
  supervisor that reports another commit; `scripts/ci-slice10.sh` refuses a
  tree with local changes ([#5](https://github.com/TekesApps/TekesKernel/issues/5)).
- `tekes-selector prune` removes published versions that are neither the
  current nor the previous selection, with their observations, so old
  development versions do not accumulate and a pruned version can be staged
  again ([#4](https://github.com/TekesApps/TekesKernel/issues/4)).

### Fixed
- The workspace-service `process_files` tests no longer fail intermittently on
  Linux with ETXTBSY ("Text file busy"); the fixture warm-up exec now retries.

## 0.2.1 — 2026-10-05

### Added
- Optional native context composition projections use the actual prepared request,
  safe tool inventories and independent byte bounds. Usage/details share a clock;
  model changes and compaction invalidate stale request readings.

### Changed
- `tekes-selector stage` of a different build under an already published
  version returns `invalid-state` with state `version-already-published`
  instead of `invalid-bundle`. Each build needs its own version; see
  `packaging/README.md` ([#4](https://github.com/TekesApps/TekesKernel/issues/4)).

### Fixed
- Private data roots open inside the macOS App Sandbox. When the sandbox denies
  the descriptor walk from `/`, the profile opens the canonical path and still
  rejects a path that traverses a symlink.
- The job sandbox error names the probe failure class and detail.
- The test suite builds and passes on Linux; CI now requires the Linux job.

## 0.2.0 — 2026-10-05

First public release. It also contains everything in 0.1.4.

Versions 0.1.1 to 0.1.4 were released from the earlier closed-source history;
their tags are not in this repository. Version 0.1.3 was skipped.

### Added
- `scripts/ci.sh`, the single hermetic check entry point, and a GitHub Actions
  workflow that runs it on macOS.
- `session.discard` transport contract example and its error codes in the
  endpoint fixture checker.
- Contributor documentation: `CONTRIBUTING.md`, `SECURITY.md`,
  `CODE_OF_CONDUCT.md`, `scripts/README.md`.
- Cost benchmark report against pi 1.0 on DeepSeek Flash
  (`docs/benchmarks/deepseek-cost-2026-10.md`) and its reproduction kit
  (`scripts/bench/`); the pi runner gains `--no-skills`.
- `docs/glossary.md`.

### Changed
- Live and benchmark scripts read `TEKES_PROVIDERS_CONFIG` and
  `TEKES_LIVE_KEYS_FILE` (or explicit flags) instead of fixed home-directory
  paths; `run-public-flow.py` needs no key file when `TEKES_KERNEL_LIVE_KEY`
  is set.
- Deployment-specific provider aliases are no longer shipped as proof rows;
  supply them in the runtime provider configuration. Unlisted DeepSeek
  Responses models now inherit the dialect's uniform reasoning levels.
- No crate is published to crates.io (`publish = false`).
- The 29 MB machine-readable architecture inventory
  (`docs/architecture/generated/inventory.json`) is no longer tracked; run
  `scripts/code-architecture.py` to write it locally. The generated Markdown
  atlas stays in the repository.
- The whole workspace is formatted with `cargo fmt` and passes
  `clippy -D warnings`.

### Fixed
- A supervisor test mutated the process environment while other tests ran.
- workspace-service tests failed intermittently under a full parallel run
  because macOS serializes the first execution of new executables.
- Two fixture checks and the management gate did not know `initialPresets.v1`
  and `session.discard`.

## 0.1.4 — 2026-10-05

### Added
- Session model catalogs expose the configured effective context window.
- A `contextUsage` control projection distinguishes conservative request bounds
  from unknown occupancy, independently of cumulative billing usage.
- Context refresh on ledger append, configuration changes, and control reconnect;
  publication sequence persistence prevents restart from replaying an older value.

### Changed
- Compaction, route/policy changes, opaque media and remote continuation state
  invalidate occupancy until a compatible request can be measured conservatively.

## 0.1.2 — 2026-10-04

### Added
- Built-in `coding` and `general` session profiles
  (`crates/tools/prompts/`). The coding profile adds four behavior rules that
  stop unrequested verification loops.
- Native initial-preset catalog (`session.initialPresets`).
- `deepseek-flash` model capability for the DeepSeek Responses dialect.

### Changed
- Automatic title requests always disable reasoning and cap output at 64
  tokens.
- Clients are never offered a `none` reasoning effort; the lowest is `low`.

## 0.1.1 — 2026-10-04

First tagged release.
