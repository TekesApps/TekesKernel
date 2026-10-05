#!/usr/bin/env python3
"""Run the pinned AgentEval catalog N times and apply its per-task baseline gate."""
import argparse
import json
import re
import subprocess
import sys
import time
from pathlib import Path
from agent_eval import TASKS


def reduce_trials(outcomes):
    reports = []
    for task_id in sorted({o['task_id'] for o in outcomes}):
        rows = [o for o in outcomes if o['task_id'] == task_id]
        passed = sum(o['passed'] for o in rows)
        reports.append(dict(task_id=task_id, dimension=rows[0]['dimension'], trials=len(rows),
                            passed=passed, harness_errors=sum(o['harness_error'] for o in rows),
                            pass_rate=passed / len(rows), avg_wall_s=sum(o['wall_s'] for o in rows) / len(rows),
                            schema_version=1))
    return reports


def compare(measured, baseline, margin):
    current = {r['task_id']: r['pass_rate'] for r in measured}
    rates = {r['task_id']: r['pass_rate'] for r in baseline}
    if len(rates) != len(baseline) or not rates or any(not 0 <= r <= 1 for r in rates.values()):
        raise ValueError('invalid or duplicate baseline rates')
    uncovered = sorted(rates.keys() - current.keys())
    regressed = sorted(k for k in rates.keys() & current.keys() if current[k] < rates[k] - margin)
    return dict(status='failed' if uncovered or regressed else 'passed', uncovered=uncovered,
                regressions=regressed, new_tasks=sorted(current.keys() - rates.keys()), noise_margin=margin)


def write_jsonl(path, rows):
    path.write_text(''.join(json.dumps(row, sort_keys=True) + '\n' for row in rows))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--trials', type=int, default=3)
    parser.add_argument('--noise-margin', type=float, default=.05)
    parser.add_argument('--baseline-directory', type=Path, required=True)
    args = parser.parse_args()
    if args.trials < 1 or not 0 <= args.noise_margin <= 1:
        parser.error('trials must be positive and margin must be between 0 and 1')
    args.output.mkdir(parents=True, exist_ok=False)
    runner = Path(__file__).with_name('run-public-flow.py')
    outcomes = []
    model = None
    for task_id, task in TASKS.items():
        for trial in range(1, args.trials + 1):
            directory = args.output / f'{task_id}-{trial}'
            print(f'{task_id}: trial {trial}/{args.trials}', flush=True)
            start = time.monotonic()
            with (args.output / f'{task_id}-{trial}.log').open('w') as log:
                result = subprocess.run([sys.executable, str(runner), '--live', '--agent-eval', task_id,
                                         '--output', str(directory)], stdout=log, stderr=subprocess.STDOUT)
            verdict_path = directory / 'agent-eval-verdict.json'
            verdict = json.loads(verdict_path.read_text()) if verdict_path.exists() else {}
            error = result.returncode != 0 or verdict.get('harness_error', True)
            outcome = dict(task_id=task_id, dimension=task['dimension'], trial=trial,
                           passed=not error and verdict.get('passed', False), harness_error=error,
                           wall_s=time.monotonic() - start, note=verdict.get('note', 'runner failed'),
                           evidence_directory=str(directory.resolve()))
            provider_path = directory / 'provider.json'
            if provider_path.exists():
                provider = json.loads(provider_path.read_text())
                # Match the public process fixture workspace model exactly.
                selected = provider['models'][0]['id']
                if model is not None and model != selected:
                    raise RuntimeError('model changed between trials')
                model = selected
            outcomes.append(outcome)
            write_jsonl(args.output / 'trials.jsonl', outcomes)
            print(json.dumps(outcome), flush=True)
    reports = reduce_trials(outcomes)
    slug = re.sub(r'[^\w.\-]', '_', model or 'unknown')
    write_jsonl(args.output / f'{slug}.jsonl', reports)
    write_jsonl(args.output / f'{slug}-trials.jsonl', outcomes)
    baseline = args.baseline_directory / f'{slug}.jsonl'
    if all(o['harness_error'] for o in outcomes):
        gate = dict(status='failed', reason='all trials were harness errors')
    elif baseline.exists():
        gate = compare(reports, [json.loads(line) for line in baseline.read_text().splitlines() if line.strip()], args.noise_margin)
    else:
        gate = dict(status='bootstrap', reason='no blessed baseline for this model; not a regression pass')
    gate.update(model=model, baseline=str(baseline), overall_pass_rate=sum(r['pass_rate'] for r in reports)/len(reports),
                dimension_pass_rates={d: sum(r['pass_rate'] for r in reports if r['dimension']==d)/sum(r['dimension']==d for r in reports)
                                      for d in sorted({r['dimension'] for r in reports})},
                timing_scope='end-to-end harness including binary build/freeze; not legacy driver latency')
    (args.output / 'gate.json').write_text(json.dumps(gate, indent=2) + '\n')
    print(json.dumps(gate, indent=2), flush=True)
    return 1 if gate['status'] == 'failed' else 0


if __name__ == '__main__':
    raise SystemExit(main())
