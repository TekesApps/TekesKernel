#!/usr/bin/env python3
"""Bind public task approval receipts to exactly one owning semantic ledger."""
import hashlib
import json
from pathlib import Path
import sys


def audit(directory):
    runtime = directory / 'runtime'
    client = json.loads((runtime / 'client-task-receipt.json').read_text())
    endpoint = json.loads((runtime / 'client-endpoint.json').read_text())
    session = endpoint['session']
    folder = runtime / 'threads' / session
    ledgers = {p.name: [json.loads(line) for line in p.read_text().splitlines()]
               for p in folder.glob('*.jsonl') if not p.name.startswith('endpoint')}
    # Requests are derived from approval_request holds (no request journal):
    # root ids hash session\0type\0seq; child ids hash the NUL-terminated
    # components child-request, session, line, type, seq.
    journal = []
    for name, events in ledgers.items():
        for hold in (e for e in events if e['kind'] == 'approval_request'):
            question = hold.get('scope') == 'answer' and 'question' in hold
            frame_type = 'question/requested' if question else 'approval/requested'
            if name == 'main.jsonl':
                preimage = f"{session}\0{frame_type}\0{hold['seq']}".encode()
            else:
                preimage = b''.join(part.encode() + b'\0' for part in ('child-request', session, name, frame_type, str(hold['seq'])))
            rpc_id = 'request-' + hashlib.sha256(preimage).hexdigest()
            journal.append({'kind': 'request', 'rpc_id': rpc_id, 'source_line': name, 'session_id': session, 'causal_kernel_seq': hold['seq']})
            answer = next((e for e in events if e['kind'] == 'approval_response' and e['call'] == hold['call'] and e['seq'] > hold['seq']), None)
            if answer is not None:
                outcome = 'answered' if question else ('allowed-once' if answer.get('grant') else 'rejected')
                journal.append({'kind': 'resolution', 'rpc_id': rpc_id, 'causal_kernel_seq': answer['seq'], 'outcome': outcome})
    assert set(client['requests']) == set(client['responses'])
    bindings = []
    for rpc_id, observed in client['requests'].items():
        requests = [r for r in journal if r['kind'] == 'request' and r['rpc_id'] == rpc_id]
        resolutions = [r for r in journal if r['kind'] == 'resolution' and r['rpc_id'] == rpc_id]
        assert len(requests) == len(resolutions) == 1
        request, resolution = requests[0], resolutions[0]
        line = request['source_line']
        item = observed['actionable']
        assert line == observed['source_line']
        assert request['session_id'] == item['sessionId'] == session
        assert request['causal_kernel_seq'] == item['revision'] == client['responses'][rpc_id]['revision']
        matches = [(name, e) for name, events in ledgers.items() for e in events
                   if e['kind'] == 'approval_response' and e.get('origin_key') == rpc_id + '/response']
        assert len(matches) == 1, (rpc_id, matches)
        name, response = matches[0]
        assert name == line, (line, name)
        assert response['seq'] == resolution['causal_kernel_seq'] > request['causal_kernel_seq']
        assert response['call'] == item['payload']['callId'] and response['grant'] is True
        assert resolution['outcome'] == 'allowed-once'
        target = session if line == 'main.jsonl' else session + ':' + line.removesuffix('.jsonl')
        assert response['origin_tuple']['target'] == target
        assert response['origin_tuple']['op'] == 'respond'
        bindings.append({'rpc_id': rpc_id, 'line': line, 'request_seq': request['causal_kernel_seq'], 'response_seq': response['seq']})
    assert any(b['line'] != 'main.jsonl' for b in bindings)
    final = client['final']
    settles = [e for e in ledgers['main.jsonl'] if e['kind'] == 'settle' and e['turn'] == final['data']['turn']]
    assert len(settles) == 1 and settles[0]['outcome'] == final['data']['reason'] == 'completed'
    assert settles[0]['validation']['outcome'] == final['data']['validationOutcome']
    result = {'status': 'passed', 'bindings': bindings, 'root_settle_seq': settles[0]['seq']}
    (directory / 'public-approval-audit.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


if __name__ == '__main__':
    result = audit(Path(sys.argv[1]))
    print(f"passed: {len(result['bindings'])} public approvals bound to their owning ledgers")
