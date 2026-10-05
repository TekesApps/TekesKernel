//! Bounded loopback HTTP and WebSocket carrier for Session Endpoint v3.
//!
//! This crate owns carrier mechanics only. Durable idempotency, semantic
//! method dispatch, endpoint-journal ordering, and exact application values
//! remain behind [`endpoint::EndpointCarrierHost`].

mod access;
mod auth;
mod server;
mod web;

pub use access::{AccessLogRecord, AccessLogSink, NoopAccessLog};
pub use auth::BearerToken;
pub use server::{
    FileChangeAuthority, FileChangeFeed, FileTransferAuthority, MAX_REQUEST_BYTES, OriginPolicy,
    ROUTE_REGISTRY, TransportConfig, TransportConfigError, TransportHandle, TransportLimits,
    TransportServer, WebClientConfig, WebClientService,
};
pub use web::WEB_CLIENT_SHA256;
