#!/usr/bin/env python3
"""Price Kernel (K*) and pi (P*) runs from proxy-recorded DeepSeek usage, split by user turn.

Layout: <runs>/<name>/ is the run output and <runs>/<name>-proxy/ the record-provider-proxy.py
directory. Writes <runs>/analysis.json and prints per-run and per-system summaries.
"""

import json
from pathlib import Path
import subprocess
import sys

RUNS = Path(sys.argv[1])
RATES = {'cached': 0.003, 'uncached': 0.15, 'output': 0.60}  # USD / M, DeepSeek Flash off-peak
ACCEPT = Path(__file__).resolve().parent/'batch_ledger_bugfix_acceptance.py'


def jsonl(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def proxy_rows(directory):
    rows = []
    n = 1
    while (directory/f'request-{n}.json').exists():
        body = json.loads((directory/f'request-{n}.json').read_text())
        resp = json.loads((directory/f'response-{n}.json').read_text()) if (directory/f'response-{n}.json').exists() else {}
        usage = resp.get('usage') or {}
        title = 'conversation title' in str(body.get('instructions', ''))
        rows.append({'n': n, 'title': title, 'status': resp.get('status'), 'model': body.get('model'),
                     'effort': (body.get('reasoning') or {}).get('effort'),
                     'tools': len(body.get('tools') or []),
                     'has_usage': bool(usage),
                     'input': int(usage.get('input_tokens') or 0),
                     'read': int((usage.get('input_tokens_details') or {}).get('cached_tokens') or 0),
                     'output': int(usage.get('output_tokens') or 0),
                     'reasoning': int((usage.get('output_tokens_details') or {}).get('reasoning_tokens') or 0)})
        n += 1
    return rows


def kernel_turn_counts(run):
    files = list((run/'runtime/threads').glob('*/main.jsonl'))
    assert len(files) == 1, files
    counts, tools, own = {}, {}, {}
    for row in jsonl(files[0]):
        turn = row.get('turn')
        if not isinstance(turn, int) or turn < 1:
            continue
        if row.get('kind') in ('output', 'error') and row.get('usage'):
            counts[turn] = counts.get(turn, 0)+1
            u = row['usage']
            o = own.setdefault(turn, [0, 0, 0])
            o[0] += int(u.get('input_tokens') or 0); o[1] += int(u.get('cache_read') or 0); o[2] += int(u.get('output_tokens') or 0)
        if row.get('kind') == 'tool_call':
            tools[turn] = tools.get(turn, 0)+1
    return counts, tools, own


def pi_turn_counts(run):
    files = list((run/'sessions').rglob('*.jsonl'))
    assert len(files) == 1, files
    counts, tools, own = {}, {}, {}
    turn = 0
    for row in jsonl(files[0]):
        if row.get('type') != 'message':
            continue
        message = row['message']
        if message.get('role') == 'user':
            turn += 1
        elif message.get('role') == 'assistant' and message.get('usage'):
            counts[turn] = counts.get(turn, 0)+1
            tools[turn] = tools.get(turn, 0)+sum(b.get('type') == 'toolCall' for b in message.get('content', []))
            u = message['usage']
            o = own.setdefault(turn, [0, 0, 0])
            o[0] += int(u.get('input') or 0)+int(u.get('cacheRead') or 0); o[1] += int(u.get('cacheRead') or 0); o[2] += int(u.get('output') or 0)
    return counts, tools, own


def price(rows):
    inp = sum(r['input'] for r in rows); read = sum(r['read'] for r in rows); out = sum(r['output'] for r in rows)
    cost = (read*RATES['cached']+(inp-read)*RATES['uncached']+out*RATES['output'])/1e6
    return {'requests': len(rows), 'input': inp, 'cached': read, 'uncached': inp-read, 'output': out,
            'reasoning': sum(r['reasoning'] for r in rows),
            'hit_rate': round(100*read/inp, 2) if inp else None, 'cost': round(cost, 6)}


def analyze(name):
    run, proxy = RUNS/name, RUNS/f'{name}-proxy'
    rows = proxy_rows(proxy)
    agent = [r for r in rows if not r['title']]
    counts, tools, own = (kernel_turn_counts if name.startswith('K') else pi_turn_counts)(run)
    issues = []
    if any(r['model'] != 'deepseek-flash' for r in rows):
        issues.append('model not deepseek-flash')
    if any(r['effort'] != 'high' for r in agent):
        issues.append('agent request without effort high')
    bad = [r['n'] for r in rows if r['status'] != 200 or not r['has_usage']]
    if bad:
        issues.append(f'requests without 200/usage: {bad}')
    good_agent = [r for r in agent if r['status'] == 200 and r['has_usage']]
    by_turn, i = {}, 0
    for turn in sorted(counts):
        by_turn[turn] = good_agent[i:i+counts[turn]]
        i += counts[turn]
    if i != len(good_agent):
        issues.append(f'turn split mismatch: ledger {i} vs proxy {len(good_agent)}')
    for turn, chunk in by_turn.items():
        p = price(chunk)
        o = own.get(turn)
        if o and (o[0], o[1], o[2]) != (p['input'], p['cached'], p['output']):
            issues.append(f'turn {turn}: system usage {o} != proxy {(p["input"], p["cached"], p["output"])}')
    acc = subprocess.run([sys.executable, str(ACCEPT), str(run/'workspace/ledger.py')], capture_output=True, text=True)
    status_file = run/'status.json'
    return {'run': name, 'issues': issues,
            'acceptance': 'pass' if acc.returncode == 0 else 'FAIL: '+(acc.stderr.strip().splitlines() or ['?'])[-1],
            'responses_by_turn': [counts.get(t, 0) for t in sorted(counts)],
            'tool_calls': sum(tools.values()),
            'all': price(rows), 'agent': price(good_agent),
            'title': price([r for r in rows if r['title']]),
            'followup': price([r for t, c in by_turn.items() if t > 1 for r in c]),
            'by_turn': {t: price(c) for t, c in by_turn.items()},
            'status': json.loads(status_file.read_text()).get('status') if status_file.exists() else None}


names = sorted(p.name for p in RUNS.iterdir() if p.is_dir() and not p.name.endswith('-proxy'))
results = [analyze(n) for n in names]
(RUNS/'analysis.json').write_text(json.dumps(results, indent=2)+'\n')
print(f"{'run':4} {'resp/turn':10} {'tools':>5} {'hit%':>6} {'cost$':>9} {'fu hit%':>7} {'fu cost$':>9} {'out':>7} {'title$':>8} accept / issues")
for r in results:
    print(f"{r['run']:4} {'/'.join(map(str, r['responses_by_turn'])):10} {r['tool_calls']:>5} {r['all']['hit_rate']:>6} {r['all']['cost']:>9.6f} "
          f"{r['followup']['hit_rate'] or 0:>7} {r['followup']['cost']:>9.6f} {r['all']['output']:>7} {r['title']['cost']:>8.6f} {r['acceptance']} {r['issues']}")

for system in ('K', 'P'):
    rows = [r for r in results if r['run'].startswith(system)]
    if not rows:
        continue
    for scope in ('all', 'followup'):
        inp = sum(r[scope]['input'] for r in rows); read = sum(r[scope]['cached'] for r in rows)
        costs = [r[scope]['cost'] for r in rows]
        print(f"{system} {scope}: mean cost {sum(costs)/len(costs):.6f} (min {min(costs):.6f} max {max(costs):.6f}) "
              f"weighted hit {100*read/inp:.2f}% output {sum(r[scope]['output'] for r in rows)} "
              f"reasoning {sum(r[scope]['reasoning'] for r in rows)} requests {sum(r[scope]['requests'] for r in rows)}")
