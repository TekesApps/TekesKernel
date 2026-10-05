#!/usr/bin/env python3
"""Independently check every F1 quality artifact and both legacy score gates."""
import argparse
import json
from pathlib import Path


def audit(root):
    receipt = json.loads((root / 'receipt.json').read_text())
    assert receipt['status'] == 'passed', 'non-passing terminal receipt'
    rows = receipt['trials']
    expected = {(arm, trial, repeat) for arm in (False, True) for trial in range(4) for repeat in range(3)}
    assert len(rows) == 24
    assert {(r['relocated'], r['trial'], r['repeat']) for r in rows} == expected
    counts = [0, 0]
    pairs = {}
    for row in rows:
        arm, trial, repeat = row['relocated'], row['trial'], row['repeat']
        stem = f'quality-{int(arm)}-{trial}-{repeat}'
        request = json.loads((root / (stem + '.request.json')).read_text())
        response = json.loads((root / (stem + '.response.json')).read_text())
        assert row['completed'] is True and 200 <= row['http_status'] < 300
        assert response['status'] == 'completed', 'provider response did not complete'
        assert request['max_output_tokens'] == 80 and request['stream'] is False
        items = request['input']
        assert len(items) == (4 if arm else 3)
        assert items[0]['type'] == 'function_call' and items[0]['name'] == 'probe'
        assert items[1]['type'] == 'function_call_output'
        assert items[0]['call_id'] == items[1]['call_id']
        assert items[1]['output'] == 'an earlier tool observation, not relevant to the question'
        assert items[2]['role'] == 'user'
        if arm:
            assert items[3]['role'] == 'system'
        text = ''.join(part['text'] for item in response['output'] for part in item.get('content', []) if part.get('type') == 'output_text')
        assert text.strip(), 'empty response cannot satisfy lowercase or no-digit checks'
        obeyed = [lambda t: t.strip().endswith('ZZ9'), lambda t: '5' not in t,
                  lambda t: t == t.lower(), lambda t: t.strip().startswith('BANANA:')][trial](text)
        assert row['obeyed'] == obeyed
        counts[int(arm)] += int(obeyed)
        pairs[(arm, trial, repeat)] = request
    for trial in range(4):
        for repeat in range(3):
            baseline, relocated = (pairs[(arm, trial, repeat)] for arm in (False, True))
            assert baseline['model'] == relocated['model'] == receipt['model']
            assert baseline['input'] == relocated['input'][:3]
            guidance = relocated['input'][3]['content'][0]['text']
            assert baseline['instructions'] == relocated['instructions'] + '\n' + guidance
    assert counts == [receipt['baseline_passed'], receipt['relocated_passed']]
    assert counts[1] >= counts[0] - 1 and counts[0] * 2 > 12
    result = {'status': 'passed', 'trials': 24, 'baseline': counts[0], 'relocated': counts[1]}
    (root / 'independent-audit.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    print(json.dumps(audit(parser.parse_args().directory)))
