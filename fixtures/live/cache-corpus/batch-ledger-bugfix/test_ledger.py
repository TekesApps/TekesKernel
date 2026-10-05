import unittest

from ledger import IdempotencyConflictError, Request, SlidingWindowLedger


class Clock:
    def __init__(self):
        self.now = 0.0

    def __call__(self):
        return self.now


class LedgerTests(unittest.TestCase):
    def setUp(self):
        self.clock = Clock()
        self.ledger = SlidingWindowLedger({"api": 5, "jobs": 2}, 10, self.clock)

    def request(self, key, amount, subject="alice"):
        return Request(subject, {"api": amount}, key)

    def test_single_consume_and_exact_expiry(self):
        self.assertTrue(self.ledger.consume(self.request("one", 4)))
        self.assertEqual(self.ledger.remaining("alice")["api"], 1)
        self.clock.now = 10
        self.assertEqual(self.ledger.remaining("alice")["api"], 5)
        self.assertTrue(self.ledger.consume(self.request("two", 5)))

    def test_single_replay_is_global_and_sticky(self):
        self.assertFalse(self.ledger.consume(self.request("denied", 6)))
        self.clock.now = 10
        self.assertFalse(self.ledger.consume(self.request("denied", 6)))
        with self.assertRaises(IdempotencyConflictError):
            self.ledger.consume(self.request("denied", 6, "bob"))

    def test_successful_batch_charges_every_new_key(self):
        self.assertEqual(self.ledger.consume_many([
            self.request("one", 2), self.request("two", 3)]), [True, True])
        self.assertEqual(self.ledger.remaining("alice")["api"], 0)

    def test_later_denial_rolls_back_all_new_requests(self):
        self.assertEqual(self.ledger.consume_many([
            self.request("one", 3), self.request("two", 3)]), [False, False])
        self.assertEqual(self.ledger.remaining("alice")["api"], 5)
        self.assertTrue(self.ledger.consume(self.request("one", 3)))

    def test_conflicting_key_in_batch_leaves_no_charge(self):
        with self.assertRaises(IdempotencyConflictError):
            self.ledger.consume_many([self.request("same", 1), self.request("same", 2)])
        self.assertEqual(self.ledger.remaining("alice")["api"], 5)
        self.assertTrue(self.ledger.consume(self.request("same", 2)))


if __name__ == "__main__":
    unittest.main()
