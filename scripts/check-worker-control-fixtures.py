#!/usr/bin/env python3
"""Validate the worker-control durable-control fixture registry and queue transaction oracle."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures/wire/durable"
META = {"README.md", "COVERAGE.md", "cases.canonical.json"}


class Invalid(Exception):
    pass


def pairs(values: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in values:
        if key in result:
            raise Invalid(f"duplicate key {key!r}")
        result[key] = value
    return result


def load(raw: bytes, where: Path | str) -> Any:
    try:
        return json.loads(raw, object_pairs_hook=pairs,
                          parse_constant=lambda value: (_ for _ in ()).throw(Invalid(value)))
    except (json.JSONDecodeError, UnicodeDecodeError, Invalid) as error:
        raise Invalid(f"{where}: invalid JSON: {error}") from error


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":")).encode()


def canonical_file(path: Path) -> Any:
    raw = path.read_bytes()
    body = raw[:-1] if raw.endswith(b"\n") else raw
    value = load(body, path)
    if body != canonical(value):
        raise Invalid(f"{path}: not canonical JSON")
    return value


def jsonl(path: Path) -> list[Any]:
    raw = path.read_bytes()
    if not raw.endswith(b"\n"):
        raise Invalid(f"{path}: missing final LF")
    rows = []
    for number, line in enumerate(raw.splitlines(), 1):
        if not line:
            raise Invalid(f"{path}:{number}: blank line")
        value = load(line, f"{path}:{number}")
        if line != canonical(value):
            raise Invalid(f"{path}:{number}: not canonical JSON")
        if not isinstance(value, dict) or len(value) != 1:
            raise Invalid(f"{path}:{number}: message must have one top-level key")
        rows.append(value)
    return rows


def validate_origin(value: Any, rpc_id: str, suffix: str) -> None:
    if not isinstance(value, dict) or set(value) != {"principal", "client", "target", "op", "key"}:
        raise Invalid(f"queue transaction: invalid {suffix} origin tuple")
    if value["key"] != f"{rpc_id}/{suffix}":
        raise Invalid(f"queue transaction: wrong {suffix} child key")
    if (value["principal"] != "uid:501" or value["client"] != "session-endpoint" or
            value["target"] != "018f0000-0000-7000-8000-000000000003" or
            value["op"] != "session.updateQueue"):
        raise Invalid(f"queue transaction: wrong {suffix} origin fields")


def validate_tx(row: Any) -> dict[str, Any]:
    if set(row) != {"queue_transaction"}:
        raise Invalid("queue transaction request has wrong top-level key")
    tx = row["queue_transaction"]
    if not isinstance(tx, dict) or set(tx) != {"delivery", "rpc_id", "target_seq", "retract_origin", "action"}:
        raise Invalid("queue transaction request fields changed")
    if not isinstance(tx["target_seq"], int) or isinstance(tx["target_seq"], bool) or tx["target_seq"] < 1:
        raise Invalid("queue transaction target_seq invalid")
    validate_origin(tx["retract_origin"], tx["rpc_id"], "retract")
    action = tx["action"]
    if not isinstance(action, dict) or action.get("kind") not in {"edit", "remove", "steer"}:
        raise Invalid("queue transaction action kind invalid")
    if action["kind"] == "edit":
        if set(action) not in ({"kind", "replacement_origin", "content", "steer"},
                               {"kind", "replacement_origin", "content", "steer", "assets"}):
            raise Invalid("queue edit action fields changed")
        if not isinstance(action["content"], list) or not action["content"]:
            raise Invalid("queue edit content missing")
        validate_origin(action["replacement_origin"], tx["rpc_id"], "replacement")
    elif action["kind"] == "remove":
        if set(action) != {"kind"}:
            raise Invalid("queue remove action has extra fields")
    else:
        if set(action) != {"kind", "replacement_origin"}:
            raise Invalid("queue steer action fields changed")
        validate_origin(action["replacement_origin"], tx["rpc_id"], "replacement")
    return tx


def validate_result(row: Any, tx: dict[str, Any], *, deduplicated: bool | None = None,
                    rejected: str | None = None) -> None:
    if set(row) != {"queue_transaction_result"}:
        raise Invalid("queue transaction result has wrong top-level key")
    result = row["queue_transaction_result"]
    if not isinstance(result, dict) or set(result) != {"delivery", "outcome"}:
        raise Invalid("queue transaction result fields changed")
    if result["delivery"] != tx["delivery"] or not isinstance(result["outcome"], dict):
        raise Invalid("queue transaction result correlation changed")
    outcome = result["outcome"]
    if rejected is None:
        if set(outcome) != {"kind", "first_seq", "last_seq", "deduplicated"} or outcome["kind"] != "committed":
            raise Invalid("queue transaction committed outcome changed")
        if outcome["deduplicated"] is not deduplicated:
            raise Invalid("queue transaction dedup flag changed")
        if not (isinstance(outcome["first_seq"], int) and outcome["first_seq"] >= 1 and
                isinstance(outcome["last_seq"], int) and outcome["last_seq"] >= outcome["first_seq"]):
            raise Invalid("queue transaction result seq range invalid")
        return
    expected = {"kind", "code"} | ({"reason"} if rejected == "attachment-error" else set())
    if set(outcome) != expected or outcome.get("kind") != "rejected" or outcome.get("code") != rejected:
        raise Invalid("queue transaction rejected outcome changed")
    if rejected == "attachment-error" and outcome.get("reason") != "CORRUPT":
        raise Invalid("queue transaction attachment rejection reason changed")


def main() -> int:
    registry = canonical_file(FIXTURES / "cases.canonical.json")
    if not isinstance(registry, dict) or set(registry) != {"format", "cases"} or registry["format"] != 1:
        raise Invalid("worker-control registry shape changed")
    cases = registry["cases"]
    ids = [case.get("id") for case in cases]
    if len(ids) != len(set(ids)):
        raise Invalid("worker-control registry has duplicate case ids")
    declared_present: set[str] = set()
    for case in cases:
        if set(case) != {"id", "artifacts", "implemented"} or not isinstance(case["implemented"], bool):
            raise Invalid("worker-control registry case shape changed")
        if case["implemented"]:
            for artifact in case["artifacts"]:
                path = FIXTURES / artifact
                if not path.is_file():
                    raise Invalid(f"implemented case {case['id']} missing {artifact}")
                declared_present.add(artifact)
    actual = {path.name for path in FIXTURES.iterdir() if path.is_file()} - META
    if actual != declared_present:
        raise Invalid(f"worker-control registry/disk mismatch missing={sorted(declared_present-actual)} extra={sorted(actual-declared_present)}")

    success = jsonl(FIXTURES / "queue-transaction.jsonl")
    if len(success) != 2:
        raise Invalid("queue transaction success transcript length changed")
    success_tx = validate_tx(success[0])
    validate_result(success[1], success_tx, deduplicated=False)

    retry = jsonl(FIXTURES / "queue-transaction-retry.jsonl")
    if len(retry) != 3:
        raise Invalid("queue transaction retry transcript length changed")
    first_tx = validate_tx(retry[0])
    second_tx = validate_tx(retry[1])
    if retry[0] != retry[1] or first_tx != second_tx:
        raise Invalid("queue transaction retry is not byte-identical")
    validate_result(retry[2], first_tx, deduplicated=True)

    recovery = canonical_file(FIXTURES / "queue-transaction-recovery.canonical.json")
    if recovery != {
        "format": 1,
        "fault": "crash-after-retraction",
        "durable_prefix": {"queue_edit_seq": 12, "replacement_absent": True},
        "management_gate": {"blocked_deliveries": ["stop-1", "input-2"],
                            "ordinary_launch": False, "projection_visible": False},
        "recovery": {"mode": "reconcile", "run_start_required": True,
                     "startup": "queue-transaction",
                     "redelivery": "delivery-queue-1", "append_only": "replacement",
                     "bypass": ["generic-reconciliation", "turn-open", "provider", "settle"],
                     "result": {"kind": "committed", "first_seq": 12,
                                "last_seq": 13, "deduplicated": True}},
        "release_order": ["stop-1", "input-2"],
    }:
        raise Invalid("queue transaction recovery state changed")

    startup = jsonl(FIXTURES / "queue-transaction-startup.jsonl")
    if len(startup) != 4 or startup[0] != {"hello": {"proto": "tekes-worker", "min": 2, "max": 2}} or startup[1] != {"selected": {"version": 2, "startup": "queue-transaction"}}:
        raise Invalid("queue transaction management startup negotiation changed")
    startup_tx = validate_tx(startup[2])
    validate_result(startup[3], startup_tx, deduplicated=True)

    rejected_rows = jsonl(FIXTURES / "queue-transaction-rejections.jsonl")
    if len(rejected_rows) != 6:
        raise Invalid("queue transaction rejection transcript length changed")
    for offset, code in ((0, "queue-item-not-found"), (2, "steer-unavailable"),
                         (4, "attachment-error")):
        rejected_tx = validate_tx(rejected_rows[offset])
        validate_result(rejected_rows[offset + 1], rejected_tx, rejected=code)

    races = canonical_file(FIXTURES / "queue-transaction-races.canonical.json")
    if races != {"format": 1, "cases": [
            {"name": "consumption-wins", "events": [], "gate_after": "released",
             "operation_phase": "complete", "rpc_phase": "complete-error",
             "result": {"kind": "rejected", "code": "queue-item-not-found"}},
            {"name": "prepared-vs-prompt",
             "acquisition_order": ["admission", "management-operation", "line"],
             "later_delivery": "blocked-before-handoff"},
            {"name": "cancel-during-incomplete", "volatile_stop_gate": True,
             "durable_stop": "after-transaction", "ordinary_work": False},
    ]}:
        raise Invalid("queue transaction race oracle changed")

    coverage = (FIXTURES / "COVERAGE.md").read_text()
    for gate in (65, 66):
        if f"Gate {gate}" not in coverage:
            raise Invalid(f"worker-control coverage omits Gate {gate}")
    print(f"worker-control durable-control fixtures: {len(cases)} cases, {len(actual)} artifacts, queue transaction recovery/rejection races covered")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Invalid as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1)
