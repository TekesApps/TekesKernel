# endpoint — client semantics and durable carriers

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Maps ledger events and worker frames into client events, and handles management transactions, requests and history synchronization.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `rpc / host / service` | Request validation, host interfaces and idempotent dispatch |
| `management / idempotency / attachment` | Session/workspace transactions, request identities and attachments |
| `projection / journal / history / stitch` | Event projection, durable carriers, pagination and stitching |
| `request_journal / respond` | Recoverable pending requests and response lifecycle |
| `hub / all_session / stream_queue` | Subscriptions, history/live handoff and bounded streams |
| `carrier_adapter / v3 / types / session_event_registry_generated` | V3 carrier composition, protocol types and event registry |

## Interfaces and calls

Main entry points: EndpointCarrierHost, EndpointHost, Projector, EndpointJournal, RequestJournal, MuxHostDescription.

Unary requests pass through host/service to supervisor authorities; worker events pass through Projector/EndpointJournal to v3 journal/control/inventory streams.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

The public protocol is v3; v2 carrier file identities and old internal types remain. Endpoint does not depend back on supervisor; concrete implementations are injected through traits.

`tests/legacy_replay.rs` (ignored, `TEKES_LEGACY_REPLAY_EXPORT`) imports the
legacy AppServer corpus export produced by `scripts/replay-legacy-sqlite.py`
as Kernel ledgers and proves validation, journal projection, history paging
and idempotence over real production threads.

`NativeEndpoint::author_keyed_with_ledger` exposes that ledger-owning keyed
append to the host (origin target must be the session); the host validates the
origin operation.

Behavior contract: [session-endpoint.md](../../../spec/session-endpoint.md#routes-and-streams).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
