#!/bin/bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
source_file="$root/packaging/macos/production-uat/main.swift"
identity_template="$root/packaging/macos/production-uat/BuildIdentity.swift.in"

usage() {
  echo 'usage: build-production-uat.sh --identity SIGNING_IDENTITY --team-id TEAMID --client-requirement REQUIREMENT --profile ABSOLUTE_PROFILE --output ABSOLUTE_APP_PATH' >&2
  exit 64
}

[[ "$#" -eq 10 ]] || usage
[[ "$1" == --identity && "$3" == --team-id && "$5" == --client-requirement \
   && "$7" == --profile && "$9" == --output ]] || usage
signing_identity="$2"
team_id="$4"
client_requirement="$6"
profile="$8"
output="${10}"
artifact_root="${TEKES_SIGNED_ARTIFACT_ROOT:-$root/target}"
[[ "$team_id" =~ ^[A-Z0-9]{10}$ ]] || usage
[[ "$profile" = /* && -f "$profile" && "$artifact_root" = /* \
   && "$output" = /* && "$output" == *.app ]] || usage
[[ "$client_requirement" == 'anchor apple generic and identifier '* ]] || usage
python3 "$root/packaging/macos/safe-target-output.py" validate \
  --root "$root" --artifact-root "$artifact_root" --output "$output" || usage
python3 "$root/packaging/macos/verify-provisioning-profile.py" \
  --profile "$profile" --team-id "$team_id" \
  --identifier com.tekes.kernel.installer \
  --access-group "${team_id}.com.tekes.shared.endpoint" \
  --access-group "${team_id}.com.tekes.kernel.provider-secrets" \
  --access-group "${team_id}.com.tekes.kernel.production-uat"
# Keep signing staging outside a possibly File Provider-backed checkout. The
# completed app is atomically published below the trusted artifact root only
# after verification.
temporary_root="${TMPDIR:-/tmp}"
temporary="$(mktemp -d "${temporary_root%/}/tekes-production-uat-build.XXXXXX")"
cleanup() { rm -rf -- "$temporary"; }
trap cleanup EXIT
app="$temporary/TekesProductionUAT.app"
contents="$app/Contents"
mkdir -p "$contents/MacOS"

python3 - "$identity_template" "$temporary/BuildIdentity.swift" "$team_id" "$client_requirement" <<'PY'
from pathlib import Path
import sys

source = Path(sys.argv[1]).read_text()
team = sys.argv[3]
requirement = sys.argv[4]
for value in (team, requirement):
    if any(character in value for character in ('"', '\\', '\n', '\r')):
        raise SystemExit('build identity contains a forbidden Swift string character')
Path(sys.argv[2]).write_text(source.replace('@TEAM_ID@', team).replace('@CLIENT_REQUIREMENT@', requirement))
PY

cat >"$temporary/entitlements.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>com.apple.application-identifier</key>
<string>${team_id}.com.tekes.kernel.installer</string>
<key>com.apple.developer.team-identifier</key>
<string>${team_id}</string>
<key>keychain-access-groups</key><array>
<string>${team_id}.com.tekes.shared.endpoint</string>
<string>${team_id}.com.tekes.kernel.provider-secrets</string>
<string>${team_id}.com.tekes.kernel.production-uat</string>
</array>
</dict></plist>
PLIST

cat >"$contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>tekes-production-uat</string>
<key>CFBundleIdentifier</key><string>com.tekes.kernel.installer</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>1.0</string>
<key>CFBundleVersion</key><string>1</string>
<key>LSBackgroundOnly</key><true/>
</dict></plist>
PLIST
/bin/cp "$profile" "$contents/embedded.provisionprofile"
/usr/bin/swiftc -O -framework Security -framework CryptoKit \
  "$source_file" "$temporary/BuildIdentity.swift" -o "$contents/MacOS/tekes-production-uat"
/bin/chmod 0644 "$contents/Info.plist" "$contents/embedded.provisionprofile"
/bin/chmod 0755 "$contents/MacOS/tekes-production-uat"
# Clear provenance/Finder xattrs from the disposable app copy before signing.
# Downloaded profiles and locally linked binaries may carry them even though
# they are not part of the signed release contract.
/usr/bin/xattr -cr "$app"
/usr/bin/codesign --force --options runtime --timestamp \
  --identifier com.tekes.kernel.installer \
  --requirements '=designated => anchor apple generic and identifier com.tekes.kernel.installer' \
  --entitlements "$temporary/entitlements.plist" \
  --sign "$signing_identity" "$app"
/usr/bin/codesign --verify --strict "$app"
/usr/bin/codesign --verify --strict \
  -R '=anchor apple generic and identifier com.tekes.kernel.installer' "$app"
"$contents/MacOS/tekes-production-uat" --describe-contract \
  >"$temporary/contract.canonical.json"
/usr/bin/cmp "$temporary/contract.canonical.json" \
  "$root/fixtures/deployment/production-uat-contract.canonical.json"
python3 "$root/packaging/macos/safe-target-output.py" publish \
  --root "$root" --artifact-root "$artifact_root" \
  --output "$output" --source "$app"
/usr/bin/codesign --verify --strict "$output"
/usr/bin/codesign --verify --strict \
  -R '=anchor apple generic and identifier com.tekes.kernel.installer' "$output"
"$output/Contents/MacOS/tekes-production-uat" --describe-contract \
  >"$temporary/published-contract.canonical.json"
/usr/bin/cmp "$temporary/published-contract.canonical.json" \
  "$root/fixtures/deployment/production-uat-contract.canonical.json"
echo "built signed production UAT app: $output"
echo "runner executable: $output/Contents/MacOS/tekes-production-uat"
