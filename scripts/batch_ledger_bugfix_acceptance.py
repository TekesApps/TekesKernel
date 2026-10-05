#!/usr/bin/env python3
"""Independent behavioral checks for the existing-repository bugfix corpus."""

import argparse
import importlib.util
from pathlib import Path
import sys


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('ledger', type=Path)
args = parser.parse_args()
spec = importlib.util.spec_from_file_location('candidate_ledger', args.ledger.resolve())
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)


class Clock:
    now = 0.0

    def __call__(self):
        return self.now


def request(key, api=0, jobs=0):
    amounts = {}
    if api:
        amounts['api'] = api
    if jobs:
        amounts['jobs'] = jobs
    return module.Request('alice', amounts, key)


def fresh():
    clock = Clock()
    return module.SlidingWindowLedger({'api': 5, 'jobs': 2}, 10, clock), clock


ledger, clock = fresh()
assert ledger.consume_many([]) == []
assert ledger.consume_many([request('a', api=2), request('a', api=2), request('b', api=3)]) == [True, True, True]
assert ledger.remaining('alice') == {'api': 0, 'jobs': 2}
assert ledger.consume_many([request('a', api=2), request('denied', api=1)]) == [True, False]
assert ledger.remaining('alice')['api'] == 0
clock.now = 10
assert ledger.remaining('alice')['api'] == 5
assert ledger.consume_many([request('denied', api=1)]) == [True]

ledger, _ = fresh()
assert ledger.consume_many([request('first', api=3), request('second', jobs=2),
                            request('third', api=3)]) == [False, False, False]
assert ledger.remaining('alice') == {'api': 5, 'jobs': 2}
assert ledger.consume(request('first', api=3)) is True

ledger, _ = fresh()
try:
    ledger.consume_many([request('x', api=1), request('x', jobs=1)])
except module.IdempotencyConflictError:
    pass
else:
    raise AssertionError('conflicting key did not raise')
assert ledger.remaining('alice') == {'api': 5, 'jobs': 2}
assert ledger.consume(request('x', jobs=1)) is True

print('batch-ledger acceptance passed')
