# Endpoint transport oracle coverage

The 54 registered cases map to Slice-9 gates as follows. A case may support
more than one gate; every registry entry has a primary executable home here.

## Gate 65 — EndpointTransportAuthority

- `http/host-describe`
- `http/history`
- `http/mutation-receipt`
- `http/typed-error`
- `http/respond`
- `management/host-describe`
- `management/workspace-list`
- `management/workspace-create`
- `management/workspace-rename`
- `management/workspace-relocate`
- `management/workspace-archive-session`
- `management/workspace-unarchive-session`
- `management/session-list`
- `management/session-create`
- `management/session-history`
- `management/session-prompt`
- `management/session-update-queue`
- `management/session-cancel`
- `management/session-rename`
- `management/session-fork`
- `management/session-discard`
- `management/session-attachment`
- `management/session-models`
- `management/session-select-model`
- `management/events-mux`
- `management/events-host`
- `management/respond`

## Gate 66 — EndpointHttpErrorAndIdempotency

- `http/malformed`
- `http/wrong-path`
- `http/wrong-method`
- `http/wrong-content-type`
- `http/oversized`
- `http/overloaded`
- `http/not-ready`
- `http/rpc-retry`
- `http/rpc-conflict`
- `http/accepted-but-not-confirmed`
- `http/attachment-validation`

## Gate 67 — EndpointSubscribeAtomicity

- `websocket/subscribe-baseline`

## Gate 68 — EndpointRawHistoryReconnect

- `websocket/history-live-identity`
- `websocket/chunk-retention`
- `websocket/overlap`
- `websocket/gap-refetch`
- `websocket/generation-change`

## Gate 69 — EndpointDrainBackpressureSecurity

- `websocket/backpressure-live-gap`
- `websocket/server-draining`
- `websocket/unknown-required`
- `security/browser-origin-reject`
- `security/credential-redaction`
- `security/loopback-bind`
- `security/api-auth-origin-matrix`

## Gate 70 — EndpointArchiveClientRematerialization

- `http/archive`
- `http/archived-reject`
- `http/unarchive`
- `client/archive-rematerialize`

Cross-gate assertions retain Client-owned DTO bytes unchanged, enforce the
closed carrier/semantic error tables, and keep archive/unarchive as distinct
operations. The 20 `management/` cases additionally cover every registration's
exact request, success and typed failure; `management-requests.canonical.jsonl`
locks the ledger-derived approval/question requested/resolved correlation. `scripts/check-endpoint-transport-fixtures.py` validates registry
parity, transcript grammar, canonical JSON, exact expected unions, body/header
lengths, close/status agreement, normalized secret-free access logs, and this
coverage list.

The root management contract fixtures additionally lock `Workspace.path`
creation resolution; recovery after archive `carriers`, config-digest drift,
and active-folder overflow; and rewrite carrier scan/drop of the endpoint
journal. The model-selection fixture locks later-spawn-only visibility.
The summary/model-catalog fixtures lock inventory bytes; the rpc-carrier and
respond-authoring fixtures lock exact retry and question-answer event bytes.
`management-control-races.canonical.json` locks the cross-route admission
order, queue rejection gate release, and stop-before-answer behavior.
The request oracle and mux transcript cover allowed-once, rejected, answered,
and cancelled resolutions; the respond-rejection fixture locks no-cache
semantics for every bare negative receipt. The attachment and auth/Origin
registered cases freeze validation and security precedence from raw carriers.
