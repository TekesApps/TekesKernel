# Sandbox profile contract v1

Status: **normative executable contract**. This file owns the policy input,
deterministic platform profiles, probes, and unsandboxed escalation binding.

## Policy input

The closed canonical policy object is:

```text
{
  "format": 1,
  "read_roots": [absolute canonical directory, ...],
  "write_roots": [absolute canonical directory, ...],
  "network": "deny" | "loopback" | "all",
  "allow_process": bool,
  "scratch"?: absolute canonical directory
}
```

Root arrays are unique and UTF-8 byte-sorted. Every write root must equal or
be beneath a read root. `scratch`, when present, is both readable and writable.
The policy digest is `sha256-` plus SHA-256 of its RFC-8785 canonical bytes.
Authorization pre-checks and the mandatory OS profile consume this exact
object; separately configured reachability is invalid.

The object is the **effective invocation policy**, not merely the workspace
base. Config-v1 supplies workspace roots. A fixed tool may additionally name
invocation-scoped roots only where builtin-tools exposes such an argument
(`shell.writable_paths`, `job.writable_paths`, or an approved out-of-workspace
`write`/`apply_patch` target). The worker canonicalizes the requested roots
without following an escaping symlink, displays those exact roots in an
approval request, and binds the grant to `{run, call, requested_roots,
base_policy_digest}`. Only after the durable grant may it derive a new
effective policy containing those roots. The advisory pre-check and OS profile
must consume the same derived bytes. A denial, changed root, changed base
policy, or replay under another call cannot reuse the grant. `/`, a home root,
or another broad root requires an exact explicit grant and is never inferred
from a child path.

For `job.start`, the supervisor-side broker repeats the subset check against
the immutable policy ceiling carried with the correlated call. The job spec
retains the normalized requested roots; the sandbox launcher receives the
exact per-call effective policy rather than a workspace-wide write profile.
The broker's automatic authority is limited to the new job's own private state
directory. Missing `working_directory` uses the immutable primary workspace
cwd supplied beside that policy and never the supervisor process cwd.

## Deterministic derivation

`darwin-seatbelt` emits UTF-8 SBPL with LF line endings and clauses in the
policy's root order. It denies by default and imports the OS-provided
`system.sb` runtime baseline, then limits additional reads to fixed system
runtime roots (`/System`, `/usr`, `/bin`, `/sbin`, and `/private/var/select`,
whose selector symlinks `/bin/sh` and the Xcode shims read on every launch)
plus `read_roots`, writes to `write_roots`/scratch, process
execution to `allow_process`, and network to the selected class. The imported
baseline is a probed platform dependency: a missing or behaviorally rejected
baseline fails the backend probe rather than widening authority. It selects no network,
loopback-only, or all network from `network`.

`linux-landlock-seccomp` emits the canonical JSON plan consumed by the Linux
launcher: the same roots and network class, Landlock ABI minimum, and a closed
seccomp syscall class. Applying that plan is mandatory; compilation without a
working kernel backend is not success.

Profiles escape path literals and reject control/NUL characters. Golden bytes
live in `fixtures/sandbox/`.

## Probe and fail-closed launch

At boot and after an executable/platform change, the Platform seam compiles
the golden probe policy and runs a confined probe which must prove both an
allowed read and a denied write outside the policy. Probe status is one of:

```text
{"available":{"backend":string,"version":string}}
{"unavailable":{"class":"missing"|"rejected"|"unsupported","detail":string}}
```

Side-effect-capable exec is refused when unavailable. It may run unsandboxed
only when replay finds a durable grant bound to the exact
`{run, call, policy_digest}`. A grant for another run, call, or policy is not
reusable; standing configuration cannot silently substitute for it. Refusal
is a terminal denied tool result, not a fake empty success.

## Platform coverage

macOS uses Seatbelt (`sandbox-exec` or equivalent libsandbox entry point) and
the profile applies before the requested executable starts. Linux uses
Landlock plus seccomp and fails closed when either required half is absent.
Mobile has no exec backend. Sandboxing is inherited by the full child tree;
rlimits, clean environment, and advisory path validation are complementary,
not substitutes.

`fixtures/sandbox/` contains canonical policies, Darwin profile bytes, Linux
plans, probe outcomes, invalid policies, and unsandboxed-binding cases.
