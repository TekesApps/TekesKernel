# Provider parity and qualification

[Verification · documentation home](../README.md) · [Verification entry point](README.md) · [Next: provider gates](gates/providers.md)

This is a protocol-profile parity record. Exact current configured/advertised profiles
must be read from the source/fixture registry, not inferred from the historical count
below. Current provider modules and call paths are in [Code architecture](../architecture/README.md).

## Purpose and status

This document records the model-provider parity closure after Slice 7. It is a
migration explanation; the binding language-neutral wire contract is
[`spec/provider-dialect-profiles.md`](../../spec/provider-dialect-profiles.md)
with `fixtures/provider-dialects/`; where this inventory differs, the contract
wins.

Slice 7 implemented five protocol-family baselines: OpenAI Responses,
Anthropic Messages, Google Generation, Google Interactions, and generic Chat
Completions. Passing one family oracle does not qualify every provider that
accepts a similar request shape. Gates 93–96 now close twelve exact profiles,
of which eleven are advertised and one generic fixture remains test-only.
Parity is **aligned for that closed v1 registry**; an unproved tuple is still
unavailable rather than inferred from its family.

## Required layering

Provider execution has four distinct identities:

1. **Protocol family** — Responses, Chat Completions, Anthropic Messages,
   Google Generation, or Google Interactions.
2. **Vendor wire dialect** — the exact request, state, streaming, terminal,
   error, usage, tool, and authentication behavior implemented by one API.
3. **Model capability profile** — exact SKU behavior for reasoning, sampling,
   tool choice, schemas, multimodal input, structured output, cache, and
   continuation.
4. **Route evidence** — endpoint owner, gateway translation, exact SKU, and
   pinned evidence revision.

Code may share a family codec, but two routes are the same dialect only when
their durable request bytes and recovery/decoding behavior are interchangeable.
A compatibility label, SDK reuse, endpoint suffix, provider display name, or
model-name prefix is never sufficient evidence. Unknown tuples fail closed and
are not advertised.

That closed selection boundary does not make response vocabularies closed.
Once a selected route has produced well-framed I-JSON, the decoder extracts
known text, reasoning, tool, usage, and lifecycle fields while retaining native
provider output items for sealed replay. Unknown event, item, block, part, and
extension variants are accepted when their enclosing structure is valid. The
runtime remains strict about malformed framing/JSON, missing discriminators,
contradictory terminal states, missing tool identities, invalid arguments, and
cross-dialect replay.

The runtime selection key is therefore:

```text
(protocol_family, dialect_id, endpoint_owner,
 gateway_translation, exact_sku, evidence_revision)
```

## Known blocking example: DeepSeek Responses

DeepSeek Responses must not use the current OpenAI `responses` dialect. The
DeepSeek contract is stateless and does not support server-managed
`previous_response_id`, conversation, or store semantics; every continuation
must send its complete eligible history. It also differs in supported request
parameters, input and built-in tool types, reasoning stream events, and
completed/incomplete/failed terminals. Unsupported OpenAI fields may be
silently ignored, so successful HTTP status is not capability proof.

Using the current server-managed dialect could make the worker omit prior
context while DeepSeek ignores `previous_response_id`. Treating
`response.reasoning_text.delta`, `response.incomplete`, or `response.failed` as
unknown events would also turn valid provider behavior into malformed recovery.
DeepSeek Responses therefore requires a separate dialect identity, request
projector, decoder, recovery profile, and fixtures.

## Initial closure inventory

The first parity closure covers every currently retained predecessor route:

| protocol family | required dialect/profile coverage |
|---|---|
| Responses | OpenAI Responses; DeepSeek Responses |
| Chat Completions | generic OpenAI-compatible baseline; direct OpenAI; DeepSeek; Kimi/Moonshot; GLM/z.ai; Ollama |
| Anthropic Messages | direct Anthropic; DeepSeek Anthropic-compatible API |
| Google | direct Generation and Interactions profiles |

Adding a provider or gateway later requires a new evidenced row when it changes
any request byte, authentication rule, state/continuation behavior, stream
event, terminal/error/usage mapping, schema projection, cache ownership, or
provider-specific normalization/repair.

## Provider-specific behavior to preserve

The executable contract and fixtures must cover, where applicable:

- reasoning/thinking fields, supported effort levels, defaults, and replay;
- sampling parameters and their interaction with thinking;
- tool-choice modes, parallel calls, built-in tools, and deferred tools;
- canonical tool-schema projection and validation against the original schema;
- stateful response references versus stateless full-history replay;
- sealed/native carriers and epoch changes when replay is not legal;
- cache-control request shape, cache owner, and usage attribution;
- structured-output and multimodal support, including fail-closed rejection of
  silently ignored inputs;
- known SSE event semantics plus open-ended, structurally valid provider
  vocabulary; terminal states, errors, usage, and finish reasons remain strict;
- bounded, explicitly evidenced provider repairs such as known DeepSeek tool
  argument defects;
- local-provider behavior such as Ollama endpoint normalization, credentials,
  and unsupported reasoning.

## Production-control wiring

Model selection is incomplete if it changes only configuration or UI state.
The session's durable `reasoning_effort` and every product-authored model
control must be validated against the selected exact profile, captured in the
immutable epoch profile, and reflected in the exact transmitted request and
request digest. Storing or echoing a selection while the worker sends empty
controls is a failing conformance case.

The exact profile also carries an explicit `serializer_revision`. It is
proof-bound and copied into the immutable epoch profile, so a projector change
cannot silently reuse an older epoch merely because the provider/model route
and current request bytes happen to remain equal.

The model catalog advertises only capabilities proven by the selected tuple.
An unproved route may remain configurable for development but is unavailable
to ordinary session routing. Production readiness consumes the pinned proof
oracle and rejects an absent, stale, digest-mismatched, or
`advertised:false` row as `dialect_unproved`.

The first closed registry does not collapse optional controls into the family
codec. V1 has a product author path only for `reasoning_effort`; sampling,
explicit tool choice, response format, caller-selected max tokens, and cache
keys reject before dispatch even if internal codec work exists. A later typed
author path must extend the contract, epoch oracle, and end-to-end gate before
any of those fields may advertise.

## Required oracle and gates

For every advertised tuple, canonical fixtures must bind:

1. exact non-stream and stream request bytes;
2. all known response and SSE event arms, plus forward-compatible unknown
   variants that retain native replay fragments;
3. normalized terminal, reasoning, tools, errors, and usage;
4. multi-turn context and continuation behavior;
5. tool-schema projection and canonical post-call validation;
6. unsupported parameter/input/tool rejection rather than silent weakening;
7. transport, retry, recovery, overflow/compaction, and cancellation windows;
8. endpoint/SKU/gateway/evidence mismatch fail-closed behavior;
9. session control selection through epoch bytes to final provider wire;
10. an opt-in live smoke for the exact direct route after hermetic gates pass.

DeepSeek Responses additionally requires a negative compatibility test proving
that no OpenAI server-managed continuation path can select it, plus full-history
replay, native reasoning/tool sealed-carrier roundtrips, and structured
incomplete/failed stream cases.

## Sequencing and completion

The executable dialect/profile contract and hermetic oracle were developed in
parallel with Slices 12 and 13. Gates 93–96 now gate the implemented
provider/model advertisement and Slice-14F `providerAdmin.v1` capability. An
opt-in live route smoke remains supplemental and cannot upgrade an unproved
tuple.

Provider parity is complete only when every retained route is either:

- implemented and advertised with an exact passing profile; or
- explicitly retired and omitted from configuration, catalogs, capabilities,
  and migration claims.

The closed v1 registry satisfies that rule. Adding a route, SKU, gateway,
serializer revision or product-authored control requires new exact evidence;
the current alignment is not a wildcard provider claim.
