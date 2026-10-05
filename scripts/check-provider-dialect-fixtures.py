#!/usr/bin/env python3
"""Validate the independent provider-dialect/profile oracle."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures/provider-dialects"
PROFILES = CORPUS / "profiles.canonical.json"
INVALID = CORPUS / "invalid.canonical.json"


def canonical(value: object) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, separators=(",", ":"), sort_keys=True
    ).encode("utf-8")


def load_canonical(path: Path) -> object:
    raw = path.read_bytes()
    if not raw.endswith(b"\n") or raw.endswith(b"\n\n"):
        raise ValueError(f"{path}: expected exactly one final LF")
    value = json.loads(raw)
    expected = canonical(value) + b"\n"
    if raw != expected:
        raise ValueError(f"{path}: bytes are not canonical JSON")
    return value


def validate_sse(raw: str) -> None:
    events = [event for event in raw.replace("\r\n", "\n").split("\n\n") if event]
    assert events
    for event in events:
        lines = event.split("\n")
        data = "\n".join(line[6:] for line in lines if line.startswith("data: "))
        assert data
        if data != "[DONE]":
            json.loads(data)


def request_digest(profile: dict[str, object]) -> str:
    target = profile["target"]
    assert isinstance(target, dict)
    route = target["route"]
    assert isinstance(route, dict)
    family = target["protocol_family"]
    model = route["exact_sku"]
    dialect = target["dialect_id"]
    url = profile["request_url"]
    assert isinstance(url, str) and url
    headers = {"accept": "text/event-stream", "content-type": "application/json"}
    if family == "anthropic_messages":
        headers["anthropic-version"] = "2023-06-01"
    body = str(profile["request_bytes"]).encode()
    preimage = (
        b"tekes-provider-request-v1\0"
        + str(dialect).encode()
        + b"\0POST\0"
        + url.encode()
        + b"\0"
        + str(model).encode()
        + b"\0"
        + canonical(headers)
        + b"\0"
        + body
    )
    return hashlib.sha256(preimage).hexdigest()


def main() -> int:
    try:
        registry = load_canonical(PROFILES)
        invalid = load_canonical(INVALID)
        assert isinstance(registry, dict) and registry.get("format") == 1
        assert isinstance(invalid, dict) and invalid.get("format") == 1
        profiles = registry.get("profiles")
        required = registry.get("required_proof_arms")
        assert isinstance(profiles, list) and len(profiles) == 14
        assert sum(profile.get("advertised") is True for profile in profiles) == 13
        assert isinstance(required, list)
        identities: set[str] = set()
        dialects: set[str] = set()
        families: set[str] = set()
        for profile in profiles:
            assert isinstance(profile, dict)
            target = profile.get("target")
            assert isinstance(target, dict)
            assert isinstance(profile.get("endpoint"), str) and profile["endpoint"]
            assert isinstance(profile.get("request_url"), str) and profile["request_url"]
            credential_header = profile.get("credential_header")
            credential_prefix = profile.get("credential_prefix")
            assert isinstance(credential_header, str) and credential_header
            assert credential_header == credential_header.lower()
            assert all(
                character.isalnum() or character == "-"
                for character in credential_header
            )
            assert isinstance(credential_prefix, str)
            assert "\r" not in credential_prefix and "\n" not in credential_prefix
            route = target.get("route")
            assert isinstance(route, dict)
            for field in (
                "protocol_family",
                "dialect_id",
                "model_profile_id",
            ):
                assert isinstance(target.get(field), str) and target[field]
            for field in (
                "endpoint_owner",
                "gateway_translation",
                "exact_sku",
                "evidence_revision",
            ):
                assert isinstance(route.get(field), str) and route[field]
            identity = canonical(target).decode()
            assert identity not in identities
            identities.add(identity)
            dialects.add(target["dialect_id"])
            families.add(target["protocol_family"])
            if profile.get("advertised") is True:
                assert all(arm in profile for arm in required)
                proof_id = profile.get("proof_id")
                assert isinstance(proof_id, str) and proof_id.startswith(
                    f"proof-{target['dialect_id']}"
                )
            serializer_revision = profile.get("serializer_revision")
            assert isinstance(serializer_revision, str) and serializer_revision
            expected_continuation = (
                "server_managed"
                if target["dialect_id"] in {"openai_responses_v1", "google_interactions_v1"}
                else "stateless_full_history"
            )
            assert profile.get("continuation") == expected_continuation
            for byte_field in (
                "request_bytes", "multiturn_request_bytes", "tool_request_bytes"
            ):
                encoded = profile.get(byte_field)
                assert isinstance(encoded, str)
                parsed = json.loads(encoded)
                if profile["capabilities"]["cache"] == "implicit_prefix":
                    assert encoded.index('"tools"') < encoded.index('"messages"')
                else:
                    assert encoded.encode() == canonical(parsed)
                if profile["capabilities"]["cache"] == "explicit_breakpoint":
                    # Breakpoints ride the last tool and the last block of the
                    # last message; a root-level marker keys the cache on the
                    # whole body and never reads back across turns.
                    assert "cache_control" not in parsed
                    marker = {"type": "ephemeral"}
                    if parsed.get("tools"):
                        assert parsed["tools"][-1]["cache_control"] == marker
                    assert parsed["messages"][-1]["content"][-1]["cache_control"] == marker
            control = profile.get("control")
            assert isinstance(control, dict)
            epoch = control.get("epoch_profile")
            assert epoch.get("serializer_revision") == serializer_revision
            assert control.get("profile_digest") == hashlib.sha256(canonical(epoch)).hexdigest()
            assert control.get("request_digest") == request_digest(profile)
            negative = profile.get("negative")
            assert isinstance(negative, list)
            assert {case.get("case") for case in negative} == {
                "unknown_control", "sealed_dialect_mismatch", "endpoint_mismatch",
                "endpoint_owner_mismatch", "gateway_mismatch", "exact_sku_mismatch",
                "evidence_revision_mismatch", "model_profile_mismatch",
                "protocol_family_mismatch", "schema_weakening", "unsupported_input",
                "unsupported_sampling", "unsupported_tool_choice",
            }
            assert all(case.get("expect") == "pre_send_reject" for case in negative)
            capabilities = profile.get("capabilities")
            assert isinstance(capabilities, dict)
            assert set(capabilities) == {
                "input_blocks", "reasoning", "sampling", "tool_choice",
                "schema", "cache", "repair",
            }
            tool_choice = capabilities["tool_choice"]
            assert isinstance(tool_choice, dict)
            assert set(tool_choice) == {"modes", "wire"}
            assert tool_choice["wire"] in {"implicit", "native"}
            assert isinstance(tool_choice["modes"], list) and tool_choice["modes"]
            assert tool_choice == {"modes": ["auto"], "wire": "implicit"}
            assert capabilities["sampling"] == "omitted"
            for case in profile.get("capability_cases", []):
                assert case.get("controls") == {}
            assert set(tool_choice["modes"]) <= {"auto", "none", "required", "named"}
            if tool_choice["wire"] == "implicit":
                assert tool_choice["modes"] == ["auto"]
            assert capabilities["sampling"] in {
                "omitted", "omitted_while_thinking", "temperature_top_p",
                "temperature_xor_top_p",
            }
            assert capabilities["schema"] in {"strict_object", "validated_object"}
            assert capabilities["cache"] in {
                "none", "server_managed", "implicit_prefix", "explicit_breakpoint",
            }
            repair = capabilities["repair"]
            assert isinstance(repair, dict)
            if repair.get("id") == "deepseek_v4_arguments_v1":
                assert repair.get("max_bytes") == 65536
                assert isinstance(repair.get("input"), str)
                assert isinstance(repair.get("output"), dict)
                cases = repair.get("cases")
                assert isinstance(cases, list) and len(cases) >= 5
                assert all(isinstance(case.get("input"), str) for case in cases)
                assert all(
                    isinstance(case.get("output"), dict) or case.get("reject") is True
                    for case in cases
                )
            else:
                assert repair == {"id": "none"}
            capability_cases = profile.get("capability_cases")
            assert isinstance(capability_cases, list) and capability_cases
            assert len({case.get("id") for case in capability_cases}) == len(capability_cases)
            for case in capability_cases:
                assert isinstance(case.get("controls"), dict)
                outcomes = [
                    name for name in ("expect", "expect_absent", "reject_contains")
                    if name in case
                ]
                assert len(outcomes) == 1
            validate_sse(profile["stream_bytes"])
            validate_sse(profile["tool_stream_bytes"])
        assert dialects == {
            "openai_responses_v1", "deepseek_responses_v1", "generic_chat_v1",
            "openai_chat_v1", "deepseek_chat_v1", "kimi_chat_v1",
            "glm_chat_v1", "ollama_chat_v1", "anthropic_messages_v1",
            "deepseek_anthropic_v1", "google_generation_v1",
            "google_interactions_v1",
        }
        assert families == {
            "responses", "chat_completions", "anthropic_messages",
            "google_generation", "google_interactions",
        }
        deepseek = next(
            value for value in profiles
            if value["target"]["dialect_id"] == "deepseek_responses_v1"
        )
        for body_field in ("request_bytes", "multiturn_request_bytes"):
            body = json.loads(deepseek[body_field])
            assert not ({"previous_response_id", "conversation", "store"} & body.keys())
        for body_field in ("sealed_reasoning_request_bytes", "sealed_tool_request_bytes"):
            body = json.loads(deepseek[body_field])
            assert not ({"previous_response_id", "conversation", "store"} & body.keys())
        reasoning_roundtrip = json.loads(deepseek["sealed_reasoning_request_bytes"])
        assert reasoning_roundtrip["input"][0] == {
            "content": [{"text": "reason", "type": "reasoning_text"}],
            "type": "reasoning",
        }
        tool_roundtrip = json.loads(deepseek["sealed_tool_request_bytes"])
        assert tool_roundtrip["input"][0]["type"] == "function_call"
        assert tool_roundtrip["input"][1]["type"] == "function_call_output"
        assert "response.reasoning_text.delta" in deepseek["stream_bytes"]
        variants = deepseek.get("stream_variants")
        assert isinstance(variants, dict) and set(variants) == {
            "completed", "incomplete", "failed"
        }
        assert "response.completed" in variants["completed"]
        assert "response.incomplete" in variants["incomplete"]
        assert "response.failed" in variants["failed"]
        for raw in variants.values():
            assert "\\n" not in raw
            validate_sse(raw)
        assert "response.incomplete" in deepseek["content_filter_stream_bytes"]
        validate_sse(deepseek["content_filter_stream_bytes"])
        assert deepseek["terminal"]["content_filter"] == {
            "event": "response.incomplete",
            "finish_reason": "content_filter",
            "reason": "content_filter",
            "usage": {"input_tokens": "2", "output_tokens": "0"},
        }
        terminal_cases = deepseek["terminal"].get("cases")
        assert isinstance(terminal_cases, list)
        assert {case.get("event") for case in terminal_cases} == {
            "response.completed", "response.incomplete", "response.failed"
        }
        invalid_cases = invalid.get("cases")
        assert isinstance(invalid_cases, list) and len(invalid_cases) >= 12
        ids = [case.get("id") for case in invalid_cases if isinstance(case, dict)]
        assert len(ids) == len(set(ids))
        manifest = json.loads((ROOT / "fixtures/manifest.json").read_text())
        declared = manifest["corpora"].get("provider-dialects")
        disk = sorted(
            path.relative_to(CORPUS).as_posix()
            for path in CORPUS.rglob("*") if path.is_file()
        )
        assert declared == disk
    except (AssertionError, KeyError, OSError, ValueError, json.JSONDecodeError) as error:
        print(f"provider dialect fixture check failed: {error}", file=sys.stderr)
        return 1
    print("provider dialect fixtures: 13 exact tuples, complete and canonical")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
