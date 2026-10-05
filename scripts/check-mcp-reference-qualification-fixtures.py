#!/usr/bin/env python3
"""Validate the language-neutral Slice-14A standalone-MCP qualification corpus."""

import hashlib
import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "fixtures" / "mcp-reference-qualification"
FIRST_PARTY = ROOT / "fixtures" / "tools" / "first-party-tools.canonical.json"
MANIFEST = ROOT / "fixtures" / "manifest.json"
SOURCE_REVISION = "918c32cba837f4131a1bc8457b8a58895dd7e723"
PRODUCTION_SIGNING_IDENTIFIER = "com.tekes.computer-use.helper"
# Placeholder signing identity. The real designated requirement is supplied to
# the live gate through TEKES_SLICE14A_REFERENCE_REQUIREMENT, never committed.
PRODUCTION_TEAM_ID = "TEKESAPP01"
PRODUCTION_DESIGNATED_REQUIREMENT = (
    'identifier "com.tekes.computer-use.helper" and anchor apple generic and '
    'certificate leaf[subject.CN] = "Apple Development: developer@example.test (DEVCERT001)" '
    'and certificate 1[field.1.2.840.113635.100.6.2.1] /* exists */'
)
PROJECTED_CATALOG_SHA256 = "7b015a1224bf598e50c4a81868d22c6553af88176ee46a6d618880597ac96ad5"


def canonical(path: Path):
    raw = path.read_bytes()
    value = json.loads(raw)
    expected = json.dumps(
        value, ensure_ascii=False, allow_nan=False, sort_keys=True, separators=(",", ":")
    ).encode() + b"\n"
    if raw != expected:
        raise SystemExit(f"noncanonical Slice-14A fixture: {path}")
    return value


def main():
    contract = canonical(CORPUS / "contract.canonical.json")
    cases = canonical(CORPUS / "cases.canonical.json")
    source_lock = canonical(CORPUS / "source-lock.canonical.json")
    corpus = json.loads(MANIFEST.read_bytes())["corpora"]
    actual = sorted(
        str(path.relative_to(CORPUS)) for path in CORPUS.rglob("*") if path.is_file()
    )
    if corpus.get("mcp-reference-qualification") != actual:
        raise SystemExit("Slice-14A fixture manifest drift")
    expected_gates = {
        "117": "generic-local-mcp-configuration-and-signed-executable",
        "118": "twelve-tool-discovery-and-call",
        "119": "hermetic-tcc-denial-grant-and-prompt-suppression",
        "120": "stop-restart-crash-recovery-and-no-special-cases",
    }
    if contract.get("format") != 1 or contract.get("gates") != expected_gates:
        raise SystemExit("Slice-14A gate contract drift")
    if contract.get("operations") != [
        "configure", "enable", "start", "discover", "call", "stop", "restart", "recover"
    ]:
        raise SystemExit("Slice-14A lifecycle operations drift")
    if contract.get("real_uat") != {
        "denial_canary": {
            "bundle_identity": "distinct-from-production",
            "distribution": "forbidden",
            "executable_env": "TEKES_SLICE14A_DENIED_REFERENCE_EXECUTABLE",
            "permission_request": "forbidden",
            "qualification_only": True,
            "runtime_manifest": "forbidden",
        },
        "executable_envs": [
            "TEKES_SLICE14A_REFERENCE_EXECUTABLE",
            "TEKES_SLICE14A_DENIED_REFERENCE_EXECUTABLE",
        ],
        "requires": [
            "macos",
            "arm64",
            "strict-codesign",
            "stable-designated-requirement",
            "launchservices-proxy",
        ],
        "revision_env": "TEKES_SLICE14A_REFERENCE_REVISION",
        "source_lock": "source-lock.canonical.json",
        "status_without_input": "not-qualified",
    }:
        raise SystemExit("real reference and qualification-only denial-canary contract drift")
    reference = contract.get("reference", {})
    if reference != {
        "server_name": "computer-use",
        "transport": "stdio",
        "version": "0.1.6",
    }:
        raise SystemExit("standalone MCP reference identity drift")
    first_party = canonical(FIRST_PARTY)
    expected_source_lock = {
        "format": 1,
        "executable": {
            "production_signing": {
                "designated_requirement": PRODUCTION_DESIGNATED_REQUIREMENT,
                "identifier": PRODUCTION_SIGNING_IDENTIFIER,
                "team_id": PRODUCTION_TEAM_ID,
            },
            "version": reference["version"],
        },
        "projected_catalog": {"sha256": PROJECTED_CATALOG_SHA256},
        "source": {
            "repository": "TekesComputerUse",
            "revision": SOURCE_REVISION,
        },
        "tool_contract": {
            "path": contract["tool_source"],
            "sha256": hashlib.sha256(FIRST_PARTY.read_bytes()).hexdigest(),
        },
    }
    if source_lock != expected_source_lock:
        raise SystemExit("real reference source-lock drift")
    if not re.fullmatch(r"[0-9a-f]{64}", source_lock["projected_catalog"]["sha256"]):
        raise SystemExit("projected catalog lock must be a lowercase SHA-256")
    tools = first_party.get("plugins", [None])[0]
    # The Slice-4 inventory records the old package identity only as migration
    # evidence. Slice 14A qualifies the unchanged executable through an ordinary
    # standalone MCP row and does not project that package into the runtime.
    if tools is None or tools.get("id") != "com.tekes.computer-use" or tools.get("version") != "0.1.5":
        raise SystemExit("first-party reference inventory identity drift")
    names = [row["name"] for row in tools.get("tools", [])]
    if len(names) != 12 or names != sorted(set(names)):
        raise SystemExit("reference inventory must contain exactly 12 sorted tools")
    executable = (CORPUS / contract["hermetic"]["executable"]).read_text()
    for name in names:
        if f'"{name}"' not in executable:
            raise SystemExit(f"hermetic server omits {name}")
    case_ids = [case["id"] for case in cases.get("cases", [])]
    if len(case_ids) != 8 or len(set(case_ids)) != len(case_ids):
        raise SystemExit("Slice-14A case closure drift")
    forbidden = ("computer-use", "computer_use", "list_apps", "permission_status", "request_permissions")
    for source in ROOT.glob("crates/*/src/**/*.rs"):
        lowered = source.read_text().lower()
        if any(token in lowered for token in forbidden):
            raise SystemExit(f"product-specific reference branch entered Kernel source: {source}")
    print("Slice-14A fixtures: 4 gates, 8 cases, 12 tools, zero Kernel special cases")


if __name__ == "__main__":
    main()
