# Client and transport gates

[All gates](README.md) · [Running tests](../running-tests.md)

This page retains the gates' original numbers and applicable baselines. It defines conformance rules, not the results of this run.

## Session Endpoint v2

47. ● **EndpointAuthorityByteSync** — Setup: the committed
    `fixtures/endpoint/authority/` corpus and its pinned Tekes commit/hash
    lock. Action: verify disk in both directions and, in the workspace parity
    lane, compare each byte to the pinned Client-owned tree; decode the unary,
    history, inventory, model, stream and unknown fixtures. Assert: no copied
    drift, exact v2 envelope shapes, native public ids are canonical UUIDs,
    and internal compound ids reject. `[session-endpoint §Canonical
    fixtures; Client interface §Contract import/URI]`
48. ● **StableProjectionJournal** — Setup: the canonical Kernel source ledger
    and expected endpoint journal. Action: project, reopen, replay, and crash
    at append→full-sync→live boundaries. Assert: byte-exact canonical records,
    contiguous seq from zero, stable 1:N slots, duplicate replay reuses seq,
    final partial tail repairs, complete invalid rows fail-stop, and no event
    emits before durability. `[session-endpoint §Durable projection
    journal/§Deterministic projection slots]`
49. ● **ChunkHistoryLiveIdentity** — Setup: an attempt with a durable streamed
    chunk followed by output. Action: retry the stream identity, restart, and
    fetch `maxMessages:1`. Assert: one chunk seq, identical history/live bytes,
    assistant message backward provenance names the chunk, and the complete
    contiguous group returns despite the message limit. `[endpoint-projection-
    v2 §Stream-frame ingestion/§History; Client v2 §7–9]`
50. ● **EndpointMutationArchiveContract** — Setup: empty native endpoint.
    Action: keyed create/prompt retry, archive, history while archived,
    unarchive, and rematerialize. Assert: retry returns the original durable
    seq; admission receipt is independent of history reconcile; archive and
    unarchive are distinct methods/capabilities; archived access is typed
    rejection; the authoritative folder/journal survive and active history is
    available after restore. `[session-endpoint §Typed mutations;
    Thread definition R6; Client interface §Kernel storage adaptations]`
51. ● **InventoryModelReadiness** — Setup: active and archived folders, a
    process-table running set, and a model catalog containing a failure.
    Action: refresh inventory and model readiness independently. Assert:
    stable UUID metadata without transcript reads; archive/running state is
    accurate; catalog failure remains typed and does not erase transcript or
    inventory readiness. `[session-endpoint §Inventory and readiness;
    Client v2 §10]`
52. ● **StitchGapOverlapFailClosed** — Setup: overlap, baseline-ahead, live-gap,
    reconnect, unknown-required/ignorable, and bounded-buffer fixtures.
    Action: run the window/mux fold. Assert: byte-equal overlap dedups, ahead
    and gap request one tail refetch, mismatched overlap and unknown required
    fail closed, `ignorable:true` skips, buffer overflow closes with
    `live-gap` rather than dropping a middle event. `[session-endpoint §Journal projection
    §Mux/§Versions; Client v2 §8–9]`

## Endpoint transport

65. ◎ **EndpointTransportAuthority** — Setup: imported Client-v2 corpus and
    raw HTTP/WebSocket fixtures. Action: round-trip every unary and stream
    shape. Assert: DTO values are unchanged, no AS-v1 method exists, UUID is
    the public id, `host.describe` matches `SessionHostDescription`, and the
    `.tekes` driver catalog equals the executable route registry in both
    directions without adding capability/generation wire fields; all 20
    registrations have exact request, success and error fixtures, every
    incomplete management operation phase recovers to its original response,
    and worker-control queue success/management-startup/rejection/crash-
    retry transcripts are canonical and registry-checked; the shared
    admission→management→line lock order is fixture-locked.
    `[session-endpoint §Closed registration table/§Idempotency and
    operation journal; session-endpoint §Listener/§Negotiation]`
66. ◎ **EndpointHttpErrorAndIdempotency** — Setup: all carrier status cases
    plus a mutation whose reply is lost. Action: send malformed/oversized/
    overloaded requests and retry the mutation on a new connection. Assert:
    exact status/typed error, endpoint-wide rpcId conflict safety, original
    durable receipt on retry, and no carrier auto-retry; a crash after queue
    retraction holds later stop/input deliveries until one reconcile-only
    management-startup worker completes and re-acks the same transaction; an
    under-lock target/steer rejection writes zero events, durably completes the
    typed error and releases the gate; active stop rejects respond as
    `stop-active` without consuming its rpc identity.
    `[session-endpoint §Idempotency and operation journal;
    worker-control §Supervisor to worker: endpoint queue transaction;
    session-endpoint §Unary execution]`
67. ◎ **EndpointSubscribeAtomicity** — Setup: a journal append racing mux
    subscription. Action: crash or disconnect at snapshot → subscribed → drain
    boundaries. Assert: each durable seq appears exactly once after stitch,
    lastSeq is the actual durable baseline, and restart never renumbers.
    `[session-endpoint §Stream handoff; session-endpoint §Mux]`
68. ◎ **EndpointRawHistoryReconnect** — Setup: message-aligned history with
    chunks, overlap, baseline-ahead and live gap. Action: exercise real HTTP and
    WebSocket carriers through the Client window algorithm. Assert: one raw
    page read, chunks retained, history/live byte identity, bounded refetch and
    typed live-gap close. `[session-endpoint §Stream handoff; Client v2
    §7–10]`
69. ◎ **EndpointDrainBackpressureSecurity** — Setup: slow subscribers, slow
    bodies, browser origin, provider/tool secrets and live mutations. Action:
    exceed each bound and drain. Assert: no middle drop, no lock retained while
    writing, typed server-draining/503 or accepted-but-not-confirmed behavior,
    accepted mutation truth is preserved; missing/wrong bearer credentials
    fail 401 before dispatch, valid credentials still undergo Origin checks,
    and logs contain no credential/secret/content. `[session-endpoint §Transport listener
    §Concurrency, draining, and security]`
70. ◎ **EndpointArchiveClientRematerialization** — Setup: active Client Mirror
    and authoritative AS session. Action: archive, attempt access, unarchive,
    then reload history/live over real transport. Assert: archive/unarchive are
    distinct capabilities, archived access rejects, Client active projection
    is deleted, and unarchive rematerializes without seq change. `[endpoint-
    management-v2 §Create, fork, archive and model publication;
    session-endpoint §Transport listener; session-endpoint §Typed mutations; 15
    §Archive]`
