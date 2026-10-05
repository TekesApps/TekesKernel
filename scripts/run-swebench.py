#!/usr/bin/env python3
"""SWE-bench-style agent eval on the Kernel (port of TekesAppServer Tests/SWEBench).

Stages, as in the legacy harness:
  1. provision — git clone the instance repo (bare-mirror cache) and check out base_commit
     into <output>/<instance>/trial-<k>/workspace, the tree the real agent edits;
  2. drive — one real turn through the spawned host and an external v3 client
     (scripts/run-public-flow.py --swebench-instance), legacy instruction verbatim;
  3. grade — `git add -A && git diff --cached HEAD` is the prediction; an empty patch is a
     skip; the official oracle (`python -m swebench.harness.run_evaluation`, Docker) grades
     when available, otherwise the mock oracle records resolved=false and the run is
     reported as ungraded;
  4. report — predictions.jsonl ({instance_id, model_name_or_path, model_patch}), the
     legacy report row shape (pass_at_1 / pass_at_k per instance) and per-trial detail.

Usage: run-swebench.py --output <new dir> [--instances id,id|N] [--trials K]
       [--fixture path] [--toolchain-root dir]... [--live-provider id --live-secret-name name]
"""
import argparse, json, os, shutil, subprocess, sys, time
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--fixture', type=Path, default=Path(__file__).resolve().parents[1] / 'fixtures/live/swebench/swe-bench-verified-smoke.jsonl')
parser.add_argument('--instances', default='1', help='count from the top of the fixture, or comma-separated instance ids, or all')
parser.add_argument('--trials', type=int, default=1)
parser.add_argument('--toolchain-root', type=Path, action='append', default=[])
parser.add_argument('--live-provider')
parser.add_argument('--live-secret-name')
parser.add_argument('--python', default=os.environ.get('TEKES_SWEBENCH_PYTHON', 'python3'))
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
args.output.mkdir(parents=True, exist_ok=False)
rows = [json.loads(line) for line in args.fixture.read_text().splitlines() if line.strip()]
if args.instances == 'all':
    selected = rows
elif args.instances.isdigit():
    selected = rows[:int(args.instances)]
else:
    wanted = set(args.instances.split(','))
    selected = [row for row in rows if row['instance_id'] in wanted]
assert selected, 'no instance selected'
docker = shutil.which('docker') is not None
swebench_module = subprocess.run([args.python, '-c', 'import swebench'], capture_output=True).returncode == 0
official_oracle = docker and swebench_module
mirrors = args.output / 'mirrors'

def provision(instance, workspace):
    mirror = mirrors / (instance['repo'].replace('/', '__') + '.git')
    if not mirror.exists():
        mirror.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(['git', 'clone', '--quiet', '--mirror', f"https://github.com/{instance['repo']}.git", str(mirror)], check=True, timeout=900)
    subprocess.run(['git', 'clone', '--quiet', str(mirror), str(workspace)], check=True, timeout=900)
    subprocess.run(['git', '-C', str(workspace), 'checkout', '--quiet', instance['base_commit']], check=True)
    subprocess.run(['git', '-C', str(workspace), 'config', 'user.email', 'swebench@tekes.local'], check=True)
    subprocess.run(['git', '-C', str(workspace), 'config', 'user.name', 'Tekes SWEBench'], check=True)

def prediction(workspace):
    subprocess.run(['git', '-C', str(workspace), 'add', '-A'], check=True)
    return subprocess.run(['git', '-C', str(workspace), 'diff', '--cached', 'HEAD'], check=True, capture_output=True, text=True).stdout

def grade_official(instance, patch, work):
    predictions = work / 'official-predictions.jsonl'
    predictions.write_text(json.dumps({'instance_id': instance['instance_id'], 'model_name_or_path': 'tekes-kernel', 'model_patch': patch}) + '\n')
    run_id = 'tekes-' + str(int(time.time()))
    result = subprocess.run([args.python, '-m', 'swebench.harness.run_evaluation', '--dataset_name', 'princeton-nlp/SWE-bench_Verified', '--predictions_path', str(predictions), '--instance_ids', instance['instance_id'], '--max_workers', '1', '--run_id', run_id], cwd=work, capture_output=True, text=True, timeout=3600)
    reports = list(work.glob(f'**/*{run_id}*.json'))
    resolved = None
    for report in reports:
        try:
            data = json.loads(report.read_text())
            if 'resolved_instances' in data: resolved = data['resolved_instances'] >= 1
            elif 'resolved' in data: resolved = bool(data['resolved'])
        except Exception: continue
    return {'oracle': 'official', 'resolved': resolved, 'exit': result.returncode, 'reports': [str(r) for r in reports], 'skipped': None if resolved is not None else 'harness produced no report json'}

outcomes = []
predictions = []
for instance in selected:
    for trial in range(1, args.trials + 1):
        cell = args.output / instance['instance_id'] / f'trial-{trial}'
        cell.mkdir(parents=True)
        workspace = cell / 'workspace'
        started = time.monotonic()
        provision(instance, workspace)
        (cell / 'instance.json').write_text(json.dumps(instance))
        command = [sys.executable, str(root / 'scripts/run-public-flow.py'), '--live', '--case', 'text', '--swebench-instance', str(cell / 'instance.json'), '--output', str(cell / 'flow'), '--no-route-probes']
        for toolchain in args.toolchain_root: command += ['--toolchain-root', str(toolchain)]
        if args.live_provider: command += ['--live-provider', args.live_provider]
        if args.live_secret_name: command += ['--live-secret-name', args.live_secret_name]
        # The flow runner's workspace is <flow output>/workspace: hand it the provisioned tree.
        (cell / 'flow').mkdir()
        shutil.move(str(workspace), str(cell / 'flow' / 'workspace'))
        workspace = cell / 'flow' / 'workspace'
        drive = subprocess.run(command, cwd=root, capture_output=True, text=True)
        (cell / 'drive.log').write_text(drive.stdout + drive.stderr)
        matrix_path = cell / 'flow' / 'matrix.json'
        matrix = json.loads(matrix_path.read_text()) if matrix_path.exists() else {'status': 'failed', 'note': 'no matrix'}
        patch = prediction(workspace) if workspace.exists() else ''
        (cell / 'prediction.diff').write_text(patch)
        turn_completed = matrix.get('status') == 'passed'
        if not patch.strip():
            grade = {'oracle': None, 'resolved': False, 'skipped': 'agent produced an empty patch'}
        elif official_oracle:
            grade = grade_official(instance, patch, cell)
        else:
            grade = {'oracle': 'mock', 'resolved': False, 'skipped': None, 'note': 'no Docker/swebench harness on this machine; mock oracle (FixedVerdictOracle false), patch exported for deferred grading'}
            predictions.append({'instance_id': instance['instance_id'], 'model_name_or_path': 'tekes-kernel', 'model_patch': patch})
        if patch.strip() and official_oracle:
            predictions.append({'instance_id': instance['instance_id'], 'model_name_or_path': 'tekes-kernel', 'model_patch': patch})
        outcome = {'instance_id': instance['instance_id'], 'trial': trial, 'turn_completed': turn_completed, 'drive_status': matrix.get('status'), 'patch_bytes': len(patch), 'files_changed': sorted({line.split(' b/')[-1] for line in patch.splitlines() if line.startswith('diff --git')}), 'resolved': bool(grade.get('resolved')), 'skipped': grade.get('skipped'), 'oracle': grade.get('oracle'), 'wall_seconds': round(time.monotonic() - started, 1)}
        outcomes.append(outcome)
        (args.output / 'trials.jsonl').write_text(''.join(json.dumps(o) + '\n' for o in outcomes))
        print(json.dumps(outcome), flush=True)
        # Keep the artifacts small: the provisioned checkout is reproducible from the mirror.
        shutil.rmtree(workspace / '.git', ignore_errors=True)

(args.output / 'predictions.jsonl').write_text(''.join(json.dumps(p) + '\n' for p in predictions))
reports = []
for instance in selected:
    rows_i = [o for o in outcomes if o['instance_id'] == instance['instance_id']]
    graded = [o for o in rows_i if not o['skipped']]
    resolved_count = sum(1 for o in graded if o['resolved'])
    reports.append({'schema_version': 1, 'instance_id': instance['instance_id'], 'model': 'tekes-kernel', 'trials': len(graded), 'resolved_count': resolved_count,
                    'pass_at_1': (resolved_count / len(graded)) if graded else 0.0, 'pass_at_k': 1.0 if resolved_count > 0 else 0.0, 'skipped_trials': len(rows_i) - len(graded)})
(args.output / 'report.jsonl').write_text(''.join(json.dumps(r) + '\n' for r in reports))
summary = {'status': 'passed' if outcomes and all(o['turn_completed'] for o in outcomes) and not all(o['skipped'] for o in outcomes) else 'failed',
           'instances': len(selected), 'trials': args.trials, 'grading': 'official-docker' if official_oracle else 'blocked_no_docker (mock oracle; predictions exported)',
           'docker': docker, 'swebench_module': swebench_module, 'predictions': len(predictions), 'turns_completed': sum(1 for o in outcomes if o['turn_completed']),
           'non_empty_patches': sum(1 for o in outcomes if o['patch_bytes'] > 0), 'pass_at_1': (sum(r['pass_at_1'] for r in reports) / len(reports)) if reports else 0.0}
(args.output / 'matrix.json').write_text(json.dumps(summary, indent=2))
print(json.dumps(summary, indent=2))
sys.exit(0 if summary['status'] == 'passed' else 1)
