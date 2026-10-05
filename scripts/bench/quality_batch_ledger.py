#!/usr/bin/env python3
"""Test quality on batch-ledger-bugfix: run each run's final test_ledger.py against its own ledger.py
and against the seed ledger.py, which still has the bug. Good regression tests fail on the seed.

usage: quality_batch_ledger.py <run-dir>...   (needs python3.14, as the corpus does)
"""
import re, shutil, subprocess, sys, tempfile
from pathlib import Path
SEED = Path(__file__).resolve().parents[2] / 'fixtures/live/cache-corpus/batch-ledger-bugfix'
def run(tests, ledger):
    with tempfile.TemporaryDirectory() as d:
        shutil.copy(tests, Path(d)/'test_ledger.py'); shutil.copy(ledger, Path(d)/'ledger.py')
        out = subprocess.run(['python3.14', '-m', 'unittest', '-q'], cwd=d, capture_output=True, text=True, timeout=120).stderr
    ran = int((re.search(r'Ran (\d+) test', out) or [0, 0])[1])
    m = re.search(r'FAILED \(([^)]*)\)', out); bad = sum(int(x) for x in re.findall(r'=(\d+)', m[1])) if m else 0
    return ran, bad
seed_ran, seed_bad = run(SEED/'test_ledger.py', SEED/'ledger.py')
print(f'seed suite on seed ledger: {seed_ran} tests, {seed_bad} fail')
for run_dir in sys.argv[1:]:
    w = Path(run_dir)/'workspace'
    own = run(w/'test_ledger.py', w/'ledger.py'); vs_seed = run(w/'test_ledger.py', SEED/'ledger.py')
    print(f'{Path(run_dir).name:6} tests {own[0]:3} (+{own[0]-seed_ran}) own-fail {own[1]} | on seed ledger fail {vs_seed[1]} (seed suite caught {seed_bad})')
