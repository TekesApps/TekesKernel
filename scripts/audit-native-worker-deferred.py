#!/usr/bin/env python3
"""Verify automatic native deferred-tool routing in a real worker run.

Reads the run-live-process.py tool-search output plus the secret-free wire
bodies the worker captured under TEKES_KERNEL_LIVE_ARTIFACT (`<output>/wire`).
"""
import json
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
mode = sys.argv[2]
assert mode in ('openai', 'anthropic')
import os
deferred_tool = 'mcp__local__get_shipping_eta' if os.environ.get('TEKES_TOOL_SEARCH_SCENARIO') == 'shipping' else 'mcp__local__uat_marker'
paths = list((root / 'runtime/threads').glob('*/main.jsonl'))
assert len(paths) == 1
ledger = [json.loads(line) for line in paths[0].read_text().splitlines()]
wire = Path(sys.argv[3]) if len(sys.argv) > 3 else root / 'wire'
requests = {p.name[:-len('.request.json')]: json.loads(p.read_text()) for p in wire.glob('*.request.json')}
assert requests, 'no captured request bodies'
attempts = [e for e in ledger if e['kind'] == 'attempt']
assert all(a['attempt'] in requests for a in attempts), 'every durable attempt has a captured request body'
report = {'mode': mode, 'turns': []}
for turn in sorted({e['turn'] for e in ledger if e['kind'] == 'tool_call' and e['name'] == 'tool_search'}):
    search = next(e for e in ledger if e['kind'] == 'tool_call' and e['name'] == 'tool_search' and e['turn'] == turn)
    result = next(e for e in ledger if e['kind'] == 'tool_result' and e['call'] == search['call'])
    assert result['outcome'] == 'ok'
    marker = next(e for e in ledger if e['kind'] == 'tool_call' and e['name'] == deferred_tool and e['turn'] == turn)
    assert result['seq'] < marker['seq']
    before = [a for a in attempts if a['turn'] == turn and a['seq'] < search['seq']]
    after = [a for a in attempts if a['turn'] == turn and a['seq'] > result['seq']]
    assert before and after, 'a search attempt and a post-search attempt'
    first, replay = requests[before[-1]['attempt']], requests[after[0]['attempt']]
    if mode == 'openai':
        for body in (first, replay):
            assert body.get('store') is False and 'previous_response_id' not in body, 'client-managed replay'
            search_tools = [t for t in body['tools'] if t.get('type') == 'tool_search']
            assert len(search_tools) == 1 and search_tools[0].get('execution') == 'client', 'native client tool_search declared once'
            assert all(t.get('name') != deferred_tool for t in body['tools']), 'deferred function stays out of the tool list'
        calls = [i for i in replay['input'] if i.get('type') == 'tool_search_call' and i.get('call_id') == search['call']]
        outputs = [i for i in replay['input'] if i.get('type') == 'tool_search_output' and i.get('call_id') == search['call']]
        assert len(calls) == 1 and len(outputs) == 1, 'sealed native search call replays with one bound output'
        assert replay['input'].index(calls[0]) < replay['input'].index(outputs[0])
        loaded = outputs[0]['tools']
        assert [t['name'] for t in loaded] == [deferred_tool] and loaded[0].get('defer_loading') is True
        assert not any(i.get('type') == 'function_call_output' and i.get('call_id') == search['call'] for i in replay['input'])
        assert any(i.get('type') == 'message' and i.get('role') == 'user' for i in replay['input']), 'original user input retained'
    else:
        for body in (first, replay):
            deferred = [t for t in body['tools'] if t.get('name') == deferred_tool]
            assert len(deferred) == 1 and deferred[0].get('defer_loading') is True, 'deferred schema declared with defer_loading'
            assert sum(1 for t in body['tools'] if t.get('name') == 'tool_search') == 1
        blocks = [b for m in replay['messages'] if m['role'] == 'user' for b in m['content'] if b.get('type') == 'tool_result' and b.get('tool_use_id') == search['call']]
        assert len(blocks) == 1 and blocks[0]['content'] == [{'type': 'tool_reference', 'tool_name': deferred_tool}], blocks
        uses = [b for m in replay['messages'] if m['role'] == 'assistant' for b in m['content'] if b.get('type') == 'tool_use' and b.get('id') == search['call']]
        assert len(uses) == 1 and uses[0]['name'] == 'tool_search'
    usage = [e for e in ledger if e['kind'] == 'usage' and e['attempt'] in (before[-1]['attempt'], after[0]['attempt'])]
    assert len(usage) == 2, 'usage present for the search and replay rounds'
    report['turns'].append({'turn': turn, 'search_call': search['call'], 'search_attempt': before[-1]['attempt'], 'replay_attempt': after[0]['attempt'], 'marker_args': marker['args']})
required_turns = int(os.environ.get('TEKES_NATIVE_AUDIT_TURNS', '2'))
assert len(report['turns']) >= required_turns, f'{required_turns} public turn(s) must search natively'
settles = [e for e in ledger if e['kind'] == 'settle']
completed = [s for s in settles if s['outcome'] == 'completed']
assert len(completed) >= required_turns, 'each audited turn completed'
report['completed_turns'] = len(completed)
report['refused_turns'] = [e['seq'] for e in ledger if e['kind'] == 'error' and e.get('classification') == 'provider_terminal' and e.get('detail') == 'content_filter']
report['passed'] = True
(root / 'native-worker-audit.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2))
