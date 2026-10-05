//! Generic MCP v1 client, catalog projection, and supervisor pool.
//!
//! This crate owns protocol and process mechanics only. Durable tool ordering,
//! approvals, and result settlement remain in `engine`/`tools`.

mod broker;
mod client;
mod management;
mod parameter_headers;
mod pool;
mod projection;
mod recovery;
mod subscription;
pub use subscription::McpSubscriptionFilter;
mod transport;
mod types;

pub use client::{McpClient, McpPeer, McpPeerFuture};
pub use management::{
    CredentialValue, McpCredentialFieldOperation, McpCredentialReference,
    McpCredentialReferenceState, McpManagementFault, McpManagementMutation, McpManagementResult,
    McpMutationReceipt, McpRegistry, McpRegistryError, McpRegistryStore, McpScope, McpServerConfig,
    McpServerReference, McpTransportConfig,
};
pub use pool::{McpPool, McpPoolError, McpPoolKey, McpPoolLease, PooledPeer};
pub use projection::{McpProjectionError, project_catalog, project_name};
pub use recovery::{McpLossState, RecoveryAction, recovery_action};
pub use transport::{HttpRequestAuthorization, HttpTransport, McpTransport, StdioTransport};
pub use types::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpCancellationToken, McpCapabilities,
    McpContent, McpError, McpIcon, McpIconTheme, McpImplementation, McpPrompt, McpResource,
    McpTask, McpTaskSupport, McpTool, McpToolAnnotations, McpToolCallContext, McpToolContinuation,
    McpToolExecution, ProtocolMode, TransportKind,
};

pub const LEGACY_PROTOCOL_VERSION: &str = "2025-11-25";
pub const MODERN_PROTOCOL_VERSION: &str = "2026-07-28";
pub const MAX_FRAME_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_CATALOG_ITEMS: usize = 1_000;
pub const MAX_CATALOG_PAGES: usize = 100;
pub const MAX_CONTINUATION_ROUNDS: u32 = 32;
pub use broker::{McpBroker, McpBrokerHandle};
