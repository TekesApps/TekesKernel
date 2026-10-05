# Web Client

[Users and protocols · documentation home](../README.md) · [Client boundary](client.md) · [Next: transport implementation](../../crates/transport/docs/README.md)

TekesKernel can expose the Tekes experience through a browser without adding
methods, DTOs, events, or capabilities to Session Endpoint v3. The Web Client
is a separately bound HTTP service over the already assembled endpoint host.
It is disabled by default.

## Runtime boundary

The native endpoint remains fixed at `127.0.0.1:7347` and owns only its
existing health and `/api` routes. Constructing or starting the native
`TransportServer` does not create a browser listener, and `/web/*` on the
native listener remains absent.

An operator may explicitly add this ordered suffix to the resident selector
`serve` command:

```text
--web-listen 127.0.0.1:7357
```

The selector passes that value to the selected supervisor. The supervisor
binds `127.0.0.1:7357` before reporting bootstrap success and runs an optional
`WebClientService` beside the native listener. If the suffix is absent, no Web
socket is bound and no Web task is started. Either enabled listener failing
causes the shared supervisor lifetime to drain; an ordinary shutdown drains
both.

The shipped LaunchAgent intentionally omits the suffix, so installation and
upgrade preserve the default-off posture. A future Tekes settings action can
choose the enabled selector invocation without changing the Session Endpoint
contract.

## Browser surface

The service embeds and serves the production HTML, CSS, and ES module assets.
It maps browser-only paths to the same in-process endpoint handlers:

| Browser path | Existing endpoint operation |
|---|---|
| `/web/api/{method}` | the named unary method |
| `/web/remote.mux` | V3 multiplexed workspace/inventory/journal/control/actionables streams |

This is an HTTP adapter, not a second application protocol. Request and
response envelopes, method names, event ordering, history pagination,
idempotency, drain behavior, and WebSocket stream semantics remain the
[Session Endpoint](../../spec/session-endpoint.md#routes-and-streams) authority bytes.

## Local access and origin policy

The Web Client is directly available at its configured loopback URL; it does
not require a bearer, launch token, or browser cookie. Browser JavaScript never
receives the installation bearer used by the native endpoint.

The service requires the exact configured `Host`. Browser data requests also
require the exact service `Origin`, preventing an unrelated website from using
the browser to call the local Kernel. The index sends a restrictive Content
Security Policy and is never cached; static assets are immutable.

The current service intentionally accepts loopback only. It provides the same
local-browser deployment posture as `dsh web`; LAN or public exposure requires
a separately designed TLS and remote-authentication boundary.

## Client state model

The Web Client connects to `/web/remote.mux`, validates V3 readiness, then opens
workspace, inventory, control and actionables streams. Selecting a session opens
its bounded `session-journal` snapshot/live stream. Older history uses
`journal-page`; approvals/questions use `actionable-respond` on that same mux.
Each logical stream has a generation-bound baseline. Journal transients do not
advance the durable cursor. Source: [browser app](../../crates/transport/web/app.js)
and [transport routes](../../crates/transport/src/server.rs).
Prompt submission, cancellation, model selection, fork/archive actions and
drain/disconnect presentation stay inside the same endpoint semantics.

## Product integrity

The three embedded source assets have a canonical per-file manifest in
`fixtures/web-client/manifest.canonical.json`. The combined digest is compiled
into the transport crate, checked by `scripts/check-web-client-assets.py`, and
the canonical manifest is copied into the signed supervisor app as
`Contents/Resources/WebClientManifest.canonical.json`. That file is also an explicit row
of the outer product manifest, so signing, selection, upgrade, rollback, and
uninstall treat the Web Client as product content.
