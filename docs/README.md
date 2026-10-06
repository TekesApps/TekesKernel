# TekesKernel: overview and reading path

TekesKernel records task history as durable events, executes tasks in workers, and manages their runtime through the supervisor.
Start with how a thread progresses, then open the crate responsible for a particular step. Old numbered-chapter references are mapped in [history/document-map.md](history/document-map.md).

## First reading

Terms such as Slice, gate, D-NN, epoch or carrier are explained in the
[glossary](glossary.md).

[What is a thread?](concepts/thread.md) → [How does a turn execute and settle?](flows/turn.md) →
[What does the user see?](interfaces/client.md) → [How is data stored?](data/storage.md) →
[Where is the implementation?](architecture/crates.md).

Crate documentation lives beside its source, for example [schema](../crates/schema/docs/README.md),
[store](../crates/store/docs/README.md), [worker](../crates/worker/docs/README.md).
Each crate's `docs/README.md` explains its responsibilities; `docs/generated/` contains module, interface and function call references.

## Read in layers

Read one layer until you can answer its question, then move deeper. The generated
inventory is a reference to consult for a specific symbol, not a book to read from
beginning to end.

| Layer | Read | What you should be able to explain |
|---|---|---|
| 1. Overall model | The rest of this overview, then [Thread fundamentals](concepts/thread.md) | What persists; how thread, execution context, run and turn differ; who owns settlement |
| 2. Behavior | [Turn flow](flows/turn.md), then [Client delivery](interfaces/client.md); add [delegation](flows/delegation.md) when needed | Follow input → execution → candidate → validation/repair → settlement → delivered answer |
| 3. Data and contracts | [Storage](data/storage.md) → [Events](data/events.md) → [Durability](data/durability.md); use the [spec index](../spec/README.md) for exact rules | Which file owns a fact, which event records it, and which invariant must hold after a crash |
| 4. Processes and crates | [Processes](architecture/processes.md) → [Crate map](architecture/crates.md) → a selected crate's `docs/README.md` | Which executable runs the work, which library owns the rule, and where IPC crosses a process boundary |
| 5. Modules and functions | The selected crate's `Cargo.toml`, root source file and `docs/generated/` | Identify the public entry point, follow calls into internal modules, and explain the relevant state change or side effect |
| 6. Verification and rationale | [Verification](verification/README.md), then the relevant [history](history/README.md) entry if needed | Distinguish the required behavior, evidence that was actually obtained, and the reason for a design choice |

Read [model context](data/context.md) when following provider input, and
[idempotency](data/idempotency.md) when following retries. Read the
[runtime guides](runtime/worker.md) for waiting, recovery and tool execution,
and the [system prompt](runtime/system-prompt.md) for which instructions are
shared and which belong to the coding profile.
These are branches from the main path; the first pass does not require every
contract, gate or historical review.

## First source walkthrough: schema

For a reader new to Rust, start with one small library before following a complete
request across the system:

1. Read the [schema guide](../crates/schema/docs/README.md) for its responsibilities
   and module map.
2. Open [Cargo.toml](../crates/schema/Cargo.toml) to identify the package, dependencies
   and target configuration. A package can contain both library and binary targets;
   a crate is one compilation unit, not a running process.
3. Open [src/lib.rs](../crates/schema/src/lib.rs). Follow `mod` declarations into
   modules and `pub use` re-exports to their definitions. A library root may also
   contain implementations; it is not required to be only an interface list.
   Binary targets conventionally start at `src/main.rs`, and Cargo can specify
   other root paths.
4. Use the [API inventory](../crates/schema/docs/generated/api.md) to locate
   `validate_ledger`, then the [fold reference](../crates/schema/docs/generated/src--fold.md)
   and [fold source](../crates/schema/src/fold.rs) to follow
   `validate_ledger → Event::decode_canonical → LedgerValidator::push → finish`.
   Ask what each function receives, checks, changes and returns, including errors.
5. Move to [store](../crates/store/docs/README.md) to see how those rules meet files
   and locks, then [engine](../crates/engine/docs/README.md) and
   [worker](../crates/worker/docs/README.md) to see how execution uses them.
   Follow [provider](../crates/provider/docs/README.md) for model calls, or
   [supervisor](../crates/supervisor/docs/README.md) →
   [endpoint](../crates/endpoint/docs/README.md) →
   [transport](../crates/transport/docs/README.md) for client delivery.

`pub` and `pub(crate)` describe visibility; they do not establish that a call
occurs. Use static graphs to find possible paths, scenario sequence diagrams to
understand ordering, and the linked source to check a specific branch. The
[index method](architecture/method.md) explains unresolved calls and other limits.

## Follow a question deeper

| Question | Where to read |
|---|---|
| How do execution contexts cooperate? | [Delegation and child contexts](flows/delegation.md) |
| How do processes run, wait and recover? | [Worker](runtime/worker.md), [Supervisor](runtime/supervisor.md) |
| How are tools executed, approved and isolated? | [Tool execution](runtime/tools.md), [Tool permissions](runtime/tool-permissions.md) |
| What are the event format and reliability rules? | [Events](data/events.md), [Durability](data/durability.md), [Idempotency](data/idempotency.md) |
| What context does the model receive? | [Context projection](data/context.md) |
| Where are the Client and Web boundaries? | [Client](interfaces/client.md), [Web](interfaces/web.md), [Exact contracts](../spec/README.md) |
| Where should I enter the source? | [Code architecture](architecture/README.md) → [Crate map](architecture/crates.md) → each crate's docs |
| Which behaviors have been verified? | [Verification entry point](verification/README.md), [DeepSeek cost benchmark](benchmarks/deepseek-cost-2026-10.md) |
| How is the Kernel launched and integrated? | [Built-in launch](builtin-launch.md), [Workspace service endpoint](workspace-service-wse.md), [Workspace Client integration](workspace-client-integration.md) (response compatibility verified; Client endpoint adaptation and UI acceptance remain open) |
| Why was it designed this way? | [History and decisions](history/README.md) |

The following sections explain the system's core model.

## The reduction

Model the agent with exactly two primitives:

- **Process** — compute. Verbs: `spawn`, `exit(code)`, `signal`, `wait`. A thread
  is "running" at a line iff a live process holds that line's file lock.
  A *file* holds a line of work; a *process* is one run over that file.
  A thread groups the main line and delegated lines; execution contexts
  (often called sessions) are reconstructed from their history and launch
  inputs, not stored in another session database. Processes die freely;
  files survive. [Thread fundamentals defines thread, turn, context and run](concepts/thread.md#thread-fundamentals-a).
- **Log** — state. An append-only JSONL file per line of work. Everything the
  system knows *about a thread* — inputs, model output, tool calls and
  results, goals, usage, outcomes — is an event; thread history not derivable
  from events does not exist. Durable append-only logs are one of five named
  authority classes (D-1): semantic thread ledgers and closed protocol-carrier
  journals, liveness, user/config files, secret stores, the external world —
  and every derived artifact is rebuildable from its authority. A protocol
  carrier may own externally exposed wire identity/bytes or D-70 internal
  exact-retry receipts that cannot be reproduced, but it is never semantic
  thread history or worker replay input.

Consequences, each of which replaces a subsystem of the predecessor
(TekesRuntime/TekesAppServer) stack:

| Rule | Replaces |
|---|---|
| Running = live process holding flock | run-state registry, runtime lease, goal.state intent field |
| Line tail settled = no live writer ∧ terminal settle recorded | stop/settlement coordination; a recorded turn outcome is distinct from process exit |
| Only the worker (or, holding the lock, the supervisor) writes a file | write coordination, echo/version conflicts |
| Result of anything = read the log | mirror-vs-truth disputes; the mirror is a view |
| Fork = rewrite projection into a new file + new process ([Thread definition](concepts/thread.md)) | fork/repair machinery; torn forks become structurally impossible |
| Sub-agent = child process writing its own file | task coordinators, action queues |
| Crash recovery = replay log + reconcile tail | recovery contracts, compensation replay |
| Extension = a process speaking a wire protocol on stdio | plugin systems |

## The kernel: four components

Everything that remains after the reduction. Nothing else is kernel.

1. **The loop** (worker): render prefix → call provider → append events → spawn
   tools → append results → repeat until a settle event. Control flow is code —
   a `switch`, not an action-object system. See [Worker: startup, waiting and recovery](runtime/worker.md).
2. **The projection**: deterministic pure functions from an event prefix to
   provider wire bytes (per-dialect), to UI state, to compact summaries, to child
   seeds. Prefix-stability here is the entire prompt-cache lever. See
   [Model context: projection, epochs and compaction](data/context.md).
3. **The tool ABI**: `tool_call` event → execute → `tool_result` event. Default
   backend is `spawn(argv, stdin) → (exit, stdout)`. The approval gate is the
   process's syscall filter. See [Tools: invocation, execution and results](runtime/tools.md).
4. **The supervisor**: the resident execution-core component. Spawn/reap/reconcile, the
   process table, cross-folder delivery, the daemon pool for non-forkable tools,
   live-stream ring buffers, the sweep. See [Supervisor: hosting, delivery and reaping](runtime/supervisor.md).

## Layer picture

This is a logical execution-core diagram, not the complete OS process tree.
The deployed product also has a resident selector and on-demand installer/helper/MCP
processes. See [current process ownership](architecture/processes.md) and
[the source atlas](architecture/README.md). The native Client uses the V3 endpoint;
this conceptual diagram does not authorize direct Client filesystem access.

```
┌─────────────────────────────────────────────────────┐
│ clients (macOS app, CLI, remote)                    │
│   render = projection over events; input = control │
└──────────────┬──────────────────────────────────────┘
               │ current Client endpoint (V3); host reads authoritative files
┌──────────────┴──────────────────────────────────────┐
│ supervisor (resident execution host)                │
│   process table · spawn/reap/reconcile · delivery   │
│   daemon pool · ring buffers · sweep · schedule     │
└──────┬──────────────┬───────────────┬───────────────┘
       │ spawn/wait   │ spawn/wait    │ spawn/wait
┌──────┴─────┐ ┌──────┴─────┐ ┌───────┴────┐
│ worker     │ │ worker     │ │ tool procs │  ← processes (ephemeral)
│ main.jsonl │ │ child.jsonl│ │ mcp/shell/…│
└──────┬─────┘ └──────┬─────┘ └────────────┘
       │ append       │ append
┌──────┴──────────────┴───────────────────────────────┐
│ thread folders (durable truth)                      │
│   *.jsonl · assets/ · .thread-search.json           │
└─────────────────────────────────────────────────────┘
```
