# Client: interfaces, progress output and final delivery

[Users and protocols · documentation home](../README.md) · [Execution and settlement](../flows/turn.md) · [Next: exact V3 contract](../../spec/session-endpoint.md#routes-and-streams)

## Current public boundary

The public authority is [Session Endpoint](../../spec/session-endpoint.md#routes-and-streams).
It replaces public V2 list/history, dual WebSocket and respond endpoints:

| Operation | Current V3 carrier |
|---|---|
| Unary mutations/catalog selection | 16 methods in the V3 base registry; extensions are registered separately |
| Workspace and session inventory | logical streams on `/api/remote.mux` |
| Journal baseline/live events | `session-journal` logical stream |
| Older journal pages | `journal-page` on the same mux connection |
| Queue/jobs/projection state | `session-control` logical stream |
| Approval/question and response | `actionables` stream and `actionable-respond` frame |

Compatibility is integer `protocolVersion == 3` plus the required capabilities.
`endpoint.jsonl` is the internal projection journal; it is not a public
protocol declaration. Kernel seq, durable endpoint seq and transient presentation
frames retain distinct roles. V3 transient frames never advance the durable cursor.

Implementation: [transport routes](../../crates/transport/src/server.rs),
[endpoint carrier](../../crates/endpoint/src/carrier_adapter.rs) and [Client wire frames](../../crates/endpoint/src/client_wire.rs), and
[production host/streams](../../crates/supervisor/src/endpoint_carrier.rs).
The full request/output path is in [Code architecture](../architecture/README.md).

## UI projection

`(event prefix) → view state`, incremental over a seq cursor.

- Must apply `supersedes` — tail consumers are *not* pure appenders; retry,
  rollback, and compact retract earlier visual state. The client keeps its
  existing thin ProjectionEngine; raw-tail rendering is forbidden.
- Unknown kinds render as fallback rows (title + disclosure), never dropped.
- Streaming frames never enter the semantic thread projection. Before an
  endpoint accepts them they may live in the supervisor ring buffer; accepted
  durable endpoint frames live in session-endpoint §Journal projection's protocol carrier so
  history/live identity survives restart. The Client keeps them transient in
  UI storage and shadows them when the settled assistant message arrives.

### Process output and final delivery

[Thread fundamentals](../concepts/thread.md#progress-output-candidates-and-settlement) separates sink, candidate and settlement
by purpose, not by transport. Tool call/result pairs and nonfinal assistant
messages are process output; status is a projection over those facts.
Candidate assistant messages also travel through the endpoint with
`sessionFinal=true`, but that flag does not close the root turn. Only `settle`
projects `turn/end`; validation adds `promotedMessageID` and `validationOutcome`
for the exact selected candidate. Clients must not promote the last message by
arrival order, or interpret `completed + inconclusive` as verified success.
The precise mapping is [session-endpoint §Journal projection](../../spec/session-endpoint.md#journal-projection),
reused by the current V3 public boundary.

Generic tool progress is not an established end-to-end event path. The MCP
transport recognizes progress notifications but does not retain their payload
in that notification record. Provider streaming and goal progress are different
concepts. Adding user-visible tool progress requires its own complete mapping;
the optional `progress?` in the domain model does not advertise it today.

## Diagnostics

Digester/narration/progress views are downstream projections computed by their
consumers (UI or a side process), never by the worker. The worker's only
outputs are events and mirror frames.
