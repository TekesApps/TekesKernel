# MCP asynchronous tool continuation: implementation requirements

Status: the worker/supervisor integration described below is now implemented
and locally tested (see "Activation (2026-09-05)"); the original official SDK
public-tunnel gates remain unrun. Scope is the original MCP task and multi-round input
lifecycles tracked in the maintainers' live-test inventory.

## Observed mismatch

`McpDynamicAuthority::execute` returns the remote result through the supervisor
control response. `DynamicSupervisorBackend::execute` currently converts every
successful value to `BackendTerminal::Completed`. A remote working task can
therefore become a completed Kernel tool call prematurely. The raw MCP client
can query and validate an existing task, but the worker does not persist and
resume that task through its execution lifecycle.

`ToolControlStore::lookup_receipt` re-acknowledges an identical request and
rejects changed bytes for an existing request identity. Reusing that identity
for a poll would return the initial response forever. Reusing an approval hold
would confuse authorization with remote task/input state. Neither is an
acceptable implementation.

## Required ownership and identities

- The root/child execution log remains the owner of its open turn and original
  tool call. Waiting and resuming do not open another user turn.
- The supervisor binds remote continuation authority to the workspace, server
  reference, negotiated protocol, credential generation, original call identity,
  tool name and original effective arguments.
- A server-issued task ID or opaque request state is persisted before reporting
  a wait to the worker. It is not an authorization credential or a model-selected
  replacement tool identity.
- Initial execution keeps its immutable request and response receipt. Each
  subsequent query, input update and cancellation has a distinct identity bound
  to the original call and a monotonic continuation step.
- Worker-control must explicitly distinguish terminal success/error from pending
  continuation. Arbitrary JSON in an ordinary tool result cannot impersonate this
  host-owned control state. The existing closed protocol requires a versioned
  extension rather than silently reinterpreting successful `value` payloads.

## Required state transitions

| State | Action | Durable result and next state |
|---|---|---|
| Initial call admitted | Execute tools/call once under existing effect policy | Terminal result, or persist pending task/input identity |
| Task working | Query exact task ID at bounded server poll interval | Append updated pending state or terminal result |
| Input required | Publish request through existing public response authority | Persist user response before one input-update continuation |
| Completed | Validate exact task identity and embedded tool result | One final tool_result for original call |
| Failed/cancelled | Preserve remote failure/cancellation | One final non-success tool_result |
| Stop while pending | Dispatch task cancellation if supported | Persist local stop and remote acknowledgement/uncertainty separately |
| Worker crash while pending | Recover original pending identity | Query task; never repeat initial side effect |
| Authority generation changed | Deny stale continuation | Explicit reconciliation/error; never bind task ID to a new principal |

The initial task result, every poll and a completed task result are separate
facts. Historical context stays append-only. Pending state must participate in
fold validation, checkpoint coverage, replay and terminal eligibility. A worker
may park and release its line lock while the turn remains open. A task terminal
result alone does not settle the enclosing turn or bypass candidate validation.

## Required proof before closing the live gates

1. Real worker + public interface: initial working task parks with no tool_result
   and no settle, then resumes under the same turn and call.
2. Restart after every persistence boundary: original invocation count remains
   one; duplicate continuation receipts are stable; exactly one tool_result.
3. Working → input_required → working → completed: public response is matched to
   the right continuation, sent once, and final embedded content reaches context.
4. Stop, task failure, malformed terminal payload and credential rotation never
   become successful tool results; queued root input follows normal settlement.
5. Provider/tool progress continues through the sink while waiting. Notification
   receipt alone is insufficient proof of delivery.
6. Original official SDK public-tunnel gates pass against the specified server
   identities and exact final payloads. Local scripted tests remain separate.

Existing MCP transport, capability, parameter-header, subscription, query and
state-validation tests are prerequisites. They do not establish this durable
worker integration.

## Implementation progress

The worker-control `continuation` module now defines a format-1 continuation
request bound to the original ToolControl and a host-issued continuation ID.
A domain-separated digest names each positive continuation step. Query/update/
cancel share the step identity so changing the action or original arguments
conflicts with an exact stored receipt instead of executing a second operation.
The module is not advertised by the existing worker handshake; durable pending
outcomes and worker recovery must be integrated before activation.

The format now also has a closed outcome union: pending, completed or failed.
Responses bind both continuation request ID and original call ID. A pending
response retains the same host continuation ID and advances exactly one step;
it cannot also carry terminal value fields. These are protocol building blocks
only, not enabled worker wire messages or a durable continuation store.

The supervisor now has a continuation journal with immutable initial binding,
per-step write-ahead intent and immutable exact response receipt. Preparation
distinguishes first execution, unresolved prior intent requiring reconciliation,
and completed receipt replay. Steps cannot skip a prior pending response or
continue after a terminal one. Authority binding cannot be replaced. Directory
publication is synced alongside atomic file publication. This journal is not yet
connected to worker-control negotiation or live task execution.

`ContinuationJournal::resolve` is the public execution entry: a per-continuation
OS lock spans preparation, remote execution/reconciliation and commit. Raw
prepare/commit are private. A failed execution leaves its intent, causing the
next resolve to invoke reconciliation instead of execute. A committed receipt
bypasses both callbacks. Actual MCP authority callbacks remain to be connected.

The broker now exposes bound-peer tasks/get, tasks/update and tasks/cancel
operations separately from tool execution. A real stdio fixture checks exact
task identity and update input responses and rejects any tools/call on this
route. These operations retain read versus mutation timeout/error handling.
Per-operation cooperative cancellation and durable journal callback binding
remain part of the unfinished integration.

The supervisor MCP bridge now checks the immutable pool authority and bound
task identity, executes query/update/cancel through the broker, validates each
polled state and maps it to explicit pending or terminal outcomes. Recovery of
a lost mutation response only queries; pending remote state cannot confirm that
mutation, so the intent stays unresolved. Terminal results are extracted from
the embedded tool result. A real child-process test covers pending/update/final,
reopen receipt replay without further remote requests, and changed-authority
rejection. Initial worker result publication and parking are still not connected.

## Event/fold/checkpoint activation requirement

Current event-v1 `state` has open subkinds, while `meta` has a closed management
shape. A continuation record must not overload title/ownership metadata. A
versioned state subkind is a possible carrier, but generic state acceptance alone
does not implement its semantics or authorize older writers to resume it.

LedgerValidator currently keeps tool_calls and tool_results in its live fold and
exports/imports them through CheckpointState. Pending continuation identity,
latest step, originating turn/call and terminal eligibility must be added to both
paths together. Checkpoint validation must bind retained pending references to
covered tool calls and reject a supposedly terminal call with live continuation.

The required regression compares full replay with checkpoint-seeded replay at
initial wait, subsequent pending step, response admission and final tool result.
Both paths must reject out-of-order steps, wrong turn/call identity, duplicate
terminal tool_result, and successful settle while continuation is pending.
Interruption/error cleanup needs an explicit transition rather than bypassing
this guard. The writer compatibility boundary must be established before any
production worker emits this state subkind.


The continuation journal now also has a four-process contention regression.
All participants reach the same execution lock before release. Only one remote
cancel callback executes; every participant returns bytes identical to the
immutable receipt. Evidence: `/tmp/tekes-continuation-process-contention.log`.
This proves journal serialization across independent processes, not worker
parking, checkpoint recovery or the original official SDK live acceptance.


The existing D-49 store boundary is now enforced by LockedLedger before tail
repair and append. Reader selection cannot raise the implementation's writer
capability. This closes a prerequisite discovered during continuation design;
it does not add continuation fold/checkpoint semantics or enable worker emission.


The continuation bridge now distinguishes task completion from embedded tool
success: completed + isError=true becomes a non-retryable Failed receipt, with
the remote tool-result details retained in the error message. An explicitly
present isError must be boolean; malformed flags cannot become successful tool
results. Absent/false flags retain the successful result path. The regression
also reopens the journal and proves failure receipt replay bypasses both remote
execution and reconciliation. Evidence: `/tmp/tekes-mcp-task-error-receipt.log`
(two bridge tests, including the real stdio query/update test). This remains in
the not-yet-activated continuation module; production worker park/resume and
ordinary synchronous MCP error mapping are not proved by this test.


## Activation (2026-09-05)

The continuation modules are now wired end to end:

- worker-control-v2 `tool_control_result` gained the closed `pending` arm
  (`PendingContinuation`); `tool_continuation` / `tool_continuation_result`
  are the negotiated wire pair (`worker_control::continuation` codecs).
- Supervisor: `DynamicSupervisorAuthority::execute_outcome` /
  `continue_task`; `McpDynamicSupervisorAuthority` binds a task-shaped
  `tools/call` result into the session's `continuations/` journal and resolves
  steps through `mcp_continuation::resolve_task` under the route's peer and
  credential checks; `ProductionToolControlHandler::continue_task`; the
  process host routes `tool_continuation` lines next to `tool_control`.
- Worker: `BackendTerminal::Pending` → `PipelineDecision::Pending` writes the
  step-0 `state{subkind: tool_continuation}` (runtime visibility);
  `wait_tool_continuation` polls with bounded backoff, records each pending
  step, publishes `input_required` as an `approval_request` (scope
  `mcp_task_input`), sends the answer as exactly one `update`, cancels on stop,
  and terminalizes through `ToolPipeline::complete_continuation` (post hooks,
  secret scan, one `tool_result`). `recover_unpaired_tool_calls` resumes a
  call with continuation records instead of aborting it; reconcile-only runs
  abort it as `recovered`.

Proof (local): supervisor
`task_result_binds_continuation_and_steps_reach_one_terminal` (fake
task-capable peer: bind, two polls, one update sent once, terminal, receipt
replay without a remote request, post-terminal and changed-action refusal,
initial call exactly once); worker
`remote_task_continuation_parks_on_input_and_resumes_to_one_result` (scripted
control channel: steps 0/1/2 durable, hold with the `inputRequests` question,
no result while pending, resume sends the answer once as step 3, one `ok`
result) and `remote_task_continuation_stop_cancels_remotely_and_terminalizes_as_error`.

Still not done: checkpoint-seeded replay does not carry parked continuation
facts (the schema fold reads them from the tail's `state` records only); no real-process test drives a worker through the supervisor
reader route; the official SDK
public-tunnel gates (`TEKES_LIVE_MCP_TASKS_URL`, `TEKES_LIVE_MCP_CONFORMANCE_URL`)
remain unrun.

Task augmentation (2026-09-05, evening): `McpTool.execution.taskSupport` is
parsed from `tools/list`; `McpPeer::call_tool_augmented` / `McpBrokerHandle::call_tool_augmented`
send `task: {ttl}`; the supervisor route calls a task-required tool only that
way (`REQUIRED_TASK_TTL_MS` = 600 000). Fixture scenario `task-augmented`
refuses plain calls and creates `task-aug`; `slice13_gates::task_required_tools_are_declared_and_called_with_task_augmentation`
covers parse, refusal, augmented call, broker route and poll; the supervisor
continuation test asserts the augmented path for a required tool.

Park-and-release (2026-09-05, evening): the schema fold tracks
`state{tool_continuation}` park records per call and exposes
`LifecycleFacts.continuation_wait_until`; `engine::ensure_action_at` (used by
every supervisor spawn decision with the wall clock) spawns nothing before the
instant, `run_decision` treats a parked continuation as an ordinary-mode
obligation regardless of `resume`, and the worker parks once its poll interval
reaches one second (`park` step with `poll_after`/`interval_ms`, clean exit,
open turn), then on resume honors the instant, polls immediately and keeps
doubling from the durable interval. Test
`remote_task_continuation_parks_when_polls_slow_and_resumes_when_due`.
