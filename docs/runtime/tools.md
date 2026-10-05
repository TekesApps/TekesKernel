# Tools: invocation, execution and results

[Runtime · documentation home](../README.md) · [Execution flow](../flows/turn.md) · [Next: permissions and approvals](tool-permissions.md)

The catalog plus hook/helper/sandbox execution substrate landed in Slice 4.
Production dispatch of all 25 fixed tools through worker/supervisor backends is
governed by [tool-runtime](../../spec/tool-runtime.md) and was introduced in
Slice 8. Earlier slices used injected ToolBackend seams. The current assembly
must still not advertise an unconnected fixed entry; see the
[source-backed tool sequence](../architecture/flows/tool-execution.md).

## The ABI

```
tool_call event (intent, fsynced if side-effectful)
  → execute
  → tool_result event (outcome + output, large output spilled to assets/)
```

That pairing is the entire tool interface. The seven manifest backend classes
in builtin-tools reduce to the three execution placements below; the events
are identical for all placements.

## Backends

1. **Exec (default):** `spawn(argv, stdin bytes) → (exit code, stdout,
   stderr)`, with an **allowlisted clean environment** — tools never inherit
   the worker's ambient env, and credentials are not in it to begin with
   (D-46). Shell, git, ripgrep, ad-hoc scripts, hooks. Output over the byte
   cap spills to a content-addressed asset; the event carries the ref plus a
   head/tail excerpt. Before any output commits to events or assets it passes
   a heuristic secret scan — defense-in-depth, not the primary boundary. The
   scan is a **total transform** (R2-3): its outcome is always a terminal
   `tool_result` — clean (verbatim), redacted (with markers), or
   `withheld {scan_failed | quarantined}` — so a rejected output or a crashed
   scanner can never leave an unpaired intent that replay would misread as
   `aborted_by_crash` and tempt the model to repeat a side effect that
   already succeeded.
2. **In-process:** local functions with no external effect. On
   process-capable platforms the filesystem tools (read, apply-patch,
   glob) default to **exec via one multi-call helper binary** instead
   (D-57): SIGKILL is only reliable across a process boundary — a hung
   in-process read (FIFO, dataless file) wedges the worker while holding its
   lock — spawn cost (~ms) is noise against model latency, the helper gets
   its own sandbox profile, and patch writes do temp+rename inside it, so
   even a kill is crash-atomic. In-process remains for the mobile seam
   (no processes there) and for individually *measured* hot paths; the ABI
   is backend-agnostic either way.
3. **Supervisor control:** for resources the supervisor owns — every MCP
   peer (local tools are MCP stdio servers), jobs — the worker sends `tool_control` over its
   control channel (worker-control) and the supervisor executes against
   the pooled peer ([Supervisor](supervisor.md)). The original design's
   "daemon socket for resources that cannot be multiply opened" is realized
   as this one supervisor-owned pool; there is no separate unix-socket lane.

**MCP:** every MCP peer — stdio and HTTP alike — is owned by the supervisor
(mcp-runtime: "no MCP child is a worker child"): the supervisor launches
the configured argv or connects the HTTP transport, pools the peer generation
and exposes its frozen catalog to workers as `supervisor_control` dynamic
tools; the worker never links the MCP client. OAuth and header secrets stay
with the supervisor/keychain (Kernel-minted OAuth grants are exchanged inside
the supervisor's HTTP authorization provider) and are never written to events.

## Side-effect discipline

- Append-before-execute: the fsynced `tool_call` is the write-ahead intent.
  On replay, an unpaired intent means "may have run" — reconciled as
  `aborted_by_crash`, surfaced to the model to decide on retry. Never blind
  re-execution.
- Eager dispatch: a call whose arguments the provider has completed
  (`ToolCallReady`) takes the same write-ahead path *before* the response
  terminal exists, and executes through the same policy/hook/approval/sandbox
  pipeline while the response keeps streaming. The terminal must
  then name the call unchanged (identity, name, materialized arguments); it
  never writes it again, the post-terminal batch never re-executes it, and
  one continuation follows the whole batch. An interruption between the
  intent and its result is the ordinary unpaired-intent recovery above; an
  interruption before the terminal closes the attempt as unresolved with the
  eager call and its single result still durable (adopt validates them
  against the queried response). A park or stop during an eager call
  suspends further eager execution for that response without cancelling it,
  so the resumed run sees exactly the state a post-terminal park leaves.
- Native deferred tools: when the resolved route declares a native mode
  (provider-adapter §capability), the turn's provider catalog includes every
  deferred dynamic schema, the request is prepared with the native transform
  (OpenAI client `tool_search`/`tool_search_output`, Anthropic `defer_loading`
  and `tool_reference`), context renders as client-managed full replay, and
  the durable `tool_search` offer of this turn becomes the bound reference.
  The host-side `tool_search` still executes and records the offer; only the
  wire representation changes. Routes without a declared mode keep host-side
  schema activation.
- Remote continuations: a dynamic supervisor tool whose result is `pending`
  (an MCP task) writes a durable `state{subkind: tool_continuation}` step
  record instead of a result, then the worker polls it with receipt-bound
  continuation steps (bounded backoff), records every pending response as a
  step, publishes `input_required` as an ordinary hold whose answer is sent
  as exactly one input update, cancels remotely on stop, and terminalizes the
  call once. Polls shorter than one second stay resident; once the (remote
  `pollIntervalMs`-aware, doubling, 30 s-capped) interval reaches one second
  the worker writes a `park` step with `poll_after`/`interval_ms`, exits
  cleanly with the turn open, and the supervisor's sweep resumes the line only
  after that instant (`ensure_action_at`), in ordinary mode regardless of the
  resume policy. A restarted worker resumes at the durable step through the
  same unpaired-intent recovery; the initial remote side effect never repeats.
- Tools that are idempotent by construction (read, search) skip the fsync.
- **Anti-hang rules, backend-independent (D-57):** descriptor-first opens —
  `openat` with `O_NONBLOCK` + no-follow/beneath flags, then `fstat` on the
  open descriptor (stat-then-open is a check/use race — R3-22); refuse
  FIFOs/devices; dataless/offline files take an explicit
  materialize-or-refuse policy — never a silent blocking read; byte caps;
  timeouts with escalation — and an in-process timeout whose work cannot be
  killed **always converts to worker exit** (R3-22): the worker never
  continues past an unkillable task, because that recreates the
  lock-holding orphan this rule exists to remove.

## Background jobs

Detached work that outlives the worker (long builds, servers, watches):

- Job = `setsid`-detached process + a small state directory on disk
  (cmd, pid, status, output tail). Poll, never wait — any later worker
  discovers state from disk; nobody blocks on a job.
- This is the current JobTool disk-only design, kept as-is; it was already
  built to this architecture's rules.
- Jobs are deliberately outside the stop cascade (D-37): stopping a thread
  never kills its detached jobs; they are managed through job control.

## Tool catalog

The fixed namespace, complete 25-name inventory, per-tool argument surface,
availability, backend, effect class, and evolution rules are owned by
[builtin-tools](../../spec/builtin-tools.md) and its canonical
[`fixtures/tools`](../../fixtures/tools) manifest. This closes the former
implicit "builtin manifest" reference: an implementation may not derive a
different list from whichever tool classes happen to compile.

The tool list rendered to one model request is a projection of:

1. the validated fixed manifest;
2. the immutable launch policy (`allowed_tools`, workspace capability policy,
   permission/network profile, platform availability);
3. the current Kernel role (ordinary, plan, validator, subagent, or
   compactor) and conditional capabilities (credential, deferred catalog);
4. resolved local-manifest/plugin tools and MCP catalogs.

Registered, resident, role-only, conditional, and discoverable are distinct
states. In particular, a conditional tool is absent rather than advertised
when it cannot run; `apply_patch` is the sole fixed file-edit tool; and a
benchmark terminal bridge is a launch profile, not a different fixed manifest.

Git and GitHub operations use `shell` with the system `git`/`gh` binaries.
They do not add fixed `git_*` or `github_pr_create` schemas.

Fixed names cannot be shadowed. Dynamic-name collisions fail catalog
construction. Resident fixed tools preserve manifest order; dynamic tools are
stable-sorted by source/name. Deferred tools expose bounded search documents,
not their full schemas, and load an exact schema only after a durable
`tool_search` offer over the same catalog. Catalog caches and schema search
indexes are rebuildable; no registry process or dynamic registration state is
authoritative outside config, live MCP catalogs, events, and cache files.
