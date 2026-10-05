#!/bin/bash
set -euo pipefail

kernel_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$kernel_root"

if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  if [[ "$CARGO_TARGET_DIR" = /* ]]; then
    kernel_target_dir="$CARGO_TARGET_DIR"
  else
    kernel_target_dir="$kernel_root/$CARGO_TARGET_DIR"
  fi
else
  kernel_target_dir="$kernel_root/target"
fi
export CARGO_TARGET_DIR="$kernel_target_dir"
# Signed artifacts use an explicit, trusted root independent of Cargo's cache.
# The default remains repository target/ for ordinary CI. A checkout managed
# by Finder/File Provider must set this to a local APFS directory outside that
# managed tree, because FinderInfo attached after signing invalidates the app.
: "${TEKES_SLICE10_ARTIFACT_ROOT:=$kernel_root/target/slice10}"
[[ "$TEKES_SLICE10_ARTIFACT_ROOT" = /* ]] || {
  echo "TEKES_SLICE10_ARTIFACT_ROOT must be absolute" >&2
  exit 64
}
kernel_release_dir="$TEKES_SLICE10_ARTIFACT_ROOT"
export TEKES_SIGNED_ARTIFACT_ROOT="$kernel_release_dir"

: "${TEKES_KERNEL_FIXTURES:=$kernel_root/fixtures}"
export TEKES_KERNEL_FIXTURES

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "Slice 10 release gates require macOS 15+; refusing a portable substitute" >&2
  exit 1
fi
macos_major="$(sw_vers -productVersion | cut -d. -f1)"
if [[ "$macos_major" -lt 15 ]]; then
  echo "Slice 10 release gates require macOS 15+" >&2
  exit 1
fi
workspace_device="$(df -P "$kernel_root" | awk 'END {print $1}')"
if ! diskutil info "$workspace_device" | grep -Eq 'File System Personality:[[:space:]]+APFS'; then
  echo "Slice 10 release gates require a local APFS workspace" >&2
  exit 1
fi

run_gate() {
  python3 - "$@" <<'PY'
import os
import signal
import subprocess
import sys

command = sys.argv[1:]
process = subprocess.Popen(command, start_new_session=True)
try:
    status = process.wait(timeout=600)
except subprocess.TimeoutExpired:
    os.killpg(process.pid, signal.SIGTERM)
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait()
    print(f"Slice 10 gate timed out after 600 seconds: {command}", file=sys.stderr)
    raise SystemExit(124)
raise SystemExit(status)
PY
}

run_production_gate() {
  local gate="$1"
  shift
  python3 - "$gate" "$@" <<'PY'
import json
import os
import signal
import subprocess
import sys

gate = int(sys.argv[1])
command = sys.argv[2:]
process = subprocess.Popen(
    command,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    start_new_session=True,
)
try:
    stdout, stderr = process.communicate(timeout=600)
except subprocess.TimeoutExpired:
    os.killpg(process.pid, signal.SIGTERM)
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait()
    print(f"Slice 10 production Gate {gate} timed out", file=sys.stderr)
    raise SystemExit(124)
if process.returncode != 0:
    sys.stderr.buffer.write(stderr)
    raise SystemExit(process.returncode)
if stderr:
    print(f"Slice 10 production Gate {gate} emitted unexpected stderr", file=sys.stderr)
    sys.stderr.buffer.write(stderr)
    raise SystemExit(1)
expected = {
    72: {
        "format": 1,
        "gate": 72,
        "embedded_build": "1.0.0",
        "launchd_restart": True,
        "nested_threads_absent": True,
        "no_orphans": True,
        "root_lock_released": True,
        "selector_sigkill": True,
        "target_user_login": True,
    },
    76: {
        "archive": True,
        "canary_attested": True,
        "client_driver": ".tekes",
        "credential_acl": True,
        "embedded_build": "1.0.0",
        "format": 1,
        "gate": 76,
        "install": True,
        "nested_threads_absent": True,
        "origin": "http://127.0.0.1:7347",
        "provider_secret_acl": True,
        "provider_secret_redacted": True,
        "provider_secret_revocation": True,
        "provider_secret_rotation": True,
        "reboot_after_target_user_login": True,
        "retained_data": True,
        "unarchive": True,
        "uninstall": True,
    },
}[gate]
canonical = json.dumps(expected, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode() + b"\n"
if stdout != canonical:
    print(f"Slice 10 production Gate {gate} evidence differs from the closed oracle", file=sys.stderr)
    sys.stderr.buffer.write(stdout)
    raise SystemExit(1)
sys.stdout.buffer.write(stdout)
PY
}

run_production_prepare() {
  local gate="$1"
  shift
  python3 packaging/macos/check-production-prepare.py "$gate" "$@"
}

run_production_verify() {
  local gate="$1" operation="$2" runner="$3"
  python3 - "$gate" "$operation" "$runner" <<'PY'
import json, subprocess, sys
gate, operation, runner = int(sys.argv[1]), sys.argv[2], sys.argv[3]
result = subprocess.run([runner, "--mode", "verify", "--operation", operation], capture_output=True, timeout=600)
if result.returncode != 0 or result.stderr:
    sys.stderr.buffer.write(result.stderr)
    raise SystemExit(result.returncode or 1)
value = json.loads(result.stdout)
canonical = json.dumps(value, sort_keys=True, separators=(",", ":")).encode() + b"\n"
if result.stdout != canonical or set(value) != {"format", "gate", "operation", "phase", "request_sha256"} or value["format"] != 1 or value["gate"] != gate or value["operation"] != operation or value["phase"] != "verified" or len(value["request_sha256"]) != 64:
    raise SystemExit("invalid production verification evidence")
sys.stdout.buffer.write(result.stdout)
PY
}

run_production_acknowledge() {
  local operation="$1" runner="$2"
  python3 - "$operation" "$runner" <<'PY'
import json, subprocess, sys
operation, runner = sys.argv[1], sys.argv[2]
result = subprocess.run([runner, "--mode", "acknowledge", "--operation", operation], capture_output=True, timeout=600)
if result.returncode != 0 or result.stderr:
    sys.stderr.buffer.write(result.stderr)
    raise SystemExit(result.returncode or 1)
value = json.loads(result.stdout)
canonical = json.dumps(value, sort_keys=True, separators=(",", ":")).encode() + b"\n"
if result.stdout != canonical or set(value) != {"format", "operation", "phase", "receipt_sha256"} or value["format"] != 1 or value["operation"] != operation or value["phase"] != "acknowledged" or len(value["receipt_sha256"]) != 64:
    raise SystemExit("invalid production acknowledgement")
sys.stdout.buffer.write(result.stdout)
PY
}

production_action="${TEKES_SLICE10_PRODUCTION_ACTION:-prepare}"
if [[ "$production_action" == verify || "$production_action" == consume || "$production_action" == acknowledge ]]; then
  : "${TEKES_SLICE10_PRODUCTION_OPERATION:?set the prepared operation UUID}"
  : "${TEKES_SLICE10_PRODUCTION_GATE:?set 72 or 76}"
  : "${TEKES_SLICE10_PRODUCTION_UAT:=$HOME/Library/Application Support/Tekes/ProductionUAT/$TEKES_SLICE10_PRODUCTION_OPERATION/TekesProductionUAT.app/Contents/MacOS/tekes-production-uat}"
  [[ "$TEKES_SLICE10_PRODUCTION_GATE" == 72 || "$TEKES_SLICE10_PRODUCTION_GATE" == 76 ]] || exit 64
  if [[ "$production_action" == verify ]]; then
    run_production_verify "$TEKES_SLICE10_PRODUCTION_GATE" "$TEKES_SLICE10_PRODUCTION_OPERATION" "$TEKES_SLICE10_PRODUCTION_UAT"
  elif [[ "$production_action" == consume ]]; then
    run_production_gate "$TEKES_SLICE10_PRODUCTION_GATE" "$TEKES_SLICE10_PRODUCTION_UAT" \
      --mode consume --operation "$TEKES_SLICE10_PRODUCTION_OPERATION"
  else
    run_production_acknowledge "$TEKES_SLICE10_PRODUCTION_OPERATION" "$TEKES_SLICE10_PRODUCTION_UAT"
  fi
  exit 0
fi
[[ "$production_action" == prepare || "$production_action" == preflight ]] || {
  echo "TEKES_SLICE10_PRODUCTION_ACTION must be preflight, prepare, verify, consume, or acknowledge" >&2
  exit 64
}

# Earlier gates and the real sibling Client transport/archive evidence remain
# prerequisites. That Client run is also one half of Gate 76's combined proof;
# the deployment installer half runs below against the built Slice-10 artifacts.
scripts/ci-slice9.sh

python3 scripts/check-deployment-fixtures.py
python3 scripts/check-secret-store-fixtures.py
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build --workspace --all-targets --locked

export TEKES_SLICE10_SELECTOR_BIN="$kernel_target_dir/debug/tekes-selector"
export TEKES_SLICE10_SUPERVISOR_BIN="$kernel_target_dir/debug/tekes-supervisor"
export TEKES_SLICE10_WORKER_BIN="$kernel_target_dir/debug/tekes-worker"
export TEKES_SLICE10_HELPER_BIN="$kernel_target_dir/debug/tekes-helper"
for artifact in \
  "$TEKES_SLICE10_SELECTOR_BIN" \
  "$TEKES_SLICE10_SUPERVISOR_BIN" \
  "$TEKES_SLICE10_WORKER_BIN" \
  "$TEKES_SLICE10_HELPER_BIN"
do
  [[ -x "$artifact" ]] || {
    echo "missing built Slice 10 artifact: $artifact" >&2
    exit 1
  }
done

# Portable byte/schema tests are supporting evidence only, never ◆◆ gates.
cargo test -p conformance --locked --test slice10_gates
cargo test -p deployment-tests --locked --test portable

run_gate cargo test -p tekes-supervisor --locked --test slice10_daemon \
  slice10_gate_71_deployment_filesystem_and_ownership -- --exact
run_gate cargo test -p tekes-supervisor --locked --test slice10_daemon \
  slice10_gate_72_launchd_crash_recovery -- --exact
run_gate cargo test -p tekes-selector --locked --test slice10_selector \
  slice10_gate_73_install_upgrade_publication_crash_matrix -- --exact
run_gate cargo test -p tekes-selector --locked --test slice10_selector \
  slice10_gate_74_crash_loop_external_rollback -- --exact
# Gate 75 is intentionally a composition of production paths, not a test that
# fabricates operational records. Gate 74 above supplies the real selector
# rollback/ready-error log path; the commands below add semantic projection,
# corruption sweep, durable credential rotation, actual HTTP rejection, and
# the production support-bundle executable. Keychain ACL/no-leak evidence is
# owned by the signed Gate-76 continuation below.
run_gate cargo test -p tekes-supervisor --locked --lib \
  process_host::tests::production_observability_projection_and_corruption_are_inert_and_redacted -- --exact
run_gate cargo test -p tekes-supervisor --locked --lib \
  secret_authority::tests::rotation_and_revocation_advance_floor_before_restart -- --exact
run_gate cargo test -p transport --locked --test slice9_transport \
  slice9_gate_66_endpoint_http_error_and_idempotency -- --exact
run_gate cargo test -p tekes-supervisor --locked --test slice10_daemon \
  slice10_support_bundle_command_and_schema -- --exact

# The bundle selection version is an immutable build input, not the Cargo
# package version. Rebuild the installed artifacts after the ordinary test
# binaries so the supervisor's embedded attribution matches the manifest.
export TEKES_SLICE10_RELEASE_VERSION=1.0.0
selector_conformance="$kernel_release_dir/selector-conformance-$PPID-$$.canonical.json"
python3 packaging/macos/assemble-selector-conformance.py \
  --workspace "$kernel_root" \
  --output "$selector_conformance" \
  --release-version "$TEKES_SLICE10_RELEASE_VERSION"
export TEKES_SELECTOR_CONFORMANCE_SHA256
TEKES_SELECTOR_CONFORMANCE_SHA256="$(/usr/bin/shasum -a 256 "$selector_conformance" | awk '{print $1}')"
TEKES_SELECTED_BUILD="$TEKES_SLICE10_RELEASE_VERSION" \
  cargo build -p tekes-supervisor --bin tekes-supervisor --locked
TEKES_SELECTED_BUILD="$TEKES_SLICE10_RELEASE_VERSION" \
  cargo build -p tekes-worker --bin tekes-worker --locked
TEKES_SELECTED_BUILD="$TEKES_SLICE10_RELEASE_VERSION" \
  cargo build -p tools --bin tekes-helper --locked
cargo build -p workspace-service --bin tekes-workspace-service --locked
TEKES_SELECTOR_CONFORMANCE_SHA256="$TEKES_SELECTOR_CONFORMANCE_SHA256" \
  cargo build -p tekes-selector --bin tekes-selector --locked
# This supporting harness drives the installer transaction against the actual
# four built artifacts, but deliberately is not named Gate 76: a filesystem
# credential/service marker cannot substitute for signed Keychain+launchd UAT.
run_gate cargo test -p deployment-tests --locked --test gate76_installer \
  slice10_installer_recovery_built_artifact_support -- --exact --ignored

# Build release-mode inputs, then copy/sign and verify the exact production
# bytes. Raw Cargo outputs are never passed to the production coordinator.
TEKES_SELECTED_BUILD="$TEKES_SLICE10_RELEASE_VERSION" \
TEKES_SELECTOR_CONFORMANCE_SHA256="$TEKES_SELECTOR_CONFORMANCE_SHA256" \
  cargo build --workspace --all-targets --release --locked
: "${TEKES_SLICE10_SIGNING_IDENTITY:?set the distribution signing identity}"
: "${TEKES_SLICE10_TEAM_ID:?set the ten-character signing team id}"
: "${TEKES_SLICE10_CLIENT_REQUIREMENT:?set the signed Client designated requirement}"
: "${TEKES_SLICE10_SUPERVISOR_PROFILE:?set the supervisor provisioning profile}"
: "${TEKES_SLICE10_INSTALLER_PROFILE:?set the installer provisioning profile}"
signed_release="$kernel_release_dir/signed-release-$PPID-$$"
packaging/macos/build-signed-release.sh \
  --identity "$TEKES_SLICE10_SIGNING_IDENTITY" \
  --team-id "$TEKES_SLICE10_TEAM_ID" \
  --release-version "$TEKES_SLICE10_RELEASE_VERSION" \
  --client-requirement "$TEKES_SLICE10_CLIENT_REQUIREMENT" \
  --supervisor-profile "$TEKES_SLICE10_SUPERVISOR_PROFILE" \
  --selector "$kernel_target_dir/release/tekes-selector" \
  --supervisor "$kernel_target_dir/release/tekes-supervisor" \
  --worker "$kernel_target_dir/release/tekes-worker" \
  --helper "$kernel_target_dir/release/tekes-helper" \
  --conformance "$selector_conformance" \
  --output "$signed_release" \
  --workspace-service "$kernel_target_dir/release/tekes-workspace-service"
export TEKES_SLICE10_SELECTOR_BIN="$signed_release/selector/bin/tekes-selector"
export TEKES_SLICE10_SUPERVISOR_APP="$signed_release/bundle/apps/TekesKernelSupervisor.app"
export TEKES_SLICE10_SUPERVISOR_BIN="$TEKES_SLICE10_SUPERVISOR_APP/Contents/MacOS/tekes-supervisor"
export TEKES_SLICE10_WORKER_BIN="$signed_release/bundle/bin/tekes-worker"
export TEKES_SLICE10_HELPER_BIN="$signed_release/bundle/bin/tekes-helper"

# Exercise the selector's real Darwin codesign/CMS/profile parser against the
# exact app that will be staged. Fake signature seams cannot satisfy this
# supporting production assertion.
TEKES_TEST_PROFILED_APP="$TEKES_SLICE10_SUPERVISOR_APP" \
TEKES_TEST_APP_REQUIREMENT='anchor apple generic and identifier com.tekes.kernel.supervisor' \
TEKES_TEST_APP_TEAM="$TEKES_SLICE10_TEAM_ID" \
TEKES_TEST_APP_IDENTIFIER=com.tekes.kernel.supervisor \
TEKES_TEST_APP_ACCESS_GROUPS="${TEKES_SLICE10_TEAM_ID}.com.tekes.shared.endpoint,${TEKES_SLICE10_TEAM_ID}.com.tekes.kernel.provider-secrets" \
  cargo test -p tekes-selector --locked --test macos_signature \
    provisioned_app_is_accepted_by_the_real_macos_verifier -- --exact --ignored

# ◆◆ Gates 72 and 76 require the distribution-signed production harness. It
# owns safe Keychain setup/cleanup, launchctl bootstrap/bootout and the real
# Tekes Client fixed-origin canary/archive/unarchive path. Absence is a red
# release gate, never an oracle-only skip.
: "${TEKES_SLICE10_PRODUCTION_UAT_APP:=$kernel_release_dir/TekesProductionUAT-$PPID-$$.app}"
packaging/macos/build-production-uat.sh \
  --identity "$TEKES_SLICE10_SIGNING_IDENTITY" \
  --team-id "$TEKES_SLICE10_TEAM_ID" \
  --client-requirement "$TEKES_SLICE10_CLIENT_REQUIREMENT" \
  --profile "$TEKES_SLICE10_INSTALLER_PROFILE" \
  --output "$TEKES_SLICE10_PRODUCTION_UAT_APP"
TEKES_SLICE10_PRODUCTION_UAT="$TEKES_SLICE10_PRODUCTION_UAT_APP/Contents/MacOS/tekes-production-uat"
[[ "$TEKES_SLICE10_PRODUCTION_UAT" = /* && -x "$TEKES_SLICE10_PRODUCTION_UAT" ]] || {
  echo "missing signed repository production UAT runner; build it with packaging/macos/build-production-uat.sh" >&2
  exit 1
}
: "${TEKES_SLICE10_CLIENT_UAT:?set TEKES_SLICE10_CLIENT_UAT to the absolute signed Tekes Client UAT app}"
[[ "$TEKES_SLICE10_CLIENT_UAT" = /* && -d "$TEKES_SLICE10_CLIENT_UAT" \
   && -x "$TEKES_SLICE10_CLIENT_UAT/Contents/MacOS/tekes-kernel-client-uat" ]] || {
  echo "TEKES_SLICE10_CLIENT_UAT must be an absolute signed app path" >&2
  exit 1
}
client_requirement_prefix='anchor apple generic and identifier '
client_identifier="${TEKES_SLICE10_CLIENT_REQUIREMENT#"$client_requirement_prefix"}"
[[ "$client_identifier" != "$TEKES_SLICE10_CLIENT_REQUIREMENT" && -n "$client_identifier" ]] || exit 64
python3 packaging/macos/verify-provisioning-profile.py \
  --profile "$TEKES_SLICE10_CLIENT_UAT/Contents/embedded.provisionprofile" \
  --team-id "$TEKES_SLICE10_TEAM_ID" \
  --identifier "$client_identifier" \
  --access-group "${TEKES_SLICE10_TEAM_ID}.com.tekes.shared.endpoint"
cargo test --workspace --all-targets --locked
if [[ "$production_action" == preflight ]]; then
  echo "Slice 10 signed preflight passed; external Gates 72/76 remain release-blocking" >&2
  exit 0
fi
: "${TEKES_SLICE10_PRODUCTION_GATE:?set the single prepare gate, 72 or 76}"
[[ "$TEKES_SLICE10_PRODUCTION_GATE" == 72 || "$TEKES_SLICE10_PRODUCTION_GATE" == 76 ]] || exit 64
run_production_prepare "$TEKES_SLICE10_PRODUCTION_GATE" "$TEKES_SLICE10_PRODUCTION_UAT" \
    --mode prepare \
    --gate "$TEKES_SLICE10_PRODUCTION_GATE" \
    --fixtures "$TEKES_KERNEL_FIXTURES/deployment" \
    --selector "$TEKES_SLICE10_SELECTOR_BIN" \
    --supervisor "$TEKES_SLICE10_SUPERVISOR_APP" \
    --worker "$TEKES_SLICE10_WORKER_BIN" \
    --helper "$TEKES_SLICE10_HELPER_BIN" \
    --client-uat "$TEKES_SLICE10_CLIENT_UAT" \
    --origin http://127.0.0.1:7347 \
    --release-version "$TEKES_SLICE10_RELEASE_VERSION" \
    --selector-conformance "$selector_conformance"
echo "production UAT prepared; a real reboot and target-user login are required before verify/consume" >&2
exit 75
