#!/usr/bin/env python3
"""Fail closed when the Slice-14F contract, oracle, or inventory drifts."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures" / "client-extensions"
MANIFEST = ROOT / "fixtures" / "manifest.json"

EXPECTED_FILES = [
    "README.md",
    "catalog.canonical.json",
    "dispositions.canonical.json",
    "errors.canonical.json",
    "method-cases.canonical.json",
    "negative.canonical.json",
    "value-cases.canonical.json",
]
EXPECTED_CAPABILITIES = {
    "recovery.v1",
    "attachments.v1",
    "approvals.v1",
    "hostFiles.v1",
    "feedback.v1",
    "settings.v1",
    "goals.v1",
    "subagents.v1",
    "sessionFiles.v1",
    "resources.v1",
    "initialPresets.v1",
    "tools.v1",
    "plugins.v1",
    "mcp.v1",
    "schedule.v1",
    "threadSearch.v1",
    "providerAdmin.v1",
    "workspacePolicy.v1",
    "usage.v1",
}


def canonical(value: object) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True
    ).encode("utf-8")


def load(name: str) -> object:
    return json.loads((CORPUS / name).read_text(encoding="utf-8"))


def require_unique(values: list[str], label: str) -> None:
    if len(values) != len(set(values)):
        duplicates = sorted(value for value in set(values) if values.count(value) > 1)
        raise SystemExit(f"{label} contains duplicates: {duplicates}")


def main() -> None:
    actual = sorted(str(path.relative_to(CORPUS)) for path in CORPUS.rglob("*") if path.is_file())
    if actual != EXPECTED_FILES:
        raise SystemExit(f"client-extension corpus mismatch: {actual}")

    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    disk_corpora = {
        corpus.name: sorted(
            str(path.relative_to(corpus)) for path in corpus.rglob("*") if path.is_file()
        )
        for corpus in sorted(path for path in MANIFEST.parent.iterdir() if path.is_dir())
    }
    if manifest != {"corpora": disk_corpora, "version": 1}:
        raise SystemExit("fixture manifest does not match disk in both directions")
    if manifest["corpora"].get("client-extensions") != EXPECTED_FILES:
        raise SystemExit("client-extension manifest entry does not match disk")

    for path in CORPUS.glob("*.canonical.json"):
        raw = path.read_bytes()
        if not raw.endswith(b"\n") or b"\n" in raw[:-1]:
            raise SystemExit(f"{path}: expected one JSON object plus LF")
        value = json.loads(raw)
        if canonical(value) + b"\n" != raw:
            raise SystemExit(f"{path}: noncanonical bytes")

    catalog = load("catalog.canonical.json")
    if catalog["format"] != 1:
        raise SystemExit("catalog format drifted")
    # The base set is the transport's unary route table plus `remote.mux`;
    # conformance gate 113 binds it to the code, this checker only keeps it
    # closed and disjoint from the extension methods.
    base = catalog["base"]
    require_unique(base, "base methods")
    if not base or any(not isinstance(name, str) or not name for name in base):
        raise SystemExit("catalog base is not a closed list of method names")
    capabilities = catalog["capabilities"]
    capability_ids = [capability["id"] for capability in capabilities]
    require_unique(capability_ids, "capability ids")
    if set(capability_ids) != EXPECTED_CAPABILITIES:
        raise SystemExit("extension capability set drifted")
    methods = [method["name"] for capability in capabilities for method in capability["methods"]]
    require_unique(methods, "extension methods")
    if set(methods) & set(base):
        raise SystemExit("extension collides with frozen base")
    if any(method["class"] not in {"read", "mutation"} for capability in capabilities for method in capability["methods"]):
        raise SystemExit("extension has unsupported method class")
    if any("computerUse" in method or "tcu" in method.lower() for method in methods):
        raise SystemExit("product-specific Computer Use method entered the generic catalog")

    cases = load("method-cases.canonical.json")["cases"]
    case_methods = [case["method"] for case in cases]
    require_unique(case_methods, "method cases")
    if set(case_methods) != set(methods):
        raise SystemExit("method cases and catalog differ in either direction")
    for case in cases:
        if set(case) != {"method", "request", "success"}:
            raise SystemExit(f"{case.get('method')}: method case field set is open")
        if not isinstance(case["request"], dict) or not isinstance(case["success"], dict):
            raise SystemExit(f"{case['method']}: request/result must be objects")
        if case["success"] == {}:
            raise SystemExit(f"{case['method']}: fake empty success")

    dispositions = load("dispositions.canonical.json")
    if dispositions["format"] != 1 or len(dispositions["sources"]) != 2:
        raise SystemExit("predecessor source lock drifted")
    legacy = [name for row in dispositions["rows"] for name in row["legacy"]]
    require_unique(legacy, "legacy inventory")
    if len(legacy) != 152:
        raise SystemExit(f"pinned predecessor route count drifted: {len(legacy)}")
    source_revisions = {source["repository"]: source["revision"] for source in dispositions["sources"]}
    if source_revisions != {
        "TekesAppServer": "e4df9dc94e60025d0e8f62bcf98c95241181793a",
        "TekesRuntime": "995d8ffeece88c84502e721c5cf9c30d286a17bd",
    }:
        raise SystemExit("predecessor revisions drifted without a new audit")
    allowed_replacements = set(base) | set(methods)
    referenced_replacements: set[str] = set()
    for row in dispositions["rows"]:
        if row["status"] not in {"base", "implemented", "replaced", "retired"}:
            raise SystemExit("unknown disposition status")
        replacement = row["replacement"]
        if not set(replacement) <= allowed_replacements:
            raise SystemExit(f"unknown replacement: {set(replacement) - allowed_replacements}")
        if row["status"] == "retired":
            if replacement or not row.get("reason"):
                raise SystemExit("retirement needs a reason and no replacement")
        elif not replacement:
            raise SystemExit("non-retired disposition lacks a replacement")
        referenced_replacements.update(replacement)
    if set(methods) - referenced_replacements:
        raise SystemExit(f"new methods lack predecessor disposition: {set(methods) - referenced_replacements}")
    for required_legacy in {
        "conversation/freeze",
        "thread/offload",
        "context.query",
        "computerUse/status",
        "git/status",
        "messageFeedback/put",
        "agentPreset/list",
        "models/profile/save",
        "workspace/capabilities/set",
    }:
        if required_legacy not in legacy:
            raise SystemExit(f"required predecessor route omitted: {required_legacy}")

    negative = load("negative.canonical.json")["cases"]
    expected_negative = {
        "partial-mcp",
        "base-collision",
        "unknown-field",
        "fake-empty-success",
        "generic-family-is-not-proof",
        "missing-route-identity",
        "provider-delete-in-use",
        "profile-delete-in-use",
        "ignored-model-forbidden",
        "ignored-policy-forbidden",
        "tcu-method",
    }
    if {case["id"] for case in negative} != expected_negative:
        raise SystemExit("negative matrix drifted")
    schedule = next(case for case in cases if case["method"] == "schedule.save")
    definition = schedule["request"]["definition"]
    if definition["permission_mode"] != "inherit" or "model_id" in definition:
        raise SystemExit("default schedule case does not prove applied default authority")
    provider = next(case for case in cases if case["method"] == "providers.profile.save")
    if not provider["request"]["profile"].get("modelProfileId"):
        raise SystemExit("provider profile lacks exact proof binding")
    if not provider["request"]["profile"].get("exactSku"):
        raise SystemExit("provider profile lacks exact model SKU")
    connection = next(
        case for case in cases if case["method"] == "providers.connection.save"
    )["request"]["connection"]
    required_connection_identity = {
        "protocolFamily",
        "dialectId",
        "endpointOwner",
        "gatewayTranslation",
        "evidenceRevision",
    }
    if not required_connection_identity <= set(connection):
        raise SystemExit("provider connection lacks durable exact-route identity")

    inspect = next(case for case in cases if case["method"] == "plugin/inspect")
    install = next(case for case in cases if case["method"] == "plugin/install")
    if "packageDigest" in inspect["request"]:
        raise SystemExit("plugin inspect discovers packageDigest; request is circular")
    if not install["request"].get("packageDigest"):
        raise SystemExit("plugin install lacks the inspected immutable package digest")
    approvals = next(case for case in cases if case["method"] == "approvals.policy")
    if [option["value"] for option in approvals["success"]["threadLevel"]] != [
        "read-only",
        "workspace-write",
        "danger-full-access",
    ] or approvals["success"]["serverLevel"] != {"options": [], "currentValue": None}:
        raise SystemExit("approval policy does not publish the three per-session modes at thread level")
    selected = next(case for case in cases if case["method"] == "approvals.select")
    if selected["request"]["mode"] != selected["success"]["mode"]:
        raise SystemExit("approvals.select case does not echo the selected mode")
    if "sessionId" not in next(case for case in cases if case["method"] == "approvals.mode")["request"]:
        raise SystemExit("approvals.mode case is not session scoped")
    fixed_tool = next(case for case in cases if case["method"] == "tools/resolve")
    if fixed_tool["success"]["tool"]["availability"] != "default":
        raise SystemExit("fixed tool availability diverges from builtin-tools")
    proof_list = next(case for case in cases if case["method"] == "providers.list")
    if proof_list["success"]["proofs"][0]["proofId"] != "proof-openai_responses_v1":
        raise SystemExit("provider list does not retain proof-oracle identity")
    cache = next(case for case in cases if case["method"] == "usage.cacheAttribution")
    entries = cache["success"]["entries"]
    if not entries or set(entries[0]) != {
        "target",
        "attempts",
        "inputTokens",
        "outputTokens",
        "cacheReadTokens",
        "cost",
    }:
        raise SystemExit("cache attribution entry shape is not executable")

    config = json.loads(
        (ROOT / "fixtures" / "config" / "providers.canonical.json").read_text(
            encoding="utf-8"
        )
    )
    configured = config["providers"][0]
    required_provider_identity = {
        "adapter",
        "dialect",
        "endpoint_owner",
        "gateway_translation",
        "evidence_revision",
    }
    if not required_provider_identity <= set(configured):
        raise SystemExit("config provider fixture lacks durable exact-route identity")
    if any(not {"profile", "id"} <= set(model) for model in configured["models"]):
        raise SystemExit("config model fixture lacks durable exact-profile identity")

    values = load("value-cases.canonical.json")
    if {reference["kind"] for reference in values["resourceReferences"]} != {
        "skill",
        "mcp",
    }:
        raise SystemExit("resource reference union coverage drifted")
    if {"text" in content for content in values["resourceContents"]} != {True, False}:
        raise SystemExit("resource content union coverage drifted")
    if {source["kind"] for source in values["toolSources"]} != {
        "builtin",
        "plugin",
        "mcp",
    }:
        raise SystemExit("tool source union coverage drifted")
    if {readiness["state"] for readiness in values["pluginReadiness"]} != {
        "ready",
        "refresh-failed",
    }:
        raise SystemExit("plugin readiness union coverage drifted")
    provider_unavailable = {
        readiness["reason"]
        for readiness in values["providerReadiness"]
        if readiness["state"] == "unavailable"
    }
    if provider_unavailable != {
        "dialect-unproved",
        "credential-unavailable",
        "route-mismatch",
    }:
        raise SystemExit("provider readiness union coverage drifted")
    connection_cases = {
        case["models"]: case["readiness"] for case in values["connectionReadinessCases"]
    }
    if connection_cases.get(0) != {"state": "ready"} or connection_cases.get(2) != {
        "state": "ready"
    }:
        raise SystemExit("connection readiness incorrectly depends on model cardinality")
    disabled_profile = next(
        case for case in values["profileReadinessCases"] if case["enabled"] is False
    )
    if disabled_profile["readiness"] != {"state": "ready"}:
        raise SystemExit("profile enabled state incorrectly changes proof readiness")
    policy_replacement = values["workspacePolicyReplacement"]
    if (
        policy_replacement["previous"]["network"] is not False
        or policy_replacement["replacement"]["network"] is not True
        or policy_replacement["expected"] != "accepted"
    ):
        raise SystemExit("workspace policy replacement lost valid local relaxation")

    print(
        f"client-extension fixtures: {len(methods)} methods in "
        f"{len(capabilities)} atomic capabilities; {len(legacy)} legacy routes disposed"
    )


if __name__ == "__main__":
    main()
