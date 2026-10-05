# Full enabled-model context retest

User requested retesting all models after the context configuration update.
Scope:14 enabled model/route combinations across8 providers. GPT uses258000
context/232200 conservative compact threshold; other enabled models use
1000000/900000. Disabled legacy models are excluded.

Complete unchanged weighted-ttl four-turn corpus for every combination. Each
test receives a snapshot configuration containing its selected model only;
the user's persistent default ordering is unchanged. Subsequent source fixes
and model substitutions are recorded below.
The original driver schedules four concurrent runs under caffeinate idle-sleep
prevention; supplementary corrected runs are tracked separately.

Artifacts: `/tmp/kernel-context-sweep-20260907`.
`manifest.json` identifies all14 jobs; `providers-snapshot.json` freezes input
configuration. `status-NN.json` records PID, timestamps and terminal code;
each `run-NN` retains binary hashes, provider/case, ledger and request assets.
Driver session4932 exited0 after all original jobs reached terminal status.
Final verification completed on September8: the effective14-combination matrix
has12 completed four-turn runs, Fable5.1 blocked by a first-request HTTP404, and
Sol Pro cancelled by the user. Historical Fable5 and failed pre-repair DeepSeek
Flash runs are retained but do not substitute for their replacement results.
Configuration validation is not proof of gateway long-context acceptance.

| ID | Model | Route |
|---|---|---|
|01|claude-fable-5|Anthropic via Cloudflare|
|02|claude-opus-5|Anthropic via Cloudflare|
|03|gpt-5.6-luna|Responses via Cloudflare|
|04|gpt-5.6-sol|Responses via Cloudflare|
|05|gpt-5.6-sol pro alias|Responses via Cloudflare|
|06|gpt-5.6-terra|Responses via Cloudflare|
|07|deepseek-v4-flash|DeepSeek Responses|
|08|deepseek-v4-pro|DeepSeek Responses|
|09|deepseek-v4-flash-vision-exp|DeepSeek Responses|
|10|glm-5.3|GLM Chat|
|11|gemini-3.8-flash|generateContent|
|12|kimi-k3|Chat|
|13|kimi-k3|Anthropic|
|14|gemini-3.8-flash|Interactions|

## Mid-run corrections and user scope update

- User clarified Sol Pro is Sol with a Pro flag. Confirmed official Responses
  API documentation: https://developers.openai.com/api/docs/guides/reasoning#reasoning-mode
  The failed run05 transmitted the internal alias as the model. Corrected the
  exact capability entry to map to `openai/gpt-5.6-sol` with `reasoning.mode=pro`,
  independently of effort. Local first/continued-request tests pass, including
  no explicit effort and the ordinary Sol negative case.
- Started `/tmp/kernel-context-sol-pro-fixed-20260907`, but the user then asked
  to stop Pro testing because of cost. Its owned processes were terminated and
  handle49499 returned143. This is user-cancelled, not a successful or failed
  post-fix provider acceptance proof. No further Pro live requests are authorized.
- User requested adapter compatibility for DeepSeek string `"null"` arguments.
  Enabled the existing unique-valid schema repair for DeepSeek Responses, with
  the raw sealed response retained. The parameterized GLM/DeepSeek ready/terminal/
  reopen test passes; non-opted-in OpenAI remains unchanged. Existing schema
  gates cover valid literals, ambiguous choices and unrelated invalid arguments.
- Baseline run07 was stopped after33 repeated schema validation errors and no
  first-turn completion; its operator-stop.json records the reason. New complete
  Flash corpus `/tmp/kernel-context-deepseek-null-fixed-20260907` (handle96714)
  tests the requested repair. It must be reported separately from baseline.
- Full workspace regression for the two code changes passed (exit0) at
  `/tmp/kernel-context-fixes-regression-20260907.log`,handle59583. The sweep
  freezes binaries per launch; pending jobs that launch after these edits can
  have different hashes. Preserve those hashes in the final report.
- User replaced Fable5 with Fable5.1. Persistent configuration now selects
  `claude-fable-5-1` and its matching capability profile, retaining1000000/900000.
  The existing profile already supports its adaptive thinking and `auto` tool
  choice. Baseline run01 remains historical Fable5 evidence. Its replacement
  four-turn corpus runs at `/tmp/kernel-context-fable51-20260907`,handle23279,
  using `/tmp/kernel-context-fable51-20260907-config.json`.
  This replacement terminated with exit1 on its first request: HTTP404
  `not_found_error`, `model: claude-fable-5-1` (root error seq9, settle seq10).
  The official model page confirms this exact API ID and1M context:
  https://platform.claude.com/docs/en/models/fable-5-1/overview
  Configuration is updated, but availability through the configured gateway
  is unproven; there is no usable cache-rate result for Fable5.1.

## Verified terminal results

The following runs have four completed root turns, a passed public-flow receipt,
verified request-asset hashes and independently passing host unit tests. Cache
rate is total provider-reported cache-read input divided by total input across
root executor requests, including initial requests and any epoch transitions.
It does not measure reusable-prefix coverage and excludes validator children.

|Model|Cache rate|Root requests|Epochs|Host tests|
|---|---:|---:|---:|---:|
|Fable5 (historical; replaced by5.1)|88.42%|13|1|44|
|Opus5|97.38%|64|1|95|
|Luna|96.72%|55|1|11|
|Sol|91.30%|24|2|17|
|Terra|91.70%|24|1|15|
|DeepSeek Flash (quoted-null repair)|97.97%|45|1|45|
|DeepSeek Pro|97.60%|28|1|26|
|DeepSeek Flash Vision Exp|98.42%|34|1|45|
|GLM5.3|94.19%|25|1|85|
|Gemini generateContent|80.94%|52|1|30|
|Gemini Interactions|93.01%|46|1|62|
|Kimi Chat|92.80%|21|1|51|
|Kimi Anthropic|89.83%|15|1|56|

Detailed evidence: `/tmp/kernel-context-sweep-20260907/audit-results.json` and
each run's `completion-audit.json`. The effective replacement-aware matrix is
`/tmp/kernel-context-sweep-20260907/effective-final-audit.json`; all12 completed
effective runs have zero trim and zero compaction events.
Luna has three calls aborted after execution
started with a crash outcome; successful final completion does not erase those
recoveries. Sol's second epoch is recovery, not trim or compaction. Host tests
are generated per run, so their counts do not imply equal correctness coverage.

All requested live runs are terminal, including the explicit Sol Pro cancellation.
Corrected DeepSeek Flash completed all four turns, with zero trim/compaction;
its tool outcomes include38 successes and13 errors. The compatibility repair
does not repair unrelated invalid arguments or incorrect working directories.
The remaining errors are10 helper-root denials, one call with two real command
forms, and two unknown-property validation failures (`wrapping_directory` and
`output_mode`). They are not repeated repairable quoted-null failures.
DeepSeek's supplementary ledger proves at least11 repaired calls
with successful execution, preserving the original sealed arguments:
`/tmp/kernel-context-deepseek-null-fixed-20260907/quoted-null-live-proof.json`.
Full completion evidence is in the supplementary run's `completion-audit.json`.
Pairwise auditing checks server continuation IDs against the preceding output's
recorded continuation, not merely their presence. All completed runs are fully
continuous except Sol's documented recovery transition (22 of23 pairs).

These task runs do not probe the gateway at the1M-token limit. The runtime
still compares serialized request bytes against the configured compact trigger,
using its existing conservative one-byte-per-token estimate. Configuring1M does
not prove actual1M-token gateway acceptance or change that estimator. Validator
results may remain inconclusive even when the root completes and host tests pass.
Model-generated trajectories and total input sizes differ despite the unchanged
four-turn corpus; these rates are observations, not an isolated causal comparison
of providers or context-window sizes.

## Commit validation

The final runtime changes were committed as `02438f7` on September8. Before
commit, `TEKES_KERNEL_FIXTURES="$PWD/fixtures" cargo test --workspace --all-targets
--locked --no-fail-fast` passed again (exit0), with its log at
`/tmp/kernel-commit-regression-20260908.log`. The three changed Python scripts
also passed syntax parsing. This local regression did not invoke paid live tests.
