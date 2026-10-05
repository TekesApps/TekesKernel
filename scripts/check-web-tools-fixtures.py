#!/usr/bin/env python3
"""Independent inventory/canonical/disposition check for web-tools."""

from __future__ import annotations

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EXPECTED = {
    "README.md",
    "extraction.canonical.json",
    "network-negative.canonical.json",
    "parity.canonical.json",
}


def fail(message: str) -> None:
    raise SystemExit(f"web tools fixture check: {message}")


def main() -> None:
    fixtures = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT / "fixtures"
    corpus = fixtures / "web-tools"
    actual = {path.name for path in corpus.iterdir() if path.is_file()}
    if actual != EXPECTED:
        fail(f"inventory mismatch: missing={sorted(EXPECTED-actual)} extra={sorted(actual-EXPECTED)}")
    for path in sorted(corpus.glob("*.canonical.json")):
        raw = path.read_bytes()
        if not raw.endswith(b"\n") or raw.endswith(b"\n\n"):
            fail(f"{path.name} must end in exactly one LF")
        value = json.loads(raw)
        expected = json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True).encode() + b"\n"
        if raw != expected:
            fail(f"{path.name} is not canonical JSON+LF")

    parity = json.loads((corpus / "parity.canonical.json").read_bytes())
    predecessor = parity["predecessor"]
    if predecessor.get("repository") != "TekesAppServer" or predecessor.get("revision") != "e4df9dc94e60025d0e8f62bcf98c95241181793a":
        fail("predecessor identity drifted")
    if predecessor.get("evidence") != sorted(predecessor.get("evidence", [])) or len(predecessor.get("evidence", [])) != 9:
        fail("predecessor evidence inventory drifted")
    rows = {row["behavior"]: row["disposition"] for row in parity["rows"]}
    expected_rows = {
        "public_get_ssrf_html_extract_truncate": "retained",
        "tavily_search_title_url_snippet": "retained",
        "jina_reader_fallback": "permanently_retired",
        "tavily_extract_fallback": "permanently_retired",
        "tavily_answer_score_published_date": "permanently_retired",
        "process_environment_web_credentials_and_reader_policy": "permanently_retired",
    }
    if rows != expected_rows:
        fail("predecessor disposition is incomplete or drifted")

    forbidden = ("jinareader", "tavilyextract", "tekes_web_reader", "jina_api_key")
    for path in (ROOT / "crates").rglob("*.rs"):
        lower = path.read_text(errors="replace").lower()
        for token in forbidden:
            if token in lower:
                fail(f"retired reader token {token!r} appears in production {path.relative_to(ROOT)}")

    manifest = json.loads((fixtures / "manifest.json").read_bytes())
    disk_corpora = {
        directory.name: sorted(
            str(path.relative_to(directory))
            for path in directory.rglob("*")
            if path.is_file()
        )
        for directory in sorted(path for path in fixtures.iterdir() if path.is_dir())
    }
    if manifest != {"corpora": disk_corpora, "version": 1}:
        fail("fixture manifest does not match disk in both directions")
    if manifest["corpora"].get("web-tools") != sorted(EXPECTED):
        fail("web-tools manifest entry does not match disk")
    print("web-tools fixtures: ok")


if __name__ == "__main__":
    main()
