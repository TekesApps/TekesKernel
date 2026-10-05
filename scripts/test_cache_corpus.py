"""Parity for the original quota-ledger corpus gate, without a model call."""
import unittest
from cache_corpus import prompts, require_successful_root_completion


class CacheCorpusTests(unittest.TestCase):
    def test_four_stable_quota_turns_and_fail_closed_selection(self):
        turns = prompts('quota-ledger')
        self.assertEqual(len(turns), 4)
        for index, marker in [(0, 'sliding_window_quota.py'),
                              (0, 'independent of mapping insertion order'),
                              (1, 'consume_many'), (2, '32-thread contention test'),
                              (2, 'final partial worker wave'), (2, '100 is not'),
                              (3, 'python3.14 -m unittest -v')]:
            self.assertIn(marker, turns[index])
        for turn in turns:
            self.assertIn('exactly once', turn)
            self.assertIn('do not', turn.lower())
            self.assertIn('Do not call task or subagent', turn)
        with self.assertRaises(ValueError):
            prompts('display-name-guessed')
        for outcome in ('error', None, 'interrupted', 'held'):
            with self.assertRaises(ValueError):
                require_successful_root_completion(outcome, 2)
        require_successful_root_completion('completed', 2)

    def test_all_original_scenarios_are_explicit(self):
        for scenario, count in [('hello', 1), ('deepseek-responses-smoke', 2),
                                ('weighted-ttl', 4), ('quota-ledger', 4)]:
            self.assertEqual(len(prompts(scenario)), count)


if __name__ == '__main__':
    unittest.main()
