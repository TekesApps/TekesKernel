# spec: credential broker

This contract is the only path by which a worker receives provider or tool
credentials. Configuration contains credential ids, never secret values. The
supervisor brokers material but does not render or send provider requests. The
supervisor resolves those ids and freezes exact scopes under
[secret-store](secret-store.md); the broker never queries Keychain.

## Launch channel

Before spawning a worker, the supervisor creates a private `AF_UNIX`
`SOCK_STREAM` socket pair and passes the worker end's nonnegative descriptor
number as `--credential-fd <n>`, alongside the existing snapshot descriptor
arguments. The descriptor is inherited across the worker exec, immediately
marked close-on-exec by the worker, never inherited by helpers/children, and is
absent when no credential-using capability is exposed. Failure to validate the
descriptor is a pre-lock exit 76.
For an active configured [web-search provider](web-search-provider.md), the
supervisor additionally supplies the non-secret `--web-search-ready true`
launch option. The option is valid only with a broker descriptor and exact
active search scope; the worker rejects the inconsistent launch before taking
the line lock.

Each message is a four-byte unsigned big-endian byte length followed by exactly
that many UTF-8 bytes containing one I-JSON object; maximum body length is
65536 and zero is invalid. Exactly one top-level key is allowed. Partial reads,
EINTR and coalesced frames are handled by the decoder. The channel is private
transport, not a ledger or log; implementations must not trace packet bodies.

## Protocol

The channel is request/response only. The worker asks once per request (a
provider attempt or a tool call) and the supervisor answers once; there is no
lease identity, no release message and no push from the supervisor.

Worker to supervisor:

```text
credential_get {request_id, attempt, credential_id, purpose: "provider" | "web_search",
                adapter, endpoint_origin}
```

Supervisor to worker:

```text
credential {request_id, material, generation}
credential_error {request_id, code: "not_found" | "revoked" |
                  "scope_mismatch" | "unavailable"}
```

`attempt` is required for provider use and is `"tool:" || call-id` for
`web_search`. `request_id` is lowercase-hex SHA-256 of
`"tekes-credential-v1\0" || attempt-id UTF-8 || 0x00 || credential-id UTF-8 ||
0x00 || endpoint-origin UTF-8` (the domain string is a hash preimage constant,
not a protocol name). `endpoint_origin` is lowercase scheme plus the configured
authority and explicit port, with no path, query, fragment, userinfo, or
redirect target. The supervisor verifies that configuration authorizes the
credential for the exact adapter/origin before returning material.

Every message payload has the exact closed field set shown above. Unknown
fields, unknown purpose/error strings, empty ids/generations and malformed
request ids are protocol errors. The error-code union is exactly
`not_found | revoked | scope_mismatch | unavailable`. A credential id may
authorize multiple configured adapter/origin/purpose tuples; the broker keeps
all of them, performs exact tuple lookup, and rotates/revokes every tuple for
that id. Authorization is never a single-value map keyed only by credential id.

Identical requests return the same answer; different bytes under one request
id are a protocol error. Rotation publishes a new generation and revocation
closes the id: both reach the next `credential_get`, which answers with the new
material or `revoked`. A request already answered keeps the material it holds
until its attempt settles; the supervisor never interrupts a dispatched
request on the credential channel. This is the per-request resolution model of
the predecessor: the worker resolves at dispatch and holds nothing across
requests. A revoked/closed channel before dispatch forbids send. EOF or
supervisor death after a possible dispatch is handled by the control channel's
maybe-sent recovery path; it never proves `not_dispatched` by itself.

During supervisor refresh, a transient authority `unavailable` observation is
unknown rather than revocation evidence. The live worker keeps its last
authoritative active/revoked generation and is neither rotated nor terminated;
only a later authoritative generation advance or `not_found` changes ownership.

The worker holds material only in owned memory for the request that obtained
it, zeroes that buffer when the request settles (the material type zeroes on
drop), and never places it in hooks, provider fixtures, events, assets,
command lines, environment, diagnostics, or support bundles.

## Oracle

`fixtures/credential-broker/` contains packet transcripts for success,
missing, revoked, rotation, scope mismatch, dedup/conflict, malformed/oversized
packet, EOF, shared-key multi-scope and unknown field. Fixtures use the
sentinel `fixture-secret-never-log`; capture tests assert it appears only in
the private `credential` packet.
