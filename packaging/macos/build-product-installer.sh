#!/bin/bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
contract="$root/packaging/macos/product-installer/contract.canonical.json"

usage() {
  echo 'usage: build-product-installer.sh --identity SIGNING_IDENTITY --team-id TEAMID --profile ABSOLUTE_PROFILE --output ABSOLUTE_APP_PATH' >&2
  exit 64
}

[[ "$#" -eq 8 ]] || usage
[[ "$1" == --identity && "$3" == --team-id && "$5" == --profile && "$7" == --output ]] || usage
signing_identity="$2"
team_id="$4"
profile="$6"
output="$8"
artifact_root="${TEKES_SIGNED_ARTIFACT_ROOT:-$root/target}"
[[ "$team_id" =~ ^[A-Z0-9]{10}$ && "$profile" = /* && -f "$profile" \
   && "$artifact_root" = /* && "$output" = /* && "$output" == *.app ]] || usage
python3 "$root/packaging/macos/safe-target-output.py" validate \
  --root "$root" --artifact-root "$artifact_root" --output "$output" || usage
python3 "$root/packaging/macos/verify-provisioning-profile.py" \
  --profile "$profile" --team-id "$team_id" --identifier com.tekes.kernel.installer \
  --access-group "${team_id}.com.tekes.shared.endpoint" \
  --access-group "${team_id}.com.tekes.kernel.provider-secrets"

temporary_root="${TMPDIR:-/tmp}"
temporary="$(mktemp -d "${temporary_root%/}/tekes-product-installer-build.XXXXXX")"
cleanup() { rm -rf -- "$temporary"; }
trap cleanup EXIT
app="$temporary/TekesKernelInstaller.app"
contents="$app/Contents"
mkdir -p "$contents/MacOS"

python3 - "$temporary/entitlements.plist" "$team_id" <<'PY'
from pathlib import Path
import sys
team = sys.argv[2]
Path(sys.argv[1]).write_text(f'''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>com.apple.application-identifier</key><string>{team}.com.tekes.kernel.installer</string>
<key>com.apple.developer.team-identifier</key><string>{team}</string>
<key>keychain-access-groups</key><array>
<string>{team}.com.tekes.shared.endpoint</string>
<string>{team}.com.tekes.kernel.provider-secrets</string>
</array></dict></plist>
''')
PY

python3 - "$contents/Info.plist" <<'PY'
from pathlib import Path
import sys
Path(sys.argv[1]).write_text('''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>tekes-kernel-installer</string>
<key>CFBundleIdentifier</key><string>com.tekes.kernel.installer</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>1.0</string>
<key>CFBundleVersion</key><string>1</string>
<key>LSBackgroundOnly</key><true/>
</dict></plist>
''')
PY

/bin/cp "$profile" "$contents/embedded.provisionprofile"
installer_target="$root/target/product-installer-$team_id"
TEKES_INSTALLER_TEAM_ID="$team_id" cargo build --locked --release \
  --manifest-path "$root/Cargo.toml" --target-dir "$installer_target" \
  --package tekes-kernel-installer
/bin/cp "$installer_target/release/tekes-kernel-installer" "$contents/MacOS/tekes-kernel-installer"
/bin/chmod 0644 "$contents/Info.plist" "$contents/embedded.provisionprofile"
/bin/chmod 0755 "$contents/MacOS/tekes-kernel-installer"
/usr/bin/xattr -cr "$app"
/usr/bin/codesign --force --options runtime --timestamp \
  --identifier com.tekes.kernel.installer \
  --requirements '=designated => anchor apple generic and identifier com.tekes.kernel.installer' \
  --entitlements "$temporary/entitlements.plist" --sign "$signing_identity" "$app"
/usr/bin/codesign --verify --strict \
  -R '=anchor apple generic and identifier com.tekes.kernel.installer' "$app"
"$contents/MacOS/tekes-kernel-installer" --describe-contract >"$temporary/contract.canonical.json"
/usr/bin/cmp "$temporary/contract.canonical.json" "$contract"
python3 "$root/packaging/macos/safe-target-output.py" publish \
  --root "$root" --artifact-root "$artifact_root" --output "$output" --source "$app"
/usr/bin/codesign --verify --strict \
  -R '=anchor apple generic and identifier com.tekes.kernel.installer' "$output"
"$output/Contents/MacOS/tekes-kernel-installer" --describe-contract \
  >"$temporary/published-contract.canonical.json"
/usr/bin/cmp "$temporary/published-contract.canonical.json" "$contract"
echo "built signed product installer: $output"
