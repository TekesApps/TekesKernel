# transport

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `AccessLogRecord` | `access::AccessLogRecord` | `pub` |
| `AccessLogSink` | `access::AccessLogSink` | `pub` |
| `NoopAccessLog` | `access::NoopAccessLog` | `pub` |
| `BearerToken` | `auth::BearerToken` | `pub` |
| `FileChangeAuthority` | `server::FileChangeAuthority` | `pub` |
| `FileChangeFeed` | `server::FileChangeFeed` | `pub` |
| `FileTransferAuthority` | `server::FileTransferAuthority` | `pub` |
| `MAX_REQUEST_BYTES` | `server::MAX_REQUEST_BYTES` | `pub` |
| `OriginPolicy` | `server::OriginPolicy` | `pub` |
| `ROUTE_REGISTRY` | `server::ROUTE_REGISTRY` | `pub` |
| `TransportConfig` | `server::TransportConfig` | `pub` |
| `TransportConfigError` | `server::TransportConfigError` | `pub` |
| `TransportHandle` | `server::TransportHandle` | `pub` |
| `TransportLimits` | `server::TransportLimits` | `pub` |
| `TransportServer` | `server::TransportServer` | `pub` |
| `WebClientConfig` | `server::WebClientConfig` | `pub` |
| `WebClientService` | `server::WebClientService` | `pub` |
| `WEB_CLIENT_SHA256` | `web::WEB_CLIENT_SHA256` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `transport::access` | `private` |  |
| `transport::auth` | `private` |  |
| `transport::server` | `private` |  |
| `transport::web` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
