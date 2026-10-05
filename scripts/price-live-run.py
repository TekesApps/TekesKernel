#!/usr/bin/env python3
"""Price one Kernel live run from its reported usage, including child sessions."""

import argparse
from collections import defaultdict
from decimal import Decimal
import json
from pathlib import Path


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--kernel-output', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--cached-per-million', type=Decimal, required=True)
parser.add_argument('--uncached-per-million', type=Decimal, required=True)
parser.add_argument('--output-per-million', type=Decimal, required=True)
parser.add_argument('--currency', required=True)
parser.add_argument('--rate-source', required=True)
parser.add_argument('--rate-label', required=True)
args = parser.parse_args()


def price(entries):
    total = sum(row['input'] for row in entries)
    cached = sum(row['read'] for row in entries)
    output = sum(row['output'] for row in entries)
    if cached > total:
        raise ValueError('cache read exceeds input tokens')
    amount = (Decimal(cached) * args.cached_per_million
              + Decimal(total-cached) * args.uncached_per_million
              + Decimal(output) * args.output_per_million) / 1_000_000
    return {'responses': len(entries), 'input_tokens': total, 'cached_input_tokens': cached,
            'uncached_input_tokens': total-cached, 'output_tokens': output,
            'hit_rate_percent': str((Decimal(cached)*100/total).quantize(Decimal('0.0001'))) if total else None,
            'estimated_amount': str(amount)}


root = args.kernel_output.resolve()
files = sorted((root/'runtime/threads').glob('*/*.jsonl'))
if not files:
    parser.error('no Kernel session ledgers found')
all_entries = []
sessions = []
unavailable = 0
for path in files:
    entries = []
    by_turn = defaultdict(list)
    for line in path.read_text().splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        if row.get('kind') == 'compact' and row.get('summary_request'):
            usage = row['summary_request'].get('usage')
        elif row.get('kind') in ('output', 'error'):
            usage = row.get('usage')
        else:
            continue
        if not usage or usage.get('availability') not in (None, 'reported'):
            unavailable += 1
            continue
        entry = {'input': int(usage.get('input_tokens') or 0),
                 'read': int(usage.get('cache_read') or 0),
                 'output': int(usage.get('output_tokens') or 0)}
        entries.append(entry)
        by_turn[row.get('turn')].append(entry)
    if entries:
        sessions.append({'path': str(path.relative_to(root)), **price(entries),
                         'by_turn': {str(turn): price(items) for turn, items in by_turn.items()}})
        all_entries.extend(entries)
report = {'currency': args.currency, 'rate_source': args.rate_source,
          'rate_label': args.rate_label,
          'rates_per_million_tokens': {'cached_input': str(args.cached_per_million),
                                       'uncached_input': str(args.uncached_per_million),
                                       'output': str(args.output_per_million)},
          'amount_kind': 'estimated_from_reported_tokens_and_published_rates',
          'unavailable_usage_responses': unavailable,
          'all_sessions': price(all_entries), 'sessions': sessions}
args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n')
print(json.dumps(report['all_sessions']))
