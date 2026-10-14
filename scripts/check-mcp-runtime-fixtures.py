#!/usr/bin/env python3
"""Fail closed when the Slice-13 MCP corpus and global manifest diverge."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures" / "mcp-runtime"
MANIFEST = ROOT / "fixtures" / "manifest.json"


TASKS_EXTENSION = "io.modelcontextprotocol/tasks"
TASK_STATUSES = {"working", "input_required", "completed", "failed", "cancelled"}


def canonical_lines(path: Path) -> list[dict]:
    """Every transcript line is one canonical JSON-RPC object."""
    objects = []
    raw = path.read_bytes()
    if not raw.endswith(b"\n"):
        raise SystemExit(f"{path}: transcript must end in LF")
    for number, line in enumerate(raw[:-1].split(b"\n"), 1):
        value = json.loads(line)
        encoded = json.dumps(
            value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
        ).encode("utf-8")
        if line != encoded:
            raise SystemExit(f"{path}:{number}: noncanonical transcript line")
        objects.append(value)
    return objects


def check_task_transcripts() -> None:
    """The tasks-extension transcripts keep the flat SEP-2663 shapes."""
    for path in sorted(CORPUS.glob("tasks-extension-*.jsonl")) + [
        CORPUS / "legacy-handshake.jsonl",
        CORPUS / "modern-discover.jsonl",
    ]:
        lines = canonical_lines(path)
        for line in lines:
            if "method" in line and line["method"] in {"tasks/result", "tasks/list"}:
                raise SystemExit(f"{path}: the tasks extension has no {line['method']}")
            if "method" in line and line["method"] == "tools/call":
                if "task" in line["params"]:
                    raise SystemExit(f"{path}: extension tools/call must not carry task")
            result = line.get("result")
            if not isinstance(result, dict):
                continue
            if result.get("resultType") == "task" and "task" in result:
                raise SystemExit(f"{path}: creation result must be flat, not nested")
            if "taskId" in result or result.get("resultType") == "task":
                for legacy in ("ttl", "pollInterval"):
                    if legacy in result:
                        raise SystemExit(f"{path}: SEP-1686 key {legacy} in a flat task")
                if result["status"] not in TASK_STATUSES:
                    raise SystemExit(f"{path}: unknown task status {result['status']}")
                for key in ("createdAt", "lastUpdatedAt", "ttlMs"):
                    if key not in result:
                        raise SystemExit(f"{path}: flat task lacks {key}")
        if path.name.startswith("tasks-extension-handshake"):
            declared = lines[0]["params"]["capabilities"]["extensions"]
            advertised = lines[1]["result"]["capabilities"]
            if declared != {TASKS_EXTENSION: {}} or "tasks" in advertised:
                raise SystemExit(f"{path}: handshake must be extension-only on both sides")
            if advertised["extensions"] != {TASKS_EXTENSION: {}}:
                raise SystemExit(f"{path}: server does not advertise the tasks extension")


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
    check_task_transcripts()
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
