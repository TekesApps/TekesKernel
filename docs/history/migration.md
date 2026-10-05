# Predecessor systems and migration background

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

This is a migration disposition record against the dated source baseline below.
For the current Rust ownership and calls, use [Code architecture](../architecture/README.md); a historical
carry-over destination is not a current Cargo dependency or runtime acceptance claim.

Source: TekesAppServer + TekesRuntime as of 2026-08. Three buckets: **carry**
(move as-is), **dissolve** (replaced by a primitive), **transform** (same
capability, new mechanism). The new-code bucket at the end is deliberately
tiny.

## Carry as-is (the valuable 90%)

| Current | Destination |
|---|---|
| `Network.ModelAPI.*` — 8 concrete adapters, stream decoding, usage/error mapping, thinking/sampling policies | Worker's provider layer; parity-locked and normalized into provider-runtime-v1's 5 protocol families |
| `Network.HTTPClient` + SSE | Worker |
| `Orchestration.Compiler.*` — context rendering, prompt assembly, tool choice, continuation | The provider-render projection (05) |
| `CacheShape`, canonical tool-call validation, requote | Provider-render projection |
| MCP client stack (`ExternalTools/MCP*`) | Generic `mcp` client/transport/pool hosted by supervisor `mcp_runtime`; worker calls use supervisor control bindings |
| Web tools provider chain | `web-tools-v1`: worker-owned bounded public fetch, URL guard, deterministic local extractor and brokered Tavily source-hit search; Jina/Tavily-extract external readers and synthesized-answer metadata are explicitly retired |
| Read/Write/ApplyPatch/Grep/Glob implementations | Exec via the multi-call helper binary (D-57); in-process only for the mobile seam and measured hot paths |
| GitAgentTools | Dissolved into `shell` — invoke system `git`/`gh` directly; no Git-specific fixed schemas and no git library in the Kernel |
| TekesLocalGit | Stays client-side, serving UI surfaces (Changes panel, diff, branch picker) — outside kernel scope; the kernel's own storage never uses git |
| Safepoint (shadow git) | `safepoint-v1` private full-tree authority + pre-tool seam; Client management binding is owned by the later Slice-14F host extension |
| JobTool disk-only background jobs | Unchanged — already conforms (06) |
| Client ProjectionEngine | UI projection consumer (05) |

The rows above describe implementation disposition, not the fixed namespace.
The exhaustive migration inventory is
[`builtin-tools-v1`](../../spec/builtin-tools.md): one catalog of 27 callable
fixed names, including role-only and conditional names. A separate audit
fixture records both legacy repository baselines and the explicit disposition
of nine removed names; no repository owner survives in the Kernel catalog.
Local-manifest/plugin/MCP instances remain dynamic and are
constrained by the same namespace, catalog, approval, and sandbox rules.
The separately versioned first-party extension fixture also records all 12
tools of the currently shipped optional Computer Use plugin without promoting
those install/platform-conditional MCP names into the fixed namespace.

## Dissolve into primitives

| Current | Replaced by |
|---|---|
| `SQLiteContextStore` / `SQLiteDatabase` / read pool | JSONL files |
| Action system (15 kinds) | The loop's switch (03) |
| `TaskGate`, `TaskCoordinator` plumbing | `spawn`/`wait` |
| Run-state registry, `RuntimeLease`, `RuntimeLifecycle` | Process table + flock (04) |
| Goal intent/actual dual-track | Goal events + process table |
| Checkpoint/page/fence four-state sync, carry-forward, compensation replay | Seq cursors + supervisor reconciliation |
| `InputQueueCoordinator` | stdin + locked delivery append |
| `EventBroadcaster` doorbell | stdout mirror + `appended{seq}` |
| `ProviderAdmissionCoordinator`/queue | Supervisor semaphores |
| ThreadService gates & cross-thread RPC | Folder CRUD + supervisor delivery |
| Most of the 20 RPC `*ContractV1`s | Read-range + control-message on the thin client interface |
| Workflow engine plan (JSC host, await bridge, journal) | Script process + its own JSONL (07) |
| Session/thread/session-run concept split | No separate session store: thread groups lines, file = line of work, process = one run; execution context is reconstructed ([Thread definition](../concepts/thread.md)) |

## Transform

| Current | New form |
|---|---|
| ValidationLoop / ValidationRoute / marker completion edge | Root worker candidate → independent validator when needed → bounded repair in the original context → writer-owned root settlement ([Turn flow](../flows/turn.md#turn-execution-and-validation)); child completion alone does not deliver the root answer |
| Validator minimal-context whitelist | Seed projection whitelist (05) |
| Compaction V3/V4/V5/V5.1 | One lossy projection via `compact` events (05) |
| `Compiler.Rollback`, ResponseRetry | `supersedes` ranges (02) |
| ApprovalCoordinator/Hook, permission probes | `approval_request`/`approval_response` protocol + static spawn policy (06) |
| ConversationCheckpoint | `checkpoint` events (02) |
| Digester / narration / progress sinks | Downstream projections by consumers (05) |
| Usage sinks / cache attribution / token estimator | `usage` events + scan aggregates |
| Schedule coordinator / cron persistence | Global schedule JSONL + supervisor timer (04) |
| Model catalog / capability probes / MCP catalogs | Rebuildable cache files (04) |
| Slash commands (expander/importer/sandbox) | Pure pre-render text transform; explicit user/workspace/project sources under `~/.agents` and `<cwd>/.agents/commands/` (legacy `.agent` fallback; D-55/72) |
| Plugins (manifest/marketplace/install/signing) | Files on disk + exec/daemon backends; install = write files |
| Key stores (model/MCP keychain) | Supervisor-held; brokered via descriptor only — never ambient environment; declared integrations may receive named targeted-injection entries at their own spawn (D-46/52, R2-7) |
| Boot/status surfaces, restart sweep | Supervisor sweep + process table (04) |
| Thread search | ripgrep projection; optional disposable FTS index |
| Mirror-first client sync | Tail-by-seq over thin RPC; local direct read |
| Memory (`Orchestration.Memory`, `SQLiteContextStore.Memory`) | Global `memory/log.jsonl` outside thread folders: canonical named-lock tool transactions, LWW + tombstone projection, epoch-snapshot inclusion per policy (R1 F21) |
| AskUser questions/answers correlation | `approval_request`-style hold with correlation id; answer echoed as event (06 Hooks/ask-user; R1 F22) |
| Plan workflow (submission/approval/revision) | `state` events + control messages over the same hold protocol (R1 F22) |
| User lifecycle hooks (`UserHookConfig/Worker`) | Config authority + exec backend, carried (06 Hooks; R1 F22) |
| Turn evaluation (`EndpointTurnEvaluationV1`) | Events + scan projection (R1 F22) |
| Composer skill/reference discovery catalogs | Rebuildable per-workspace projections over user/workspace/all-bound-project `.agents/skills/` sources with explicit origins and collision failure inside one workspace; unrelated workspaces never share a collision domain (D-55/72/73; R1 F22) |

Conformance step (R1 F22): regenerate this table mechanically from the public
RPC surface, tool registry, and config producers, and diff against these rows
— a feature with no row is a build blocker, not an oversight.

## New code (complete list — the published topology, R3-13)

1. Supervisor **core** (capped, holds no truth — D-24): root singleton,
   spawn/reap/reconcile, delivery + receipts, stop reconciliation, admission
   leases (04).
2. Supervisor **bundled services** (contract-isolated, individually
   evictable): ring-buffer mux, daemon brokers, schedule, catalogs/search
   projections, rewrite/GC (04).
3. Event schema library: envelope, torn-tail repair, replay fold (02).
4. Worker shell: startup, loop switch, stdio protocol, hold policy (03) —
   wrapping the carried-over projection/dialect code.
5. The multi-call filesystem helper (D-57).
6. The production provider adapter/runtime library: carried dialect rendering,
   HTTP/SSE, terminal normalization and provider-adapter recovery
   ([provider-runtime-v1](../../spec/provider-runtime.md)).
7. The Session Endpoint physical transport: a thin HTTP/WebSocket binding over
   the Slice-6 projection/service
   ([endpoint-management-v2](../../spec/session-endpoint.md),
   [endpoint-transport-v2](../../spec/session-endpoint.md)).
8. External deployment assets and harness: launchd plist, installer/version
   selector, readiness/observability and rollback tests; these are packaging,
   not a fifth Kernel authority ([deployment-v1](../../spec/deployment.md)).

Anything appearing in an implementation that is not in one of these
lists needs a decision recorded in [Design decision records](decisions.md) first.

## What evaporates

- SQLite context store and its read pool — the JSONL file *is* the store.
- The Action system (15 kinds) — control flow returns to code; the program
  counter is the state.
- Checkpoint/page/fence sync machinery — pages are event ranges; cursors are
  `seq`; compensation is a reconcile-only recovery run (D-61).
- Run-state registry, runtime lease, TaskGate — the file lock and `wait()`.
- Goal intent/actual dual-track — goals are events; liveness is the file lock.
- Most RPC contracts — reduced to "write a control event to stdin" and "read an
  event range from a file".
- The planned workflow engine (JSC host, await bridge) — a workflow is a script
  process that spawns workers; its journal is its own JSONL file.

## What is carried over unchanged

The valuable 90%: provider dialects (`Network.ModelAPI.*`), context rendering and
prompt assembly (`Orchestration.Compiler.*`), cache shape, the MCP client stack,
web providers, git tooling, safepoints. They are pure functions or external-world
adapters; the reduction does not touch them. See [Predecessor systems and migration background](migration.md).
