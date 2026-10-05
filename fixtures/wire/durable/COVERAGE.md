# worker-control durable-control oracle coverage

- Gate 65 consumes `queue-transaction-success`,
  `queue-transaction-management-startup`, and
  `queue-transaction-rejection-races` to lock the exact request/result bytes,
  management-only startup discriminator, and closed typed rejection union used
  by `session.updateQueue`.
- Gate 66 consumes `queue-transaction-crash-retry`: after a durable retraction,
  a reconcile-only run receives the byte-identical transaction, appends only
  its missing replacement, re-acks, then releases stop/input deliveries in
  arrival order. The race oracle also locks admission→management→line ordering,
  zero-event complete-error, and cancel's volatile stop gate.

The older negotiation/tool-control registry entries retain their existing
Slice-8 obligations. `scripts/check-worker-control-fixtures.py` checks artifact
existence in both directions, canonical JSON/JSONL, closed queue shapes, origin
child keys, exact retry equality, rejection semantics, and recovery-gate
records.
