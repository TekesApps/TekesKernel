#!/usr/bin/env python3
"""Port of the legacy exact-route Anthropic canonical schema evidence probe."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--config', type=Path, default=os.environ.get('TEKES_PROVIDERS_CONFIG'), required='TEKES_PROVIDERS_CONFIG' not in os.environ, help='providers.json (default: $TEKES_PROVIDERS_CONFIG)')
    parser.add_argument('--keys', type=Path, default=os.environ.get('TEKES_LIVE_KEYS_FILE'), required='TEKES_LIVE_KEYS_FILE' not in os.environ, help='Key file (default: $TEKES_LIVE_KEYS_FILE)')
    parser.add_argument('--provider', required=True, help='Configured Anthropic Messages provider id')
    parser.add_argument('--model', default='claude-opus-5')
    args = parser.parse_args()
    providers = json.loads(args.config.read_text())['providers']
    provider = next(p for p in providers if p['id'] == args.provider)
    endpoint = provider['endpoint'].rstrip('/')
    if provider['dialect'] != 'anthropic_messages_v1' or not endpoint.startswith('https://gateway.ai.cloudflare.com/') or not endpoint.endswith('/anthropic'):
        raise SystemExit('Probe requires the configured native Cloudflare Anthropic gateway route')
    if not any(m['id'] == args.model and m.get('enabled') for m in provider['models']):
        raise SystemExit('Probe model must be enabled in the selected provider')
    args.output.mkdir(parents=True, exist_ok=False)
    secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"', args.keys.read_text(), re.M)) if args.keys.exists() else {}
    key = os.environ.get('TEKES_KERNEL_LIVE_KEY') or secrets.get('cloudflare')
    receipt = {'schema':'tekes.kernel.anthropic-schema-evidence.v1', 'route':endpoint, 'model':args.model,
               'legacy_test':'gateway_accepts_canonical_nonstrict_tool_schema',
               'scope':'Only this exact configured gateway route and model; no other route is inferred.', 'results':[]}
    receipt_path = args.output/'receipt.json'
    if not key:
        receipt['status'] = 'blocked_missing_credential'
        receipt_path.write_text(json.dumps(receipt, indent=2)+'\n')
        return 1
    env = dict(os.environ, TEKES_SCHEMA_ENDPOINT=endpoint, TEKES_SCHEMA_MODEL=args.model,
               TEKES_SCHEMA_KEY=key, TEKES_SCHEMA_OUTPUT=str(args.output.resolve()))
    result = subprocess.run(['cargo','test','-p','provider','--test','live_schema_probe','--','--ignored','--nocapture'],
        cwd=Path(__file__).resolve().parents[1], env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    log = result.stdout.decode(errors='replace')
    for secret in [key, *secrets.values()]:
        if len(secret) > 8:
            log = log.replace(secret, '[REDACTED]')
    (args.output/'test.log').write_text(log)
    print(log, end='')
    return result.returncode



if __name__ == '__main__':
    raise SystemExit(main())
