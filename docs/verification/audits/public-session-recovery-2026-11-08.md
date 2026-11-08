# Public observation and recovery review

This review starts from Kernel 0.9.21, source tree `4e1bdfd` / main `10fb789`.
It compares the relevant Pi 1.5 and DSH 0.2.2 mechanisms with existing Kernel
contracts. It is not a proposal to copy their wire formats. Implementation issue:
[#133](https://github.com/TekesApps/TekesKernel/issues/133).

## Findings

| Area | Existing contract and evidence | Decision |
| --- | --- | --- |
| Atomic publication | `endpoint::Projector::reconcile` appends one event's projection slots as one durable journal batch. Baseline/publish handoff is synchronized. The context details/usage pair is installed together in control state; live journal deliberately emits two same-sequence frames. ClientKit normalization handles either arrival order and uses the details' embedded usage or invalidates stale details. | Keep the current wire contract. No demonstrated consumer mismatch justifies adding a second batch protocol. AgentKit/Web's missing use of these context fields is a consumer capability mapping question. |
| Passive observation / activation | Startup leaves old sessions deferred. Journal/history reads can verify and repair tails or sidecars, but do not release ordinary execution. `sessions.recover` explicitly releases selected sessions. Existing durable stop propagation remains a separate authorized cleanup exception. | Keep explicit recovery; no new `activate` flag is needed. Correct the false-success recovery receipt described below. |
| Tool result recovery / context compaction | The worker distinguishes unstarted safe work, uncertain side effects and approvals; context projection retains call/result pairing through trimming and compaction. Existing tests preserve the original provider failure scenarios. | Reuse the existing implementation and run selected regressions; do not build another recovery engine. |
| Fork / child / permission / history recovery | Ordinary fork lineage and explicit child edges have separate sources. Permission modes are persisted under the session folder and reread. Compaction preserves the durable historical prefix and reconnect reconstructs projections. | No new product defect established. A single combined external-process restart + permission + full compacted-history browser scenario is not established by this review's unit/integration tests. |

## Corrected receipt

Before this change, `ProductionProcessHost::recover_client_sessions` skipped a
quarantined session, but `ProductionClientExtensions` echoed every requested ID
as `recoveredSessionIds`. A mixed corrupt/healthy batch therefore reported the
corrupt session as recovered even though it remained deferred.

The host now returns only IDs whose state was verified and released. The existing
public JSON field uses that result. Healthy sessions continue to be released when
another session is quarantined. The receipt still means admission was released;
it does not promise worker startup or execution completion. Request preflight and
repeat-recovery behavior remain unchanged.

The regression calls the actual public extension authority with a corrupt ledger
and a healthy ledger. Old code fails on the returned ID list; the fix must also
preserve the corrupt session's deferred state, release the healthy session, and
retain correct results on repeated recovery without starting an idle worker.

## Verification scope

The finite selection covers the new public receipt regression, existing explicit
recovery/idempotency and read-side repair tests, atomic journal/control handoff,
compacted context pairing, the tool write-ahead crash matrix, approval-held batch
siblings, fork child-edge remapping, subagent catalog derivation and durable
permission-mode readback. Tests use local fixtures; no paid-model or performance
experiment is part of this batch. Daily instances and user data are not inputs.

The selected ClientKit normalization tests were inspected as source evidence, not
rerun or changed. This batch does not claim new Swift, GUI, model-quality, public
release or installed-instance acceptance.

Local result: the old receipt failed the new regression with both requested IDs
reported; the fix passed all 17 selected tests (one new receipt regression and
16 existing contract cases). Supervisor all-targets Clippy with warnings denied,
workspace formatting, generated-atlas freshness and atlas link checks passed.
The original failing output remains retained in the private batch evidence.
