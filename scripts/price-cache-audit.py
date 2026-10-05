#!/usr/bin/env python3
"""Price a cache audit from observed tokens using an explicitly supplied rate card."""

import argparse
from decimal import Decimal
import json
from pathlib import Path


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--audit', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--cached-per-million', type=Decimal, required=True)
parser.add_argument('--uncached-per-million', type=Decimal, required=True)
parser.add_argument('--output-per-million', type=Decimal, required=True)
parser.add_argument('--currency', required=True)
parser.add_argument('--rate-source', required=True)
parser.add_argument('--rate-label', required=True)
args = parser.parse_args()

audit = json.loads(args.audit.read_text())
if audit['status'] != 'passed':
    parser.error('audit must pass before pricing')
rates = {'cached_input': args.cached_per_million,
         'uncached_input': args.uncached_per_million,
         'output': args.output_per_million}


def priced(turns):
    input_tokens = sum(int(turn['input']) for turn in turns)
    cached = sum(int(turn['read']) for turn in turns)
    output = sum(int(turn['output']) for turn in turns)
    uncached = input_tokens - cached
    if uncached < 0:
        raise ValueError('cache read exceeds input tokens')
    amount = (Decimal(cached) * rates['cached_input']
              + Decimal(uncached) * rates['uncached_input']
              + Decimal(output) * rates['output']) / 1_000_000
    return {'input_tokens': input_tokens, 'cached_input_tokens': cached,
            'uncached_input_tokens': uncached, 'output_tokens': output,
            'hit_rate_percent': str((Decimal(cached) * 100 / input_tokens).quantize(Decimal('0.0001'))) if input_tokens else None,
            'estimated_amount': str(amount)}


report = {'scenario': audit['scenario'], 'prompt_sha256': audit['prompt_sha256'],
          'currency': args.currency, 'rate_label': args.rate_label,
          'rate_source': args.rate_source,
          'rate_per_million_tokens': {key: str(value) for key, value in rates.items()},
          'amount_kind': 'estimated_from_reported_tokens_and_published_rates',
          'systems': {}}
for name in ('kernel', 'dsh'):
    turns = {int(key): value for key, value in audit[name]['turns'].items()}
    report['systems'][name] = {
        'all_turns': priced([turns[key] for key in sorted(turns)]),
        'continuation': priced([turns[key] for key in sorted(turns) if key > 1]),
        'by_turn': {str(key): priced([turns[key]]) for key in sorted(turns)},
    }
args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({name: {scope: report['systems'][name][scope]
                         for scope in ('all_turns', 'continuation')}
                  for name in ('kernel', 'dsh')}))
