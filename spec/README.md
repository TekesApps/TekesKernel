# Exact contract index

[Documentation reading path](../docs/README.md) → data and protocols layer.
Read the [Thread domain model](../docs/concepts/thread.md) first, then enter the relevant topic below.
These files define exact contracts; see the [Thread alignment record](../docs/verification/thread-alignment.md) for implementation and verification status.

The Client boundary is [Session Endpoint](session-endpoint.md); its
`protocolVersion` integer is negotiated on the wire and appears in no file
name. Do not infer support for a public capability from a filename.

## Thread data and lifecycle

| Contract | Scope |
|---|---|
| [event](event.md) | Semantic events, identity, turns, output, settle and reference constraints |
| [tail-lifecycle](tail-lifecycle.md) | Log-tail and writer-lock classification; authoritative state machine for running, parked, recovery and settled states |
| [rewrite-publication](rewrite-publication.md) | Fork, redact and durable publication |
| [thread-search](thread-search.md) | Rebuildable thread title search |

## Model execution and context

| Contract | Scope |
|---|---|
| [provider-runtime](provider-runtime.md) | Provider calls, final candidates, validation/repair and root-turn settlement |
| [provider-adapter](provider-adapter.md) | Base adapter interfaces and recovery semantics |
| [provider-dialect-profiles](provider-dialect-profiles.md) | Dialect and model profiles |
| [config](config.md) | Configuration identities and snapshots |
| [instruction-snapshot](instruction-snapshot.md) | Instruction discovery, snapshots and scope |
| [launch-bindings](launch-bindings.md) | Runtime bindings fixed at launch |

## Process and client communication

| Contract | Scope |
|---|---|
| [worker-control](worker-control.md) | Internal supervisor/worker control protocol (negotiated version integer; base and durable control messages) |
| [session-endpoint](session-endpoint.md) | The Client boundary: routes and streams, journal projection, transport listener, management authority and recovery; start here |
| [client-extensions](client-extensions.md) | Product extension capabilities and management semantics; V3 governs public entry points |

## Tool calls and external capabilities

| Contract | Scope |
|---|---|
| [builtin-tools](builtin-tools.md) | Built-in tool names, arguments and results, including verify |
| [tool-runtime](tool-runtime.md) | Tool execution, results and recovery |
| [tool-hook](tool-hook.md) | Before/after tool hooks and effective calls |
| [exec-helper](exec-helper.md) | Helper protocol |
| [sandbox-profile](sandbox-profile.md) | Execution isolation and permissions |
| [mcp-runtime](mcp-runtime.md) | MCP client, transport, lifecycle and capabilities |
| [web-tools](web-tools.md) | Web tool behavior |
| [web-search-provider](web-search-provider.md) | Search provider contract |
| [skill-package](skill-package.md) | Skill packages and resources |
| [command-catalog](command-catalog.md) | Command catalog and execution |
| [plugin-package](plugin-package.md) | Plugin packages, installation and lifecycle |
| [schedule](schedule.md) | Durable scheduling |
| [lifecycle-hook](lifecycle-hook.md) | Lifecycle hook protocol v2: frozen command bindings for `turn.before`, `context.prepare`, `tool.completed`, `context.before_compact` and `turn.settled` |

## Credentials and independent permission boundaries

| Contract | Scope |
|---|---|
| [secret-store](secret-store.md) | Application-owned launch credentials: immutable environment-captured store, runtime interface and connection authentication |
| [credential-broker](credential-broker.md) | Credential authorization and delivery |

## Rules for changes and explanations

Domain terminology links to [Thread fundamentals](../docs/concepts/thread.md); fields/state machines belong to their topic specs, call relationships to code architecture,
and runtime evidence to verification records. Behavior changes must also be checked against implementation and relevant tests; explanatory text must not
hide implementation discrepancies. Add new contracts to this index; topic guides link to their contracts without copying the full text.
