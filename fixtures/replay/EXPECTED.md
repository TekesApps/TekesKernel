# replay — checkpoint markers and origin keys (R12-10/R14)

`checkpoint-valid.jsonl`: genesis -> two queued inputs -> `turn_open`
consuming seq 2 (seq 3 stays queued; prefix-completeness only binds seqs
below the maximum consumed) -> settle -> checkpoint covering 1..5. A
checkpoint is a plain covering marker (the boundary a later `compact` may
cover); replay always starts at genesis and rebuilds the origin-key map from
the events.

`checkpoint-invalidated.jsonl`: adds a `queue_edit` superseding the still-
queued input seq 3 - inside the checkpoint's covered prefix. The full-genesis
fold accepts the ledger; the retraction is ordinary and the checkpoint stays
a valid marker (event checkpoint rule; doc 12).

`cancel-before-start.jsonl` (R15-2): a stop echo lands in an unstarted
child (genesis with parent, no turn) — the generation is OPEN, not
born-closed; the gate-exempt reconcile-only run opens turn 1 with
`trigger: "genesis"` solely to settle it interrupted/recovered, which
closes the generation. Valid under the full fold.
