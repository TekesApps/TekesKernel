#!/usr/bin/env python3
"""Verify embedded Tekes Web Client source bytes and their signed manifest."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WEB = ROOT / "crates/transport/web"
MANIFEST = ROOT / "fixtures/web-client/manifest.canonical.json"
ASSETS = ("app.css", "app.js", "index.html")


def canonical(value: object) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def main() -> int:
    digest = hashlib.sha256()
    rows = []
    for name in ASSETS:
        payload = (WEB / name).read_bytes()
        digest.update(name.encode())
        digest.update(b"\0")
        digest.update(len(payload).to_bytes(8, "big"))
        digest.update(payload)
        rows.append(
            {"bytes": len(payload), "path": name, "sha256": hashlib.sha256(payload).hexdigest()}
        )
    value = {"assets": rows, "format": 1, "sha256": digest.hexdigest()}
    expected = canonical(value) + b"\n"
    if MANIFEST.read_bytes() != expected:
        raise SystemExit("web client manifest differs from embedded source bytes")
    source = (ROOT / "crates/transport/src/web.rs").read_text()
    if value["sha256"] not in source:
        raise SystemExit("WEB_CLIENT_SHA256 differs from embedded source bytes")
    print(f"verified Tekes Web Client assets: {value['sha256']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
