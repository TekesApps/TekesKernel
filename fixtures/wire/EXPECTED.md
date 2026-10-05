hello-ok/hello-disjoint: negotiation transcripts (existing). unknown-message:
receiver ignores the unknown key and continues. A malformed (non-JSON) line
is a protocol error: worker exits 76 / supervisor kills and reconciles —
see malformed.txt (not JSONL by design).

R13-10 additions: appended/frame/state — coalescible mirror messages
(frame shows two deltas for one block; droppable by the supervisor only).
lease-deny: denial means wait or abandon per budget. launch-result-error:
`ok: false, "missing"` → the parent appends `child_result {failed}`.
approval-deny: grant:false pairs the ask call with a denied result.
unknown-then-known: the receiver ignores the unknown key and keeps
processing later lines. malformed.txt line 2 is a multi-key object —
a protocol error like non-JSON (one top-level key per line).
launch-mismatch: the request's spawn_id differs from the child genesis's
parent.spawn_id binding — {ok:false, "spawn_mismatch"}; the parent
appends child_result {failed} (R15-10).
meta: keyed rename/labels mutation (R17k) — the worker appends the
externally-originated `meta` barrier (origin tuple recorded, manual-compact
pattern) and receipts only after it; ownership/upgrade are never accepted
from clients. With no live worker the supervisor performs the locked append
directly (D-4) — no wire round-trip.
