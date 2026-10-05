#!/usr/bin/env python3
"""Audit a completed public corpus run without equating settlement with tests passing."""
import argparse
import hashlib
import json
from pathlib import Path
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('output', type=Path)
args = parser.parse_args()
root = args.output
case = json.loads((root/'case.json').read_text())
runtime = root/'runtime'
endpoint = json.loads((runtime/'client-endpoint.json').read_text())
ledger = runtime/'threads'/endpoint['session']/'main.jsonl'
rows = [json.loads(line) for line in ledger.read_text().splitlines()]
issues = []
settles = [row for row in rows if row['kind']=='settle']
if len(settles) != len(case['prompts']) or any(row['outcome']!='completed' for row in settles):
    issues.append('not all corpus turns completed successfully')
turns = []
for index in range(1,len(case['prompts'])+1):
    calls = [row for row in rows if row['kind']=='tool_call' and row.get('turn')==index]
    results = {row['call']:row for row in rows if row['kind']=='tool_result' and row.get('turn')==index}
    if case['scenario']=='quota-ledger':
        if any(call['name'] in ('task','subagent') for call in calls):issues.append(f'turn {index}: forbidden child task')
    unit_calls = []
    for call in calls:
        if call['name']!='shell':continue
        command = call['args'].get('command') or ''
        if not re.match(r'^python3\.14\s+-m\s+unittest\s+-v\s*$',command.strip()):continue
        result = results.get(call['call'])
        parsed = None
        if result:
            content = ''.join(block.get('text','') for block in result['content'])
            try:parsed=json.loads(content)
            except json.JSONDecodeError:pass
        steps = parsed.get('steps',[]) if isinstance(parsed,dict) else []
        successful = bool(steps) and all(step.get('exit_code')==0 for step in steps)
        report = '\n'.join(step.get(stream,{}).get('data','') for step in steps for stream in ('stdout','stderr'))
        ran = re.search(r'Ran (\d+) tests?', report)
        suite_passed = successful and bool(ran) and int(ran.group(1))>0 and bool(re.search(r'^OK\s*$',report,re.M))
        unit_calls.append({'seq':call['seq'],'suite_passed':suite_passed,'tests':int(ran.group(1)) if ran else None})
    if case['scenario']=='quota-ledger' and (sum(call['suite_passed'] for call in unit_calls)!=1 or sum(call['tests'] is not None for call in unit_calls)!=1):
        issues.append(f'turn {index}: exactly one successful unittest execution is unproven')
    turns.append({'turn':index,'tool_calls':len(calls),'unittest_calls':unit_calls})
cache_file = runtime/'usage.cacheAttribution.json'
entries = []
if cache_file.exists():
    response = json.loads(cache_file.read_text())
    entries = response.get('result',{}).get('value',{}).get('entries',[])
if not entries:issues.append('no attributable cache rows')
report = {'status':'passed' if not issues else 'incomplete','scenario':case['scenario'],
          'issues':issues,'turns':turns,'cache_entries':entries,
          'ledger_sha256':hashlib.sha256(ledger.read_bytes()).hexdigest()}
(root/'corpus-audit.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'status':report['status'],'issues':issues}))
raise SystemExit(0 if not issues else 1)
