import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('scored', Path(__file__).with_name('run-agent-eval-scored.py'))
scored = importlib.util.module_from_spec(spec)
spec.loader.exec_module(scored)


class ScoredTests(unittest.TestCase):
    def test_task_regression_cannot_hide_in_overall_mean(self):
        measured = [{'task_id': 'a', 'pass_rate': 0}, {'task_id': 'b', 'pass_rate': 1}]
        baseline = [{'task_id': 'a', 'pass_rate': 1}, {'task_id': 'b', 'pass_rate': 0}]
        self.assertEqual(scored.compare(measured, baseline, .05)['regressions'], ['a'])

    def test_missing_baseline_task_fails_and_new_task_stays_ungated(self):
        gate = scored.compare([{'task_id': 'new', 'pass_rate': 1}], [{'task_id': 'old', 'pass_rate': 1}], .05)
        self.assertEqual(gate['status'], 'failed')
        self.assertEqual(gate['uncovered'], ['old'])
        self.assertEqual(gate['new_tasks'], ['new'])

    def test_harness_errors_remain_in_denominator_and_are_counted_separately(self):
        rows = [dict(task_id='a', dimension='memory', passed=True, harness_error=False, wall_s=1),
                dict(task_id='a', dimension='memory', passed=False, harness_error=True, wall_s=3)]
        report = scored.reduce_trials(rows)[0]
        self.assertEqual((report['pass_rate'], report['harness_errors'], report['avg_wall_s']), (.5, 1, 2))

    def test_empty_or_duplicate_baseline_fails_closed(self):
        for rows in ([], [{'task_id': 'a', 'pass_rate': 1}] * 2):
            with self.assertRaises(ValueError):
                scored.compare([], rows, .05)


if __name__ == '__main__':
    unittest.main()
