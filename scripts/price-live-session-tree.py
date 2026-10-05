#!/usr/bin/env python3
"""Price every model response in Kernel and DSH session trees."""

import argparse
from decimal import Decimal
import json
from pathlib import Path
import subprocess


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--kernel-output', type=Path, required=True)
parser.add_argument('--dsh-output', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--cached-per-million', type=Decimal, required=True)
parser.add_argument('--uncached-per-million', type=Decimal, required=True)
parser.add_argument('--output-per-million', type=Decimal, required=True)
parser.add_argument('--currency', required=True)
parser.add_argument('--rate-label', required=True)
parser.add_argument('--rate-source', required=True)
args = parser.parse_args()


def ledger(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def dsh_ledger(path):
    data = subprocess.run(['zstd', '-dc', str(path)], check=True,
                          capture_output=True).stdout.decode()
    return [json.loads(line) for line in data.splitlines() if line.strip()]


def priced(entries):
    total = sum(item['input'] for item in entries)
    hit = sum(item['read'] for item in entries)
    output = sum(item['output'] for item in entries)
    miss = total-hit
    if miss < 0:
        raise ValueError('cache read exceeds input tokens')
    amount = (Decimal(hit)*args.cached_per_million
              + Decimal(miss)*args.uncached_per_million
              + Decimal(output)*args.output_per_million) / 1_000_000
    return {'reported_responses': len(entries), 'input_tokens': total,
            'cached_input_tokens': hit, 'uncached_input_tokens': miss,
            'output_tokens': output,
            'hit_rate_percent': str((Decimal(hit)*100/total).quantize(Decimal('0.0001'))) if total else None,
            'estimated_amount': str(amount)}


systems = {}
for system, root in [('kernel', args.kernel_output), ('dsh', args.dsh_output)]:
    files = (sorted((root/'runtime/threads').glob('*/*.jsonl')) if system == 'kernel'
             else sorted((root/'home/sessions').rglob('session.v3.jsonl.zstd')))
    sessions = []
    all_entries = []
    for path in files:
        rows = ledger(path) if system == 'kernel' else dsh_ledger(path)
        entries = []
        unavailable = 0
        summary_requests = 0
        if system == 'kernel':
            for row in rows:
                if row.get('kind') == 'compact' and row.get('summary_request'):
                    # The compactor calls the provider outside the ordinary
                    # output stream. Its usage is billable even when the
                    # proposed summary is rejected and the fallback is used.
                    summary_requests += 1
                    usage = row['summary_request'].get('usage')
                    if not usage:
                        unavailable += 1
                        continue
                elif row.get('kind') in ('output', 'error') and row.get('usage'):
                    usage = row['usage']
                else:
                    continue
                if usage.get('availability') not in (None, 'reported'):
                    unavailable += 1
                    continue
                entries.append({'input': int(usage.get('input_tokens') or 0),
                                'read': int(usage.get('cache_read') or 0),
                                'output': int(usage.get('output_tokens') or 0)})
        else:
            for row in rows:
                if row.get('type') != 'assistant/message':
                    continue
                usage = row.get('data', {}).get('usage')
                if not usage:
                    unavailable += 1
                    continue
                read = int(usage.get('cacheReadTokens') or 0)
                entries.append({'input': int(usage.get('inputTokens') or 0)+read,
                                'read': read, 'output': int(usage.get('outputTokens') or 0)})
        if entries or unavailable:
            sessions.append({'path': str(path.relative_to(root)),
                             'unavailable_usage_responses': unavailable,
                             'summary_requests': summary_requests, **priced(entries)})
            all_entries.extend(entries)
    systems[system] = {'all_sessions': priced(all_entries), 'sessions': sessions,
                       'summary_requests': sum(x['summary_requests'] for x in sessions),
                       'unavailable_usage_responses': sum(x['unavailable_usage_responses'] for x in sessions)}

report = {'currency': args.currency, 'rate_label': args.rate_label,
          'rate_source': args.rate_source,
          'rate_per_million_tokens': {'cached_input': str(args.cached_per_million),
                                      'uncached_input': str(args.uncached_per_million),
                                      'output': str(args.output_per_million)},
          'amount_kind': 'estimated_from_reported_tokens_and_published_rates',
          'systems': systems}
args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n')
print(json.dumps({name: systems[name]['all_sessions'] for name in ('kernel', 'dsh')}))
