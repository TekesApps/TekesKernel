# Anthropic preserved thinking follow-up — 2026-09-09

## Result and boundary

The remaining request binding policy and transformation diagnostics are now
implemented. Native block/signature preservation already existed; it was not
reimplemented or moved into a new dialect. Fable 5.1 live binding acceptance is
still blocked: the configured Cloudflare route returns `not_found_error` with
`model: claude-fable-5-1` before the test reaches prefix validation.

Reference: [Anthropic preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking).

## Historical cache constraints retained

Reviewed the cache-breakpoint and cache-hit-rate handoffs dated 2026-09-06,
the route-sweep handoff dated 2026-09-07, and associated Git history:

- `a2048f2` already retained native blocks and signatures; `ddb6896` added
  adaptive thinking and exact-model controls.
- `2b91b03` placed cache control on the last tool and the latest message's last
  content block, instead of root-level cache control through Cloudflare.
- `217b05b` stabilized visible tool-search offers; `9b3d024` persisted the tools
  asset in the epoch. Existing epoch, in-place result trimming, and explicit
  reset behavior remain unchanged.
- The previous complete Anthropic sweep reported 4/4 completed root turns,
  16/16 append-extension pairs and 89.9% cache-read share. These are historical
  results, not a new benchmark of this patch. The earlier 95.1% partial corpus
  must not be substituted for the complete run.

## Changes

- Native Anthropic requests with `thinking.type=adaptive|enabled` add
  `block_binding.prefix_mismatch_behavior=drop_block` and merge the binding
  beta with the existing fallback beta before request digest computation.
  Compatible non-Anthropic dialects do not inherit these controls.
- Preserve local signed and encrypted blocks, including across model changes.
  Let the server drop invalid blocks per request; do not mutate durable history
  or disable thinking to work around intentional prefix changes.
- Retain `input_transformations` from JSON, streaming message start, and final
  message delta. Unknown report entries are retained. Worker records the report
  as runtime-visible, spill-capable diagnostics, outside the cached context.
- Add offline replay/telemetry coverage, a runtime-context invariance test,
  and a three-request live binding probe (initial, append-only, changed system).

## Verification

- `cargo check -p provider` and `cargo check -p tekes-worker --locked`: passed.
- `cargo test -p provider --locked`: passed, including 12 Anthropic alignment
  tests. Coverage includes empty signed thinking, signature deltas, encrypted
  blocks, exact replay, and JSON/SSE transformation reports.
- `TEKES_KERNEL_FIXTURES="$PWD/fixtures" cargo test --workspace --all-targets
  --locked --no-fail-fast`: passed. Log:
  `/tmp/kernel-anthropic-binding-workspace-20260909.log`.
- The final diagnostic helper was additionally verified with
  `cargo test -p tekes-worker --locked
  input_transformation_diagnostics_do_not_change_the_cached_context`: passed.
  Log: `/tmp/kernel-anthropic-binding-worker-20260909.log`.
- Live Opus 5 smoke: passed, two rounds including tool-result continuation.
  The gateway accepted the new controls and returned `input_transformations:[]`.
  Evidence: `/tmp/kernel-anthropic-opus-binding-20260909/receipt.json` and raw
  responses in that directory. This run had no thinking blocks or cache reads;
  it proves neither signed-prefix enforcement nor a new cache-hit rate.
- Live Fable 5.1 binding probe: failed on model availability, not binding.
  Evidence: `/tmp/kernel-anthropic-binding-20260909/binding-0.response.sse`
  and `test.log`. That first receipt retained `running` after test failure;
  the runner now marks existing receipts `failed` on unsuccessful runs.

Once the configured route serves Fable 5.1, rerun:

```sh
python3 scripts/run-live-anthropic-thinking.py --model claude-fable-5-1 \
  --binding --output /tmp/kernel-anthropic-binding-rerun
```

The probe must observe signed thinking, unchanged-prefix telemetry, and a
prefix-mismatch report after changing the system. A full Kernel route sweep
with system tools and compaction remains a separate end-to-end acceptance
gate; neither the Opus smoke nor offline tests replace it.
