#!/usr/bin/env python3
"""Run a TekesKernel cache corpus through pi 1.0 RPC mode (DeepSeek Responses API)."""

import argparse
import json
import os
from pathlib import Path
import queue
import re
import subprocess
import threading
import time

from cache_corpus import prompts, seed_files

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--pi-bin', type=Path, required=True)
parser.add_argument('--keys', type=Path, default=os.environ.get('TEKES_LIVE_KEYS_FILE'), help='JSON-like key file (default: $TEKES_LIVE_KEYS_FILE); TEKES_KERNEL_LIVE_KEY overrides it')
parser.add_argument('--base-url', default='https://api.deepseek.com',
                    help='provider base URL; point at scripts/record-provider-proxy.py to capture request bodies and usage')
parser.add_argument('--corpus', choices=['weighted-ttl', 'quota-ledger', 'batch-ledger-bugfix', 'hello'],
                    default='batch-ledger-bugfix')
parser.add_argument('--turn-timeout', type=int, default=2400)
parser.add_argument('--no-skills', action='store_true',
                    help='pass --no-skills so user-global skills (for example ~/.agents/skills) stay out of the system prompt')
args = parser.parse_args()

root = args.output.resolve()
root.mkdir(parents=True, exist_ok=False)
agent_dir = root/'agent'
workspace = root/'workspace'
sessions = root/'sessions'
agent_dir.mkdir(mode=0o700)
workspace.mkdir()
seed = seed_files(args.corpus)
for relative_path, content in seed.items():
    (workspace/relative_path).write_text(content)

provider, model, effort = 'deepseek-responses', 'deepseek-flash', 'high'
models = {'providers': {provider: {
    'baseUrl': args.base_url, 'api': 'openai-responses', 'apiKey': '$TEKES_PI_PROBE_API_KEY',
    'models': [{
        'id': model, 'name': 'DeepSeek Flash (Responses)', 'reasoning': True,
        'thinkingLevelMap': {'minimal': None, 'low': 'low', 'medium': None, 'high': 'high', 'xhigh': None, 'max': 'max'},
        'input': ['text'], 'contextWindow': 1000000, 'maxTokens': 384000,
        'cost': {'input': 0.15, 'output': 0.6, 'cacheRead': 0.003, 'cacheWrite': 0},
        'compat': {'supportsDeveloperRole': False, 'supportsStrictMode': False,
                   'supportsLongCacheRetention': False},
    }],
}}}
(agent_dir/'models.json').write_text(json.dumps(models, indent=2)+'\n')

secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"', args.keys.read_text(), re.M)) if args.keys and args.keys.exists() else {}
key = os.environ.get('TEKES_KERNEL_LIVE_KEY') or secrets.get('deepSeekKey')
if not key:
    raise SystemExit('DeepSeek credential unavailable')
env = dict(os.environ, PI_CODING_AGENT_DIR=str(agent_dir), PI_OFFLINE='1',
           PI_SKIP_VERSION_CHECK='1', PI_TELEMETRY='0', TEKES_PI_PROBE_API_KEY=key)
for name in ('OPENAI_API_KEY', 'ANTHROPIC_API_KEY', 'DEEPSEEK_API_KEY'):
    env.pop(name, None)

version = subprocess.run([str(args.pi_bin), '--version'], capture_output=True, text=True, env=env).stdout.strip()
manifest = {'system': 'pi', 'pi_version': version, 'provider': provider, 'api': 'openai-responses',
            'model': model, 'effort': effort, 'base_url': args.base_url,
            'prompts': prompts(args.corpus), 'scenario': args.corpus, 'seed_files': seed,
            'no_skills': args.no_skills}
(root/'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')

child = subprocess.Popen([str(args.pi_bin), '--mode', 'rpc', '--provider', provider, '--model', model,
                          '--thinking', effort, '--session-dir', str(sessions)]
                         + (['--no-skills'] if args.no_skills else []),
                         cwd=workspace, env=env, stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE, stderr=subprocess.PIPE)
frames = queue.Queue()


def read_stdout():
    with (root/'frames.jsonl').open('wb') as log:
        for line in iter(child.stdout.readline, b''):
            log.write(line)
            log.flush()
            try:
                frames.put(json.loads(line))
            except json.JSONDecodeError:
                frames.put({'non_json_stdout': line.decode(errors='replace')})
    frames.put({'stdout_eof': True})


def read_stderr():
    with (root/'stderr.log').open('w') as log:
        for line in iter(child.stderr.readline, b''):
            log.write(line.decode(errors='replace').replace(key, '[REDACTED]'))


threading.Thread(target=read_stdout, daemon=True).start()
threading.Thread(target=read_stderr, daemon=True).start()


def send(command):
    child.stdin.write((json.dumps(command)+'\n').encode())
    child.stdin.flush()


def wait_for(predicate, seconds, label):
    deadline = time.monotonic()+seconds
    seen = []
    while True:
        left = deadline-time.monotonic()
        if left <= 0:
            raise TimeoutError(f'{label} timed out')
        item = frames.get(timeout=left)
        if item.get('stdout_eof'):
            raise RuntimeError(f'pi exited during {label}')
        seen.append(item)
        if predicate(item):
            return item, seen


status = {'turns': [], 'status': 'running'}
try:
    for turn, prompt in enumerate(manifest['prompts'], 1):
        send({'id': f'prompt-{turn}', 'type': 'prompt', 'message': prompt})
        response, _ = wait_for(lambda r: r.get('id') == f'prompt-{turn}' and r.get('type') == 'response',
                               60, f'turn {turn} acceptance')
        if not response.get('success') or response.get('data', {}).get('disposition') != 'started':
            raise RuntimeError(f'turn {turn} prompt rejected: {response}')
        _, seen = wait_for(lambda r: r.get('type') == 'agent_settled', args.turn_timeout, f'turn {turn}')
        ends = [r for r in seen if r.get('type') == 'turn_end']
        last = ends[-1]['message'] if ends else {}
        status['turns'].append({'turn': turn, 'model_turns': len(ends),
                                'stop_reason': last.get('stopReason'),
                                'error': last.get('errorMessage')})
        (root/'status.json').write_text(json.dumps(status, indent=2)+'\n')
        print(f'pi turn {turn}: {len(ends)} model responses, stop={last.get("stopReason")}', flush=True)
        if last.get('stopReason') in ('error', 'aborted'):
            raise RuntimeError(f'turn {turn} ended with {last.get("stopReason")}: {last.get("errorMessage")}')
    child.stdin.close()
    child.wait(timeout=60)
    status['status'] = 'completed'
finally:
    if child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=10)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait()
    (root/'status.json').write_text(json.dumps(status, indent=2)+'\n')
print(f'Evidence: {root}')
