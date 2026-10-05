#!/usr/bin/env python3
"""Validate the complete Slice-9 endpoint transport fixture oracle."""

from __future__ import annotations

import base64
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures/endpoint-transport"
MAGIC = b"# tekes-endpoint-transport\n"
SAFE_MAX = 9_007_199_254_740_991
EXPECTED_KEYS = {"carrier_status", "rpc_result", "durable_effect", "connection_action"}
ACTIONS = {"keep-open", "reject", "close", "tail-refetch", "retire-projection", "rematerialize"}
RECORD_KEYS = {
    "http-request": ({"kind", "method", "path", "headers"}, ({"body_b64"}, {"body_repeat"})),
    "http-response": ({"kind", "status", "headers", "body_b64"}, (set(),)),
    "ws-open": ({"kind", "path", "headers"}, (set(),)),
    "ws-text": ({"kind", "body_b64"}, (set(),)),
    "ws-close": ({"kind", "code", "reason"}, (set(),)),
    "client-action": ({"kind", "name"}, (set(), {"session_id"})),
    "access-log": ({"kind", "record"}, (set(),)),
    "listener": ({"kind", "address", "result"}, (set(),)),
    "fault": ({"kind", "name"}, (set(),)),
}
CARRIER_ERRORS = {
    400: ("invalid-request", "Request is malformed"),
    401: ("unauthorized", "Authentication required"),
    403: ("forbidden-origin", "Browser origin is not allowed"),
    404: ("not-found", "Path not found"),
    405: ("method-not-allowed", "Method is not allowed"),
    413: ("payload-too-large", "Request body is too large"),
    415: ("unsupported-media-type", "Content-Type must be application/json"),
    429: ("overloaded", "Endpoint admission limit reached"),
}
MANAGEMENT_ROUTES = {
    "host-describe": "host.describe",
    "workspace-list": "workspace.list",
    "workspace-create": "workspace.create",
    "workspace-rename": "workspace.rename",
    "workspace-relocate": "workspace.relocate",
    "workspace-archive-session": "workspace.archiveSession",
    "workspace-unarchive-session": "workspace.unarchiveSession",
    "session-list": "session.list",
    "session-create": "session.create",
    "session-history": "session.history",
    "session-prompt": "session.prompt",
    "session-update-queue": "session.updateQueue",
    "session-cancel": "session.cancel",
    "session-rename": "session.rename",
    "session-fork": "session.fork",
    "session-discard": "session.discard",
    "session-attachment": "session.attachment",
    "session-models": "session.models",
    "session-select-model": "session.selectModel",
    "events-mux": "events.mux",
    "events-host": "events.host",
    "respond": "respond",
}
MANAGEMENT_ERRORS = {
    "bad-request": "Request payload is invalid",
    "unsupported-capability": "Capability is unavailable",
    "idempotency-conflict": "rpcId was already used for another request",
    "session-not-found": "Session was not found",
    "workspace-not-found": "Workspace was not found",
    "workspace-invalid-path": "Workspace path is invalid",
    "workspace-ambiguous": "Workspace path matches more than one workspace",
    "workspace-name-conflict": "Workspace title already exists",
    "workspace-title-invalid": "Workspace title is invalid",
    "workspace-busy": "Workspace has a running session",
    "session-conflict": "Session identity conflicts with existing state",
    "session-running": "Session has a running worker",
    "archived": "Session is archived",
    "ephemeral": "Session is ephemeral and cannot be archived",
    "not-ephemeral": "Session is not ephemeral",
    "not-archived": "Session is not archived",
    "invalid-cursor": "Session cursor is invalid",
    "invalid-at-seq": "Fork position is not a complete projection boundary",
    "active-session-limit": "Active session limit reached",
    "queue-item-not-found": "Queued item is no longer pending",
    "steer-unavailable": "Current turn no longer accepts steering",
    "attachment-error": "Image attachment is unavailable",
    "model-unavailable": "Model is unavailable",
    "title-invalid": "Session title is invalid",
    "accepted-but-not-confirmed": "Mutation was accepted but its receipt was not confirmed",
    "internal": "Endpoint operation failed",
}
ROUTE_ALLOWED_ERRORS = {
    "host.describe": {"internal"},
    "workspace.list": {"internal"},
    "workspace.create": {"workspace-invalid-path", "workspace-title-invalid", "workspace-name-conflict", "idempotency-conflict", "internal"},
    "workspace.rename": {"workspace-not-found", "workspace-title-invalid", "workspace-name-conflict", "idempotency-conflict", "internal"},
    "workspace.relocate": {"workspace-not-found", "workspace-invalid-path", "workspace-ambiguous", "workspace-busy", "idempotency-conflict", "internal"},
    "workspace.archiveSession": {"session-not-found", "session-running", "idempotency-conflict", "internal"},
    "workspace.unarchiveSession": {"session-not-found", "not-archived", "active-session-limit", "idempotency-conflict", "internal"},
    "session.list": {"invalid-cursor", "internal"},
    "session.create": {"bad-request", "workspace-not-found", "workspace-invalid-path", "workspace-ambiguous", "session-conflict", "active-session-limit", "unsupported-capability", "idempotency-conflict", "internal"},
    "session.history": {"session-not-found", "archived", "internal"},
    "session.prompt": {"session-not-found", "archived", "steer-unavailable", "attachment-error", "idempotency-conflict", "accepted-but-not-confirmed", "internal"},
    "session.updateQueue": {"session-not-found", "archived", "queue-item-not-found", "steer-unavailable", "attachment-error", "idempotency-conflict", "accepted-but-not-confirmed", "internal"},
    "session.cancel": {"session-not-found", "archived", "idempotency-conflict", "accepted-but-not-confirmed", "internal"},
    "session.rename": {"session-not-found", "archived", "title-invalid", "idempotency-conflict", "accepted-but-not-confirmed", "internal"},
    "session.fork": {"session-not-found", "archived", "invalid-at-seq", "active-session-limit", "idempotency-conflict", "internal"},
    "session.discard": {"session-not-found", "archived", "not-ephemeral", "session-running", "idempotency-conflict", "internal"},
    "session.attachment": {"session-not-found", "archived", "attachment-error", "internal"},
    "session.models": {"session-not-found", "archived", "internal"},
    "session.selectModel": {"session-not-found", "session-running", "archived", "model-unavailable", "idempotency-conflict", "internal"},
}


class Invalid(Exception):
    pass


def pairs(values: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in values:
        if key in result:
            raise Invalid(f"duplicate JSON key {key!r}")
        result[key] = value
    return result


def load_json(raw: bytes, where: Path | str) -> Any:
    try:
        return json.loads(raw, object_pairs_hook=pairs, parse_constant=lambda value: (_ for _ in ()).throw(Invalid(f"non-I-JSON number {value}")))
    except (json.JSONDecodeError, UnicodeDecodeError, Invalid) as error:
        raise Invalid(f"{where}: invalid JSON: {error}") from error


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def validate_i_json(value: Any, where: str) -> None:
    if value is None or isinstance(value, (str, bool)):
        return
    if isinstance(value, int):
        if abs(value) > SAFE_MAX:
            raise Invalid(f"{where}: unsafe integer {value}")
        return
    if isinstance(value, float):
        raise Invalid(f"{where}: fixture uses float; checker requires independently frozen number bytes")
    if isinstance(value, list):
        for index, item in enumerate(value):
            validate_i_json(item, f"{where}[{index}]")
        return
    if isinstance(value, dict):
        for key, item in value.items():
            if not isinstance(key, str):
                raise Invalid(f"{where}: non-string key")
            validate_i_json(item, f"{where}.{key}")
        return
    raise Invalid(f"{where}: unsupported JSON type {type(value).__name__}")


def exact_keys(record: dict[str, Any], where: str) -> None:
    kind = record.get("kind")
    if kind not in RECORD_KEYS:
        raise Invalid(f"{where}: unknown record kind {kind!r}")
    required, variants = RECORD_KEYS[kind]
    keys = set(record)
    if not any(keys == required | variant for variant in variants):
        raise Invalid(f"{where}: wrong fields for {kind}: {sorted(keys)}")


def decode_b64(value: Any, where: str) -> bytes:
    if not isinstance(value, str):
        raise Invalid(f"{where}: base64 is not a string")
    try:
        decoded = base64.b64decode(value, validate=True)
    except ValueError as error:
        raise Invalid(f"{where}: invalid base64") from error
    if base64.b64encode(decoded).decode() != value:
        raise Invalid(f"{where}: non-canonical padded base64")
    return decoded


def validate_headers(value: Any, where: str) -> dict[str, str]:
    if not isinstance(value, list) or any(not isinstance(item, list) or len(item) != 2 or not all(isinstance(part, str) for part in item) for item in value):
        raise Invalid(f"{where}: headers must be string pairs")
    if value != sorted(value):
        raise Invalid(f"{where}: headers are not sorted")
    names = [item[0] for item in value]
    if any(name != name.lower() for name in names) or len(names) != len(set(names)):
        raise Invalid(f"{where}: header names must be lowercase and unique")
    return dict(value)


def body_bytes(record: dict[str, Any], where: str) -> tuple[bytes | None, int]:
    if "body_b64" in record:
        body = decode_b64(record["body_b64"], where)
        return body, len(body)
    if "body_repeat" in record:
        repeat = record["body_repeat"]
        if not isinstance(repeat, dict) or set(repeat) != {"byte", "count"}:
            raise Invalid(f"{where}: invalid body_repeat")
        byte, count = repeat["byte"], repeat["count"]
        if not isinstance(byte, int) or isinstance(byte, bool) or not 0 <= byte <= 255:
            raise Invalid(f"{where}: invalid repeated byte")
        if not isinstance(count, int) or isinstance(count, bool) or not 0 < count <= SAFE_MAX:
            raise Invalid(f"{where}: invalid repeated count")
        return None, count
    return None, 0


def validate_record(record: Any, where: str) -> None:
    if not isinstance(record, dict):
        raise Invalid(f"{where}: transcript record is not an object")
    exact_keys(record, where)
    kind = record["kind"]
    if kind in {"http-request", "http-response", "ws-open"}:
        parsed = validate_headers(record["headers"], where)
        body, length = body_bytes(record, where)
        if kind.startswith("http"):
            if parsed.get("content-length") != str(length):
                raise Invalid(f"{where}: content-length mismatch")
            if parsed.get("content-type") is None:
                raise Invalid(f"{where}: missing content-type")
            if body is not None and parsed.get("content-type") == "application/json" and "/malformed." not in where:
                payload = load_json(body, where)
                if body != canonical(payload):
                    raise Invalid(f"{where}: HTTP JSON body is not canonical")
        if kind == "http-response" and record["status"] != 200:
            assert body is not None
            payload = load_json(body, where)
            if record["status"] == 503:
                allowed = {("not-ready", "Endpoint is not ready"), ("server-draining", "Endpoint is draining")}
            else:
                allowed = {CARRIER_ERRORS[record["status"]]}
            error = payload.get("error", {}) if isinstance(payload, dict) else {}
            if (error.get("code"), error.get("message")) not in allowed or error.get("details") != {}:
                raise Invalid(f"{where}: non-canonical carrier error")
    elif kind == "ws-text":
        payload = decode_b64(record["body_b64"], where)
        value = load_json(payload, where)
        if payload != canonical(value):
            raise Invalid(f"{where}: ws-text body is not canonical JSON")
        if not isinstance(value, dict) or value.get("type") != "server-request":
            raise Invalid(f"{where}: ws-text must carry a server-request envelope")
        frame = value.get("payload")
        if not isinstance(frame, dict) or not isinstance(frame.get("type"), str):
            raise Invalid(f"{where}: server-request payload has no frame type")
        if value.get("method") != frame["type"]:
            raise Invalid(f"{where}: server-request method must equal payload.type")
        if frame["type"] == "stream/error" and value["method"] != "stream/error":
            raise Invalid(f"{where}: stream/error envelope has the wrong method")
    elif kind == "ws-close":
        if record["code"] not in {1000, 1002, 1009, 1011, 1013} or not isinstance(record["reason"], str):
            raise Invalid(f"{where}: invalid close")
    elif kind == "client-action":
        if not isinstance(record["name"], str) or not record["name"]:
            raise Invalid(f"{where}: empty client action")
    elif kind == "access-log":
        log = record["record"]
        required = {"v", "request_id", "operation", "path", "status", "elapsed_ms"}
        if not isinstance(log, dict) or set(log) not in {frozenset(required), frozenset(required | {"error_code"})}:
            raise Invalid(f"{where}: invalid access-log fields")
        if log["v"] != 1 or log["elapsed_ms"] != 0:
            raise Invalid(f"{where}: access-log is not normalized")
    elif kind == "listener":
        if record["result"] not in {"accepted", "rejected"} or not re.fullmatch(r"(?:\d{1,3}\.){3}\d{1,3}:\d+", record["address"]):
            raise Invalid(f"{where}: invalid listener observation")


def transcript(path: Path) -> list[dict[str, Any]]:
    raw = path.read_bytes()
    if not raw.startswith(MAGIC) or not raw.endswith(b"\n"):
        raise Invalid(f"{path}: bad magic or missing final LF")
    records = []
    for number, line in enumerate(raw[len(MAGIC):].splitlines(), 1):
        if not line:
            raise Invalid(f"{path}:{number}: blank record")
        value = load_json(line, f"{path}:{number}")
        validate_i_json(value, f"{path}:{number}")
        if line != canonical(value):
            raise Invalid(f"{path}:{number}: record is not canonical JSON")
        validate_record(value, f"{path}:{number}")
        records.append(value)
    return records


def validate_expected(path: Path) -> dict[str, Any]:
    raw = path.read_bytes()
    body = raw[:-1] if raw.endswith(b"\n") else raw
    value = load_json(body, path)
    validate_i_json(value, str(path))
    if body != canonical(value):
        raise Invalid(f"{path}: expected result is not canonical JSON")
    if not isinstance(value, dict) or set(value) != EXPECTED_KEYS:
        raise Invalid(f"{path}: expected result fields are not the closed set")
    status = value["carrier_status"]
    if status != "open" and not re.fullmatch(r"http-\d{3}|ws-closed-\d{4}", status):
        raise Invalid(f"{path}: invalid carrier_status")
    if value["connection_action"] not in ACTIONS:
        raise Invalid(f"{path}: invalid connection_action")
    rpc = value["rpc_result"]
    if rpc != "none":
        if not isinstance(rpc, dict) or rpc.get("ok") not in {True, False}:
            raise Invalid(f"{path}: invalid rpc_result")
        if rpc["ok"] is True and set(rpc) != {"ok", "value"}:
            raise Invalid(f"{path}: successful rpc_result must carry only value")
        if rpc["ok"] is False:
            if set(rpc) != {"ok", "error"} or not isinstance(rpc["error"], dict) or set(rpc["error"]) != {"code", "message", "details"}:
                raise Invalid(f"{path}: failed rpc_result shape")
    effect = value["durable_effect"]
    if effect != "none":
        if not isinstance(effect, dict) or set(effect) not in ({"kind", "identity"}, {"kind", "identity", "seq"}):
            raise Invalid(f"{path}: durable_effect shape")
        if not all(isinstance(effect[key], str) and effect[key] for key in ("kind", "identity")):
            raise Invalid(f"{path}: empty durable effect identity")
        if "seq" in effect and (not isinstance(effect["seq"], int) or isinstance(effect["seq"], bool) or not 0 <= effect["seq"] <= SAFE_MAX):
            raise Invalid(f"{path}: invalid durable effect seq")
    return value


def decoded_http(record: dict[str, Any], where: str) -> Any:
    body = decode_b64(record["body_b64"], where)
    return load_json(body, where)


def validate_management_case(case: str, requests: list[dict[str, Any]], responses: list[dict[str, Any]], outcome: dict[str, Any]) -> None:
    route = MANAGEMENT_ROUTES[case]
    if route in {"events.mux", "events.host"}:
        opens = [record for record in requests if record["kind"] == "ws-open"]
        if len(opens) != 1 or opens[0]["path"] != f"/api/{route}":
            raise Invalid(f"management/{case}: wrong WebSocket registration")
        frames = [decoded_http({"body_b64": record["body_b64"]}, f"management/{case}") for record in responses if record["kind"] == "ws-text"]
        if not frames or not any(frame.get("payload", {}).get("type") == "stream/error" for frame in frames):
            raise Invalid(f"management/{case}: missing success/error stream coverage")
        non_errors = [frame for frame in frames if frame.get("payload", {}).get("type") != "stream/error"]
        if not non_errors:
            raise Invalid(f"management/{case}: no successful frame")
        return

    http_requests = [record for record in requests if record["kind"] == "http-request"]
    http_responses = [record for record in responses if record["kind"] == "http-response"]
    if len(http_requests) < 2 or len(http_responses) < 2:
        raise Invalid(f"management/{case}: request/success/error coverage incomplete")
    path = "/api/respond" if route == "respond" else f"/api/{route}"
    if any(record["method"] != "POST" or record["path"] != path for record in http_requests):
        raise Invalid(f"management/{case}: wrong method/path")
    request_values = [decoded_http(record, f"management/{case}.request") for record in http_requests]
    response_values = [decoded_http(record, f"management/{case}.response") for record in http_responses]

    if route == "respond":
        for request in request_values:
            if set(request) != {"type", "rpcId", "result"} or request["type"] != "client-response" or request["result"].get("ok") is not True:
                raise Invalid("management/respond: wrong client-response envelope")
        values = [request["result"]["value"] for request in request_values]
        if not any(set(value) == {"sessionId", "approvalId", "outcome"} and value["outcome"] in {"allowed-once", "rejected"} for value in values):
            raise Invalid("management/respond: approval value arm missing")
        if not any(set(value) == {"sessionId", "answer"} and set(value["answer"]) == {"answers"} and isinstance(value["answer"]["answers"], list) for value in values):
            raise Invalid("management/respond: question value arm missing")
        if response_values[0] != {"accepted": True} or not any(value == {"accepted": False, "reason": "unknown-rpc-id"} for value in response_values):
            raise Invalid("management/respond: success/rejection receipts changed")
        if outcome["rpc_result"] != {"ok": True, "value": {"accepted": True}}:
            raise Invalid("management/respond: expected result is not success receipt")
        return

    for request in request_values:
        if set(request) != {"type", "rpcId", "method", "payload"} or request["type"] != "client-request" or request["method"] != route:
            raise Invalid(f"management/{case}: wrong client-request envelope")
    successes = [response for response in response_values if response.get("result", {}).get("ok") is True]
    failures = [response for response in response_values if response.get("result", {}).get("ok") is False]
    if not successes or not failures:
        raise Invalid(f"management/{case}: no success or typed semantic error")
    for response in response_values:
        if response.get("type") != "server-response" or not isinstance(response.get("rpcId"), str):
            raise Invalid(f"management/{case}: wrong server-response envelope")
    for response in failures:
        error = response["result"]["error"]
        if set(error) != {"code", "message", "details"} or MANAGEMENT_ERRORS.get(error["code"]) != error["message"]:
            raise Invalid(f"management/{case}: error outside closed table")
    first_value = successes[0]["result"]["value"]
    if outcome["rpc_result"] != {"ok": True, "value": first_value}:
        raise Invalid(f"management/{case}: expected success does not match transcript")

    if route == "session.updateQueue":
        actions = {request["payload"]["action"]["kind"] for request in request_values}
        if actions != {"edit", "remove", "steer"}:
            raise Invalid("management/session-update-queue: action union incomplete")
        queue_frames = []
        for record in responses:
            if record["kind"] == "ws-text":
                frame = decoded_http({"body_b64": record["body_b64"]}, "management queue frame")["payload"]
                if frame.get("type") == "session/queue":
                    queue_frames.append(frame)
        if len(queue_frames) != 1 or [item["id"] for item in queue_frames[0]["items"]] != ["input:11", "input:13"]:
            raise Invalid("management/session-update-queue: replacement tail order changed")
        request_markers = {record.get("name") for record in requests if record["kind"] in {"fault", "client-action"}}
        response_markers = {record.get("name") for record in responses if record["kind"] == "client-action"}
        if request_markers != {"crash-after-queue-retraction", "concurrent-stop", "concurrent-input"}:
            raise Invalid("management/session-update-queue: crash/concurrency inputs changed")
        if response_markers != {"management-gate-holds", "reconcile-queue-transaction", "release-concurrent-deliveries"}:
            raise Invalid("management/session-update-queue: recovery gate observations changed")
    if route == "session.create":
        if not any(request["payload"].get("agentPreset") == "standard" for request in request_values):
            raise Invalid("management/session-create: agentPreset rejection arm missing")
        codes = {response["result"]["error"]["code"] for response in failures}
        if not {"unsupported-capability", "active-session-limit"} <= codes:
            raise Invalid("management/session-create: create closure errors missing")
    if route == "session.list":
        payloads = [request["payload"] for request in request_values]
        if payloads[0] != {} or not any(payload == {"cursor": "!"} for payload in payloads):
            raise Invalid("management/session-list: reserved cursor behavior changed")


def validate_request_journal() -> tuple[set[str], str, str]:
    # Requests are derived from the semantic ledger (no request journal); the
    # oracle freezes the derived request/resolution identities and envelopes.
    path = FIXTURES / "management-requests.canonical.jsonl"
    raw = path.read_bytes()
    if not raw.endswith(b"\n"):
        raise Invalid("management request oracle lacks final LF")
    rows = [load_json(line, f"{path}:{index}") for index, line in enumerate(raw.splitlines(), 1)]
    if len(rows) != 10:
        raise Invalid("management request oracle must cover five request/resolution pairs")
    for index, (line, row) in enumerate(zip(raw.splitlines(), rows), 1):
        validate_i_json(row, f"{path}:{index}")
        if line != canonical(row):
            raise Invalid(f"{path}:{index}: noncanonical line")
    request_rows = [row for row in rows if row.get("kind") == "request"]
    resolution_rows = [row for row in rows if row.get("kind") == "resolution"]
    if len(request_rows) != 5 or len(resolution_rows) != 5:
        raise Invalid("management request oracle pair kinds changed")
    rpc_ids: list[str] = []
    for row in request_rows:
        if set(row) != {"kind", "rpc_id", "session_id", "frame_type", "causal_kernel_seq", "envelope"}:
            raise Invalid("management request record shape changed")
        preimage = row["session_id"].encode() + b"\0" + row["frame_type"].encode() + b"\0" + str(row["causal_kernel_seq"]).encode()
        derived = "request-" + hashlib.sha256(preimage).hexdigest()
        if row["rpc_id"] != derived:
            raise Invalid("management request rpcId derivation changed")
        envelope = row["envelope"]
        if envelope != {"type": "server-request", "rpcId": derived, "method": row["frame_type"], "payload": envelope.get("payload")}:
            raise Invalid("management requested envelope changed")
        if envelope["payload"].get("type") != row["frame_type"] or envelope["payload"].get("sessionId") != row["session_id"]:
            raise Invalid("management request payload does not match record")
        rpc_ids.append(derived)
    if {row.get("rpc_id") for row in resolution_rows} != set(rpc_ids):
        raise Invalid("management request resolutions do not close every request")
    for row in resolution_rows:
        if set(row) != {"kind", "rpc_id", "causal_kernel_seq", "outcome"}:
            raise Invalid("management resolution record shape changed")
    if sorted(row.get("outcome") for row in resolution_rows) != [
            "allowed-once", "answered", "cancelled", "cancelled", "rejected"]:
        raise Invalid("management request resolution vocabulary changed")
    return set(rpc_ids), rpc_ids[0], rpc_ids[1]


def canonical_contract(path: Path) -> Any:
    raw = path.read_bytes()
    body = raw[:-1] if raw.endswith(b"\n") else raw
    value = load_json(body, path)
    validate_i_json(value, str(path))
    if body != canonical(value):
        raise Invalid(f"{path}: contract fixture is not canonical JSON")
    return value


def validate_management_contract_and_auxiliary() -> None:
    spec = (ROOT / "spec/session-endpoint.md").read_text()
    registration_section = spec.split("### Closed registration table", 1)[1].split("### Closed semantic errors", 1)[0]
    registrations = set(re.findall(r"^\| `([^`]+)` \|", registration_section, re.MULTILINE))
    if registrations != set(MANAGEMENT_ROUTES.values()):
        raise Invalid(f"management spec/route registry mismatch missing={sorted(set(MANAGEMENT_ROUTES.values())-registrations)} extra={sorted(registrations-set(MANAGEMENT_ROUTES.values()))}")

    error_section = spec.split("## Closed semantic errors", 1)[1]
    code_table, allowed_table = error_section.split("The allowed semantic codes are closed per registration", 1)
    spec_errors = {code: message for code, message in re.findall(r"^\| `([^`]+)` \| `([^`]+)` \|", code_table, re.MULTILINE)}
    if spec_errors != MANAGEMENT_ERRORS:
        raise Invalid("management spec/checker closed semantic error table differs")
    parsed_allowed: dict[str, set[str]] = {}
    for registrations_cell, codes_cell in re.findall(r"^\| ([^|]+) \| ([^|]+) \|", allowed_table.split("`events.mux`", 1)[0], re.MULTILINE):
        routes = re.findall(r"`([^`]+)`", registrations_cell)
        codes = set(re.findall(r"`([^`]+)`", codes_cell))
        for route in routes:
            parsed_allowed[route] = codes
    if parsed_allowed != ROUTE_ALLOWED_ERRORS:
        raise Invalid("management spec/checker per-route error table differs")

    resolution = canonical_contract(FIXTURES / "management-create-resolution.canonical.json")
    if resolution.get("workspace") != {"workspaceId": "018f0000-0000-7000-8000-000000000001", "path": "/workspace/project"}:
        raise Invalid("management create resolution does not use Workspace.path")
    cases = {case.get("name"): case for case in resolution.get("cases", [])}
    if set(cases) != {"workspace-id-only", "cwd-only", "both-match", "both-mismatch", "neither-single-active", "neither-multiple-active", "agent-preset"}:
        raise Invalid("management create resolution case closure changed")
    if cases["both-mismatch"].get("error") != "workspace-invalid-path" or cases["agent-preset"].get("error") != "unsupported-capability":
        raise Invalid("management create resolution failure semantics changed")
    if "Workspace.cwd" in spec or "cwd[0]" in spec:
        raise Invalid("management spec retains nonexistent Workspace.cwd")

    recovery = canonical_contract(FIXTURES / "management-recovery.canonical.json")
    archive = recovery.get("archive_carriers_crash", {})
    if archive.get("phase") != "carriers" or archive.get("lock_at_crash") != "none" or archive.get("recovery", [])[:2] != ["acquire-exclusive-lifecycle-lock", "revalidate-reservation"]:
        raise Invalid("archive carriers recovery does not reacquire/revalidate")
    drift = recovery.get("workspace_digest_mismatch", {})
    op = drift.get("synthetic_operation", {})
    if op.get("metadata_updated_at") != op.get("started_at") or op.get("preserve_created_at") is not True:
        raise Invalid("workspace digest reconciliation timestamp changed")
    overflow = recovery.get("active_inventory_overflow", {})
    if overflow != {"active_folders": 257, "limit": 256, "readiness": False,
                    "mux_partial_attach": False,
                    "error": {"code": "active-session-limit", "details": {"limit": 256}}}:
        raise Invalid("active-session overflow readiness semantics changed")

    carriers = canonical_contract(FIXTURES / "management-redact-carriers.canonical.json")
    if set(carriers.get("source", {})) != {"endpoint.jsonl"} or set(carriers.get("destination", {})) != set(carriers["source"]):
        raise Invalid("endpoint rewrite carrier fixture is incomplete")
    for forbidden in carriers.get("redact", {}).get("scan_forbidden", []):
        if not any(forbidden in value for value in carriers["source"].values()):
            raise Invalid("endpoint rewrite fixture forbidden source missing")
        if any(forbidden in value for value in carriers["destination"].values()):
            raise Invalid("endpoint rewrite destination retains forbidden bytes")
    if carriers.get("fork", {}).get("copy") is not False or carriers.get("redact", {}).get("copy") is not False:
        raise Invalid("endpoint journals may be copied by rewrite fixture")
    for name, raw in carriers["source"].items():
        if not raw.endswith("\n"):
            raise Invalid(f"endpoint rewrite source {name} lacks final LF")
        for number, line in enumerate(raw.splitlines(), 1):
            value = load_json(line.encode(), f"endpoint rewrite {name}:{number}")
            if line.encode() != canonical(value):
                raise Invalid(f"endpoint rewrite source {name}:{number} is not canonical")

    model = canonical_contract(FIXTURES / "management-model-selection.canonical.json")
    if (model.get("active_attempt", {}).get("result", {}).get("code") != "session-running" or
            model.get("pending_input", {}).get("result", {}).get("code") != "session-running" or
            model.get("pending_input", {}).get("live_inputs") != 1 or
            model.get("selection", {}).get("lock_held") is not False or
            model.get("selection", {}).get("tail_state") != "settled" or
            model.get("selection", {}).get("live_inputs") != 0 or
            model.get("selection", {}).get("incomplete_operations") != 0 or
            model.get("selection", {}).get("admission_gate") is not True or
            model.get("visibility", {}).get("in_process_mutation") is not False or
            model.get("visibility", {}).get("immediate_prompt", {}).get("model") != model.get("selection", {}).get("model")):
        raise Invalid("session model selection admission race changed")

    attachment_policy = canonical_contract(
        FIXTURES / "management-attachment-policy.canonical.json"
    )
    if attachment_policy != {
            "format": 1,
            "attachment_response_name": "omit",
            "prompt_name": "durable-image-block",
            "limits": {"decoded_bytes": 8_388_608, "height": 16_384,
                       "pixels": 67_108_864, "width": 16_384},
            "reasons": ["INVALID_BASE64", "MEDIA_TYPE_MISMATCH", "TOO_LARGE",
                        "INVALID_DIMENSIONS", "DECODE_FAILED", "NOT_REFERENCED",
                        "CORRUPT", "QUEUE_EDIT_NON_TEXT"],
            "validation_order": ["base64", "decoded-bytes", "media-signature",
                                 "bounded-decode", "dimensions-pixels",
                                 "asset-publication"]}:
        raise Invalid("attachment limits/reason precedence changed")
    attachment_value = [
        decoded_http(record, "management/session-attachment")
        for record in transcript(FIXTURES / "management/session-attachment.response.raw")
        if record["kind"] == "http-response"
    ][0]["result"]["value"]
    if ("name" in attachment_value.get("attachment", {}) or
            set(attachment_value.get("attachment", {})) != {
                "attachmentId", "mediaType", "bytes", "width", "height"}):
        raise Invalid("native attachment response exposes an unsupported name")

    time_zone = canonical_contract(FIXTURES / "management-time-zone.canonical.json")
    if (time_zone.get("accepted", {}).get("projection_metadata") !=
            {"client_time_zone": "Asia/Shanghai"} or
            time_zone.get("accepted", {}).get("public_projection") != "omitted" or
            time_zone.get("accepted", {}).get("rewrite_destination") != "omitted" or
            time_zone.get("rejected") != ["", " Asia/Shanghai", "Asia/Shanghai ",
                                          "GMT+8", "Mars/Base"] or
            time_zone.get("worker_replay_visible") is not False):
        raise Invalid("clientTimeZone projection-only retention changed")

    summary = canonical_contract(FIXTURES / "management-session-summary.canonical.json")
    summary_cases = {case.get("name"): case for case in summary.get("cases", [])}
    if set(summary_cases) != {"native-titled", "fork-running-blank"}:
        raise Invalid("session summary derivation cases changed")
    native = summary_cases["native-titled"]
    fork = summary_cases["fork-running-blank"]
    if native["summary"].get("blank") is not False or native["summary"].get("running") is not False:
        raise Invalid("native summary blank/running derivation changed")
    if (fork["summary"].get("blank") is not True or fork["summary"].get("running") is not True or
            fork["summary"].get("parentSessionId") != fork["authority"].get("fork_source") or
            fork["summary"].get("origin") != "fork"):
        raise Invalid("fork summary lineage derivation changed")

    catalog = canonical_contract(FIXTURES / "management-model-catalog.canonical.json")
    projected = catalog.get("projected")
    if not isinstance(projected, dict) or projected.get("routable") is not True:
        raise Invalid("native model catalog projection is not routable")
    for group in projected.get("groups", []):
        if group.get("id") != group.get("name"):
            raise Invalid("native provider display name is guessed")
        for model_row in group.get("models", []):
            if model_row.get("id") != model_row.get("name") or "description" in model_row:
                raise Invalid("native model display fields are guessed")
            for effort in model_row.get("reasoning", {}).get("efforts", []):
                if effort.get("id") != effort.get("name"):
                    raise Invalid("native effort display name is guessed")
    model_response = [decoded_http(record, "management/session-models") for record in transcript(FIXTURES / "management/session-models.response.raw") if record["kind"] == "http-response"][0]
    if model_response.get("result") != {"ok": True, "value": projected}:
        raise Invalid("native model projection fixture differs from route response")

    list_response = [decoded_http(record, "management/session-list") for record in transcript(FIXTURES / "management/session-list.response.raw") if record["kind"] == "http-response"][0]
    if list_response.get("result", {}).get("value", {}).get("items", [None])[0] != native["summary"]:
        raise Invalid("session summary fixture differs from list response")

    rpc_carrier = canonical_contract(FIXTURES / "management-rpc-carrier.canonical.json")
    rows = []
    for number, row_text in enumerate(rpc_carrier.get("records", []), 1):
        row = load_json(row_text.encode(), f"rpc carrier:{number}")
        if row_text.encode() != canonical(row):
            raise Invalid("rpc carrier embeds a noncanonical row")
        rows.append(row)
    if [row.get("ordinal") for row in rows] != [0, 1, 2] or [row.get("phase") for row in rows] != ["prepared", "handed-off", "complete"]:
        raise Invalid("rpc carrier phase order changed")
    fixed = {(row.get("rpc_id"), row.get("operation"), row.get("request_sha256"), row.get("target_session")) for row in rows}
    if len(fixed) != 1 or "response_b64" not in rows[-1] or "durable_identity" not in rows[-1]:
        raise Invalid("rpc carrier correlation/commit record changed")
    response = load_json(decode_b64(rows[-1]["response_b64"], "rpc carrier response"), "rpc carrier response")
    if response != {"type": "server-response", "rpcId": "rpc-prompt-1", "result": {"ok": True, "value": {"accepted": True}}}:
        raise Invalid("rpc carrier exact response bytes changed")

    authored = canonical_contract(FIXTURES / "management-respond-authoring.canonical.json")
    question, event = authored.get("question", {}), authored.get("event", {})
    if (event.get("kind") != "approval_response" or event.get("call") != question.get("held_call") or
            event.get("grant") is not True or event.get("answer") != question.get("value", {}).get("answer") or
            authored.get("scope") != "omitted" or "scope" in event):
        raise Invalid("question response durable authoring changed")
    if authored.get("delivery_paths") != {
            "live": "worker-control-barrier-receipt",
            "parked": "supervisor-locked-append-barrier-receipt"}:
        raise Invalid("question response live/parked author paths changed")
    origin = event.get("origin_tuple", {})
    expected_key = question.get("rpc_id", "") + "/response"
    if (event.get("origin_key") != expected_key or origin != {"principal": "uid:501", "client": "session-endpoint",
            "target": question.get("value", {}).get("sessionId"), "op": "respond", "key": expected_key}):
        raise Invalid("question response origin tuple changed")

    rejected = canonical_contract(FIXTURES / "management-respond-rejections.canonical.json")
    expected_reasons = ["unknown-rpc-id", "already-resolved", "response-type-mismatch",
                        "session-mismatch", "archived", "stop-active"]
    rejection_rows = rejected.get("rejections", [])
    if ([row.get("reason") for row in rejection_rows] != expected_reasons or
            any(row.get("durable_rpc_carrier") != "absent" or
                row.get("mutation_identity_consumed") is not False or
                not str(row.get("retry", "")).startswith("permitted")
                for row in rejection_rows) or
            rejected.get("success") != {"cache_after": "approval_response-barrier",
                                        "durable_rpc_carrier": "complete",
                                        "response": {"accepted": True}}):
        raise Invalid("respond rejection no-cache contract changed")

    control_races = canonical_contract(FIXTURES / "management-control-races.canonical.json")
    if control_races != {
            "format": 1,
            "admission": {
                "lock_order": ["admission", "management-operation", "line"],
                "operations": ["session.prompt", "respond", "session.cancel",
                               "session.updateQueue", "session.selectModel",
                               "ensure-running", "worker-spawn"],
                "cancel_after_prepared": {"volatile_stop_gate": True,
                                           "durable_stop": "after-queue-transaction",
                                           "ordinary_work": False}},
            "queue_rejection": {"events": [], "gate_after": "released",
                                "operation_phase": "complete",
                                "queue_snapshot": "unchanged",
                                "rpc_phase": "complete-error"},
            "respond_recovery_needed": {"authoring": "held-no-append",
                                        "redelivery": "only-if-unresolved"},
            "respond_stop_active": {"event": "none",
                                    "eventual_resolution": "cancelled",
                                    "response": {"accepted": False,
                                                 "reason": "stop-active"},
                                    "rpc_identity": "unconsumed"}}:
        raise Invalid("management control-race oracle changed")
    if ('"archived"|"stop-active"' not in spec or
            not re.search(r"admission\s+gate → management-operation lock →\s+line/lifecycle lock", spec)):
        raise Invalid("management contract omits stop-active or shared lock order")

    retired = canonical_contract(FIXTURES / "management-redact-authorities.canonical.json")
    forbidden = retired.get("forbidden")
    source_rpc = load_json(retired.get("source", {}).get("rpc", "{}").encode(), "source rpc")
    source_rpc_response = decode_b64(source_rpc.get("response_b64"), "source rpc response").decode()
    if (not isinstance(forbidden, str) or
            forbidden not in retired.get("source", {}).get("operation", "") or
            forbidden not in source_rpc_response):
        raise Invalid("redact authority source lacks forbidden value")
    if any(forbidden in value for value in retired.get("destination", {}).values()):
        raise Invalid("redact authority destination retains forbidden value")
    retired_rpc = load_json(retired["destination"]["rpc"].encode(), "retired rpc")
    retired_op = load_json(retired["destination"]["operation"].encode(), "retired operation")
    expected_rpc_hash = hashlib.sha256(source_rpc["rpc_id"].encode()).hexdigest()
    if (retired_rpc.get("phase") != "retired" or retired_rpc.get("retired_reason") != "redact" or
            retired_rpc.get("rpc_sha256") != expected_rpc_hash or "rpc_id" in retired_rpc or
            "response_b64" in retired_rpc or retired_op.get("retired") != "redact" or
            retired_op.get("rpc_sha256") != expected_rpc_hash or "rpc_id" in retired_op or
            retired_op.get("intent", {}).get("kind") != "retired" or "response" in retired_op or
            retired.get("retry", {}).get("code") != "idempotency-conflict"):
        raise Invalid("redact authority retirement semantics changed")


def main() -> int:
    validate_management_contract_and_auxiliary()
    registry_path = FIXTURES / "cases.canonical.json"
    registry_raw = registry_path.read_bytes()
    registry_json = registry_raw[:-1] if registry_raw.endswith(b"\n") else registry_raw
    registry = load_json(registry_json, registry_path)
    if registry_json != canonical(registry):
        raise Invalid("cases.canonical.json is not canonical")
    declared = {(family, case) for family, names in registry.items() if isinstance(names, list) for case in names}
    if len(declared) != 55:
        raise Invalid(f"registry declares {len(declared)} cases, expected 55")
    if set(registry.get("management", [])) != set(MANAGEMENT_ROUTES):
        raise Invalid("management registry and the closed registration table differ")

    wanted = set()
    for family, case in declared:
        for suffix in ("request.raw", "response.raw", "expected.canonical.json"):
            wanted.add(FIXTURES / family / f"{case}.{suffix}")
    actual = {path for path in FIXTURES.glob("*/*") if path.is_file()}
    extras = actual - wanted
    missing = wanted - actual
    if missing or extras:
        raise Invalid(f"registry/disk mismatch missing={sorted(map(str, missing))} extra={sorted(map(str, extras))}")

    for family, case in sorted(declared):
        request_records = transcript(FIXTURES / family / f"{case}.request.raw")
        response_records = transcript(FIXTURES / family / f"{case}.response.raw")
        outcome = validate_expected(FIXTURES / family / f"{case}.expected.canonical.json")
        closes = [record for record in response_records if record["kind"] == "ws-close"]
        http = [record for record in response_records if record["kind"] == "http-response"]
        if outcome["carrier_status"].startswith("ws-closed-"):
            if not closes or outcome["carrier_status"] != f"ws-closed-{closes[-1]['code']}":
                raise Invalid(f"{family}/{case}: expected close does not match transcript")
        if outcome["carrier_status"].startswith("http-"):
            if not http or outcome["carrier_status"] != f"http-{http[-1]['status']}":
                raise Invalid(f"{family}/{case}: expected HTTP status does not match transcript")
        if not request_records and not response_records:
            raise Invalid(f"{family}/{case}: empty case")
        if family == "management":
            validate_management_case(case, request_records, response_records, outcome)
        api_records = [record for record in request_records if record["kind"] in {"http-request", "ws-open"} and record["path"].startswith("/api/")]
        auth_values = [dict(record["headers"]).get("authorization") for record in api_records]
        if (family, case) == ("security", "credential-redaction"):
            if auth_values != [None, "Bearer wrong-token", "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"]:
                raise Invalid("credential-redaction auth matrix changed")
            statuses = [record["status"] for record in response_records if record["kind"] == "http-response"]
            if statuses[:2] != [401, 401] or statuses[-1:] != [200]:
                raise Invalid("credential-redaction 401-before-dispatch matrix changed")
        elif (family, case) == ("security", "api-auth-origin-matrix"):
            expected_auth = [None, "Bearer wrong-token",
                             "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                             None,
                             "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                             "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                             None, "Bearer wrong-token",
                             "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                             "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                             "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"]
            if auth_values != expected_auth:
                raise Invalid("api-auth-origin matrix authentication order changed")
            origins = [dict(record["headers"]).get("origin") for record in api_records]
            statuses = [record["status"] for record in response_records
                        if record["kind"] == "http-response"]
            if (origins != [None, None, None, "https://evil.example",
                            "http://127.0.0.1:8080", "null", None, None,
                            "http://127.0.0.1:8080", "null", None] or
                    statuses != [401, 401, 200, 401, 403, 403, 401, 401, 403, 403]):
                raise Invalid("HTTP/WS authentication and Origin precedence matrix changed")
        elif any(value != "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" for value in auth_values):
            raise Invalid(f"{family}/{case}: authenticated /api carrier missing fixture token")

    attachment_requests = [
        decoded_http(record, "http/attachment-validation")
        for record in transcript(FIXTURES / "http/attachment-validation.request.raw")
        if record["kind"] == "http-request"
    ]
    attachment_responses = transcript(FIXTURES / "http/attachment-validation.response.raw")
    if len(attachment_requests) != 4 or len(attachment_responses) != 4:
        raise Invalid("attachment validation transcript cardinality changed")
    parts = [request["payload"]["content"][0] for request in attachment_requests]
    if (parts[0].get("name") != "pixel.png" or parts[1].get("data") != "@@" or
            base64.b64decode(parts[2]["data"]) != b"not-png" or
            base64.b64decode(parts[3]["data"]) != b"\x89PNG\r\n\x1a\n"):
        raise Invalid("attachment validation inputs no longer isolate name/base64/signature/decode")
    attachment_results = [
        decoded_http(record, "http/attachment-validation")
        for record in attachment_responses if record["kind"] == "http-response"
    ]
    reasons = [value.get("result", {}).get("error", {}).get("details", {}).get("reason")
               for value in attachment_results[1:]]
    if (attachment_responses[0].get("status") != 200 or
            attachment_results[0].get("result", {}).get("value") != {"accepted": True} or
            reasons != ["INVALID_BASE64", "MEDIA_TYPE_MISMATCH", "DECODE_FAILED"]):
        raise Invalid("attachment validation first-failure precedence changed")

    request_rpc_ids, approval_rpc, question_rpc = validate_request_journal()
    mux_frames = [decoded_http({"body_b64": record["body_b64"]}, "management/events-mux") for record in transcript(FIXTURES / "management/events-mux.response.raw") if record["kind"] == "ws-text"]
    mux_rpc_ids = {frame.get("rpcId") for frame in mux_frames}
    if not request_rpc_ids <= mux_rpc_ids:
        raise Invalid("management mux omits derived requested frames")
    resolved_frames = [frame["payload"] for frame in mux_frames
                       if frame.get("payload", {}).get("type") in {
                           "approval/resolved", "question/resolved"}]
    approval_resolutions = {(frame.get("approvalId"), frame.get("outcome"))
                            for frame in resolved_frames
                            if frame.get("type") == "approval/resolved"}
    question_resolutions = {(frame.get("questionRpcId"), frame.get("outcome"))
                            for frame in resolved_frames
                            if frame.get("type") == "question/resolved"}
    if approval_resolutions != {
            ("approval:call-approval-1", "allowed-once"),
            ("approval:call-approval-reject", "rejected"),
            ("approval:call-approval-cancel", "cancelled")} or question_resolutions != {
            (question_rpc, "answered"),
            ("request-63408fb77ccb8d6469fed4dcccf3686e3690dee8d6c43a5ff4f7a3cc37377b67",
             "cancelled")}:
        raise Invalid("management mux request-resolution union coverage changed")
    queue_frames = [frame for frame in mux_frames if frame.get("payload", {}).get("type") == "session/queue"]
    by_rpc = {frame.get("rpcId"): frame.get("payload") for frame in queue_frames}
    required_queue = {"push-queue-baseline", "push-queue-empty", "push-queue-hold", "push-queue-hold-reconnect"}
    if not required_queue <= set(by_rpc):
        raise Invalid("management mux queue lifecycle fixtures incomplete")
    baseline_items = by_rpc["push-queue-baseline"]["items"]
    if ([item["id"] for item in baseline_items] != ["input:11", "input:13"] or
            [item["message"]["id"] for item in baseline_items] != ["input-11", "input-13"] or
            any("rpcId" not in item["message"]["source"] for item in baseline_items)):
        raise Invalid("management queue item/message derivation changed")
    if by_rpc["push-queue-empty"]["items"] != [] or by_rpc["push-queue-hold"]["items"] != by_rpc["push-queue-hold-reconnect"]["items"]:
        raise Invalid("management queue empty/reconnect semantics changed")
    projections = [frame["payload"] for frame in mux_frames if frame.get("payload", {}).get("type") == "session/projection"]
    if projections != [{"type": "session/projection", "sessionId": "018f0000-0000-7000-8000-000000000003",
                        "seq": 6, "name": "sessionTitle", "value": {"title": "Greeting"}}]:
        raise Invalid("management projection causal endpoint seq changed")
    respond_requests = [decoded_http(record, "management/respond") for record in transcript(FIXTURES / "management/respond.request.raw") if record["kind"] == "http-request"]
    if not {approval_rpc, question_rpc} <= {request.get("rpcId") for request in respond_requests}:
        raise Invalid("management respond does not echo derived request rpcIds")

    malformed_requests = [decoded_http(record, "http/malformed") for record in transcript(FIXTURES / "http/malformed.request.raw") if record["kind"] == "http-request" and decode_b64(record["body_b64"], "http/malformed") != b"{"]
    if len(malformed_requests) != 1 or not malformed_requests[0].get("rpcId", "").startswith("request-"):
        raise Invalid("reserved server request rpcId namespace fixture missing")

    # Imported values are wrapped without mutation.
    authority_host = load_json((ROOT / "fixtures/endpoint/authority/session-inventory.json").read_bytes(), "authority host")["host"]
    authority_history = load_json((ROOT / "fixtures/endpoint/authority/session-history.json").read_bytes(), "authority history")
    for case, expected_value in (("host-describe", authority_host), ("history", authority_history)):
        records = transcript(FIXTURES / f"http/{case}.response.raw")
        body = decode_b64(records[-1]["body_b64"], case)
        envelope = load_json(body, case)
        if envelope.get("result") != {"ok": True, "value": expected_value}:
            raise Invalid(f"http/{case}: imported authority value changed")

    # Respond is the Client-owned bare receipt, not a server-response wrapper.
    respond_records = transcript(FIXTURES / "http/respond.response.raw")
    respond_body = load_json(decode_b64(respond_records[-1]["body_b64"], "respond"), "respond")
    if respond_body != {"accepted": True}:
        raise Invalid("http/respond: response is not the accepted Client receipt")

    # Archive and unarchive are distinct methods and paths in both directions.
    archive_raw = (FIXTURES / "http/archive.request.raw").read_bytes()
    unarchive_raw = (FIXTURES / "http/unarchive.request.raw").read_bytes()
    if b"workspace.archiveSession" not in base64.b64decode(transcript(FIXTURES / "http/archive.request.raw")[-1]["body_b64"]) or b"workspace.unarchiveSession" not in base64.b64decode(transcript(FIXTURES / "http/unarchive.request.raw")[-1]["body_b64"]):
        raise Invalid("archive/unarchive method distinction missing")
    if archive_raw == unarchive_raw:
        raise Invalid("archive and unarchive transcripts are identical")

    secret_response = (FIXTURES / "security/credential-redaction.response.raw").read_bytes()
    for forbidden in (b"secret-provider-key", b"secret-prompt", b"authorization", b"token="):
        if forbidden in secret_response:
            raise Invalid(f"credential-redaction response leaks {forbidden!r}")

    coverage = (FIXTURES / "COVERAGE.md").read_text()
    for gate in range(65, 71):
        if f"Gate {gate}" not in coverage:
            raise Invalid(f"COVERAGE.md omits Gate {gate}")
    for family, case in declared:
        if f"`{family}/{case}`" not in coverage:
            raise Invalid(f"COVERAGE.md omits {family}/{case}")

    print(f"endpoint transport fixtures: {len(declared)} cases, {len(wanted)} artifacts, {len(MANAGEMENT_ROUTES)} management routes, Gates 65-70 covered")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Invalid as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1)
