# provider — model protocols and transport

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Builds requests for model dialects, handles HTTP/SSE, normalizes results and provides credential channels.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `dialect` | DialectId, target matching and qualified profiles |
| `request` | prepare; context to request bytes |
| `http / sse` | HTTP lifecycle, status and SSE framing |
| `normalize` | ProviderCompletion/Failure, content blocks, tool calls and usage |
| `credential` | Broker/client, per-request material, rotation/revocation and descriptor channels |
| `secret_store` | Read-only launch environment credentials and configuration-reference resolution; mutation helpers are test/embedding facilities and are not installed by production startup |
| `oauth` | Kernel-minted OAuth refresh grants (`OAuthGrant`; mint/rotate/revoke) and `OAuthTokenExchange`: refresh exchange with per-generation access-token cache, rotation write-back, 429 Retry-After, sticky HTTP 400 revocation |
| `compaction_summary` | The compactor-role summary request (`prepare_summary_request`, tool choice required) and artifact extraction from its terminal |

## Interfaces and calls

Main entry points: prepare, HttpRuntime, ProviderStreamDecoder, ProviderCompletion, CredentialBroker, CredentialClient.

Worker selects profile → prepare → HttpRuntime → SSE/normalize → completion; credential channels resolve material per request.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Model transport and workspace-tool network policy are separate paths; completing one provider response does not necessarily complete the turn.

Explicit Anthropic custom references and OpenAI client tool search are available
through `prepare_with_native_deferred_tools`, with catalog and source-call
binding checks. OpenAI native calls decode in JSON and SSE while preserving
native replay. The worker routes automatically per exact route through
`NativeDeferredMode` in the capability catalog (`native_deferred_tools`), and an
unbound search replays as an empty `tool_search_output`. The
[native protocol audit](../../../docs/verification/audits/native-deferred-tools-gap.md)
records the remaining boundaries. Ordinary dynamic schema activation does not
prove native tool-reference support.

HTTP 429 and the Cloudflare AI Gateway wholesale limit (HTTP 402 whose body
reports "Wholesale rate limit exceeded", `http::wholesale_limited`) classify as
`ProviderFailure::RateLimited` with the declared `Retry-After`; other 402
bodies remain provider-terminal candidates.

Behavior contract: [provider-runtime.md](../../../spec/provider-runtime.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.

Native Anthropic image input uses the existing base64 image encoder and is
advertised by the dialect capability table. OpenAI tools with `uniqueItems`
retain that constraint with strict mode disabled. See the
[public image verification audit](../../../docs/verification/audits/kernel-alignment-2026-09-04.md#public-image-attachment-round-trip)
for the completed four-route image matrix and its precise verification scope.

Kimi K3 streaming terminal choices can carry usage under `choices[0].usage`.
The Kimi decoder accepts that vendor-specific placement and preserves prompt,
completion and cached-token counters; top-level usage takes precedence when
present. Other Chat dialects do not inherit this fallback, and alternate-choice
counters are not summed. The behavior is covered by a captured-shape regression
and the live-write evidence listed in the verification audit.

DeepSeek's explicit `prompt_cache_miss_tokens` is retained as optional
`Usage.cache_miss` and written to the `usage` object of the settling output/error. Missing counters
remain absent, including Responses replies that report only nested cached
tokens. The original DeepSeek write/cache gate is proven on the Chat route.

For Responses tool argument streams, call identity and tool name cannot change
mid-stream. `function_call_arguments.done` must agree with any nonempty arguments
already streamed, and further argument deltas after completion are rejected.
This keeps sink-visible arguments consistent with the completed call at this
boundary. Completion-only arguments remain supported when no content was streamed.
When a Responses stream supplies tool arguments only in the arguments-done
event, the adapter immediately emits those bytes through the ordinary tool-delta
sink. It does not emit them again if argument deltas already supplied them.
This presentation event is not permission to execute a tool before response
validation, nor a durable tool result or turn settlement.
`ToolCallReady` is emitted once per call when the stream marks its arguments
complete — Responses arguments-done, Anthropic `content_block_stop`/`message_stop`,
Chat Completions finish reasons, Google function-call parts and Interactions
step stops all resolve through one shared `mark_call_ready` path. It is a presentation completion signal routed
through the sink. The response terminal still determines whether tool execution
can proceed. Worker-control maps it to an empty tool delta with
`arguments_complete`, and endpoint projection exposes `argumentsComplete` with
the shared `assistantFrameId`.
After a call is marked ready, `output_item.done` and an explicit terminal output
manifest must preserve its call ID, tool name, and JSON argument value. An
explicit terminal manifest cannot omit a completed call. JSON whitespace may
change without changing the argument value; absent terminal output retains the
existing streamed-item reconstruction path.
