# Original workflow cache corpus

These inputs preserve all four scenarios from the frozen TekesAppServer
`cacheCorpusPrompts` helper: hello, DeepSeek Responses smoke, weighted TTL,
and quota ledger. `source.swift` retains the original helper; `provenance.json`
identifies its source revision and file hash. `prompts.json` applies only Swift
multiline-string whitespace and interpolation semantics.

Use `scripts/cache_corpus.py` to select an exact scenario. Unknown names fail
closed; a successful Kernel root completion requires a durable `completed`
settlement. This replaces the legacy `fact` success label and additionally
rejects absent, interrupted or held completions.

The quota corpus has four turns. Preserve no child tasks,
exactly-once test execution and the final partial contention-wave instructions.
The Python 3.14 executable requirement is original input and has not been
silently substituted. Running the corpus gate does not prove model execution,
artifact correctness, or measurable cache attribution.

Verification: `python3 -m unittest discover -s scripts -p test_cache_corpus.py -v`.

Run a real public-interface corpus with:

```sh
python3 scripts/run-public-flow.py --live --corpus quota-ledger --toolchain-root /opt/homebrew --output /tmp/tekes-corpus-run
python3 scripts/audit-cache-corpus.py /tmp/tekes-corpus-run
```

The client submits the next prompt only after the previous promoted final and
completed turn. It saves the public `usage.summary` and
`usage.cacheAttribution` replies. The independent audit checks successful
unittest output instead of treating a settled turn as execution proof. A host
Python installation alone is insufficient: the declared execution environment
must make the original interpreter available to sandboxed shell tools.

The example explicitly authorizes the local Homebrew installation. Supply the
actual intended toolchain root on other hosts; the runner does not inherit one.

## Existing-repository bugfix probe

`batch-ledger-bugfix` is a separate controlled fixture, not part of the frozen
four-scenario corpus above. Both runners seed the same README, source, and five
tests. The baseline has two failing tests. Three sequential prompts ask the
agent to repair the batch transaction, extend regression coverage, and verify
the result. After a run, use
`scripts/batch_ledger_bugfix_acceptance.py <run>/workspace/ledger.py` for
behavioral checks that were not supplied to the agent. Use
`scripts/price-live-run.py` to price a single Kernel ledger without a DSH pair.
