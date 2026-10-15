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
  "scratch"?: absolute canonical directory,
  "protected_roots"?: [absolute canonical directory, ...]
}
```

Root arrays are unique and UTF-8 byte-sorted. Every write root must equal or
be beneath a read root. `scratch`, when present, is both readable and writable.
`protected_roots` is unique and byte-sorted too, and is omitted when empty, so a
policy without protected roots keeps its earlier bytes and digest.
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

### Kernel-owned roots

The **Kernel-owned roots** are the canonical storage root and the service
user's platform credential store (`~/Library/Keychains` on macOS,
`~/.local/share/keyrings` elsewhere, located through the password database
rather than `HOME`; a store that does not exist is omitted). No session can
write them, whatever folder it runs in:

- A launch policy lists in `protected_roots` every Kernel-owned root that lies
  at or beneath one of its permitted write roots, for example the storage root
  under a writable home folder. Roots outside every write root are already
  unwritable and are not listed.
- No write reaches a protected root or anything beneath it. The only
  exception is a write root declared by its canonical name strictly beneath a
  protected root: the broker's own job state directory. Workers never derive
  such a root from tool input: a `write`/`edit`/`apply_patch` target or a
  `shell`/`job` writable path that names, or resolves through a symlink to, a
  Kernel-owned root or a path inside one is refused before any policy is
  derived.
- Folder entry refuses the same roots (see
  [session-endpoint](session-endpoint.md#create-fork-archive-and-model-publication)).

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
execution to `allow_process`, and network to the selected class. Each
protected root then gets a `file-write*` deny on its subpath, after the write
grants because Seatbelt applies the last matching rule. Seatbelt matches
paths, not objects, so every ancestor of a protected root that lies within a
write root also gets a `file-write*` deny on that literal path: renaming or
replacing the ancestor would otherwise move the protected tree out from under
its rule, while creating siblings inside the ancestor stays allowed. Write
roots strictly beneath a protected root are granted again last. The imported
baseline is a probed platform dependency: a missing or behaviorally rejected
baseline fails the backend probe rather than widening authority. It selects no network,
loopback-only, or all network from `network`.

Linux production launchers request `LinuxAutoV1`: probe bubblewrap first and
use Landlock only when that probe is unavailable. A successful probe records
the concrete backend; policy rejection or launch failure never retries a
weaker backend or unsandboxed execution. Callers can also require either
concrete backend explicitly. Both support x86_64 and aarch64.

`linux-bubblewrap-seccomp` uses a root-owned, non-user-writable installation
at `/usr/bin/bwrap` or `/bin/bwrap` (bubblewrap 0.4.0 or later). It requires
working unprivileged user namespaces, mount/PID/IPC/UTS namespaces and seccomp,
but does not require Landlock or a privileged daemon. Its canonical plan is
emitted by `compile_linux_bubblewrap_plan`; the existing `compile_linux_plan`
continues to describe the explicit Landlock backend.

- Start with an empty mount tree, bind fixed runtime and declared read roots
  read-only, and bind write roots and scratch read-write. Canonical targets
  and explicit aliases preserve the authorized object. Writable parent grants
  also cover nested read roots. Synthetic parent directories are read-only.
- A protected root inside a writable bind gets its own read-only bind. Mounts
  are applied shallowest first, so a write root strictly beneath it is bound
  read-write again after it. Bind mounts follow the directory, so renaming an
  ancestor moves the read-only view with it. A write root strictly beneath a
  protected root that is not its own canonical name is refused.
- Create a private `/proc` for the PID namespace and a minimal `/dev`. Never
  bind the host process/device trees. Roots resolving to `/`, write roots
  overlapping `/proc` or `/dev`, and aliases to those host trees are refused.
The host must keep policy root identities stable during launch. Bubblewrap
0.4 resolves bind-source paths during setup; this contract does not isolate
against a hostile, concurrent same-UID host process replacing authorized
roots. User namespaces are not a substitute for separate service accounts.

- Drop every capability. Build and seal a seccomp BPF memfd in the parent;
  the fork child only passes its descriptor. Bubblewrap installs the filter
  **after** namespace/mount setup and before executing the target. It uses the
  common syscall restrictions below, except metadata mutation syscalls:
  read-only/absent mounts enforce those boundaries, so chmod/utime/xattr work
  inside writable roots.
- Network `deny` adds a private network namespace and the common socket
  restrictions. `all` retains the host network. `loopback` and
  `allow_process=false` are explicitly refused; executable replacement is not
  confined by this backend. No unsupported policy silently loses a restriction.
- Ordinary helpers die with their parent. Detached jobs keep their dedicated
  host session/process group and survive broker exit; their PID namespace
  is bound to the durable job identity. Cancellation/stop must terminate the
  namespace, including descendants outside the initial process group. Durable
  jobs require `pidfd_open`/`pidfd_send_signal`; launch checks support and signal
  permission before releasing the target, using signal zero without killing it.

`linux-landlock-seccomp` emits the canonical JSON plan that describes what the
Linux launcher applies: the same roots and network class, Landlock ABI
minimum, and a closed seccomp syscall class. Applying it is mandatory;
compilation without a working kernel backend is not success. The launcher
prepares a Landlock ruleset and a seccomp filter in the parent and applies
both in the forked child, after `PR_SET_NO_NEW_PRIVS` and before `execve`:

- **Landlock** requires ABI 3 or later: ABI 1/2 cannot prevent truncating
  files outside the write roots and are rejected as `unsupported`. It
  handles every filesystem right the running ABI defines up to
  `TRUNCATE` (ABI 3). Read roots and the fixed system runtime roots (`/usr`,
  `/bin`, `/sbin`, `/lib*`, `/etc`, `/proc`, `/run/systemd/resolve`,
  `/sys/devices/system/cpu`, `/sys/fs/cgroup`) get read access; write roots
  and scratch get every handled right; `/dev/null`, `/dev/zero`, `/dev/full`,
  `/dev/random`, `/dev/urandom` and `/dev/tty` are readable and writable.
  Missing system roots grant nothing. Execution is granted on the readable
  roots only when `allow_process` is true; the requested executable and its
  ELF interpreter are always executable. Landlock also keeps the confined
  tree from tracing or reading `/proc/<pid>` state of processes outside it.
  Landlock only grants access and cannot keep a subtree of a write root
  read-only, so a policy with a protected root at or beneath a write root
  (other than a write root strictly beneath that protected root) is refused
  as unavailable; such a session needs the bubblewrap backend.
- **seccomp** kills a foreign syscall ABI (and refuses x32 numbers), and
  refuses: kernel-module, kexec, mount, `pivot_root`, `setns`, `unshare` and
  namespace-creating `clone` flags, the kernel keyring, `bpf`,
  `perf_event_open`, `userfaultfd`, `io_uring` (whose operations bypass socket
  rules) and the `TIOCSTI`/`TIOCLINUX` terminal-injection ioctls. `clone3`
  returns `ENOSYS` so libc falls back to the inspectable `clone`. Because
  Landlock does not mediate file metadata mutations, seccomp also refuses
  the chmod, chown, set/remove-xattr and utime syscall families, including
  their fd and `*at` variants. These operations are refused even inside
  writable roots; file creation modes and ordinary content writes remain
  available.
- **Network** `deny` refuses `socket()` for every domain, plus `connect`,
  `bind`, `sendto`, `sendmsg` and `sendmmsg`. `socketpair` read/write IPC
  stays available, but cannot be redirected to an outside Unix socket.
  `all` adds no socket rule. `loopback` cannot be enforced by
  Landlock or seccomp and is refused at launch.
- **Process** creation without `allow_process` refuses `fork`, `vfork` and
  `clone` without `CLONE_THREAD`, so only threads of the requested program
  run. This does not provide complete process isolation: signal scoping
  is not enabled, so the sandbox can signal same-UID processes, and
  the allowed ELF loader can replace the current process with another
  readable executable. Production shell/job policies allow processes.

Other architectures probe as `unsupported`. Landlock does not mediate
filesystem Unix sockets; `all` network also reaches local socket files the
user can open. Bubblewrap exposes only sockets in mounted roots but does not
restrict network destinations under `all`.

Profiles escape path literals and reject control/NUL characters. Golden bytes
live in `fixtures/sandbox/`.

## Probe and fail-closed launch

At boot and after an executable/platform change, the Platform seam compiles
the golden probe policy and runs a confined probe which must prove both an
allowed read, an allowed content write, and a denied write outside the policy.
Each probe subprocess has a five-second deadline. Probe status is one of:

```text
{"available":{"backend":string,"version":string}}
{"unavailable":{"class":"missing"|"rejected"|"unsupported","detail":string}}
```

Side-effect-capable exec is refused when unavailable. It may run unsandboxed
only when replay finds a durable grant bound to the exact
`{run, call, policy_digest}`. A grant for another run, call, or policy is not
reusable; standing configuration cannot silently substitute for it. Refusal
is a terminal denied tool result, not a fake empty success.

## MCP-only hosts

A host whose own application sandbox forbids applying a nested profile can
place a regular file named `.mcp-only-app-sandbox` in the storage root,
owned by the supervisor's effective uid with mode `0600`. The supervisor then
keeps its job broker present but refuses every job as unavailable instead of
probing a backend; MCP tools are unaffected. A marker with another owner,
mode or file type fails startup with `required-broker-unavailable`. Other
side-effect-capable tools still probe their backend and fail closed as above.

## Platform coverage

macOS uses Seatbelt (`sandbox-exec` or equivalent libsandbox entry point) and
the profile applies before the requested executable starts. Linux prefers
bubblewrap plus seccomp and falls back to Landlock (ABI 3 or later) plus
seccomp only during backend probing. If neither is usable, probing reports
unavailable. A failed namespace, ruleset or filter setup never starts the
requested program without confinement.
Mobile has no exec backend. Sandboxing is inherited by the full child tree;
rlimits, clean environment, and advisory path validation are complementary,
not substitutes.

`fixtures/sandbox/` contains canonical policies, Darwin profile bytes, Linux
plans, probe outcomes, invalid policies, and unsandboxed-binding cases.
