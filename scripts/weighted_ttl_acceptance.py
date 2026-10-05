#!/usr/bin/env python3
"""Run one independent acceptance suite against a weighted TTL cache module."""

import argparse
import importlib.util
from pathlib import Path
import sys
import unittest


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('module', type=Path)
args = parser.parse_args()
spec = importlib.util.spec_from_file_location('weighted_ttl_cache', args.module.resolve())
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)
WeightedTTLCache = module.WeightedTTLCache


def entry_key(entry):
    return entry.key if hasattr(entry, 'key') else entry[0]


def entry_value(entry):
    return entry.value if hasattr(entry, 'value') else entry[1]


class Clock:
    def __init__(self):
        self.now = 0.0

    def __call__(self):
        return self.now


class Acceptance(unittest.TestCase):
    def setUp(self):
        self.clock = Clock()
        self.cache = WeightedTTLCache(5, clock=self.clock)

    def test_basic_get_default_and_delete(self):
        self.assertEqual(self.cache.get('missing', 'default'), 'default')
        self.cache.put('a', 1, weight=2, ttl=10)
        self.assertEqual(self.cache.get('a'), 1)
        self.assertTrue(self.cache.delete('a'))
        self.assertEqual(self.cache.get('a'), None)

    def test_get_promotes_lru(self):
        self.cache.put('a', 1, weight=2, ttl=10)
        self.cache.put('b', 2, weight=2, ttl=10)
        self.cache.get('a')
        self.cache.put('c', 3, weight=2, ttl=10)
        self.assertEqual(self.cache.get('b'), None)
        self.assertEqual(self.cache.get('a'), 1)

    def test_weighted_eviction(self):
        self.cache.put('a', 1, weight=2, ttl=10)
        self.cache.put('b', 2, weight=2, ttl=10)
        self.cache.put('c', 3, weight=4, ttl=10)
        self.assertEqual([entry_key(entry) for entry in self.cache.snapshot()], ['c'])
        self.assertEqual(self.cache.total_weight, 4)

    def test_expiry_at_exact_deadline(self):
        self.cache.put('a', 1, weight=2, ttl=10)
        self.clock.now = 10
        self.assertIsNone(self.cache.get('a'))
        self.assertEqual(len(self.cache), 0)
        self.assertEqual(self.cache.total_weight, 0)

    def test_expired_entries_removed_before_eviction(self):
        self.cache.put('a', 1, weight=3, ttl=1)
        self.cache.put('b', 2, weight=2, ttl=10)
        self.clock.now = 1
        self.cache.put('c', 3, weight=3, ttl=10)
        self.assertEqual(self.cache.get('b'), 2)
        self.assertEqual(self.cache.get('c'), 3)

    def test_replacement_changes_weight_expiry_and_recency(self):
        self.cache.put('a', 1, weight=2, ttl=2)
        self.cache.put('b', 2, weight=2, ttl=10)
        self.clock.now = 1
        self.cache.put('a', 10, weight=3, ttl=10)
        self.assertEqual(self.cache.total_weight, 5)
        self.assertEqual([entry_key(entry) for entry in self.cache.snapshot()], ['a', 'b'])
        self.clock.now = 2
        self.assertEqual(self.cache.get('a'), 10)

    def test_invalid_put_does_not_change_existing_entry(self):
        self.cache.put('a', 1, weight=2, ttl=10)
        for weight, ttl in [(0, 10), (2, 0), (6, 10)]:
            with self.subTest(weight=weight, ttl=ttl):
                with self.assertRaises((TypeError, ValueError)):
                    self.cache.put('a', 9, weight=weight, ttl=ttl)
                self.assertEqual(self.cache.get('a'), 1)
                self.assertEqual(self.cache.total_weight, 2)

    def test_snapshot_is_mru_to_lru_and_detached(self):
        self.cache.put('a', 1, weight=1, ttl=10)
        self.cache.put('b', 2, weight=1, ttl=10)
        old = self.cache.snapshot()
        self.assertEqual([entry_key(entry) for entry in old], ['b', 'a'])
        self.cache.put('b', 3, weight=1, ttl=10)
        self.assertEqual(entry_value(old[0]), 2)

    def test_compute_hit_skips_factory(self):
        self.cache.put('a', 1, weight=1, ttl=10)
        def fail():
            raise AssertionError('factory called on hit')
        self.assertEqual(self.cache.get_or_compute('a', fail, ttl=10, weight=1), 1)

    def test_compute_miss_calls_once_and_stores(self):
        calls = []
        def factory():
            calls.append(1)
            return 7
        self.assertEqual(self.cache.get_or_compute('a', factory, ttl=10, weight=2), 7)
        self.assertEqual(len(calls), 1)
        self.assertEqual(self.cache.get('a'), 7)

    def test_compute_exception_does_not_change_other_entries(self):
        self.cache.put('a', 1, weight=2, ttl=10)
        before = self.cache.snapshot()
        def fail():
            raise RuntimeError('factory failure')
        with self.assertRaisesRegex(RuntimeError, 'factory failure'):
            self.cache.get_or_compute('b', fail, ttl=10, weight=2)
        self.assertEqual(self.cache.snapshot(), before)
        self.assertIsNone(self.cache.get('b'))


if __name__ == '__main__':
    unittest.main(argv=[sys.argv[0]], verbosity=2)
