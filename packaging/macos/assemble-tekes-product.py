#!/usr/bin/env python3
"""Assemble the signed deployment release and product installer for Tekes."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import plistlib
import re
import shutil
import stat
import subprocess
import sys
import tempfile


TEAM = re.compile(r"[A-Z0-9]{10}")
VERSION = re.compile(r"[A-Za-z0-9_-][A-Za-z0-9._-]{0,127}")
INSTALLER_IDENTIFIER = "com.tekes.kernel.installer"
INSTALLER_REQUIREMENT = f"anchor apple generic and identifier {INSTALLER_IDENTIFIER}"
INSTALLER_PROTOCOL = "tekes-kernel-product-installer"
# The installer embeds this file and prints it for --describe-contract, so it is
# the single source of the operation list.
CONTRACT_PATH = Path(__file__).resolve().parent / "product-installer/contract.canonical.json"


class InvalidProduct(Exception):
    pass


def canonical(value: object) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode() + b"\n"


def read_canonical(path: Path) -> tuple[object, bytes]:
    raw = path.read_bytes()
    if not raw.endswith(b"\n") or raw.endswith(b"\n\n"):
        raise InvalidProduct(f"{path}: expected canonical JSON plus one LF")
    value = json.loads(raw[:-1])
    if canonical(value) != raw:
        raise InvalidProduct(f"{path}: noncanonical JSON")
    return value, raw


def load_contract(path: Path = CONTRACT_PATH) -> bytes:
    value, raw = read_canonical(path)
    if not isinstance(value, dict) or set(value) != {"format", "identifier", "operations", "protocol"}:
        raise InvalidProduct("installer contract fields differ")
    operations = value["operations"]
    if (
        type(value["format"]) is not int
        or value["format"] != 1
        or value["identifier"] != INSTALLER_IDENTIFIER
        or value["protocol"] != INSTALLER_PROTOCOL
        or not isinstance(operations, list)
        or not operations
        or not all(isinstance(operation, str) for operation in operations)
        or operations != sorted(set(operations))
    ):
        raise InvalidProduct("installer contract values differ")
    return raw


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def absolute_directory(path: Path) -> Path:
    if not path.is_absolute() or path.resolve() != path or not path.is_dir():
        raise InvalidProduct(f"not an absolute lexical directory: {path}")
    return path


def reject_symlinks(root: Path) -> None:
    for path in (root, *root.rglob("*")):
        if path.is_symlink():
            raise InvalidProduct(f"symlink forbidden: {path}")


def run(command: list[str], *, capture: bool = False) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        command,
        check=False,
        stdin=subprocess.DEVNULL,
        capture_output=capture,
        env={},
        timeout=120,
    )


def verify_installer(app: Path, team: str, contract: bytes) -> None:
    executable = app / "Contents/MacOS/tekes-kernel-installer"
    if not executable.is_file() or stat.S_IMODE(executable.stat().st_mode) != 0o755:
        raise InvalidProduct("installer executable is missing or has the wrong mode")
    verified = run([
        "/usr/bin/codesign", "--verify", "--strict", "-R", f"={INSTALLER_REQUIREMENT}", str(app)
    ], capture=True)
    if verified.returncode != 0:
        raise InvalidProduct("installer signature requirement failed")
    detail = run(["/usr/bin/codesign", "-d", "--verbose=4", str(app)], capture=True)
    if detail.returncode != 0 or f"TeamIdentifier={team}".encode() not in detail.stderr:
        raise InvalidProduct("installer team differs from release")
    entitlements = run(["/usr/bin/codesign", "-d", "--entitlements", ":-", str(app)], capture=True)
    try:
        values = plistlib.loads(entitlements.stdout)
    except (ValueError, plistlib.InvalidFileException) as error:
        raise InvalidProduct("installer entitlements are invalid") from error
    groups = set(values.get("keychain-access-groups", []))
    required = {
        f"{team}.com.tekes.shared.endpoint",
        f"{team}.com.tekes.kernel.provider-secrets",
    }
    if not required.issubset(groups):
        raise InvalidProduct("installer Keychain groups differ from deployment")
    described = run([str(executable), "--describe-contract"], capture=True)
    if described.returncode != 0 or described.stderr or described.stdout != contract:
        raise InvalidProduct("installer admission probe differs from contract")


def verify_release(root: Path, conformance: Path, repository: Path) -> tuple[str, str]:
    expected = {"bundle", "install-identity.canonical.json", "selector"}
    observed = {path.name for path in root.iterdir()}
    if observed not in (expected, expected | {"supervisor.entitlements.plist"}):
        raise InvalidProduct("release root differs from the deployment contract")
    identity, _ = read_canonical(root / "install-identity.canonical.json")
    bundle, _ = read_canonical(root / "bundle/manifest.canonical.json")
    if not isinstance(identity, dict) or not isinstance(bundle, dict):
        raise InvalidProduct("release identity or manifest is not an object")
    team = identity.get("team_id")
    version = bundle.get("version")
    if not isinstance(team, str) or not TEAM.fullmatch(team):
        raise InvalidProduct("release team is invalid")
    if not isinstance(version, str) or not VERSION.fullmatch(version):
        raise InvalidProduct("release version is invalid")
    bundle_sha = digest(root / "bundle/manifest.canonical.json")
    commands = [
        [
            sys.executable, str(repository / "packaging/macos/verify-release.py"), "bundle", "--codesign",
            "--root", str(root / "bundle"), "--manifest", str(root / "bundle/manifest.canonical.json"),
            "--install-identity", str(root / "install-identity.canonical.json"),
            "--authority-registry", str(repository / "fixtures/deployment/authority-registry.canonical.json"),
            "--expected-manifest-sha256", bundle_sha,
        ],
        [
            sys.executable, str(repository / "packaging/macos/verify-release.py"), "selector", "--codesign",
            "--root", str(root), "--manifest", str(root / "selector/manifest.canonical.json"),
            "--install-identity", str(root / "install-identity.canonical.json"),
            "--conformance-evidence", str(conformance),
        ],
    ]
    for command in commands:
        result = run(command, capture=True)
        if result.returncode != 0:
            sys.stderr.buffer.write(result.stderr)
            raise InvalidProduct("signed release verification failed")
    return team, version


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--release-root", required=True, type=Path)
    parser.add_argument("--installer-app", required=True, type=Path)
    parser.add_argument("--selector-conformance", required=True, type=Path)
    parser.add_argument("--artifact-root", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--tekes-checker", type=Path)
    args = parser.parse_args()
    repository = Path(__file__).resolve().parents[2]
    try:
        release = absolute_directory(args.release_root)
        installer = absolute_directory(args.installer_app)
        artifact_root = absolute_directory(args.artifact_root)
        if not args.output.is_absolute() or args.output.parent.resolve() != args.output.parent:
            raise InvalidProduct("output must have an absolute lexical parent")
        reject_symlinks(release)
        reject_symlinks(installer)
        team, version = verify_release(release, args.selector_conformance.resolve(), repository)
        contract = load_contract()
        verify_installer(installer, team, contract)
        with tempfile.TemporaryDirectory(prefix="tekes-kernel-product-") as temporary:
            staging = Path(temporary) / "TekesKernelProduct"
            (staging / "installer").mkdir(parents=True)
            shutil.copytree(installer, staging / "installer/TekesKernelInstaller.app", copy_function=shutil.copy2)
            (staging / "installer/contract.canonical.json").write_bytes(contract)
            (staging / "release").mkdir()
            shutil.copytree(release / "bundle", staging / "release/bundle", copy_function=shutil.copy2)
            shutil.copytree(release / "selector", staging / "release/selector", copy_function=shutil.copy2)
            shutil.copy2(
                release / "install-identity.canonical.json",
                staging / "release/install-identity.canonical.json",
            )
            product = {
                "format": 1,
                "installer": {
                    "contract_sha256": digest(staging / "installer/contract.canonical.json"),
                    "path": "installer/TekesKernelInstaller.app",
                    "requirement": INSTALLER_REQUIREMENT,
                },
                "protocol": "tekes-kernel-product-artifact-v1",
                "release": {
                    "bundle_manifest_sha256": digest(staging / "release/bundle/manifest.canonical.json"),
                    "install_identity_sha256": digest(staging / "release/install-identity.canonical.json"),
                    "path": "release",
                    "selector_manifest_sha256": digest(staging / "release/selector/manifest.canonical.json"),
                },
                "team_id": team,
                "version": version,
            }
            (staging / "product-manifest.canonical.json").write_bytes(canonical(product))
            reject_symlinks(staging)
            verify_installer(staging / "installer/TekesKernelInstaller.app", team, contract)
            if args.tekes_checker:
                result = run([sys.executable, str(args.tekes_checker.resolve()), "--root", str(staging), "--codesign"], capture=True)
                if result.returncode != 0:
                    sys.stderr.buffer.write(result.stderr)
                    raise InvalidProduct("Tekes product checker rejected the wrapper")
            safe = repository / "packaging/macos/safe-target-output.py"
            result = run([
                sys.executable, str(safe), "publish", "--root", str(repository),
                "--artifact-root", str(artifact_root), "--output", str(args.output), "--source", str(staging),
            ], capture=True)
            if result.returncode != 0:
                sys.stderr.buffer.write(result.stderr)
                raise InvalidProduct("atomic product publication failed")
        if args.tekes_checker:
            result = run([sys.executable, str(args.tekes_checker.resolve()), "--root", str(args.output), "--codesign"], capture=True)
            if result.returncode != 0:
                sys.stderr.buffer.write(result.stderr)
                raise InvalidProduct("published wrapper failed Tekes byte-sync")
    except (InvalidProduct, OSError, KeyError, TypeError, ValueError, subprocess.SubprocessError) as error:
        print(f"product assembly failed: {error}", file=sys.stderr)
        return 65
    print(f"assembled signed TekesKernel product: {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
