# Shared general rules: Kernel A/B on DeepSeek Flash (October 2026)

[Documentation home](../README.md) · [Reproduction kit](../../scripts/bench/README.md) · [Run data](data/2026-10-general-rules/) · [System prompt](../runtime/system-prompt.md)

## Summary

The system prompt was split into an identity line, shared general rules and
profile-specific rules. Two of the four coding rules (stop and report when the
work is done; be concise, no narration before tool calls) became general rules
for every profile. Before the split, general sessions had only their identity
line.

Same corpus, same model, two builds (6 interleaved runs per arm per corpus):

| Corpus | Profile | Before | After | Change | p |
|---|---|---:|---:|---:|---:|
| research-brief (non-coding) | general | $0.00377 | $0.00276 | **−27%** | 0.0065 |
| batch-ledger-bugfix | coding | $0.0112 | $0.0121 | +8% | 0.28 |
| weighted-ttl | coding | $0.0218 | $0.0172 | −21% | 0.15 |

1. **General sessions got cheaper.** On the non-coding corpus, output tokens fell
   by 33% and reasoning tokens by 50%, and every run on both sides passed the
   hidden acceptance check.
2. **Coding sessions did not regress in cost.** Neither coding corpus shows a
   significant difference. The coding text is the same set of rules in a new
   order, and one rule was reworded (see [Setup](#setup)).
3. **Coding test thoroughness needs watching.** On weighted-ttl the mutation
   score of the agents' own tests was 93.1% before and 87.9% after (p = 0.057).
   Most of that gap comes from one run (75.6%); the other five after-runs
   averaged 90.4%. The earlier benchmark measured 88.3% for the coding profile
   with all four rules.

## Setup

| | Before (arm A) | After (arm B) |
|---|---|---|
| Build | `f710fc1` (prompt as in 0.5.0, plus the A/B harness) | `8dbbd96` (split prompt) |
| Coding prompt | Identity, R1 "requested change ... requested tests pass", no fuzz, run tests, concise | Identity, G1 "requested work is done", concise, C1 "done when the requested tests pass", no fuzz, run tests |
| General prompt | Identity line only | Identity line, G1, concise |

Common to both arms:

- Model `deepseek-flash`, reasoning effort `high`, DeepSeek Responses API,
  through the recording proxy. Runs on 2026-10-14, arms alternating
  (KA1, KB1, KA2, ...), one at a time, on one macOS machine.
- Runner: `scripts/bench/run_ab.sh` (public Session Endpoint client).
- research-brief runs used `--identity-profile general`. The coding corpora use
  the legacy genesis without an identity field, which reads as `coding`.
- Pricing: DeepSeek's off-peak rate card for `deepseek-flash` ($0.003 per
  million cached input, $0.15 uncached input, $0.60 output). Most runs fell in
  peak hours, so the real bill was higher for both arms; ratios do not change.

### Tasks

| Corpus | Turns | Work | Hidden acceptance |
|---|---|---|---|
| `research-brief` | 3 | Read three short documents, write a launch brief, add a risks section with counts, check every number and date | 20 checks: section headings, source facts, no wrong figures, at most 300 words |
| `batch-ledger-bugfix` | 3 | Fix a bug in an existing project, add regression tests, final review | 3 behavior checks |
| `weighted-ttl` | 4 | Write a weighted TTL + LRU cache and its tests, audit it, add a feature, final review | 11 behavior checks |

Prompts and seed files are in [fixtures/live/cache-corpus/](../../fixtures/live/cache-corpus/).

### Network reruns

Between about 07:00 and 09:05 UTC, runs failed with transport errors before
completing: the provider connection closed mid-stream, both through the local
network proxy and when connecting directly. Runs from 09:10 UTC on completed
normally. Every failed run ended with a `transport`
turn error, not an agent or acceptance failure, and both arms were affected alike.
Failed runs were discarded and rerun, not counted:

- batch-ledger-bugfix KB6 was rerun at 09:11 UTC (the other eleven runs were
  05:26–05:51 UTC);
- weighted-ttl was rerun entirely, 09:13–09:44 UTC;
- research-brief ran 05:15–05:26 UTC without failures.

## Results

### research-brief (general, 6 vs 6)

| Metric | Before | After | Ratio | p |
|---|---:|---:|---:|---:|
| Cost, all turns | $0.00377 | $0.00276 | 1.36 | 0.0065 |
| Cost, follow-up turns | $0.00212 | $0.00128 | 1.66 | 0.0032 |
| Output tokens | 4,691 | 3,148 | 1.49 | 0.0043 |
| Reasoning tokens | 2,046 | 1,033 | 1.98 | 0.011 |
| Cache hit rate | 89.9% | 88.9% | 1.01 | 0.13 |
| Tool calls | 11.0 | 9.8 | 1.12 | 0.12 |

All 12 briefs passed acceptance. Mean brief length was 207 words before and
220 words after, so the rules did not cut content.

### batch-ledger-bugfix (coding, 6 vs 6)

| Metric | Before | After | Ratio | p |
|---|---:|---:|---:|---:|
| Cost, all turns | $0.0112 | $0.0121 | 0.92 | 0.28 |
| Output tokens | 14,975 | 16,668 | 0.90 | 0.24 |
| Cache hit rate | 94.7% | 95.0% | 1.00 | 0.32 |
| Tool calls | 15.7 | 15.5 | 1.01 | 0.50 |

All 12 runs passed acceptance. Test quality was the same: every run added 3–5
regression tests, and its own suite failed on the seed ledger, which still has
the bug (4 failures in 10 runs, 5 in one before-run, 3 in one after-run). The "before" mean ($0.0112)
matches the earlier benchmark's coding profile ($0.0111).

### weighted-ttl (coding, 6 vs 6)

| Metric | Before | After | Ratio | p |
|---|---:|---:|---:|---:|
| Cost, all turns | $0.0218 | $0.0172 | 1.27 | 0.15 |
| Output tokens | 29,614 | 23,559 | 1.26 | 0.15 |
| Reasoning tokens | 14,809 | 9,908 | 1.49 | 0.11 |
| Model requests | 28.3 | 23.0 | 1.23 | 0.11 |
| Mutation score (mean) | 93.1% | 87.9% | | 0.057 |

All 12 runs passed acceptance. One "before" run cost $0.038; without it the
before-mean is $0.0185.

Per-run data: [research-brief](data/2026-10-general-rules/research-brief.analysis.json),
[batch-ledger-bugfix](data/2026-10-general-rules/batch-ledger-bugfix.analysis.json),
[weighted-ttl](data/2026-10-general-rules/weighted-ttl.analysis.json),
[mutation scores](data/2026-10-general-rules/weighted-ttl.mutation.json).

## Not tested

- Models other than DeepSeek Flash.
- Creative or domain workflows beyond the research brief (for example the drama
  skill workflow), and sessions with subagents.
- Whether the general rules change quality on open-ended writing, where "done"
  is less clear than in a fact-checked brief.

## Reproduce

```sh
git worktree add /tmp/kernel-base f710fc1
scripts/bench/run_ab.sh research-brief 6 /tmp/ab-brief /tmp/kernel-base . --identity-profile general
scripts/bench/run_ab.sh batch-ledger-bugfix 6 /tmp/ab-batch /tmp/kernel-base .
scripts/bench/run_ab.sh weighted-ttl 6 /tmp/ab-ttl /tmp/kernel-base .
python3 scripts/price-proxy-runs.py /tmp/ab-brief
python3 scripts/bench/stats.py /tmp/ab-brief KA KB
```

Check research-brief runs with `scripts/research_brief_acceptance.py`,
weighted-ttl runs with `scripts/weighted_ttl_acceptance.py` and
`scripts/bench/mutation.py`.
