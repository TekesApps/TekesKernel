# mcp — MCP client, connection pool and management

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Provides generic MCP communication and tool catalog projection; supervisor::mcp_runtime integrates the production lifecycle.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `client / types` | Protocol negotiation, calls and typed content |
| `transport` | Stdio child processes and HTTP transport |
| `pool / broker` | Connection reuse and runtime bridge |
| `management` | Configuration/credential references and registry |
| `projection / recovery` | Tool-name projection and connection-loss decisions |
| `parameter_headers` | Per-tool projection of call arguments into `mcp-param-*` request headers (modern surface) |
| `subscription` | Modern `subscriptions/listen` streams and their notification routing |
| `bin/mcp-fixture-server` | Test-server executable target |

## Interfaces and calls

Main entry points: McpClient, McpPeer, McpPool, McpBroker, StdioTransport, HttpTransport, McpRegistryStore.

Supervisor resolves server configuration → establish transport/client → pool/catalog → tool call → normalized result.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

MCP host and MCP server are different roles; the engine/tools execution path still owns approvals and result persistence.

`McpTool.execution.taskSupport` (`required`/`optional`/`forbidden`) is projected
from `tools/list`. `McpClient::call_tool_augmented` sends the SEP-1686 task
augmentation (`task.ttl`) for tools that require it, and a task-shaped result
becomes a pending continuation that the supervisor's `mcp_runtime` binds and
polls (`tasks/get`) instead of a terminal tool value. The fixture server's
`task-augmented` scenario exercises both (first poll working with a poll
interval, then completed).

The client adopts the official SEP-1686 task shape (protocol 2025-11-25:
`ttl`, `pollInterval`, status-only `tasks/get`, payload via `tasks/result`)
into the Kernel vocabulary (`ttlMs`, `pollIntervalMs`, inline `result`) on
every task object it returns, so supervisor and worker see one shape. The
public-tunnel gates against the official TypeScript SDK live in
`tests/live_official_sdk_tunnel.rs` (`scripts/run-live-mcp-official-sdk.py`,
`scripts/mcp-official-sdk`).

Auto-mode downgrade evidence includes a JSON-RPC `-32600` unsupported-protocol-
version error that enumerates an older supported version, even when the server
answers with a placeholder response id (public DeepWiki); the HTTP transport
surfaces that body as the remote error rather than a bare HTTP 400.

Behavior contract: [mcp-runtime.md](../../../spec/mcp-runtime.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
