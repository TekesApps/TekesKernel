# Delegation: parent-child relationships and independent contexts

[Behavioral flows · documentation home](../README.md) · [Main execution flow](turn.md) · [Next: how context is formed](../data/context.md)

## Root is a position, not a type

All line files are schema-identical; all are run by the same worker binary.
"Root" means: the file the user's attention and the supervisor's delivery are
bound to, i.e. the top of the logical task tree. This is not necessarily the OS
PPID tree: the supervisor launches child workers. [Process ownership](../architecture/processes.md)
is documented separately. Parentage lives in events
(`genesis.parent` in the child, `spawn` in the parent) so the tree is
reconstructible after every process is gone.

## Spawning a child

For model-requested delegation, ordering is fixed — **create before register**:

1. After the `tool_call`, hooks, approval, and durable effective invocation
   (`effective_execution` when hooks mutate it), append one model-visible
   `state {subkind:"delegation", payload:{call,name,invocation}}`; the payload
   carries the effective invocation and is reused byte-for-byte on recovery.
2. Copy+sync every asset referenced by that seed record, then write
   the seed snapshot asset ([child seed](#child-seed) / event: canonical source-event JSONL,
   source seqs are provenance only), synced per D-45; then the child
   file: `genesis` (parent pointer **with `spawn_id`** — the durable
   binding, R15-10 — seed provenance + snapshot ref, the child's
   immutable v1 `resume` policy `{"bounded":3}` — D-60). fsync file and
   directory (D-45).
3. Append `spawn {child, tool_call id, spawn id, resume}` to the parent
   (D-44/D-60). fsync.
4. Launch the child worker (`launch_child` — re-sent by any later parent
   run while the `spawn` is unpaired; worker-control §Launch lifecycle,
   R13-7); `wait` by tailing the child ledger; append `child_result
   {tool_call id, spawn id}` — exactly one terminal result per call id.

Automatic validation uses the same create-before-register durability but is
host-owned: `validation.candidate` supplies the frozen scope, the child genesis
carries `validator_for`, and its spawn correlation is `validation-<candidate_seq>`.
It does not fabricate a model `tool_call`. The judge seed is a materialized
`state/validation.judge` with request, candidate, frozen artifact references and root execution evidence
([Context projection](#child-seed)); verdict admission precedes root settlement.

A spawn is refused after a `stop_requested` generation for the tree (D-37):
workers check the gate at yield, the spawn service enforces it, so no
grandchild can slip in while a cascade is in flight.

Crash analysis: a crash after 1 reuses the same effective delegation record;
a crash between 2 and 3 leaves an *orphan file* — harmless by
construction (nothing references it; the sweep quarantines it). A `spawn`
event without a file cannot occur in this order except by external deletion,
which the sweep reconciles as `child_result {failed}`. The only residue this
design permits is the harmless kind.

## Isolation is the point

- The child's full history stays in its file; the parent receives exactly one
  `child_result` event (summary + artifact refs). Context hygiene is enforced
  by the file boundary, not by discipline.
- Child crash ≠ parent crash; the parent (or supervisor) reaps and decides.
- Fork-tearing is structurally impossible: forking projects history into a new
  file for a new process (the rewrite projection; [storage](../data/storage.md)) — there is no shared
  mutable fork state to tear.

## Workflows

A deterministic orchestration (fixed control flow over many agents) is a
**script process**: any executable that spawns workers via the supervisor and
waits on them. Its journal is its own JSONL file, same schema (`spawn` /
`child_result` / `settle`). This absorbs the planned workflow engine — no
embedded JS host, no await bridge; the OS is the await bridge.

## Cross-thread communication

A worker never writes another thread's folder. Send = hand the message to the
supervisor; delivery is stdin (alive) or locked append (dead), per [supervisor delivery](../runtime/supervisor.md).
Authorization for cross-thread sends is supervisor policy, evaluated at
delivery, recorded in the target's file as an `input` with origin metadata.

## Concurrency shape

- Within a thread: one live writer per file; parallel children = parallel
  files. The input queue serializes user input into the mainline.
- Across threads: independent by construction; global limits (provider
  admission) are supervisor semaphores at spawn/call time.

## Child seed

`(parent prefix, whitelist) → genesis + seed snapshot asset` for a new
child file (R15-5/R16): the selected source events are copied as their exact
canonical event lines, in increasing source-seq order, to the
content-addressed JSONL asset referenced from `genesis.seed.snapshot`.
Their seq/turn/correlation fields are retained as source provenance but never
become child-ledger seqs, participate in the child's fold/admission/dedup, or
import a source provider continuation. The renderer consumes normalized
model-visible payloads in asset order and folds them into the first child
epoch's baseline. Event-v1 §Seed snapshot asset format is the byte contract.

- Model-requested delegation uses a task-scoped selection of source events.
  Automatic validation instead materializes a host-authored
  `state/validation.judge` containing the admitted request, exact candidate,
  covered set, frozen artifact references and execution evidence; its genesis declares
  `seed.kinds=["state"]` and a `validator_for` binding. It does not copy the
  executor's entire conversation or use a generic input/output selector.
  Artifact bytes that cannot be supplied are marked unavailable, never
  silently treated as verified. See [Turn flow](turn.md#turn-execution-and-validation).
  Execution evidence contains the root turn's tool calls, results, ordinary
  spawns and child joins through the candidate output sequence. It excludes
  later repair actions, other turns and host-validator spawns. It does not
  enumerate actions inside child logs. Secret-scanner exclusions and size/count
  bounds are reported as omitted events, so missing evidence cannot establish
  that a requested action did not occur.
- Seeds are **materialized** (copied into the child's seed asset), not
  referenced back to the parent: redundancy buys self-containment —
  a line replays from its ledger plus the referenced assets in the thread
  folder. Moving a child JSONL alone does not carry its seed/artifact blobs.
  Asset references inside copied source events are a transitive closure: leaf assets are copied+synced before the
  seed snapshot, which is copied+synced before genesis; GC and rewrite/redact
  scanners traverse the carrier.

## Sub-agents

Spawning a child (see [the topology above](#root-is-a-position-not-a-type)): create and sync the canonical seed
snapshot asset, then create the child file with its referencing `genesis` →
fsync → append `spawn` to own file → **ask the
supervisor to launch it — all worker launches are supervisor-only** (the
single point where the stop gate and admission are enforced, R2-2) → await
its settlement → append `child_result`. The child's full history stays in the child's file; the parent
context receives only the result event.

### Named task inputs

When `task.input_sources` names an earlier task on the same parent line, the
worker resolves it from that task's durable delegation and completed
`child_result`. The original effective invocation remains unchanged. A separate
`resolved_inputs` array in the durable child seed records the source name,
output contract, file path or inline summary, and causal record sequences.
The child uses the resolved path instead of treating a task name as a filename.
Direct file paths remain unchanged. Re-executing a task name supersedes its older version; the latest version
preceding this call must be completed. An unfinished named source returns a
tool validation error without creating a child or restarting the worker. Recovery reuses an existing seed, including older seeds without
this optional mapping, rather than recomputing it from later parent state.
