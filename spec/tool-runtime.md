# spec: tool runtime v1

This contract closes the production execution seam for Slice 8. The fixed
namespace and classifications are owned by [builtin-tools](builtin-tools.md);
hook, helper, and sandbox bytes are owned by their existing contracts. This
file specifies how one provider tool call becomes one durable execution and
how the 23 tools reach their real backends.

## One dispatcher

The engine hosts one `ToolDispatcher`; the `tools` package remains the
catalog/hook/helper/sandbox substrate. The dispatcher resolves the call name against the
effective catalog frozen in the current epoch, validates the exact argument
shape, derives `backend`, `effect`, availability, policy and hooks from that
entry, and then executes. A provider-supplied backend/effect/policy value is
ignored and rejected if present in the invocation envelope.

There is no Runtime catalog, AS catalog, compatibility alias, Git family, or
second fixed namespace. Dynamic external/plugin/MCP tools enter the same
collision and policy pass but cannot shadow a fixed name.

## Invocation identity and durable order

The invocation identity is `(thread, call_id)`. `call_id` is unique for the
entire thread file; its event envelope supplies the one turn. The dispatcher
follows this order:

1. validate call id, name, arguments, catalog availability and turn budget
   without side effects;
2. append `tool_call` and barrier-sync it when the effect is side-effectful;
3. run ordered pre-hooks; if they mutate, append and barrier-sync
   `effective_execution`; the resulting arguments become the only executable
   arguments;
4. revalidate the effective invocation, derive its non-downgradable approval
   class, and run the generic allow/deny/durable-hold gate; a required hold
   appends and barrier-syncs `approval_request` and parks before any effect;
5. append and barrier-sync `tool_execution_started`, then invoke exactly one backend with the full durable `ToolExecution` plus its
   effective invocation; the closed backend terminal is `completed(value) |
   hold(request) | unavailable(error)`;
6. run ordered post-hooks over the backend result, then run the total
   secret/redaction/withhold transform over the combined result;
7. publish any referenced assets first, then append exactly one
   `tool_result`; before that result can be rendered into another provider
   request or followed by a final settle, the next `attempt` or `settle`
   barrier syncs the whole prefix through the result;
8. release backend resources and continue the provider loop.

A pre-hook deny or approval denial appends a denied `tool_result` without
executing. A backend `hold` appends the durable request and returns control to
tail-lifecycle without fabricating a successful `tool_result`. An unmutated
call's effective execution is its durable `tool_call`; a mutated call's later
`effective_execution` overrides it. A pre-hook
ask follows the hold path. A post-hook deny/secret rejection appends the
contract's withheld result; it does not erase the already-durable execution.
An unpaired durable call is recovered using event/D-19 and the backend's
effect class. No process-local completion proves execution.

## Backend mapping

| manifest backend | owner and transport | recovery boundary |
|---|---|---|
| `in_process` | worker library over ledger/profile state | pure or ledger transaction; replay by call id |
| `worker_hold` | worker approval/question state machine | durable request/response barriers |
| `supervisor_control` | correlated worker-control request through the engine adapter | durable control receipt permits exact restart replay; semantic fact remains worker-authored |
| `helper_fs` | exec-helper with inherited root descriptors | append-before-execute; helper request id = call id |
| `helper_exec` | exec-helper process group | append-before-execute; crash is unknown side effect unless result proves otherwise |
| `background_exec` | supervisor job broker + durable job directory | job id is stable and queryable after worker exit |
| `worker_http` | worker-owned bounded HTTP client | append-before-send; typed network result |

The dispatcher must implement every fixed entry whose `availability` resolves
true. If its backend dependency is absent, the entry is not advertised. It is
forbidden to advertise and return a successful empty placeholder.

## Fixed-family obligations

- Filesystem/search/shell operations use descriptor-first helper operations;
  `shell` invokes system `git`/`gh` when requested and no Git-specific fixed
  tool exists.
- `apply_patch.expected_artifact_version` is a durable history token, not a
  number derived from file metadata. Before the helper effect, the worker must
  reserve the next version for `(workspace, normalized path, call_id)` under a
  cross-process named lock and barrier-sync a canonical append-only authority
  record outside the writable workspace. The reservation is a safety fence:
  an uncommitted reservation survives worker death and rejects the old token.
  Authority state binds every visible version to the observed content SHA;
  an out-of-band SHA change advances the version, while a SHA change following
  a pending reservation adopts that already-reserved version. After the
  helper's atomic SHA compare-and-replace, a committed record binds the
  resulting SHA to the reserved version. The helper SHA CAS remains mandatory
  and is the final filesystem race guard.
- Ask/plan are durable holds. Answer, denial, stop, archive/unarchive, and park
  follow tail-lifecycle and worker-control exactly.
- `task`/`subagent` use the doc 07 spawn transaction. After hooks and approval,
  the worker first records one `state {subkind:"delegation"}` whose payload is
  `{call, name, invocation}` and whose `invocation` is the durable effective
  invocation (not the provider's pre-hook arguments). That exact source event
  is the v1 child seed snapshot; the fixed child resume policy is
  `{"bounded":3}`. A retry reuses the matching delegation state, `spawn`, and
  child binding and MUST reject mismatched durable bytes. `report` succeeds
  only after the child-to-parent proof is verified and the exact parent line
  is alive or durably queued to run; `report` and `context` use typed
  supervisor-control messages and origin-key dedup. The generic
  `tool_result` still closes the tool lifecycle fold, but for a call with a
  `spawn` it is not provider-projected: the exactly-one `child_result` is the
  sole provider-visible tool result for that call.
- `job` creates or addresses one ownerless job directory. `job.start` carries
  its requested `writable_paths` into the durable job specification and the
  sandbox-launch boundary. The supervisor injects an immutable primary
  workspace cwd plus the permitted-write-root ceiling for that call; absent
  `working_directory` resolves exactly to that cwd. Before creating a job
  directory, the broker canonicalizes the explicit/default cwd, independently
  proves every requested write root is a subset of the injected ceiling, and
  passes only the resulting effective policy to the launcher. The broker may
  add the new job's own private state directory, never the whole job root, as
  internal read/write authority. Detaching never converts an unknown process
  into success; query/cancel are idempotent.
- Web/network tools obey the effective sandbox network policy, redirect/size/
  timeout limits, and response secret scan.
- Oversized tool output is trimmed rather than summarized when a request is
  over budget: the worker appends a replacement `tool_result` carrying a
  bounded head and tail around one marker, `supersedes` the original, and
  opens an epoch with reason `tool_result_trim`. It makes no model call and
  the whole output stays on disk. This is the only pressure relief available
  inside an open turn, because compaction covers only settled turns.
- skill/tool discovery reads the immutable instruction snapshot or a declared
  dynamic catalog; it does not scan mutable directories mid-run. A
  successful `skill_explorer`/`tool_search` `tool_result` that is still
  model-visible — not superseded and not covered by a `compact` — is the
  causal-offer proof consumed by `skill` or a deferred dynamic tool, in its
  own turn or a later one of the same provider conversation. The dispatcher
  verifies that durable result before execution; no `offers` side log, cache
  entry, or process memory is authority. Because the offer outlives its turn,
  the declared tool catalog only grows between compactions; a catalog change
  opens an epoch with reason `tool_profile_change`.
- Goal operations use `goals/log.jsonl`, with adjacent locking, canonical JSONL,
  `(thread, call)` exact-retry deduplication and a durability barrier before
  success. The artifact-version authority remains
  `tool-state/artifact-versions.jsonl`.
- The legacy `memory/log.jsonl` is an inactive historical archive. Kernel no
  longer stages, merges, reads or repairs it. See
  [retired Kernel memory](../docs/history/retired-kernel-memory.md).

## Supervisor-control extension

Slice 8 activates [worker-control](worker-control.md) with one correlated
pair:

```text
tool_control {request_id, session, thread, turn, call_id, name, arguments}
tool_control_result {request_id, call_id, value | error}
```

`request_id` is derived from the durable `(session, thread, turn, call_id)` as specified
by worker-control. Retries with identical bytes return the original response
from its durable control-receipt authority;
different bytes are `idempotency_conflict`. The supervisor result has already
passed the effective secret policy before its D-70 receipt, but is not a
`tool_result`; the worker validates it, runs post-hooks then the total secret
transform, and authors
the event. EOF leaves the durable call unpaired for recovery.

Effectful dynamic supervisor routes add a stronger barrier: durable
`control-intent` → external execute with the stable `request_id` idempotency
key → durable receipt. Recovery of intent-without-receipt performs the declared
read-only reconciliation query and accepts only `confirmed | not_found |
unknown | conflicted`; it never converts timeout into failure or blindly
repeats the effect. Routes that cannot provide this contract fail closed before
execution.

## Resource bounds and secrets

Arguments and inline results obey event spill rules. Helper/stdout, HTTP,
and daemon responses have explicit byte and wall limits from the effective
profile. Secret values are handles until the last responsible backend and
never appear in hook payloads, logs, events, result details, or diagnostics.
Secret scanning occurs after all post-hook transformations and before result
exposure, asset publication, or ledger commit.

## Fixture contract

`fixtures/tool-runtime/cases.canonical.json` enumerates every fixed tool exactly
once and declares exact input/expected artifact paths for every
backend/effect/crash case. Slice 8 adds canonical request/result
transcripts for all supervisor-control and job-broker operations, plus positive
and negative cases for policy, hook mutation, approval, secret withholding,
timeout, cancellation, crash-after-effect, dedup, and unavailable dependencies.
Fixtures are hand-authored and checked against the builtin catalog in both
directions.

The registry's `artifact_contract` is executable. Every tool name owns exactly
`tools/<tool>.case.canonical.json`; every `matrix_cases` row owns exactly
`matrix/<backend>--<effect>--<failure>.case.canonical.json`. Each case is one
RFC-8785 canonical object plus LF with exactly `input`, `preconditions`,
`crash_points`, `expected_events`, and `expected_result`.

`matrix_semantics=minimum-axis-cover` means the committed matrix is the
smallest marginal coverage set, not the invalid 7×10×10 Cartesian product:
every declared backend, effect, and required failure occurs in at least one
row; every row is a real backend/effect pairing owned by a fixed manifest
entry; rows are unique; and row count is exactly the maximum of the three axis
cardinalities. The ten hand-owned rows therefore cover all three axes once at
minimum size. Dispatcher/backend tests may add operation-specific cases, but
cannot substitute them for or delete this normative minimum.

Per-tool cases validate one schema-positive invocation and the exact derived
availability/backend/effect classification without producing an event. Matrix
cases begin after a durable effective invocation and own the required barrier,
terminal, no-repeat, recovery, denial, withholding, mutation, and failure
expectations. Production code and tests never generate or rewrite either
corpus.
