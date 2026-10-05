#!/usr/bin/env python3
"""Execute the legacy raw Chat Completions layout gates on their default routes."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--providers', default='deepseek,glm,kimi')
    parser.add_argument('--keys', type=Path, default=os.environ.get('TEKES_LIVE_KEYS_FILE'), required='TEKES_LIVE_KEYS_FILE' not in os.environ, help='Key file (default: $TEKES_LIVE_KEYS_FILE)')
    args = parser.parse_args()
    routes = {
        'deepseek': ('https://api.deepseek.com/v1','deepseek-chat','deepSeekKey'),
        'glm': ('https://api.z.ai/api/paas/v4','glm-4.6','glm'),
        'kimi': ('https://api.moonshot.cn/v1','moonshot-v1-8k','kimi'),
    }
    selected = args.providers.split(',')
    if not selected or any(p not in routes for p in selected):
        raise SystemExit('Select deepseek, glm, or kimi')
    args.output.mkdir(parents=True, exist_ok=False)
    secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"',args.keys.read_text(),re.M)) if args.keys.exists() else {}
    rows = []
    for provider in selected:
        endpoint,model,field = routes[provider]
        endpoint = os.environ.get('TEKES_LAYOUT_'+provider.upper()+'_ENDPOINT',endpoint)
        model = os.environ.get('TEKES_LAYOUT_'+provider.upper()+'_MODEL',model)
        key = os.environ.get(provider.upper()+'_API_KEY') or secrets.get(field)
        cell = args.output/provider
        cell.mkdir()
        row = {'provider':provider,'endpoint':endpoint,'model':model,'artifact':str(cell.resolve())}
        if not key:
            row['status'] = 'blocked_missing_credential'
        else:
            env = dict(os.environ,TEKES_LAYOUT_ENDPOINT=endpoint,TEKES_LAYOUT_MODEL=model,
                       TEKES_LAYOUT_KEY=key,TEKES_LAYOUT_OUTPUT=str(cell.resolve()))
            run = subprocess.run(['cargo','test','-p','provider','--test','live_chat_layout','--','--ignored','--nocapture'],
                cwd=Path(__file__).resolve().parents[1],env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
            log = run.stdout.decode(errors='replace')
            for secret in [key,*secrets.values()]:
                if len(secret)>8: log=log.replace(secret,'[REDACTED]')
            (cell/'test.log').write_text(log)
            row['status'] = 'passed' if run.returncode == 0 and (cell/'receipt.json').exists() else 'failed'
        rows.append(row)
        (args.output/'matrix.json').write_text(json.dumps(rows,indent=2)+'\n')
        print(provider+': '+row['status'],flush=True)
    return 0 if all(r['status']=='passed' for r in rows) else 1

if __name__ == '__main__':
    raise SystemExit(main())
