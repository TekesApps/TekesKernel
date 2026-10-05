#!/usr/bin/env python3
"""Run the legacy budgeted Anthropic thinking smoke through configured Cloudflare."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--model')
parser.add_argument('--binding', action='store_true', help='Verify stable and edited prefixes with preserved thinking')
parser.add_argument('--config', type=Path, default=os.environ.get('TEKES_PROVIDERS_CONFIG'), required='TEKES_PROVIDERS_CONFIG' not in os.environ, help='providers.json (default: $TEKES_PROVIDERS_CONFIG)')
parser.add_argument('--keys', type=Path, default=os.environ.get('TEKES_LIVE_KEYS_FILE'), required='TEKES_LIVE_KEYS_FILE' not in os.environ, help='Key file (default: $TEKES_LIVE_KEYS_FILE)')
args = parser.parse_args()
providers = [p for p in json.loads(args.config.read_text())['providers'] if p['dialect']=='anthropic_messages_v1' and p['endpoint_owner']=='cloudflare']
if len(providers) != 1:
    raise SystemExit('Expected one configured CF Anthropic provider')
if args.model:
    selected = [m for m in providers[0]['models'] if m['id']==args.model and m.get('enabled')]
    if len(selected)!=1:
        raise SystemExit('Requested model must be enabled in the selected CF provider')
    providers[0]['models'] = selected
args.output.mkdir(parents=True, exist_ok=False)
(args.output/'provider.json').write_text(json.dumps(providers[0], indent=2))
secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"', args.keys.read_text(), re.M)) if args.keys.exists() else {}
key = os.environ.get('TEKES_KERNEL_LIVE_KEY') or secrets.get('cloudflare')
if not key:
    (args.output/'receipt.json').write_text(json.dumps({'status':'blocked_missing_credential'}))
    raise SystemExit(1)
env = dict(os.environ, TEKES_SCHEMA_OUTPUT=str(args.output.resolve()), TEKES_SCHEMA_KEY=key)
test_name = 'cloudflare_generation_thinking_and_optional_tool_replay'
if args.binding:
    test_name = 'cloudflare_preserved_thinking_prefix_binding'
run = subprocess.run(['cargo','test','-p','provider','--locked','--test','live_anthropic_thinking','--','--ignored','--exact',test_name,'--nocapture'],
    cwd=Path(__file__).resolve().parents[1], env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
log = run.stdout.decode(errors='replace')
for secret in [key, *secrets.values()]:
    if len(secret)>8:
        log = log.replace(secret, '[REDACTED]')
(args.output/'test.log').write_text(log)
passed = run.returncode == 0 and '1 passed' in log and (args.output/'receipt.json').exists()
if not passed and (args.output/'receipt.json').exists():
    receipt = json.loads((args.output/'receipt.json').read_text())
    receipt['status'] = 'failed'
    (args.output/'receipt.json').write_text(json.dumps(receipt, indent=2))
print('passed' if passed else 'failed: inspect retained test.log')
raise SystemExit(0 if passed else 1)
