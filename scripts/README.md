# Scripts

Start with `scripts/ci.sh`. It runs the hermetic check set that every pull
request must pass: no network, provider credentials, signing identities or
sibling checkouts.

```bash
scripts/ci.sh
```

Everything else here is either a narrower slice of that set, a fixture tool,
or a live/benchmark runner that spends provider tokens.

## Hermetic checks (part of `ci.sh`)

| Script | Purpose |
|---|---|
| `check-*.py` | Validate one fixture corpus each (byte-canonical JSON, closed tables, manifest agreement). No arguments. |
| `test_cache_corpus.py`, `test_agent_eval_audit.py`, `test_agent_eval_scored.py`, `test-audit-live-python-task.py`, `test-audit-live-task-validation.py` | Unit tests for the live-run audit and scoring helpers. |
| `code-architecture.py --check` and `--links` | Verifies the generated architecture atlas. Needs `pip install -r scripts/architecture-requirements.txt`. Run without `--check` to regenerate after code changes. |

## Narrower gate runners

`ci-slice*.sh` and `ci-provider-dialects.sh` run the conformance gates of one
implementation slice (for example `ci-slice13.sh` for MCP). They chain earlier
slices and overlap with `cargo test --workspace`; use them to iterate on one
area. `ci-slice12.sh` and `ci-slice14a.sh` also run real-reference
qualification when `TEKES_COMPUTER_USE_ARCHIVE` or signed executables are
provided, and report "not qualified" otherwise.

## Tests that need a build or macOS tooling

| Script | Needs |
|---|---|
| `test-builtin-launch.py` | `cargo build` first (`target/debug/tekes-supervisor`, `tekes-worker`) |
| `test-builtin-client-contract.py` | macOS with `swiftc` (Swift 6) |

## Fixture tools

| Script | Purpose |
|---|---|
| `update-fixture-manifest.py` | Regenerate `fixtures/manifest.json` from the files on disk. |
| `add-provider-model-proof-alias.py` | Derive one exact model proof row from a reviewed route proof. |
| `replay-legacy-sqlite.py` | Replay a legacy SQLite session export into a ledger (migration aid). |
| `diff-attempt-requests.py`, `audit-request-segments.py` | Inspect recorded provider request bodies. |

## Live provider runs (spend tokens)

These call real provider APIs. They never run in CI.

| Variable | Meaning |
|---|---|
| `TEKES_PROVIDERS_CONFIG` | Path to a `providers.json` with your provider routes (or pass `--config`). |
| `TEKES_LIVE_KEYS_FILE` | Path to a key file of `"name": "value"` lines (or pass `--keys`). |
| `TEKES_KERNEL_LIVE_KEY` | A single provider key; overrides the key file where supported. |

| Script | Purpose | Typical cost |
|---|---|---|
| `run-live-*.py` | One provider/dialect behavior each (schema, thinking, cache, continuation, native deferred tools, process lifecycle) | cents per run |
| `run-public-flow.py --live` | One corpus through a real worker and the public client | $0.01–0.05 per run on DeepSeek Flash |
| `run-swebench.py` | SWE-bench-style instances; grading needs the official harness | depends on instances |
| `run-agent-eval-scored.py` | Scored agent-eval tasks | depends on tasks |

## Cost benchmarks

| Script | Purpose |
|---|---|
| `record-provider-proxy.py` | Local proxy that records every request body and the provider's raw usage. Point a run's base URL at it. |
| `run-pi-long-session-cache.py` | Run a corpus through pi in RPC mode (DeepSeek Responses API). |
| `run-dsh-long-session-cache.py` | Run a corpus through DSH's SDK profile. |
| `price-proxy-runs.py` | Price proxy-recorded runs and split them by turn. |
| `price-live-run.py`, `price-cache-audit.py`, `audit-long-session-cache.py` | Price one Kernel run from its ledger, or a Kernel/DSH pair. |
| `batch_ledger_bugfix_acceptance.py`, `weighted_ttl_acceptance.py` | Hidden acceptance checks for the two benchmark corpora. |

Compare at least six interleaved runs per arm; single runs of the same
configuration differ by up to 2x.
