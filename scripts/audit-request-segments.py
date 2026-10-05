#!/usr/bin/env python3
"""Measure saved Kernel provider requests without printing their contents."""

import argparse
from collections import Counter
import json
from pathlib import Path


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("run", type=Path, help="run-public-flow output directory")
parser.add_argument("--output", type=Path)
args = parser.parse_args()


def size(value):
    return len(json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode())


ledgers = list((args.run / "runtime" / "threads").glob("*/main.jsonl"))
if len(ledgers) != 1:
    parser.error(f"expected one root ledger, found {len(ledgers)}")
ledger = ledgers[0]
rows = [json.loads(line) for line in ledger.read_text().splitlines() if line.strip()]
requests = []
previous_body = None
for row in rows:
    if row.get("kind") != "attempt" or not row.get("request"):
        continue
    request = row["request"]
    body = json.loads((ledger.parent / "assets" / request["asset"]).read_text())
    stable_prefix = None if previous_body is None else (
        all(previous_body.get(key) == body.get(key)
            for key in ("instructions", "tools", "model", "reasoning"))
        and body.get("input", [])[:len(previous_body.get("input", []))]
        == previous_body.get("input", []))
    parts = Counter()
    counts = Counter()
    parts["instructions"] = size(body.get("instructions"))
    parts["tools"] = size(body.get("tools", []))
    for item in body.get("input", []):
        kind = item.get("type", "unknown")
        if kind == "message":
            kind += ":" + str(item.get("role", "unknown"))
        parts[kind] += size(item)
        counts[kind] += 1
    requests.append({"turn": row["turn"], "attempt": row["attempt"],
                     "wire_bytes": request["bytes"], "parts_bytes": dict(parts),
                     "counts": dict(counts), "stable_prefix_from_previous": stable_prefix})
    previous_body = body

turns = {}
for request in requests:
    turns.setdefault(str(request["turn"]), []).append(request)
summary = {turn: {"requests": len(items), "first": items[0], "last": items[-1]}
           for turn, items in turns.items()}
report = {"source": str(ledger), "request_count": len(requests),
          "stable_prefix_transitions": sum(item["stable_prefix_from_previous"] is True for item in requests),
          "unstable_prefix_transitions": sum(item["stable_prefix_from_previous"] is False for item in requests),
          "turns": summary,
          "requests": requests}
if args.output:
    args.output.write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps({"request_count": len(requests),
                  "stable_prefix_transitions": report["stable_prefix_transitions"],
                  "unstable_prefix_transitions": report["unstable_prefix_transitions"],
                  "turns": summary}, indent=2))
