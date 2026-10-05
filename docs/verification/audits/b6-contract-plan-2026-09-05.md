# B6 contract plan: Brief V2, compaction V5 shadow artifact, OAuth refresh lifecycle

Status: plan for Frank's ruling (2026-09-05). Three pinned legacy tests stay
`blocked_contract_gap` because each needs a new versioned contract, not more
evidence. Each section states what the legacy test proves, what Kernel has,
the proposed contract, the proof, the size, and the decisions only Frank makes.

## 1. Brief V2 — strict, reference-native brief (DONE 2026-09-05)

Status: implemented as planned; live proof passed on the Cloudflare OpenAI
route (`/tmp/tekes-brief-v2-20260905-2`). One deviation from the first live
attempt: the model initially left the antecedent fact out of `goal` and filled
`completion` explicitly, so the v2 tool description now states that the goal
lists every atom the work depends on (facts included) and that `completion`
stays empty unless it genuinely differs from the host derivation.

**Legacy (`live_openai_cloudflare_brief_v2_strict_mixed_antecedent`).** The
model calls `brief` once with atoms `{id, kind ∈ intent|fact|constraint, text,
source_refs}`, `goal.atom_ids`, `completion.atom_ids`; sources are aliases
`s1` (current immutable input) and `s2` (a V1 antecedent). Host reading proves:
kinds ⊇ {intent, fact, constraint}; every atom cites a source; the goal
references a fact; explicit completion empty; *effective* completion = goal
atoms minus facts (a fact stays useful without becoming completion); the root
input bytes are unchanged.

**Kernel today.** `brief` v1: `source`, `canonical_user_goal`,
`retained_items`, `completion_criteria`; the workflow backend echoes the
validated invocation; the only consumer is the compaction anchor rule.

**Proposed contract (builtin-tools-v1, `brief` schema version 2).**
- Arguments: `{schema_version: 2, atoms: [{id, kind, text, source_refs:
  [alias]}], goal: {atom_ids}, completion: {atom_ids}}`, strict (no additional
  properties), atom ids unique, every referenced id defined, `source_refs`
  non-empty and ⊆ the turn's alias table.
- Alias table, host-owned and projected into the model context as a `sources`
  block: `s1` = the inputs the turn admitted (exact bytes, never rewritten);
  `s2…sN` = antecedents: the latest accepted brief of each earlier turn
  (V1 `canonical_user_goal` or V2 goal atoms) and earlier inputs, newest first.
- Host rule `engine::brief_contract`: effective completion = explicit
  `completion.atom_ids` when non-empty, else `goal.atom_ids` minus `fact`
  atoms. The tool result carries `{schema_version, atoms, goal,
  effective_completion}`; the durable `tool_call` keeps the model's exact
  arguments.
- Selection: V1 remains the default; V2 is selected per workspace policy
  (`brief_schema_version: 2`) or per launch binding; no silent switch. Both
  versions stay compaction anchors.

**Proof.** Unit: schema acceptance/rejection (unknown kind, dangling id,
foreign alias, empty refs), effective-completion rule, alias-table projection.
Live: `scripts/run-public-flow.py --live --brief-v2` on CF OpenAI with the
legacy system framing (s1 current input, s2 V1 antecedent seeded by a first
turn), asserting the durable brief call exactly as the legacy test does, plus
the unchanged input bytes.

**Size.** ~1 day. **Decisions.** (a) selection knob: workspace policy vs
settings; (b) whether antecedents come from the ledger only (proposed) or may
be supplied by the client.

## 2. Compaction V5 — shadow summary artifact with evidence-address admission (DONE 2026-09-05)

Status: landed as planned with two deviations. (1) The telemetry lives on the
compact event itself (`compact.shadow`, event-v1) rather than a `state`
record: the supervisor-authored manual compact has no open turn, and `state`
is turn-bound. (2) The 50 % prefetch band is not implemented; the summary
request runs at compaction time (inside the attempt lease for auto-compaction,
before the line lock for the supervisor's manual compact), its rendering
capped at 60 % of the model window. A worker-authored manual compact at a
yield has no lease and carries no shadow. Live A/B (`run-compaction-shadow.py`,
`/tmp/tekes-compaction-shadow-20260905-2`): promote 4/4 codes recalled with the
artifact admitted and written as the summary, deterministic 4/4.

**Legacy (`live_compaction_v5_shadow_artifact`).** From a frozen source
bundle (exact record addresses, hashed), an independent summary request asks
the model for a continuation with `evidence_refs`; the host admits it only when
every reference is an address inside the frozen bundle; accepted artifacts are
shadow-only (telemetry, no projection change) until promoted; a real-model A/B
proves recall of four private codes after compaction.

**Kernel today.** Auto and manual compaction use `engine::plan_context_compaction`
(deterministic quoted history, semantic anchors); `summary_artifact` exists for
the compactor role but only echoes `{continuation, evidence_refs}`; no
independent summary request, no admission, no telemetry.

**Proposed contract (event-v1 + tool-runtime-v1 addendum, "compaction v5").**
- Source bundle: the planned `covers` ranges plus `sha256` over their
  canonical bytes, frozen before the summary request.
- Summary request: the worker runs one bounded provider request in the
  compactor role (tools: `summary_artifact` only; context = the covered events
  rendered; prompt asks for the minimal sufficient continuation with exact
  `seq` evidence addresses). Budget: prefetch when projected cost enters the
  50 % band of the window; the request itself capped at 60 % of the window.
- Admission: `evidence_refs` must be `seq` addresses ⊆ bundle covers, unique,
  non-empty; `continuation` non-empty and ≤ the deterministic summary bound.
  Result recorded as `state{subkind: "compaction.shadow", visibility: runtime,
  payload: {bundle: {covers, sha256}, continuation, evidence_refs, accepted,
  reason?, usage}}` — the telemetry the legacy test reads.
- Policy `settings.compaction_shadow: off | shadow | promote` (default
  `shadow` once landed): `shadow` records only and compaction keeps the
  deterministic summary; `promote` writes the accepted continuation as the
  `compact.summary` (deterministic text stays the fallback on rejection or
  provider failure). Replay is unaffected: the compact event is the authority.

**Proof.** Unit: bundle freezing, admission (foreign/duplicate/missing refs
rejected), promote/shadow/off behavior, fallback on rejection. Live
`scripts/run-compaction-shadow.py`: four seeded turns each carrying a private
code (legacy words), forced compaction under `promote` and under
deterministic, then one recall turn; report codes recalled per arm; the gate
asserts the promoted arm recalls ≥ the deterministic arm and all telemetry
rows are present.

**Size.** 1.5–2 days. **Decisions.** (a) default policy after landing
(`shadow` proposed); (b) budget constants (50 % / 60 % proposed, matching
legacy); (c) whether the summary request may run in the supervisor as a helper
instead of inline in the worker (proposed: worker, inline, before the compact
event, under the same lease).

## 3. OAuth — refresh-token persistence, rotation and revocation (DONE 2026-09-05, live gate passed)

Status: decisions (a) Kernel mints its own records, (b) one Keychain item per
grant (canonical JSON material), (c) exchange inside the connector's
authorization provider — all implemented (the proposed
`OAuthAuthorizationProvider` shipped as `provider::OAuthTokenExchange` behind
the transport's authorization provider closure). Hermetic proof green (provider
`oauth` tests, supervisor `oauth_binding_exchanges_kernel_grants_and_injects_platform_tokens`,
durable-floor write path). Live gate passed against Cloudflare
(`/tmp/tekes-oauth-live-5`): rotation on both connects (floor generation 3),
RFC 7009 revocation at the advertised endpoint answered 200, the third
connector failed with the exact legacy text and the record closed. Learned:
Cloudflare answers an overlapping refresh with 429 + Retry-After (honoured,
bounded) and the MCP client's one retry needs the failure sticky per connector. The macOS Keychain write path is exercised only by the
signed supervisor (access-group entitlement); the gate runs the memory item
lane behind the same durable floor.

**Legacy (`rpcOAuthPersistsRefreshTokenAndReconnectsAfterRestart`).**
Interactive OAuth start against `https://mcp.cloudflare.com` (browser), refresh
token persisted in Keychain; after a restart a fresh connector exchanges the
refresh token at the real token endpoint, a rotated refresh token is written
back, a second restart uses it, remote revocation then makes a new connector
fail with `token endpoint returned HTTP 400`.

**Kernel today.** mcp-runtime-v1: `oauth.start` records a pending reference
and returns the authorization URL; the platform installs the token and calls
`oauth_bind`; the SecretStore is read-only `resolve`; the HTTP transport
injects whatever bearer the store resolves. No exchange, rotation or
revocation handling.

**Proposed contract (secret-store-v1 addendum "OAuth secret mutation",
mcp-runtime-v1 "oauth" binding semantics).**
- Kernel gets a narrow, versioned mutation authority over records **it
  minted**: `mint_oauth(reference) → credentialId` (refresh token + client
  identity + token endpoint + resource, one Keychain item, generation 1) and
  `rotate(credentialId, new_refresh_token)` (atomic in-place update, generation
  bump, journaled). Platform-minted records stay read-only for Kernel.
- `oauth` bindings resolve through an `OAuthAuthorizationProvider`: exchange
  the refresh token at `token_url` (client id, resource, PKCE-less refresh
  grant), cache the access token in memory per identity generation, rotate
  when the response carries a new refresh token, and on `invalid_grant` /
  HTTP 400 mark the binding `revoked` (no retry, peer generation closed).
- Interactive start: still the platform's in production. For the Kernel gate
  a headless-with-browser helper `scripts/run-live-mcp-oauth.py` performs
  dynamic client registration + authorization-code + PKCE against Cloudflare
  (opens the system browser once, loopback redirect), mints the record through
  the new authority, then runs the Kernel gate: restart the supervisor process,
  reconnect (refresh exchange), assert catalog and a read-only `execute`,
  rotation persisted, second restart, remote revocation, and the final
  connector failing with the exact 400 mapping.

**Proof.** Unit: mint/rotate journal and atomicity, provider exchange against
a local token-endpoint fixture (rotation, invalid_grant), binding state
transitions. Live: the helper above (needs Frank at the keyboard once for the
browser consent).

**Size.** ~2 days plus one interactive session. **Decisions.** (a) allow
Kernel to mint OAuth records at all (or only rotate platform-minted ones —
then the live gate needs the Tekes app to do the start); (b) Keychain item
layout for the minted record (single item vs one per field); (c) whether the
Kernel gate may use dynamic client registration on Cloudflare's public MCP.

## Order

1 (Brief V2) is self-contained and cheapest; 2 (compaction V5) touches the
worker's compaction path and needs the A/B live run; 3 (OAuth) needs the
secret-mutation decision and an interactive step. Proposed order: 1 → 2 → 3.
