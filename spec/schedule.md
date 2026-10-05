# Schedule v1

Status: executable internal contract for Slice 14C.

This contract owns host-local scheduled prompt authority, timer ownership,
durable launch claims, restart recovery and management semantics. It exposes no
Client route or capability. Slice 14F may bind this seam to a separately
versioned Client-owned extension; the Session Endpoint base set is
unchanged.

## Files, writer and durability

The supervisor is the sole owner of `<storage>/schedules.jsonl` and
`.schedules.lock`. A log published under the earlier `schedules-v1.jsonl`
name is renamed in place when the authority opens, and the stateless legacy
lock file is removed. Every mutation takes the named lock exclusively, replays
the longest complete canonical prefix, discards at most one incomplete final
line, appends exactly one RFC-8785 line with `O_APPEND`, and performs
`F_FULLFSYNC` before returning a receipt or launch work. A complete malformed,
noncanonical, unknown-field, wrong-format, non-contiguous-seq or impossible
transition is corruption and fails closed. No schedule fact is stored in a
thread ledger, endpoint journal, SQLite database or timer heap.

Each line has `format:1`, a contiguous `seq` beginning at 1, a canonical UTC
seconds timestamp `at`, and one closed `op`: `save`, `delete`, `claim`, `bind`,
`status`, or `missed`. The Rust types in `crates/schedule` are the normative
field schema. The global JSONL is append-only truth; any in-memory deadline
heap is a rebuildable cache.

## Definition and cron

A definition is the closed object:

```text
{id:uuid, name:str, workspace_id:str, cron:str, time_zone:str,
 prompt:str, permission_mode:str, model_id?:str, enabled:bool,
 missed_policy:"skip_and_record"}
```

`id` is a lower-case canonical UUID. `workspace_id` uses config's
1...128-byte `[A-Za-z0-9._-]` grammar and does not begin with `.`. Name, prompt
and permission mode are nonempty after trimming and at most 16 KiB; a model id
is absent or 1...512 bytes. Workspace existence, permission policy and model
profile readiness are injected validations performed before `save`; this
contract does not silently invent those authorities.

Cron is exactly five ASCII fields separated by one ASCII space: minute, hour,
day-of-month, month, weekday. A field is a comma list of `*`, integer,
inclusive range, or either base followed by `/positive-step`. Ranges are
minute 0...59, hour 0...23, day 1...31, month 1...12, weekday 0...7 with both
0 and 7 meaning Sunday. After expansion, a field whose value set covers its
entire legal domain is unrestricted (`*` and `*/1` are therefore identical).
When both day-of-month and weekday are restricted, either matching is
sufficient; otherwise the restricted field must match.
The first matching wall-clock minute strictly after the input instant is the
next occurrence. `time_zone` is an IANA identifier from the implementation's
pinned tzdb. Evaluation proceeds over real UTC minutes: nonexistent DST wall
minutes never occur and repeated fall-back wall minutes are two distinct
occurrences. No occurrence within ten years is `no-occurrence`.

All durable timestamps are UTC RFC-3339 with whole seconds and `Z` (for example
`2026-08-29T04:00:00Z`). Scheduler comparisons use instants, never locale or
wall-clock strings.

## Keyed management

The internal seam is:

```text
list(workspace_id?) -> [ScheduleView]
save(origin_tuple, definition, now) -> ScheduleView
delete(origin_tuple, task_id, now) -> deleted:bool
run_now(origin_tuple, task_id, now) -> LaunchClaim
recover(now) -> [LaunchClaim]
poll_due(now) -> [LaunchClaim]
bind_launch(task_id, claim_id, session_id, input_seq, now) -> ScheduleView
record_status(task_id, claim_id, status, error?, now) -> ScheduleView
```

Every external mutation uses a nonempty `(client_id,key)` origin tuple. Its
request fingerprint is durable. Repeating the same tuple and same fingerprint
returns the original result even after later state changes; reusing it for
different work is `origin-collision`. A receipt is returned only after the
operation line is durable. List order is task-id UTF-8 ascending. Delete of a
missing id durably returns false and is still deduplicated. Save and delete
reject a task whose last status is active. Run-now rejects a missing or active
task; disabling a task suppresses cron claims but does not suppress run-now.

The closed statuses are `idle`, `claimed`, `running`, `parked`, `completed`,
`failed`, and `interrupted`; active means claimed/running/parked. A status
observation may move running to parked and parked to running, or either to a
terminal status. The observation must name the active claim. Status `claimed`
is authored only by a claim and `running` begins only after bind. Terminal
status clears the active claim. A later observation for that claim is a typed
claim mismatch, never a second completion.

## Claim-before-launch and recovery

For a due task, the supervisor appends and syncs `claim` before invoking the
ordinary keyed create/input authority. `claim_id` is the deterministic SHA-256
identity of task id, scheduled instant, reason and (for manual runs) origin.
The launch uses this claim id as its keyed submission identity. It returns a
canonical session UUID and durable input event seq, after which `bind` is
synced. The eventual `turn_open` is found from consumption of that input; the
scheduler never invents a turn id. Thus:

- crash before claim: no launch was authorized;
- crash after claim and before or during launch: `recover` returns the same
  unbound claim, and keyed launch re-drive creates at most one turn;
- crash after the keyed launch commits but before bind: the same re-drive
  returns the existing session/input receipt and bind completes once;
- crash after bind: recovery does not relaunch; endpoint/ledger projection
  supplies later parked/running/terminal observations.

Bind is idempotent only for the exact same `(claim,session,input_seq)` tuple. A
different binding fails closed. The timer never waits for the lifetime of a
turn; parked approval work therefore consumes no scheduler execution slot,
although the task remains active and cannot overlap another occurrence.

## Poll, missed occurrences and timer ownership

Exactly one supervisor timer loop owns `poll_due`; tests may inject time and
call it directly. The loop exists only while an enabled definition or active
run exists. Saving the first enabled task starts it after the save barrier;
disabling/deleting the last task stops it after the mutation barrier. A wake
replays truth under the lock and never trusts a stale heap deadline.

After startup, a launch or projection error is retained as a typed readiness
diagnostic but does not terminate that sole loop: while durable work remains,
the next bounded tick retries from truth. Complete schedule-log corruption
still fails supervisor startup closed.

`poll_due(now)` first returns every existing unbound claim, then processes each
enabled due task in task-id order. If it is inactive, exactly one occurrence is
claimed and `next_run_at` advances to the first occurrence after `now`; this
bounds a delayed tick to one launch. If active, no launch occurs: a `missed`
line records the earliest due instant and advances to the first occurrence
after `now`.

At supervisor startup, `recover(now)` returns unbound claims for re-drive and
applies the v1 `skip_and_record` downtime policy to inactive definitions: the
earliest overdue `next_run_at` is recorded as `missed_at` and the next deadline
advances strictly beyond `now`; no backlog turn is launched. Active bound runs
are reconciled from their named session/input consumption by the ordinary projection and
then `record_status`. This is the complete restart/catch-up rule.

## Errors and boundaries

Closed errors include invalid id/workspace/text/origin, invalid cron, unknown
time zone, no occurrence, task not found, task active, origin collision, claim
mismatch, ledger corruption, and I/O/durability failure. A failed validation
or append changes neither the log nor any thread.

Slice 14C does not register `schedule/list`, `schedule/save`,
`schedule/delete`, `schedule/runNow`, or any similarly named public management
route/capability. The frozen Client event vocabulary may already recognize
`schedule/change`, but Slice 14C does not produce it; Slice 14F owns the DTO,
capability negotiation and Client projection. Gate 101 proves cron/oracle
bytes, Gate 102 keyed management and
durability, Gate 103 crash/restart/missed behavior, and Gate 104 fail-closed
non-mutation plus the no-route boundary.
