#!/usr/bin/env python3
"""Independently audit six saved public-flow runs from received WebSocket frames."""
import argparse
import hashlib
import json
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--runs', nargs=6, type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
results = []
for root in args.runs:
    case = json.loads((root/'case.json').read_text())
    matrix = json.loads((root/'matrix.json').read_text())
    assert matrix['status'] == 'passed', (root, matrix)
    raw = (root/'runtime/client-frames.jsonl').read_bytes()
    frames = [json.loads(line) for line in raw.splitlines()]
    assert not any(f.get('type') in ('error', 'websocket-close') for f in frames)
    events = [f['frame']['event'] for f in frames if f.get('frame', {}).get('type') == 'journal-event']
    seqs = [e['seq'] for e in events]
    assert seqs == sorted(set(seqs))
    assert not any(e['type'] == 'response/error' for e in events)
    starts = [i for i, e in enumerate(events) if e['type'] == 'turn/start']
    ends = [i for i, e in enumerate(events) if e['type'] == 'turn/end']
    assert len(starts) == len(ends) == 1
    end = events[ends[0]]
    assert end['data']['reason'] == 'completed'
    user = next(i for i, e in enumerate(events) if e['type'] == 'user/message')
    assert case['prompt'] in json.dumps(events[user], ensure_ascii=False).replace('\\n', '\n')
    answer = next(i for i, e in enumerate(events) if e['type'] == 'assistant/message' and e['data']['message']['id'] == end['data']['promotedMessageID'])
    assert events[answer]['data']['message']['content'] == [{'type': 'text', 'text': case['token']}]
    assert starts[0] < user < answer < ends[0]
    if case['call']:
        name = case['call']['name']
        successes = []
        for i, call in enumerate(events):
            if call['type'] != 'tool/call' or call['data']['name'] != name:
                continue
            for j, result in enumerate(events):
                if result['type'] == 'tool/result' and result['data']['message']['source']['callId'] == call['data']['callId'] and not result['data']['error']:
                    assert user < i < j < answer
                    successes.append(result)
        assert successes, (root, name)
        if case['id'] == 'shell':
            payload = json.loads(successes[-1]['data']['message']['content'][0]['text'])
            assert payload['status'] == 'completed' and payload['is_error'] is False
            assert all(step['exit_code'] == 0 for step in payload['steps'])
            assert any(step['stdout']['encoding'] == 'utf8' and Path(step['stdout']['data'].strip()).resolve() == (root/'workspace').resolve() for step in payload['steps'])
    if case.get('expected_file'):
        expected = case['expected_file']
        assert (root/'workspace'/expected['path']).read_bytes() == expected['content'].encode()
    results.append({'case': case['id'], 'live': case.get('live', False), 'evidence': str(root.resolve()), 'frames_sha256': hashlib.sha256(raw).hexdigest(), 'indices': [starts[0], user, answer, ends[0]]})
assert sorted(r['case'] for r in results) == ['glob', 'grep', 'read', 'shell', 'text', 'write']
args.output.write_text(json.dumps({'status': 'passed', 'cases': results}, indent=2)+'\n')
print('six public flow cases passed')
