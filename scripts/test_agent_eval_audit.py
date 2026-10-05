import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from agent_eval import TASKS

spec = importlib.util.spec_from_file_location('audit_eval', Path(__file__).with_name('audit-agent-eval.py'))
audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(audit)


class AuditTests(unittest.TestCase):
    def fixture(self, folder, accepted=True, joined=True):
        task = 'subagent-summarize-notes'
        root = Path(folder)
        (root/'case.json').write_text(json.dumps(dict(TASKS[task], scenario=task, allowed_tools=['subagent'])))
        (root/'matrix.json').write_text(json.dumps({'status': 'passed'}))
        threads = root/'runtime/threads/session'
        threads.mkdir(parents=True)
        events = [dict(type='turn/end', data={'reason': 'completed', 'promotedMessageID': 'final'}),
                  dict(type='assistant/message', data={'sessionFinal': True, 'message': {'id': 'final'}}),
                  dict(type='tool/result', data={'error': False, 'message': {'source': {'callId': 'call'}}})]
        (root/'runtime/client-flow-receipt.json').write_text(json.dumps({'events': events}))
        main = [dict(kind='tool_call', name='subagent', call='call'),
                dict(kind='spawn', call='call', child='child.jsonl', seq=3, spawn_id='spawn')]
        if joined:
            main.append(dict(kind='child_result', call='call', child='child.jsonl', spawn_id='spawn', outcome='completed', summary='summary'))
        child = [dict(parent={'file': 'main.jsonl', 'seq': 3, 'spawn_id': 'spawn'}),
                 dict(kind='tool_call', name='report', call='report', args={'result': 'summary'})]
        child[0]['kind'] = 'genesis'
        if accepted:
            child.append(dict(kind='tool_result', call='report', outcome='ok'))
        for name, rows in [('main', main), ('child', child)]:
            (threads/f'{name}.jsonl').write_text(''.join(json.dumps(r)+'\n' for r in rows))
        return root

    def test_success_requires_both_accepted_report_and_parent_join(self):
        for accepted, joined in [(True, True), (False, True), (True, False)]:
            with self.subTest(accepted=accepted, joined=joined), tempfile.TemporaryDirectory() as folder:
                root = self.fixture(folder, accepted, joined)
                self.assertEqual(audit.audit(root)['passed'], accepted and joined)

    def test_failed_public_harness_is_not_capability_measurement(self):
        with tempfile.TemporaryDirectory() as folder:
            root = self.fixture(folder)
            (root/'matrix.json').write_text(json.dumps({'status': 'failed'}))
            self.assertTrue(audit.audit(root)['harness_error'])


if __name__ == '__main__':
    unittest.main()
