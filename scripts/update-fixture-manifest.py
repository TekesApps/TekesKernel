#!/usr/bin/env python3
"""Regenerate fixtures/manifest.json from the complete on-disk corpus tree."""

from __future__ import annotations

import json
import os
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures"
MANIFEST = FIXTURES / "manifest.json"


def main() -> int:
    corpora: dict[str, list[str]] = {}
    for corpus in sorted(path for path in FIXTURES.iterdir() if path.is_dir()):
        corpora[corpus.name] = sorted(
            path.relative_to(corpus).as_posix()
            for path in corpus.rglob("*")
            if path.is_file()
        )

    encoded = (
        json.dumps(
            {"corpora": corpora, "version": 1},
            ensure_ascii=False,
            indent=1,
            sort_keys=True,
        )
        + "\n"
    ).encode("utf-8")

    descriptor, temporary = tempfile.mkstemp(
        prefix=".manifest.", suffix=".json", dir=FIXTURES
    )
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(encoded)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, MANIFEST)
        directory = os.open(FIXTURES, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass

    print(
        f"fixture manifest: {len(corpora)} corpora, "
        f"{sum(map(len, corpora.values()))} files"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
