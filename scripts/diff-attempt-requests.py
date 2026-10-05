#!/usr/bin/env python3
"""Pairwise diff of consecutive provider request bodies in one thread ledger.

Each `attempt` names its exact transmitted body as a thread asset. For every
consecutive pair this reports whether request N+1 is an append-extension of
request N (the shape a prompt cache reads back), and when it is not, which
part moved: the system prompt, the tool catalog, or the conversation items.

Usage: diff-attempt-requests.py <thread folder>
"""
from __future__ import annotations

import json
import sys
from pathlib import Path


def load(folder: Path):
    ledger = folder / "main.jsonl"
    rows = [json.loads(line) for line in ledger.read_text().splitlines() if line.strip()]
    assets = folder / "assets"
    attempts = []
    for row in rows:
        if row["kind"] != "attempt" or "request" not in row:
            continue
        blob = assets / row["request"]["asset"]
        body = json.loads(blob.read_bytes()) if blob.exists() else None
        attempts.append((row["seq"], row["attempt"], row["epoch"], row["request"]["bytes"], body))
    epochs = {r["id"]: r for r in rows if r["kind"] == "epoch"}
    usage = {}
    for row in rows:
        if row["kind"] in ("output", "error") and row.get("usage", {}).get("availability") == "reported":
            usage[row.get("attempt")] = dict(row["usage"], _adapter=row.get("sealed", {}).get("adapter"))
    return rows, attempts, epochs, usage


def continuation_of(body: dict):
    """A server-managed request carries the conversation by identity, not by
    replay: its body is a delta, so pairwise prefix comparison is meaningless
    there and the chain identity is what matters instead."""
    for key in ("previous_response_id", "previous_interaction_id"):
        if body.get(key):
            return body[key]
    return None


def cache_counts(usage: dict) -> tuple[int, int, int]:
    read = int(usage.get("cache_read", "0") or 0)
    write = int(usage.get("cache_write", "0") or 0)
    total = int(usage.get("input_tokens", "0") or 0)
    # Anthropic's input excludes cache reads even when read < input. Prefer
    # the persisted adapter over inferring accounting from counter magnitudes.
    disjoint = usage.get("_adapter") in ("anthropic_messages_v1", "deepseek_anthropic_v1")
    if usage.get("_adapter") is None:
        disjoint = write > 0 or read > total
    return read, write, total + read + write if disjoint else total


def items_of(body: dict) -> list:
    for key in ("input", "messages", "contents"):
        if isinstance(body.get(key), list):
            return body[key]
    return []


def head_of(body: dict) -> dict:
    return {
        "system": body.get("instructions") or body.get("system") or body.get("system_instruction"),
        "tools": body.get("tools"),
        "model": body.get("model"),
    }


def without_markers(value):
    """Strip cache_control before comparing. An Anthropic breakpoint is meant
    to move forward with the conversation, so the marker's own position
    differs between two requests that are otherwise a clean extension."""
    if isinstance(value, dict):
        return {k: without_markers(v) for k, v in value.items() if k != "cache_control"}
    if isinstance(value, list):
        return [without_markers(v) for v in value]
    return value


def canon(value) -> str:
    return json.dumps(
        without_markers(value), sort_keys=True, separators=(",", ":"), ensure_ascii=False
    )


def common_prefix(left: list, right: list) -> int:
    n = 0
    while n < len(left) and n < len(right) and canon(left[n]) == canon(right[n]):
        n += 1
    return n


def main() -> int:
    folder = Path(sys.argv[1])
    rows, attempts, epochs, usage = load(folder)
    print(f"{len(attempts)} attempts with a stored body; {len(epochs)} epoch(s)")
    for eid, e in epochs.items():
        print(f"  epoch {e['seq']:>4} reason={e['reason']!r} id={eid}")
    print()
    print(f"{'pair':>9} {'bytes N->N+1':>16} {'items':>11} {'prefix':>7}  verdict")
    extensions = 0
    for (sa, ida, ea, ba, a), (sb, idb, eb, bb, b) in zip(attempts, attempts[1:]):
        if a is None or b is None:
            print(f"{sa:>4}->{sb:<4} (body missing)")
            continue
        ia, ib = items_of(a), items_of(b)
        shared = common_prefix(ia, ib)
        head_same = canon(head_of(a)) == canon(head_of(b))
        chain = continuation_of(b)
        if chain:
            u = usage.get(idb, {})
            print(f"{sa:>4}->{sb:<4} {ba:>7}->{bb:<7} {len(ia):>4}->{len(ib):<4} {'-':>7}  server-managed delta on {chain[:18]}")
            extensions += 1
            continue
        if head_same and shared == len(ia) and len(ib) >= len(ia):
            verdict = f"append-extension (+{len(ib) - len(ia)} items)"
            extensions += 1
        else:
            moved = []
            if not head_same:
                for part in ("system", "tools", "model"):
                    if canon(head_of(a)[part]) != canon(head_of(b)[part]):
                        moved.append(part)
            if shared < len(ia):
                moved.append(f"items[{shared}] changed (was {len(ia)}, now {len(ib)})")
            verdict = "PREFIX BROKE: " + ", ".join(moved)
            if ea != eb:
                verdict += f"  [epoch {ea[-12:]} -> {eb[-12:]}: {epochs.get(eb, {}).get('reason')}]"
        u = usage.get(idb, {})
        rate = ""
        if u.get("input_tokens"):
            read, _, total = cache_counts(u)
            rate = f"{read / total * 100:5.1f}%" if total else ""
        print(f"{sa:>4}->{sb:<4} {ba:>7}->{bb:<7} {len(ia):>4}->{len(ib):<4} {shared:>7}  {verdict} {rate}")
    print()
    print(f"append-extensions (or unbroken server chains): {extensions}/{max(len(attempts) - 1, 0)}")
    reported = list(usage.values())
    counts = [cache_counts(u) for u in reported]
    read = sum(c[0] for c in counts)
    write = sum(c[1] for c in counts)
    denominator = sum(c[2] for c in counts)
    if denominator:
        print(
            f"cache read share over {len(reported)} settled attempts: "
            f"{read}/{denominator} = {read / denominator * 100:.1f}%"
            + (f"  (cache_write {write})" if write else "")
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
