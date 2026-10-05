# invalid — rejection corpus (R13-10/R14)

Every case is a **self-contained ledger** derived from
`_valid-baseline.jsonl` (the positive control: 8 lines, genesis through
settle, accepted by a conforming validator) by exactly one mutation; a
conforming validator rejects it citing the named rule of
[spec/event](../../spec/event.md). Where a mutation inherently trips
an overlapping rule, that is declared here.

- `seq-gap.jsonl` — envelope `seq`: +1, no gaps (7 -> 9).
- `double-settle.jsonl` — constraint 1: two final settles for turn 1
  (constraint 8 trips too by construction — a settle is itself a
  turn-bound event after the first settle; filed under 1).
- `turn-after-settle.jsonl` — constraint 8 only: a `note` bound to turn 1
  after turn 1's settle.
- `origin-mismatch.jsonl` — constraint 6: input `origin_key` "kX" !=
  `origin_tuple.key` "k9".
- `forward-supersede.jsonl` — constraint 7: `supersedes` names seq 11
  ahead of the referring event.
- `null-schema-field.jsonl` — Conventions: schema-defined field `null`
  (`visibility: null`).
- `envelope-missing-field.jsonl` — envelope: required `ts` absent.
- `bad-thread-uuid.jsonl` — genesis schema: native `thread` is not a
  lower-case canonical UUID.
- `bad-ordinal.jsonl` — constraint 9: `recovery_ordinal` 5 with no prior
  reconcile run_starts.
- `spill-undersized.jsonl` — Spill rule: `$spill` with `bytes` 100
  (must be > 16384).
- `double-output.jsonl` — constraint 2: two outputs settle one attempt.
- `unpaired-result.jsonl` — constraint 3: `tool_result` with no
  `tool_call`.
- `admits-runtime.jsonl` — constraint 4: `admits` covers seq 3
  (`run_start`, never model-visible).
- `skipped-input.jsonl` — constraint 8 (prefix-completeness): `turn_open`
  consumes seq 3 while live unconsumed seq 2 is skipped.
- `double-consume.jsonl` — constraint 8 (exactly-once): input seq 2
  consumed by two turn_opens.

Constraint 5 (barrier-list equality with D-45) is a spec-level list
assertion, not a ledger predicate — it has no ledger case. Constraint 10
is the fold mandate itself, exercised by every case above.
- `turn-open-forward-input.jsonl` — constraint 7: `turn_open` names input
  seq 4 ahead of itself (R15-1); constraint 8's trigger check trips too
  by construction (a not-yet-appended maximum cannot be runnable) —
  filed under 7.
- `turn-open-held-as-maximum.jsonl` — constraint 8 (trigger check): turn
  2's maximum consumed seq 5 predates the born-closed generation's
  closure point (seq 7) — a held pre-closure input may ride, never
  trigger (R15-4).
- `admits-unconsumed-input.jsonl` — constraint 4 (input eligibility):
  the attempt admits input seq 3, which turn 1's `turn_open` does not
  consume (R15-3).
- `attempt-admits-future-input.jsonl` — constraint 7: the attempt at seq 6
  admits steer input seq 7. The steer is otherwise eligible for turn 1, but
  it did not exist when the attempt/wire digest was committed (R16).
- `epoch-pending-future.jsonl` — constraint 7 (R16r): `epoch.pending`
  names seq 7 ahead of the epoch at seq 5.
- `turn-open-consumes-superseded.jsonl` — constraint 8 (R16r): the
  `turn_open` names input seq 2, which the queue_edit at seq 3 already
  superseded — named inputs must be live at the turn_open's position.
- `queue-edit-consumed-target.jsonl` — constraint 8 (R16r): the
  queue_edit at seq 6 supersedes input seq 2, which turn 1's `turn_open`
  already consumed — retraction targets must be unconsumed. Together with
  the previous case, the consumption-vs-retraction race resolves by file
  order: whichever lands second is invalid.
- `origin-missing-key.jsonl` — constraint 6 (R16r, strengthened): the
  input carries a payload `origin_tuple` but no envelope `origin_key`.
