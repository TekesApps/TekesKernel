#!/usr/bin/env python3
"""Independent byte and inventory check for web-search-provider."""

from __future__ import annotations

import json
import sys
from pathlib import Path


EXPECTED = {
    "README.md",
    "cases.canonical.json",
    "expected.canonical.json",
    "invalid-responses.canonical.json",
    "request.canonical.json",
    "response.canonical.json",
}


def fail(message: str) -> None:
    raise SystemExit(f"web search fixture check: {message}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else "fixtures")
    corpus = root / "web-search-provider"
    actual = {path.name for path in corpus.iterdir() if path.is_file()}
    if actual != EXPECTED:
        fail(f"inventory mismatch: missing={sorted(EXPECTED-actual)} extra={sorted(actual-EXPECTED)}")
    for path in sorted(corpus.glob("*.canonical.json")):
        raw = path.read_bytes()
        if not raw.endswith(b"\n") or raw.endswith(b"\n\n"):
            fail(f"{path.name} must end in exactly one LF")
        value = json.loads(raw)
        canonical = json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True).encode() + b"\n"
        if raw != canonical:
            fail(f"{path.name} is not canonical JSON+LF")
        if b"fixture-secret-never-log" in raw:
            fail(f"{path.name} contains credential material")
    request = json.loads((corpus / "request.canonical.json").read_bytes())
    if set(request) != {"query", "search_depth", "max_results", "topic", "include_answer", "include_raw_content", "include_images"}:
        fail("request field set drifted")
    if request["search_depth"] != "basic" or any(request[name] for name in ("include_answer", "include_raw_content", "include_images")):
        fail("request constants drifted")
    cases = json.loads((corpus / "cases.canonical.json").read_bytes())
    if cases["adapter"] != "tavily_v1" or cases["credential_scope"]["purpose"] != "web_search":
        fail("adapter or credential scope drifted")
    if cases.get("ssrf_rejections") != ["ipv4-mapped-loopback"]:
        fail("mapped IPv6 SSRF rejection is not frozen")
    invalid = json.loads((corpus / "invalid-responses.canonical.json").read_bytes())
    invalid_ids = [case["id"] for case in invalid["cases"]]
    if cases.get("invalid_responses") != invalid_ids or len(invalid_ids) != len(set(invalid_ids)):
        fail("invalid response inventory drifted")
    print("web-search-provider fixtures: ok")


if __name__ == "__main__":
    main()
