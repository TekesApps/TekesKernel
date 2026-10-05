# spec: macOS local deployment v1

This contract defines Slice 10: packaging the completed Kernel as a local,
production App Server on macOS. It does not add semantic authority or a
self-evolution subsystem. Ledger, config, protocol-carrier, secret-store, and
external-world authority classes remain those of D-1; binary promotion and
rollback remain external as required by doc 13.

## Supported product

The v1 service bundle contains exactly these three long-lived/worker
executables. The supervisor is carried inside a signed application bundle;
the other two are bare command-line binaries:

- `apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor` — the one
  resident owner of listener, sweep, brokers and worker process table;
- `tekes-worker` — per-line ephemeral worker;
- `tekes-helper` — sandboxed multi-call exec/filesystem helper.

The enclosing installer package also contains the external `tekes-selector`
launcher/management artifact installed by `packaging/` and defined below. It
is the resident child-process owner launched by launchd, but it is never a
Kernel process and is not reachable as a supervisor self-update API. Its only
authority is installed-binary selection, process lifetime, health attribution,
and deployment rollback; it cannot write thread, config, protocol, secret, or
other semantic-authority files.

For the Tekes application product, the signed deployment release is wrapped
with `TekesKernelInstaller.app` and one canonical
`product-manifest.canonical.json`. The manifest binds the exact installer
contract, release install identity, bundle manifest and selector manifest by
SHA-256. The installer accepts only `status`, `install-or-upgrade`,
`ensure-running`, `enable`, and `disable` under protocol
`tekes-kernel-product-installer`. It validates the entire wrapper and the
calling signed Client before its first mutation. The Client never writes the
Keychain item, install root or LaunchAgent directly, and the production wrapper
has no debug helper or ProductionUAT fallback.

The launchd label is `com.tekes.kernel.supervisor`. The authoritative lane is
arm64 macOS 15 or newer on a supported local APFS volume. x86_64 may be shipped
only after the same non-performance gates pass. Network/iCloud storage is
rejected before readiness.

The external `tekes-selector` artifact creates and syncs the exact
storage root plus an empty root-lock file during installation. Slice 10 has one
production discovery authority: the fixed origin `http://127.0.0.1:7347` with
Client driver name `.tekes`. The installer writes the storage root and that
literal loopback address as launchd `ProgramArguments`; neither is inferred
from cwd, environment, a random port, or a second discovery file. The
service uses no shell, inherited user environment, or relative path. The
selector replaces its launchd-started process image before doing any work when
the process environment is nonempty; that replacement has an empty environment.
Every selector-to-supervisor and supervisor-to-worker spawn also starts from an
empty environment and adds no values. Named integration credentials remain
limited to their separately declared, targeted child-spawn contracts. Storage,
config, cache, logs, and protocol journals use the layouts in docs 01 and 08.

The product copied into `Tekes.app` at build time is only a signed candidate
and installer input. Running it in place would bypass the immutable install
identity, durable activation transaction, launchd ownership, candidate
observation, last-known-good selection, and crash-loop rollback required by
D-53/D-69. Those requirements—not generic app-update behavior—are why the
installer and resident selector remain separate from the GUI process.

The `.tekes` Client driver MUST default to that origin and MUST reject a
non-loopback replacement in Slice 10. A bind conflict reports bootstrap code
`listener-unavailable` before any HTTP listener exists; selector status/logs,
not a fabricated `/health/ready` response, expose it. During a candidate
observation it is one failed launch identity; on an ordinary promoted reboot it
leaves the selector resident without a child until the conflict is removed and
the normal retry schedule succeeds.

The package installer creates one 32-byte random endpoint bearer credential in
the login Keychain using exact generic-password identity
`service=com.tekes.kernel.endpoint`, `account=loopback-bearer`, and access
group `<team-id>.com.tekes.shared.endpoint`. `<team-id>` comes from the
immutable installation identity described below, not from whichever candidate
manifest is currently being staged. (`TEKESAPP01` is only the canonical fixture
team id.) The signed installer has management access sufficient only to create,
replace and delete this exact item; the signed Tekes Client and
`tekes-supervisor` have read access. All three designated requirements and
signed `keychain-access-groups` entitlements MUST name the installation team
and effective group. Selector, helper and worker have no such entitlement and
never receive the value; no
descriptor or IPC broker is an implicit actor. Missing item, wrong byte count,
or ACL/access-group failure reports `endpoint-credential-unavailable` over the
bootstrap pipe before listener bind. The Client reports a local credential
error without sending an unauthenticated mutation.

Upgrade and rollback retain the item's value unchanged. A bearer written
before the service was renamed lives under `com.tekes.kernel.endpoint.v1`:
the first read republishes those exact 32 bytes under the current service and
deletes the superseded item, so the token every reader already holds stays
valid. Uninstall deletes both names. Provider secrets move the same way, one
credential at a time, on the read that precedes any generation decision. Explicit installer rotation
requires the launchd job booted out and `.service-lock` obtainable, replaces
the Keychain value atomically with a new 32-byte value, and bootstraps the job;
both supervisor and Client reread it for each new process/connection. Rotation
is not a selector transaction and cannot change selected bundles. Full
uninstall deletes exactly that Keychain item after bootout while retaining user
thread/config/archive data; a later reinstall generates a fresh value. The
credential and Authorization header MUST NOT appear in plist, argv,
environment, logs, discovery material, selector state or support bundles.

Provider and credential-conditional tool secrets use the separate derived
generic-password authority in [secret-store](secret-store.md): service
`com.tekes.kernel.provider-secret`, account equal to the config credential
id, and access group
`<team-id>.com.tekes.kernel.provider-secrets`. Only the signed installer and
supervisor carry that group. The Client carries only the endpoint group;
selector, helper and worker carry neither. Release verification checks both
derived groups against the same immutable `InstallIdentity.team_id`; the
endpoint bearer is never a provider-secret fallback.

Any actor carrying a `keychain-access-groups` entitlement MUST be an
application bundle containing `Contents/Info.plist`, its sole executable under
`Contents/MacOS`, `Contents/_CodeSignature/CodeResources`, and
`Contents/embedded.provisionprofile`. The embedded profile MUST decode as an
Apple CMS profile, be unexpired, match the immutable installation team and the
actor's application identifier (an exact grant or `TEAMID.*`), and authorize every effective access
group either explicitly or through that team's wildcard. The release builder
executes a side-effect-free contract/build probe from each newly signed app;
an app that macOS admission kills or rejects is not a valid artifact even when
`codesign --verify` succeeds. Bare supervisor/installer/Client-UAT binaries and
profiles placed beside a binary are invalid. The closed actor/profile rows are
`fixtures/deployment/provisioning-profiles.canonical.json`.

The signed application's own application identifier and team entitlements remain
exact; wildcard authorization is accepted only in the Apple-issued profile.

Before its first Keychain or install-root mutation, the installer validates its
own code signature/entitlement, the installed Tekes Client, the stable selector
manifest and the first supervisor bundle, then publishes this canonical-plus-LF
file with temp-write, `F_FULLFSYNC`, rename and parent-directory sync:

```text
InstallIdentity = {format:1,team_id:str,access_group:str,
  installer_requirement:str,client_requirement:str,
  selector_requirement:str,supervisor_requirement:str}
```

It is immutable until a closed full uninstall ends that installation. Every staged bundle and selector update
MUST have the same `signing.team_id`; the actual code signature, designated
requirement and effective access-group entitlement of every applicable actor
MUST match InstallIdentity before publication. Candidate bundle or selector
manifest/signature mismatch is selector `invalid-bundle`/65; installer, Client
or effective entitlement mismatch is installer `invalid-install`/65. All fail
before activation or stable-selector replacement. Signing
team migration is not an upgrade: it requires complete uninstall followed by a
fresh install identity and fresh credential.

## Installed layout and permissions

This is a per-user LaunchAgent. It is eligible only after the target user has
logged in; v1 makes no pre-login or system-boot residency claim. After a
reboot, that login starts the service without requiring the Tekes GUI to be
opened. `USER_HOME` is resolved by the installer from the target uid, never
from inherited environment. The production layout is:

```text
USER_HOME/Library/Application Support/Tekes/Kernel/
  bundles/<version>/manifest.canonical.json
  bundles/<version>/apps/TekesKernelSupervisor.app/Contents/
    {Info.plist,embedded.provisionprofile,_CodeSignature/CodeResources,
     MacOS/tekes-supervisor}
  bundles/<version>/bin/{tekes-worker,tekes-helper}
  selector/bin/tekes-selector
  selector/{current.json,previous.json,active,.lock,.service-lock}
  selector/operations/current.json
  selector/observations/<version>.json
USER_HOME/Library/Application Support/Tekes/Installer/
  {.lock,install-identity.json,operation.json}
USER_HOME/.agents/
  settings/
  workspaces/<workspace-id>/{workspace.json,state/,skills/}
  threads/.root-lock
  jobs/ config/ cache/ runtime/composer/ skills/ client/ not-in-project/
  credential-state/{.lock,provider-secret-generations.json}
  logs/kernel/
USER_HOME/Library/LaunchAgents/com.tekes.kernel.supervisor.plist
```

The install root contains replaceable signed program bytes and its installation
journal; `~/.agents` is the separate durable data root. The installer performs
an idempotent journaled same-volume migration of legacy data directories under
Application Support and historical `~/Library/Logs/Tekes/Kernel`, rejects
old/new conflicts and symlinks, and never moves Keychain items or the
LaunchAgent plist.

The legacy-data migration journal is a closed canonical object whose unit
names come only from the fixed migration inventory. On every retry the
installer re-enumerates that inventory and verifies each recorded
device/inode/mode against exactly one of source or destination. A source
omitted by the journal, duplicate unit/completed entry, `completed` phase that
does not name every recorded unit, forged completed unit still at source, or
source+destination conflict rejects. Each unit is a same-volume atomic rename;
`EXDEV` never falls back to copy/delete.

The install root, bundle directories, selector directories, data root,
cache, log and application-bundle directories are mode `0700`; bundle manifests, selector JSON,
observation/operation/lock files, installer JSON/lock files and `.root-lock` are `0600`; the plist is
`0644`; app `Info.plist`, embedded profile and `CodeResources` are `0644`;
executables are `0755`. All are owned by the target uid, have no
group/world write bit, and contain no symlink except selector-owned `active`.
Every parent is opened no-follow and path-to-inode revalidated. A mismatch is
`invalid-install`, never repaired by a Kernel process.

The plist's `Program` is the stable
`.../Kernel/selector/bin/tekes-selector`. Its `ProgramArguments` are exactly
that path, `--install-root`, the absolute Kernel install root, `serve`,
`--storage-root`, the absolute `~/.agents/threads` root, `--listen`, and literal
`127.0.0.1:7347`. `serve` rejects any other listen address in v1 and takes
`selector/.service-lock`
for its entire lifetime. Under short tenures of `selector/.lock` it completes
operation recovery, validates `active/current/previous` and the selected
immutable bundle, freezes the selected tuple, and then spawns the supervisor as
its direct child. launchd owns/restarts only the selector; the selector owns,
signals, drains, observes, and reaps the supervisor and its process group.

The plist contains `RunAtLoad=true`,
`KeepAlive=true`, `ProcessType=Background`, and absolute stdout/stderr paths
under the log directory. It contains no `EnvironmentVariables`, shell,
relative path, secret, provider setting or mutable bundle path. The canonical
plist fixture owns the XML bytes; installation substitutes only the explicit
`@INSTALL_ROOT@`, `@STORAGE_ROOT@`, `@SELECTOR@`, `@STDOUT@`, and `@STDERR@` tokens,
then validates and publishes it.

Before each spawn, `serve` creates a pipe and passes the frozen selection by
exact argv, never by a post-unlock reread:

```text
tekes-supervisor --install-root <absolute-kernel-install-root>
  --storage-root <absolute-data-threads-root> --listen <loopback-address>
  --selected-version <id> --selector-generation <int>=1
  --launch-id <generation-attempt-32-lowercase-hex>
  --manifest-sha256 <lowercase-64-hex> --bootstrap-status-fd 3
  --authority-registry-sha256 <lowercase-64-hex> --launcher-lifetime-fd 4
```

File descriptor 3 is the write end of a selector-created `CLOEXEC`-by-default
bootstrap pipe. File descriptor 4 is the read end of a lifetime pipe whose
write end is held only by selector; EOF means the launcher died. Only fds 3–4
have `CLOEXEC` cleared for the child;
all other selector and root descriptors remain `CLOEXEC`. The supervisor derives readiness
`build/generation` and `run_start.binary` from this immutable handoff, verifies
its embedded build and authority-registry digest against it, and never rereads
mutable selector files. Worker/helper spawn inherits the already-validated v1
writer-profile obligation through the existing frozen supervisor contracts;
they do not read selector state.

The supervisor's sole side-effect-free `--describe-build` invocation prints
canonical-plus-LF `{authority_registry_sha256,format:1,identifier,source_revision?,version}`
from compile-time constants. `source_revision` is the 40-hex Git commit given
at compile time in `TEKES_SOURCE_REVISION`; development builds omit it, and any
other value fails the build. Signed assembly accepts the release version and
the source revision as explicit inputs and rejects any probe byte, embedded
version, source revision, or registry digest that differs before publication.
The revision is provenance only: signatures and digests stay the authority.
Because the signed supervisor is a bundle file, the bundle manifest digest
binds it, and `tekes-supervisor --describe-build` on the artifact recovers the
commit without a new manifest field.

## Ownership and readiness

Startup order is fixed:

1. validate executable ownership, permissions and code signature;
2. open the installer-created storage root/root-lock without following links
   and acquire the single supervisor-root ownership lock nonblocking; the
   selector launches the supervisor from the selected app bundle's
   `Contents/MacOS` path, never from `bin/`;
3. under that lock, validate/create the exact storage subdirectories without
   following links;
4. require the production authority root to reside on local APFS, then probe
   locking, append, full-sync, rename and directory-sync behavior without
   releasing the ownership lock; the generic Store may support other local
   filesystems, but that does not widen this deployment profile. Before any
   mutable probe, canonicalize the storage path and reject any path containing
   the adjacent components `Library/Mobile Documents`, including an ancestor
   symlink alias into that iCloud-managed tree;
5. load/validate config and secret handles;
6. repair only permitted torn tails and recover staged operations;
7. run the boot sweep and endpoint-journal validation;
8. bind `127.0.0.1:7347` and report ready; `EADDRINUSE` reports
   `listener-unavailable` over the bootstrap-status pipe.

A second instance exits with typed `already-running`; it never steals a lock or
starts a listener. Any corruption, unsupported filesystem, invalid config,
missing executable, protocol mismatch, or required broker failure is
fail-stop/not-ready. Optional capability failures remove that capability and
its catalog entries rather than failing readiness.

`GET /health/ready` exists only after step 8 has bound the listener and has no
RPC envelope. Ready is HTTP 200 with exact canonical JSON body
`{"build":B,"generation":G,"ready":true}`; `B/G` are the frozen selector
handoff. HTTP 503 is limited to an already-bound listener that is still
finishing boot or is draining, with
`{ready:false,error:{code,message,details:{}}}`. The closed HTTP rows are:

| code | message |
|---|---|
| `boot-incomplete` | `Startup has not completed` |
| `server-draining` | `Endpoint is draining` |

No extra fields or nulls are permitted. `/health/live` is HTTP 200 with
`{"live":true}` only after process initialization; it says nothing about
readiness. Bodies use `application/json` and decimal content length.

A rejection before listener bind is never synthesized as HTTP. The supervisor
writes exactly one canonical JSONL `BootstrapStatus` record to fd 3 before
exit, and the selector publishes the same stable classification in its status
reply and structured log:

```text
BootstrapStatus = {format:1,launch_id:str,selection:Selection,
  state:"listener-bound"}
| {format:1,launch_id:str,selection:Selection,state:"failed",
   code:"already-running"|"invalid-install"|"unsupported-filesystem"|
        "invalid-config"|"corrupt-ledger"|"protocol-mismatch"|
        "required-broker-unavailable"|"selector-mismatch"|
        "listener-unavailable"|"endpoint-credential-unavailable"|"io"}
```

`listener-bound` is emitted once immediately after bind; subsequent readiness
is observed over HTTP. EOF/child exit before either record is `launch-failed`.
An invalid/mismatched record is `protocol-mismatch`. Thus a zero-listener
startup failure is inspected through `tekes-selector status`, not an impossible
HTTP request.

The supervisor process exits `0` after a completed drain, `66` for a typed
startup rejection such as already-running/invalid config/unsupported
filesystem, `74` for I/O before semantic recovery can proceed, and `75` for
corruption. launchd signal termination remains the OS status. The stable
readiness code, not exit text, distinguishes rows sharing an exit.

## launchd and process lifetime

launchd owns selector restart; the selector owns supervisor restart. The
supervisor owns workers/helper/job and daemon process groups according to their
contracts and reaps them. Workers do not daemonize. On SIGTERM or an explicit
management drain, selector `serve` stops respawning, forwards the bounded drain
request/signal, waits for the supervisor process group and reaps it, then
releases `.service-lock` and exits. A second SIGTERM or deadline expiry kills
the child group before selector exit. SIGKILL of either process is recovered by
launchd restarting `serve`; boot `serve` first recovers selection transactions,
then the next supervisor performs the existing boot sweep.

Supervisor continuously watches fd 4. EOF first makes readiness false and then
forces the same bounded drain/reap/release-root-lock exit path, so a selector
SIGKILL cannot leave an init-adopted listener indefinitely. A newly started
selector probes the root ownership lock before allocating a new launch attempt.
If the predecessor still owns it, selector logs
`selector-predecessor-draining`, waits at 100 ms intervals for at most the
30-second drain deadline, and does not increment candidate failure count. It
spawns only after the lock becomes free. Timeout is `orphan-owner-timeout`;
selector full-sync appends the closed `orphan-owner-timeout` operational record
before remaining resident without a child and requiring operator repair. This
existing observability authority is the durable prelaunch/status carrier, not
an `Observation`: it creates no `launch_id`, does not increment `attempt` or
`consecutive_failures`, and never drives recovery or fabricates candidate
evidence. Its closed fields carry the selected version/generation/manifest and
the timeout; `status` projects them as `prelaunch_failure`. The next successful
root-lock probe full-sync appends `selector-prelaunch-cleared` before publishing
a new launch observation. Rotation reads generations oldest-to-newest, so the
latest of those two codes is total even across restart. A
child bootstrap `already-running` caused by a manual race follows the same
wait/retry path and is never candidate failure evidence.

Crash-loop observation and last-known-good selection are owned and executed by
resident `tekes-selector serve`, not by a Kernel process or a test-only actor.
The supervisor emits attribution/health evidence but cannot invoke the
selector, replace, or relaunch its own binary. `deployment-tests` drives this
same production path.

## Selector CLI and durable state

The selector command is language-neutral:

```text
tekes-selector describe-conformance
tekes-selector --install-root <absolute-path> stage --bundle <absolute-path> --version <id>
tekes-selector --install-root <absolute-path> activate --version <id>
tekes-selector --install-root <absolute-path> rollback --reason <code>
tekes-selector --install-root <absolute-path> status
tekes-selector --install-root <absolute-path> recover
tekes-selector --install-root <absolute-path> attest-canary --version <id> --session <UUID> --run <id> --storage-root <absolute-path>
tekes-selector --install-root <absolute-path> serve --storage-root <absolute-path> --listen 127.0.0.1:7347
tekes-selector --install-root <absolute-path> update-selector --artifact <absolute-path> --manifest <absolute-path>
```

`id` and `reason` are 1–128 ASCII bytes from `[A-Za-z0-9._-]` and do not start
with `.`. Options occur exactly once in the shown order; unknown, missing or
duplicate options fail without state change. Each short command's success
stdout is exactly one of these RFC-8785 canonical JSON objects plus LF:

```text
StageReply    = {format:1,operation:"stage",selection:Selection}
ActivateReply = {current:SelectionFile,format:1,operation:"activate",
                 previous:PreviousFile}
RollbackReply = {current:SelectionFile,format:1,operation:"rollback",
                 previous:PreviousFile}
RecoverReply  = {format:1,operation:"recover",recovered:bool,
                 selection?:SelectionFile}
StatusReply   = {format:1,service:"running"|"stopped",
                 selection?:SelectionFile,previous?:PreviousFile,
                 observation?:Observation,last_failure?:Failure,
                 prelaunch_failure?:PrelaunchFailure}
CanaryReply   = {format:1,operation:"attest-canary",run:str,session:UUID,
                 version:str}
UpdateReply   = {format:1,operation:"update-selector",sha256:hex,version:str}
ConformanceReply = {architecture:"aarch64",conformance_sha256:hex,format:1,
                    operation:"describe-conformance",version:str}
Failure       = {code:str,launch_id:str}
PrelaunchFailure = {code:"orphan-owner-timeout",format:1,generation:int>=1,
  manifest_sha256:lowercase-64-hex,observed_at:RFC3339Nano,version:str}
```

The selector fixture owns one exact example of every reply and every failure
row. Failure stderr is exactly
`{"error":{"code":C,"details":D,"message":M}}` plus LF, where `C/M` and
the exact keys of `D` come from that closed registry. `serve` emits no success
reply; normal lifetime output is structured operational logging, and a fatal
entry rejection emits only the ordinary selector error line. Neither stream
contains bundle contents, inherited environment or secrets.

Closed exits are `0 success`, `64 usage`, `65 invalid-bundle`,
`66 invalid-state`, `69 unavailable`, `74 io`, and `75 corruption`. A signal
exit remains an OS status and is never translated to success.

`current.json` and `previous.json` are canonical JSON plus LF:

```text
SelectionFile = {format:1, generation:int>=1, selection:Selection}
PreviousFile  = {format:1, generation:int>=1, selection?:Selection}
Selection = {version:str, manifest_sha256:lowercase-64-hex}
```

Both files always exist after first activation and have equal `generation`;
`previous.selection` is absent for the first version. `active` is a relative
symlink exactly `../bundles/<current.version>`. `status` exits 75 if these
facts disagree. JSON publication is same-directory temp-write, file
`F_FULLFSYNC`, rename, then selector-directory sync.

The bundle manifest is canonical JSON plus LF:

```text
BundleManifest = {
  format:1, version:str, minimum_os:"15.0", architectures:["aarch64"],
  files:[BundleFile,...], compatibility:{
    authority_registry_sha256:lowercase-64-hex,
    reader_profile:"v1",writer_profile:"v1"},
  signing:{team_id:str,requirement:str}
}
BundleFile = {path:str,bytes:int,sha256:lowercase-64-hex,mode:"0755"}
AuthorityRegistry = {format:1,profile:"v1",authorities:[Authority,...]}
Authority = {authority:str,contract:str,writer_profile:"v1"}
```

`signing.team_id` is exactly ten uppercase ASCII letters or digits and MUST
equal immutable `InstallIdentity.team_id`. The endpoint access group derives
only when that installation identity is first created; a candidate manifest
cannot substitute another team or access group. The provider-secret group is
independently derived from that same team id as frozen by secret-store;
adding it does not create another install identity. `SelectorManifest.signing`
is subject to the same equality but grants neither Keychain group.

`files` is sorted by path and contains exactly the three `bin/` executable
paths. Paths are normalized relative paths with no `..`; targets are regular
files. The manifest digest is SHA-256 over the exact canonical-plus-LF bytes.
`stage` verifies counts, hashes, modes, architecture, OS, compatibility and
code requirement before publication. A published version directory is never
replaced: `stage` of a valid bundle whose version is already published with a
different manifest digest returns `invalid-state`/66 with state
`version-already-published`, so every distinct build needs a distinct version.

The authority registry is RFC-8785 canonical JSON plus LF, its rows are sorted
by `authority`, and its digest covers the exact bytes. It is the closed Slice 10
inventory of every durable or recoverable writer: semantic ledgers, config and
instruction snapshots, seed/checkpoint assets, rewrite operations, endpoint
projection journals, management carriers, control receipts,
memory/goals/tool-state/jobs, launch bindings, provider sealed/adopt assets,
and the provider-secret generation authority.
An executable that adds a durable authority not present in the registry, does
not implement every registry row, or binds a different registry digest is an
`invalid-bundle`/65; unknown authorities fail closed. The canonical registry is
a conformance oracle embedded by digest in the signed executables/manifest, not
a mutable installed authority file. The deployment fixture owns the exhaustive
names and contract paths.

There is deliberately no scalar writer generation and no incomplete
per-authority version vector. Slice 10 supports exactly reader/writer profile
`v1`. Let `P` be the selected version retained as rollback-eligible previous
and `C` the candidate. Activation while `P` is rollback eligible requires:

```text
C.compatibility == P.compatibility
C.compatibility.authority_registry_sha256 == sha256(authority-registry.canonical.json)
C.compatibility.reader_profile == "v1"
C.compatibility.writer_profile == "v1"
```

The frozen registry digest is passed in argv and every candidate writer MUST
continue emitting the existing v1 bytes throughout the rollback window. This
strong equality is intentional: the candidate may crash after any durable
write or leave an incomplete operation that the previous binary must recover.
A schema/profile change may be activated only after the previous observation
has been promoted with `rollback_eligible=false`, and then only under a
separately approved offline migration contract that replaces the closed
registry/profile; the stale `rollback` command remains forbidden. `stage`
validates the signed candidate manifest and its offline executable evidence
against selector's embedded accepted registry digest; `activate` applies the
exact equality against current/previous. The compatibility fixture
owns accepted equality, digest mismatch, profile mismatch, and unknown-authority
rejections.

The stable selector is independently package-bound by canonical-plus-LF:

```text
SelectorManifest = {format:1,version:str,minimum_os:"15.0",
  architecture:"aarch64",conformance_sha256:lowercase-64-hex,
  file:{path:"selector/bin/tekes-selector",bytes:int,
        sha256:lowercase-64-hex,mode:"0755"},
  signing:{team_id:str,requirement:str}}
```

`describe-conformance` is the sole command without `--install-root`. It is
read-only, accepts no other argument, opens no installation or user path, and
prints exactly one `ConformanceReply` plus LF from constants embedded in that
selector executable.

`update-selector` requires launchd booted out and `.service-lock` obtainable.
It opens the candidate no-follow, verifies package signature, manifest, bytes,
hash, architecture, OS, mode and code requirement, then invokes that already
verified candidate as `describe-conformance` with null stdin, captured
stdout/stderr and a 10-second/4096-byte bound. The probe starts with an empty environment, inherits
no selector-owned descriptor (all are `CLOEXEC`), and has only null stdin plus
captured stdout/stderr; any stderr byte is rejection. The exact canonical
single-line reply's version, architecture and conformance digest MUST equal the
candidate manifest. It does not compare
the new digest with the old selector's digest: the signed candidate proves its
own package conformance evidence. Timeout, nonzero exit, extra output,
noncanonical/unknown fields or any mismatch is `invalid-bundle`/65 before
mutation. It then copies to a same-directory private temp, `F_FULLFSYNC`s it,
revalidates it, renames it over the stable
path, then syncs `selector/bin`. The stable pathname is therefore always the
complete old or complete new executable; it is never overwritten in place.
The package manifest is an input oracle, not a separately committed runtime
pointer, so a crash cannot expose a new binary with an old installed manifest.
The update command may finish from its already-open old executable after the
rename. Repeating the same verified update returns the existing `UpdateReply`.

Crash atomicity is not functional last-known-good for the selector itself.
Before `update-selector`, the package-owned new selector MUST pass the complete
offline selector fixture/conformance suite, including `serve` bootstrap in an
isolated root, and the signed SelectorManifest binds that evidence digest.
Gate 73 rejects update without that evidence. If a conformant, signed new
selector nevertheless fails only in the real installation after rename,
launchd may repeatedly fail to start it; supervisor bundle rollback cannot and
MUST NOT replace it. Recovery is an explicit installer repair that boots out
the job and atomically republishes a known-good signed selector. v1 deliberately
does not claim automatic selector-binary fallback; that would require a
separate immutable bootstrap or dual-slot authority.

The consumer product installer implements that explicit repair when the installed selector
cannot reach the atomic update boundary. After the installer has independently verified the
package root, canonical manifests, digests, no-symlink layout, and code signatures, it invokes
the package-owned candidate with the same closed `update-selector` grammar. The candidate must
still acquire the offline service lock, recover the installation, revalidate itself against the
immutable install identity and SelectorManifest, and atomically publish the stable pathname.
This is an installer-bounded repair of a broken old selector, not an automatic runtime fallback.

Observation state is canonical JSON plus LF and is published by temp-write,
file `F_FULLFSYNC`, rename and observations-directory sync:

```text
Observation = {format:1,version:str,generation:int>=1,
  manifest_sha256:lowercase-64-hex,attempt:int>=1,launch_id:str,
  started_at:RFC3339Nano,deadline_at:RFC3339Nano,
  canary_required:bool,canary_deadline_at?:RFC3339Nano,
  rollback_eligible:bool,window_closes_at?:RFC3339Nano,
  consecutive_failures:int>=0,last_code:str,
  canary_session?:UUID,canary_run?:str,
  state:"pending"|"ready"|"failed"|"promoted"}
```

`canary_session` and `canary_run` are present together only after accepted
attestation and are absent together otherwise. The pair is immutable while the
observation remains for that selected generation.

`serve` is the sole observer. Each child spawn gets a new `launch_id`, increments
`attempt`, fixes `started_at/deadline_at`, and publishes `pending` under a short
transaction-lock tenure before spawn. It never holds `.lock` while waiting on a
pipe, child, timer, HTTP readiness or canary. The closed failure-attribution
oracle separates candidate evidence from shared environment/data failures.
Only `invalid-install`, `selector-mismatch`, `protocol-mismatch`, unexpected
child exit, readiness deadline, and canary timeout/attribution failure are
candidate-attributable. A unique attributable launch attempt increments
`consecutive_failures` at most once; repeated polls within one attempt are not
independent failures.

`already-running`, `unsupported-filesystem`, `invalid-config`,
`corrupt-ledger`, `required-broker-unavailable`,
`endpoint-credential-unavailable`, `listener-unavailable`, and bootstrap `io`
are environment/data classifications. They publish the failed observation and
leave selector resident/not-ready, but never increment candidate failure count
or switch generation. Retry delays are exactly 1, 2, 4, 8, then 30 seconds.
During each delay selector polls at 100 ms and compares a fact fingerprint over
the byte-exact config tree (path/type/mode/content), install-identity bytes,
installer `operation.json` bytes-or-missing (the non-secret credential-rotation
change signal),
storage-root `(device,inode,mode)`, root-lock state and whether the fixed
listener can be bound. A changed fingerprint ends the delay and resets the next
delay to 1 second; an unchanged fingerprint advances the schedule. The digest
is process-local, is never logged or durable authority, and a bind probe is
released immediately. Keychain secret bytes are deliberately outside selector
authority: installer credential rotation bootouts and restarts `serve`, which
also resets the in-memory schedule; an out-of-band Keychain-only repair waits
at most the capped 30-second retry. There is no unspecified notification or
filesystem watcher. The next successful `listener-bound` also resets the
schedule. Thus replacing candidate with previous cannot be a
false repair for a shared bad config, credential, port or ledger. Planned drain, selector
termination and recovered stale-pending state are `launcher-interrupted` and do
not increment the counter.

The constants are exact: bootstrap status timeout 10 seconds; readiness timeout
30 seconds from spawn; health polls at 100, 200, 400 and 800 milliseconds then
1 second until deadline; candidate canary timeout 120 seconds from
`listener-bound`; rollback observation window 300 seconds from accepted canary;
 attributable failed child attempts are respawned after 1 second. Active timers use monotonic
time; RFC3339Nano fields are durable audit anchors and recovery never extends a
recorded deadline after wall-clock rollback.

Activation sets `canary_required=true` and `rollback_eligible` according to
whether previous exists. The explicit deployment controller—installer for an
upgrade, production UAT for first install—uses the real Tekes Client to create
a disposable, non-secret, no-tool canary thread from the canonical deployment
fixture, submit one turn, and wait for settle. It then calls `attest-canary`
before archiving the disposable thread. Selector opens exactly
`<storage-root>/<session>/main.jsonl` no-follow, verifies genesis equals the
UUID, validates the complete durable prefix through the named settle, rejects
zero or multiple matching run ids in that ledger, verifies that run's latest
preceding `run_start.binary` and current frozen version, and records the
`session/run` pair. Run ids are ledger-local and are never searched globally.
Selector never creates or mutates canary semantic events. Missing/invalid
attestation by the 120-second deadline is one candidate-attributable failure.

The consumer Tekes.app product installer has a separate health-only admission command,
`attest-install-health --version <id>`. A clean consumer install has no provider credential and
therefore cannot make model inference a prerequisite for starting the settings surface where that
credential is entered. This command is used only after the independently signed installer and
selector both observe the exact frozen build and generation at the fixed loopback readiness
endpoint; it clears the pending canary requirement and starts the same rollback observation
window. Production deployment and UAT MUST NOT use this lane and continue to require the durable
Client canary ledger above.
Both attestation commands and the resident selector's short observation transactions wait up to
five seconds for the selector transaction lock. This prevents the external admission and the
resident 100 ms observation poll from phase-locking or terminating either side on a transient
`lock-busy`; neither waits on the lifetime service lock.

A candidate attempt becomes `ready` only after bootstrap `listener-bound`, HTTP
200 with the same frozen build/generation, and accepted canary attestation.
Ordinary boot or post-login reboot start of an already `ready` or `promoted`
selection sets `canary_required=false`; bootstrap plus matching HTTP readiness
is sufficient, so an idle logged-in machine with no Client or GUI activity
cannot fail for lack of a new run. A reboot while candidate attestation is
still pending preserves its absolute canary deadline and does not itself
increment failure count.

On accepted canary, `state=ready`, consecutive failures reset to zero and
`window_closes_at` is fixed 300 seconds later. When a rollback-eligible
previous exists, selector remains `state=ready,rollback_eligible=true` for
that complete interval; failures through the inclusive instant immediately
before `window_closes_at` still participate in the rollback counter. At
`window_closes_at`, after first recording any already-observed attributable
failure, selector durably publishes `state=promoted,rollback_eligible=false`;
previous remains an immutable installed bundle but is no longer an automatic
rollback target. Promotion also resets `consecutive_failures=0`; failures after
promotion start a new, non-rollback counter epoch. On first install, where no previous exists, accepted canary
publishes `state=promoted,rollback_eligible=false` immediately and omits
`window_closes_at`.
After promotion, three later ordinary-launch failures leave selector resident
without a child, expose selector-status code `readiness-crash-loop`, and
require operator repair; they never silently revive an expired predecessor.
Before the window closes, three consecutive independent
failures publish `failed`, stop/reap the child, durably roll back, then spawn
previous. If no rollback-eligible previous exists, selector remains resident
without a child and exposes the failed observation through `status`.

Promotion and failure commits are serialized by `selector/.lock` using
monotonic time. A terminal attributable failure whose commit begins with
`now < window_closes_at` is recorded before promotion. At equality or later,
promotion wins only when the matching child is still listener-bound and HTTP
ready. If an attempt that began before the boundary is pending or not-ready at
the boundary, selector leaves `rollback_eligible=true` and defers promotion
until that exact attempt terminates: its failure is counted in the old epoch,
or its restored readiness is followed by promotion and counter reset. A launch
created after the boundary cannot extend rollback eligibility. This ordering,
including the strict `<` comparator, is part of the promotion fixture.

`.service-lock` and `.lock` have distinct meanings. `serve` holds only the
former for its lifetime; every selector process holds `.lock` only around
operation recovery and one bounded state read/write. External `activate` and
`rollback`, and every `update-selector`, acquire `.service-lock` and then
nonblocking-probe the installer-created sibling `threads/.root-lock`. Busy is
typed `invalid-state {state:"child-group-running"}`/66 and missing is
corruption; mutation begins only when the probe acquires/releases that lock.
Because the supervisor releases the root lock only after reaping its complete
child group, this is the proof that a SIGKILLed old `serve` has no surviving
child group. Holding `.service-lock` prevents launchd's replacement `serve`
from entering while the offline operation runs. The installer MUST still
bootout launchd first; otherwise a racing live selector makes acquisition fail
without mutation. `stage`, `status`, and `recover` may run
while `serve` is alive; `attest-canary` may do so after its read-only ledger
validation. Their short transaction-lock tenure cannot race an unresolved state
publication.

## Install, upgrade, rollback, uninstall

Installation is staged beside the final versioned bundle, verified, then
published by atomic rename and directory sync. The stable launchd plist points
to an external activation link/version selector. Activation never overwrites a
running executable in place.

An upgrade performs:

1. verify package signature, hashes, exact authority-registry/profile
   compatibility and
   all offline conformance evidence;
2. stage and sync the complete version;
3. `launchctl bootout` the job, causing selector to drain/reap the old
   supervisor, and wait until `.service-lock` is obtainable;
4. if the package changes `tekes-selector`, atomically update and verify its
   stable path while the job is stopped;
5. atomically switch the external selection and sync its directory;
6. `launchctl bootstrap` the unchanged plist when unloaded, or issue a
   non-terminating kickstart when loaded but not ready. A matching ready build
   makes `ensure-running` return without any launchd mutation. Ordinary
   ensure/enable MUST NOT pass `-k`: repeated liveness probes preserve the
   selector/supervisor PIDs, live stream generation and in-flight work;
7. after readiness, have the deployment controller submit the canonical canary
   through the real Client and call `attest-canary`; resident `serve` verifies
   its `run_start` attributes the new build;
8. retain the previous version as last-known-good until the observation window
   closes.

Failure before step 5 leaves the old version active. Failure after step 5 is
handled by resident `serve`: stop/reap the candidate, switch the selection
back, sync, and spawn last-known-good. During the rollback window the candidate MUST
NOT publish a thread, config, or protocol format that the retained previous
binary cannot read and continue writing. Merely failing closed is insufficient
for last-known-good readiness. An incompatible publication is deferred until
that previous version is no longer rollback-eligible under an explicit later
migration contract. Rollback never rewrites ledgers.

### Publication and recovery state machine

Every selector state access takes exclusive `selector/.lock` nonblocking,
recovers the sole `selector/operations/current.json` record, and then validates
`active/current/previous` before continuing. It releases the lock before any
process wait, health request, canary or signal. Any other regular file in the
operations directory is corruption. The record is canonical JSON plus LF:

```text
SelectorOperation = {format:1,op_id:str,command_sha256:lowercase-64-hex,
  actor:"cli"|"serve",type:"stage"|"activate"|"rollback",phase:str,from?:Selection,
  to:Selection,generation:int>=0,launch_id?:str,reason?:str,response?:JsonValue}
```

`command_sha256` is an immutable request identity from `prepared`. For
`actor="cli"` it hashes the RFC-8785 canonical JSON bytes (no LF) of
`{actor:"cli",argv:[str,...]}`. Path operands MUST already be absolute lexical
normal form: one leading slash, no empty/`.`/`..` component, UTF-8 NFC, and no
trailing slash except `/`; path normalization never resolves a symlink, while
the separate validation opens every component no-follow. Hashing uses that
validated vector exactly, including option order. For automatic rollback it
hashes canonical bytes of `{actor:"serve",action:"automatic-rollback",
generation:int,launch_id:str,reason:str,from:Selection,to:Selection}`. Thus no
CLI invocation or hidden stdout actor is fabricated.

`response` is absent before `closed`. At closed it is required only for
`actor="cli"` and is the exact success object whose canonical-plus-LF bytes go
to that invocation's stdout. It is always absent for `actor="serve"`; recovery
of a closed automatic rollback validates the selected state and resumes spawn,
without emitting a CLI reply.

For `stage`, actor is always `cli`, `to` is the candidate manifest selection, `generation` is current
generation or zero before first activation, and `from/reason` are absent. For
`activate`, actor is always `cli`, `from` is current or absent only on first activation, `generation`
is the new generation and `reason` is absent. For `rollback`, `from` is current,
`to` is previous, `generation` is the new generation and `reason` is required.
CLI rollback has `actor="cli"`; crash-loop rollback has `actor="serve"` and its
`launch_id` field plus request identity bind the failing launch. `launch_id` is
required exactly for serve rollback and absent for all CLI operations.
`phase` must belong to the exact list for `type`; other field combinations are
corruption, never inferred defaults.

The prepared record durably declares intent before the first side effect. For
each later side effect, recovery derives whether it occurred from the synced
filesystem fact; after the side effect and its directory sync, the selector
publishes the correspondingly named phase before continuing.
Stage phases are `prepared -> copied -> verified -> bundle-published ->
closed`; activation and rollback phases are `prepared -> link-published ->
previous-published -> current-published -> closed`.
The `closed` record is retained and synced as the exact-retry authority for
only the most recently completed request. A CLI retry with the same actor/hash
and matching current generation/state returns its stored response bytes without
a second effect. Recovery of the same serve request performs no second switch
and continues process reconciliation. A different valid mutation
atomically replaces the closed record with its own durable `prepared` record
before any effect; an in-progress record with another hash is `invalid-state`.
After replacement, an older CLI identity is no longer replayable: it is
validated as a new invocation against current state. Content-identical `stage`
or `activate` already satisfied by current immutable state publishes a new
no-effect closed record and current reply without incrementing generation;
an old `rollback` that no longer satisfies the failed-observation/current-
previous guard returns `invalid-state`/66. No unbounded command history exists.

For stage, a crash before `bundle-published` removes or revalidates the private
staging directory; at or after it the immutable final bundle must hash-match
and recovery advances to closed. For activation/rollback, the directory-synced
`active` link is the decision of record. While it still names `from`, outside
readers see the old selection and recovery re-drives the declared swap; once
it names `to`, recovery idempotently publishes `previous=from`,
`current=to`, both at `generation`, then closes. A link target matching neither
side, or a state file disagreeing with
the operation, exits 75. Recovery never guesses from process liveness.

No service binary reads the selection while an operation is unresolved:
launchd always enters through `serve`, which takes the same selector lock and
performs this recovery before resolving `active` and spawning a child. External
bootstrap/kickstart happens only after the transaction has closed and is safely
repeatable; it is not a transaction phase. This is what makes the
multi-file selector protocol recoverable after machine power loss rather than
only after an installer happens to run again.

`activate` requires a verified immutable bundle and sets generation to one on
first activation or current plus one thereafter. First activation treats an
absent `active/current/previous` set as `from` absent, publishes an absent
`previous.selection`, and is tested at every phase. `rollback` requires a
failed observation for current, `rollback_eligible=true`, and a present previous
selection, uses another new generation, swaps current/previous, and never
deletes either bundle. After promotion, `rollback_eligible=false` makes the CLI
return `invalid-state`/66 with detail state `rollback-window-closed`; a later
downgrade requires a separately approved offline migration contract and a new
activation, never this stale rollback path.
Repeating the most recently completed command returns the retained closed
response without incrementing generation; older identities follow the explicit
revalidation rule above. Thus every power-loss boundary, including
loss after commit but before stdout, exposes one complete old or new selection
and one replayable terminal result.

### Installer transaction and recovery

Install, credential rotation and uninstall span Keychain, selector and launchd,
so the signed installer owns a separate durable operation. It serializes with
`Installer/.lock`, publishes `Installer/operation.json` as canonical JSON plus
LF using temp-write, `F_FULLFSYNC`, rename and directory sync, and never stores
credential bytes or a token hash:

```text
InstallerOperation = {format:1,op_id:str,type:"install"|"rotate-credential"|
  "uninstall",request_sha256:lowercase-64-hex,phase:str,
  install_identity_sha256:lowercase-64-hex,response?:JsonValue}
```

`request_sha256` uses the same exact normalized CLI identity rule as selector
CLI operations. `response` is absent before `closed` and required at `closed`
as the exact canonical-plus-LF installer stdout reply. The closed response rule
is most-recent-only. Phases are:

```text
install: prepared -> identity-published -> credential-published ->
         bundles-published -> selection-published -> plist-published ->
         service-started -> closed
rotate-credential: prepared -> service-stopped -> credential-replaced ->
                   service-started -> closed
uninstall: prepared -> service-stopped -> credential-deleted ->
           plist-removed -> binaries-removed -> closed
```

Each phase is published only after its named effect is atomically complete and
durable. Installer recovery validates the exact completed fact and idempotently
performs the next effect. In particular, recovery after
`credential-published` reuses the existing 32 bytes; it never creates a second
value. Credential replacement and deletion are idempotent Keychain operations.
An item without a matching live/closed installer operation and installation
identity is `corruption`, never adopted or silently rotated.

If launchd reaches `serve` while an installer operation is not closed, selector
logs `installer-recovery-required`, remains resident without a supervisor, and
does not count a candidate failure; selector cannot complete or mutate the
installer operation. Re-running the same signed installer command is the sole
recovery actor and returns its retained terminal response after completion.
For the Tekes product actor, `install-or-upgrade` closes the internal durable
transaction after the service has been bootstrapped, then returns `running`.
The separate `ensure-running` operation owns the public ready-build
postcondition. It is idempotent for an already-ready selected build and does
not bootout, terminate or restart that build; a loaded but inactive job may be
started only by non-terminating kickstart. This ordering is required because
selector intentionally parks
while an installer operation is incomplete; waiting for HTTP readiness before
closing the transaction would deadlock. `disable` does not return until both
launchd ownership and the endpoint readiness socket are absent, and `enable`
does not return until the selected bundled build is ready.
The Installer directory and closed operation remain after uninstall so retry
is defined; a subsequent fresh install atomically replaces that closed record,
publishes a new immutable InstallIdentity and generates a fresh credential.

Uninstall first drains/stops and unloads launchd, deletes the exact endpoint
Keychain item, then removes installed binaries/plist under this transaction.
Authoritative user data, config, retained provider secrets, and their
`credential-state/provider-secret-generations.json` anti-rollback authority,
and archives are retained by default. Removing an exact provider item does not
remove its floor. A reinstall under the same immutable team/access group
continues those floors. Destructive removal of the complete provider-security
state is a separate explicit installer operation that names the exact validated
storage root, removes all provider items and referring config before floors,
and is crash-recoverable; Kernel never deletes a floor merely because a config
reference disappeared. Team migration likewise must explicitly migrate or
purge the old complete provider-security state before publishing the new
install identity.

## Observability and privacy

Structured operational records contain timestamp, severity, component,
build/run attribution, stable error code, and correlation ids. They may contain
session UUID and event seq but never prompt/tool content, provider bodies,
credentials, authorization headers, raw hook/helper payloads, or asset bytes.
High-cardinality provider request ids are diagnostic fields, not metric labels.

Required metrics cover readiness, active workers, parked/running/settled counts,
lease utilization, append/barrier latency, tail repair/corruption, provider
classification, tool backend outcome, endpoint request/stream pressure, sweep
duration, and restart/rollback count. Metrics and logs are projections and
cannot drive recovery decisions. Rotation is bounded and does not touch thread
folders.

Every fail-stop condition surfaces a stable code through structured log and
either bootstrap status/selector `status` before listener bind or
`/health/ready` after bind; selector entry failures also use the closed stderr
envelope. Secrets are redacted before formatting. A support bundle contains
versions, capability/config digests, counters and redacted logs only.

Operational log files are canonical JSONL. Every line has the closed schema:

```text
LogRecord = {v:1,ts:RFC3339Nano,severity:"debug"|"info"|"warn"|"error",
  component:"installer"|"supervisor"|"worker"|"helper"|"selector"|"transport",
  build:str,code:str,message:str,correlation:{request_id?:str,run?:str,
  session_id?:UUID,event_seq?:int,launch_id?:str,attempt?:int,
  generation?:int,manifest_sha256?:lowercase-64-hex,operation_id?:str},
  fields:object}
```

`fields` contains scalar string/bool/safe-int values only and its keys come
from the closed per-code fixture registry; arbitrary payloads are forbidden.
For supervisor/worker/helper/transport records and selector records concerning
a child, `build` is the selected bundle version and correlation MUST contain
the complete frozen `(launch_id,attempt,generation,manifest_sha256)` tuple.
Selector lifecycle records unrelated to a child use its own selector version as
`build`; installer records use installer version, and both carry
`operation_id` when an operation exists. Launch ids are globally unique within
one installation: `generation-attempt-128randombits`, with generation/attempt
decimal and randomness lowercase hex. They are never reset or reused after
selector restart. `rollback-complete` binds the automatic/CLI selector
operation id plus the failed child tuple. The observability checker requires
the byte fixture to contain every code claimed by the redaction case, not only
a registry entry.

The active `supervisor.jsonl` operational log and every one of its retained
generations are owner-only regular files. The supervisor opens them with
`O_NOFOLLOW`, verifies effective owner, mode `0600`, and descriptor/path inode
identity, and serializes the complete rotate-plus-append operation within the
process. A failed supervisor-log append is never ignored: the endpoint becomes
not-ready immediately and stderr receives the exact content-free fallback record
`{"code":"operational-log-unavailable","format":1,"message":"Operational log append failed"}`.
The fallback is a last-resort health signal, not a `LogRecord` and never enters
the retained log or recovery truth.
A later successful canonical append clears this transient fault and restores
ready only while the host is not draining; drain remains the higher authority.

The selector's closed code/field rows are:

| code | severity | exact `fields` keys | correlation |
|---|---|---|---|
| `selector-predecessor-draining` | warn | `root_lock` | lifecycle; empty |
| `orphan-owner-timeout` | error | `deadline_ms,generation,manifest_sha256,root_lock,version` | lifecycle; empty |
| `selector-prelaunch-cleared` | info | `root_lock` | lifecycle; empty |
| `selector-recovered` | info | `operation,phase` | lifecycle; exact selector operation id |
| `selector-update-complete` | info | `sha256,version` | lifecycle; empty (the update protocol has no operation record) |
| `selector-child-launch` | info | `canary_required` | complete child tuple |
| each closed attributable/environment code and `launcher-interrupted` | error for attributable, warn otherwise | `classification` | complete child tuple |
| `readiness-crash-loop` | error | `classification` | complete threshold-crossing child tuple |
| `promotion-complete` | info | `from_state` | complete child tuple |
| `rollback-complete` | warn | `from,reason,to` | complete failed-child tuple plus `operation_id` |

Selector appends each canonical line with `O_APPEND`, mode 0600 and a successful
full sync before treating the record as published. It rotates before an append
that would exceed 10 MiB, retains `.1` through `.5`, and directory-syncs the
rename set. The two prelaunch lifecycle rows intentionally have no child tuple:
no supervisor launch exists yet. Their selected-build facts live only in their
closed scalar fields, while `build` is the selector's own version.
Files are 0600, rotate at 10 MiB, retain five generations, and never rotate a
thread-folder file.

Metrics are canonical JSON plus LF:

```text
MetricSnapshot = {v:1,ts:RFC3339Nano,metrics:[Metric,...]}
Metric = {name:str,type:"gauge"|"counter"|"histogram",
  value:nonnegative-number,labels:{component?:str,classification?:str}}
```

Names are the closed set `readiness`, `active_workers`, `threads_parked`,
`threads_running`, `threads_settled`, `provider_leases`, `append_latency_ms`,
`barrier_latency_ms`, `tail_repairs_total`, `corruptions_total`,
`provider_outcomes_total`, `tool_outcomes_total`, `endpoint_requests_total`,
`endpoint_stream_pressure`, `sweep_duration_ms`, `restarts_total`, and
`rollbacks_total`. Session/event/request ids, provider/model/tool name and free
text are forbidden labels.

A support bundle is a directory published by temp-directory rename and sync,
containing exactly `manifest.canonical.json`, `logs.jsonl` and
`metrics.canonical.json`, all 0600. Its manifest is canonical JSON plus LF:

```text
SupportManifest = {format:1,created_at:RFC3339Nano,build:str,
  capability_digest:hex,config_digests:[hex,...],files:[
  {path:"logs.jsonl"|"metrics.canonical.json",bytes:int,sha256:hex}]}
```

File entries are path-sorted. Logs are revalidated/redacted records, metrics
use the schema above, and no thread/config/asset/credential file is copied.
The observability fixture owns allowed field keys and forbidden canary strings;
generation fails closed if any forbidden bytes remain.

The production entry point is the installed supervisor app executable, invoked only
after the LaunchAgent has been booted out and the supervisor/root lock owner is
gone:

```text
TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor
  support-bundle --destination <new-absolute-directory>
  --log-root <absolute-USER_HOME/.agents/logs/kernel>
  --build <selected-build> --capability-digest <lowercase-64-hex>
  --config-digests <lowercase-64-hex>[,<lowercase-64-hex>...]
```

The destination MUST be absent. The command reads only the closed selector and
supervisor log generations plus the last canonical metric snapshot, validates
every input byte against the schemas above, and then uses the publication
protocol above. Missing log generations are empty history; a missing,
noncanonical or incomplete metric snapshot and any invalid/secret-bearing log
line fail closed without publishing the destination. Requiring an offline
owner makes the selected JSONL prefix and metric snapshot stable without a
second daemon-control protocol.

## Production acceptance

`fixtures/deployment/cases.canonical.json` names the exact selector commands,
preconditions, fault points and expected filesystem/readiness results for the
installation and fault matrix. Slice 10 requires clean-machine install, reboot start, duplicate
instance, unsupported filesystem, corrupt tail, locked config, drain with live
provider/tool/stream work, automatic crash-loop rollback, power loss at every
stage/first-activation/upgrade publication boundary, stable-selector update
power loss, install/credential-rotation/uninstall transaction recovery, archive
preservation, upgrade/downgrade gate, promotion boundary, log attribution/
redaction, and uninstall-with-data-retained cases.

For each registry case, setup/steps/expected are RFC-8785 canonical JSON with
no trailing LF and closed schemas:

```text
Setup = {format:1,case:str,gate:int,platform:{os:"macos",minimum:"15.0",
  architecture:"aarch64",filesystem:str},initial:{current?:str,previous?:str,
  generation:int,service:"absent"|"stopped"|"running",threads:[str,...],
  observation?:"candidate-pending"|"rollback-window"|"promoted"}}
Steps = {command:[[str,...],...],fault_points:[str,...],expected_exit:[int,...],
  expected_selection:{current?:str,previous?:str,generation:int},
  expected_readiness:{ready:bool,code?:str}}
Expected = {format:1,case:str,gate:int,selection:{current?:str,previous?:str,
  generation:int},readiness:{ready:bool,code?:str},
  filesystem:{present:[str,...],absent:[str,...],modes:{path:octal,...}},
  processes:{selectors:int,supervisors:int,listeners:int,orphans:int},
  preserved:[str,...],observability:{codes:[str,...],forbidden:[str,...]}}
```

Lists are order-significant and duplicate-free; paths are fixture-root-relative
unless prefixed `USER_HOME/`. Command argv are exact selector, service,
launchctl, Client or deployment-harness actions, never shell strings.
Absent `initial.observation` means no observation file exists; it is never an
implicit promoted or rollback-eligible state. Crash-loop rollback starts in
`rollback-window`; ordinary reboot followed by target-user login, without
opening the Tekes GUI, starts `promoted`.
`expected_exit` aligns one-to-one with `command`. Every declared fault point
names an exact protocol boundary or precondition from this contract. The
readiness outcome names the resulting service state: a false pre-listener
startup code must be obtained from selector status and must not be paired with
an HTTP GET; post-uninstall `service-absent` is established by the installation
harness. HTTP 503 codes come only from the readiness oracle. The
deployment checker validates registry/disk parity, schemas, canonical bytes,
selector transitions, bootstrap/readiness rows, full installed-layout parity,
gate coverage and the separate plist, bundle, selector-update, log, metric and
support-bundle oracles in both directions.

Production-ready means the Slice 9 endpoint is exercised by the real Tekes
Client `.tekes` driver against fixed origin `http://127.0.0.1:7347` and the
launchd-owned service, all Slice 1–10 gates pass, the release
bundle is signed/notarizable for its distribution channel, and local `main`
matches the merged remote evidence commit. It does not mean every optional
provider, plugin, MCP server, or remote-deployment capability is advertised.

The Gate-72/76 coordinator is the repository-built signed native application
described by `packaging/macos/PRODUCTION-UAT.md`. Its protocol is necessarily
two-stage. `prepare` binds the host, boot-session UUID, operation UUID, release,
selector conformance and every input byte digest; installs the production
service; SIGKILLs the launchd-owned selector and records the different
replacement PID/launch id; then installs a bounded-retry Aqua resume LaunchAgent.
It never calls reboot and returns `reboot-required`. After an external real
reboot and target-user login, only the signed `resume` runner may authenticate
the operation secret, require a different boot-session UUID, prove launchd
selector/supervisor cardinality and root-lock ownership, execute the Client
observations, and publish HMAC-bound evidence. Ordinary CI may only `verify`
or idempotent `consume`; consume performs uninstall before setting the final
Gate-76 `uninstall:true`. Wrong host/build/input, same-boot resume and missing
signed evidence fail closed; after acknowledgement, only the cleanup tombstone
may be replayed and it is not gate evidence.

Each gate uses a fresh dedicated account with no preexisting Tekes application
support root. Before install mutation, prepare publishes one atomic
HMAC-authenticated request/state record. The operation identity is deterministic
over host and frozen inputs while the request separately binds the initial boot
session, so an exact retry adopts or rolls
back only its own empty-root partial install. Resume and consume use the closed
phase lists in the Production UAT fixture; Client session creation receives the
operation UUID as its idempotency key, provider generations use compare/adopt
transitions, and uninstall bootout/deletes are repeatable. Consume retains the
operation key and canonical result until the release controller archives it and
calls `acknowledge`; only acknowledgement deletes the key, with a durable
non-authoritative tombstone making a lost acknowledgement response repeatable.

The selector witness binds the old supervisor and its pre-kill descendant set,
requires all of them to exit, and attributes the replacement listener and busy
root lock to the new selector's sole supervisor child. Gate 76 binds the exact
session active/archive file inventory and SHA-256 digest before uninstall and
re-derives it afterward. Commands are bounded by timeout and output size and
reap their process group; the Aqua job has a five-attempt durable
retry budget. None of these coordinator-owned facts may be supplied by Client.

The Client UAT owns only endpoint-observable facts. `post-reboot` returns
`.tekes` driver and endpoint readiness; `session` returns a session UUID plus
archive/unarchive/retention facts, never a ledger-local run id. The coordinator
derives the unique settled run from that session's canonical `main.jsonl`
before selector attestation. Provider phases require that session UUID and use
`session.models` to observe active/rotated readiness and typed revoked
not-readiness. Process, lock, selector-crash and secret-byte-scan facts remain
coordinator-owned. The clean UAT install writes canonical provider/settings/
workspace authority for `provider-uat`/`model-uat`; live HTTP material switching
remains Gate 56, not a Client self-report.

Before its first mutation, prepare validates the external Client application
container and CMS-decodes its embedded provisioning profile. Expiry,
application/team identifier, or endpoint access-group authorization mismatch is
fatal. It then runs the signed Client's `--describe-contract` admission probe
with the bounded empty-environment command runner and requires the fixture's
exact canonical protocol/identifier/release bytes with no stderr. Prepare
prints its canonical reboot-required record and exits 75; orchestration accepts
neither status 0 nor status 75 paired with malformed bytes.

Raw Cargo outputs are never production inputs. The repository signing builder
copies them into a new path below the explicit trusted artifact root (repository
`target/` by default), wraps the entitled supervisor in an embedded-profile
app, applies fixed identifiers and only the supervisor endpoint/provider
entitlements, executes its build probe, then runs `verify-release --codesign`
both before and after atomic publication. A Finder/File Provider-managed
checkout must select a local-APFS artifact root outside the managed tree;
post-publication metadata that invalidates `codesign --verify --strict` is a
failed build, never a successful release. The coordinator is likewise an
embedded-profile app and has an exclusive production-UAT Keychain
group for evidence authentication. The exact prepare/resume/verify/consume/
acknowledge and
Client argv/results, plus the supervisor's nine option/value pairs including
`--launch-id`, and the Client session-phase mutation identities are frozen by
`fixtures/deployment/production-uat-contract.canonical.json`. The operation
UUID is the `session.create` RPC id; workspace, prompt, archive and unarchive
use exactly `uat-<operation>-workspace`, `uat-<operation>-prompt`,
`uat-<operation>-archive` and `uat-<operation>-unarchive`. Re-entry after an
accepted effect reuses those bytes and reconciles authoritative inventory
before the next mutation.
