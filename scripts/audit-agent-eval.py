#!/usr/bin/env python3
"""Audit exact pinned AgentEval tasks using public final answers and bound child reports."""
import argparse, hashlib, json
from pathlib import Path
from agent_eval import TASKS


def audit(directory):
    case = json.loads((directory / 'case.json').read_text())
    task_id = case['scenario']
    task = TASKS[task_id]
    assert case['prompts'] == task['prompts'] and case['seed_files'] == task['seed_files'], 'task inputs drifted'
    matrix = json.loads((directory / 'matrix.json').read_text())
    report = {'task_id': task_id, 'dimension': task['dimension'], 'passed': False,
              'harness_error': matrix['status'] != 'passed', 'matched': [], 'missing': [], 'evidence': {}}
    if task_id == 'subagent-summarize-notes' and ('subagent' not in case.get('allowed_tools', []) or 'report' in case.get('allowed_tools', [])):
        report['harness_error'] = True
        report['note'] = 'invalid trial setup: root must expose subagent without selecting child-only report role'
        return report
    if report['harness_error']:
        report['note'] = 'public process/client harness failed'
        return report
    runtime = directory / 'runtime'
    client = json.loads((runtime / 'client-flow-receipt.json').read_text())
    events = client['events']
    ends = [e for e in events if e['type'] == 'turn/end']
    assert len(ends) == len(task['prompts']) and all(e['data']['reason'] == 'completed' for e in ends)
    assert not any(e['type'] == 'response/error' for e in events)
    final = next(e for e in events if e['type'] == 'assistant/message'
                 and e['data']['message']['id'] == ends[-1]['data']['promotedMessageID'])
    assert final['data']['sessionFinal'] is True
    roots = list((runtime / 'threads').glob('*/main.jsonl'))
    assert len(roots) == 1
    path = roots[0]
    records = [json.loads(line) for line in path.read_text().splitlines()]
    report['evidence']['main_sha256'] = hashlib.sha256(path.read_bytes()).hexdigest()
    report['evidence']['final_message_id'] = final['data']['message']['id']
    checks = {}
    if task_id == 'memory-recall-codename':
        checks['turn_2_final_contains_BLUEHERON'] = final['data']['turn'] == 2 and any(
            'blueheron' in b.get('text', '').lower() for b in final['data']['message']['content'])
    else:
        calls = [e for e in records if e['kind'] == 'tool_call' and e['name'] == 'subagent']
        checks['successful_subagent_tool_result'] = any(
            result['type'] == 'tool/result' and result['data']['error'] is False
            and result['data']['message']['source']['callId'] == call['call']
            for result in events for call in calls)
        delivered = False
        for call in calls:
            for spawn in [e for e in records if e['kind'] == 'spawn' and e['call'] == call['call']]:
                child_path = path.parent / spawn['child']
                assert child_path.parent.resolve() == path.parent.resolve()
                child = [json.loads(line) for line in child_path.read_text().splitlines()]
                parent = child[0]['parent']
                assert parent['file'] == 'main.jsonl' and parent['seq'] == spawn['seq'] and parent['spawn_id'] == spawn['spawn_id']
                reports = [e for e in child if e['kind'] == 'tool_call' and e['name'] == 'report']
                for child_report in reports:
                    accepted = any(e['kind'] == 'tool_result' and e['call'] == child_report['call'] and e['outcome'] == 'ok' for e in child)
                    text = child_report['args'].get('result')
                    joined = any(e['kind'] == 'child_result' and e['call'] == call['call'] and e['spawn_id'] == spawn['spawn_id']
                                 and e['child'] == spawn['child'] and e['outcome'] == 'completed' and e.get('summary') == text for e in records)
                    if accepted and joined and isinstance(text, str) and text:
                        delivered = True
                        report['evidence']['child_sha256'] = hashlib.sha256(child_path.read_bytes()).hexdigest()
        checks['child_report_accepted_and_joined'] = delivered
    report['matched'] = [name for name, matched in checks.items() if matched]
    report['missing'] = [name for name, matched in checks.items() if not matched]
    report['passed'] = not report['missing']
    report['note'] = 'Kernel semantic/public-interface port; external legacy RPC record names are not reused'
    return report

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    args = parser.parse_args()
    report = audit(args.directory)
    (args.directory / 'agent-eval-verdict.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))
    # Original capability smoke gates harness health, not model capability.
    raise SystemExit(1 if report['harness_error'] else 0)
