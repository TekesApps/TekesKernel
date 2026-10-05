# Lifecycle hook protocol v2

Status: **normative executable contract** for host lifecycle extensions. This is
Kernel's command-hook contract, not an MCP protocol extension. Memory, telemetry
and other extensions may use the same points. No memory service is bundled here.

## Configuration ownership

A binding lives in the existing user/workspace/project `hooks/**` instruction
sources (see [instruction snapshots](instruction-snapshot.md)). The snapshot
validates every source, including disabled and shadowed sources; logical-path
precedence and UTF-8 ordering are unchanged. Execution reads frozen bytes, never
rescans configuration. Installation alone does not activate plugin components.
The existing plugin receipt/grant rules still apply; this change does not add a
second plugin registry or automatically execute a plugin's hook component.

One binding is one canonical RFC-8785 JSON object plus LF:

```json
{"argv":["/absolute/path/to/extension","context"],"enabled":true,"env":{},"event":"context.prepare","format":2,"id":"memory-context.json","stderr_bytes":4096,"stdout_bytes":65536,"timeout_ms":500}
```

`id` must equal the path below `hooks/`. `enabled` defaults to true; a disabled
higher-precedence binding can disable a lower-precedence one. `argv` is an
argv-direct command, not a shell string. `env` is the complete explicit environment.
An extension may receive its own service reference/budget through argv or env;
Kernel does not interpret memory-specific settings. No ambient credentials are
inherited. Executable content itself is not frozen by a configuration snapshot:
use an immutable installed executable when version pinning is required.

Format 1 remains exclusively the existing [tool pre/post hook](tool-hook.md).
Format 2 accepts exactly the fields above. Unknown events, fields, versions and
invalid limits reject snapshot capture. Limits are positive; timeout is at most
5,000ms and each output stream at most 16MiB. Lifecycle runtime failure is always
non-fatal to the task: these are extension points, not authorization gates.

## Points and authority

| Event | Boundary | Response authority |
|---|---|---|
| `turn.before` | Before ordinary provider-turn execution, once successfully acknowledged per turn/binding | Acknowledge only; not an input-admission or permission gate |
| `context.prepare` | After history projection, before preparing each ordinary model request | Append bounded text reference context to this request |
| `tool.completed` | Observe a final, post-policy `tool_result`, after the source prefix is synced | Acknowledge only |
| `context.before_compact` | Before constructing/publishing an actual manual or automatic compact record, after determining covers | Acknowledge only |
| `turn.settled` | Observe durable `settle`, including completed, error and interrupted outcomes | Acknowledge only |

`turn.before` is not emitted for a recovery-only run that never executes a model.
`context.prepare` is not a generic model transport interceptor: internal summary
requests are excluded. Tools may continue across attempts; pending/approval states
are not completed results. Observations are drained before the next ordinary
request and at worker exit; they are not inline mutation hooks inside a tool.
The compactor may already have generated a summary when `context.before_compact`
runs; the original ledger is still available and no compact record has been written.
A no-op automatic compaction emits no hook. Manual compaction retains its existing
origin receipt and retry rules.

`turn.settled` is a lifecycle terminal, not proof that the user's objective or an
external effect succeeded. Inspect `outcome`, `reason`/`classification`, and source
evidence. A stopped or crashing worker cannot promise a before-hook will run.

## Wire request and response

Same command carrier as tool hooks: one canonical JSON line on stdin, EOF;
exactly one canonical JSON line on stdout. Stderr is bounded diagnostic output.

Request fields:

```text
{format:2, hook_id:string, event_id:string, event:EventName,
 workspace_id:string, thread_id:string, turn_id:uint,
 data:{source:{ledger_path:string}, payload:object}}
```

`workspace_id` and `thread_id` come from host-owned configuration/genesis.
`event_id` binds workspace, thread, ledger filename, frozen binding and trigger
identity. Consumers MUST deduplicate on this id when performing effects.
`ledger_path` locates authoritative local evidence; asset references resolve in
that thread's `assets/` store. Do not treat the path as permission to modify the
ledger. These command extensions run as the current user, like existing tool
hooks; this is not a new sandbox boundary.

Payloads:

- `turn.before`: `source_seq`, `turn` (the turn-open record).
- `context.prepare`: `through_seq`, `items` (the projected provider items).
- `tool.completed` / `turn.settled`: `source_seq`, `record` (the final source event).
- `context.before_compact`: `through_seq`, `covers` (sequence numbers), `manual`.

Source data and returned context pass the existing total secret transform.
Input exceeding 2MiB is withheld and the task continues. Extensions should use
source references for large histories rather than duplicate the whole ledger.

Response:

```text
{format:2, hook_id:string, event_id:string, context?:[string,...]}
```

All three correlation fields must match. Only `context.prepare` may return
nonempty `context`; other points cannot deny, rewrite, steer or reopen a turn.
The response permits at most 32 strings and 32KiB of UTF-8 text. All hooks together
contribute at most 32KiB per prepared request. Text is appended as explicitly
labelled host-extension reference context, never as system or user instructions.
The actual prepared request is stored through the existing content-addressed
request-body publication path, so the material sent to the provider is inspectable.

## Failure, cancellation and delivery

The carrier concurrently writes/drains all three pipes. The deadline covers stdin
backpressure, child execution and descendants retaining output pipes. Timeout or
cancellation terminates the process group, escalates to KILL, and reaps the child.
User stop/supervisor loss cancels preparation hooks; terminal observations remain
eligible so interruption can be recorded. Malformed output, cap violations and
process failures contribute no context and do not alter task outcome. Invalid
configuration is rejected during capture; failure opening hook receipt storage
at startup disables hooks for that worker and emits a diagnostic.

Under the thread folder, `lifecycle-hooks/<binding-set-digest>/<line-digest>/`
contains an atomic first-registration baseline and successful response receipts.
The baseline avoids replaying an entire old conversation into a newly installed
extension. The source ledger is the outbox for terminal observations after that
baseline. It is synced before external observation; receipt publication is atomic.
Repeated successful deliveries use the saved receipt, including saved context.

An unacknowledged observation retries on the next worker startup with the same
binding set. Within one worker, each source observation is attempted once. A
crash between the extension's external effect and receipt publication can cause
redelivery; this is **at-least-once on subsequent worker activation**, not global
exactly-once execution. There is no new daemon retry scheduler: a dormant thread
has no wall-clock retry guarantee. Changing the binding set starts a new baseline;
it does not run an old executable under new configuration. Disabled hooks do not
run. Deterministic ledger folding/replay itself never executes hooks.

## Evidence

- `crates/tools/tests/lifecycle_hooks.rs`: wire authority, correlation, environment,
  cancellation, blocked stdin and inherited-pipe deadlines.
- Worker `lifecycle_*` tests: frozen configuration, receipt replay, failed-delivery
  identity, terminal outcomes and compaction ordering.
- Worker production continuation test: hook text present in actual loopback HTTP
  provider requests, one final tool observation and one settled observation.

These are local deterministic tests, not live-model or external Memory service acceptance.
