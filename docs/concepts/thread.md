# Thread fundamentals

[Concepts · documentation home](../README.md) · [System overview](../README.md) · [Next: execution and settlement](../flows/turn.md)

## Thread fundamentals (A)

This section defines the shared domain semantics of a thread (abbreviated A in the alignment record). It is not a new JSON schema
and introduces no `sessions/` directory. [Storage layout](../data/storage.md) explains file structure separately; exact fields and state transitions are defined by
[event](../../spec/event.md), [provider runtime](../../spec/provider-runtime.md)
and [tail lifecycle](../../spec/tail-lifecycle.md). When explanations, contracts and code conflict,
record and resolve the discrepancy using the [alignment rules](../verification/thread-alignment.md), rather than redefining confirmed semantics to match the implementation.

```text
thread
  identity
    thread_id
  data/files
    main.jsonl                  main execution log
    <child-id>.jsonl            child execution logs, including validators
    assets/                    external content and file snapshots referenced by events
    auxiliary files            protocol records, locks, rebuildable indexes, etc.
  execution contexts           contexts called sessions in this discussion
    role: executor | validator the two responsibilities considered here, not every runtime role
    runtime: worker            can exit, restart and recover from the log
  turns
    input                      input processed in this turn
    flow                       execute → candidate → validate/repair as needed → settle
    sink                       progress output from tools and the assistant
    candidate                  candidate answer reference + artifact snapshot
    settlement                 the unique settlement result for this turn
```

### Terminology and boundaries

| Concept | Definition and relationship |
|---|---|
| thread | A persistent task container with a stable identity, one main execution log and zero or more child logs; it can process multiple turns |
| line / execution log | Work history carried by one semantic JSONL log; the main log lives as long as the thread, and child logs carry delegated tasks |
| execution context / session | Context needed for model execution and recovery, projected from logs, seeds, epochs and configuration; not a separate storage entity |
| worker / run | Worker is the executable; a run is one execution holding the log's write lock and recording `run_start`. Multiple runs may continue the same execution log |
| turn | Processing of admitted input, from `turn_open` to that turn's unique `settle`; queued input does not create a new turn before admission |
| role | The executor executes and repairs; the validator independently checks the candidate and artifacts. The same worker program supports different responsibilities |

In public APIs, `session` often means the root thread; in validation, a `validator session` means a child execution context.
Interpret these uses according to the interface. Do not infer one process or file per session.
Turn identifiers belong to execution logs, so a reference must identify its line as well. Parent-child relationships are recorded by `genesis.parent` and
`spawn`, not inferred from OS PPIDs or nested directories. A main-log turn may refer to a validation child log;
the child has its own event sequence and turns, which must not be conflated with the parent's.

### Progress output, candidates and settlement

| Aspect | Content | Boundary |
|---|---|---|
| sink | Tool `call_id`, name, args, status, result/error; assistant progress messages with `final_answer=false` | A classification of output purpose, not a new log or protocol event type; status can be projected from calls/results |
| progress? | Optional tool progress capability | There is currently no complete general tool-progress path; accepting an MCP notification does not establish delivery of progress to the user |
| final output | `response.output.final_answer=true` identifies the model's final answer | A new root final leads directly to a durable settlement; an existing validation turn may instead treat it as a candidate |
| settlement | `outcome: completed \| interrupted \| error`; completed carries a direct `promoted_output_seq` or a legacy validation result and answer reference; interruption/errors record a reason | Each turn ultimately has exactly one settlement; completed does not guarantee tests passed |

These three aspects can reuse one transport channel. The current endpoint also sends candidate
`assistant/message` events with `sessionFinal` marking the final-answer attribute; consumers must wait for settlement's
`promotedMessageID` to identify the final delivered answer, rather than completing the turn on message arrival or a final flag alone.
See [output projection](../interfaces/client.md#ui-projection) and
the [endpoint promotion contract](../../spec/session-endpoint.md#journal-projection).

### Execution and completion rules

1. A new root final directly settles the turn and references the exact final output. It does not start an independent validator.
2. A turn already in the earlier validation flow finishes that flow on recovery. When fresh validation is needed, create an independent validator context. `pass` settles immediately without requiring another executor answer.
   `fail` feeds back into the original executor context for one repair; a second negative result settles as `inconclusive`.
   Abnormal validator termination without a valid result also leads to `inconclusive`; a user stop follows interruption handling.
3. Validation binds the specific thread, turn, worker, candidate output and artifact snapshot.
   Identical file contents do not allow a new candidate to reuse an old verdict; exact replay of the same candidate may reuse its durable decision.
4. Repair stays in the original turn and creates no new user turn; queued input for the next turn is admitted after this turn settles.
5. Durable `settle` means the turn's final state has been recorded (`terminal_recorded`); it does not prove process exit.
   The current log tail's `settled` lifecycle also requires that no active writer holds the lock.
   A worker may also exit while waiting for approval without a settle; that is parked, not completed.
6. The main log owns the user-delivery flow described here. An ordinary delegated child's completion first becomes a result for its parent;
   a validator completing its own `verify` does not end the root turn either. For the complete branches, see [Turn flow](../flows/turn.md#turn-execution-and-validation).

### When each lifetime ends

- A thread is the persistent container. Turn settlement and worker exit do not
  delete or permanently finish it; later input can open another turn. Archiving
  and deletion are separate management operations.
- A turn's final outcome becomes durable at its unique `settle`. A candidate,
  a child result, a validator verdict, or a process exit alone is insufficient.
- A worker run ends when its process exits and releases its line lock. The
  current ordinary path exits after settling one turn. An approval/question
  hold can instead park without settling; an answered hold resumes that same
  turn in another run. A crash can leave unfinished work requiring recovery.
- The line-tail state `settled` also requires no other live lock holder. It is
  distinct from the already durable turn outcome and from thread lifetime.
