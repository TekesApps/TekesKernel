# worker — executable for a log execution run

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

tekes-worker is a binary target that assembles logs, snapshots, providers, tools and control channels into an execution loop.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `main` | Process entry, option parsing, profile loading and the run loop |
| `control_channel` | Supervisor control reader, cancellation and protocol failures |
| `delivery` | Ready and parked delivery handling and post-turn exit |
| `recovery` | Reconciliation of unresolved provider attempts and pending tool calls |
| `ledger_events` | Run start, turn open, appended receipts and event helpers |
| `queue` | Queue transactions: targets, retractions and replacements |
| `provider_turn` | Provider turn loop, eager dispatch and provider selection |
| `context_projection` | Provider context projection, epochs and rendering |
| `compaction` | Automatic compaction and prefix token bounds |
| `turn_terminal` | Terminal validation, provider failures, admission retry and settlement |
| `tool_backends` | Tool catalog context, backend assembly and tool-control exchange |
| `tool_calls` | Provider tool-call batch execution |
| `continuation` | Remote tool continuations: polling, parking and input updates |
| `child_agents` | Child agent spawn, launch and result delivery |
| `web_search` | Tavily web-search provider |
| `validation_runtime` | Validator launch, feedback and settlement for candidate outputs |
| `provider_context_tests` (with `live_tests / validation_tests`) | Test modules, not additional runtime processes |

## Interfaces and calls

Main entry points: main/run are process entry points; exchange_tool_control is a pub function inside the binary, not an importable worker library API.

main → run → load_profiles / LockedLedger::open → run_decision → run_provider_turn → run_provider_turn_inner → provider/tool/validation branches.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

One turn may contain multiple model requests; output does not automatically mean settle. A pub item in main.rs is not thereby a library API for other crates.

The validator seed freezes the request, candidate, artifact references and
root execution facts through that candidate. Action requirements are checked
against calls and successful results/joins; final prose need not repeat a tool
invocation. Omitted or unavailable evidence is explicit.

Eager dispatch executes a provider tool call as soon as the stream marks its
arguments complete, writing the `tool_call` ahead of the response terminal;
the terminal must then reproduce the same call or the attempt fails, and
replay orders an eager result after its attempt's sealed output. Native
deferred tools route per exact provider capability
(`ResolvedDialectProfile::native_deferred_tools`, a `provider::NativeDeferredMode`).
A remote tool continuation polls with a growing interval and parks the line
(`state{tool_continuation}` with `poll_after`) when the next poll is far away;
the supervisor resumes the run when due. A rate-limited attempt waits durably
before its retry (`wait_provider_admission`, `state{provider_admission}` with
`next_attempt_at`; `Retry-After` or a doubling backoff clamped to thirty
seconds), cut short by stop or supervisor loss.

A prepared candidate above `compact_trigger_tokens` or a context overflow
triggers auto-compaction; before the attempt's lease is released the worker
runs the summary request (`run_compaction_summary`, `summary_artifact` only,
bundle capped at 60 % of the window) and `build_compaction_event` writes the
admitted continuation as the summary and the verdict as
`compact.summary_request`, falling back to the deterministic quoted history.

Behavior contract: [provider-runtime.md](../../../spec/provider-runtime.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.

Provider continuation may reuse only the latest epoch when model, system,
renderer and tool catalog still match. Returning to an earlier catalog digest
creates a new epoch; it cannot resurrect an older response chain. Deferred tool
schemas are projected from successful search results in the current turn.

For configured toolchains, process tools receive a PATH built from the frozen
`toolchain_roots` bin directories plus standard system bins. The corresponding
installations are sandbox-readable, not writable. Host PATH and host environment
variables are not copied into tool processes.

A ledger version gate is a protocol incompatibility (exit 76). The worker
negotiates its control connection, then checks the ledger before requesting a
provider lease or appending execution facts. Unsupported logs retain their
original bytes, including tails the current writer cannot interpret.

Delegated workers derive their subagent role from the child genesis and matching
parent spawn. That role exposes `report` and instructs the model to report back;
ordinary roots cannot gain the role by selecting `report`. A parent summary uses
only a successfully executed report in the child's latest turn. Reporting alone
does not settle the child turn or end the parent thread.
Provider response adoption checks every already-durable call in the recovering
attempt before appending response facts. The adopted response must retain each
call ID, tool name, and materialized JSON arguments, including spilled assets.
An omitted or changed call fails recovery instead of silently reusing its ID.
