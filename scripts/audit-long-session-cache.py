#!/usr/bin/env python3
"""Compare one Kernel public-flow run with one DSH SDK run of identical prompts."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--kernel-output', type=Path, required=True)
parser.add_argument('--dsh-output', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()


def read_jsonl(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


kernel = args.kernel_output.resolve()
dsh = args.dsh_output.resolve()
kernel_case = json.loads((kernel/'case.json').read_text())
dsh_manifest = json.loads((dsh/'manifest.json').read_text())
same_prompts = kernel_case['prompts'] == dsh_manifest['prompts']
same_seed_files = kernel_case.get('seed_files', {}) == dsh_manifest.get('seed_files', {})
prompt_sha256 = hashlib.sha256(json.dumps(kernel_case['prompts'], ensure_ascii=False,
                                       separators=(',', ':')).encode()).hexdigest()
kernel_files = list((kernel/'runtime/threads').glob('*/main.jsonl'))
assert len(kernel_files) == 1, f'expected one Kernel root ledger, got {kernel_files}'
krows = read_jsonl(kernel_files[0])
dsh_files = list((dsh/'home/sessions').rglob('cache-probe/session.v3.jsonl.zstd'))
assert len(dsh_files) == 1, f'expected one DSH root log, got {dsh_files}'
dsh_plain = subprocess.run(['zstd', '-dc', str(dsh_files[0])], check=True,
                           capture_output=True).stdout.decode()
drows = [json.loads(line) for line in dsh_plain.splitlines() if line.strip()]


def summarize_kernel(rows):
    by_turn = {}
    answers = {}
    for row in rows:
        turn = row.get('turn')
        if not isinstance(turn, int) or turn < 1:
            continue
        entry = by_turn.setdefault(turn, {'requests': 0, 'reported': 0, 'unavailable': 0,
                                          'input': 0, 'read': 0, 'output': 0,
                                          'tool_calls': 0, 'settlement': None})
        kind = row.get('kind')
        if kind == 'attempt_dispatched':
            entry['requests'] += 1
        elif kind == 'tool_call':
            entry['tool_calls'] += 1
        elif kind == 'settle':
            entry['settlement'] = row.get('outcome')
        elif kind in ('output', 'error') and row.get('usage'):
            usage = row['usage']
            if usage.get('availability') == 'reported':
                entry['reported'] += 1
                entry['input'] += int(usage.get('input_tokens') or 0)
                entry['read'] += int(usage.get('cache_read') or 0)
                entry['output'] += int(usage.get('output_tokens') or 0)
            else:
                entry['unavailable'] += 1
            if kind == 'output':
                answers[turn] = ''.join(block.get('text', '') for block in row.get('content', [])
                                        if block.get('type') == 'text')
    return {'turns': by_turn,
            'answers': answers,
            'epochs': [{'reason': row.get('reason'), 'seq': row.get('seq')}
                       for row in rows if row.get('kind') == 'epoch'],
            'compactions': sum(row.get('kind') in ('compaction', 'trim') for row in rows)}


def summarize_dsh(rows):
    by_turn = {}
    answers = {}
    for row in rows:
        turn = row.get('data', {}).get('turn')
        if not isinstance(turn, int) or turn < 1:
            continue
        entry = by_turn.setdefault(turn, {'requests': 0, 'reported': 0, 'unavailable': 0,
                                          'input': 0, 'read': 0, 'output': 0,
                                          'tool_calls': 0, 'settlement': None})
        kind = row.get('type')
        if kind == 'step/start':
            entry['requests'] += 1
        elif kind == 'tool/call':
            entry['tool_calls'] += 1
        elif kind == 'turn/end':
            reason = row['data'].get('reason')
            entry['settlement'] = reason.get('kind') if isinstance(reason, dict) else reason
        elif kind == 'assistant/message':
            answers[turn] = ''.join(block.get('text', '')
                                    for block in row['data'].get('message', {}).get('content', [])
                                    if block.get('type') == 'text')
            usage = row['data'].get('usage')
            if usage:
                entry['reported'] += 1
                read = int(usage.get('cacheReadTokens') or 0)
                entry['input'] += int(usage.get('inputTokens') or 0) + read
                entry['read'] += read
                entry['output'] += int(usage.get('outputTokens') or 0)
            else:
                entry['unavailable'] += 1
    headers = [row['data']['header']['config'] for row in rows if row.get('type') == 'request/header']
    return {'turns': by_turn, 'answers': answers, 'headers': headers,
            'compactions': sum('compact' in row.get('type', '') for row in rows),
            'subagent_events': sum(row.get('type', '').startswith('subagent/') for row in rows)}


def metrics(summary, expected_turns):
    turns = {int(key): value for key, value in summary['turns'].items()}
    continuation = [value for turn, value in sorted(turns.items()) if turn > 1]
    read = sum(value['read'] for value in continuation)
    total = sum(value['input'] for value in continuation)
    reported = sum(value['reported'] for value in continuation)
    return {'completed_turns': sum(value['settlement'] == 'completed' for value in turns.values()),
            'expected_turns': expected_turns,
            'continuation_requests': sum(value['requests'] for value in continuation),
            'continuation_reported': reported,
            'continuation_unavailable': sum(value['unavailable'] for value in continuation),
            'continuation_cache_read': read,
            'continuation_input': total,
            'continuation_uncached': total-read,
            'continuation_hit_rate_percent': round(100*read/total, 4) if total else None,
            'uncached_per_reported_request': round((total-read)/reported, 2) if reported else None,
            'tool_calls': sum(value['tool_calls'] for value in turns.values())}


ks = summarize_kernel(krows)
ds = summarize_dsh(drows)
expected = len(kernel_case['prompts'])
km = metrics(ks, expected)
dm = metrics(ds, expected)
issues = []
if not same_prompts:
    issues.append('prompt lists differ')
if not same_seed_files:
    issues.append('seed files differ')
if km['completed_turns'] != expected:
    issues.append('Kernel did not complete every turn')
if dm['completed_turns'] != expected:
    issues.append('DSH did not complete every turn')
if not ds['headers'] or any(header.get('provider') != dsh_manifest['provider']
                            or header.get('model') != dsh_manifest['model']
                            or header.get('reasoningEffort') != dsh_manifest['effort']
                            for header in ds['headers']):
    issues.append('DSH request header does not match the declared route')
if kernel_case['scenario'] in ('long-session-cache', 'large-prefix-cache'):
    if km['tool_calls'] or dm['tool_calls']:
        issues.append('tool-free probe made a tool call')
    if km['continuation_requests'] != expected-1 or dm['continuation_requests'] != expected-1:
        issues.append('tool-free probe was not one request per turn')
    for turn in range(1, expected+1):
        answer = (f'LARGE_PREFIX_{turn}_OK' if kernel_case['scenario'] == 'large-prefix-cache'
                  else f'CACHE_PROBE_{turn}_OK')
        if ks['answers'].get(turn) != answer:
            issues.append(f'Kernel turn {turn} answer differs from its marker')
        if ds['answers'].get(turn) != answer:
            issues.append(f'DSH turn {turn} answer differs from its marker')
    assets = kernel_files[0].parent/'assets'
    attempts = [row for row in krows if row.get('kind') == 'attempt' and row.get('request')]
    bodies = [json.loads((assets/row['request']['asset']).read_text()) for row in attempts]
    if len(bodies) != expected:
        issues.append('Kernel request assets do not cover all turns')
    for index, body in enumerate(bodies):
        if body.get('model') != dsh_manifest['model'] or body.get('reasoning', {}).get('effort') != dsh_manifest['effort']:
            issues.append(f'Kernel request {index+1} has a different model or effort')
        if body.get('tools') not in ([], None):
            issues.append(f'Kernel request {index+1} advertised tools')
        if index:
            old = bodies[index-1]
            same_head = all(old.get(key) == body.get(key) for key in ('instructions', 'tools', 'model', 'reasoning'))
            old_input, new_input = old.get('input'), body.get('input')
            if not same_head or not isinstance(old_input, list) or not isinstance(new_input, list) or new_input[:len(old_input)] != old_input:
                issues.append(f'Kernel request {index+1} did not preserve the prior request prefix')
report = {'status': 'passed' if not issues else 'incomplete', 'scenario': kernel_case['scenario'],
          'issues': issues, 'prompt_sha256': prompt_sha256,
          'kernel': {'metrics': km, **ks}, 'dsh': {'metrics': dm, **ds}}
args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n')
print(json.dumps({'status': report['status'], 'issues': issues, 'kernel': km, 'dsh': dm}))
raise SystemExit(0 if not issues else 1)
