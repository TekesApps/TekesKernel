# transport — HTTP and WebSocket transport

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Provides loopback HTTP, authentication, rate limiting, draining, v3 mux and the optional browser service inside the supervisor process.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `server` | TransportServer, unary, remote_mux_loop, WebClientService |
| `auth` | BearerToken and comparison |
| `access` | Access-log interface |
| `web` | Embedded assets and browser access policy |

## Interfaces and calls

Main entry points: TransportServer, TransportConfig, WebClientService, WebClientConfig, BearerToken.

router → unary_inner → EndpointCarrierHost::unary; remote_mux_loop → v3 stream/page/actionable methods.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Native uses /api/remote.mux; browser uses /web/remote.mux. Request reads and transport timeouts do not directly establish whether a domain operation has become durable.

Behavior contract: [session-endpoint.md](../../../spec/session-endpoint.md#routes-and-streams).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
