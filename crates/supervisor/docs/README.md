# supervisor — resident process and production wiring

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

The host owns worker lifecycle, client management, dynamic bindings, credentials, MCP, schedules and output projection.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `main / daemon` | CLI dispatch, installation identity, listeners, draining and service startup |
| `lib / process_host` | Snapshot launch boundary, worker management, control frames, sweep and delivery |
| `endpoint_host / endpoint_carrier` | Unary management, v3 streams and carrier wiring |
| `client_admin / client_extensions / resource_capability` | Configuration, extension management and resource capabilities |
| `tool_control / production_tool_control` | Internal tool RPC, idempotent receipts, child tasks and jobs |
| `dynamic_bindings / mcp_runtime` | Launch bindings over the MCP catalog, MCP management and execution (including the `oauth` binding exchange through `SecretAccess`) |
| `continuation_journal / mcp_continuation` | Durable remote tool-continuation journal and the MCP task continuation (bind/poll/update/cancel) behind worker-control durable control |
| `builtin / observability` | Application-owned launch, logs, metrics and health |

## Interfaces and calls

Main entry points: daemon::run_daemon, ProductionProcessHost, ProductionCarrierAssembly, launch_profiled_worker_with_secret_store_and_binding_resolver.

main::run_production → run_daemon → run_daemon_inner → ProductionProcessHost / endpoint assembly → TransportServer; inputs and worker lifecycle pass through process_host.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Lib and bin are different targets within the same package; endpoint/transport are libraries inside the host, while MCP/worker control boundaries include real IPC.

The response authority validates child-ledger ancestry against durable parent
spawn events before deriving a private worker key. Public session identity
remains the root session UUID. Appended notifications publish durable root and
child actionables; opening the actionable stream recovers holds from the spawn
tree. See the [current audit](../../../docs/verification/audits/kernel-alignment-2026-09-04.md)
for the actual external-client test scope.

Answer holds carry the singular `ask_user_questions` invocation (`question` and
`options`). The carrier wraps that invocation in the public `questions` array;
client answers are bound to the originating hold and written as durable
`approval_response` facts before the workflow resumes.

MCP task results bind a remote continuation (`mcp_runtime::execute_outcome`,
`continue_task`), the production secret policy scans pending states and
continuation responses, and a cached binding keeps its routes alive across
worker runs, rebinding when its authority is gone. The MCP catalog is merged
before allowed-tool validation so an allowed `mcp__*` tool is never rejected as
absent. The periodic sweep honors `LifecycleFacts::durable_wait_until` (parked
continuation or provider-admission wait) through `ensure_action_at` with a
clock, spawning nothing before the due instant. After the first
accepted root input the host seeds a deterministic title `meta` and refines it
once through the pinned `deepseek-v4-flash` route (`automatic_title_route`,
session-endpoint §Management authority and recovery). A root line that exits settled/completed runs the
post-turn memorization (`merge_turn_memory`): the candidates the thread noted
and left unconsumed are memorized through
`WorkflowBackend::automatic_memory_merge` on the workflow memory log, with no
model request and nothing written when there is none.

`commands/run` with a body that is exactly `compact` (command-catalog
reserved verb) is delivered through `SessionDeliveryAuthority::compact`: a live
worker receives worker-control `compact` and receipts it; an idle line is
compacted by `locked_compact` (checkpoint, `engine::plan_context_compaction`,
origin-keyed `compact` event, same origin → original receipt); the model
summary request (`summary_for_manual_compaction`, own thread, before the line
lock) supplies the event's `summary_request` record and its summary.
An `oauth` MCP binding whose record is a Kernel-minted refresh grant is
exchanged inside the HTTP authorization provider through `SecretAccess`
(`http_request_authorization`, `provider::OAuthTokenExchange`); production
startup installs no mutation authority. Credentials are owned by the launching application.


These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.

After a locked approval response, a delegated child resumes with dependency
admission when its genesis matches an unresolved parent spawn. This preserves
progress when a waiting parent occupies the ordinary worker limit. Missing,
forged, or already resolved bindings cannot acquire that admission. The child
still owns its tool result and turn settlement; the parent owns `child_result`.
A provider-frame cache refresh projects only the durable prefix preceding that
attempt's response records. The worker may already have appended its output
while earlier pipe frames are unread; reading that newer disk tail must not let
output overtake those frames. A semantic result already published through another
path remains sealed and is never reopened by a late frame.
