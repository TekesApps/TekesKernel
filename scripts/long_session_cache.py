"""Deterministic, tool-free prompts for a single-session cache-continuity probe."""

import hashlib


def prompts():
    # High-entropy rows prevent a provider from satisfying the probe by
    # compressing a repetitive synthetic prefix. Both runners import this list.
    facts = []
    for index in range(72):
        digest = hashlib.sha256(f"tekes-long-session-cache-v1:{index:03d}".encode()).hexdigest()
        facts.append(f"Ledger row {index:03d}: {digest[:32]} {digest[32:]}")
    out = [
        "This is a cache-continuity measurement in one conversation. Do not call tools. "
        "Keep the ledger below as conversation context; answer this turn with exactly "
        "CACHE_PROBE_1_OK and no other text.\n\n" + "\n".join(facts)
    ]
    for turn in range(2, 9):
        digest = hashlib.sha256(f"tekes-long-session-cache-v1:turn:{turn}".encode()).hexdigest()
        out.append(
            f"Continue this same conversation. Do not call tools. New turn fact {turn}: {digest}. "
            f"Answer with exactly CACHE_PROBE_{turn}_OK and no other text."
        )
    return out


def large_prefix_prompts():
    """Three fixed-output turns whose first request crosses the old 900 kB preflight."""
    rows = (
        f"Ledger row {index:05d}: "
        + hashlib.sha256(f"tekes-large-prefix-v1:{index:05d}".encode()).hexdigest()
        for index in range(11_000)
    )
    return [
        "Cache-continuity probe. Do not call tools. Keep this ledger in the conversation. "
        "Reply with exactly LARGE_PREFIX_1_OK and no other text.\n\n"
        + "\n".join(rows)
        + "\n\nEnd of ledger. Reply with exactly LARGE_PREFIX_1_OK and no other text.",
        "Continue the same conversation. Do not call tools. Reply with exactly LARGE_PREFIX_2_OK and no other text.",
        "Continue the same conversation. Do not call tools. Reply with exactly LARGE_PREFIX_3_OK and no other text.",
    ]
