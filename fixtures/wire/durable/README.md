# worker-control durable-control fixtures

These JSONL transcripts freeze the durable control messages (queue
transaction, tool control) of `spec/worker-control.md`; the base message
transcripts live one directory up in `fixtures/wire/`.
`cases.canonical.json` is the closed artifact inventory; entries marked
`implemented:false` must become present, canonical, and true before a
supervisor-control tool is advertised.

`queue-transaction.jsonl` freezes the Slice-9 request/receipt bytes.
`queue-transaction-retry.jsonl` and its recovery record freeze the
crash-after-retraction retry: later stop/input deliveries remain gated until a
reconcile-only run appends only the missing replacement and returns the
deduplicated committed result. `queue-transaction-startup.jsonl` freezes the
management-only negotiation discriminator and preloaded transaction;
`queue-transaction-rejections.jsonl` plus the race record freeze zero-event
under-lock rejection, gate release, the admission/management/line lock order,
and cancel priority. Run `scripts/check-worker-control-fixtures.py` to verify the
registry in both directions and all transaction transcripts.
