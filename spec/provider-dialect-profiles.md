# spec: provider dialect profiles v1

This contract narrows the five protocol-family codecs in
[provider-runtime](provider-runtime.md) into exact vendor dialect and
model-profile identities. It is the authority for provider/model
advertisement, immutable epoch controls, exact request projection, dialect
decoding and route-evidence checks. A protocol-family fixture is codec coverage;
it is never proof that two vendors implement an interchangeable API.

## Identity and precedence

A runnable provider target has the closed identity:

```text
ProviderTargetV1 {
  protocol_family: responses | chat_completions |
                   anthropic_messages | google_generation |
                   google_interactions,
  dialect_id: str,
  model_profile_id: str,
  route: {
    endpoint_owner: str,
    gateway_translation: direct | str,
    exact_sku: str,
    evidence_revision: str
  }
}
```

All six scalar strings are non-empty, byte-exact identifiers. `direct` means
the endpoint owner implements the dialect; every translating gateway has a
distinct non-empty stable id. Endpoint hostname, URL suffix, provider display
name and model-name prefix do not infer any field. Configuration selects the
entire target and the endpoint, and the kernel runs exactly what is configured.

## Configured routes

The kernel has no runtime route allow-list. A configured provider is runnable
when its `dialect` is one this build serializes and its `adapter` is that
dialect's protocol family; `endpoint_owner`, `gateway_translation`,
`evidence_revision`, model SKU and endpoint are taken as configured. The
credential header and prefix come from the dialect (`cloudflare` /
`cloudflare-native-anthropic` uses `cf-aig-authorization: Bearer`). The
configured endpoint is used as the request base unchanged. A route the upstream
does not accept fails at request time with the provider's error; an unknown
dialect or a family/dialect disagreement is `misconfigured` readiness.

`tekes-supervisor --models-available` lists the dialects this build supports
as `[{dialect_id, protocol_family}]`.

The registry below is the conformance corpus that pins each dialect's exact
serializer and decoder behavior. It is test data, not compiled into the
runtime, and it neither admits nor rejects a configured route.

The tuple is part of the immutable epoch profile. Changing any tuple field,
control, serializer revision, tool-schema projection, cache policy or provider
repair starts a new epoch. Sealed carriers are reusable only when the source
and target tuples are equal. `attempt.wire_digest` continues to bind the exact
request through provider-runtime; the digest preimage uses `dialect_id` in
the former adapter-id position so different dialects cannot collide on equal
body bytes.

Two targets may share a codec implementation only when their independently
named proof sets establish identical request bytes, state/continuation rules,
known SSE semantics, terminal/error/usage normalization, tool/schema projection,
reasoning/sampling controls, cache behavior and bounded repairs. Otherwise
they have distinct `dialect_id`s even if both endpoints call themselves
OpenAI-compatible.

Registry selection is closed; provider response vocabulary is not. After a
selected route yields well-framed I-JSON, known response/event variants are
normalized and unknown structurally valid variants are retained in the native
sealed carrier when they contribute replayable provider output. Unknown
metadata events may be ignored. Missing discriminators, malformed containers,
contradictory terminal state, invalid tool identity/arguments, and cross-tuple
sealed replay remain errors.

## Registry and advertisement

The production registry is closed. Each row contains the exact identity plus:

```text
ProfileProofV1 {
  target: ProviderTargetV1,
  endpoint: str,
  request_url: str,
  credential_header: str,
  credential_prefix: str,
  serializer_revision: str,
  continuation: server_managed | stateless_full_history,
  advertised: bool,
  capabilities: {
    reasoning: {levels: [str]*, wire: str},
    sampling: str,
    input_blocks: [text | image | file | tool_call | tool_result]*,
    tool_choice: {modes: [auto | none | required | named]*, wire: str},
    schema: unsupported | validated_object | strict_object,
    cache: none | implicit_prefix | explicit_breakpoint | server_managed,
    repair: RepairV1
  },
  proof_id: str,
  request_bytes: str,
  multiturn_request_bytes: str,
  tool_request_bytes: str,
  stream_bytes: str,
  tool_stream_bytes: str,
  terminal: JsonValue,
  tool_terminal: JsonValue,
  recovery: JsonValue,
  negative: [JsonValue]+,
  mismatch: JsonValue,
  control: {
    session: JsonValue,
    epoch_profile: JsonValue,
    profile_digest: str,
    request_digest: str
  },
  capability_cases: [JsonValue]+
}
```

`endpoint` is the route rule owned by the proof row, not by compiled dialect
code. It is either an exact normalized base URL or a URL whose whole path
segments are named placeholders such as `{tenant}`; placeholders match one
non-empty, non-dot segment and never match scheme, host, query, or fragment.
`request_url` is the exact URL used by that row's request-digest oracle.
`credential_header` and `credential_prefix` describe how the separately leased
credential is attached after digesting; the runtime does not infer either from
the provider, router, model, protocol family, or endpoint hostname.

This is the literal top-level shape of each canonical registry row; capability
claims are nested under `capabilities`, while proof artifacts and
`serializer_revision` are siblings of it. Dialect-specific proof arms may add
fields but may not replace these required members. The identical non-empty
`serializer_revision` is embedded in `control.epoch_profile`, checked by the
production projector, and therefore changes the immutable profile digest even
when a new serializer happens to emit the same request bytes.

Each reasoning-level entry gives a closed input domain and one of: exact native field,
deterministic mapping, deterministic omission, or rejection. An accepted
control may never be stored-only. Deterministic omission is legal only when the
profile proves the provider owns/fixes the value (for example a fixed sampling
policy); it is not a way to tolerate a silently ignored parameter. Unknown
controls and values fail before an attempt or lease.

`RepairV1` names an exact provider defect signature, input domain, bounded
rewrite and evidence revision. Repairs never trigger from message text alone,
never broaden schemas and never run for another target tuple.

Advertisement requires every artifact named by `proof_id`: first-turn and
multi-turn request bytes, ordinary and tool-call stream bytes, normalized
ordinary and tool terminals, recovery, structured negative/mismatch cases,
the declared input/tool/schema/cache/repair capability surface, and
control-to-epoch-to-wire. The production registry, projector, decoder and
recovery decision function must consume those artifacts; the mere presence of
a same-named JSON member is not proof. An opt-in live smoke is supplemental and runs only after hermetic
proof; live success cannot replace a missing fixture.

The initial registry's product-input control surface is deliberately exact,
not the union of fields a protocol codec could encode. V1 exposes only
`reasoning_effort`, and only for profiles whose `reasoning.levels` are
nonempty. `temperature`, `top_p`, explicit `tool_choice`, `response_format`,
`prompt_cache_key`, and caller-selected `max_tokens` have no typed
session/endpoint author path in v1 and therefore fail before lease or attempt.
They require a later contract and new proof before advertisement.

No profile lists `none` in `reasoning.levels`, even where the provider
accepts it: current models give better results with thinking on, so the
lowest user-selectable effort is `low`.

One runtime-internal control has no product author path:
`reasoning_disabled: true`, used by supervisor helper requests (automatic
titles) that must not reason whatever the model's default. It is independent
of `reasoning.levels`, is never advertised to clients, and cannot be combined
with `reasoning_effort`. Only dialects with a verified off switch accept it:
`deepseek_responses_v1` renders `reasoning: {effort: "none"}` and
`deepseek_chat_v1` renders `thinking: {type: "disabled"}`. Every other dialect
fails to prepare, so the helper sends nothing.

A second runtime-internal control is title-only: `title_max_output_tokens`
(a positive integer), the output ceiling of the automatic thread-title request
(64). It is not a session control and never limits ordinary turns.
`deepseek_responses_v1` renders it as the wire field `max_output_tokens`;
`deepseek_chat_v1` renders it as `max_tokens`, keeping a smaller catalog
ceiling. Every other dialect fails to prepare. A title response cut at the
ceiling ends with a length terminal, which the supervisor treats as failure,
so the deterministic seed title stays.

`capability_cases` binds each profile's closed product default; a test may not
manufacture an otherwise unreachable epoch control and call it product proof.

## Initial closed dialect registry

The initial closure has twelve distinct testable dialect profiles. Eleven are
advertised exact tuples. `generic_chat_v1` is a conservative codec fixture. The
exact fixture SKU is a conformance identity, not a wildcard declaration:

| dialect id | family | endpoint owner | exact fixture SKU | continuation |
|---|---|---|---|---|
| `openai_responses_v1` | responses | `openai` | `gpt-5` | server-managed |
| `deepseek_responses_v1` | responses | `deepseek` | `deepseek-v4-flash` | stateless full history |
| `generic_chat_v1` | chat completions | `fixture` | `generic-chat-fixture-1` | test-only; never advertised |
| `openai_chat_v1` | chat completions | `openai` | `gpt-4.1` | stateless full history |
| `deepseek_chat_v1` | chat completions | `deepseek` | `deepseek-v4-flash` | stateless full history |
| `kimi_chat_v1` | chat completions | `moonshot` | `kimi-k3` | stateless full history |
| `glm_chat_v1` | chat completions | `zai` | `glm-5.2` | stateless full history |
| `ollama_chat_v1` | chat completions | `ollama` | `llama3.3` | stateless full history |
| `anthropic_messages_v1` | anthropic messages | `anthropic` | `claude-sonnet-4-20250514` | stateless full history |
| `deepseek_anthropic_v1` | anthropic messages | `deepseek` | `deepseek-chat` | stateless full history |
| `google_generation_v1` | google generation | `google` | `gemini-2.5-flash` | stateless full history |
| `google_interactions_v1` | google interactions | `google` | `gemini-2.5-flash` | server-managed |

Rows pin serializer behavior; they do not enumerate the SKUs or gateways a
deployment may configure.

## DeepSeek Responses

`deepseek_responses_v1` is not `openai_responses_v1`. It is stateless:

- request bodies omit `previous_response_id`, `conversation`, `store` and every
  server-managed continuation field;
- every attempt sends the complete eligible history rendered for its epoch;
- a continuation id in input is a pre-send `dialect_mismatch` error;
- image/file inputs and unsupported built-in tools are rejected before send,
  including fields the server might silently replace or ignore;
- only profile-declared controls are emitted; unsupported OpenAI Responses
  parameters fail closed rather than being dropped;
- `response.reasoning_text.delta` and `.done` are reasoning arms;
- `response.completed`, `response.incomplete` and `response.failed` are three
  distinct terminals. Incomplete maps only from a structured reason; failed
  preserves the structured provider error and never becomes success. The
  normalized terminal carries `incomplete_reason?:str` and
  `provider_error?:JsonValue`; `max_output_tokens` maps to `length`,
  `content_filter` maps to `content_filter`, and every unknown/missing arm is
  malformed rather than a generic success;
- recovery is maybe-sent after the dispatch boundary and replays the full
  history only after provider-adapter closes the old attempt.

Its sealed full-history carrier is also native: reasoning is
`{type:"reasoning",content:[{type:"reasoning_text",text:str}]}` and tool calls
retain their native `function_call` item. The proof oracle round-trips both an
ordinary reasoning terminal and a tool terminal through the next request;
using OpenAI's `summary/summary_text` carrier is a dialect mismatch.

No request projector, stream decoder, continuation branch or sealed carrier
selected for `openai_responses_v1` may be reached merely because both share the
`responses` family.

## Chat-completions dialects

All chat dialects replay full history and use `/chat/completions`, but their
profiles remain distinct.

- `generic_chat_v1` proves only the conservative common subset: text, function
  tools with implicit auto-selection, no explicit tool-choice control, no
  reasoning, no cache claim and no vendor repair.
- `openai_chat_v1` declares only controls and structured-output shapes proven
  for its exact SKU.
- `deepseek_chat_v1` preserves `reasoning_content` through tool-call rounds.
  Thinking omits `temperature` and `top_p`; forced/named tool choice while
  thinking is rejected or deterministically mapped only where its profile
  proof says so. Known tool-argument repair is signature-bound and bounded.
- `glm_chat_v1` is the exact GLM-5.2 profile: it maps `high|max` to the
  top-level `reasoning_effort`, keeps `thinking.clear_thinking=false` so
  replayed reasoning remains legal, and rejects sampling controls that are not
  represented by the session-control contract.
- `kimi_chat_v1` is the exact Kimi-K3 profile: it maps `low|high|max` to the
  top-level `reasoning_effort`, omits provider-fixed sampling, and advertises
  only the implicit-auto tool-choice surface that Kernel actually projects.
- `ollama_chat_v1` normalizes the configured owner route to its exact `/v1`
  form before digesting, rejects reasoning for the named profile, and has no
  credential-dependent capability inference.

Every dialect rejects malformed `reasoning_content`, duplicate call ids,
schema weakening and a tool choice outside its closed set.

## Anthropic dialects

`anthropic_messages_v1` and `deepseek_anthropic_v1` share a family codec only
where their proof sets agree. Header set/version, thinking blocks and
signatures, sampling restrictions, tool choice, cache-control placement,
terminal reasons and usage are dialect behavior. The native Anthropic profile
preserves ordered content blocks, including empty signed `thinking`, streamed
`signature_delta`, and opaque `redacted_thinking`, through durable replay.
Reasoning controls remain exact-model catalog capabilities, not capabilities
inferred from an endpoint accepting `/messages`.

Native requests with adaptive or enabled thinking use
`thinking.block_binding.prefix_mismatch_behavior=drop_block` with the
`thinking-binding-controls-2026-08-01` beta header. Compaction, result trimming,
and system/tool changes may intentionally change a prefix; the server decides
which thinking blocks remain valid for that request. Kernel does not erase
durable native blocks on model changes or locally guess signature validity.
Provider `input_transformations` reports are retained as runtime diagnostics,
never appended to the model context. Streaming reports may arrive at message
start and be updated by a final message delta.

The established last-tool and latest-message content cache breakpoints remain
unchanged. This policy does not replace tool-catalog stability or epoch/reset
management and does not itself guarantee a cache hit. DeepSeek's compatible
route has its own evidence and never inherits direct-Anthropic controls merely
because it accepts `/messages`. Live Fable 5.1 binding acceptance is separate
from codec fixtures and the successful Opus gateway smoke test; see
`docs/verification/audits/anthropic-preserved-thinking-2026-09-09.md`.

## Google dialects

`google_generation_v1` sends complete history with the exact
`generateContent` or `streamGenerateContent?alt=sse` route. Controls live in
the profile-declared `generationConfig`; tools, thought parts, usage metadata,
safety terminals and cached-content behavior are explicit proof arms.

`google_interactions_v1` is independently stateful. Only it may use
`previous_interaction_id`, and only after a terminal from the same target
tuple. Its `store`, input-step, response-format, stream and terminal behavior
is not inferred from Generation.

## Session controls and epoch bytes

The effective session controls are resolved before epoch append:

1. select the exact configured target;
2. validate every requested control against its exact profile;
3. apply only the profile's explicit defaults/mappings;
4. encode `controls` and the complete `target` into the immutable epoch profile;
5. compute the profile digest from those canonical bytes;
6. project the request from that same value; and
7. compute `attempt.wire_digest` from the exact request bytes.

`reasoning_effort` follows this path. An endpoint success that changes only
session settings is insufficient: the next compatible epoch and request bytes
must reflect the selected value. A worker-created `controls:{}` is legal only
when the effective validated control set is actually empty. Recovery uses the
durable epoch value, not mutable settings.

## Exact proof fixture

`fixtures/provider-dialects/profiles.canonical.json` is the closed tuple and
proof registry. For every advertised row it contains or names:

- exact canonical request bytes for first and later turns;
- exact tool/schema request bytes;
- ordered raw ordinary and tool-call SSE transcripts, each consumed by the
  production dialect decoder;
- completed/tool/incomplete/failed terminal arms as applicable;
- exact usage/cache fields and executable input/schema/cache/repair capability
  cases;
- old-attempt closure and resend/adopt recovery expectations;
- unsupported-control/input/tool/schema negatives;
- exact tuple mismatch rejection; and
- control, epoch-profile, profile-digest and request-digest bytes.

## Model capability catalog

`fixtures/provider-dialects/model-capabilities.canonical.json` is the production authority for
model-specific reasoning controls. Rows are keyed by the exact
`(dialect_id, model_profile_id, exact_sku)` tuple rather than by dialect: two models served through
the same wire protocol may support different values or defaults. The `reasoning.levels` array is
ordered product data. `session.models`, epoch validation, and request encoding must preserve that
order exactly; sorting is forbidden. Duplicate values, a default absent from the array, an invalid
catalog digest, or a malformed evidence URL fails catalog loading. An executable unproved target
without a catalog row remains usable but advertises no reasoning controls.

Each reviewed row carries the official documentation URL used for its current capability claim
when one is available. Updating provider behavior therefore requires one explicit catalog edit and
the corresponding proof/test refresh, not a new client-side inference.


`invalid.canonical.json` is the rejection corpus, while
`forward-compatible*.canonical.json` proves open provider vocabulary and exact
native replay. The checker verifies JCS
bytes, exact registry membership, per-row proof completeness, mismatch
fail-closed behavior, DeepSeek Responses stateless prohibitions, and manifest
parity. Runtime tests must independently reproduce the bytes; they may not
generate or update the oracle.

The hermetic gates are:

1. **DialectRegistryClosure** — fourteen exact rows, thirteen advertised tuples, no
   duplicate identity, and no advertised row without all proof arms;
2. **ExactDialectBytes** — first/multi-turn and tool/schema request bytes, every
   declared SSE variant, forward-compatible unknown variants, terminals and
   normalized/native replay bytes match through the production projector and
   decoder;
3. **DialectRecoveryAndMismatch** — multi-turn/recovery and every cross-target
   mismatch fail closed;
4. **SessionControlsToWire** — control validation changes immutable epoch,
   request bytes and both digests exactly.

An optional exact-route live smoke names the full target tuple and evidence
revision in its result. It cannot run before gates 1–4.
