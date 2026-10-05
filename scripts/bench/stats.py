#!/usr/bin/env python3
"""Compare two arms of a priced benchmark: means, ranges, Welch t and an exact permutation test.

usage: stats.py <runs-dir> <prefix-A> <prefix-B>

Reads <runs-dir>/analysis.json written by scripts/price-proxy-runs.py. Runs whose
name starts with prefix A form arm A, prefix B arm B (for example K and P). The
permutation p-value is one-sided in the direction of the observed difference and
enumerates every split when that is feasible (at most 200,000 splits), otherwise
it samples 200,000 random splits with a fixed seed.
"""
import itertools
import json
import math
import random
import statistics
import sys
from pathlib import Path

runs_dir, prefix_a, prefix_b = Path(sys.argv[1]), sys.argv[2], sys.argv[3]
rows = json.loads((runs_dir / 'analysis.json').read_text())
arm_a = [row for row in rows if row['run'].startswith(prefix_a)]
arm_b = [row for row in rows if row['run'].startswith(prefix_b)]
if len(arm_a) < 2 or len(arm_b) < 2:
    raise SystemExit(f'need at least two runs per arm, got {len(arm_a)} and {len(arm_b)}')


def welch(x, y):
    vx, vy = statistics.variance(x), statistics.variance(y)
    se = math.sqrt(vx / len(x) + vy / len(y))
    if se == 0:
        return float('nan'), float('nan')
    t = (statistics.mean(x) - statistics.mean(y)) / se
    df = (vx / len(x) + vy / len(y)) ** 2 / ((vx / len(x)) ** 2 / (len(x) - 1) + (vy / len(y)) ** 2 / (len(y) - 1))
    return t, df


def permutation_p(x, y):
    observed = statistics.mean(x) - statistics.mean(y)
    pooled = x + y
    n = len(x)
    sign = 1 if observed >= 0 else -1
    total = math.comb(len(pooled), n)
    if total <= 200_000:
        splits = (list(c) for c in itertools.combinations(range(len(pooled)), n))
    else:
        rng = random.Random(0)
        splits = (rng.sample(range(len(pooled)), n) for _ in range(200_000))
        total = 200_000
    extreme = 0
    for chosen in splits:
        chosen_set = set(chosen)
        a = [pooled[i] for i in chosen]
        b = [pooled[i] for i in range(len(pooled)) if i not in chosen_set]
        if sign * (statistics.mean(a) - statistics.mean(b)) >= sign * observed - 1e-15:
            extreme += 1
    return extreme / total


METRICS = [
    ('cost, all turns ($)', lambda r: r['all']['cost']),
    ('cost, follow-up turns ($)', lambda r: r['followup']['cost']),
    ('output tokens', lambda r: r['all']['output']),
    ('reasoning tokens', lambda r: r['all']['reasoning']),
    ('uncached input tokens', lambda r: r['all']['uncached']),
    ('cache hit rate (%)', lambda r: r['all']['hit_rate']),
    ('model requests', lambda r: r['agent']['requests']),
    ('tool calls', lambda r: r['tool_calls']),
]
print(f'{prefix_a}: {len(arm_a)} runs   {prefix_b}: {len(arm_b)} runs')
print(f"{'metric':28} {prefix_a + ' mean':>12} {prefix_b + ' mean':>12} {'ratio':>7} {'Welch t':>8} {'perm p':>8}  ranges")
for label, value in METRICS:
    x = [value(row) for row in arm_a]
    y = [value(row) for row in arm_b]
    t, _ = welch(x, y)
    ratio = statistics.mean(x) / statistics.mean(y) if statistics.mean(y) else float('nan')
    print(f'{label:28} {statistics.mean(x):12.6g} {statistics.mean(y):12.6g} {ratio:7.3f} {t:8.2f} '
          f'{permutation_p(x, y):8.4f}  [{min(x):.6g}, {max(x):.6g}] vs [{min(y):.6g}, {max(y):.6g}]')
