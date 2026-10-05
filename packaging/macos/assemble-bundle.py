#!/usr/bin/env python3
"""Create the canonical Slice-10 bundle manifest from built executable bytes."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import stat
import tempfile
from pathlib import Path


FILES = (
    ("apps/TekesKernelSupervisor.app/Contents/Info.plist", "0644"),
    ("apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor", "0755"),
    ("apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json", "0644"),
    ("apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources", "0644"),
    ("apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile", "0644"),
    ("bin/tekes-helper", "0755"),
    ("bin/tekes-worker", "0755"),
    ("bin/tekes-workspace-service", "0755"),
)


def canonical(value: object) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def publish(path: Path, payload: bytes) -> None:
    descriptor, temporary = tempfile.mkstemp(prefix=".bundle-manifest.", dir=path.parent)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(temporary, 0o600)
        os.replace(temporary, path)
        directory = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle-root", required=True, type=Path)
    parser.add_argument("--version", required=True)
    parser.add_argument("--team-id", required=True)
    parser.add_argument("--requirement", required=True)
    parser.add_argument("--authority-registry", required=True, type=Path)
    args = parser.parse_args()
    if not re.fullmatch(r"[A-Za-z0-9_-][A-Za-z0-9._-]{0,127}", args.version):
        parser.error("invalid deployment version")
    if not re.fullmatch(r"[A-Z0-9]{10}", args.team_id):
        parser.error("team id must be ten uppercase ASCII letters/digits")
    registry = args.authority_registry.read_bytes()
    if not registry.endswith(b"\n"):
        parser.error("authority registry must be canonical-plus-LF")
    try:
        decoded = json.loads(registry[:-1])
    except (UnicodeDecodeError, json.JSONDecodeError):
        parser.error("invalid authority registry JSON")
    if canonical(decoded) + b"\n" != registry:
        parser.error("authority registry must be canonical-plus-LF")

    rows = []
    for relative, expected_mode in FILES:
        path = args.bundle_root / relative
        metadata = path.lstat()
        mode = f"{stat.S_IMODE(metadata.st_mode):04o}"
        if not stat.S_ISREG(metadata.st_mode) or mode != expected_mode:
            parser.error(f"{relative} must be a regular mode-{expected_mode} file")
        payload = path.read_bytes()
        rows.append(
            {
                "bytes": len(payload),
                "mode": expected_mode,
                "path": relative,
                "sha256": hashlib.sha256(payload).hexdigest(),
            }
        )
    manifest = {
        "architectures": ["aarch64"],
        "compatibility": {
            "authority_registry_sha256": hashlib.sha256(registry).hexdigest(),
            "reader_profile": "v1",
            "writer_profile": "v1",
        },
        "files": rows,
        "format": 1,
        "minimum_os": "15.0",
        "signing": {"requirement": args.requirement, "team_id": args.team_id},
        "version": args.version,
    }
    publish(args.bundle_root / "manifest.canonical.json", canonical(manifest) + b"\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
