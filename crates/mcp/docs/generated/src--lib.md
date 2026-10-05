# mcp

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [mcp::LEGACY_PROTOCOL_VERSION](../../src/lib.rs#L36) | const_item | `pub` |  |
| [mcp::MODERN_PROTOCOL_VERSION](../../src/lib.rs#L37) | const_item | `pub` |  |
| [mcp::MAX_FRAME_BYTES](../../src/lib.rs#L38) | const_item | `pub` |  |
| [mcp::MAX_CATALOG_ITEMS](../../src/lib.rs#L39) | const_item | `pub` |  |
| [mcp::MAX_CATALOG_PAGES](../../src/lib.rs#L40) | const_item | `pub` |  |
| [mcp::MAX_CONTINUATION_ROUNDS](../../src/lib.rs#L41) | const_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `McpSubscriptionFilter` | `subscription::McpSubscriptionFilter` | `pub` |
| `McpClient` | `client::McpClient` | `pub` |
| `McpPeer` | `client::McpPeer` | `pub` |
| `McpPeerFuture` | `client::McpPeerFuture` | `pub` |
| `CredentialValue` | `management::CredentialValue` | `pub` |
| `McpCredentialFieldOperation` | `management::McpCredentialFieldOperation` | `pub` |
| `McpCredentialReference` | `management::McpCredentialReference` | `pub` |
| `McpCredentialReferenceState` | `management::McpCredentialReferenceState` | `pub` |
| `McpManagementFault` | `management::McpManagementFault` | `pub` |
| `McpManagementMutation` | `management::McpManagementMutation` | `pub` |
| `McpManagementResult` | `management::McpManagementResult` | `pub` |
| `McpMutationReceipt` | `management::McpMutationReceipt` | `pub` |
| `McpRegistry` | `management::McpRegistry` | `pub` |
| `McpRegistryError` | `management::McpRegistryError` | `pub` |
| `McpRegistryStore` | `management::McpRegistryStore` | `pub` |
| `McpScope` | `management::McpScope` | `pub` |
| `McpServerConfig` | `management::McpServerConfig` | `pub` |
| `McpServerReference` | `management::McpServerReference` | `pub` |
| `McpTransportConfig` | `management::McpTransportConfig` | `pub` |
| `McpPool` | `pool::McpPool` | `pub` |
| `McpPoolError` | `pool::McpPoolError` | `pub` |
| `McpPoolKey` | `pool::McpPoolKey` | `pub` |
| `McpPoolLease` | `pool::McpPoolLease` | `pub` |
| `PooledPeer` | `pool::PooledPeer` | `pub` |
| `McpProjectionError` | `projection::McpProjectionError` | `pub` |
| `project_catalog` | `projection::project_catalog` | `pub` |
| `project_name` | `projection::project_name` | `pub` |
| `McpLossState` | `recovery::McpLossState` | `pub` |
| `RecoveryAction` | `recovery::RecoveryAction` | `pub` |
| `recovery_action` | `recovery::recovery_action` | `pub` |
| `HttpRequestAuthorization` | `transport::HttpRequestAuthorization` | `pub` |
| `HttpTransport` | `transport::HttpTransport` | `pub` |
| `McpTransport` | `transport::McpTransport` | `pub` |
| `StdioTransport` | `transport::StdioTransport` | `pub` |
| `JsonRpcError` | `types::JsonRpcError` | `pub` |
| `JsonRpcRequest` | `types::JsonRpcRequest` | `pub` |
| `JsonRpcResponse` | `types::JsonRpcResponse` | `pub` |
| `McpCancellationToken` | `types::McpCancellationToken` | `pub` |
| `McpCapabilities` | `types::McpCapabilities` | `pub` |
| `McpContent` | `types::McpContent` | `pub` |
| `McpError` | `types::McpError` | `pub` |
| `McpIcon` | `types::McpIcon` | `pub` |
| `McpIconTheme` | `types::McpIconTheme` | `pub` |
| `McpImplementation` | `types::McpImplementation` | `pub` |
| `McpPrompt` | `types::McpPrompt` | `pub` |
| `McpResource` | `types::McpResource` | `pub` |
| `McpTask` | `types::McpTask` | `pub` |
| `McpTaskSupport` | `types::McpTaskSupport` | `pub` |
| `McpTool` | `types::McpTool` | `pub` |
| `McpToolAnnotations` | `types::McpToolAnnotations` | `pub` |
| `McpToolCallContext` | `types::McpToolCallContext` | `pub` |
| `McpToolContinuation` | `types::McpToolContinuation` | `pub` |
| `McpToolExecution` | `types::McpToolExecution` | `pub` |
| `ProtocolMode` | `types::ProtocolMode` | `pub` |
| `TransportKind` | `types::TransportKind` | `pub` |
| `McpBroker` | `broker::McpBroker` | `pub` |
| `McpBrokerHandle` | `broker::McpBrokerHandle` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `mcp::broker` | `private` |  |
| `mcp::client` | `private` |  |
| `mcp::management` | `private` |  |
| `mcp::parameter_headers` | `private` |  |
| `mcp::pool` | `private` |  |
| `mcp::projection` | `private` |  |
| `mcp::recovery` | `private` |  |
| `mcp::subscription` | `private` |  |
| `mcp::transport` | `private` |  |
| `mcp::types` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
