#!/usr/bin/env python3
"""Verify task final promotion and unchanged artifact bindings from a retained live run."""
import argparse
import hashlib
import json
from pathlib import Path


def audit(root):
    runtime = root / 'runtime'
    ledgers = list((runtime / 'threads').glob('*/main.jsonl'))
    assert len(ledgers) == 1, 'expected one root session'
    events = [json.loads(line) for line in ledgers[0].read_text().splitlines()]
    by_seq = {event['seq']: event for event in events}
    settles = [event for event in events if event['kind'] == 'settle']
    assert len(settles) == 1, 'expected exactly one root settlement'
    settle = settles[0]
    assert settle['outcome'] == 'completed'
    validation = settle['validation']
    assert validation['outcome'] == 'pass', 'validator did not pass'
    candidate = by_seq[validation['candidate_seq']]
    decision = by_seq[validation['decision_seq']]
    output = by_seq[validation['promoted_output_seq']]
    assert candidate['subkind'] == 'validation.candidate'
    assert decision['subkind'] == 'validation.decision'
    assert decision['payload']['candidate_seq'] == candidate['seq']
    assert decision['payload']['decision']['outcome'] == 'pass'
    assert output['kind'] == 'output' and output['final_answer'] is True
    binding = candidate['payload']['binding']
    assert binding['output_seq'] == output['seq']
    assert binding['thread'] == ledgers[0].parent.name
    assert binding['turn'] == output['turn'] == decision['turn'] == settle['turn']
    assert output['seq'] < candidate['seq'] < decision['seq'] < settle['seq']
    snapshot = binding['snapshot']
    assert snapshot, 'task has no artifact snapshot'
    workspace = (root / 'workspace').resolve()
    for name, digest in snapshot.items():
        path = Path(name).resolve()
        assert path.is_relative_to(workspace), 'artifact escaped disposable workspace'
        assert 'sha256-' + hashlib.sha256(path.read_bytes()).hexdigest() == digest
    receipt = {'status': 'passed', 'settle_seq': settle['seq'], 'candidate_seq': candidate['seq'],
               'decision_seq': decision['seq'], 'promoted_output_seq': output['seq'], 'snapshot': snapshot}
    (root / 'validation-audit.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return receipt


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('run', type=Path)
    args = parser.parse_args()
    audit(args.run.resolve())
    print('task validation bindings passed')
