#!/usr/bin/env python3
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location('task_audit', Path(__file__).with_name('audit-live-task-validation.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

class FinalAnswerGateTests(unittest.TestCase):
    def fixture(self, root, final):
        session = '018f0000-0000-7000-8000-000000000128'
        ledger = root/'runtime/threads'/session/'main.jsonl'
        ledger.parent.mkdir(parents=True)
        artifact = root/'workspace/proof.md'
        artifact.parent.mkdir()
        artifact.write_text('proof')
        snapshot = {str(artifact.resolve()):'sha256-'+hashlib.sha256(artifact.read_bytes()).hexdigest()}
        records = [
            {'seq':1,'kind':'tool_result','name':'skill_explorer','outcome':'ok'},
            {'seq':2,'kind':'tool_result','name':'skill','outcome':'ok'},
            {'seq':3,'kind':'child_result','summary':'SKILL_POST_COMPACT_OK','outcome':'completed'},
            {'seq':4,'kind':'output','turn':2,'final_answer':final,'content':[{'type':'text','text':'SKILL_POST_COMPACT_OK'}]},
            {'seq':5,'kind':'state','subkind':'validation.candidate','payload':{'binding':{'thread':session,'turn':2,'output_seq':4,'snapshot':snapshot}}},
            {'seq':6,'kind':'state','turn':2,'subkind':'validation.decision','payload':{'candidate_seq':5,'decision':{'outcome':'pass'}}},
            {'seq':7,'kind':'settle','turn':2,'outcome':'completed','validation':{'outcome':'pass','candidate_seq':5,'decision_seq':6,'promoted_output_seq':4}},
        ]
        ledger.write_text(''.join(json.dumps(row)+'\n' for row in records))

    def test_explicit_root_final_with_matching_bindings_passes(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);self.fixture(root, True)
            self.assertEqual(module.audit(root)['status'],'passed')

    def test_skill_chain_child_marker_and_commentary_cannot_supply_root_final(self):
        with tempfile.TemporaryDirectory() as name:
            root=Path(name);self.fixture(root, False)
            with self.assertRaises(AssertionError):module.audit(root)
            self.assertFalse((root/'validation-audit.json').exists())

if __name__=='__main__':unittest.main()
