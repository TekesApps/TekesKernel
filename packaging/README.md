# Slice 10 macOS packaging

`macos/render-launch-agent.py` is the only plist renderer. It substitutes the
five deployment tokens, rejects ambient/relative paths and atomically
publishes a mode-0644 LaunchAgent with the fixed selector `serve` argv.

`macos/assemble-bundle.py` emits the canonical-plus-LF manifest only after the
four exact release executables (Supervisor, Worker, tool helper and workspace
service) exist as regular mode-0755 files. The registry
digest covers its exact canonical bytes.

`macos/verify-release.py` checks a staged bundle or stable-selector update
against canonical-plus-LF manifests, exact file sets, bytes, SHA-256, mode,
platform, immutable install identity and authority-registry compatibility.
Release invocations pass `--codesign`; tests intentionally omit it only for
synthetic executable bytes. This verifier does not activate a bundle, mutate
selector state, touch Keychain, or call launchctl. Those authorities remain in
the signed installer and external `tekes-selector` transaction paths.

Selector verification additionally requires the canonical evidence emitted by
`macos/assemble-selector-conformance.py`; its SHA-256 must equal the signed
selector manifest and the digest embedded at selector compile time.
`macos/probe-supervisor-bootstrap.py` launches the actual installed supervisor
with the contract fd3/fd4 handoff and rejects embedded build/version mismatch.
`macos/build-signed-release.sh` takes that release version explicitly and
requires the signed supervisor's canonical `--describe-build` response to bind
both the same version and the exact authority-registry digest before publish.
It also runs the Web Client source-manifest checker and requires the signed
supervisor's `--describe-web-client` digest to match the separately signed
`Contents/Resources/WebClientManifest.canonical.json` resource. This keeps stale embedded
browser bytes from entering an otherwise valid release bundle.
Both signed builders publish only below `TEKES_SIGNED_ARTIFACT_ROOT` (default:
repository `target/`) and repeat signature/byte verification at the final
path. Set the root to an explicit local-APFS directory outside the checkout
when Finder/File Provider manages the repository; otherwise FinderInfo added
after signing correctly makes the build fail.

`macos/build-product-installer.sh` builds the separately signed
`com.tekes.kernel.installer` application from the Rust `tekes-kernel-installer`
crate. The signing team is bound at compile time by `TEKES_INSTALLER_TEAM_ID`;
the app identifier, entitlements and installer protocol remain unchanged.
Its production executable admits
only the closed `tekes-kernel-product-installer` operations `status`,
`install-or-upgrade`, `ensure-running`, `enable`, `disable`, and
`migrate-provider-credential`; the
side-effect-free `--describe-contract` probe is the only other entry point.
The installer alone owns the installation-scoped endpoint bearer, stable
install root, atomic deployment operation, LaunchAgent publication and
bootstrap/bootout. Tekes.app launches this actor with an empty environment and
does not write those authorities directly.

`macos/assemble-tekes-product.py` combines that signed installer with one
already verified deployment release. It rejects symlinks and extra release
files, re-verifies every signature/manifest/entitlement, emits the canonical
product manifest with SHA-256 bindings, optionally invokes the sibling Tekes
checker, and atomically publishes the wrapper. A typical release invocation is:

```text
TEKES_SIGNED_ARTIFACT_ROOT=/absolute/local-apfs/artifacts \
packaging/macos/build-product-installer.sh \
  --identity SIGNING_IDENTITY --team-id TEAMID \
  --profile ABSOLUTE_INSTALLER_PROFILE \
  --output /absolute/local-apfs/artifacts/TekesKernelInstaller.app

packaging/macos/assemble-tekes-product.py \
  --release-root /absolute/local-apfs/artifacts/SIGNED_DEPLOYMENT_V1 \
  --installer-app /absolute/local-apfs/artifacts/TekesKernelInstaller.app \
  --selector-conformance /absolute/selector-conformance.canonical.json \
  --artifact-root /absolute/local-apfs/artifacts \
  --output /absolute/local-apfs/artifacts/TekesKernelProduct \
  --tekes-checker /absolute/Tekes/scripts/check-bundled-kernel-product.py
```

Run `macos/test-product-installer.sh --tekes-repo /absolute/Tekes` for the
closed CLI, malicious-layout, canonical-contract and cross-repository byte-sync
checks. This product path is not the dedicated-account/reboot Gate-72/76
coordinator described below.

The production acceptance lane must additionally run Gate 76 on macOS 15+
local APFS with signed installer, Client and selector inputs. The portable CI
lane verifies the complete install/update/UAT oracle but is not a substitute
for that human-owned signed release environment.

### Rust installer and platform support

`crates/product-installer` replaces the Swift product installer, including its
legacy-data migration and service launch policy tests. The existing build and
focused-test script paths remain the integration entry points. Build artifacts
use a separate Cargo target directory for each signing team.

Durable writes, install receipts, recovery, artifact digest validation and
legacy-data migration are Rust modules. `src/platform` contains the macOS
service manager, signed-caller/artifact verification and Security.framework
Keychain adapters. The macOS layout and artifact schema are still the existing
product contract; they are not Linux defaults.

The installer type-checks for `x86_64-unknown-linux-gnu`, but Linux installation
returns `platform-unavailable` before reading an artifact or mutating state.
A Linux release still needs its own service manager, credential/trust policy,
layout and artifact packaging, followed by runtime acceptance. This migration
does not claim Linux deployment support. The separate signed macOS production
UAT harness under `macos/production-uat` remains Swift; it is not linked into the
installer or required to compile the Rust crate.

Validation commands:

```sh
bash packaging/macos/test-product-installer.sh --tekes-repo /absolute/Tekes
cargo check --locked -p tekes-kernel-installer --target x86_64-unknown-linux-gnu
```

The focused suite covers journal recovery and tampering, artifact digest and
team binding, install-lock contention, selection rollback, command contracts,
and byte-for-byte LaunchAgent compatibility. Signed application assembly and
live install/upgrade/disable/enable, caller validation and Keychain access must
still be exercised with a provisioned release and its signed Tekes client.
