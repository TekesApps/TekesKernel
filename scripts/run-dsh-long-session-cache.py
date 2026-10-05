#!/usr/bin/env python3
"""Run the shared eight-turn cache probe through DSH's public SDK stdio profile."""

import argparse
from collections import deque
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
parser.add_argument('--node', type=Path, required=True)
parser.add_argument('--dsh-bin', type=Path, required=True)
parser.add_argument('--keys', type=Path, default=os.environ.get('TEKES_LIVE_KEYS_FILE'), help='JSON-like key file (default: $TEKES_LIVE_KEYS_FILE); TEKES_KERNEL_LIVE_KEY overrides it')
parser.add_argument('--turn-timeout', type=int, default=300)
parser.add_argument('--cordis-patch', type=Path,
                    help='copied to $DSH_HOME/cordis.patch.yml before launch (e.g. to disable the system-prompt or tool plugins for an A/B run)')
parser.add_argument('--cordis-patch-asset', type=Path, action='append', default=[],
                    help='file copied beside the patch in $DSH_HOME (a plugin the patch names by ./relative path)')
parser.add_argument('--base-url', default='https://api.deepseek.com',
                    help='provider base URL; point at scripts/record-provider-proxy.py to capture request bodies')
parser.add_argument('--corpus', choices=['long-session-cache', 'large-prefix-cache', 'weighted-ttl', 'deepseek-responses-smoke', 'batch-ledger-bugfix'],
                    default='long-session-cache')
args = parser.parse_args()
root = args.output.resolve()
root.mkdir(parents=True, exist_ok=False)
home = root/'home'
workspace = root/'workspace'
home.mkdir(mode=0o700)
workspace.mkdir()
seed = seed_files(args.corpus)
for relative_path, content in seed.items():
    destination = workspace/relative_path
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(content)
provider = 'deepseek-responses'
model = 'deepseek-v4-flash'
effort = 'high'
settings = {
    'llm-pi-ai': {'providers': {provider: {
        'displayName': 'DeepSeek', 'api': 'openai-responses',
        'apiKeyEnv': 'TEKES_DSH_CACHE_PROBE_API_KEY',
        'baseURL': args.base_url,
        'compat': {'supportsStrictMode': False, 'supportsDeveloperRole': False},
        'models': [{'id': model, 'contextWindow': 1000000,
                    'reasoningEfforts': {'off': 'none', 'low': 'low', 'high': 'high', 'max': 'max'},
                    'compat': {'supportsStrictMode': False, 'supportsDeveloperRole': False}}],
    }}},
    'agent-default-model': {'provider': provider, 'model': model, 'reasoningEffort': effort},
}
(home/'settings.yaml').write_text(json.dumps(settings, indent=2)+'\n')
os.chmod(home/'settings.yaml', 0o600)
secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"', args.keys.read_text(), re.M)) if args.keys and args.keys.exists() else {}
key = os.environ.get('TEKES_KERNEL_LIVE_KEY') or secrets.get('deepSeekKey')
if not key:
    raise SystemExit('DeepSeek credential unavailable')
env = dict(os.environ, DSH_HOME=str(home), DSH_TELEMETRY_DISABLED='1',
           TEKES_DSH_CACHE_PROBE_API_KEY=key)
if args.cordis_patch:
    (home/'cordis.patch.yml').write_text(args.cordis_patch.read_text())
    for asset in args.cordis_patch_asset:
        (home/asset.name).write_bytes(asset.read_bytes())
manifest = {'provider': provider, 'model': model, 'effort': effort, 'base_url': args.base_url,
            'cordis_patch': args.cordis_patch.read_text() if args.cordis_patch else None,
            'prompts': prompts(args.corpus), 'profile': 'sdk',
            'scenario': args.corpus, 'seed_files': seed,
            'tool_calls_allowed': args.corpus not in ('long-session-cache', 'large-prefix-cache')}
(root/'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
child = subprocess.Popen([str(args.node), str(args.dsh_bin), '--profile', 'sdk'],
                         cwd=workspace, env=env, stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1)
frames = queue.Queue()
pending = deque()

def read_stdout():
    with (root/'frames.jsonl').open('w') as log:
        for line in child.stdout:
            log.write(line)
            log.flush()
            try:
                frames.put(json.loads(line))
            except json.JSONDecodeError:
                frames.put({'non_json_stdout': line})
    frames.put({'stdout_eof': True})

def read_stderr():
    with (root/'stderr.log').open('w') as log:
        for line in child.stderr:
            log.write(line.replace(key, '[REDACTED]'))

threading.Thread(target=read_stdout, daemon=True).start()
threading.Thread(target=read_stderr, daemon=True).start()

def take(predicate, seconds):
    deadline = time.monotonic()+seconds
    while True:
        for item in list(pending):
            if predicate(item):
                pending.remove(item)
                return item
        left = deadline-time.monotonic()
        if left <= 0:
            raise TimeoutError('DSH SDK response or turn/end timed out')
        item = frames.get(timeout=left)
        if item.get('stdout_eof') or item.get('non_json_stdout'):
            raise RuntimeError(f'DSH SDK stream ended unexpectedly: {item}')
        if predicate(item):
            return item
        pending.append(item)

def rpc(number, method, params=None):
    assert child.stdin is not None
    request = {'jsonrpc': '2.0', 'id': number, 'method': method}
    if params is not None:
        request['params'] = params
    child.stdin.write(json.dumps(request)+'\n')
    child.stdin.flush()
    response = take(lambda item: item.get('id') == number, 60)
    if 'error' in response:
        raise RuntimeError(f'{method}: {response["error"]}')
    return response.get('result')

status = {'turns': [], 'status': 'running'}
try:
    initialized = rpc(1, 'initialize', {'cwd': str(workspace), 'provider': provider,
                                         'model': model, 'reasoningEffort': effort})
    assert initialized['serverInfo']['name'] == 'deepseek-harness-sdk-runtime'
    for turn, prompt in enumerate(manifest['prompts'], 1):
        receipt = rpc(turn+1, 'session/prompt', {'sessionId': 'cache-probe',
                                                'contentBlocks': [{'type': 'text', 'text': prompt}]})
        assert receipt.get('messageId')
        end = take(lambda item: item.get('method') == 'session.event'
                   and item.get('params', {}).get('sessionId') == 'cache-probe'
                   and item.get('params', {}).get('event', {}).get('type') == 'turn/end',
                   args.turn_timeout)
        reason = end['params']['event']['data']['reason']
        status['turns'].append({'turn': turn, 'message_id': receipt['messageId'], 'reason': reason})
        (root/'status.json').write_text(json.dumps(status, indent=2)+'\n')
        print(f'DSH turn {turn}: {reason}', flush=True)
        if reason.get('kind') in ('error', 'aborted', 'cancelled'):
            raise RuntimeError(f'turn {turn} ended with {reason}')
    rpc(20, 'shutdown')
    child.wait(timeout=30)
    if child.returncode != 0:
        raise RuntimeError(f'DSH exited {child.returncode}')
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
