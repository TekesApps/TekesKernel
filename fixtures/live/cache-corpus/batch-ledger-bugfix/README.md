# Batch ledger bugfix fixture

This existing two-file Python project starts with two failing tests. The
`consume` API is correct for individual requests. `consume_many` incorrectly
applies each request as it goes, so a later denial or conflicting key leaves
earlier mutations in place.

Contract for a batch:

- An exact replay of a previously decided key returns its saved decision.
- Reusing a key for a different subject or allocation raises
  `IdempotencyConflictError`, before changing any state.
- Each new key in a successful batch is charged at most once; an exact repeat
  within the batch returns the same decision.
- If any new request cannot be allowed, every new request in that batch
  returns `False`, none is charged, and no new idempotency key is recorded.
- Empty batches return an empty list. Preserve the established `consume` and
  `remaining` behavior, including expiry exactly at the window boundary.
