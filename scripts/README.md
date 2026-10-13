# Scripts

Start with `scripts/ci.sh`. It runs the hermetic check set that every pull
request must pass: no network, provider credentials, signing identities or
sibling checkouts.

```bash
scripts/ci.sh
```

The main scripts fall into the groups below: narrower slices of that set,
fixture tools, and live or benchmark runners that spend provider tokens.
Helpers that these scripts call are not listed.

## Hermetic checks (part of `ci.sh`)

| Script | Purpose |
|---|---|
| `check-*.py` | Validate one fixture corpus each (byte-canonical JSON, closed tables, manifest agreement). Run without arguments. `check-endpoint-authority.py` also compares the vendored corpus with a sibling `../Tekes` checkout when one exists, and skips that step otherwise. |
| `test_cache_corpus.py`, `test_agent_eval_audit.py`, `test_agent_eval_scored.py`, `test-audit-live-python-task.py`, `test-audit-live-task-validation.py` | Unit tests for the live-run audit and scoring helpers. |
| `code-architecture.py --check` and `--links` | Verifies the generated architecture atlas. Needs `pip install -r scripts/architecture-requirements.txt`. Run without `--check` to regenerate after code changes. |

## Narrower gate runners

`ci-slice*.sh` and `ci-provider-dialects.sh` run the conformance gates of one
implementation slice (for example `ci-slice13.sh` for MCP). Most chain earlier
slices, and they overlap with `cargo test --workspace`; use them to iterate on
one area. `ci-slice12.sh` and `ci-slice14a.sh` also run real-reference
qualification when `TEKES_COMPUTER_USE_ARCHIVE` or signed executables are
provided, and report "not qualified" otherwise.

`ci-linux-sandbox.sh` runs Linux backend unit tests, public helper/job
runtime tests and helper/probe/approval gates on a native Linux x86_64 or
aarch64 host. By default it requires bubblewrap/seccomp, Landlock/seccomp
and automatic backend selection to pass independently, so automatic
fallback cannot hide a broken backend. A missing test suite or unavailable
requested backend fails the run. The tests exercise confined processes and
filesystem/network denials without provider credentials or external
network access. GitHub's Linux jobs install bubblewrap and run this script
on native `ubuntu-24.04` (x86_64) and `ubuntu-24.04-arm` (aarch64) runners
before the complete workspace suite:

```bash
scripts/ci-linux-sandbox.sh --backend all
```

The bubblewrap pass also runs its explicitly selected lifecycle tests:
helper cancellation and timeouts, stopping jobs after reopening the broker,
and detached jobs surviving the launching supervisor. Namespace identity
tests verify that forged or stale process metadata cannot authorize signals
and writable roots cannot cover the broker's private launch receipts.

Use `--backend bubblewrap`, `--backend landlock` or `--backend auto` for a
host with only that capability; these runs report only the selected path.
The selector `TEKES_SANDBOX_TEST_BACKEND` is interpreted by integration
tests only, not by production code. Bubblewrap needs version 0.4.0 or later,
working unprivileged user/mount/PID/IPC/UTS namespaces, network namespaces
for network-denied tools, and seccomp. Landlock needs ABI 3 or later (to
confine truncation) and seccomp. Neither backend requires running Kernel as
root.

The script prints the kernel and, when readable, enabled LSMs for diagnosis.
An unreadable `/sys/kernel/security/lsm` alone does not indicate missing
support; actual launch probes decide. Host policy may restrict unprivileged
user namespaces even when bubblewrap is installed. On Ubuntu, inspect the
distribution's bubblewrap AppArmor profile instead of disabling AppArmor
or user-namespace restrictions globally; see
[Ubuntu's explanation](https://discourse.ubuntu.com/t/understanding-apparmor-user-namespace-restriction/58007).
The test script never installs packages, changes host configuration or
restarts services. On a shared server, use a separate checkout and target
directory, and bound build/test concurrency using `CARGO_BUILD_JOBS` and
`RUST_TEST_THREADS`.

## Tests that need a build or macOS tooling

| Script | Needs |
|---|---|
| `test-builtin-launch.py` | `cargo build` first (`target/debug/tekes-supervisor`, `tekes-worker`) |
| `test-builtin-client-contract.py` | macOS with `swiftc` (Swift 6), a `cargo build`, and a TekesClientKit checkout (`TEKES_CLIENT_KIT_ROOT`, default `../TekesClientKit`). |

## Fixture tools

| Script | Purpose |
|---|---|
| `update-fixture-manifest.py` | Regenerate `fixtures/manifest.json` from the files on disk. |
| `replay-legacy-sqlite.py` | Validate a copy of a predecessor TekesAppServer SQLite database and export transcripts for `crates/endpoint/tests/legacy_replay.rs`. Only useful with such a database. |
| `diff-attempt-requests.py`, `audit-request-segments.py` | Inspect recorded provider request bodies. |

## Live provider runs (spend tokens)

These call real provider APIs. They never run in CI.

| Variable | Meaning |
|---|---|
| `TEKES_PROVIDERS_CONFIG` | Path to a `providers.json` with your provider routes (or pass `--config` where the script supports it). |
| `TEKES_LIVE_KEYS_FILE` | Path to a key file of `"name": "value"` lines (or pass `--keys` where the script supports it). |
| `TEKES_KERNEL_LIVE_KEY` | A single provider key; overrides the key file where supported. |

| Script | Purpose | Typical cost |
|---|---|---|
| `run-live-*.py` | One provider, dialect or runtime behavior each (for example schema, thinking, cache, continuation, native deferred tools, process lifecycle, MCP, provider error queue) | cents per run |
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
| `bench/` | Reproducible Kernel vs pi cost kit; see [`bench/README.md`](bench/README.md). |

Compare at least six interleaved runs per arm; single runs of the same
configuration differ by up to 2x.
