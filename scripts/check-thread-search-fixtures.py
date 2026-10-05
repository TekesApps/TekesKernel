#!/usr/bin/env python3
"""Fail closed when the Slice-14D contract, oracle, or route boundary drifts."""

from __future__ import annotations

import base64
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures" / "thread-search"
MANIFEST = ROOT / "fixtures" / "manifest.json"


def canonical(value: object) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
    ).encode("utf-8")


def main() -> None:
    expected_names = [
        "README.md",
        "cases.canonical.json",
        "index.canonical.json",
        "invalid.canonical.json",
    ]
    actual_names = sorted(
        str(path.relative_to(CORPUS)) for path in CORPUS.rglob("*") if path.is_file()
    )
    if actual_names != expected_names:
        raise SystemExit(f"thread-search corpus mismatch: {actual_names}")
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    disk_corpora = {
        corpus.name: sorted(
            str(path.relative_to(corpus))
            for path in corpus.rglob("*")
            if path.is_file()
        )
        for corpus in sorted(path for path in MANIFEST.parent.iterdir() if path.is_dir())
    }
    if manifest != {"corpora": disk_corpora, "version": 1}:
        raise SystemExit("fixture manifest does not match disk in both directions")
    if manifest["corpora"].get("thread-search") != expected_names:
        raise SystemExit("thread-search manifest entry does not match disk")

    for path in CORPUS.glob("*.canonical.json"):
        raw = path.read_bytes()
        if not raw.endswith(b"\n") or b"\n" in raw[:-1]:
            raise SystemExit(f"{path}: expected one JSON object plus LF")
        value = json.loads(raw)
        if canonical(value) + b"\n" != raw:
            raise SystemExit(f"{path}: noncanonical bytes")

    cases = json.loads((CORPUS / "cases.canonical.json").read_text())
    cursor = cases["active_first_cursor"]
    encoded, digest = cursor.split(".")
    padding = "=" * ((4 - len(encoded) % 4) % 4)
    body_bytes = base64.urlsafe_b64decode(encoded + padding)
    if base64.urlsafe_b64encode(body_bytes).decode().rstrip("=") != encoded:
        raise SystemExit("cursor is not canonical unpadded base64url")
    if hashlib.sha256(body_bytes).hexdigest() != digest:
        raise SystemExit("cursor checksum mismatch")
    body = json.loads(body_bytes)
    if canonical(body) != body_bytes:
        raise SystemExit("cursor body is not canonical")
    if set(body) != {
        "v",
        "workspace_id",
        "visibility",
        "normalized_query",
        "catalog_digest",
        "score",
        "session_id",
    }:
        raise SystemExit("cursor body field set drifted")

    index = json.loads((CORPUS / "index.canonical.json").read_text())
    source = canonical([index["entry"]])
    if index["source_digest"] != "sha256-" + hashlib.sha256(source).hexdigest():
        raise SystemExit("index source digest mismatch")

    forbidden = ('"thread.search"', '"thread/search"', '"session.search"')
    extension_owner = ROOT / "crates" / "supervisor" / "src" / "client_extensions.rs"
    for subtree in [ROOT / "crates" / "endpoint" / "src", ROOT / "crates" / "supervisor" / "src"]:
        for path in subtree.rglob("*.rs"):
            text = path.read_text(encoding="utf-8")
            if path != extension_owner and any(token in text for token in forbidden):
                raise SystemExit(f"{path}: Slice 14D registered a public search route")
    extension_text = extension_owner.read_text(encoding="utf-8")
    if '("thread.search", MethodClass::ReadOnly)' not in extension_text:
        raise SystemExit("Slice 14F production thread.search owner is absent")
    print("thread-search fixtures: 4 files; public route owner: Slice 14F client extensions")


if __name__ == "__main__":
    main()
