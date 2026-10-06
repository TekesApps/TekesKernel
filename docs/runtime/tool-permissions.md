# Tool permissions: isolation, approvals and hooks

[Runtime · documentation home](../README.md) · [Tool execution](tools.md) · [Next: tools implementation](../../crates/tools/docs/README.md)

## Sandboxing

**Spawn is not a sandbox** (D-56): it gives crash isolation and lifecycle,
zero authority containment — a child inherits the user's full filesystem and
network authority. Containment is four layers from one policy source:

1. **Authorization** — static allowlist + approval gate (below): who may run
   what.
2. **Environment** — allowlisted clean env; credentials unreachable (D-46).
3. **Advisory pre-check** — path/policy validation before spawn: cheap, good
   errors, bypassable by the program at runtime — never the real boundary.
4. **Mandatory OS profile** — derived from the *same* policy (writable
   roots, network class) and applied at spawn: Seatbelt on macOS,
   landlock/seccomp on Linux, via the `Platform` seam. One policy source,
   two enforcement points; configuring them separately is a defect.

Mechanics: macOS = Seatbelt (SBPL profile via libsandbox/sandbox-exec);
Linux = Landlock for the filesystem plus seccomp for sockets, process
creation and kernel interfaces, applied in the child before `execve`; no
namespace or bubblewrap layer. Both are
**inherited by the whole child tree**, which is why they work for `sh -c`
and its grandchildren. Both are **probed dependencies that fail closed**
(R3-21): the Seatbelt entry points are formally deprecated and landlock
depends on kernel version — boot probes the backend, and where no working
profile mechanism exists, side-effect-capable exec refuses to run without a
durable per-run unsandboxed approval. Silent unconfined execution is never a
fallback. Profiles are deny-by-default, derived from the one
policy source: read allowlist = workspace cwds + system read-only paths;
write allowlist = writable roots + scratch; network by tool class. Unix
permission bits are **not** a containment mechanism — tools run as the same
uid, so mode bits cannot distinguish them from the user; rlimits/umask/
O_CLOEXEC/clean env are complementary *resource* caps, not reachability
bounds. Running a command unsandboxed is an approval-gate escalation
recorded as an event, never a standing configuration.

Rules: exec spawns **argv-direct, no shell interpretation** — only the shell
tool itself runs `sh -c`. In-process tools cannot be individually profiled
(same process); their containment is code-enforced path policy, and the
worker process itself may run under a coarse profile (thread folder +
workspace roots + provider endpoints). Daemon-pool backends get their
profiles at daemon start. Profile strictness follows the trust gradient
([Evolution mechanism](../history/evolution.md)): builtin loosest, user scripts middle, third-party plugins and MCP
servers tightest by default.

## Approval gate

The agent-side syscall filter, in two layers:

- **Static policy** (pre-check): allow/deny lists passed at spawn as config —
  a pure function consulted before execution; no round trip.
- **Interactive approval:** append `approval_request` → block on stdin →
  `approval_response` arrives as a control message and is echoed as a
  **barrier event before its receipt** (replay must see every acknowledged
  grant). The unified idle-timeout applies ([Worker lifecycle](worker.md)): long
  waits park — the worker exits with no settle, the trailing request is the
  durable hold (R9-2); the answer respawns it.
- Grants are per-request and recorded; standing grants are config changes,
  not events.

**Ask-user shares the hold.** A model question to the user is an ordinary
`tool_call` (provider-born, admitted at birth) plus an `approval_request`-
family event carrying the question (runtime-only, correlation id = call id);
the worker then blocks on stdin like every other wait (D-12). Fast path: the
keyed answer arrives, is echoed as an event, and lands as the `tool_result`
paired to the call — flagged `anchor`, so the accepted answer survives
compaction (today's ask-user anchor group). Slow path: idle-timeout parks
the hold (exit, no settle — R9-2); the eventual answer is appended under
lock, acked, and the respawned worker pairs the trailing unanswered call
with the delivered answer on replay — the same turn, which never settled. Duplicate answers die on the origin-key tuple and the
one-terminal-result-per-call rule.

## Hooks

Hooks are processes registered in config (the user-config authority,
versioned into the spawn snapshot — D-36), interposed at two fixed points of
the loop, speaking the exec ABI: allowlisted clean environment (D-46), JSON
on stdin, verdict JSON on stdout, tool-grade byte caps and timeouts.

- **Pre-tool**: runs after the `tool_call` intent is durable and before
  execution — never earlier: the model's call is provider-born and enters
  history verbatim; a hook gates or transforms *execution*, never the event.
  Verdicts: `allow` / `deny` / `mutate` / `ask`. Deny reasons and the
  effective (mutated) invocation are recorded in the `tool_result` payload
  for audit; `ask` reuses the ask-user hold above — one wait mechanism.
  **A `mutate` on any call appends + fsyncs a runtime-only
  `effective_execution {call id, full effective invocation — asset-spilled
  when large}` before executing** (R3-3): replay and supervisor-control
  binding must name what actually ran even for read-only calls; a digest alone
  leaves recovery unable to identify the effective invocation (R4 pass-1).
  After that effective execution and any approval are durable, the backend
  runs; there is no Kernel-side worktree snapshot before a mutation (the
  user's own VCS owns file history, as in DSH).
- **Post-tool**: runs after execution, before the `tool_result` is appended;
  its annotations join the result payload. Hook failure follows per-hook
  fail-open/fail-closed config and is recorded in the payload.
- **Replay**: hooks are exempt with the side effect they wrap — replay
  re-executes neither; a reconciled `aborted_by_crash` result carries no
  hook annotation.
- **External supervisor effects**: MCP/plugin tools that change external
  business state must declare `idempotency-reconcile`. Kernel persists the
  request before dispatch, supplies the stable request id as an out-of-band
  idempotency key, and queries the external authority after crash/timeout.
  `confirmed/not_found/unknown/conflicted` are distinct states; an unqueryable
  effect fails closed before execution.
- Config changes take effect at the next spawn via the recorded snapshot;
  hooks are a runtime-only surface and never open an epoch.
