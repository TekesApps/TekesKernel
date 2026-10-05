"""A small sliding-window quota ledger with global idempotency keys.

The single-request path is established behavior. The batch path has a known
transaction defect, covered by the repository's failing tests.
"""

from collections import defaultdict, deque
from dataclasses import dataclass
from typing import Callable, Mapping


class IdempotencyConflictError(ValueError):
    """A key was reused for a different subject or resource allocation."""


@dataclass(frozen=True)
class Request:
    subject: str
    amounts: Mapping[str, int]
    idempotency_key: str


class SlidingWindowLedger:
    def __init__(self, limits: Mapping[str, int], window: float, clock: Callable[[], float]):
        if not limits or any(not isinstance(v, int) or isinstance(v, bool) or v <= 0
                             for v in limits.values()):
            raise ValueError("limits must be positive integers")
        if window <= 0:
            raise ValueError("window must be positive")
        self.limits = dict(limits)
        self.window = window
        self.clock = clock
        self._events = defaultdict(deque)
        self._decisions = {}

    def _signature(self, request: Request):
        if not request.subject or not request.idempotency_key:
            raise ValueError("subject and idempotency key are required")
        amounts = dict(request.amounts)
        if not amounts or any(resource not in self.limits or not isinstance(amount, int)
                              or isinstance(amount, bool) or amount <= 0
                              for resource, amount in amounts.items()):
            raise ValueError("amounts must be positive integers for known resources")
        return request.subject, tuple(sorted(amounts.items()))

    def _live_total(self, subject: str, resource: str, now: float) -> int:
        events = self._events[(subject, resource)]
        while events and events[0][0] <= now - self.window:
            events.popleft()
        return sum(amount for _, amount in events)

    def remaining(self, subject: str) -> dict[str, int]:
        now = self.clock()
        return {resource: limit - self._live_total(subject, resource, now)
                for resource, limit in self.limits.items()}

    def consume(self, request: Request) -> bool:
        signature = self._signature(request)
        key = request.idempotency_key
        if key in self._decisions:
            saved_signature, decision = self._decisions[key]
            if saved_signature != signature:
                raise IdempotencyConflictError(key)
            return decision
        now = self.clock()
        allowed = all(self._live_total(request.subject, resource, now) + amount
                      <= self.limits[resource] for resource, amount in signature[1])
        if allowed:
            for resource, amount in signature[1]:
                self._events[(request.subject, resource)].append((now, amount))
        self._decisions[key] = (signature, allowed)
        return allowed

    def consume_many(self, requests: list[Request]) -> list[bool]:
        # BUG: a later denial leaves earlier requests charged and recorded.
        return [self.consume(request) for request in requests]
