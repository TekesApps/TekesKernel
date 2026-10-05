# engine — execution rules and tool coordination

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Provides lifecycle, admission, delivery, recovery, transaction and tool-dispatch rules used by worker/supervisor.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `lifecycle / admission / delivery / stop / supervisor` | State classification, resource admission, idempotent delivery and stop/recovery rules |
| `context / compact_gate` | Pure semantic-anchor selection, bounded history-compaction planning, and the compact transition gate (`first_post_compact_attempt`); the worker owns checkpoint and event writes |
| `compaction_summary` | The compaction summary (event §compact `summary_request`): frozen source bundle, evidence-address admission of the compactor's `summary_artifact`, and the record/summary the compact writer applies (`CompactionSummary::apply`; deterministic quoted history as the fallback) |
| `brief_contract` | Host reading of the `brief` call (`read_brief`): atom/source/goal cross-checks and the host-derived effective completion |
| `transactions / provider` | Attempt phases, commit order and provider recovery decisions |
| `dispatcher / dynamic_catalog` | Fixed/dynamic tool checks, approvals and dispatch |
| `tool / system_tools / supervisor_tools / workflow_tools` | Backend routing, system operations and workflow tools |
| `seams` | ProcessHost, ProviderAdapter, ToolBackend and fake implementations |
| `validation / validation_writer` | Worktree candidate-validation rules and writer-owned commits |

## Interfaces and calls

Main entry points: classify, run_decision, AdmissionPool, ToolDispatcher, DynamicToolDispatcher, ToolBackend, validation_decision.

ToolDispatcher::dispatch → validate catalog/schema → ToolPipeline → policy/hook → backend → durable result.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

Dynamic provider projection exposes resident schemas plus deferred schemas
backed by a successful prior `tool_search` result in the current turn. Projection
and dispatch share `ToolPipeline::has_causal_offer`; missing backend dependencies
still suppress exposure. Reopening a log reconstructs availability from durable
facts, while a later turn needs its own offer. Search does not permanently enable
a tool.

## Boundary

This is not a separate process, nor is it entirely pure functions: backends can perform side effects. Validation code in the worktree does not establish real acceptance of its migration.

`lifecycle::run_decision` and `ensure_action_at` treat a parked remote
continuation and a durable provider-admission wait alike through
`LifecycleFacts::durable_wait_until`: the line is `recovery_needed`, nothing is
spawned before the due instant, and the run that follows is ordinary regardless
of the resume policy. `WorkflowBackend::automatic_memory_merge` is the post-turn
memorization: when a root turn completes it memorizes every candidate of that
thread still unconsumed in the memory log (`unconsumed_candidates`) in one
merge keyed by thread and turn — no model request, nothing written when there
is no candidate, and a repeated merge is a no-op receipt.

Behavior contract: [tool-runtime.md](../../../spec/tool-runtime.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
