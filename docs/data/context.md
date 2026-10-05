# Model context: projection, epochs and compaction

[Data and models · documentation home](../README.md) · [Execution flow](../flows/turn.md) · [Next: worker implementation](../../crates/worker/docs/README.md)

A projection is a pure, deterministic, rebuildable function over an event
prefix. All reading of thread state is projection; nothing ever reads thread
"state" that isn't derived from events (other state derives from its own
named authority — D-1). Corollary: **the log is the sole truth of thread
history — the moment any derived artifact becomes load-bearing truth, a
second store and a sync seam have been recreated.** That is the bug class
this architecture exists to kill.

## Provider render (the hot one)

`(event prefix, dialect, config) → wire bytes`. The sections below explain rendering constraints, compaction, epochs,
visibility, and baseline/continuation. Start with the properties rendering must preserve:

- Carried over from TekesRuntime: `Orchestration.Compiler.*` (context
  rendering, prompt assembly, tool choice, continuation) and
  `Network.ModelAPI.*` (the predecessor's eight concrete adapters, normalized
  into provider-runtime's five protocol families), `CacheShape`.
- **Determinism is a hard contract**: stable JSON key order, stable block
  ordering, no timestamps or randomness in rendered output. The 2.6% → 93.9%
  prompt-cache lesson lives here.
- **Prefix stability**: within one epoch, appending events only extends the
  admitted conversation item sequence; earlier items and sealed native
  fragments remain unchanged. This is not a byte-prefix requirement on the
  entire HTTP JSON envelope, whose closing delimiters and per-request controls
  necessarily change. Epoch resets (the CacheResetReason taxonomy) are the
  *named, deliberate* exceptions, recorded as events.
- Rollback/retry: `supersedes` ranges are excluded from the render — history
  is never edited, only re-projected.

## Compaction

One lossy projection replaces the four coexisting compaction generations.

- A `compact` event declares `covers` ranges plus a replacement summary.
- Renders skip covered ranges and emit the summary; covered events remain on
  disk for audit, fork, and re-compaction.
- Because compaction is *only* a projection marker, changing compaction policy
  never migrates data — write a new `compact` event superseding the old one.
- Lesson encoded: a compact summary must carry enough semantics to continue
  the conversation (the /compact semantic-loss failure is a summary-quality
  bug, unfixable by machinery — so keep the machinery trivial and iterate on
  the summary).
- Iterating on the summary is the model summary request: the planned covers
  are frozen as a source bundle, one compactor-role request asks the model for
  a continuation with `seq` evidence addresses inside the bundle, an admitted
  continuation is written as the summary and the verdict rides on the
  `compact` event as `summary_request`; the deterministic quoted history is
  the fallback when the request fails or the artifact is rejected
  ([spec/event](../../spec/event.md) §compact, `engine::compaction_summary`).

The concrete definition of the hot projection sketched in [Context projection](context.md) §1.
It is the direct successor of TekesRuntime's provider-context ledger contract
(REQ-014, `provider-context-ledger.md`): same invariants, simpler carriers —
a line file is single-writer and totally ordered by `seq`, which collapses the
hardest machinery (consumed-event proofs, plane merging) into integers and
tables.

## Shape

```
render(prefix, dialect) =
  frozen head        exact SystemProfile + control envelope + startup tool set
                     (bytes fixed at epoch start, stored once)
+ sealed baseline    the provider conversation already admitted in this epoch
                     (continuation identity, or replay of already-projected items)
+ new events         model-visible events with seq > baseline coverage,
                     projected in seq order by the kind table below
```

A pure function of `(event prefix, epoch profile, dialect)`. No wall clock, no
randomness, stable key order; the renderer version is part of the epoch profile,
so changing the renderer is an epoch, never a silent re-render. A durable
epoch is authoritative from its append; attempt-less trailing epochs are
inert (event §Authority — R13-5).

## Epochs

An `epoch` event opens each provider-conversation segment and freezes:

- adapter / endpoint / model identity,
- the exact native system + control envelope (stored once, large bodies as an
  asset ref — restart never depends on re-rendering or a digest alone),
- the ordered startup tool set, stored the same way (asset ref plus digest,
  content-addressed so an unchanged catalog costs one blob per thread): a live
  MCP server's schemas are not derivable from config and code, so a digest
  alone would leave the request unreconstructable,
- reasoning/continuation policy, permission/policy identities, renderer version,
- `reason` — from the closed reset vocabulary (model or adapter change, system
  source change, tool-profile change, permission/policy change, compaction,
  rollback/retry retraction, revocation, attachment-protocol change,
  continuation-expired recovery, unresolved-dispatch recovery (R7-3), …;
  carried over from today's CacheResetReason taxonomy).

Rules:

1. Render always starts at the **latest** `epoch` event. Events before it
   reach the model only through the epoch's **snapshot** — itself a projection
   of the **admitted and anchor** history before the epoch event (this is how
   seeds, carried-forward anchors, and compact summaries enter request 1).
   Pre-epoch host events that were never admitted or are still withheld do
   **not** bake into the snapshot: the epoch event records a pending-host
   inventory `{eligible: [seqs], withheld: [seqs]}`, those events survive as
   themselves, and later attempts admit them under their original seqs
   (R3-8) — an overflow retry re-sends the steer, and a withheld input stays
   withheld, across the epoch boundary.
2. Within an epoch nothing before the tail is regenerated, moved, or edited:
   for stateless adapters, the projected item list for prefix P is an exact
   prefix of the list for P+Δ (semantic item order, not HTTP byte order).
   Protocol-required per-request controls (Google-style scoped system/tools)
   retransmit byte-identical epoch values and are not events.
3. The worker writes the first `epoch` event after `genesis`/seed, before the
   first provider call — freezing the profile is part of startup, and children
   therefore never inherit a parent's provider baseline (each file opens its
   own epochs).
   A genesis seed snapshot is read in its canonical asset-line order;
   source seq/turn/correlation fields are provenance only. The child renderer
   imports normalized model-visible payloads, never the source continuation or
   sealed-provider baseline, and the snapshot's SHA-256 participates in the
   first epoch profile (event §Seed snapshot asset format).
4. Anything that cannot keep old items visible-and-valid closes the epoch with
   a named reason. Cache reuse never overrides correctness or revocation.

## Visibility: the kind table

Visibility is a schema dimension — kind defaults, overridable per event with a
`visibility` field. Unknown kinds default to runtime-only, so old renderers
never leak new event types to a provider.

| kind | default | projected native item |
|---|---|---|
| `input` | model | user message (assets → provider image/file blocks) |
| `output` | model | assistant message (text; reasoning per dialect policy) |
| `reasoning` | dialect-dependent | native reasoning replay where the provider requires it |
| `tool_call` | model | native tool-use item (call id preserved) |
| `tool_result` | model | native tool result (paired; `{aborted: "crash"}` renders as an error result); for a call that has a `spawn`, the generic lifecycle-closing result is suppressed and the `child_result` row below is the sole provider result |
| `child_result` | model | the tool result paired with the spawning `tool_call` — spawn/child_result *are* the pairing |
| `state` (goal, validation.feedback, memory.delta, tool.offer, tool.activated, …) | model | typed state item (provider-appropriate carrier); never a rewritten system. An **unknown subkind of `state` still renders** as a generic typed item — the kind carries visibility, so forward-added visible facts are never silently hidden (F20). Whole-kind unknowns stay runtime-only |
| `approval_request` / `approval_response` | runtime | — (a denial surfaces as the tool's error result) |
| `error` (all classifications — R13-9) | runtime | — (retry/terminal history is ledger-only; clients surface it, the model sees consequences through the turn structure) |
| `compact` | never directly | enters the next epoch's snapshot as the summary |
| `genesis`, `epoch`, `settle`, `spawn`, `checkpoint`, `attempt`, `attempt_dispatched`, `attempt_recovery`, `stop_requested`, `queue_edit`, `run_start`, `effective_execution`, `meta` | never | — (exhaustive for core marker/runtime kinds — R3-12/R7/R9/R11) |

## Admission, baseline, and continuation

*(Rewritten after review R1 — the earlier single-integer `covers_through`
claim was wrong: admitted events are not a contiguous seq prefix once steers,
queued inputs, and post-output tool events exist.)*

**Admission model (D-29/D-43).** An event is *admitted* when it has entered
this epoch's provider conversation:

- **Provider-born events** — `output`, and `tool_call`s the provider emitted —
  are admitted at birth: the provider already knows them; they are never
  resent to a server-managed continuation, and stateless replay renders them
  from their sealed fragments.
- **Host-born events** — inputs, tool_results, state deltas — are admitted by
  the `attempt` that rendered them: each `attempt` records `admits`
  seq-ranges. Cumulative admitted set = union of `admits` over attempts
  **settled by a durable `output`** in this epoch — an attempt settled by an
  `error`, or still unpaired, contributes nothing: a failed send must leave
  its events unadmitted so the retry re-renders them (R2-1).

Request N+1 therefore = baseline continuation + every **eligible**
model-visible host-born event not in the admitted union, in seq order.
Eligibility for inputs is consumption-bound (event constraint 4 —
R15-3): a non-steer input is eligible only for the turn whose `turn_open`
names it; a steer only for its target turn; every other queued input is
ineligible regardless of visibility — it cannot be skipped, double-sent,
or rendered early. In the common case (no steer, no mid-turn queue)
the union is a contiguous prefix and behaves like a single watermark.

**Baseline carrier.** Every settled `output` stores: the attempt id it
settles (D-43); the provider continuation identity for server-managed
adapters; and **sealed native fragments** (D-42) — the exact adapter-owned
provider items, opaque reasoning signatures included, versioned and
asset-spilled when large. Stateless replay renders sealed fragments verbatim;
normalized content is for projections and UI, never for provider replay.

- A candidate request that never produced a durable `output` never advances
  the baseline; its `attempt` settles via an `error` naming it (D-43), and an
  unpaired attempt takes the adapter's three-way recovery ([Idempotency rules](idempotency.md)). A terminal
  `error {unresolved_dispatch}` additionally **closes the epoch** (R7-3):
  the possibly-advanced server continuation is never used again, and the
  next ordinary run opens a named recovery epoch re-rendering from the
  projection.
- An expired or incompatible continuation identity, or an absent /
  version-incompatible sealed carrier, is an incompatible baseline: **fail
  closed** into a named recovery epoch — never silent re-normalization.

## Supersedes vs. prefix stability

Appending cannot un-send bytes. The reconciliation rule:

- An event whose `supersedes` ranges cover **only events not yet admitted**
  (outside the admitted union) is free — the retracted events simply never
  render.
- A supersede covering **already-admitted** events closes the epoch
  (`reason: rollback` / `retry`); the next epoch's snapshot is built from the
  post-supersede projection. History on disk never changes; what changes is
  which epoch the provider conversation lives in.

## Compaction interplay

`compact` always closes the epoch (`reason: compaction`). The new epoch's
snapshot = the compact summary + **anchors materialized verbatim** + still-valid
durable facts. Anchor is a kind/event property (the initial user input, the
latest accepted ask-user answer, the latest task result per task —
the worker's semantic anchor groups). The worker derives these from admitted
inputs and accepted tool/child results, excluding superseded facts. Retaining
a tool result also retains its complete assistant tool-call batch and paired
results. Anchor selection used for replay is frozen at the epoch boundary;
a later result cannot retract an already-admitted item in that epoch.
Explicit `anchor:true` remains supported. Anchors survive every compaction
unchanged, which is the structural fix for compaction semantic loss. Covered
originals stay on disk, excluded from render, available to fork and audit.
The normalized provider item for each active compact summary is fixed as
`{role:"user", content:[{type:"text", text:<summary>}]}` and is placed before
the surviving event items, irrespective of the compact event's tail seq. The
summary text itself begins with `[compacted history]`; adapters translate this
normalized item without provider-specific reinterpretation.

## Determinism obligations (test surface)

1. Same prefix, same epoch profile, same dialect → byte-identical wire.
2. Prefix-of implies projected-prefix-of, per adapter, within an epoch.
3. Every epoch transition carries a named reason event; a changed wire
   without one is a defect (the "changed view without a durable delta" rule).
4. Provider-reported usage remains the only cache-hit authority; semantic
   append equality never claims a hit.


The worker context renderer is version 2 for semantic-anchor preservation.
An older renderer epoch is incompatible and opens a `renderer_change` epoch;
provider adapter serializer revisions remain separately tracked. This avoids
silently replaying changed context through an old continuation identity.

## Client context occupancy

Session model catalogs expose optional `contextWindow` from the effective model
configuration. The control projection `contextUsage` contains optional
`usedTokens` and `contextWindow`, `basis` (`reported`, `estimated`, `upperBound`,
or `unknown`), and the effective provider/model when known. Unknown occupancy
is null, never a measured zero; cumulative billing totals are a separate metric.

The supervisor currently uses the Kernel conservative request-byte policy as an
upper bound for compatible text requests and subsequent event/asset bytes. It
invalidates this projection after compaction, epochs, replacements, configuration
changes, unreadable assets, opaque media or server-held continuation state. A
later compatible prepared request restores the bound. This is a presentation
reference, not a tokenizer result or a new compaction gate.

Values are rederived from the ledger and current configuration. The per-session
`context-usage.projection.json` persists the publication clock and last value so
control reconnects/restarts cannot reuse an older sequence. It is not a history
or execution authority. Its independent control sequence is not inserted into a
journal snapshot's durable cut.
