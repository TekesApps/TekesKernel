# Native public queue repair (Kernel #86)

The 0.9.4 candidate restores the documented `session/queue` producer. Durable live
input events are projected as complete public queue snapshots after prompt
admission (before scheduling the worker) and after reconciled ledger changes.
Consumed/superseded inputs disappear; source RPC identity and content are retained.
The control baseline restores known queue projections without folding untouched
catalog sessions. Ledger length changes invalidate recovered sidecars; stale folds
are rejected before publication. Pinned mux capacity does not fail a durable
reconcile after the control cache has been updated.

Only supervisor queue publication is changed. Endpoint wire version remains 4.
No sandbox, provider, tool or Swift/SDK behavior is changed. Generated architecture
indexes reflect the source/version changes.

Local checks: workspace strict Clippy; supervisor library tests (158 passed,
12 existing ignored); all 19 lazy-boot stream tests. Initial mapping test passed,
but the earlier candidate also produced two session-limit failures; those negative
logs remain in AgentKit's private SSD evidence alongside the corrected results.
Independent final source review found no blocking defect. None of these is the
real busy-queue acceptance: exact-source binary/protocol, performance and final
CI must be recorded before release. Old Kernel0.9.3 queue failures stay valid at
their original identities.

The first real protocol attempt stopped at a stale fixture version assertion before
the workflow; the exact 0.9.4 expectation was corrected in a new fixture snapshot.
The next attempt received real nonempty queue facts but rejected extra
`submission` metadata. The producer now keeps the documented four-field imported
queue message; no queue assertion was relaxed. Both failed attempts and process
retirement receipts are preserved; a new exact-source replay remains required.
