#!/usr/bin/env python3
"""Fail closed when the Slice-13 MCP corpus and global manifest diverge."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures" / "mcp-runtime"
MANIFEST = ROOT / "fixtures" / "manifest.json"


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    expected = manifest["corpora"].get("mcp-runtime")
    actual = sorted(
        str(path.relative_to(CORPUS))
        for path in CORPUS.rglob("*")
        if path.is_file()
    )
    if expected != actual:
        raise SystemExit(f"mcp-runtime manifest mismatch: expected={expected} actual={actual}")
    for path in CORPUS.rglob("*.canonical.json"):
        raw = path.read_bytes()
        if not raw.endswith(b"\n") or raw[:-1].find(b"\n") != -1:
            raise SystemExit(f"{path}: expected one JSON line plus LF")
        value = json.loads(raw)
        encoded = json.dumps(
            value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
        ).encode("utf-8") + b"\n"
        if raw != encoded:
            raise SystemExit(f"{path}: noncanonical fixture bytes")
    recovery = json.loads((CORPUS / "recovery-matrix.canonical.json").read_text())
    expected_recovery_cases = {
        "cancelled_late_response": "ignore_once",
        "dead_peer": "evict",
        "handshake": "reconnect_once",
        "http_identity_change": "drop_session",
        "list_changed": "advance_catalog_generation",
        "protocol_error": "close_peer",
        "read": "reconnect_once",
        "resumable_task": "query_identity",
        "tool_mutation": "unknown_effect",
    }
    if recovery.get("executable_cases") != expected_recovery_cases:
        raise SystemExit("recovery matrix executable cases are incomplete")
    management = json.loads((CORPUS / "management-save.canonical.json").read_text())
    if set(management) != {"server", "credentialFields"}:
        raise SystemExit("management save DTO is not closed")
    operation = management["credentialFields"].get("environment.API_TOKEN")
    if operation != {"operation": "replace", "credential_id": "sentinel-credential"}:
        raise SystemExit("management keyed credential operation drifted")
    receipt = json.loads((CORPUS / "management-receipt.canonical.json").read_text())
    if "sentinel-token-material" in json.dumps(receipt):
        raise SystemExit("management receipt contains resolved credential material")
    print(f"mcp-runtime fixtures: {len(actual)} files")


if __name__ == "__main__":
    main()
