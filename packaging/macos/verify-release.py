#!/usr/bin/env python3
"""Verify exact Slice-10 bundle/selector manifest bytes against release files."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import plistlib
import re
import stat
import subprocess
import sys
from pathlib import Path
from typing import Any


HEX = re.compile(r"[0-9a-f]{64}")
TEAM = re.compile(r"[A-Z0-9]{10}")


class InvalidRelease(Exception):
    pass


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def read_canonical_lf(path: Path) -> tuple[Any, bytes]:
    raw = path.read_bytes()
    if not raw.endswith(b"\n") or raw.endswith(b"\n\n"):
        raise InvalidRelease(f"{path}: manifest is not canonical-plus-LF")
    try:
        value = json.loads(raw[:-1])
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise InvalidRelease(f"{path}: invalid JSON: {error}") from error
    if raw[:-1] != canonical(value):
        raise InvalidRelease(f"{path}: non-canonical JSON bytes")
    return value, raw


def regular_with_mode(path: Path, expected: str) -> None:
    info = path.lstat()
    if (
        not stat.S_ISREG(info.st_mode)
        or f"{stat.S_IMODE(info.st_mode):04o}" != expected
    ):
        raise InvalidRelease(f"{path}: expected regular mode {expected}")


def digest(path: Path) -> tuple[int, str]:
    payload = path.read_bytes()
    return len(payload), hashlib.sha256(payload).hexdigest()


def verify_codesign(path: Path, team_id: str, requirement: str | None) -> None:
    if sys.platform != "darwin":
        raise InvalidRelease("codesign verification requires macOS")
    verify_arguments = ["/usr/bin/codesign", "--verify", "--strict"]
    if requirement is not None:
        verify_arguments.extend(["-R", f"={requirement}"])
    verify_arguments.append(str(path))
    verified = subprocess.run(
        verify_arguments,
        capture_output=True,
        text=True,
        check=False,
    )
    if verified.returncode != 0:
        raise InvalidRelease(f"{path}: codesign verification failed")
    details = subprocess.run(
        ["/usr/bin/codesign", "-d", "--verbose=4", "-r-", str(path)],
        capture_output=True,
        text=True,
        check=False,
    )
    evidence = details.stdout + details.stderr
    if f"TeamIdentifier={team_id}" not in evidence:
        raise InvalidRelease(f"{path}: signing identity/requirement mismatch")


def verify_keychain_groups(path: Path, required: set[str], forbidden: set[str]) -> None:
    entitlements = subprocess.run(
        ["/usr/bin/codesign", "-d", "--entitlements", ":-", str(path)],
        capture_output=True,
        check=False,
    )
    if entitlements.returncode != 0:
        raise InvalidRelease(f"{path}: entitlement extraction failed")
    # codesign returns success with an empty stdout payload for a valid signed
    # binary that has no entitlements. Treat that as the empty entitlement
    # dictionary; required groups below still fail closed.
    if not entitlements.stdout.strip():
        payload = {}
    else:
        try:
            payload = plistlib.loads(entitlements.stdout)
        except (ValueError, plistlib.InvalidFileException) as error:
            raise InvalidRelease(f"{path}: invalid entitlement plist") from error
    groups = set(payload.get("keychain-access-groups", []))
    if not required.issubset(groups) or groups.intersection(forbidden):
        raise InvalidRelease(f"{path}: Keychain access-group mismatch")


def verify_file(
    path: Path,
    row: dict[str, Any],
    signing: dict[str, str],
    codesign: bool,
    exact_requirement: str | None = None,
) -> None:
    if (
        set(row) != {"path", "bytes", "sha256", "mode"}
        or row["mode"] not in {"0644", "0755"}
    ):
        raise InvalidRelease(f"{path}: invalid manifest file row")
    regular_with_mode(path, row["mode"])
    size, sha256 = digest(path)
    if row["bytes"] != size or row["sha256"] != sha256 or not HEX.fullmatch(sha256):
        raise InvalidRelease(f"{path}: byte/hash parity mismatch")
    if codesign:
        verify_codesign(path, signing["team_id"], exact_requirement)


def verify_signing(
    signing: Any, identity: dict[str, Any], requirement_key: str | None
) -> None:
    if (
        not isinstance(signing, dict)
        or set(signing) != {"team_id", "requirement"}
        or not TEAM.fullmatch(signing.get("team_id", ""))
        or signing["team_id"] != identity["team_id"]
        or not signing["requirement"].startswith("anchor apple generic and identifier ")
        or requirement_key is not None
        and signing["requirement"] != identity[requirement_key]
    ):
        raise InvalidRelease("manifest signing identity differs from immutable install identity")


def verify_selector_conformance_probe(path: Path, manifest: dict[str, Any]) -> None:
    expected = {
        "architecture": manifest["architecture"],
        "conformance_sha256": manifest["conformance_sha256"],
        "format": 1,
        "operation": "describe-conformance",
        "version": manifest["version"],
    }
    try:
        result = subprocess.run(
            [str(path), "describe-conformance"],
            stdin=subprocess.DEVNULL,
            capture_output=True,
            env={},
            timeout=10,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        raise InvalidRelease(f"{path}: selector conformance probe timed out") from error
    if (
        result.returncode != 0
        or result.stderr
        or result.stdout != canonical(expected) + b"\n"
    ):
        raise InvalidRelease(
            f"{path}: selector conformance probe differs from release manifest"
        )


def verify_bundle(args: argparse.Namespace, identity: dict[str, Any]) -> None:
    manifest, raw = read_canonical_lf(args.manifest)
    if set(manifest) != {"format", "version", "minimum_os", "architectures", "files", "compatibility", "signing"}:
        raise InvalidRelease("bundle manifest envelope")
    if manifest["format"] != 1 or manifest["minimum_os"] != "15.0" or manifest["architectures"] != ["aarch64"]:
        raise InvalidRelease("bundle platform contract")
    verify_signing(manifest["signing"], identity, "supervisor_requirement")
    registry, registry_raw = read_canonical_lf(args.authority_registry)
    if registry.get("format") != 1 or registry.get("profile") != "v1":
        raise InvalidRelease("authority registry profile")
    compatibility = manifest["compatibility"]
    if compatibility != {
        "authority_registry_sha256": hashlib.sha256(registry_raw).hexdigest(),
        "reader_profile": "v1",
        "writer_profile": "v1",
    }:
        raise InvalidRelease("bundle compatibility/authority digest mismatch")
    paths = [row.get("path") for row in manifest["files"]]
    expected = [
        "apps/TekesKernelSupervisor.app/Contents/Info.plist",
        "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
        "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json",
        "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources",
        "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile",
        "bin/tekes-helper",
        "bin/tekes-worker",
        "bin/tekes-workspace-service",
    ]
    if paths != expected:
        raise InvalidRelease("bundle executable set/order mismatch")
    for row in manifest["files"]:
        path = args.root / row["path"]
        executable = row["mode"] == "0755"
        exact_requirement = None
        if row["path"].endswith("/tekes-supervisor"):
            exact_requirement = identity["supervisor_requirement"]
        elif row["path"] == "bin/tekes-worker":
            exact_requirement = "anchor apple generic and identifier com.tekes.kernel.worker"
        elif row["path"] == "bin/tekes-helper":
            exact_requirement = "anchor apple generic and identifier com.tekes.kernel.helper"
        elif row["path"] == "bin/tekes-workspace-service":
            exact_requirement = 'anchor apple generic and identifier "com.tekes.kernel.workspace-service"'
        verify_file(
            path,
            row,
            manifest["signing"],
            args.codesign and executable,
            exact_requirement,
        )
        if args.codesign and executable:
            endpoint = identity["access_group"]
            provider_secret = f"{identity['team_id']}.com.tekes.kernel.provider-secrets"
            if row["path"].endswith("/tekes-supervisor"):
                verify_keychain_groups(path, {endpoint, provider_secret}, set())
            else:
                verify_keychain_groups(path, set(), {endpoint, provider_secret})
    if args.codesign:
        verify_codesign(
            args.root / "apps/TekesKernelSupervisor.app",
            identity["team_id"],
            identity["supervisor_requirement"],
        )
    if hashlib.sha256(raw).hexdigest() != args.expected_manifest_sha256:
        raise InvalidRelease("bundle manifest digest differs from frozen selection")


def verify_selector(args: argparse.Namespace, identity: dict[str, Any]) -> None:
    manifest, _ = read_canonical_lf(args.manifest)
    if set(manifest) != {"format", "version", "minimum_os", "architecture", "conformance_sha256", "file", "signing"}:
        raise InvalidRelease("selector manifest envelope")
    if (
        manifest["format"] != 1
        or manifest["minimum_os"] != "15.0"
        or manifest["architecture"] != "aarch64"
        or not HEX.fullmatch(manifest["conformance_sha256"])
    ):
        raise InvalidRelease("selector platform/conformance contract")
    verify_signing(manifest["signing"], identity, "selector_requirement")
    _, evidence_raw = read_canonical_lf(args.conformance_evidence)
    if manifest["conformance_sha256"] != hashlib.sha256(evidence_raw).hexdigest():
        raise InvalidRelease("selector conformance evidence digest mismatch")
    row = manifest["file"]
    if row.get("path") != "selector/bin/tekes-selector":
        raise InvalidRelease("selector stable pathname mismatch")
    selector = args.root / row["path"]
    verify_file(
        selector,
        row,
        manifest["signing"],
        args.codesign,
        identity["selector_requirement"],
    )
    verify_selector_conformance_probe(selector, manifest)
    if args.codesign:
        verify_keychain_groups(
            args.root / row["path"],
            set(),
            {
                identity["access_group"],
                f"{identity['team_id']}.com.tekes.kernel.provider-secrets",
            },
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("kind", choices=("bundle", "selector"))
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--install-identity", required=True, type=Path)
    parser.add_argument("--authority-registry", type=Path)
    parser.add_argument("--expected-manifest-sha256")
    parser.add_argument("--conformance-evidence", type=Path)
    parser.add_argument("--codesign", action="store_true")
    args = parser.parse_args()
    identity, _ = read_canonical_lf(args.install_identity)
    required_identity = {
        "format", "team_id", "access_group", "installer_requirement",
        "client_requirement", "selector_requirement", "supervisor_requirement",
    }
    if set(identity) != required_identity or identity["format"] != 1:
        raise InvalidRelease("install identity envelope")
    if identity["access_group"] != f"{identity['team_id']}.com.tekes.shared.endpoint":
        raise InvalidRelease("install identity access group")
    if args.kind == "bundle":
        if args.authority_registry is None or args.expected_manifest_sha256 is None:
            parser.error("bundle requires --authority-registry and --expected-manifest-sha256")
        verify_bundle(args, identity)
    else:
        if args.conformance_evidence is None:
            parser.error("selector requires --conformance-evidence")
        verify_selector(args, identity)
    print(f"verified Slice-10 {args.kind} release: {args.manifest}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (InvalidRelease, OSError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(65)
