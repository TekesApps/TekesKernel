# fixtures — the hand-authored oracle

Authored before any encoder exists (R7-7/R11-6/R12-10). The values are
hand-authored; their canonical bytes were derived with a small reference
JCS serializer (RFC 8785: object keys sorted by UTF-16 code units, compact
separators, no ASCII escaping, integers, ES6-serialized doubles, `null` —
R13-3) — a reviewable derivation tool, **not** the implementation under
test. Implementations must reproduce
these bytes; none may generate or overwrite them. Canonical bytes exclude
the JSONL LF; stored files end each line with one LF.

- `manifest.json` — machine-readable corpus inventory (R12-10).
- `events/` — one canonical envelope per core kind plus every
  settle/tool_result/attempt_recovery/error union arm, all five `Block`
  variants, spill cases for every `X | spilled` position (16384-inline /
  16385-spilled boundary included), visibility/anchor/supersedes/
  min_reader cases, JCS escaping, full-`JsonValue` (null + ES6 number
  forms), number edges, and the UTF-16 key-order case (a non-BMP key
  sorts before a high-BMP key). R14: `turn_open` (both triggers), every
  `input.source` arm, every `child_result` outcome, bounded resume
  (genesis with parent/seed/instruction, and spawn), epoch `pending`,
  explicit `visibility: "model"`, an `error: true` tool-result block,
  `usage.cache_read`, an answer-carrying approval denial, a boolean in
  `JsonValue`. `-0` canonicalizes to `0` (no distinct fixture is
  representable); the spill boundary is exercised once — the rule is
  position-uniform.
- `assets/` — two real content-addressed assets: the 16,385-byte
  JSON-encoded string backing spilled reasoning, and the canonical
  source-event JSONL seed snapshot referenced by `genesis-bounded`. All
  filenames equal SHA-256(content); the seed asset ends in LF. The seed retains source seqs as
  provenance and is not a child ledger. Every other asset id is unresolved by
  design (the asset-absent branch).
- `wire/` — worker-control base messages: negotiation, every message, dedup
  receipt, text/tool stream frames (including the tool `call_id`), unknown
  message, malformed line (`malformed.txt`).
- `wire/durable/` — worker-control durable-control transcripts and receipts
  (queue transaction, tool control, exact re-ack, control receipt) with their
  closed case registry; `scripts/check-worker-control-fixtures.py` verifies it.
- `torn/` — half-line, bare-LF, invalid-UTF-8, and interleaved-loss damaged
  suffixes.
- `invalid/` — rejection corpus: self-contained one-violation ledgers
  with a `_valid-baseline.jsonl` positive control; `EXPECTED.md` names
  each violated rule (R13-10/R14).
- `replay/` — checkpoint-marker ledgers (with and without a supersede into
  the covered prefix) and the cancel-before-start child.
- `config/` — config canonical authority documents plus closed-schema,
  path, secret-material, transport-shape, and safe-integer rejections.
- `instruction-tree/` — the portable two-level discovery tree used to derive
  the instruction snapshot oracle; project cwd identity comes from its
  ordered fixture root, never an absolute path embedded in the snapshot.
- `instructions/` — the canonical instruction snapshot and independent
  settings/index/effective-projection/source-order rejection cases.
- `hooks/` — tool-hook pre/post requests, every verdict arm, and protocol
  rejection cases.
- `helper/` — exec-helper negotiation, request/result/error transcripts,
  and malformed framing.
- `sandbox/` — sandbox-profile policy/profile/plan golden bytes,
  unsandboxed approval binding, and policy rejections.
- `tools/` — builtin-tools's complete fixed-name manifest and locked
  five-field catalog digest, separate migration audit fixture, current
  legacy-repository migration provenance, ownerless single-catalog argument
  order, availability, backend, and effect classifications, plus the separately
  versioned shipped first-party extension inventory.
- `endpoint/` — the pinned byte-identical Client-v2 authority corpus and hash
  lock, Kernel-ledger→stable-journal oracle, stitch/gap/overlap cases, and
  typed mutation/archive/unarchive cases for Slice 6.
- `provider-runtime/` — Slice-7 adapter/parity and failure-case inventory; raw
  request/response/stream/result bytes must be completed before an adapter is
  advertised.
- `web-search-provider/` — the independent `tavily_v1` request, response,
  readiness, credential-scope, error-classification, and rejection oracle.
- `credential-broker/` — Slice-7 private descriptor-channel, scope, rotation,
  revocation, dedup and EOF inventory; fixtures use only a sentinel secret.
- `secret-store/` — SecretStore's closed Keychain record bytes, resolution
  classes and non-reuse of the endpoint bearer authority.
- `tool-runtime/` — Slice-8 exact 26-tool dispatcher coverage plus backend,
  effect, crash, approval, policy, secret, and dedup case inventory.
- `endpoint-transport/` — Slice-9 HTTP/WebSocket wrapping matrix over the
  unchanged Client-v2 DTO authority.
- `deployment/` — Slice-10 macOS install, ownership, fault, upgrade/rollback,
  redaction, and uninstall acceptance matrix.
- `resources/` — Slice-11 pinned AppServer/Runtime migration evidence,
  whole-directory user/project skill packages, command catalog/expansion
  oracle, negative arguments, and keyed-run request/result bytes.
- `plugins/` — Slice-12 generic lifecycle/gate authority. Historical
  TekesComputerUse-shaped bytes, where retained for migration comparison, are
  non-runtime evidence and do not bind the current standalone MCP executable
  to a plugin.
- `mcp-reference-qualification/` — Slice-14A's language-neutral four-gate
  lifecycle oracle: ordinary local stdio MCP configuration, exact
  12-tool argument/effect inventory, synthetic TCC denial/grant, prompt
  suppression, stop/restart/crash recovery and no-special-case source audit.
  Its source lock binds the external TCU revision, production signing tuple,
  canonical migration contract and exact projected real catalog without
  pinning timestamped archive bytes. It never controls a real UI or claims an
  interactive macOS grant.
- `client-extensions/` — Slice-14F's frozen twenty-method base, ten atomic
  additive capability catalogs, one request/result case per retained method,
  closed errors/negative cases, and exhaustive predecessor-route dispositions.
  It adds no SessionEvent or Computer Use family.
