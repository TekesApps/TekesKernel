#!/usr/bin/env python3
"""Fail closed when the Slice-14C oracle or internal/public boundary drifts."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures" / "schedule"
MANIFEST = ROOT / "fixtures" / "manifest.json"


def canonical(value: object) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
    ).encode("utf-8")


def main() -> None:
    expected = [
        "README.md",
        "cases.canonical.json",
        "invalid.canonical.json",
        "recovery.canonical.json",
    ]
    actual = sorted(
        str(path.relative_to(CORPUS)) for path in CORPUS.rglob("*") if path.is_file()
    )
    if actual != expected:
        raise SystemExit(f"schedule corpus mismatch: {actual}")
    manifest = json.loads(MANIFEST.read_bytes())
    disk = {
        corpus.name: sorted(
            str(path.relative_to(corpus))
            for path in corpus.rglob("*")
            if path.is_file()
        )
        for corpus in sorted(path for path in MANIFEST.parent.iterdir() if path.is_dir())
    }
    if manifest != {"corpora": disk, "version": 1}:
        raise SystemExit("fixture manifest does not match disk in both directions")
    if manifest["corpora"].get("schedule") != expected:
        raise SystemExit("schedule manifest entry does not match disk")
    for path in CORPUS.glob("*.canonical.json"):
        raw = path.read_bytes()
        if not raw.endswith(b"\n") or b"\n" in raw[:-1]:
            raise SystemExit(f"{path}: expected one object plus LF")
        value = json.loads(raw)
        if canonical(value) + b"\n" != raw:
            raise SystemExit(f"{path}: noncanonical bytes")

    forbidden = ('"schedule/list"', '"schedule/save"', '"schedule/delete"', '"schedule/runNow"')
    for subtree in [ROOT / "crates" / "endpoint" / "src", ROOT / "crates" / "transport" / "src"]:
        for path in subtree.rglob("*.rs"):
            text = path.read_text(encoding="utf-8")
            if any(token in text for token in forbidden):
                raise SystemExit(f"{path}: Slice 14C registered a public management route")
    print("schedule fixtures: 4 files; public management routes: 0")


if __name__ == "__main__":
    main()
