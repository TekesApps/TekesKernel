#!/bin/bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"

usage() {
  echo 'usage: build-signed-release.sh --identity SIGNING_IDENTITY --team-id TEAMID --release-version VERSION --client-requirement REQUIREMENT --supervisor-profile ABSOLUTE_PROFILE --selector ABSOLUTE --supervisor ABSOLUTE --worker ABSOLUTE --helper ABSOLUTE --conformance ABSOLUTE --output ABSOLUTE --workspace-service ABSOLUTE --source-revision GIT_COMMIT' >&2
  exit 64
}

[[ "$#" -eq 26 ]] || usage
[[ "$1" == --identity && "$3" == --team-id && "$5" == --release-version \
   && "$7" == --client-requirement && "$9" == --supervisor-profile \
   && "${11}" == --selector && "${13}" == --supervisor \
   && "${15}" == --worker && "${17}" == --helper \
   && "${19}" == --conformance && "${21}" == --output && "${23}" == --workspace-service \
   && "${25}" == --source-revision ]] || usage
identity="$2"
team_id="$4"
release_version="$6"
client_requirement="$8"
supervisor_profile="${10}"
selector="${12}"
supervisor="${14}"
worker="${16}"
helper="${18}"
conformance="${20}"
output="${22}"
workspace_service="${24}"
source_revision="${26}"
artifact_root="${TEKES_SIGNED_ARTIFACT_ROOT:-$root/target}"
[[ "$team_id" =~ ^[A-Z0-9]{10}$ \
   && "$release_version" =~ ^[A-Za-z0-9_-][A-Za-z0-9._-]{0,127}$ \
   && "$source_revision" =~ ^[0-9a-f]{40}$ \
   && "$artifact_root" = /* && "$output" = /* ]] || usage
[[ "$client_requirement" == 'anchor apple generic and identifier '* ]] || usage
python3 "$root/packaging/macos/safe-target-output.py" validate \
  --root "$root" --artifact-root "$artifact_root" --output "$output" || usage
python3 "$root/packaging/macos/verify-provisioning-profile.py" \
  --profile "$supervisor_profile" --team-id "$team_id" \
  --identifier com.tekes.kernel.supervisor \
  --access-group "${team_id}.com.tekes.shared.endpoint" \
  --access-group "${team_id}.com.tekes.kernel.provider-secrets"
for path in "$selector" "$supervisor" "$worker" "$helper" "$conformance" "$workspace_service"; do
  [[ "$path" = /* && -f "$path" ]] || usage
done
python3 "$root/scripts/check-web-client-assets.py"

# Do not sign inside the repository tree. A File Provider-backed checkout may
# asynchronously attach FinderInfo/file-provider xattrs to a newly created
# .app between assembly and codesign, making otherwise identical release bytes
# unsignable. Build in the host temporary directory, then atomically publish
# the verified tree below the trusted artifact root.
temporary_root="${TMPDIR:-/tmp}"
temporary="$(mktemp -d "${temporary_root%/}/tekes-signed-release.XXXXXX")"
cleanup() { rm -rf -- "$temporary"; }
trap cleanup EXIT
supervisor_app="$temporary/bundle/apps/TekesKernelSupervisor.app"
supervisor_contents="$supervisor_app/Contents"
mkdir -p "$temporary/bundle/bin" "$temporary/selector/bin" \
  "$supervisor_contents/MacOS" "$supervisor_contents/Resources"
/bin/cp "$helper" "$temporary/bundle/bin/tekes-helper"
/bin/cp "$workspace_service" "$temporary/bundle/bin/tekes-workspace-service"
/bin/cp "$supervisor" "$supervisor_contents/MacOS/tekes-supervisor"
/bin/cp "$root/fixtures/web-client/manifest.canonical.json" \
  "$supervisor_contents/Resources/WebClientManifest.canonical.json"
/bin/cp "$worker" "$temporary/bundle/bin/tekes-worker"
/bin/cp "$selector" "$temporary/selector/bin/tekes-selector"
/bin/cp "$supervisor_profile" "$supervisor_contents/embedded.provisionprofile"
/bin/chmod 0755 "$temporary"/bundle/bin/* "$supervisor_contents/MacOS/tekes-supervisor" "$temporary/selector/bin/tekes-selector"

cat >"$supervisor_contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>tekes-supervisor</string>
<key>CFBundleIdentifier</key><string>com.tekes.kernel.supervisor</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>1.0</string>
<key>CFBundleVersion</key><string>1</string>
<key>LSBackgroundOnly</key><true/>
</dict></plist>
PLIST
/bin/chmod 0644 "$supervisor_contents/Info.plist" \
  "$supervisor_contents/Resources/WebClientManifest.canonical.json" \
  "$supervisor_contents/embedded.provisionprofile"

cat >"$temporary/supervisor.entitlements.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>com.apple.application-identifier</key>
<string>${team_id}.com.tekes.kernel.supervisor</string>
<key>com.apple.developer.team-identifier</key>
<string>${team_id}</string>
<key>keychain-access-groups</key><array>
<string>${team_id}.com.tekes.shared.endpoint</string>
<string>${team_id}.com.tekes.kernel.provider-secrets</string>
</array></dict></plist>
PLIST

# Finder/provenance xattrs can follow locally built binaries and downloaded
# provisioning profiles into the staging tree. They are not release content,
# and codesign rejects an app bundle that carries resource-fork/Finder
# detritus. Sanitize only the disposable staging copy; never mutate inputs.
/usr/bin/xattr -cr "$temporary"

sign() {
  local identifier="$1" path="$2" entitlements="${3:-}"
  local options=(--force --options runtime --timestamp --identifier "$identifier" --requirements "=designated => anchor apple generic and identifier \"$identifier\"")
  [[ -z "$entitlements" ]] || options+=(--entitlements "$entitlements")
  /usr/bin/codesign "${options[@]}" --sign "$identity" "$path"
  /usr/bin/codesign --verify --strict "$path"
}
sign com.tekes.kernel.selector "$temporary/selector/bin/tekes-selector"
sign com.tekes.kernel.supervisor "$supervisor_app" "$temporary/supervisor.entitlements.plist"
sign com.tekes.kernel.worker "$temporary/bundle/bin/tekes-worker"
sign com.tekes.kernel.helper "$temporary/bundle/bin/tekes-helper"
sign com.tekes.kernel.workspace-service "$temporary/bundle/bin/tekes-workspace-service"
registry_sha="$(/usr/bin/shasum -a 256 "$root/fixtures/deployment/authority-registry.canonical.json" | awk '{print $1}')"
python3 - "$supervisor_contents/MacOS/tekes-supervisor" "$release_version" "$registry_sha" "$source_revision" <<'PY'
import json
import subprocess
import sys

executable, release_version, registry_sha, source_revision = sys.argv[1:]
result = subprocess.run(
    [executable, "--describe-build"],
    stdin=subprocess.DEVNULL,
    capture_output=True,
    env={},
    timeout=10,
    check=False,
)
expected = {
    "authority_registry_sha256": registry_sha,
    "format": 1,
    "identifier": "com.tekes.kernel.supervisor",
    "source_revision": source_revision,
    "version": release_version,
}
canonical = json.dumps(expected, sort_keys=True, separators=(",", ":")).encode() + b"\n"
if result.returncode != 0 or result.stderr or result.stdout != canonical:
    raise SystemExit("signed supervisor build probe differs from explicit release inputs")
PY

python3 - "$supervisor_contents/MacOS/tekes-supervisor" "$supervisor_contents/Resources/WebClientManifest.canonical.json" <<'PY'
import json
import pathlib
import subprocess
import sys

executable, manifest_path = sys.argv[1], pathlib.Path(sys.argv[2])
manifest_raw = manifest_path.read_bytes()
manifest = json.loads(manifest_raw)
if json.dumps(manifest, sort_keys=True, separators=(",", ":")).encode() + b"\n" != manifest_raw:
    raise SystemExit("signed Web Client manifest is not canonical-plus-LF")
result = subprocess.run(
    [executable, "--describe-web-client"],
    stdin=subprocess.DEVNULL,
    capture_output=True,
    env={},
    timeout=10,
    check=False,
)
expected = json.dumps(
    {"format": 1, "sha256": manifest["sha256"]},
    sort_keys=True,
    separators=(",", ":"),
).encode() + b"\n"
if result.returncode != 0 or result.stderr or result.stdout != expected:
    raise SystemExit("signed supervisor Web Client digest differs from signed resource manifest")
PY

python3 - "$temporary/install-identity.canonical.json" "$team_id" "$client_requirement" <<'PY'
import json, pathlib, sys
path, team, client = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
value = {
    "access_group": f"{team}.com.tekes.shared.endpoint",
    "client_requirement": client,
    "format": 1,
    "installer_requirement": "anchor apple generic and identifier com.tekes.kernel.installer",
    "selector_requirement": "anchor apple generic and identifier com.tekes.kernel.selector",
    "supervisor_requirement": "anchor apple generic and identifier com.tekes.kernel.supervisor",
    "team_id": team,
}
path.write_bytes(json.dumps(value, sort_keys=True, separators=(",", ":")).encode() + b"\n")
PY
python3 "$root/packaging/macos/assemble-bundle.py" \
  --bundle-root "$temporary/bundle" --version "$release_version" --team-id "$team_id" \
  --requirement 'anchor apple generic and identifier com.tekes.kernel.supervisor' \
  --authority-registry "$root/fixtures/deployment/authority-registry.canonical.json"
python3 - "$temporary/selector/manifest.canonical.json" "$temporary/selector/bin/tekes-selector" "$conformance" "$team_id" "$release_version" <<'PY'
import hashlib, json, pathlib, sys
manifest, binary, evidence, team, version = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), pathlib.Path(sys.argv[3]), sys.argv[4], sys.argv[5]
raw = binary.read_bytes()
value = {
    "architecture": "aarch64",
    "conformance_sha256": hashlib.sha256(evidence.read_bytes()).hexdigest(),
    "file": {"bytes": len(raw), "mode": "0755", "path": "selector/bin/tekes-selector", "sha256": hashlib.sha256(raw).hexdigest()},
    "format": 1,
    "minimum_os": "15.0",
    "signing": {"requirement": "anchor apple generic and identifier com.tekes.kernel.selector", "team_id": team},
    "version": version,
}
manifest.write_bytes(json.dumps(value, sort_keys=True, separators=(",", ":")).encode() + b"\n")
PY
manifest_sha="$(/usr/bin/shasum -a 256 "$temporary/bundle/manifest.canonical.json" | awk '{print $1}')"
python3 "$root/packaging/macos/verify-release.py" bundle --codesign \
  --root "$temporary/bundle" --manifest "$temporary/bundle/manifest.canonical.json" \
  --install-identity "$temporary/install-identity.canonical.json" \
  --authority-registry "$root/fixtures/deployment/authority-registry.canonical.json" \
  --expected-manifest-sha256 "$manifest_sha"
python3 "$root/packaging/macos/verify-release.py" selector --codesign \
  --root "$temporary" --manifest "$temporary/selector/manifest.canonical.json" \
  --install-identity "$temporary/install-identity.canonical.json" --conformance-evidence "$conformance"
python3 "$root/packaging/macos/safe-target-output.py" publish \
  --root "$root" --artifact-root "$artifact_root" \
  --output "$output" --source "$temporary"
trap - EXIT
python3 "$root/packaging/macos/verify-release.py" bundle --codesign \
  --root "$output/bundle" --manifest "$output/bundle/manifest.canonical.json" \
  --install-identity "$output/install-identity.canonical.json" \
  --authority-registry "$root/fixtures/deployment/authority-registry.canonical.json" \
  --expected-manifest-sha256 "$manifest_sha"
python3 "$root/packaging/macos/verify-release.py" selector --codesign \
  --root "$output" --manifest "$output/selector/manifest.canonical.json" \
  --install-identity "$output/install-identity.canonical.json" \
  --conformance-evidence "$conformance"
echo "built verified signed release: $output"
