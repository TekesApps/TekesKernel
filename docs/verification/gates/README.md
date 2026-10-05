# Conformance gates by topic

[Verification entry point](../README.md) · [Run commands](../running-tests.md)

| Topic | Gates covered |
|---|---|
| [Core execution](core.md) | Creation, input delivery, attempts, usage and tool fundamentals |
| [Recovery and data](recovery-and-data.md) | Lifecycle, stops, replay, configuration, context and durability |
| [Provider](providers.md) | Production providers and dialect qualification |
| [Tools and integrations](tools.md) | Execution, plugins, MCP and extension tools |
| [Client](client.md) | Endpoint and transport |
| [Deployment](deployment.md) | macOS release and external pipelines |

Gate numbers and fixture identities remain unchanged. Stage execution order is recorded separately in
the [historical stage map](../../history/gate-rollout.md); common assertion conventions and fixture rules remain below.
Chapter shorthand in original gate text can be traced through the [migration map](../../history/document-map.md).

This chapter preserves the numbered Slice/gate contracts and their original protocol
baselines. They are not a report of tests run on the current worktree. Current public
V3 requirements are in [session-endpoint](../../../spec/session-endpoint.md#routes-and-streams), with
[endpoint mux tests](../../../crates/endpoint/tests/session_endpoint.rs) and
[cross-crate mux tests](../../../crates/conformance/tests/session_endpoint.rs).
Retained V2 tests do not establish public V2 availability. Current test targets are
listed in the [generated atlas](../../architecture/generated/index.md).

The normative test definitions (R4-8; extended R5: 28–32; R6: 33 + repairs). Every test names setup, action,
assertion, and its doc anchors; fixtures are language-neutral JSON (D-59).
**Slice-1 gates** are marked ▸, **Slice-2 gates** are marked ◆, and
**Slice-3 gates** are marked ◇; **Slice-4 gates** are marked ■; **Slice-5
gates** are marked ⬟; **Slice-6 gates** are marked ●; **Slice-7 gates** are
marked ◉; **Slice-8 gates** are marked ⬢; **Slice-9 gates** are marked ◎;
**Slice-10 gates** are marked ◆◆; **Slice-11 gates** are marked ✦;
**Slice-12 gates** are marked ◈; **Slice-13 gates** are marked ⬣;
**Slice-14B safepoint gates** (⟲, removed 2026-09-06) are retired; **Slice-14C schedule gates** are
marked ⏱; **Slice-14D thread-search gates** are marked ⌕; **Slice-14E
Web-chain gates** are marked ⌁; **Slice-14F Client-extension gates** are
marked ⊕. Tests
25–27 belong to the external evolution
pipeline ([Evolution mechanism](../../history/evolution.md)) and are listed for ownership, not kernel CI. Separately, the
Slice-10 release job runs its in-repository selector gates and requires signed
production-harness evidence for 72 and 76; none is an optional skip.

Conventions: "crash at every boundary" = kill the process between each
adjacent pair of named steps and run recovery (sweep/respawn) after each;
"assert exactly one X" = across all retries and recoveries combined.

## Thread semantics coverage

The current thread model is [Thread definition](../../concepts/thread.md); the focused rule-to-test
matrix and actual execution results are in [thread-alignment](../thread-alignment.md).
The older numbered gates below retain their fixture identities. Root validation
adds candidate admission, bounded same-context repair, exact output promotion
and crash-replay obligations; a historical completion gate alone does not prove
all of these. In particular, deployment "External pipeline" gates near the end
are not the root executor/validator loop.

## Fixtures contract

`fixtures/` at the repo root is normative and language-neutral (D-59), with
named cases per corpus, inventoried by `fixtures/manifest.json` (R12-10):
`fixtures/events/<case>.canonical.json` — every core kind and union arm per
[spec/event](../../../spec/event.md), canonical per RFC 8785 (test 28);
`fixtures/wire/*.jsonl` transcripts — negotiation (`hello-ok`,
`hello-disjoint`), every message, dedup, unknown, plus `malformed.txt`
(test 31); `fixtures/torn/` — half-line, bare-LF, invalid-UTF-8, and
`interleaved-loss` (test 29);
`fixtures/replay/` — checkpoint-valid and checkpoint-invalidated ledgers
(test 30); `fixtures/checkpoint/` — exact dependency prefix, canonical
format-1 state asset, seeded-replay ledger, and invalidation fallback ledger
(tests 38–42); `fixtures/tools/` — the exact builtin-tools inventory,
argument order, availability, backend/effect classes, separate migration audit,
and separate shipped-extension inventory (test 43);
`fixtures/endpoint/` — the pinned Client-v2 corpus/hash lock, exact stable
projection journal, history/live stitch cases, and typed native mutations
(tests 47–52);
`fixtures/provider-runtime/`, `fixtures/tool-runtime/`, `fixtures/web-tools/`,
`fixtures/endpoint-transport/`, `fixtures/deployment/`, and
`fixtures/mcp-runtime/` — the required-case registries for tests 53–76 and
86–92; each later slice completes its immutable raw
transcripts before implementation and adds them to the manifest before the
corresponding capability may advertise;
`fixtures/schedule/` — canonical schedule definitions, cron/DST outcomes,
closed invalid cases and claim/restart/missed-policy recovery cases (tests
101–104);
`fixtures/provider-dialects/` — fourteen exact vendor/model/route profiles
(thirteen advertised, one test-only), their canonical request/SSE/terminal/control
bytes, multi-turn/recovery cases, forward-compatible provider-vocabulary
replay, and fail-closed structural/mismatch corpus (tests 93–96);
protocol-family fixtures in
`provider-runtime/` are not substitutes;
`fixtures/resources/` — pinned predecessor skill/command evidence,
whole-package precedence, catalog, expansion, negative and keyed-input oracles
(tests 77–80);
`fixtures/invalid/` — self-contained one-violation ledgers
covering the validation-fold rules (constraint 5 is spec-level — no
ledger case), with a `_valid-baseline.jsonl` positive control and
EXPECTED.md naming each rule (test 28's negative half — R13-10/R14). The
corpus is hand-authored and committed **before** any
implementation; every implementation replays it unchanged, byte-for-byte. **The oracle is independent of every encoder**
(R7-7): the expected-byte cases are hand-authored from RFC 8785 rules
before any implementation exists — number edge cases, UTF-16 code-unit key
ordering, Unicode escaping, lone-surrogate rejection — and canonical bytes
exclude the JSONL LF framing (the file adds it). Slice-1 encoders must
reproduce these bytes; no implementation may generate its own oracle.
