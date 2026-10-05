# worker-control — internal supervisor-worker protocol

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Defines control messages, negotiation and codecs; it launches and supervises no processes.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `lib` | Hello/Selected, input, receipts, leases, frames, child launch and codecs |
| `v2` | Production launch selection, queue transactions and tool_control correlation |

## Interfaces and calls

Main entry points: Hello, SupervisorMessage, WorkerMessage, Frame, encode_line, v2::ToolControl.

Message structure → encode_line → stdio; the receiving side decodes and invokes a concrete host handler. Distinguish codec calls from IPC in diagrams.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

The internal worker-control protocol version (an integer negotiated in `hello`/`selected`, currently 2) and the public Session Endpoint version are unrelated numbers; the supervisor has no stdin control shell.

`Frame.ledger_seq` names the durable prefix a streamed frame may follow, so a
publisher never lets an eager ledger record overtake unread pipe frames. The durable
`tool_control_result` has a `pending` arm (`PendingContinuation`), paired with
the `tool_continuation` / `tool_continuation_result` messages that drive a
remote continuation to its terminal result.

Behavior contract: [worker-control.md](../../../spec/worker-control.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
