# Cost benchmark kit

Reproduces the Kernel vs pi cost comparison in
[docs/benchmarks/deepseek-cost-2026-10.md](../../docs/benchmarks/deepseek-cost-2026-10.md).
It runs both agents on the same multi-turn coding task, records every provider
request and the provider's own usage through a local proxy, and compares cost,
cache hit rate and test quality.

## Requirements

- A DeepSeek API key in `TEKES_KERNEL_LIVE_KEY`. Nothing writes it to the outputs.
- pi 1.0.x: `npm install --ignore-scripts @earendil-works/pi-coding-agent@1.0.0`,
  then `PI_BIN=<prefix>/node_modules/.bin/pi`.
- `python3.14` on `PATH`. The corpus prompts name it, and the agents run the
  tests with it. `TOOLCHAIN_ROOT` (default `/opt/homebrew`) must be the prefix
  whose `bin/` contains it, so Kernel's shell sandbox can reach it.
- macOS (Kernel's shell sandbox) and a Kernel build (the runner builds it).

Expect about $0.01–0.03 per run on `deepseek-flash` at off-peak rates; 6 + 6
runs of one corpus cost well under $0.50.

## Run

```sh
scripts/bench/run_interleaved.sh batch-ledger-bugfix 6 /tmp/bench-batch
python3 scripts/price-proxy-runs.py /tmp/bench-batch
python3 scripts/bench/stats.py /tmp/bench-batch K P
python3 scripts/bench/quality_batch_ledger.py /tmp/bench-batch/[KP][0-9]*
```

For `weighted-ttl`, use `python3 scripts/weighted_ttl_acceptance.py
<run>/workspace/weighted_ttl_cache.py` for the hidden acceptance suite and
`python3 scripts/bench/mutation.py <run>...` for the mutation score.
`price-proxy-runs.py` runs the batch-ledger acceptance check, so ignore its
acceptance column for `weighted-ttl`.

## What each step measures

| Step | Output |
|---|---|
| `run_interleaved.sh` | Alternates Kernel run `K<i>` and pi run `P<i>`. Each run has `<run>-proxy/request-N.json` (request body) and `response-N.json` (HTTP status and the provider's raw `usage`). |
| `price-proxy-runs.py` | Splits each run by user turn using that system's own log, prices it with DeepSeek's off-peak rate card ($0.003 cached input, $0.15 uncached input, $0.60 output per million tokens), checks the system's own usage against the proxy, and writes `analysis.json`. |
| `stats.py` | Means, ranges, Welch t and an exact one-sided permutation p-value for each metric. |
| `quality_batch_ledger.py` | Runs each run's final tests against the seed implementation, which still has the bug. Useful regression tests fail there. |
| `mutation.py` | Injects single-point bugs into each run's own module and counts how many its own tests catch. |

## Rules

- Compare at least six interleaved runs per arm. Identical configurations differ
  by up to 2x between single runs.
- Price both arms with the same rate card. DeepSeek bills peak hours (01:00–04:00
  and 06:00–10:00 UTC, Monday to Friday) at twice the off-peak rate, which
  changes absolute dollars but not ratios.
- Report the model id the requests used and the date: provider model ids can
  route to new versions.
