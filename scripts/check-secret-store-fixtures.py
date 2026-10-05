#!/usr/bin/env python3
"""Independent byte, digest and anti-rollback inventory check."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path


EXPECTED = {
    "README.md",
    "active.canonical.json",
    "cases.canonical.json",
    "generation-floor.canonical.json",
    "revoked.canonical.json",
}


def fail(message: str) -> None:
    raise SystemExit(f"secret store fixture check: {message}")


def canonical(value: object) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else "fixtures")
    corpus = root / "secret-store"
    actual = {path.name for path in corpus.iterdir() if path.is_file()}
    if actual != EXPECTED:
        fail(f"inventory mismatch: missing={sorted(EXPECTED-actual)} extra={sorted(actual-EXPECTED)}")

    values: dict[str, object] = {}
    for path in sorted(corpus.glob("*.canonical.json")):
        raw = path.read_bytes()
        if not raw.endswith(b"\n") or raw.endswith(b"\n\n"):
            fail(f"{path.name} must end in exactly one LF")
        try:
            value = json.loads(raw)
        except json.JSONDecodeError as error:
            fail(f"{path.name} is invalid JSON: {error}")
        if raw != canonical(value) + b"\n":
            fail(f"{path.name} is not canonical JSON+LF")
        values[path.name] = value

    active_raw = (corpus / "active.canonical.json").read_bytes()[:-1]
    active = values["active.canonical.json"]
    floor = values["generation-floor.canonical.json"]
    if set(floor) != {"access_group", "credentials", "format", "service"}:
        fail("generation authority field set drifted")
    if floor["format"] != 1 or floor["service"] != "com.tekes.kernel.provider-secret":
        fail("generation authority identity drifted")
    entry = floor["credentials"].get("provider-main")
    if set(entry or {}) != {"generation", "record_sha256"}:
        fail("generation floor shape drifted")
    if entry["generation"] != active["generation"]:
        fail("generation floor does not bind the active record generation")
    if entry["record_sha256"] != hashlib.sha256(active_raw).hexdigest():
        fail("generation floor digest does not bind exact SecretRecord bytes")
    floor_raw = (corpus / "generation-floor.canonical.json").read_bytes()
    if b"material" in floor_raw or b"fixture-secret-never-log" in floor_raw:
        fail("generation authority contains secret record material")

    cases = values["cases.canonical.json"]
    case_ids = {case["id"] for case in cases["cases"]}
    required = {
        "active-first-observation",
        "restart-lower-generation",
        "restart-same-generation-different-record",
        "rotation",
        "revocation",
        "publication-crash-debris",
    }
    if not required <= case_ids:
        fail(f"anti-rollback cases missing: {sorted(required-case_ids)}")
    print("secret-store fixtures: ok")


if __name__ == "__main__":
    main()
