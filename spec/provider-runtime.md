# spec: provider runtime v1

This contract closes the protocol-family production-provider seam used by
Slice 7. It extends, and never replaces,
[provider-adapter](provider-adapter.md): that contract owns recovery decisions;
this contract owns family codecs, HTTP/SSE execution, normalized frames, and
terminal material. [provider-dialect-profiles](provider-dialect-profiles.md)
is the authority for exact vendor/model/route selection, controls, request
projection and advertisement.

## Scope and authority

The worker owns provider turns. The supervisor owns only the admission lease
service and the private [credential broker](credential-broker.md); neither may render a prompt, interpret a
provider response, or author semantic outcomes. Provider configuration comes
from [config](config.md), context from event plus doc 11, and the
attempt/send/recovery order from provider-adapter and event.

The v1 family-codec registry is closed to these adapter ids:

| adapter id | protocol family | continuation |
|---|---|---|
| `responses` | OpenAI Responses (`openai_responses`) | server-managed; query-by-identity is absent in v1 |
| `anthropic` | Anthropic Messages (`anthropic_messages`) | stateless |
| `google_generation` | Google generateContent/streamGenerateContent | stateless |
| `google_interactions` | Google Interactions | server-managed; query-by-identity is absent in v1 |
| `chat_completion` | OpenAI-compatible chat completions, including DeepSeek-compatible endpoints | stateless |

These ids are implementation lineage, not advertisable identities. An unknown
family makes the provider/model unavailable. Endpoint hostname heuristics never
select a family, dialect or capability. The exact target tuple comes from
provider-dialect-profiles; in particular `deepseek_responses_v1` uses the
Responses family codec with stateless full-history behavior and may never be
routed through the server-managed `responses` target. Every advertised
capability and route-evidence revision is bound into the epoch profile.

An exact model capability may define `wire_model` for a local variant and
`pro_reasoning` for OpenAI Responses. The Sol Pro local variant sends the base
Sol wire model with `reasoning.mode = "pro"`, independently of reasoning effort.
The local selection identity remains distinct; arbitrary model suffixes are
never stripped to infer a wire model.

## Provider invocation seam

The worker calls one language-neutral seam:

```text
prepare(attempt_id, adapter, model, epoch_profile, continuation_id?, rendered_items, tool_catalog)
  -> PreparedRequest {
       method, url, headers_without_secret, body,
       candidate_tokens, dispatch_marker_required,
       request_digest, query_key?
     }

send(prepared, credential_material, cancellation)
  -> ordered ProviderFrame* then ProviderCompletion

query(query_key, credential_material, cancellation)
  -> provider-adapter QueryResult
```

`body` is the exact transmitted byte string. `prepare` is pure: identical
inputs produce identical method, URL, non-secret headers, and body bytes.
The worker publishes `body` as a content-addressed thread asset and the
`attempt` event references it as `request: {asset, bytes}` (event §attempt):
the rendered request is durable history, not only its digest, so context,
cost and cache attribution can be re-read from the ledger without re-rendering
(the predecessor keeps the routed request the same way). Rewrites never carry
`attempt` rows, so the asset never enters a fork or redact destination, and a
redact scan that finds forbidden bytes in it poisons it like any other asset.
`continuation_id` is present only for a server-managed adapter and is copied to
that dialect's native continuation field (`previous_response_id` or
`previous_interaction_id`); it is never inferred from endpoint text.
`request_digest` is the lowercase hexadecimal digest defined below. For an
adapter declaring query-by-identity, `query_key` is REQUIRED and is the
lowercase hexadecimal SHA-256 of
`"tekes-provider-query-v1\0" || attempt-id UTF-8`. The adapter binds that exact
key to a provider-supported client-supplied idempotency/query field before the
final request bytes and `request_digest` are computed. It is therefore
reconstructible from the durable, file-unique `attempt.attempt` without a
digest/query-field cycle; `attempt.wire_digest` independently binds the exact
transmitted request. A response id learned only after dispatch never qualifies
as recovery query identity.

`candidate_tokens` is required (`PreparedRequest.candidate_bytes` in code).
Provider runtime v1 retains the deterministic `body.len()` byte bound; it does
not guess from a model name or depend on a tokenizer package. For D-39
preflight, the worker first checks this bound against `compact_trigger_tokens`.
If the bound crosses the trigger and a prior request in the same epoch has
provider-reported input usage, the worker verifies that the prior request is
an exact prefix of the candidate and uses the reported tokens plus one token
per newly serialized byte and a framing margin. If the observation or prefix
cannot be verified, it falls back to the byte bound. The provider's context
overflow response remains the hard authority. Changing the adapter's
`candidate_bytes` value itself is a versioned adapter change whose fixtures
must bind the new value.

Authorization material is obtained through the credential broker after fixture comparison, injected at the last
responsible HTTP boundary, and is never
returned, logged, or placed in an event/asset. Redirects are disabled unless
the configured endpoint explicitly permits one same-origin redirect; a
cross-origin redirect is `transport` failure.

The worker computes `request_digest` over:

```text
lowercase-hex(sha256("tekes-provider-request-v1\0" UTF-8 ||
       adapter-id UTF-8 || 0x00 || uppercase-method ASCII || 0x00 ||
       final-url UTF-8 || 0x00 || model UTF-8 || 0x00 ||
       JCS(headers_without_secret) || 0x00 || body-bytes))
```

`final-url` is the exact absolute URL returned by `prepare`, before any
permitted same-origin redirect; no URL normalization is performed by the
digest step. `headers_without_secret` is the adapter-returned I-JSON object
with lowercase header names; duplicate names are invalid. The
`attempt.wire_digest` MUST equal this value. Attempt identity
remains the event authority. The fixture registry contains the exact
preimage components and expected lowercase-hex result.

## Send ordering and cancellation

The order is fixed:

1. acquire the supervisor admission lease;
2. obtain the credential material required by the configured adapter
   (one `credential_get` keyed by this attempt);
3. publish the exact request body as a thread asset, then append and
   barrier-sync `attempt` referencing it;
4. when required by the adapter, append and barrier-sync
   `attempt_dispatched`;
5. inject the credential at the HTTP boundary and begin HTTP;
6. decode frames without appending provider partials to the semantic ledger;
7. receive one `ProviderCompletion`;
8. append the outcome (carrying the attempt's usage) as one barrier transaction;
9. report `attempt_settled`, release the admission lease, and drop (zero)
   the credential material.

`ProviderCompletion` is the closed union `terminal(ProviderTerminal) |
failure(auth_failure | rate_limited | transport | malformed | cancelled)`. `send` produces
exactly one completion; a local cancellation is the `cancelled` failure arm,
never a fabricated provider terminal.

Cancellation closes the socket/body stream, stops decoding, and follows the
same durable recovery classification as an equivalent transport loss. EOF,
supervisor death, or worker SIGTERM cannot fabricate `not_dispatched` after
step 5. A worker must hold the admission lease and this attempt's credential
material at step 5; credential rotation or revocation reaches the next
attempt's get, and a dead supervisor causes the worker's control EOF path.

## HTTP and streaming limits

- Connect timeout: fixed at 30 seconds in v1.
- Response-idle timeout: fixed at 120 seconds in v1; each complete
  protocol frame resets it.
- Whole-attempt wall limit: the remaining turn budget derived from
  `config`'s effective `max_wall_seconds`; absence means no additional
  whole-turn wall cap. Retries and compaction never reset the deadline.
- Maximum response frame: 8 MiB; maximum accumulated terminal asset: 64 MiB.
- HTTP 401/403 is `auth_failure`; 429, and the Cloudflare AI Gateway wholesale
  limit (HTTP 402 whose body reports "Wholesale rate limit exceeded"), are
  `rate_limited` with the declared `Retry-After` — the worker records
  `error {classification: rate_limit}` and waits durably before the retry
  (event `state{provider_admission}`); 408/425, 5xx and recoverable connection
  failures are transport failures and use the same durable backoff, retaining
  `classification: transport`. Without `Retry-After`, retries wait one, two,
  then four seconds under the unchanged three-retry budget; they do not exhaust
  that budget in an immediate connection-failure loop. Other non-success statuses are `provider_terminal` only
  when the provider returned a valid terminal error object, otherwise
  `malformed`.
- SSE accepts CRLF or LF framing, UTF-8 split across transport chunks, comments,
  and multi-line `data:` fields. Truncated UTF-8, an over-limit frame, an
  unknown required provider event, or data after a terminal is `malformed`.
- Backpressure is bounded: at most 256 decoded frames or 4 MiB may await the
  worker consumer. Reaching either limit suspends socket reads; it never drops
  or reorders a frame.

## Normalized output

`ProviderFrame` is one of `text_delta`, `reasoning_delta`, `tool_delta`,
`status`, or `usage_delta`. Tool deltas always carry the stable provider call
id and, on their first occurrence, the tool name. Frames are endpoint-carrier
input only; event semantic facts are emitted from the terminal.

`ProviderTerminal` contains:

- ordered content blocks;
- ordered sealed native fragments with adapter/version stamps; the attempt's
  epoch supplies the model binding, so the carrier does not duplicate it;
- zero or more tool calls with stable `call_id`, name, and exact I-JSON
  arguments;
- continuation/query identities when supported;
- usage values as decimal strings, or explicit `unavailable`;
- a closed finish reason: `completed`, `tool_calls`, `length`,
  `context_overflow`, `content_filter`, or `provider_error`.

The adapter rejects NaN/infinity, invalid UTF-8, duplicate tool-call ids,
arguments that are not I-JSON, or a spill-discriminant collision. It never
guesses a missing tool name or call id. Terminal native bytes used by adopt are
published as the provider-adapter response asset before the recovery decision.

At the worker normalization boundary, GLM Chat and DeepSeek Responses opt into
the fixed-schema quoted-null repair. An invalid call may replace literal
`"null"` with JSON null only in nullable-string fields of an exactly-one-non-null
group, and only when exactly one candidate satisfies the complete schema.
Valid calls, ambiguous repairs and unrelated invalid fields are unchanged.
Ready, terminal and recovered calls use the same repair before durable call
insertion; sealed provider fragments retain the original arguments.

Response completion, final-answer marking, and durable turn settlement are distinct
records. Normally a final answer directly causes settlement; a turn that already
entered independent validation can resume its executor after feedback. A normalized terminal exposes
`is_final_answer`: it is false for reasoning-only/empty output and any tool calls.
Responses messages with a phase are final only for a completed, nonempty
`final_answer` message; `commentary` never finishes a turn. Legacy explicit
`isFinalAnswer` is preserved. Protocols without message phases use their native
completed/end-turn reason plus nonempty assistant text, matching TekesRuntime.

The semantic mapping is exhaustive: `completed` emits output with
`final_answer`. A new root final directly appends a completed settlement with
`promoted_output_seq` referring to that exact output. It does not inspect an
artifact snapshot or start an independent validator. Turns with a durable
`validation.candidate` from the earlier validation flow continue that flow on
recovery: artifact-free output receives `not_required`; an artifact-bearing
candidate without exact prior coverage forks an independent validator session
with `verify` as its terminal mandate. Failed validation appends feedback to the same worker; one
repair opportunity is allowed before a second negative round settles as
`inconclusive`. A synced validation decision owns the exact candidate output
reference, and only its recoverable settlement projection completes the turn.
Coverage binds thread, turn, worker, output sequence and artifact snapshot;
unchanged artifact bytes do not authorize a different candidate answer.
The snapshot includes the latest artifact revisions from the entire session
and its joined descendants, including earlier turns. A new answer cannot
bypass validation merely because it did not write another file in this turn.
Repair resumes the original executor context in the same turn. The validator's
own completed turn is evidence for the root decision, not root completion.
The validator's private workspace has a writable scratch directory and a
read-only materialization of the frozen candidate artifacts. The source
workspace remains readable but is not writable by validator tools. The child
receives a separate tool launch policy; the root's request profile and tool
catalog do not change. A successful `verify` over the exact covered set
settles the child immediately, including an empty set for text-only evidence.
Only `verify` calls count against the four-attempt verdict mandate. After two
ordinary outputs without `verify`, the child receives a reminder; four such
outputs without a verdict settle as an explicit validator failure. Exhaustion
is failure evidence, never authorization to promote the candidate as validated.
Adopted finals enter the same direct-final or legacy-recovery path. Nonfinal output
continues the same turn with the persisted response context (a server-managed
continuation can have no new input items). At most 32 nonfinal continuations
without tool calls are allowed per run; exhaustion is an explicit provider
error, never successful completion. `tool_calls` emits output plus its calls and continues
the same turn; `length` emits any valid partial output then settles
`interrupted/budget_tokens`; `content_filter` emits no unreviewed content and,
like `provider_error`, emits `error {classification: provider_terminal}`
carrying unavailable/reported usage and settles
`error/provider_terminal`. `auth_failure`, `transport`, `malformed`, and
`cancelled` enter provider-adapter recovery as maybe-sent whenever dispatch was
possible; their old attempt closes by that contract before retry/settle. No
finish or failure arm is mapped by implementation convention.

A `transport` failure that interrupts a streaming body carries
`partial_fragments`: the native output items the decoder had already
completed, in output order and in the adapter's sealed-fragment shape — every
item the stream closed (`response.output_item.done`, `content_block_stop`) and
every tool call whose arguments completed (the fact that made it eligible for
eager dispatch), with the same resolved arguments the terminal would carry.
Frames still never reach the semantic ledger (step 6); the worker consumes the
partial at step 8. When the lost response had eagerly dispatched calls and the
partial covers all of them, the recoverable `error` seals it as the attempt's
partial carrier (doc `event.md`, `error.sealed`), so the retry replays those
native items verbatim ahead of the durable results instead of a normalized
`tool_call` projection a provider may reject (DeepSeek thinking mode requires
the `reasoning` item back with its `function_call`). Without eager calls the
partial is dropped: nothing of the lost response is replayed.
`context_overflow` is recognized only from an adapter's structured error code,
never message-text guessing; it closes the attempt with a recoverable error
(carrying usage) before the worker compacts, opens a `compaction` epoch, and retries.

## Context and tool parity

Every adapter consumes the same normalized render input from doc 11. Dialect
code may change wire representation but not visibility, `admits`, supersede,
epoch, or prefix-stability decisions. Tool schemas are compiled from the one
effective catalog; an unsupported schema feature is a pre-send typed failure,
not a silently weakened declaration.

An active `compact` is supplied to every new epoch snapshot as the exact
normalized item defined by doc 11 (`role:user`, one text block) before
surviving event items. A server-managed incremental request omits it after the
snapshot has produced a continuation id. The adapter neither relocates nor
rewrites its text. This ordering is part of request-byte parity and is covered
by the overflow/compact retry gate.

Slice 7 does not claim predecessor parity until the pinned TekesRuntime
request/response/stream snapshots and the REQ-014 context ledger scenarios
pass against the four predecessor rows. `google_interactions` is a new adapter,
not present in that snapshot; its repository-owned, independently authored
request/response/stream expected bytes are the authority named by the
fixture registry. All five rows must pass their named authority. A registry row may ship disabled; it may
not advertise models until its complete parity set passes.

## Fixture contract

`fixtures/provider-runtime/cases.canonical.json` is the required-case registry.
Each row declares exact request, response, stream, stream-result, and normalized
result paths plus source/hash authority; no Cartesian filename expansion or
implicit artifact exists. `negative.canonical.json` declares the closed
negative-case inventory. Before adapter code begins, every named artifact
exists and the manifest matches the registry in both directions. Predecessor
bytes are copied from the pinned snapshot; independently authored cases name a
pinned public protocol revision. The implementation never generates or updates
these fixtures.

Required negative cases include malformed SSE, split UTF-8, duplicate terminal,
missing call id, invalid I-JSON arguments, oversized frame, timeout,
cross-origin redirect, credential redaction, and every HTTP classification.

### Anthropic conditional schema transport

Anthropic tool rendering may hoist a single top-level `allOf` branch containing only `if`, `then`, and optional `else`, provided the enclosing schema has none of these keys. This preserves the conditional group and all enclosing constraints. Other compositions remain intact; arbitrary conjunction merging is not supported. Canonical tool validation still uses the original schema.

Kimi tool schema encoding moves structural constraints from a nullable two-type union into an `anyOf` concrete-type branch and adds a null branch. Sibling constraints remain in place. An existing `anyOf` at the same node is rejected because replacing it would lose constraints. This follows the legacy Kimi compiler and avoids the service's `msh-schema-warning` response.
