# Original section destinations

[Current overview](../README.md) · [Reorganization verification](../verification/document-reorganization.md)

The following map traces old references; it is not a reading order. All 226 top-level content blocks from the 22 numbered documents
(including introductions) have been assigned destinations. Bodies no longer use old chapter numbers and are split by reader questions or combined with related topics.
New contextual navigation replaces duplicated old navigation. Gate, decision and protocol version numbers retain their meaning.

| Original document | Current body location |
|---|---|
| `docs/00-overview.md` | [TekesKernel: overview and reading path](../README.md); [Predecessor systems and migration background](migration.md) |
| `docs/01-thread-folder.md` | [Thread fundamentals](../concepts/thread.md); [Data storage: directories, execution logs and assets](../data/storage.md) |
| `docs/02-event-schema.md` | [Events: identity, input and results](../data/events.md); [Durability, replay and settlement](../data/durability.md) |
| `docs/03-worker.md` | [Worker: startup, waiting and recovery](../runtime/worker.md); [A turn: execution, validation and delivery](../flows/turn.md); [Delegation: parent-child relationships and independent contexts](../flows/delegation.md) |
| `docs/04-supervisor.md` | [Supervisor: hosting, delivery and reaping](../runtime/supervisor.md) |
| `docs/05-projections.md` | [Model context: projection, epochs and compaction](../data/context.md); [Client: interfaces, progress output and final delivery](../interfaces/client.md); [Delegation: parent-child relationships and independent contexts](../flows/delegation.md); [Data storage: directories, execution logs and assets](../data/storage.md) |
| `docs/06-tools.md` | [Tools: invocation, execution and results](../runtime/tools.md); [Tool permissions: isolation, approvals and hooks](../runtime/tool-permissions.md) |
| `docs/07-topology.md` | [A turn: execution, validation and delivery](../flows/turn.md); [Delegation: parent-child relationships and independent contexts](../flows/delegation.md) |
| `docs/08-portability.md` | [Platforms and portability boundaries](../architecture/platform.md) |
| `docs/09-migration.md` | [Predecessor systems and migration background](migration.md) |
| `docs/10-decisions.md` | [Design decision records](decisions.md) |
| `docs/11-context-projection.md` | [Model context: projection, epochs and compaction](../data/context.md) |
| `docs/12-idempotency.md` | [Idempotency, admission and retries](../data/idempotency.md) |
| `docs/13-evolution.md` | [External evolution and extension boundaries](evolution.md) |
| `docs/14-conformance.md` | [Conformance gates by topic](../verification/gates/README.md); [Core execution and tool gates](../verification/gates/core.md); [Client and transport gates](../verification/gates/client.md); [Provider and dialect gates](../verification/gates/providers.md); [Tool, plugin and MCP gates](../verification/gates/tools.md); [Deployment and external pipeline gates](../verification/gates/deployment.md); [Recovery, data and context gates](../verification/gates/recovery-and-data.md); [Historical stage-to-gate mapping](gate-rollout.md) |
| `docs/15-client-endpoint.md` | [Client: interfaces, progress output and final delivery](../interfaces/client.md); [Client V2: historical boundaries and storage mappings](client-v2.md) |
| `docs/16-implementation-plan.md` | [Phased implementation plan](implementation-plan.md) |
| `docs/17-rust-implementation.md` | [Rust: toolchain and implementation constraints](../architecture/rust.md); [Running tests and checks](../verification/running-tests.md); [Rust implementation milestones](rust-rollout.md) |
| `docs/18-feature-parity.md` | [Feature parity and qualification](../verification/feature-parity.md) |
| `docs/19-provider-dialect-parity.md` | [Provider parity and qualification](../verification/provider-parity.md) |
| `docs/20-web-client.md` | [Web Client](../interfaces/web.md) |
| `docs/21-code-architecture.md` | [Code architecture: process → crate → module → function](../architecture/README.md) |

See the [machine map](document-map.json) for complete destinations by original section.

The 21 crate guides formerly centralized under `docs/architecture/crates/` now live in their respective
`crates/<name>/docs/README.md` files; generated API/module/function diagrams moved alongside them to
`crates/<name>/docs/generated/`. Project dependency graphs and the complete machine inventory remain cross-crate architecture resources.
