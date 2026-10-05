# Core execution and tool gates

[All gates](README.md) · [Running tests](../running-tests.md)

This page retains the gates' original numbers and applicable baselines. It defines conformance rules, not the results of this run.

## Creation, delivery, idempotency

1. ▸ **ColdBootCreateRetry** — Setup: empty root. Action: keyed create;
   crash after durable `genesis`, before reply; retry the same tuple; also
   issue two concurrent same-tuple creates. Assert: exactly one folder, the
   retry and the loser both receive the first thread id. `[Thread definition R2; 02
   genesis; Supervisor §Client interface; Idempotency rules §Rule 2]`
2. ▸ **DeadTargetKeyedDelivery** — Setup: settled thread, no worker. Action:
   keyed submit; crash at each append→sync→ack→ensure boundary. Assert: one
   `input`, no ack before its barrier, retry re-acks the original seq.
   `[Supervisor §Delivery; Idempotency rules §Rule 2; D-45/48]`
3. ▸ **AliveTargetReceiptCorrelation** — Setup: live worker, two deliveries
   in flight. Action: drop one receipt; kill the supervisor after the other
   append. Assert: retries dedup to original seqs; no ack without its
   correlated receipt; doorbell coalescing never substitutes. `[Worker lifecycle §stdio;
   Supervisor §Delivery; Idempotency rules §Rule 4; D-48]`

## Provider path

4. ▸ **AttemptLeaseCommit** — Setup: one admission slot. Action: run the
   fixed order (id → lease → attempt barrier → dispatch barrier → HTTP →
   terminal); crash at every boundary; additionally deliver an
   `attempt_settled` for an unknown/void lease. Assert: crash-while-queued
   leaves no attempt; post-barrier crash settles `not_dispatched`;
   post-dispatch takes three-way recovery with a durable `attempt_recovery`;
   lease frees only by `attempt_settled` or reap; the stale release is an
   idempotent no-op, never a protocol error (R8-3). `[Worker lifecycle §Budgets/stdio;
   Event definitions §Core kinds; Idempotency rules §Rule 3; D-43]`
5. ▸ **OutcomeUsageCrashWindow** — Setup: one attempt reaching a terminal
   outcome; run twice — once ending in `output`, once in terminal `error`.
   Action: execute the normative transaction `append the outcome carrying
   usage → one barrier → attempt_settled`. Assert: an outcome without
   `usage` is not schema-valid, so a durable settled outcome always has its
   usage (the error variant carries `availability: unavailable` when the
   provider gave no figures) and there is no crash window between the two;
   no baseline advance, no `attempt_settled`, no lease release before the
   barrier. `[Event definitions §Durability/usage; Worker lifecycle
   §Loop/Startup; R4-1; R6-3]`
6. ▸ **CompletedTurnUsageExit** — Setup: turn with two provider calls and a
   tool. Action: run to completion. Assert: every outcome carries usage;
   exactly one `settle {completed}`; immediate exit (no idle linger); turn
   aggregate equals the projection over outcome usage.
   `[02 usage; Worker lifecycle §Waiting; D-12; R3-4/5]`

## Tools and hooks

7. **MutatedToolWriteAhead** — Setup: pre-hook that mutates a
   side-effectful call. Action A: execute; crash before `tool_result` —
   assert replay reads the full effective invocation from
   `effective_execution` and reconciles without blind re-execution. Action
   B (named variant): execution succeeds but the secret scanner fails —
   assert a terminal `tool_result {withheld: scan_failed}` still appears
   and pairing survives. `[Tool execution §Hooks/§Backends; D-19/46; R3-3]`
8. **AskUserFastAnswer** — Setup: ask-user hold. Action: answer; redeliver
   the same key. Assert: the `approval_response` barrier precedes its receipt;
   one anchored `tool_result` is paired to the call; the duplicate re-acks
   the original seq; the anchor survives a subsequent compaction.
   `[Tool execution §Ask-user; Model context §Compaction; Idempotency rules §Rule 2]`
9. **SpawnValidatorOrdering** — Setup: parent spawning executor then
   validator. Action: crash across child-create → spawn → launch → settle →
   result. Assert: pre-register crash leaves only a harmless orphan; one
   `child_result` per call id before any parent settle; startup reconciles
   spawn edges before generic tool calls. The seed snapshot is exact
   canonical source-event JSONL in increasing source-seq order and contains
   the durable `state {subkind:"delegation"}` with the post-hook effective
   invocation (never the pre-hook provider arguments); its final-LF
   SHA-256 matches both `digest` and the asset name, and those source seqs
   never enter the child fold; `seed.kinds` is the sorted unique set of kinds
   actually present; every nested asset reference resolves within the child
   folder and remains live under GC. A lost launch/report reply reuses the
   same spawn binding and the exact parent line is reawakened; repeated
   recovery still authors exactly one `child_result`. `[event §Seed
   snapshot asset format; tool-runtime §Fixed-family obligations; 01 rule
   4; 03
   §Startup; Turn flow §Spawning; D-13/44; R2-6]`
10. **DescriptorFirstFileOpen** — Setup: path swapped to FIFO/symlink
    between validation and open. Action: fs-helper read. Assert: no block,
    no escape beneath roots; unkillable in-process timeout converts to
    worker exit. `[Tool execution §Anti-hang; D-57; R3-22]`
11. **SandboxBackendProbe** — Setup: sandbox backend removed/rejected.
    Action: side-effectful exec. Assert: fail-closed refusal unless a
    durable per-run unsandboxed approval exists. `[Tool execution §Sandboxing; D-56;
    R3-21]`
43. ■ **BuiltinToolManifestParity** — Setup: load the canonical
    `fixtures/tools/builtin-tools.canonical.json`, its separate migration
    audit fixture, and the pinned legacy source-repository inventories (read
    from the fixture's exact commits, never sibling working-tree bytes). Action:
    validate the closed single catalog and construct default,
    missing-credential, benchmark, plan, validator, subagent, compactor,
    and deferred-catalog profiles; inject fixed/dynamic and
    dynamic/dynamic name collisions. Assert: canonical/sorted/unique format-1
    bytes; historical owner/source/exposure/schema fields reject; exactly 26
    callable fixed names; exact schema name, argument order, backend and
    effect parity; `apply_patch` is the sole fixed file-edit schema; conditional tools are absent
    when unrunnable; role subsets are exact; collisions fail launch; every
    model-visible descriptor mutation changes catalog identity; all nine legacy
    replacements are explicit and neither migration inventory has an omitted
    or extra name; the separate shipped
    extension inventory matches all 12 Computer Use MCP tools without making
    them fixed. `[builtin-tools;
    Tool execution §Tool catalog; Migration background §Carry; D-55/56/57]`
44. ■ **ExecHelperProcessProtocol** — Setup: real helper executable and
    canonical helper transcripts. Action: negotiate, run descriptor-first
    operations, cancel and crash at each write boundary. Assert: exact framing,
    confinement, atomic publication, bounded output and process-group reap.
    `[exec-helper; Tool execution §Anti-hang]`
45. ■ **SandboxProbeAndApprovalBinding** — Setup: supported and missing/rejected
    sandbox backends plus a durable grant. Action: derive and launch each
    profile. Assert: deterministic profile bytes, fail-closed unavailability,
    and approval bound to the effective invocation. `[sandbox-profile;
    builtin-tools §Approval mapping]`
46. ■ **HookProcessLimitsAndCorrelation** — Setup: real pre/post hook process
    fixtures. Action: exercise mutation, deny, timeout, oversize and malformed
    outputs. Assert: exact correlation/order, bounded process lifetime and
    fail-closed malformed/limit handling. `[tool-hook]`
