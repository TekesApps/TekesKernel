#!/bin/bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
tekes_repo="${TEKES_CLIENT_REPO:-}"
if [[ "${1:-}" == --tekes-repo && "$#" -eq 2 ]]; then
  tekes_repo="$2"
elif [[ "$#" -ne 0 ]]; then
  echo 'usage: test-product-installer.sh [--tekes-repo ABSOLUTE_TEKES_REPO]' >&2
  exit 64
fi
if [[ -n "$tekes_repo" && ( "$tekes_repo" != /* || ! -f "$tekes_repo/scripts/check-bundled-kernel-product.py" ) ]]; then
  echo 'Tekes repository must be absolute and contain the product checker' >&2
  exit 64
fi

temporary="$(mktemp -d "${TMPDIR:-/tmp}/tekes-product-installer-test.XXXXXX")"
cleanup() { rm -rf -- "$temporary"; }
trap cleanup EXIT

installer_target="$root/target/product-installer-tests"
cargo test --locked --manifest-path "$root/Cargo.toml" \
  --target-dir "$installer_target" --package tekes-kernel-installer
cargo build --locked --manifest-path "$root/Cargo.toml" \
  --target-dir "$installer_target" --package tekes-kernel-installer
/bin/cp "$installer_target/debug/tekes-kernel-installer" "$temporary/tekes-kernel-installer"
"$temporary/tekes-kernel-installer" --describe-contract >"$temporary/contract.canonical.json"
/usr/bin/cmp "$temporary/contract.canonical.json" \
  "$root/packaging/macos/product-installer/contract.canonical.json"

set +e
"$temporary/tekes-kernel-installer" --operation explore \
  >"$temporary/invalid.stdout" 2>"$temporary/invalid.stderr"
invalid_status="$?"
set -e
[[ "$invalid_status" -eq 65 && ! -s "$temporary/invalid.stdout" ]]
/usr/bin/cmp "$temporary/invalid.stderr" <(printf '%s\n' '{"error":{"code":"invalid-arguments"}}')

python3 -m unittest discover -s "$root/packaging/macos/tests" -p 'test_*.py'

if [[ -n "$tekes_repo" ]]; then
  python3 - "$root/packaging/macos/product-installer/contract.canonical.json" \
    "$tekes_repo/scripts/check-bundled-kernel-product.py" <<'PY'
import importlib.util
import json
from pathlib import Path
import sys

contract_path, checker_path = map(Path, sys.argv[1:])
spec = importlib.util.spec_from_file_location("tekes_product_checker", checker_path)
assert spec and spec.loader
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)
expected = checker.canonical(checker.INSTALLER_CONTRACT) + b"\n"
if contract_path.read_bytes() != expected:
    raise SystemExit("Kernel installer contract differs from Tekes Client authority")
PY
fi

echo 'product installer focused tests passed'
