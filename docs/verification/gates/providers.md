# Provider and dialect gates

[All gates](README.md) · [Running tests](../running-tests.md)

This page retains the gates' original numbers and applicable baselines. It defines conformance rules, not the results of this run.

## Production provider runtime

53. ◉ **ProviderDialectParityOracle** — Setup: the completed per-adapter
    request/response/stream/result corpus, the four-row pinned TekesRuntime
    source lock, and the repository-owned Google Interactions oracle.
    Action: render and decode every case for all five registry rows.
    Assert: request bytes and normalized terminal are exact; an incomplete row
    advertises no model; REQ-014 visibility, tools, sealed fragments and usage
    retain parity. `[provider-runtime §Scope/§Context and tool parity]`
54. ◉ **ProviderStreamFramingBounds** — Setup: LF/CRLF SSE, split UTF-8,
    comments, multiline data, over-limit, duplicate-terminal and
    after-terminal cases. Action: feed every transport chunk partition under
    slow consumption. Assert: identical ordered frames/terminal, bounded
    backpressure without loss, and deterministic malformed/limit failures.
    `[provider-runtime §HTTP and streaming limits/§Normalized output]`
55. ◉ **ProviderSendRecoveryMatrix** — Setup: every adapter capability,
    dispatch-marker policy, query result and run mode. Action: crash at
    admission lease → credential get → attempt barrier → dispatch barrier →
    HTTP → terminal → outcome barrier → release. Assert: provider-adapter's
    decision table is exact, the durable
    attempt reconstructs the exact query key, no send occurs without a held
    admission and credential lease, no two live attempts exist, and adopt
    completes only from its asset.
    `[provider-runtime §Send ordering; provider-adapter §Recovery decision
    table/§Decision of record]`
56. ◉ **ProviderHttpClassificationAndSecrets** — Setup: every HTTP status,
    timeout, redirect, cancellation and credential-broker case. Action: execute
    with
    capture at logs/events/assets. Assert: the closed classification is exact,
    cross-origin redirect rejects, cancellation is maybe-sent when applicable,
    rotation/revocation reaches the next request, and no credential/header/
    body secret reaches a durable or diagnostic sink.
    `[provider-runtime §Provider invocation seam/§HTTP and streaming limits;
    credential-broker; secret-store]`
57. ◉ **ProviderTerminalNormalization** — Setup: text, reasoning, sealed
    fragment, image, tool calls, usage-absent, invalid numeric and duplicate
    call-id terminals. Action: normalize and emit semantic events. Assert:
    exact order and I-JSON values, stable call ids, explicit unavailable usage,
    one outcome transaction, and fail-closed malformed inputs. `[provider-
    runtime-v1 §Normalized output; event constraint 2]`
58. ◉ **ProviderOverflowCompactRetry** — Setup: first-send unknown usage and
    a rendered request over the configured context limit. Action: preflight,
    settle the attempt, compact/open the next epoch, and retry. Assert: no HTTP
    before preflight acceptance, the old attempt closes before the new epoch,
    and every request remains prefix-stable within its epoch. `[provider-
    runtime-v1 §Context and tool parity; Worker lifecycle §Compaction; Model context §Epoch]`

## Provider dialect and model-profile parity

93. ◉D **DialectRegistryClosure** — Setup: the provider-dialect profile
    registry and invalid corpus. Action: compare disk, manifest and every
    advertised tuple. Assert: the twelve exact profiles are unique, the generic
    fixture profile is test-only, the eleven advertised tuples have request,
    stream, terminal, recovery, negative, multi-turn, mismatch and control
    proof, and production readiness consumes the pinned oracle rather than
    accepting field presence. `[provider-dialect-profiles §Registry and
    advertisement/§Exact proof fixture]`
94. ◉D **ExactDialectBytes** — Setup: every exact profile's first/multi-turn
    and tool/schema requests, ordinary/tool raw SSE, terminal oracle, and
    declared input/cache/repair surface. Action: prepare/decode through its
    production-selected dialect. Assert: bytes and normalized results match
    exactly, advertised input variants project, unadvertised variants reject,
    implicit-prefix ordering and bounded repairs execute, and DeepSeek
    Responses reasoning/tool carriers round-trip natively while structured
    completed/incomplete/failed arms all parse; no OpenAI-compatible heuristic
    selects a vendor dialect. `[provider-dialect-
    profiles-v1 §DeepSeek Responses/§Initial closed dialect registry]`
95. ◉D **DialectRecoveryAndMismatch** — Setup: multi-turn state, every dispatch
    boundary, cross-dialect sealed carriers, and route/SKU/gateway/revision
    mismatches. Action: recover or reselect. Assert: old attempts close before
    continuation/resend, stateless dialects replay full eligible history, and
    every identity mismatch fails closed before send/advertisement.
    `[provider-dialect-profiles §Identity and precedence/§Exact proof fixture]`
96. ◉D **SessionControlsToWire** — Setup: every supported
    `reasoning_effort` and representative unsupported sampling/tool-choice/
    response-format/cache-key controls for every advertised exact profile.
    Action: resolve a new epoch and prepare its request. Assert: unsupported or
    product-unreachable values fail before lease/attempt; each supported value
    changes canonical epoch bytes/profile digest, exact request bytes and
    request digest; an empty control object is legal only for a profile with no
    exposed control.
    `[provider-dialect-profiles §Session controls and epoch bytes]`
