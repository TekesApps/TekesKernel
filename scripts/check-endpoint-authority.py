#!/usr/bin/env python3
"""Verify the vendored Session Endpoint v2 corpus and its pinned Tekes source."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parent.parent
CORPUS = ROOT / "fixtures" / "endpoint" / "authority"
LOCK = ROOT / "fixtures" / "endpoint" / "authority-lock.canonical.json"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--require-source", action="store_true")
    parser.add_argument("--tekes-repo", type=Path)
    args = parser.parse_args()
    lock = json.loads(LOCK.read_bytes())
    declared = set(lock["files"])
    disk = {path.name for path in CORPUS.iterdir() if path.is_file()}
    if declared != disk:
        raise SystemExit(f"authority corpus mismatch missing={declared-disk} extra={disk-declared}")
    for name, expected in lock["files"].items():
        actual = digest((CORPUS / name).read_bytes())
        if actual != expected:
            raise SystemExit(f"{name}: expected {expected}, found {actual}")

    source = args.tekes_repo
    if source is None:
        override = os.environ.get("TEKES_ENDPOINT_AUTHORITY_REPO")
        source = Path(override) if override else ROOT.parent / "Tekes"
    if not (source / ".git").exists():
        if args.require_source:
            raise SystemExit(f"Tekes source repository not found at {source}")
        return 0
    prefix = lock["path"]
    revision = lock["commit"]
    for name in sorted(declared):
        result = subprocess.run(
            ["git", "-C", str(source), "show", f"{revision}:{prefix}/{name}"],
            check=True,
            stdout=subprocess.PIPE,
        )
        local = (CORPUS / name).read_bytes()
        if result.stdout != local:
            raise SystemExit(f"{name}: vendored bytes differ from {revision}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
