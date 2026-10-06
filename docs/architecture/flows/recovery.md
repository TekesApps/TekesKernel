# Recovery and replay

[Architecture entry point](../README.md) · [store](../../../crates/store/docs/README.md) · [engine](../../../crates/engine/docs/README.md)

Recovery inspects durable state and locks before choosing an action. Losing an in-memory running flag does not justify writing completed.

```mermaid
sequenceDiagram
    participant P as supervisor sweep
    participant S as store
    participant F as schema fold
    participant E as engine lifecycle
    participant W as candidate worker
    participant C as endpoint carrier
    P->>S: scan_valid_prefix_seeded
    alt valid checkpoint state available
        S->>F: restore_from_checkpoint + replay suffix
    else no usable checkpoint
        S->>F: replay valid ledger prefix
    end
    F-->>S: LedgerProjection / LifecycleFacts
    S-->>P: projection + tail repair facts
    P->>E: classify with lock facts / run_decision
    alt owned live execution or no runnable work
        E-->>P: do not launch duplicate execution
    else ordinary or reconcile work required
        E-->>P: selected run mode
        P->>W: launch through admission and backoff
        W->>S: acquire ledger lock / load projection
        W->>E: recheck facts and decision
        W->>S: append run_start
        W->>W: recover unpaired tools / provider attempt / pending validation
        W->>S: durable outcomes, settle only when justified
        W-->>P: appended / process exit
    end
    P->>C: repair_main_projection / publish status
```

A candidate worker must still acquire the lock and re-evaluate state; the supervisor's scan does not replace the writer's final check.
Corrupt complete events and partial tail records require different treatment; not every validation failure is a truncatable tail.
Recovery may only reconcile or may continue ordinary execution; resume policy, stops, approvals, pending input and validation obligations all affect the decision.

For process reaping, see `reconcile_after_exit`.
A checkpoint accelerates log folding.

## Source evidence

[daemon storage preflight](../../../crates/supervisor/src/daemon.rs) ·
[sweep and exit recovery](../../../crates/supervisor/src/process_host.rs) ·
[tail scan](../../../crates/store/src/tail.rs) ·
[fold](../../../crates/schema/src/fold.rs) ·
[lifecycle](../../../crates/engine/src/lifecycle.rs) ·
[provider recovery](../../../crates/engine/src/provider.rs) ·
[worker](../../../crates/worker/src/main.rs) ·
[tail contract](../../../spec/tail-lifecycle.md)
