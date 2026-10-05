# Signed production UAT boundary

The ordinary Tekes product path is the signed actor built by
`build-product-installer.sh` and the wrapper produced by
`assemble-tekes-product.py`; it is documented in `packaging/README.md`. That
actor is what Tekes.app invokes for the closed five-operation lifecycle. It is
not this production-UAT coordinator and does not claim the dedicated-account,
reboot/login, crash-witness, provider-rotation, or uninstall evidence below.
Conversely, `TekesProductionUAT.app` is a release qualification harness and
must never be substituted for the product installer embedded in Tekes.

The repository owns fixture validation, package assembly/verification,
selector/supervisor process tests, the recoverable installer harness, and the
buildable production coordinator in `packaging/macos/production-uat/`. Two ◆◆
rows still require a clean production signing account, matching supervisor and
installer provisioning profiles, and a real signed Tekes Client UAT app.
`scripts/ci-slice10.sh` builds the coordinator as an `.app` and requires the
Client actor at the explicit `TEKES_SLICE10_CLIENT_UAT` app path. None has a
mock fallback.

The three signed app identities are exact:

| actor | application identifier | required Keychain access groups |
|---|---|---|
| supervisor | `TEAMID.com.tekes.kernel.supervisor` | `TEAMID.com.tekes.shared.endpoint`, `TEAMID.com.tekes.kernel.provider-secrets` |
| production coordinator | `TEAMID.com.tekes.kernel.installer` | the two groups above plus `TEAMID.com.tekes.kernel.production-uat` |
| Client UAT | `TEAMID.com.tekesapps.TekesUI` | `TEAMID.com.tekes.shared.endpoint` only |

Each embedded CMS profile must be unexpired and grant the app identifier exactly
or through `TEAMID.*`, and authorize those groups either
explicitly or through the team's wildcard grant. The effective signed
entitlements remain exact: a wildcard in the profile is not permission to add
another entitlement to the app. `fixtures/deployment/provisioning-profiles.canonical.json`
is the byte oracle for this table.

For local Tekes Xcode builds, `Debug Local Kernel` prepares these signing assets
through Xcode automatic signing using the selected `DEVELOPMENT_TEAM`. The user
must be signed into that team in Xcode. Each build lets Xcode refresh profiles;
profile/certificate/team changes invalidate the signed product cache. There is
no manual profile download or expiry-date setting for this workflow. Explicit
profile and identity build-setting overrides remain available for managed CI.

Build the coordinator with the distribution identity:

```text
packaging/macos/build-production-uat.sh \
  --identity SIGNING_IDENTITY --team-id TEAMID \
  --client-requirement 'anchor apple generic and identifier com.tekesapps.TekesUI' \
  --profile ABSOLUTE_INSTALLER_PROVISIONPROFILE \
  --output ABSOLUTE_REPO/target/production-uat/TekesProductionUAT.app
```

`TEKES_SIGNED_ARTIFACT_ROOT` is the trusted ancestor of `--output` and defaults
to the repository `target/`. In a Finder/File Provider-managed checkout it must
instead name a preselected local-APFS artifact directory outside the managed
tree. Both builders verify the published path, so metadata attached during
publication fails closed rather than leaving a nominally successful but
unlaunchable app.

`packaging/macos/build-signed-release.sh` separately accepts
`--release-version VERSION` and
`--supervisor-profile ABSOLUTE_SUPERVISOR_PROVISIONPROFILE`, copies the four
Cargo release outputs into a new normalized descendant of that artifact root,
wraps the supervisor in `TekesKernelSupervisor.app`, signs
selector/supervisor/worker/helper with fixed identifiers (only supervisor gets
endpoint/provider Keychain groups), assembles both manifests and runs
`verify-release --codesign`. Both build scripts reject an existing output,
repository root, `/`, or a `..` alias; neither deletes an arbitrary output.

The build generates, signs and verifies a native app with the endpoint,
provider-secret management and exclusive production-UAT evidence access groups
derived from `TEAMID`. It validates the embedded profile and executes the
app's side-effect-free contract probe, because a bare entitled Mach-O or a
`codesign --verify`-only artifact can still be rejected by macOS admission.
The source never embeds credentials. An unsigned compile may run only the
side-effect-free `--describe-contract` byte oracle; production argv verifies
its own signature and entitlements before touching Keychain, launchd or the
install layout.

Before prepare creates an operation Keychain item or writes any install/UAT
state, it CMS-decodes the external Client app's
`Contents/embedded.provisionprofile`, rejects expiry, application/team-id or
endpoint access-group mismatch, verifies the app signature and restricted
entitlements, and executes the Client's side-effect-free
`--describe-contract` admission probe. The probe must return exactly the
canonical protocol/identifier/release tuple frozen in the production-UAT
fixture with status 0 and empty stderr.

Prepare is invoked for exactly one gate with:

```text
TekesProductionUAT.app/Contents/MacOS/tekes-production-uat \
  --mode prepare --gate 72|76 --fixtures ABSOLUTE_DEPLOYMENT_FIXTURES \
  --selector ABSOLUTE_BUILT_SELECTOR --supervisor ABSOLUTE_SUPERVISOR_APP \
  --worker ABSOLUTE_BUILT_WORKER --helper ABSOLUTE_BUILT_HELPER \
  --client-uat ABSOLUTE_SIGNED_CLIENT_UAT_APP \
  --origin http://127.0.0.1:7347 --release-version 1.0.0 \
  --selector-conformance ABSOLUTE_CANONICAL_EVIDENCE
```

It must accept only those ordered arguments, enforce macOS 15+ local APFS and
run each Gate 72/76 operation in a fresh dedicated target-user account whose
entire `~/Library/Application Support/Tekes` install root and `~/.agents` data
root are absent. It refuses to
overwrite an existing install root, launchd label or endpoint
Keychain item. Signing identity, the exact supervisor/installer provisioning
profiles, notarization ticket and the real Client app build are runner-owned
inputs; secrets never enter argv, environment evidence, logs
or support bundles. Cleanup boots out the test job, reaps both process groups,
removes the exact test Keychain item and binaries, and preserves thread/archive
data. Prepare records a real selector SIGKILL/replacement witness, writes an
authenticated immutable operation request, and installs a bounded-retry Aqua
resume LaunchAgent. Before the first install side effect it publishes an atomic
HMAC-authenticated request and `preparing` phase. The operation UUID is a stable
function of host and frozen input digest while the request separately binds the
initial boot session, so repeating the exact prepare command after a crash or
unplanned reboot finds and re-drives only that owned clean-root operation; any
session/archive or unknown path makes pre-install recovery fail closed. It
outputs `reboot-required` but never reboots. An external
controller performs a real reboot. Target-user login automatically invokes
`--mode resume --operation UUID`; resume rejects the original boot session and
wrong host/build/input bytes, while exact retries continue from the last
authenticated phase. CI later invokes `verify`, then
`consume`; only consume uninstalls and may emit final Gate-76
`uninstall:true`. The HMAC key lives in an installer-only Keychain group, so an
ordinary CI process can verify/consume through the signed runner but cannot
author evidence.

The external release controller records the prepare JSON/operation UUID,
performs the hardware reboot, waits for target-user Aqua login, then invokes
`scripts/ci-slice10.sh` with `TEKES_SLICE10_PRODUCTION_ACTION=verify` and the
same `TEKES_SLICE10_PRODUCTION_OPERATION`/gate. After archiving that proof it
invokes idempotent `consume` and archives its canonical final result. It then
invokes `acknowledge`; acknowledgement writes a durable receipt tombstone and
deletes the operation Keychain secret. A lost acknowledgement response is
re-driven from that tombstone, but it is never release evidence. A prepare
invocation itself exits 75 after publishing its
canonical `reboot-required` record so it can never be mistaken for a completed
release job. `check-production-prepare.py` accepts only that exact status plus
the closed canonical evidence bytes; status 0 is a failure even when the bytes
look valid.

`TEKES_SLICE10_PRODUCTION_ACTION=preflight` is the non-mutating implementation
integration lane. It builds and verifies all signed actors and runs every
repository-owned gate, but it does not create an operation, touch the install
layout, switch users, or reboot. Its success never satisfies the external
reboot/login evidence required by Gates 72 and 76; those remain mandatory
before release.

The rendered LaunchAgent must pass the exact storage root
`USER_HOME/.agents/threads`. Both production runs
must reject a nested `threads/threads` directory before success. Archived data
lives in the sibling `USER_HOME/.agents/archive`, not
under the active threads root.

## Repository-owned versus runner-owned

This repository owns the closed fixture checker, canonical bundle assembly and
verification, LaunchAgent rendering, immutable embedded-build bootstrap probe,
offline selector evidence assembly/digest verification, process-group
timeout/reaping and the exact Gate 72/76 result validator. Those parts are
executable in `packaging/macos/` and `scripts/ci-slice10.sh` now.

The coordinator is the distribution-signed installer actor. Each command has a
60-second timeout, a 1 MiB stdout/stderr limit and process-group TERM/KILL
reaping. The Aqua resume job retries unsuccessful exits at most five times;
exhaustion leaves verification red and stops automatic retries. It uses
Security.framework in-process to create/replace/delete only the exact endpoint
and `provider-uat` fixture records, journals the installer phases, installs the
four signed, `verify-release --codesign`-accepted binaries, invokes only the
closed `launchctl` bootstrap/bootout
argv, and cleans up only those identities and paths. The injected real sibling
Tekes Client UAT app reads the endpoint item via `.tekes` and reports
only endpoint-observable facts after the signed resume runner has proved the
new boot/login session. An unsigned Kernel script cannot acquire those
entitlements, and the existing Slice-9 Client test injects a bearer directly,
so neither may be used as a green substitute.

## Signed Client UAT protocol

The coordinator invokes the injected executable with exactly; `OPERATION` is
also the Client's idempotency identity for session creation and every repeated
observation:

```text
CLIENT --protocol tekes-kernel-production-uat --gate 72|76 \
  --operation OPERATION \
  --phase PHASE --origin http://127.0.0.1:7347 \
  --release-version 1.0.0 --install-root ABSOLUTE_KERNEL_ROOT \
  --storage-root ABSOLUTE_THREADS_ROOT --selector ABSOLUTE_SELECTOR
```

`PHASE` is one of `post-reboot`, `session`, `provider-initial`,
`provider-rotated`, or `provider-revoked`. Provider phases append the required
`--session-id UUID`; other phases forbid it. Output is exactly one
canonical JSON object plus LF, stderr is empty, and every response contains
`{format:1,gate,phase}`. `post-reboot` reports only `.tekes` driver and endpoint
readiness. `session` performs discovery, canary create/settle, history/archive/
unarchive and returns `session_id`, never a ledger-local run id; the coordinator
derives the run from canonical `main.jsonl` before `attest-canary`. Provider
phases call `session.models` for the named session and report active/rotated
readiness or revoked typed not-readiness. Selector crash/PIDs, boot identity,
process cardinality, root lock and secret scans are coordinator-owned. Any
missing, extra, noncanonical, mismatched or false fact aborts.

The byte-level protocol oracle is
`fixtures/deployment/production-uat-contract.canonical.json`.

On success stdout is exactly one canonical JSON object plus LF and stderr is
empty. Gate 72 returns:

```json
{"embedded_build":"1.0.0","format":1,"gate":72,"launchd_restart":true,"nested_threads_absent":true,"no_orphans":true,"root_lock_released":true,"selector_sigkill":true,"target_user_login":true}
```

Gate 76 returns:

```json
{"archive":true,"canary_attested":true,"client_driver":".tekes","credential_acl":true,"embedded_build":"1.0.0","format":1,"gate":76,"install":true,"nested_threads_absent":true,"origin":"http://127.0.0.1:7347","provider_secret_acl":true,"provider_secret_redacted":true,"provider_secret_revocation":true,"provider_secret_rotation":true,"reboot_after_target_user_login":true,"retained_data":true,"unarchive":true,"uninstall":true}
```

Gate 76 must use the installed launchd-owned service for the complete run. The
Client must obtain the endpoint bearer through the `.tekes` driver/Keychain
path; feeding the token directly to `DSHHostEndpoint` is useful transport
evidence but is not this gate. It creates and settles the canonical disposable
canary, calls selector `attest-canary`, performs history/archive/unarchive, then
records the exact active/archive file inventory, byte lengths and SHA-256 digest
for the returned session, then uninstalls and re-derives the identical digest
before setting `uninstall:true`; directory existence is not evidence. It also
verifies absent service and credential.

Gate 72 records the old supervisor and its complete descendant PID set before
the selector SIGKILL. Prepare accepts the replacement only after that entire
set exits, a different selector owns a different supervisor, and that supervisor
owns the fixed listener while the root lock is held. A merely busy lock or a
new waiting child cannot produce green evidence.

The operation phase graph is frozen by the fixture. Prepare uses `preparing ->
installing -> crash-proved -> prepared`; resume journals process, Client,
session, attestation and each provider generation separately before `resumed`;
consume journals every bootout/delete/publication boundary before `consumed`.
Every side effect is either idempotent or adopted from its durable external
state on retry. HMAC payload and authenticator are one atomically published
canonical record, never two independently durable files.

The signed fixture installer also publishes one
`com.tekes.kernel.provider-secret` generic-password record under account
`provider-uat` and access group
`<InstallIdentity.team_id>.com.tekes.kernel.provider-secrets`. The signed
supervisor must report its configured provider ready, then observe an atomic
higher-generation rotation and revocation without placing sentinel material in
ledger, carrier, log, metric, bootstrap or support-bundle bytes. The Client,
selector, worker and helper must fail direct reads of this item. Cleanup
revokes and deletes only the fixture account; it never reuses or changes the
endpoint bearer item.

The existing Slice-9 Swift test is reusable for same-instance
create/history/archive/unarchive carrier work because its ready-file template
accepts an arbitrary origin. It does not cover `.tekes` discovery or Keychain
ACLs; that missing Client-side entrypoint/test belongs in the sibling Tekes
repository and is a genuine external prerequisite, not code that Kernel's
deployment harness can safely invent.
