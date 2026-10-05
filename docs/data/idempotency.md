# Idempotency, admission and retries

[Data and reliability · documentation home](../README.md) · [Durability rules](durability.md) · [Next: crates responsible for these rules](../architecture/crates.md)

Every operation in the system falls under one of four rules. What makes four
enough: one writer per file plus total order by `seq` means every dedup check
is "look at the tail while holding the lock" — never a distributed protocol.

## Rule 1 — Ensure, never start (lock-arbitrated liveness)

"Start a worker" does not exist; the only verb is **ensure-running(file)**.
Any number of triggers — user input, delivery, sweep, doorbell races — may
call it concurrently: each spawns a candidate; the candidate's first acts
are, in order, the protocol handshake (pre-lock — exit **76** on mismatch,
quarantined by the supervisor for that process-table lifetime, R7-6) and then taking the flock; every lock
loser exits 0 silently — exit 0 means lock loss and nothing else
(normative: nonblocking `LOCK_EX | LOCK_NB` — tail-lifecycle §Acquisition,
R13-8). The lock is the arbiter; the
supervisor's process table is a cache of the outcome, never the authority.
Duplicate ensures are free by construction — including mixed ones (R6-1):
all candidates are equivalent *at the lock* because the winner derives the
authoritative state and mode from
[spec/tail-lifecycle.md](../../spec/tail-lifecycle.md) under the lock (D-61/
R11-5): a `parked_hold` winner exits 0; an `answered_hold` winner resumes
ordinary; a winner finding **runnable** input upgrades to ordinary — with
the standing exceptions that an active stop dominates everything and held
pre-closure inputs never auto-run (R8-1).

Child spawn is the same shape: create the child file with `O_EXCL`; an
existing file with the expected `genesis` means "already created" and the
retry resumes at the next step of create-before-register ([Turn flow](../flows/turn.md)).

The supervisor itself obeys Rule 1: one exclusive lock on the storage root
(D-41) makes "ensure supervisor" idempotent and excludes a second instance.

## Rule 2 — Externally-originated mutations are keyed

Every mutation entering from outside the process tree — user submits, control
messages (steer / stop / approval), cross-thread deliveries — carries a
caller-supplied idempotency key: `origin_key`, the successor of today's
requestToken (a string, per D-18), **deduplicated as the namespaced tuple
(principal, client, target, operation, key)** so two clients' keys can never
collide (D-33).

Append is conditional: the lock holder checks the tuple against the dedup
projection (in-memory, rebuilt from genesis on every open, with full-tuple
identity) and drops duplicates,
acking with the original `seq`. **Thread creation is keyed the same way**
(R3-1): `genesis` carries the creator's origin key; the index check and the
folder creation are one atomic step of the supervisor's serialized creation
path, and the rebuildable creation-receipt index returns the existing thread
id on retry — neither a crash between genesis and reply nor two concurrent
same-key creates can mint a second thread. Origin tuples never expire: every
retried tuple re-acks its original seq for the life of the ledger.
At-least-once delivery plus keyed conditional append yields exactly-once
effect.

**Acknowledgement is durable-first (F2):** for an alive target the supervisor
forwards to worker stdin and acks the client only after the worker reports
the appended, synced `seq`; a broken pipe or missing report is retried under
the same key. For a dead target the supervisor appends under the lock, syncs,
then acks. No path acknowledges before durability.

On stdin, the worker echoes each control message as an event; a redelivered
key that is already echoed is dropped by the same check.

## Rule 3 — Intent before side effect (attempt discipline)

The generalization of append-before-execute (D-19) to every non-idempotent
externality:

- **Tool calls**: `tool_call` fsync → execute → `tool_result`. Local
  side-effects retain the `aborted_by_crash` rule ([Tool execution](../runtime/tools.md)). Effectful
  dynamic supervisor calls additionally use `control-intent` fsync → execute
  with stable `request_id` business key → receipt. An interrupted intent is
  queried, not re-executed: `confirmed` adopts, `not_found` permits one same-key
  attempt, and `unknown/conflicted` stop for reconciliation/manual review.
- **Provider sends**: fsync `attempt {attempt_id, epoch, wire_digest, admits}`
  before the HTTP request; every `output`/`error` names the attempt it
  settles, exactly one terminal settlement per attempt (D-43). On replay, an
  unpaired attempt **without** its `attempt_dispatched` marker settles
  locally as `not_dispatched` — nothing left the host; **with** the marker
  the provider may have received and billed it: **in an ordinary run** a
  stateless adapter simply re-sends (duplicate billing is accepted —
  state cannot corrupt, because the baseline advances only on a durable
  `output`); a server-managed adapter takes its declared three-way recovery
  (query by identity / recovery epoch / resend) — never a blind re-append
  onto a chain that may already contain the response.
- The baseline advances **only** on durable output ([Model context](context.md)): a lost response is a
  retried request, never a torn conversation.

## Rule 4 — Watermarks may coalesce; the truth path never debounces

Doorbells (`appended {seq}`), manifest refreshes, usage rollups, and mirror
pushes are **high-water marks, not queues**: delivering only the latest value
is correct, so bursts collapse for free. Debouncing lives exclusively on
these derived surfaces. Truth-path appends are never debounced, batched away,
or dropped — they are deduplicated (Rules 1–3), which is a different thing.

**Delivery `receipt`s are not watermarks** (R2-4): each is correlated to one
delivery's origin tuple and must never coalesce — a doorbell tells you the
file grew; only a receipt tells you *which* mutation became durable.

## Operation table

| Operation | Rule | Mechanism |
|---|---|---|
| ensure worker | 1 | flock race; losers exit 0 |
| spawn child | 1 + 3 | `O_EXCL` create; create-before-register ([Turn flow](../flows/turn.md)) |
| user submit / queued input | 2 | `origin_key` conditional append |
| steer / stop / approval | 2 | keyed control message + event echo |
| cross-thread send | 2 | supervisor delivery carrying the sender's key |
| provider request | 3 | `attempt` event before HTTP; baseline only on durable output |
| tool execution | 3 | append-before-execute (D-19) |
| recovery after reap (D-61) | 1 | mode derived under lock; already-settled check; a spawned run authors the facts |
| doorbell / mirror / manifest / usage rollup | 4 | high-water coalescing |
