#!/usr/bin/env python3
"""Add one exact model target derived from a reviewed route/serializer proof.

All identity and URL inputs are arguments. The tool has no provider, model, router,
or installation defaults; callers decide which proof artifact they are producing.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROFILES = ROOT / "fixtures/provider-dialects/profiles.canonical.json"


def canonical(value: object) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, separators=(",", ":"), sort_keys=True
    ).encode()


def replace_exact(value: object, old: str, new: str) -> object:
    if isinstance(value, str):
        return new if value == old else value
    if isinstance(value, list):
        return [replace_exact(item, old, new) for item in value]
    if isinstance(value, dict):
        return {key: replace_exact(item, old, new) for key, item in value.items()}
    return value


def request_digest(row: dict[str, object], request_url: str) -> str:
    target = row["target"]
    route = target["route"]
    headers = {"accept": "text/event-stream", "content-type": "application/json"}
    if target["protocol_family"] == "anthropic_messages":
        headers["anthropic-version"] = "2023-06-01"
    preimage = b"".join(
        (
            b"tekes-provider-request-v1\0",
            str(target["dialect_id"]).encode(),
            b"\0POST\0",
            request_url.encode(),
            b"\0",
            str(route["exact_sku"]).encode(),
            b"\0",
            canonical(headers),
            b"\0",
            str(row["request_bytes"]).encode(),
        )
    )
    return hashlib.sha256(preimage).hexdigest()


def alias_row(
    source: dict[str, object], proof_id: str, sku: str, profile: str,
    endpoint: str, request_url: str, evidence_revision: str | None,
    credential_header: str, credential_prefix: str
) -> dict[str, object]:
    row = copy.deepcopy(source)
    old_sku = str(row["target"]["route"]["exact_sku"])
    old_profile = str(row["target"]["model_profile_id"])
    row["proof_id"] = proof_id
    row["endpoint"] = endpoint
    row["request_url"] = request_url
    row["credential_header"] = credential_header
    row["credential_prefix"] = credential_prefix
    row["target"]["route"]["exact_sku"] = sku
    row["target"]["model_profile_id"] = profile
    epoch_target = row["control"]["epoch_profile"]["target"]
    epoch_target["route"]["exact_sku"] = sku
    epoch_target["model_profile_id"] = profile
    if evidence_revision is not None:
        row["target"]["route"]["evidence_revision"] = evidence_revision
        epoch_target["route"]["evidence_revision"] = evidence_revision
    for key, value in tuple(row.items()):
        if "request_bytes" not in key or not isinstance(value, str):
            continue
        parsed = replace_exact(json.loads(value), old_sku, sku)
        parsed = replace_exact(parsed, old_profile, profile)
        row[key] = canonical(parsed).decode()
    row["control"]["profile_digest"] = hashlib.sha256(
        canonical(row["control"]["epoch_profile"])
    ).hexdigest()
    row["control"]["request_digest"] = request_digest(row, request_url)
    return row


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-proof", required=True)
    parser.add_argument("--proof-id", required=True)
    parser.add_argument("--sku", required=True)
    parser.add_argument("--profile", required=True)
    parser.add_argument("--endpoint", required=True)
    parser.add_argument("--request-url", required=True)
    parser.add_argument("--evidence-revision")
    parser.add_argument("--credential-header", required=True)
    parser.add_argument("--credential-prefix", required=True)
    return parser.parse_args()


def main() -> int:
    args = arguments()
    registry = json.loads(PROFILES.read_text())
    rows = registry["profiles"]
    source = next(
        (row for row in rows if row.get("proof_id") == args.source_proof), None
    )
    if source is None:
        raise ValueError(f"missing source proof {args.source_proof}")
    rows[:] = [row for row in rows if row.get("proof_id") != args.proof_id]
    rows.append(
        alias_row(
            source, args.proof_id, args.sku, args.profile,
            args.endpoint, args.request_url, args.evidence_revision,
            args.credential_header, args.credential_prefix
        )
    )
    PROFILES.write_bytes(canonical(registry) + b"\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
