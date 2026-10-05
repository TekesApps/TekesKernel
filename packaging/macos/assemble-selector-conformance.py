#!/usr/bin/env python3
"""Bind the exact offline selector suite sources into canonical evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import tempfile
from pathlib import Path


FIXED_INPUTS = (
    "fixtures/deployment/selector.canonical.json",
    "fixtures/deployment/publication-state-machine.canonical.json",
    "fixtures/deployment/failure-attribution.canonical.json",
    "fixtures/deployment/cases/selector-update-power-loss.setup.canonical.json",
    "fixtures/deployment/cases/selector-update-power-loss.steps.canonical.json",
    "fixtures/deployment/cases/selector-update-power-loss.expected.canonical.json",
    "crates/selector/tests/slice10_selector.rs",
)
GATES = (
    "slice10_gate_73_install_upgrade_publication_crash_matrix",
    "slice10_gate_74_crash_loop_external_rollback",
)


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def publish(path: Path, payload: bytes) -> None:
    path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(prefix=".selector-conformance.", dir=path.parent)
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
    parser.add_argument("--workspace", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--release-version", required=True)
    args = parser.parse_args()
    if not args.workspace.is_absolute() or not args.output.is_absolute():
        parser.error("workspace and output must be absolute")
    sources = list(FIXED_INPUTS)
    sources.extend(
        path.relative_to(args.workspace).as_posix()
        for path in sorted((args.workspace / "crates/selector/src").glob("*.rs"))
    )
    if len(sources) != len(set(sources)):
        parser.error("selector conformance inputs are not unique")
    rows = []
    for relative in sorted(sources):
        path = args.workspace / relative
        payload = path.read_bytes()
        rows.append({
            "bytes": len(payload),
            "path": relative,
            "sha256": hashlib.sha256(payload).hexdigest(),
        })
    evidence = {
        "commands": [
            ["cargo", "test", "-p", "tekes-selector", "--locked", "--test", "slice10_selector", gate, "--", "--exact"]
            for gate in GATES
        ],
        "format": 1,
        "gates": list(GATES),
        "inputs": rows,
        "release_version": args.release_version,
    }
    publish(args.output, canonical(evidence) + b"\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
