# Native deferred-tool protocol gap

This audit tracks implementation and live evidence separately. It preserves the
native protocol requirements of the two pending legacy tests. The successful
Kernel two-turn marker gate exercises ordinary function schemas and cannot
replace either test below.

## Required gates

| Legacy gate | Required evidence |
|---|---|
| `live_openai_cloudflare_native_client_tool_search_round_trip` | Client-managed replay; native client search declaration; decoded search call and identity; original user input retained on replay; native search output carrying the bound loaded schema; next response calls `get_shipping_eta`; usage is present |
| `live_anthropic_cloudflare_custom_tool_reference_round_trip` | Deferred schema in the declared catalog; first response calls custom `tool_search`; correlated result contains native `tool_reference`; replay preserves call/result identity; second response calls `get_shipping_eta`; usage is present |

Both originals use order `LIVE-42`, the search/loaded fixtures near lines
2020–2080 of the pinned TekesRuntime `LiveTests.swift`, and the configured
Cloudflare routes. Their source assertions are at lines 365 and 428. Source
checkout: `/tmp/tekes-turn-live-audit/TekesRuntime`, revision
`995d8ffeece88c84502e721c5cf9c30d286a17bd`.

## Current source findings

| Boundary | Current implementation | Work still required |
|---|---|---|
| Capability | `ProviderCapabilities` in `provider/src/request.rs` declares server-managed continuation, query support and dispatch markers only | Declare qualified native mode by exact provider/model/API path; do not infer it from the broad adapter family |
| Request tools | Explicit native preparation supports Anthropic deferred definitions and OpenAI client tool search | Model-qualified automatic capability routing |
| Responses call decode | Native client calls decode in ordinary JSON and SSE, with identity/argument checks and native sealed replay | Worker native-path live acceptance |
| Responses replay | Explicit native preparation emits bound `tool_search_output` with loaded schemas and client-managed replay | Worker native-path live acceptance |
| Anthropic replay | Explicit native preparation replaces the bound successful search result with a pure `tool_reference` array; catalog digest/revision and call identity are checked | Adapter-direct live gate passed; worker integration remains |
| Worker authority | Same-turn search offers authorize ordinary dynamic schema projection and dispatch | Carry native references through immutable catalog, durable offer and provider context without accepting unbound names or stale digests |

The normative `native_deferred_tools` requirement in
[provider-adapter](../../../spec/provider-adapter.md) remains outstanding. This
is not a decision to remove it or redefine native support as host-side schema
activation. Exact replay, usage, secret handling and dialect gates must remain
intact during implementation.

## External protocol references

Checked on 2026-09-04 alongside the pinned legacy builders:

- [OpenAI tool search](https://developers.openai.com/api/docs/guides/tools-tool-search)
- [Anthropic custom tool search](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool)

The pinned OpenAI builder represents discovery results with `tool_search_output`
and replay calls with `tool_search_call`. Anthropic documents custom search
results as `tool_reference` blocks referring to tools declared at the top level.
Actual gateway/model acceptance still requires the live gates above.


## Anthropic implementation and initial live attempts

`native_deferred.rs` introduces explicit catalog definitions and references.
The adapter rejects stale digests/revisions, duplicate or undeclared tools,
references to the search tool itself, missing/duplicate/out-of-order source
calls, and failed results. Its transformation occurs before request encoding,
byte accounting and request-digest calculation. Ordinary preparation is unchanged.
Only Anthropic Messages is accepted by this entry point at present.

Three native binding tests and all 31 provider library tests pass in
`/tmp/tekes-native-provider-regression.log`. The live test preserves the legacy
LIVE-42 fixture, first search and second shipping-tool assertions, sealed replay,
and second-response usage requirement. It records raw request/response bodies.

`/tmp/tekes-native-anthropic-20260904-1` accepted initial discovery but rejected
a mixed text/reference result. The adapter now emits only reference blocks,
matching the legacy builder and native API contract. Run 2 reached the second
request but encountered gateway HTTP 429; this is not accepted as a pass.
Worker native routing and OpenAI native search are still outstanding.


After waiting more than a minute for the gateway limit, run 3 passed unchanged
model/fixture assertions at `/tmp/tekes-native-anthropic-20260904-3`. The model
`claude-fable-5` called `tool_search`, then `get_shipping_eta` with `LIVE-42`.
Both responses report usage. The second request retains sealed assistant replay,
contains the native reference and deferred schema, and has its final serialized
bytes included in the request digest. This closes the original adapter-direct
Anthropic gate; worker automatic native routing remains a separate gap.


## OpenAI native adapter completion

Explicit native preparation now supports OpenAI Responses client tool search.
It emits a native search declaration, omits deferred tools from the initial tool
list, and transforms the correlated search result into `tool_search_output`
carrying loaded schemas with `defer_loading`. It requires client-managed replay
and preserves the original user input and native sealed search call. References
must match the catalog revision, exact schema digest and preceding native client
call; failed source results and unbound outputs are rejected.

Native `tool_search_call` decoding accepts object arguments and stable call
identity in JSON and SSE, including terminal-only streams. Native replay is not
augmented with duplicate synthetic function calls. Malformed/client-vs-server
mismatches and DeepSeek Responses native-search input fail closed. Thirty-two
provider library tests and ten finality/decoding tests pass in
`/tmp/tekes-openai-native-regression.log`.

`/tmp/tekes-native-openai-20260904-1` passes the original adapter-direct two-request
LIVE-42 gate on configured Cloudflare/openai/gpt-5.6-luna. Request and response
bodies, request digests, normalized calls and reported usage are retained. This
closes the OpenAI adapter-direct test. Neither this nor the Anthropic test proves
automatic worker mode selection or its durable native-reference integration.

The shared preparation path was also rechecked against Anthropic in
`/tmp/tekes-native-anthropic-20260904-4`; that native-reference gate passes.
The SSE regression additionally rejects identity changes between added and
completed native search items.


## Worker automatic native routing (2026-09-05)

Capability: `model-capabilities.canonical.json` now carries
`native_deferred_tools` for the two live-proven routes — `openai/gpt-5.6-luna`
on Cloudflare `router` (`openai_client_tool_search`) and `claude-fable-5` on
Cloudflare `cloudflare-native-anthropic` (`anthropic_custom_tool_reference`);
the pinned catalog digest was updated. `resolve_profile` exposes the mode only
when the dialect is the mode's proved dialect and the target's endpoint owner
and gateway translation match a listed route (`NativeDeferredMode`,
`crates/provider/tests/native_deferred_routing.rs`).

Worker (`run_provider_turn_inner`): when the route declares a mode and the
launch catalog has deferred tools, the provider catalog adds every deferred
schema, `native_search_references` folds this turn's durable successful
`tool_search` offers into `DeferredToolReference`s (name and exact schema
digest must match the declared catalog; the catalog revision is a digest over
the deferred names/digests), the context is projected statelessly (no
`previous_response_id` even on the server-managed Responses dialect) and the
request goes through `prepare_with_native_deferred_tools`. The ordinary
`tool_search` builtin still executes the native search call and records the
offer; nothing about durable execution changes. `apply_openai` now replays a
search that bound nothing as an empty `tool_search_output` instead of failing
the request (a zero-match or failed host search must not kill the turn).

Evidence capture: the worker writes secret-free request/response bodies under
`TEKES_KERNEL_LIVE_ARTIFACT` in production builds too, and the supervisor
forwards only that variable across the cleared child environment;
`scripts/audit-native-worker-deferred.py` checks the captured bodies against
the ledger (native declaration, deferred schema absent/deferred, sealed native
call replay, bound output or reference, retained user input, usage).

Scripted proof: worker
`native_route_declares_client_tool_search_and_replays_bound_output` (CF luna
route with a local server: first request `tool_search` native + `store=false`
+ deferred function absent; native `tool_search_call` executes the durable
search; second request replays the sealed call with `tool_search_output`
carrying the deferred schema, no `previous_response_id`, retained user input;
turn completes with usage on both rounds).

Live proof (worker, real CF routes, `run-live-process.py --scenario tool-search`):

- OpenAI `openai/gpt-5.6-luna` on CF router: run 1
  (`/tmp/tekes-native-worker-openai-20260905-1`) completed both public turns and
  passed the native audit, but the legacy process test still demanded a
  changing catalog digest; that expectation is now mode-aware (native = one
  stable digest). Run 2 (`/tmp/tekes-native-worker-openai-20260905-2`) passes
  runner and audit: turns 1/2 search natively (`call_JqTAMWD…`, `call_V9WvEY…`),
  replay attempts carry `tool_search_output` with the deferred `uat_marker`
  schema, no `previous_response_id`, markers FIRST/SECOND_HOT_OK executed, one
  epoch for the whole run, usage on every attempt.
- Anthropic `claude-fable-5` on CF native gateway: runs 1–2 refused the original
  prompt at the first request (`stop_reason: refusal`, `category: cyber`); run 3
  with a neutral prompt variant (`TEKES_TOOL_SEARCH_PROMPT_STYLE=plain`) got the
  native `tool_search` call, executed the durable offer, and the replay request
  carrying `[{"type":"tool_reference","tool_name":"uat_marker"}]` with
  `defer_loading` was accepted (HTTP 200) before the model refused again. The
  worker wire path is therefore accepted by the API, but no completed two-turn
  Anthropic worker run exists; the adapter-direct LIVE-42 gate stays the
  completed Anthropic evidence. Wire bodies: `/tmp/tekes-native-worker-anthropic-20260905-3-wire/`.

Anthropic follow-up (evening): a neutral shipping-ETA variant of the same
public scenario (`scripts/shipping-dynamic-helper.sh`,
`TEKES_TOOL_SEARCH_SCENARIO=shipping`, prompts about order LIVE-42/43) avoids
the marker-wording refusal. Its first run exposed a real replay bug: eager
dispatch makes the search `tool_result` durable before the attempt's `output`,
and stateless full replay rendered it before the sealed `tool_use`; the
worker now renders such results right after their attempt's output
(`eager_results_render_after_their_attempt_output_in_replay`). Runs 2 and 3
(`/tmp/tekes-native-worker-anthropic-shipping-20260905-{2,3}`) then complete a
full native worker turn: `tool_search` called natively with `defer_loading`
declared, the durable offer executes, the replay carries the `tool_reference`
result, `claude-fable-5` calls `get_shipping_eta` for LIVE-42 and answers
ETA-3D, usage on both rounds. The second public turn is refused
(`stop_reason: refusal`, `category: cyber`) at its first request in both runs,
so the Anthropic worker proof is one completed native turn
(`native-worker-audit.json` with `TEKES_NATIVE_AUDIT_TURNS=1`).

Residuals: the Anthropic second public turn is refused by the model's content
classifier in every wording tried; adopt/query recovery on a `store=false` Responses attempt
cannot re-fetch the response by id (falls to the existing non-adopt recovery);
native mode is declared for exactly two routes and any new route needs its own
adapter-direct live gate before catalog declaration.
