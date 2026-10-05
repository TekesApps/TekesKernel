#!/usr/bin/env python3
"""Verify real parallel tool completion, public correlation and durable fan-in."""
import json
import sys
from pathlib import Path

root=Path(sys.argv[1])
client=json.loads((root/'runtime/client-flow-receipt.json').read_text())
events=client['events']
paths=list((root/'runtime/threads').glob('*/main.jsonl'))
assert len(paths)==1
ledger=[json.loads(line) for line in paths[0].read_text().splitlines()]
calls=[e for e in ledger if e['kind']=='tool_call' and e['name'] in ('mcp__local__record_left','mcp__local__record_right')]
assert len(calls)==2 and len({c['attempt'] for c in calls})==1, 'two calls must belong to one response'
assert {(c['name'],c['args']['value']) for c in calls}=={('mcp__local__record_left','A'),('mcp__local__record_right','B')}
attempt=calls[0]['attempt']
ready=[(i,e['data']['chunk']) for i,e in enumerate(events) if e['type']=='assistant/chunk' and e['data']['chunk'].get('argumentsComplete') and e['data']['chunk'].get('assistantFrameId')==attempt]
assert len(ready)==2 and {c['id'] for _,c in ready}=={c['call'] for c in calls}, 'public sink must complete both calls in the same frame'
results=[]
for call in calls:
    projected=[(i,e) for i,e in enumerate(events) if e['type']=='tool/call' and e['data']['callId']==call['call']]
    assert len(projected)==1 and projected[0][1]['data']['arguments']==call['args']
    assert next(i for i,c in ready if c['id']==call['call'])<projected[0][0], 'completion must precede durable tool call'
    outcomes=[e for e in ledger if e['kind']=='tool_result' and e['call']==call['call']]
    assert len(outcomes)==1 and outcomes[0]['outcome']=='ok'
    results.extend(outcomes)
    public=[i for i,e in enumerate(events) if e['type']=='tool/result' and e['data']['message']['source']['callId']==call['call'] and e['data']['error'] is False]
    assert len(public)==1 and public[0]>projected[0][0]
next_attempt=next(e for e in ledger if e['kind']=='attempt' and e['seq']>max(c['seq'] for c in calls))
assert next_attempt['seq']>max(r['seq'] for r in results), 'next model request must wait for both tool results'
# Eager dispatch: every call and its single result are durable before the
# response's own terminal records (usage/output). Execution is pre-terminal,
# not merely presented early.
terminal=[e for e in ledger if e['kind'] in ('usage','output') and e.get('attempt')==attempt]
assert terminal and all(e['kind']!='error' for e in terminal), 'the eager attempt must reach a successful response terminal'
first_terminal_seq=min(e['seq'] for e in terminal)
assert max(r['seq'] for r in results)<first_terminal_seq, 'eager results must precede the response terminal records'
assert max(c['seq'] for c in calls)<first_terminal_seq, 'eager write-ahead calls must precede the response terminal records'
eager_records=sum(1 for e in ledger if e['kind']=='tool_call' and e.get('attempt')==attempt)
assert eager_records==2, 'the terminal must not write the eager calls again'
assert len([e for e in ledger if e['kind']=='settle'])==1
assert events[-1]['type']=='turn/end' and events[-1]['data']['reason']=='completed'
report=dict(passed=True,attempt=attempt,calls=[c['call'] for c in calls],ready_count=len(ready),next_attempt_seq=next_attempt['seq'],result_seqs=[r['seq'] for r in results],call_seqs=[c['seq'] for c in calls],first_terminal_seq=first_terminal_seq,eager_execution=True)
(root/'parallel-public-audit.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
