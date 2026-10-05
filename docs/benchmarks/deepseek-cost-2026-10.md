# Cost and cache hit rate on DeepSeek Flash: TekesKernel vs pi (October 2026)

[Documentation home](../README.md) · [Reproduction kit](../../scripts/bench/README.md) · [Run data](data/2026-10/)

## Summary

On two multi-turn Python coding tasks, with the same model and the same API,
TekesKernel completed the work at **39% and 56% lower cost** than pi 1.0
(6 interleaved runs per arm per task; one-sided permutation p = 0.013 and
0.001). Every run on both sides passed a hidden acceptance suite.

Three findings matter more than the headline numbers:

1. **Cache hit rate does not explain cost.** Both agents keep a stable prompt
   prefix and reach 95–99% cache hits. Output tokens (mostly reasoning) are
   78–85% of the bill on both sides.
2. **The cost difference comes from how much the model thinks and verifies.**
   Kernel's built-in coding profile carries four behavior rules that stop
   unrequested verification loops. In a controlled A/B on Kernel itself they cut
   cost by 55% and 59% (p = 0.001) without lowering test quality.
3. **pi wrote more thorough tests on the larger task.** On weighted-ttl, pi's
   test suites were larger (72 vs 46 tests on average) and caught more injected
   bugs (95.0% vs 88.3%, p = 0.017). Both passed every hidden acceptance check.
   Part of Kernel's saving on this task is less testing.

## Setup

| | TekesKernel | pi |
|---|---|---|
| Version | Development build between releases 0.1.2 and 0.1.4 (2026-10-04); the coding profile and title fixes it tested ship in 0.2.0 | `@earendil-works/pi-coding-agent` 1.0.0 from npm |
| Interface | `scripts/run-public-flow.py` (public Session Endpoint client) | RPC mode, one process for all turns |
| System prompt | Built-in `coding` profile, about 670 characters with the working directory ([System prompt](../runtime/system-prompt.md)) | pi's default prompt with `--no-skills`, about 3,050 characters |
| Tools | `apply_patch`, `edit`, `glob`, `grep`, `read`, `shell`, `write` | `read`, `bash`, `edit`, `write` (pi defaults) |
| Extra requests | One session-title request (reasoning off, ≤ 64 output tokens) | None |

Common to both:

- Model `deepseek-flash` (served as DeepSeek V4.1 Flash on the run date),
  reasoning effort `high`, DeepSeek Responses API (`/responses`).
- Runs on 2026-10-04 23:37 – 2026-10-05 00:41 UTC, Kernel and pi alternating
  (K1, P1, K2, P2, …), one at a time, on one macOS machine.
- Every request went through a local recording proxy that stored the request
  body and the provider's raw `usage`. Both agents' own usage logs matched the
  proxy exactly.

### Tasks

| Corpus | Turns | Work | Hidden acceptance |
|---|---|---|---|
| `batch-ledger-bugfix` | 3 | Fix a known bug in an existing two-file project, add focused regression tests, final review | 3 behavior checks of the batch transaction |
| `weighted-ttl` | 4 | Write a weighted TTL + LRU cache and its tests from scratch, audit it, add a feature, final review | 11 behavior checks of the cache |

Prompts and seed files are in [fixtures/live/cache-corpus/](../../fixtures/live/cache-corpus/).

### Pricing

Costs are estimates from provider-reported tokens using DeepSeek's published
off-peak rate card for `deepseek-flash`: $0.003 per million cached input
tokens, $0.15 per million uncached input tokens, $0.60 per million output
tokens ([source](https://api-docs.deepseek.com/quick_start/pricing/), fetched
2026-10-04). Peak-hour prices are exactly double, so ratios do not change.
Part of these runs fell into peak hours; the actual bill was higher than the
estimate for both arms alike.

## Results

### batch-ledger-bugfix (6 vs 6)

| Metric | Kernel | pi | Kernel / pi | One-sided p |
|---|---:|---:|---:|---:|
| Cost per run, all turns | $0.0123 (0.0079–0.0190) | $0.0204 (0.0148–0.0265) | 0.61 | 0.013 |
| Cost, follow-up turns | $0.0080 | $0.0157 | 0.51 | 0.012 |
| Output tokens | 16,861 | 28,705 | 0.59 | 0.014 |
| Reasoning tokens | 13,095 | 21,705 | 0.60 | 0.022 |
| Uncached input tokens | 10,470 | 12,080 | 0.87 | 0.067 |
| Cache hit rate | 94.96% | 97.16% | −2.2 pts | — |
| Model requests | 15.7 | 21.8 | 0.72 | 0.005 |
| Tool calls | 16.2 | 23.2 | 0.70 | 0.009 |
| Hidden acceptance | 6 / 6 | 6 / 6 | | |
| New tests that fail on the original bug | 3–4 per run | 4–5 per run | | |

### weighted-ttl (6 vs 6)

| Metric | Kernel | pi | Kernel / pi | One-sided p |
|---|---:|---:|---:|---:|
| Cost per run, all turns | $0.0169 (0.0132–0.0241) | $0.0381 (0.0242–0.0741) | 0.44 | 0.001 |
| Cost, follow-up turns | $0.0086 | $0.0268 | 0.32 | 0.001 |
| Output tokens | 23,432 | 49,343 | 0.47 | 0.002 |
| Reasoning tokens | 10,547 | 25,421 | 0.41 | 0.007 |
| Uncached input tokens | 10,650 | 21,835 | 0.49 | 0.009 |
| Cache hit rate | 97.28% | 98.57% | −1.3 pts | — |
| Model requests | 20.2 | 37.5 | 0.54 | 0.002 |
| Tool calls | 19.8 | 35.0 | 0.57 | 0.011 |
| Hidden acceptance | 6 / 6 | 6 / 6 | | |
| Tests in the final suite | 46.3 (34–76) | 71.8 (57–87) | 0.65 | |
| Mutation score (pooled) | 88.3% (159 / 180) | 95.0% (190 / 200) | | 0.017 |

Per-run data: [batch-ledger-bugfix](data/2026-10/batch-ledger-bugfix.analysis.json),
[weighted-ttl](data/2026-10/weighted-ttl.analysis.json),
[mutation scores](data/2026-10/weighted-ttl.mutation.json).

## Where the cost goes

| Share of cost | Kernel, batch | pi, batch | Kernel, ttl | pi, ttl |
|---|---:|---:|---:|---:|
| Output tokens | 82.0% | 84.6% | 83.3% | 77.6% |
| Uncached input | 12.7% | 8.9% | 9.5% | 8.6% |
| Cached input | 5.3% | 6.5% | 7.2% | 13.8% |

Output dominates. A lower cache hit rate is not a cost problem by itself:
Kernel's hit rate is lower because it sends fewer requests, so less history is
replayed from the cache, while its absolute uncached input is lower too.

The largest share of output is reasoning in a few long responses. In earlier
analysis on batch-ledger, the extra cost of the previous Kernel version came
almost entirely from responses with 2,000 or more reasoning tokens, mostly in
the regression-test turn. There, the bug was already fixed, the new tests
passed at once, and the model kept searching for a defect: it worked out test
outcomes at length in its reasoning, wrote ad-hoc fuzz scripts, and ran
differential checks against its own reference.

## The coding profile rules

Kernel's coding profile ([crates/tools/prompts/coding.md](../../crates/tools/prompts/coding.md))
adds four sentences after the identity line:

1. When the requested change is done and the requested tests pass, stop and
   report; if no defect is found, say so instead of continuing to search.
2. Do not write extra fuzz, randomized, or differential test scripts unless
   asked.
3. Check expected results by writing and running tests, not by working out
   program output at length in reasoning.
4. Be concise; do not narrate before each tool call.

Same-binary A/B on Kernel, `deepseek-flash`, 6 vs 6 interleaved runs each:

| Corpus | Without the rules | With the rules | Change | p |
|---|---:|---:|---:|---:|
| batch-ledger-bugfix | $0.0248 | $0.0111 | −55% | 0.001 |
| weighted-ttl | $0.0439 | $0.0180 | −59% | 0.001 |

The rules removed most ad-hoc verification scripts (weighted-ttl: 23 → 0 over
six runs) and cut long-reasoning responses by 59–71%. On weighted-ttl the
mutation score of Kernel's own tests stayed level (89.3% without, 88.3% with
the rules). The gap to pi's 95.0% above therefore predates the rules: it is a
difference in how thoroughly each agent tests, not a side effect of them.

On GPT-5.6 luna (3 vs 3) the rules changed cost by −6%, which is not
significant, and quality did not drop; that model did not over-verify to begin
with.

## Fairness notes

- **Tuning.** Kernel's coding rules were written after analysing Kernel runs on
  `batch-ledger-bugfix`. pi ran with its defaults. `weighted-ttl` was not used to
  derive the rules and shows the larger difference.
- **pi configuration.** pi ran with `--no-skills` so that skills installed
  globally on the test machine stayed out of its prompt. An earlier run that
  included them (8,346-character prompt) cost the same on average ($0.0200 vs
  $0.0204 on batch-ledger).
- **Extra request.** Kernel's session-title request is included in its cost. It
  averaged $0.00002–0.00004 per run.
- **Test thoroughness.** Lower cost on weighted-ttl comes partly from fewer
  tests. Whether 46 tests are enough or 72 are too many depends on the user.
  Both met the hidden acceptance bar; pi's tests found more injected bugs.
- **Scope.** One model family, Python tasks, two corpora, one machine. Each run
  is noisy: identical configurations vary by up to 2x between single runs,
  which is why each arm has six interleaved runs.

## Not tested

- Models other than DeepSeek Flash in the Kernel vs pi comparison.
- `deepseek-v4-pro`, peak-hour behavior, non-Python tasks, and long sessions
  that trigger context compaction.
- Other agent harnesses at their current versions.

## Earlier findings

Before the coding rules, a one-line identity statement ("You are a coding agent
powered by the {model} model.") was the main lever on deepseek-v4-flash's
trajectory length in comparisons with DeepSeek Harness; per-tool usage
paragraphs had no measurable effect. Those runs used two to three samples per
cell and an older Kernel, so they are summarized here only as background.

## Reproduce

See [scripts/bench/README.md](../../scripts/bench/README.md). In short, with a
DeepSeek key, pi 1.0 and `python3.14` installed:

```sh
scripts/bench/run_interleaved.sh batch-ledger-bugfix 6 /tmp/bench-batch
python3 scripts/price-proxy-runs.py /tmp/bench-batch
python3 scripts/bench/stats.py /tmp/bench-batch K P
```
