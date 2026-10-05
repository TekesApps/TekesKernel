#!/usr/bin/env python3
"""Run the original salted cache measurement through Kernel provider normalization."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--scenario', choices=['fanout','warm'], default='fanout')
parser.add_argument('--config', type=Path, default=os.environ.get('TEKES_PROVIDERS_CONFIG'), required='TEKES_PROVIDERS_CONFIG' not in os.environ, help='providers.json (default: $TEKES_PROVIDERS_CONFIG)')
parser.add_argument('--keys', type=Path, default=os.environ.get('TEKES_LIVE_KEYS_FILE'), required='TEKES_LIVE_KEYS_FILE' not in os.environ, help='Key file (default: $TEKES_LIVE_KEYS_FILE)')
args = parser.parse_args()
providers = [p for p in json.loads(args.config.read_text())['providers'] if p['dialect']=='openai_responses_v1' and p['endpoint_owner']=='cloudflare']
if len(providers) != 1:
    raise SystemExit('Expected one configured CF OpenAI provider')
args.output.mkdir(parents=True, exist_ok=False)
(args.output/'provider.json').write_text(json.dumps(providers[0], indent=2))
secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"', args.keys.read_text(), re.M)) if args.keys.exists() else {}
key = os.environ.get('TEKES_KERNEL_LIVE_KEY') or secrets.get('cloudflare')
if not key:
    (args.output/'receipt.json').write_text(json.dumps({'status':'blocked_missing_credential'}))
    raise SystemExit(1)
env = dict(os.environ, TEKES_SCHEMA_OUTPUT=str(args.output.resolve()), TEKES_SCHEMA_KEY=key)
test_name = {'fanout':'fan_out_warmup_payoff','warm':'per_model_warm_cache_rate'}[args.scenario]
run = subprocess.run(['cargo','test','-p','provider','--locked','--test','live_cache','--','--ignored','--exact',test_name,'--nocapture'],
    cwd=Path(__file__).resolve().parents[1], env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
log = run.stdout.decode(errors='replace')
for secret in [key, *secrets.values()]:
    if len(secret)>8:
        log = log.replace(secret, '[REDACTED]')
(args.output/'test.log').write_text(log)
receipt_path = args.output/'receipt.json'
receipt = json.loads(receipt_path.read_text()) if receipt_path.exists() else {}
passed = run.returncode == 0 and '1 passed' in log and receipt.get('status') == 'passed'
print('passed' if passed else 'failed: inspect retained test.log')
raise SystemExit(0 if passed else 1)
