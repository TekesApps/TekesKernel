# Kernel source and live-test alignment — 2026-09-04

[Verification](../README.md)

This audit is in progress. Changes are confined to TekesKernel. No commit or
publication is part of this audit.

## Documentation findings

Read all 21 crate guides and compared module declarations and public re-exports
with the source index. The generated atlas was stale: source references in
`profile/launch` even exceeded the current file length. Regeneration now covers
21 packages, 208 Rust files, 6,120 symbols and 187 artifacts. Both
`code-architecture.py --check` and `--links` pass using the existing
`/tmp/tekes-architecture-venv/bin/python` environment.

The worker guide omitted `usage_preview`; it now describes the presentation-only
estimate and its separation from accounting. The safepoint guide overstated
capture as the complete worktree: the implementation uses a fresh private Git
index and `git add -A`, so ignore rules apply, `.git` is excluded and nested
repositories are rejected. The guide now states these limits.

This establishes index freshness and these specific corrections. It is not yet
a completed behavioral audit of every crate or every specification assertion.

## Current execution evidence

Command:

```sh
python3 scripts/run-validator-exhaustion.py \
  --output /tmp/tekes-kernel-alignment-20260904-compact \
  --skill-before-write --compact-next-turn --reload-skill-next-turn
```

Passed with freshly built, frozen worker/helper and supervisor-test binaries.
The local scripted provider drives real worker and validator processes and an
independent WebSocket client. The saved ledger records queued input at seq 34,
first settlement at 38 (`inconclusive`), admission at 41, compact at 47, the new
epoch at 48, and second settlement at 71 (`pass`). The client received both
settlements with matching validation outcomes. Skill discovery/load after
compaction and three overlapping, isolated runtime sessions also passed.

Evidence directory: `/tmp/tekes-kernel-alignment-20260904-compact`. Binary hashes,
script snapshots, provider requests, durable ledgers and client receipts are
retained there. This run did not enable the stale-skill negative variant. It
does not establish manual compact request ordering, external model adherence
or installed application acceptance.

Deployment fixture checks passed (22 cases, 66 artifacts), the inventory-script
regression passed, and `git diff --check` passed.

## Remaining scope

The pinned legacy inventory check scanned 18 suites: 99 registered tests and
zero missing entries. There are still 36 `not_run` entries, 15
`partial_live_evidence` entries, and other explicitly qualified, failed or
blocked results. The successful rerun above preserves the existing partial
status; it does not close the old manual-compaction/live-model requirements.

Continue from each original test's assertions, including parameterized catalogs,
rather than counting one source function as one runtime case. The earlier task
also left child approval publication and ancestry authorization incomplete;
request-journal persistence alone does not close that public round trip.

## Child approval response follow-up

The production response authority now resolves child requests from the durable
request journal. It checks the child genesis identity and follows exact
parent-file/sequence/spawn-id bindings back to the session main ledger, rejects
invalid filenames and non-regular ledger files, and includes ancestor stop
state in response admission. The internal worker key is derived from this
binding while the public request lifecycle retains the root session UUID.

Permission holds also contain a `question` metadata object. The prior binding
validator treated any such field as a structured question, incorrectly rejecting
real edit approvals. It now distinguishes the `answer` scope.

`cargo test -p tekes-supervisor --test slice9_endpoint_carrier --locked` passes
all eight tests. The new test proves child-only append and journal resolution,
plus rejection without mutation for wrong spawn identity, sequence, parent
path, child identity and a self-parent reference. It uses the production edit
approval shape. The complete public live round trip remains open: production
request-journal publication is not yet wired, and child `Appended` notifications
are still filtered by the process host. These tests manually persist requests
and therefore do not claim automatic client delivery at that stage.

## Production actionable publication follow-up

Production `Appended` handling now reconciles root and child approval holds
into the durable request journal before notifying v3 subscribers. Child
actionables retain independent IDs and do not enter the root transcript.
Opening an actionable stream also recovers holds by following the durable
spawn tree, covering reconnect after publication was missed. Existing request
payloads are preserved and checked against their causal holds.

The response lifecycle accepts an already-observed resolution only when its
session, semantic sequence and outcome exactly match the author's receipt.
This closes the race between the Appended observer and response completion.
The focused race test passes; wrong sequence/outcome/session are rejected
without adding journal rows. All eight carrier tests pass, including automatic
root and child recovery without a manually inserted child request.

Two fresh process runs passed with `--public-approval`:
`/tmp/tekes-kernel-public-approval-20260904-1` and
`/tmp/tekes-kernel-public-approval-20260904-2`. The second also enabled
`--skill-before-write --compact-next-turn --reload-skill-next-turn`.
The fixture disables internal approval authoring; the independent WebSocket
client opens actionables, receives the real root `apply_patch` hold, answers
through `actionable-respond`, and records the matching response result. Helper
write, two settlements, queue release and compact/skill reload passed.
The runner now rejects process projection/reconciliation failure markers.

These runs use a local scripted provider. Their logs retain automatic-title
warnings because the disposable fixture has no configured DeepSeek Flash
model; title refinement is not asserted by this gate. A live ordinary-child
process receiving its approval through the external client remains to be run,
as do the other uncompleted legacy inventory entries.

## Ordinary child public approval and validator evidence

`run-live-process.py --scenario task --public-approval` now starts the real
production transport with the existing real-model task fixture. The independent
`task-approval-websocket-client.py` subscribes before execution, receives and
answers approvals exclusively over WebSocket, and records root completion.
The fixture's direct approval author is disabled. A read-only client audit
restricts approval to the fixture's task/file operations and disposable workspace.

The CF OpenAI route passed in `/tmp/tekes-kernel-public-child-20260904-1`:
one root task approval and two child file approvals, root validation pass,
and the saved deliverable. `audit-public-task-approvals.py` proves each received
request corresponds to exactly one semantic response in its owning ledger,
with matching request revision, resolution sequence, call and private target.

The three-dependent-Python-task run in
`/tmp/tekes-kernel-public-children-python-20260904-1` passed its public approval,
artifact and functional checks, but exposed an unnecessary repair. Its first
validator claimed the final prose did not call `task`, even though three task
calls and successful child joins were durable. Inspection of the frozen seed
confirmed that it had no execution records.

The validator seed now includes root tool calls/results and ordinary child
spawn/join facts through the candidate output sequence. Other turns, later
repair actions and host validator spawns are excluded. Evidence is bounded to
512 events and 256 KiB; exclusions by size or the secret scanner are explicitly
counted. Judge instructions distinguish action evidence from final-answer text
and do not infer missing child actions from this root-only evidence. A focused
test proves candidate/turn boundaries, host-spawn exclusion and bounded omission.

The corrected rerun in `/tmp/tekes-kernel-public-children-python-20260904-2`
passed: exactly three task calls, nine correctly routed public approvals,
12 frozen root execution facts, zero repair feedback, root settlement seq 52
with validation pass, and executable Python dependency checks. The first run
is retained as causal evidence; it is not replaced by the successful rerun.

The store guide also incorrectly assigned compaction to `store::rewrite`.
`RewriteKind` has only Fork and Redact. The guide now identifies worker-authored
checkpoint/compact events through `LockedLedger` as the automatic compaction path.

Ordinary-child public process approval is now evidenced. The remaining legacy
inventory entries and full code/document behavioral audit remain in progress.


## Permanent provider error and queued recovery

`/tmp/tekes-kernel-public-provider-400-20260904-4` passes the previously
worker-only permanent-400 workflow through a separate authenticated HTTP and
WebSocket client, real worker processes, and the production Kernel v3 carrier.
The client submits both inputs. The fixture delays the 400 until the second
input is durable. Assertions bind one first-turn attempt, the durable raw error,
public error/end/start ordering, a successful second-turn brief, and the promoted
recovered final answer. Exactly three provider requests occur. Binary hashes,
request bodies, client frames, and both receipts are retained with the run.

Earlier attempts are retained: run 1 lacked required fixture model limits;
run 2 omitted function-call stream events; run 3 incorrectly expected raw
provider wording in the sanitized public error. These were fixture corrections.
The gate checks public classification and recoverability separately from raw
ledger error detail. This is Kernel v3 semantic parity, not legacy stdio wire
compatibility. Run with `python3 scripts/run-live-provider-error-queue.py --output
<new-artifact-directory>`.


## Structured question publication and answer delivery

The production workflow stores the validated singular `ask_user_questions`
invocation in an answer hold. Carrier reconciliation incorrectly looked for
`question.questions`, so a real question could not be published. It now wraps
that invocation in the public frame's `questions` array. Permission metadata
continues to use the approval path, selected by hold scope.

The carrier regression covers null options and explicit options, exact public
payload, durable answer binding, journal resolution, and reopening without
reoffering an answered hold. All nine carrier integration tests pass in
`/tmp/tekes-kernel-carrier-question-regression.log`.

`/tmp/tekes-kernel-public-question-20260904-3` also passes the real worker and
independent HTTP/WebSocket client gate with `--question`. After permanent 400
recovery, the worker calls brief and asks a question. The client answers through
`actionable-respond`; the gate checks one durable response and successful tool
result, exact decoded answer in the provider continuation, matching response
revision and request identity, and final client delivery. Four provider requests
occur. Run 1 exposed an incorrect fixture expectation of an `accepted` field
in the v3 acknowledgment; run 2 passed before tightening exact provider-answer
comparison. These artifacts are retained. This additional regression does not
claim completion of unrelated pending legacy live tests.


## Thread, turn and worker termination boundaries

A focused follow-up found that the turn-flow pseudocode described same-process
next-turn looping. The current worker entry calls `open_ready_turn` at startup,
recurses only within `run_provider_turn_inner`, drains late deliveries, then
returns through `post_turn_exit`. The supervisor ensures a new run for runnable
queued input. The guide now distinguishes this implementation from the protocol's
permission for multiple turns per run. The public permanent-400 and question
process artifacts retain both run_start records, two settlements, and ordered
public turn notifications as runtime evidence.

Thread persistence, unique durable turn settlement, process exit, and parked
holds are now summarized together in the thread guide. This correction does not
claim all crash, stop, budget and application UI cases have received fresh live
acceptance; the larger legacy inventory remains open.


## Deferred tool schema activation

Review of the pending legacy initial-tool-search live test found a production
gap: the worker repeatedly called the resident-only `provider_catalog`, so a
successful search could authorize dispatch but never expose the deferred schema
to the next provider request. The worker now calls `provider_catalog_for_turn`,
which uses the same durable causal-offer predicate as dispatch. It preserves
backend dependency gating and immutable catalog order. The catalog digest is
recomputed by the existing provider path, including newly exposed schemas.

Nine dynamic-catalog integration tests pass in
`/tmp/tekes-kernel-deferred-catalog-regression.log`; the new regression covers
pre-result sequence boundaries, same-turn activation, wrong offered names,
cross-turn exclusion, missing backends, and reconstruction from reopened logs.
Worker compilation passes in `/tmp/tekes-kernel-deferred-worker-check.log`.
These are implementation/regression evidence, not completion of the pending
live test. Its native-provider reference path and two-turn legacy reuse remain
unverified. The Kernel contract requires a same-turn search offer; the legacy
second turn's currently-available-tool assumption must be audited explicitly,
not silently counted as equivalent.


## Real two-turn deferred-tool gate

The external-client fixture uses the two original initial-tool-search prompts,
a configured Cloudflare OpenAI Responses model, real worker processes and a
sandboxed external dynamic helper. The helper discovers one nonresident
`uat_marker` schema and returns the requested marker on invocation. The client
submits turn two only after receiving the first promoted final answer.

Runs 1–4 retained in `/tmp/tekes-kernel-public-tool-search-20260904-*` exposed:

- Discovery and derived exec bindings used `.` although helper cwd requires
  the empty relative path to name its authorized root. Both bindings are fixed;
  four focused supervisor binding tests pass.
- The disposable Python fixture depended on the Xcode runtime outside the tool
  sandbox. The shell replacement initially used macOS `/bin/sh`, which emits
  stderr when denied access to `/private/var/select/sh`. The fixture now invokes
  `/bin/bash --noprofile --norc`; sandbox restrictions are unchanged.
- Run 4 completed the first search/call/final but failed on the next turn with
  an orphan function-call-output error. `latest_compatible_epoch` searched all
  prior epochs and resurrected the pre-activation chain when the catalog reset.
  It now considers only the latest epoch. A regression proves an older matching
  digest cannot revive that chain. All 80 ordinary worker unit tests pass (one
  live test remains ignored) in `/tmp/tekes-kernel-hot-tool-worker-regression.log`.

Run 5 passes both turns: search/call sequences 9/16 and 33/40, four distinct epoch
records with alternating resident/activated catalog digests, successful helper
results and client delivery of FIRST_HOT_OK and SECOND_HOT_OK. The source now
also verifies model, system/tools digests and usage attribution for every attempt.
This gate uses Kernel's same-turn causal search rule, not permanent activation;
the unchanged second prompt led the model to search again. It does not establish
native provider tool-reference wire compatibility or legacy SQLite diagnostics.

Run 6 repeats the complete live gate with the additional per-attempt attribution
checks and early client-failure reporting; it passes. Authoritative receipts are
`/tmp/tekes-kernel-public-tool-search-20260904-6/runtime/receipt.json` and
`client-tool-search-receipt.json` in the same directory. The inventory now marks
this Kernel two-turn gate passed, while the separate native-reference tests
remain outstanding.


## Native deferred-tool protocol audit

The two independent native-reference tests have now been traced through their
original assertions and current provider request/decoder/replay code. Both
remain unimplemented and unrun; the marker gate is not a substitute. See the
[boundary-by-boundary gap audit](native-deferred-tools-gap.md). The provider guide
and normative adapter document now explicitly identify the outstanding
implementation without removing its requirement.


## Anthropic native custom-reference gate

The provider now has an explicit native preparation entry point with typed
catalog definitions and references. It validates schema digest, catalog revision,
source search call, successful result and declaration membership before emitting
Anthropic deferred tools and pure tool-reference content. Request identity is
computed after the native transformation. Thirty-one provider library tests pass.

`/tmp/tekes-native-anthropic-20260904-3` passes the original adapter-direct
LIVE-42 round trip through configured Cloudflare/claude-fable-5, including sealed
replay, shipping-tool invocation and usage. Earlier mixed-content rejection and
HTTP 429 are retained. The separate worker-routing and OpenAI protocol gaps
remain explicit in the [native audit](native-deferred-tools-gap.md).


## OpenAI native client-search gate

OpenAI native search preparation and JSON/SSE decoding now preserve client call
identity, object arguments and original native replay. Explicit native requests
use client-managed replay and emit bound tool_search_output schemas. Stale
references, mismatched source calls, unsupported dialects and changed streaming
identities are rejected. Thirty-two provider library and ten finality/decoding
tests pass in `/tmp/tekes-openai-native-regression.log`.

`/tmp/tekes-native-openai-20260904-1` passes the original adapter-direct LIVE-42
round trip through configured Cloudflare/OpenAI, including original user input,
native search call/output, loaded shipping tool and usage. Anthropic is still
passing after the shared change in `/tmp/tekes-native-anthropic-20260904-4`.
Both original native adapter tests are now evidenced. Automatic worker native
mode selection and integration remain explicitly open in the native audit.

## Public image attachment round-trip

The original 64x64 PNG and color question now travel through an independent
HTTP/WebSocket client, production Kernel v3 intake, content-addressed storage,
and a real worker with the ordinary tool catalog. The client omits a filename,
requires the promoted answer to be exactly `red`, and observes completed turn
settlement. The host independently verifies stored bytes against the original
fixture and requires provider usage.

The first OpenAI run exposed an actual catalog compatibility bug: a nullable
array using `uniqueItems` was advertised as strict and rejected with HTTP 400.
The adapter now preserves the original schema while disabling strict mode for
that constraint, with a focused regression for Responses and Chat Completions.
`/tmp/tekes-public-image-openai-20260904-2` passes with
`openai/gpt-5.6-luna`. The failed first run remains available. The entire legacy
four-provider matrix is still partial until each route is evidenced.

The first Anthropic run was rejected locally before HTTP: its dialect capability
list omitted images despite an existing native image encoder. Native Anthropic
now advertises image input; DeepSeek's Anthropic-compatible dialect remains
text-only. `/tmp/tekes-public-image-anthropic-20260904-2` passes with
`claude-fable-5`, including byte verification, exact public answer and settlement.

`/tmp/tekes-public-image-google-20260904-1` also passes, using the configured
`gemini-3.6-flash` model and ordinary tool catalog. Three routes are now evidenced.
The original OpenRouter route is not present in the configured provider list;
its test remains pending, not waived. The canonical Anthropic dialect fixture
now records image support, matching the production table and live observation.

Provider library and exact-dialect regression gates pass in
`/tmp/tekes-image-provider-regression3.log`. The fixture's sole semantic change
is native Anthropic image capability; its pinned proof digest was updated to
match. No proof mismatch is bypassed. The fixture canonicality verifier,
generated architecture/link checks and 99-test inventory check also pass.

The fourth image route now passes in
`/tmp/tekes-public-image-openrouter-20260904-1`, using the original
`qwen/qwen3-vl-235b-a22b-instruct` model, OpenRouter Chat Completions endpoint
and existing `openRouterKey` credential. Its isolated provider file does not
change installed configuration. The four-route image matrix is complete; the
endpoint and worker assertions are identical across all four routes.

## Router write smoke gates

`/tmp/tekes-openrouter-write-20260904-1/matrix.json` passes the Kernel semantic
port of OpenRouter's write smoke: exact `LIVE_OK` file, exact `LIVE_OK_DONE`
answer, one root turn, one successful validator and reported usage. This uses
Kernel `apply_patch/read`, not the retired Swift `write(category,name)` API.
The original harness permits a model override; this run uses the accessible
Qwen3-VL OpenRouter route proven by the image gate. It does not claim the legacy
default OpenAI upstream was exercised. Isolated configuration and credential
field selection are now explicit live-runner arguments.

A second run against a third-party OpenAI-compatible Responses relay with the
`openai/gpt-5.4-mini` model also passes the same assertions.

Ollama remains open: on 2026-09-04 the original localhost:11434 endpoint refuses
connections, and this host has no `ollama` executable, `/Applications/Ollama.app`,
or local model manifests. The required original `gemma4:12b` live runtime is
therefore unavailable; a hosted provider has not been substituted for this gate.

## Brief V2 mixed-antecedent gap

The original Cloudflare brief test (legacy LiveTests.swift:267) requires typed
intent/fact/constraint atoms, source aliases, goal references and a host-derived
effective completion set that excludes facts. Kernel's builtin v1 brief schema
instead has `source`, `canonical_user_goal`, `retained_items` and
`completion_criteria`; `WorkflowBackend` returns the validated invocation.
It has no V2 atom resolver or effective-completion rule. This gate remains open
as a versioned contract gap, not a provider failure or a passing semantic port.

## Chinese drama workflow gate

The raw original prompt and seeded skill are preserved under
`fixtures/live/drama/`. The Kernel skill port changes only the retired write
API references to `apply_patch` and full workspace paths. The three literal
task names, dependency names, paths, length bounds and final-delivery contract
remain intact. The production process fixture uses public approval responses;
`audit-live-drama.py` checks the saved deliverables and promoted answer in
addition to the existing frozen-artifact validation audit. Live execution is
not considered passed until all those checks succeed.

The first drama run exposes a dependency-source gap: the final child attempts
`read("story_brief")` and `read("episode_map")` rather than their completed
artifact paths, then drafts unrelated characters. The worker seeds the effective
task invocation but does not resolve named task inputs to completed outputs.
This is not a passing result even though all three files exist. The independent
final-outline audit also checks the final episode paragraph lengths; verifying
only the intermediate episode map would miss a contract violation.

The first drama run ultimately passes at root settle 122 after one real
validator rejection and same-turn repair. The promoted final body equals the
saved 2519-character outline, all 12 episode paragraphs are 101–112 characters,
and all 12 public approvals bind to the owning parent/child ledgers. Evidence:
`/tmp/tekes-public-drama-20260904-1/{drama-audit,validation-audit,public-approval-audit}.json`.

The named-input fix keeps effective invocations byte-for-byte intact and adds
host-authored `resolved_inputs` to new durable delegation seeds. Only earlier
completed tasks can supply mappings; ambiguous or unfinished named sources
reject, while direct paths remain unchanged. Existing seeds are reused without
re-resolution, including legacy seeds without mappings. A fresh second drama
run is required to verify the corrected seed reaches real child workers.

The second drama run confirms real children read both mapped upstream paths,
but exposes a repair/recovery defect: repeated task names were rejected as
ambiguous before emitting a tool result, exhausting the supervisor restart
limit. The stopped run and its repeated `run_start` evidence are retained in
`/tmp/tekes-public-drama-20260904-2/stopped-recovery-loop.json`.

Resolution now selects the latest preceding delegation for each task name.
An unfinished latest version cannot fall back to an older completed version;
that condition is a typed, recoverable tool validation error, not a process
failure. Existing immutable seeds still resume unchanged. All 80 worker unit
tests pass in `/tmp/tekes-task-input-recovery-tests2.log`, including the stale
completion regression. The drama audit permits re-execution of the same three
registered task names during repair while preserving each task's dependency
contract, completed upstream order, resolved path reads and final artifacts.

Ollama environment preparation now uses the official 0.33.3 macOS standalone
release, verified against its published checksum
`342db03df80bb9db84ff64246031bd5f70c09b59ff52fa5cc9aaae3476cc4a9d`.
The binary and model cache reside under Kernel `target/live-deps`; the loopback
service disables cloud inference. The original `gemma4:12b` model is being
pulled. No Ollama live gate is marked passed before actual workflow execution.

The third drama runtime completes at settle 82 with no validation feedback.
All dependent children successfully read the resolved upstream paths; nine
public approvals bind correctly. The final answer contains all saved prose,
with only leading/trailing line whitespace and spaces following bold Markdown
labels differing. The original byte-equality audit was stricter than the
complete-prose requirement. The corrected audit ignores only those layout
spaces, while explicit guard checks still reject missing headings, changed
words/punctuation, or removed paragraph boundaries. All runtime, frozen-artifact,
approval and drama audits pass in
`/tmp/tekes-public-drama-20260904-3/reviewed-result.json`. The original driver
failure log is retained. Final episode paragraphs are 80–90 characters, the
saved outline has 2219 characters, and the delivered answer has 2214.

The original Ollama model is now downloaded, digest
`4eb23ef187e2c5462566d6a1d3bbbc2f1346d0b4327cbb66d58fffbcc9b2b05c`,
recorded in `/tmp/tekes-ollama-model-evidence.json`. Its write smoke is running
in `/tmp/tekes-ollama-write-20260904-1`. The local endpoint requires no auth;
the adapter's secret-store placeholder is explicitly non-secret and provides
no authentication evidence. Configurable live-runner and worker deadlines
allow 600/540 seconds for local inference while preserving the default
240/180-second limits and all original outcome assertions.

The first Ollama smoke fails after repeated invalid unified-diff create calls
and a final `<channel|>` response; it does not create the required passing
artifact/answer. Evidence is retained in `/tmp/tekes-ollama-write-20260904-1`.
The public apply_patch description previously only said “V4A body-only”. It
now explicitly explains plus-prefixed creation content, omitted file/hunk
headers, update bodies and durable version tokens. The engine error also gives
a valid generic `+hello` example. Parser and write semantics are unchanged;
the canonical schema description/digest fixture is updated. Nine schema tests
pass in `/tmp/tekes-patch-contract-tests2.log`. A second local run uses this
clearer contract description with the same model and outcome assertions.

### Cache measurements and Ollama trailing usage

The original salted 60-paragraph cache experiments were ported in
`crates/provider/tests/live_cache.rs`. Cloudflare `openai/gpt-5.6-luna`
completed both experiments. Evidence:
`/tmp/tekes-cache-fanout-cf-20260904-1/receipt.json` and
`/tmp/tekes-cache-warm-cf-20260904-1/receipt.json`.
The warm request reported 3,691 cached tokens out of 3,694 input tokens.
Four parallel calls and single-flight warmup both reported substantial cache
reuse; this sample does not establish a general benefit from worker warmup.
These measurements do not establish other providers' cache behavior.

Ollama's second write run passed the file, final-answer and validator checks,
but lacked reported usage. Its streaming compatibility API requires
`stream_options.include_usage`; the Ollama renderer now requests that field,
with a versioned serializer and updated dialect proof fixture. A regression
checks that a usage-only frame after `finish_reason=stop` survives normalization
and that absent cache accounting remains absent. The third live run is pending;
the second run is not reported as a pass.

### Realtime HTTP sink gate

`http_content_sink_runs_before_server_releases_terminal_bytes` gates a real
loopback HTTP response on acknowledgement from the production provider frame
callback. Responses, Anthropic, Google Generation and Chat Completion each
must deliver content before the server sends terminal bytes; buffering the
whole response therefore fails the test. The separate production carrier test
`provider_frame_reaches_transient_sink_before_background_jsonl_confirmation`
checks immediate `session/transient` delivery and subsequent durable carrier
identity. These are complementary interface tests, not yet one end-to-end
spawned-worker/WebSocket test. The latter remains required for full chain proof.


### Spawned-worker public realtime sink proof

The former whole-chain gap is now covered by
`scripts/run-live-provider-error-queue.py --realtime`. Evidence:
`/tmp/tekes-realtime-public-sink-20260904-1/matrix.json`,
`terminal-release.json`, and `runtime/client-frames.jsonl`.
The fixture server flushes Responses text deltas, then withholds completion
bytes until an external authenticated WebSocket client acknowledges the exact
`journal-transient` assistant chunk. The client asserts the recovered turn
has not ended at acknowledgement. Only then does the server send completion;
the ordinary answer promotion and completed settlement checks subsequently
pass. This exercises actual worker binaries, provider HTTP decoding, worker
stdio, supervisor sink, and the public v3 transport. It also retains the
permanent-400/queued-input ordering checks. This is deterministic transport
proof for Responses; it is not evidence of live remote provider behavior or
whole-chain coverage of every dialect.


Ollama run 3 has now terminated: reported usage is present for its first three
attempts, confirming the request fix on the real local server. The last attempt
failed its response stream and the turn settled `interrupted/budget_wall`.
`/tmp/tekes-ollama-write-20260904-3` remains a failed workflow, not a pass.

The Ollama run 3 raw stream shows a repeated reasoning loop after it supplied
`read.offset=0`. The public schema already enforces minimum 1, and the
implementation defaults to line 1, but its model-facing description omitted
the indexing convention. The description now explicitly says one-based,
first line 1, and null defaults. The canonical descriptor digest is updated;
validation remains unchanged. Run 4 uses this corrected contract and retains
the original prompt and model.

### Append-only context and recovery regression

`appending_visible_state_preserves_context_prefix_and_recovery_bytes` verifies
Anthropic, Google Generation, OpenAI Chat, Ollama Chat and DeepSeek Responses
worker projections: appending a model-visible state adds exactly one item,
preserves every previous canonical item byte, preserves the original on-disk
ledger prefix, and reproduces identical items/admission after closing and
reopening the ledger. The compaction regression also asserts that its appended
checkpoint/compact markers preserve all original history bytes. These checks
cover the worker projection and storage seam; they do not replace adapter
native-wire replay tests or every epoch transition case.


Ollama run 4 passed the original write/read workflow on local `gemma4:12b`
with the clarified read contract. Exact file bytes, final `LIVE_OK_DONE`,
successful validator, one root turn and reported provider usage all passed.
Evidence: `/tmp/tekes-ollama-write-20260904-4/matrix.json` and the frozen test
artifacts beneath it. The earlier failed runs remain retained. Transport is
Ollama's anonymous OpenAI compatibility endpoint, not native `/api/chat`.

### Public flow matrix in progress

The six original AppServer flow cases now have preserved fixtures under
`fixtures/live/flow` and a real-worker/public-v3 runner,
`scripts/run-public-flow.py`. Text, write, shell, read and glob passed their
received event ordering, tool-result, final-answer and settlement checks.
Evidence directories: `/tmp/tekes-flow-{text,write,shell,glob}-20260904-1`
and `/tmp/tekes-flow-read-20260904-2`. Read run 1 exposed a fixture error:
the scripted server forgot prior results across Responses continuation; it
now retains its server-side conversation state.

Grep failed: the closed worker/helper environment cannot resolve bare `rg`.
`SystemToolConfig::workspace` chooses `rg`, but no bundled executable or native
fallback is wired. The documented native fallback therefore remains unproved
and the six-case matrix is incomplete. A test-only PATH override would hide
this deployment defect and is not used. The runner ports semantic events to
v3 rather than claiming the retired Swift stdio wire API is preserved.


The six-case offline public flow matrix is now passed; aggregate evidence is
`/tmp/tekes-flow-offline-matrix-20260904.json`, including retained frame hashes
and start/input/answer/end indices. Final grep run:
`/tmp/tekes-flow-grep-20260904-3`. The helper now supplies bounded native search
when the external executable is missing, with regex, local ignore rules, glob
overrides, content/files/count modes, explicit truncation and root-confined
reads. Native search regression, nine engine system-tool tests, and
`cargo check --workspace --tests --locked` pass. Native helper protocol is
specified in `spec/exec-helper-v1.md`. Remote-model flow cases remain open.


### Remote-model public flow matrix

All six original flow tasks passed the configured Cloudflare Responses route
with real workers and public v3. Independent artifact audit:
`/tmp/tekes-live-flow-matrix-20260904.json`, produced by
`scripts/audit-public-flow-matrix.py` from received WebSocket frames.
It verifies one turn, start/input/tool-success/final/end order, exact final
text, exact live-write bytes and actual successful `pwd` workspace output.
Shell run 1 completed after parameter repairs but the initial client wrongly
required first-call success; run 2 passed the corrected successful-result
anchor. All attempted calls remain in the frame evidence. The original
retired Swift stdio notification names are not claimed as current API.

The temporary Ollama 0.33.3 service started for this audit has been stopped
after its passing run; its model cache and evidence remain available.


### Semantic compaction anchors

The audit found that only explicit `anchor:true` was honored, although the
context guide promised semantic anchor groups. The worker now derives the
first admitted input, latest accepted brief and user-question result, and
latest result per task name. Retained results preserve their complete provider
tool batches to avoid orphan calls/results. Replay freezes anchor selection
at the epoch boundary. Context renderer version 2 opens a named
`renderer_change` epoch instead of reusing older continuation state.
The new selection regression covers stale replacement, sibling tool pairing,
initial input and renderer compatibility. The first worker regression run
passed 81 tests; a second run with the new test passed 81 but encountered an
unrelated socket WouldBlock in the queued-400 test, which is being rechecked.
A real-process compaction/skill continuation run is pending.


The queue socket regression passed on focused rerun. The actual-worker
compaction/skill-continuation matrix also passed:
`/tmp/tekes-kernel-anchor-compact-20260904-1`. It preserves existing history,
compacts across turns and reloads the skill after the new epoch.

The original `live_compaction_v5_shadow_artifact` remains a separate contract
gap: it requires frozen-source evidence admission, shadow artifacts/telemetry
without projection promotion, and real-model A/B private-code recall. Kernel's
`summary_artifact` currently returns validated fields and has no equivalent
shadow-admission/prefetch protocol. Passing ordinary compaction does not close
that legacy test; the inventory explicitly records this missing implementation.


### Manual compaction control delivery

`compact` was decoded but ignored by the worker. It is now parked as a delivery,
checkpointed and appended with the origin tuple before its receipt. Duplicates
reuse the same sequence across reopen without another checkpoint. Empty
history receives an empty-coverage receipt-bearing event. Any compact after
an epoch invalidates continuation compatibility. A pre-dispatch yield that
applies compaction now releases the credential lease and rebuilds the prepared
request before asking for provider admission, preventing stale-context sends.
Focused regressions cover empty/nonempty history, immutable history prefix,
reopen dedup, and discarding a stale prepared request before lease admission.
This closes the worker control seam; it does not claim a public manual-compact
route or the legacy shadow-summary admission contract.

Manual-compaction focused tests passed. The complete worker unit suite now
passes 84 tests (one live test ignored). The intermittent queued-400 fixture
failure was traced to a macOS accepted socket inheriting nonblocking mode;
the fixture now explicitly enables bounded blocking reads. This was a test
server defect, not evidence of a production provider retry failure.

The actual-worker automatic-compaction/skill continuation regression also
passed after the control-path changes:
`/tmp/tekes-kernel-manual-regression-20260904-1`. This run checks existing
automatic behavior; manual delivery has focused worker-control tests, not yet
an external public API acceptance run.


### Actual-process manual compact and next-request proof

`crates/worker/tests/shell.rs::real_worker_receipts_manual_compact_and_rebuilds_its_next_provider_epoch`
passed against a launched worker, credential broker and real HTTP fixture.
The control compact arrives between lease request and grant. Its origin-bearing
compact is receipted; the next provider request has a new compaction epoch,
no previous response ID, the initial input and prior accepted commentary.
Exactly one turn settles, and reopening the ledger confirms that the worker
released its writer lock. This extends the focused control test to a real
process/HTTP boundary; the public v3 unary registry still has no compact route.


### Compaction policy placement and public protocol boundary

Semantic-anchor selection and bounded summary planning now live in
`engine::context`; worker retains checkpoint, spill, durable append and receipt
ownership. After extraction, all four focused compaction regressions and the
actual-process manual-compact/next-request test passed.

The frozen Session Endpoint v3 unary contract has 14 operations and does not
include `session.compact`. No unversioned public compact route was retained.
The legacy live workflow invokes a named `compact` command through
`commands/run`; its separate queued-request/new-generation test is an in-memory
acceptance-helper test, not itself a live API test. Kernel already exposes the
versioned command extension, but its specified behavior submits expanded text
as durable input. The mapping from that command to an effective compaction
control request remains unimplemented and must respect the command contract;
worker-control receipt proof does not establish this client-visible behavior.


### Resource-command protocol audit

The command and Client-extension normative documents still named the retired
v2 base while their executable registries and canonical catalog already used
v3. Their base lists and collision-check references now match the current
14 unary methods plus `remote.mux`. Command expansion remains a keyed input
submission as specified. Original-source inspection distinguishes the compact
acceptance-helper unit test from the actual `commands/run` live workflow; the
inventory now records the missing queued/effective-generation semantics rather
than treating that helper as an unexecuted live network test.


### Kimi K3 terminal-choice usage and live write

The current-code rerun `/tmp/tekes-kimi-write-20260904-2` produced exact saved
bytes, the exact final answer and one successful validator, but correctly
failed the smoke's reported-usage gate. Raw Kimi K3 SSE frames carried usage
inside `choices[0].usage`; the decoder previously read only top-level usage.
The Kimi-only fallback now preserves input/output/cache counters, prefers
reported top-level usage and does not reinterpret other dialects or aggregate
alternate-choice counters. All 12 turn-finality regressions passed.

The full post-fix run `/tmp/tekes-kimi-write-20260904-3` passed. It saved
`docs/live-smoke.md` as exactly `LIVE_OK` without a newline, promoted exactly
`LIVE_OK_DONE`, and completed one root turn with one successful validator.
All three root provider attempts reported usage; later attempts preserved
cache-read values 512 and 1024. Earlier failures remain in the evidence set.
This closes the Kimi write cell, not the other provider/cache gaps in the
original parameterized write suite.


### DeepSeek Chat write and explicit cache accounting

The original smoke requires explicit cache hit and miss values in every
usage sample, including validator work. Responses-path evidence did not
supply that pair. An isolated, proved DeepSeek Chat profile on
`api.deepseek.com` with `deepseek-v4-flash` returned both fields in the real
write run. The product configuration was not modified.

Normalization previously dropped `prompt_cache_miss_tokens`. `Usage` and the
durable `usage` event now preserve it as optional `cache_miss`, only when
explicitly reported. No subtraction invents an absent value. Existing public
v3 DTOs remain unchanged. Focused provider tests cover explicit zero and
absent-field behavior; a schema regression preserves decimal-string precision
and rejects counters accompanying unavailable usage.

`/tmp/tekes-deepseek-chat-write-20260904-4` passed the complete write smoke:
exact file/final, one root turn and one successful validator. Its
`live-deepseek-chat/legacy_write/cache-audit.json` matches all five raw response
usage objects to root/validator durable records, with source hashes. The
legacy live harness now asserts the durable pair for DeepSeek Chat writes.
Earlier credential-selection/configuration mistakes and pre-fix runs remain
in separate evidence directories and were not counted as passing runs.


### Workflow cache corpus preparation

All original `cacheCorpusPrompts` scenarios are preserved under
`fixtures/live/cache-corpus` with source revision/hash and the original Swift
helper. The quota-ledger four-turn helper gate passed, including no guessed
scenario fallback and explicit successful Kernel settlement requirements.
`python3 -m unittest discover -s scripts -p test_cache_corpus.py -v` passed two
tests. This closes the original corpus unit gate only. The actual spawned
server multi-turn cache attribution measurement remains open and must consume
these inputs without weakening their brief, no-subagent, test-count or
contention-boundary instructions.


### Public multi-turn cache corpus execution

The existing real-process public flow fixture now accepts a preserved corpus
and waits for its exact number of completed turns in one session. The external
WebSocket client serializes prompt admission on `turn/end` and reads both
public usage capabilities after the final turn. The separate corpus audit
requires nonempty attribution and checks actual successful unittest reports.

The first quota-ledger run is retained at
`/tmp/tekes-public-quota-corpus-20260904-1`. Early execution exposed that the
closed shell environment cannot resolve the host's `python3.14`; a root turn
can settle without running that suite. This is an unresolved execution
environment prerequisite, not a successful corpus acceptance result.


### Toolchain authority and post-settlement input delivery

The first public quota run terminated after three successful settlements:
the fourth prompt raced an exiting worker and received Broken pipe. Its
independent audit also rejected all unproven unittest executions. The
retained directory is `/tmp/tekes-public-quota-corpus-20260904-1`.

Prompt delivery now falls back to the existing locked input author only after
confirmed child-process exit. It keeps the same origin so an input whose
receipt was lost can deduplicate. A real-child regression reconstructing a
stale live handle passed. Transport failure alone does not transfer ownership.

Workspace policy now supports explicit canonical `toolchain_roots`; worker
uses their bin directories for PATH and installations for sandbox reads. No
host environment is inherited, and changes force respawn. Profile tests cover
canonical alias deduplication, invalid PATH separators and separation from
project/write roots; workspace test compilation passed.

The full rerun with `/opt/homebrew` explicitly declared is in progress at
`/tmp/tekes-public-quota-corpus-20260904-2`; it is not yet an acceptance claim.


The second corpus run was stopped through the fixture's normal failure channel
after confirming its frozen sandbox still denied Python `realpath` traversal.
It remains failed evidence, not an accepted workflow. Declared roots now grant
metadata-only traversal of their ancestor paths on Darwin. A real Python 3.14
process imported unittest/threading inside the compiled sandbox, while reading
an undeclared sibling file remained denied. The earlier `/etc/passwd` negative
probe was unsuitable because Darwin's imported system profile permits it; the
retained successful test uses a private sibling fixture instead.

The shell schema's documented 600-second ceiling also disagreed with the
helper's 300-second limit. Client/helper now share the 600-second bound; a
real immediately exiting command verifies acceptance at 600000 ms and
rejection at 600001 ms. Eight Slice-4 gates and the fixture oracle passed.
The oracle exposed an unregistered `fixtures/live` corpus, now explicitly
listed in the manifest without excluding it from validation.

`/tmp/tekes-public-quota-corpus-20260904-3` is the current full rerun with these
fixes. No complete corpus acceptance is claimed until its independent audit.


The third quota-ledger run completed successfully. Independent audit at
`/tmp/tekes-public-quota-corpus-20260904-3/corpus-audit.json` verifies all four
completed root turns, exactly one successful unittest run per turn (7, 10, 12,
12 tests), and nonempty public cache attribution for 43 attempts. The source
ledger SHA-256 is `0ee3938ead3e3eac42101748c5f69873e7b9e7e84a1898779e21d5f0cde3ef5a`.
Earlier failed runs remain failed evidence. The frozen runner's
`provider_requests:0` counts its unused scripted server, not live API requests;
the current runner labels that counter explicitly and counts durable dispatches.

This run also exposed two recoverable tool failures. Large shell here-documents
need an explicit writable temporary directory. HelperClient now owns a mode-0700
scratch directory through all clones, and worker shell environments set TMPDIR
to that path. Last-clone drop removes it. Actual sandboxed Python plus a large
here-document and undeclared-sibling denial passed. A safepoint retry after
worker recovery was rejected because the same tool intent acquired a new wall
clock timestamp. `begin_tool_mutation` retains the persisted first timestamp
under the mutation lock, while all other identity fields remain strict. Reopen
and changed-intent regressions plus production worker safepoint tests passed.
These two fixes are newer than the successful frozen corpus binary.


Both configured CF providers now pass the original compile/run prompt:
`/tmp/tekes-turn-live-audit/cf-legacy-compile-python-3` (OpenAI) and
`/tmp/tekes-turn-live-audit/cf-legacy-compile-python-4` (Anthropic). Each retained
`compile-audit.json` pairs successful shell results with their calls and hashes
the durable ledger. All three seeded scripts produced their expected output.
OpenAI recovered from denied bytecode writes with in-memory compilation;
Anthropic compiled bytecode and ran all three scripts. These frozen binaries
include managed private scratch and the safepoint retry fix. Scratch ownership
and exact 0700 permissions also passed their focused regression.

The original Cloudflare OAuth restart gate was audited against the pinned Swift
source. Its refresh-token exchange, rotation and remote revocation require a
platform token-writing authority. Kernel's documented SecretStore is read-only;
MCP OAuth start records a pending reference and requires trusted completion.
This remains an explicit contract gap rather than a passed binding test.


Modern MCP continuation parity audit found a wire mismatch before attempting
the official SDK tunnel gate. The pinned Swift MCPModernHTTPClient sends the
original arguments with top-level requestState/inputResponses. Current Rust
continue_tool_call omits arguments and wraps the local DTO under continuation.
The existing Slice-13 fixture returns a successful tool result without checking
those fields, so its bounded-round success does not establish protocol parity.
The inventory now records this concrete repair prerequisite together with the
remaining parameter-header and subscription requirements.


MCP continuation encoding is now repaired. The API requires original arguments,
sends protocol fields at the top level, and retains the round bound locally.
Task continuation queries the server-issued task ID rather than re-executing a
tool. A strict modern stdio fixture rejects the former envelope and checks the
exact original arguments/state/input responses before returning confirmation.
The targeted test passed in `/tmp/tekes-mcp-continuation-wire.log`. This is
wire regression evidence, not official SDK public-tunnel acceptance.


The continuation repair now also has real HTTP/SSE transport evidence in
`/tmp/tekes-mcp-http-continuation.log`. A strict socket server checks Unicode
arguments, top-level continuation fields, method headers and exact task-ID
query routing; every response is SSE and the client checks returned content.
This closes the gap between the earlier stdio wire test and HTTP dispatch.
Schema-directed x-mcp-header projection remains absent: request_headers emits
standard protocol/method/name headers, but no catalog-derived parameter fields.
The pinned original compiler requires statically reachable primitive properties,
case-insensitive unique HTTP token names, and identical argument-derived values.
That implementation gap remains necessary for the official SDK gate.


Schema-directed MCP parameter headers are now compiled at catalog load and
projected from the original request arguments at HTTP send time. Scoped HTTP
requests clone the projection. The compiler rejects ambiguous names, unsupported
types and unreachable annotations; reserved configuration headers remain blocked.
Focused schema/value tests passed. The HTTP continuation test now loads an
annotated catalog and verifies a Unicode parameter header alongside its original
body. Its first run exposed that the existing HTTP test reader lowercases header
values; the assertion was corrected and an exact-case unit assertion retained.
Official SDK tunnel and subscription acceptance remain outstanding.


`parameter_headers_follow_scoped_arguments_and_catalog_replacement` passed
against a real HTTP socket server (`/tmp/tekes-mcp-scoped-header-refresh.log`).
Both scoped calls reach the server before either receives its response; each
header matches its own arguments. Reloading a catalog without the annotation
removes the prior parameter header from subsequent requests. Production peer
construction calls list_tools before publication, installing this projection.

The next subscription gap is concrete: the pinned client opens a dedicated
subscriptions/listen request with selected notifications and validates them
against the subscription identity. Kernel currently reduces capability objects
to booleans, losing listChanged details, and its stream observer exposes only
method names. Incidental list-changed notifications on ordinary calls do not
prove that dedicated subscription contract.


Subscription capability flags are now retained through parsing and serialization:
tools/prompts/resources listChanged and resources subscribe. A focused test
proves enabled round trips, absence/false without inferred enablement, and
rejection of non-boolean flags (`/tmp/tekes-mcp-subscription-capabilities.log`).
Supervisor test compilation passed after updating its explicit capability fixture.
This is a prerequisite only; dedicated subscriptions/listen and correlated
notification/acknowledgement handling still require implementation and live proof.


McpSubscriptionFilter now implements the pinned subscription correlation rules.
It requires exact numeric subscription identity and a first correlated
acknowledgement, intersects accepted notifications/resources with the original
request, and rejects unrequested changes or a change before acknowledgement.
The focused regression passed (`/tmp/tekes-mcp-subscription-filter.log`).
This reusable filter is not yet wired to the HTTP SSE transport; no live
subscription delivery is claimed from its unit test.


The subscription filter is now connected to a dedicated HTTP/SSE request via
HttpTransport::listen_subscription. Full validated notification bytes reach
the filter and accepted changes reach the sink during streaming. The socket
test sends wrong-ID and unrequested changes, then a valid change without ever
finishing the response; sink delivery triggers cancellation and the server
observes EOF. Its first run exposed the missing acknowledgement method in the
notification whitelist, now fixed. Detailed subscription callbacks bypass the
ordinary method-only queue, avoiding unbounded retained subscription events.
Evidence: `/tmp/tekes-mcp-dedicated-subscription.log`. Supervisor automatic
subscription ownership and official SDK public-tunnel acceptance remain open.


Broker registration now starts at most one catalog subscription on its long-lived
runtime. The short-lived preparation runtime does not own the subscription.
Accepted notifications increment the peer's catalog generation; terminal stream
outcomes invalidate it for the next preparation. The client-owned socket test
verifies exact generation change, duplicate-start suppression and cancellation
EOF on peer close (`/tmp/tekes-mcp-owned-subscription-test.log`). Broker-level
registration/removal integration and official SDK public-tunnel acceptance still
need direct evidence; automatic resource-URI subscriptions are not claimed.


`broker_registration_owns_subscription_until_removal` passed with a real socket
server (`/tmp/tekes-mcp-broker-subscription.log`). Registration alone starts the
subscription on the broker runtime; the handle observes exactly one accepted
change, removal clears the pooled peer, and the server observes connection EOF.
Both TEKES_LIVE_MCP_CONFORMANCE_URL and TEKES_LIVE_MCP_TASKS_URL are currently
unset. These are prerequisites for the original public SDK gates and are not
replaced by the local fixture evidence.


C# SDK task-gate audit found that the original modern client reads the task
capability from extensions[io.modelcontextprotocol/tasks]. Kernel formerly
rejected extensions and only read top-level tasks. Capability snapshots now
preserve extension objects and task operations accept either supported form,
without rewriting extension advertisement into a top-level tasks field.
Unrelated or non-object extensions do not enable tasks. The focused round-trip
test is `/tmp/tekes-mcp-task-extension.log`; supervisor compilation is checked
separately. Completed task result extraction and original hold lifecycle still
need parity proof, so the C# SDK live gate remains incomplete.


Task continuation now validates exact task identity, known status, RFC-3339
timestamps, polling metadata and required state-dependent payloads before
returning a poll result. Existing toy task replies were upgraded to actual
timestamped completed states with result objects. New negative tests reject
changed identity, missing result, unknown status, malformed timestamps, negative
poll intervals and incomplete input-required state. Durable initial holds and
final result promotion remain separate unproven integration requirements.
Evidence: `/tmp/tekes-mcp-task-state-gates.log`.


Production task-hold integration audit traced a remaining semantic mismatch.
MCP authority execute returns raw tools/call results; DynamicControlBackend in
engine/dynamic_catalog.rs maps every successful ToolControlResult.value to
BackendTerminal::Completed. This includes a server's working task state.
ToolControlStore::lookup_receipt re-acknowledges identical requests, so merely
changing Completed to an approval Hold would replay the same initial response
instead of polling the task. The existing Deferred arm is worker-owned child
spawning, not a generic external task continuation.

Required next implementation is an explicit durable async-tool continuation:
persist initial server/task identity before parking, give query/update/cancel
operations their own idempotency identities, resume without repeating the initial
side effect, and author exactly one final tool_result after terminal result
resolution. Thread/turn identity must remain unchanged while waiting. This is
an actual production gap; the successful raw HTTP task-query tests do not close
it, and existing approval semantics must not be repurposed silently.


The remaining asynchronous tool lifecycle now has an explicit implementation
requirements record: [MCP async continuation](mcp-async-continuation-design.md).
It preserves immutable initial receipts, requires separate continuation-step
identities and a versioned host-owned pending outcome, and identifies the fold,
checkpoint, worker parking, public response and crash-recovery proof needed.
The record is clearly a design for incomplete work, not an implemented spec.


Continuation request identity is now executable in worker-control: format,
host continuation reference and positive exact-integer step are validated;
request IDs are domain-separated from initial execution. Exact receipt matching
rejects action/argument changes under the same step. Focused regression evidence
is `/tmp/tekes-continuation-identity.log`. This module is not enabled by protocol
negotiation yet and does not claim durable task parking/recovery is implemented.


Continuation pending/terminal outcomes are now separate tagged variants. The
response validator checks request/call identity and exact next-step progression;
unknown terminal fields on pending state reject during deserialization. Tests
cover round-trip, wrong call/reference, skipped/repeated step and forged mixed
outcome (`/tmp/tekes-continuation-outcomes.log`). Negotiation, durable state and
worker resume remain incomplete and this module is still not activated.


ContinuationJournal persists immutable bindings, step intents and receipts with
exclusive locking and atomic synced publication. Reopen regression distinguishes
an unresolved intent from completed replay, rejects changed actions/authority,
requires predecessor pending state and rejects further steps after terminal
completion. Initial binding bytes remain unchanged. Evidence:
`/tmp/tekes-continuation-journal-reopen.log`. The journal remains unactivated
until worker pending-state and negotiated control integration are complete.


Journal resolution now holds a per-continuation NamedLock across remote callback
and receipt commit; raw prepare/commit are private. A new restart regression
simulates a lost response after remote execution, verifies reconciliation-only
recovery, and then verifies receipt replay calls neither callback. The original
operation count stays one. Both journal tests passed in
`/tmp/tekes-continuation-journal-resolve.log`. This proves journal callback
selection, not yet the production MCP task/worker wiring.


McpBrokerHandle now routes task operations to the already bound peer rather
than resending tools/call. A real child fixture checks get/update/cancel task
identity and update responses, rejects tools/call, and verifies removed peers
cannot receive task operations (`/tmp/tekes-mcp-broker-task-routing.log`).
Journal-to-MCP authority binding and cooperative task cancellation remain open.


Task operations now carry cancellation from broker to client. Real HTTP tests
cover get/update/cancel: pre-cancel sends no request, cancellation after confirmed
dispatch closes the socket, reads return Cancelled and mutations return
UnknownEffect. The broker evicts the closed generation. Evidence:
`/tmp/tekes-mcp-task-cancel-http.log`. This closes the cooperative task-operation
transport prerequisite, not the remaining journal/worker integration.


MCP task journal callbacks are connected through resolve_task. The bridge checks
pool authority plus initial task identity, executes broker operations, validates
remote state and extracts completed tool content. Uncertain mutation recovery
queries only and cannot confirm a still-pending result. A real Python stdio
server test covers query/pending, update/final, immutable terminal replay after
reopen with no additional wire request, and authorization mismatch rejection.
Evidence: `/tmp/tekes-mcp-continuation-bridge-test.log`. This remains below the
worker-control activation/park-resume layer and does not close the SDK live gate.


Event/fold checkpoint review identified another activation requirement for
asynchronous tools. Generic state subkinds exist, but LedgerValidator's durable
checkpoint currently carries only the existing tool call/result state, not
continuation steps. Adding a pending event without corresponding checkpoint
export/import would lose wait semantics after accelerated replay. The design
record now requires paired full/checkpoint replay tests and writer compatibility
before emission. No new event semantics were silently assigned to meta or state.


The continuation journal now also has a four-process contention regression.
All participants reach the same execution lock before release. Only one remote
cancel callback executes; every participant returns bytes identical to the
immutable receipt. Evidence: `/tmp/tekes-continuation-process-contention.log`.
This proves journal serialization across independent processes, not worker
parking, checkpoint recovery or the original official SDK live acceptance.


D-49 writer enforcement audit found that LockedLedger could repair a gated
ledger's tail and append despite the schema read-only projection. Its open path
now checks framed genesis, upgrade meta and per-event reader minima before
creating assets, replaying checkpoints or truncating bytes. Append rejects
unsupported minima before changing the fold or file. The format writer version
is implementation-owned (currently 1); choosing reader version 2 cannot enable
writer version 2. Supervisor diagnostics identify this as version-gate rather
than corrupt-ledger. This is the existing format boundary, not activation of a
new continuation event or worker-control version.

Verification: `/tmp/tekes-store-version-gates.log` passes all 8 store tests,
including three version-gate regressions; `/tmp/tekes-version-gate-checkpoint-regression.log`
passes all 5 checkpoint conformance gates; `/tmp/tekes-version-gate-integration-check.log`
passes worker and supervisor compilation. Rejected opens preserve the entire
original file, including an unrecognized suffix, without creating an asset
folder. Rejected appends preserve next_seq and bytes; supported writes and
checkpoint-seeded reopen still succeed.


D-49 follow-through now covers creation and rewrite paths that bypass
LockedLedger. ThreadStore rejects unsupported genesis versions before staging;
deduplicated and staged creation also check the existing target before publishing
retry assets. Rewrite rejects gated source ledgers and rebuilt payloads. Redact
checks source identity/version before moving the active source to its tombstone,
in addition to its existing post-move validation.

`/tmp/tekes-store-create-rewrite-version-gates.log`: all 9 store tests pass.
The new regression covers reader-only and writer-only gates, rejects initial and
replayed creation, fork and redact, and verifies unchanged source bytes, no retry
asset publication and no destination/staging/trash publication.
`/tmp/tekes-rewrite-version-gate-regression.log`: all 3 slice-5 gates pass,
including redaction closure, rewrite publication crash matrix and staging GC.
These checks do not close asynchronous continuation or any external live gate.


D-49 process boundary is now exercised with a real profiled worker and a bound
loopback provider socket. Both reader-only and writer-only version gates exit
with EX_PROTOCOL (76), emit only hello (no lease or appended notification), leave
the original ledger plus unrecognized tail byte-identical, and make no provider
connection. The worker classifies StoreError::VersionGate as protocol failure
instead of generic exit 1. Evidence: `/tmp/tekes-worker-version-gate-process.log`.
The ordinary configured provider-loop test passes as a positive control:
`/tmp/tekes-worker-version-gate-positive-control.log`. This is local process and
interface evidence, not an external vendor live acceptance or async task recovery.


The continuation bridge now distinguishes task completion from embedded tool
success: completed + isError=true becomes a non-retryable Failed receipt, with
the remote tool-result details retained in the error message. An explicitly
present isError must be boolean; malformed flags cannot become successful tool
results. Absent/false flags retain the successful result path. The regression
also reopens the journal and proves failure receipt replay bypasses both remote
execution and reconciliation. Evidence: `/tmp/tekes-mcp-task-error-receipt.log`
(two bridge tests, including the real stdio query/update test). This remains in
the not-yet-activated continuation module; production worker park/resume and
ordinary synchronous MCP error mapping are not proved by this test.


Ordinary production MCP execution now checks the remote tool result's isError
flag before returning a successful dynamic-authority value. True becomes a
ToolFailed operation error; its v2 control mapping is unavailable with
retryable=false, preserving remote result details. This is distinct from
retryable transport unavailability. Explicit non-boolean flags are protocol
errors. The normal successful result remains unchanged.

`/tmp/tekes-mcp-production-tool-error.log`: the real stdio loader/authority
success and tool-error tests pass; the public Cloudflare HTTPS test is explicitly
ignored in this run. `/tmp/tekes-mcp-tool-error-control.log`: definitive failure
versus transient-unavailable retry semantics pass. This closes synchronous
isError mapping, not asynchronous pending-task publication or worker recovery.


Post-classification external regression: the production HTTPS Cloudflare docs
loader test passes on the current code (`/tmp/tekes-mcp-production-classification-live.log`,
one actual test, zero ignored). This exercises discovery, projected authority
and a nonempty successful remote response. The matching loader inventory entry
now includes this evidence and the synchronous tool-error tests. Official SDK
inventory descriptions were rewritten to describe the current implemented
prerequisites and remaining worker/public-tunnel gaps without implying completion.


Chat streaming finality now closes content at finish_reason while allowing
subsequent usage-only chunks until the stream terminator. Previously the Chat
accumulator could append late text, reasoning or tool arguments after a finish
reason. Non-null delta fields after that boundary are now rejected as malformed;
a valid final delta and finish_reason in the same choice remain accepted.
`/tmp/tekes-provider-content-finality.log` passes all 13 finality tests, including
late text/reasoning/tool-delta rejection, trailing usage, Ollama usage tails and
Kimi choice-local usage. This change does not equate provider finish with turn
settlement; worker candidate validation and settlement remain separate.


The causally gated HTTP content-sink test now includes Google Interactions,
using its interaction.created / step.start / step.delta prefix before the server
releases remaining steps and interaction.completed. All five adapter families
(Responses, Anthropic, Chat Completion, Google Generation, Google Interactions)
now prove content delivery before terminal bytes are sent. Evidence:
`/tmp/tekes-provider-five-family-sink.log` (one named test iterating five actual
socket exchanges). This is family-level HTTP/decoder/callback timing evidence;
it does not establish every vendor dialect's fields or external live acceptance.


Workspace regression audit found and corrected two stale test contracts.
The periodic sweep subscriber fixture used session ...0111 while its copied
genesis still named ...0003. The fixture now binds its genesis to SESSION;
production actionable ancestry checks remain unchanged. The focused recovery
and live subscriber event-order test passes
(`/tmp/tekes-periodic-sweep-identity-regression.log`).

The hermetic reference MCP package tests previously unwrapped successful values
for TCC denial and forbidden prompts. They now require ToolFailed with the exact
reason, retaining all successful discovery, grant and restart assertions.
`/tmp/tekes-reference-package-tool-failure.log`: four tests pass, the explicitly
signed-app preflight remains ignored. Workspace runs 1 and 2 failed on these
stale assertions and are retained as failure evidence; neither is an all-green
workspace claim. A full no-fail-fast run follows the fixes.


Workspace regression run 3 completed with exit 0:
`cargo test --workspace --locked --no-fail-fast`.
Evidence: `/tmp/tekes-workspace-alignment-regression-3.log`.
Cargo metadata confirms the expected 21 workspace crates. All executed default
test targets and doc-tests passed after the fixture corrections above. Explicitly
ignored live, provisioned-app and configured-toolchain tests remain ignored;
this result does not close the 99-entry original live-test inventory or prove
unfinished asynchronous continuation integration. The earlier workspace failures
remain available in runs 1 and 2 rather than being relabeled successful.


The ignored real-worker final-before-settlement recovery test was explicitly
executed against freshly built worker/helper binaries and passed (zero ignored).
Its five boundaries are output, candidate, decision, validator death, and queued
input after validator death. Added a byte-prefix assertion at every boundary:
recovery may only append to the preexisting log. Existing assertions retain one
turn-1 settle, exact promoted output, nonduplicated validation records, actual
validator run/error settlement, and queued turn admission after the first settle.
Evidence: `/tmp/tekes-real-recovery-append-only.log`; binary hashes and boundary
list: `/tmp/tekes-real-recovery-append-only-receipt.json`.
This proves the final-candidate recovery process path with resume=never; it does
not prove MCP pending-task recovery or external provider acceptance.


AgentEval exact-task port now runs the two pinned prompts/tasks through the
public process runner. Inputs live in scripts/agent_eval.py; --agent-eval selects
the task and scripts/audit-agent-eval.py separates harness health from capability.
Memory run 1 passes the second-turn BLUEHERON final-answer anchor. Valid subagent
run 3 passes tool execution but fails accepted child-report integration. Both
public harnesses complete. Evidence is listed in the updated inventory.

Subagent runs 1 and 2 are excluded as invalid setup: missing subagent exposure,
then accidental root report-role selection. Run 3 exposes subagent in the ordinary
root role. Production child_report_summary currently reads only explicit report
tool-call args, not the settled promoted child answer; the observed children end
with final output and no report, leaving parent child_result without summary.
The parent subsequently rereads NOTES.md. This is not proof of successful report
integration. The scored suite has been audited (per-task, default 3 trials,
model-specific baseline, no silent blessing) but is not yet executed or closed.

### Delegated report role and approval-resume gap

The worker now derives the subagent role from matching durable child genesis and
parent spawn records, exposes report to that role, and includes the report-back
instruction in the frozen system context. Ordinary roots cannot select report to
impersonate a child. Child summaries require a successful paired report result in
the latest turn. Worker regression: 84 passed, 1 ignored
(`/tmp/tekes-subagent-report-accepted-regression.log`).

Real trial `/tmp/tekes-agent-eval-subagent-20260904-4` failed its public harness
with a client timeout. The child issued report and received a durable granted
approval response, but did not resume to append a tool result or settle. The
parent remained at spawn. This trial does not prove accepted report delivery.
The locked-answer path in LiveRespondAuthority::ensure_after_locked_append uses
ordinary schedule_line capacity admission, whereas initial dependency launch
uses schedule_child_from_parent. A waiting parent can therefore block child
resumption at the worker limit. This path requires a durably validated dependency
resume fix and a real-process regression; the lifecycle audit remains open.

The approval-resume starvation is now fixed: schedule_line validates matching
child/parent identities and the still-unresolved spawn before using dependency
admission. The max_workers=1 process test now exercises the locked-answer resume
path, rejects a forged spawn binding, and keeps unrelated work queued.
`/tmp/tekes-child-resume-capacity-3.log`: 1 passed, 0 ignored.

Real run `/tmp/tekes-agent-eval-subagent-20260904-5` passes both the public harness
and the accepted-report capability anchor. Parent SHA-256:
278a278c80f1641995a00259f6775c539705fefe8794a3f1a57bb3661d44a667;
child SHA-256: 3ee7c08ff6205a4ba118fd7084919c0b3950eee5262b5137f40e4a25d09a068d.
The two-task AgentEval smoke inventory item is now passed; scored trials and the
remaining inventory remain separate open requirements.

Post-fix supervisor library regression passed: 95 passed, 11 ignored
(`/tmp/tekes-child-resume-supervisor-regression.log`). Inventory verification
found all 99 tests across 18 suites, with no missing registrations. Ignored
live tests and the scored suite are not covered by this regression result.

### AgentEval scored suite port

`scripts/run-agent-eval-scored.py` ports the pinned sequential catalog loop with
three trials per task, per-task and per-dimension rates, separate harness error
counts, JSONL task/trial reports, and model-specific baseline lookup. Missing
baseline coverage fails; new tasks stay ungated; all-harness-error runs fail;
missing model baselines produce bootstrap status and never write a blessed file.
Four focused scorer tests passed (`python3 scripts/test_agent_eval_scored.py`).
Timing includes the Kernel process harness build/freeze overhead and is explicitly
not comparable to the old in-process driver latency. The first six-trial live run
is `/tmp/tekes-agent-eval-scored-20260904-1`; completion remains unverified until
its process exits and the final gate/report files are inspected.

The public AgentEval auditor also has focused synthetic receipt tests: accepted
report and parent join are independently required, and a failed public harness
remains a harness error. `python3 scripts/test_agent_eval_audit.py`: 2 passed
(including the three accepted/joined combinations).

The six-trial run exited 0: memory 3/3, subagent 3/3, zero harness errors.
All six manifests contain identical frozen binary hashes. `gate.json` records
bootstrap for openai/gpt-5.6-luna because that model has no blessed baseline;
this completes the original suite's bootstrap branch, not a baseline regression
claim. Task and trial JSONL are retained in the run directory. No baseline was
created or modified. The inventory now records passed_live_bootstrap.

### Responses argument stream consistency

The eager-parallel legacy test requires two distinct strict tools and per-call
completion records before the response manifest. That complete gate remains open.
While tracing it, the Responses adapter was found to overwrite streamed arguments
at arguments.done without comparison. It now rejects changed call identity/name,
completion arguments that differ from nonempty streamed content, and deltas after
arguments.done. Completion-only arguments remain supported. Focused finality
suite: 14 passed (`/tmp/tekes-response-arguments-finality.log`). This is consistency
validation, not proof of eager execution or of the full legacy eager test.

Provider regression exited 0: 68 passed, 16 ignored
(`/tmp/tekes-provider-arguments-regression.log`). A fresh real Cloudflare Responses
parent/child report flow passed on the updated adapter:
`/tmp/tekes-provider-arguments-live-20260904-1/agent-eval-verdict.json`.
Main SHA-256: ddfae02364d75f8c09ef830b35b1041f28840e887b7f0256f6dd8356a45961da.
Generated architecture and links checked successfully. Other adapter live gates
and the eager-parallel manifest contract are not inferred from this result.

### Completion-only tool arguments reach the live sink

Responses arguments-done previously updated only accumulated state when the API
omitted argument deltas. The adapter now emits the missing argument bytes through
the existing ToolDelta sink at that event, without replaying already streamed
arguments. The socket test includes two distinct calls (record_left and
record_right): its server withholds response completion until both tool frames
have been acknowledged by the sink. It passed with all five adapter-family
content cases (`/tmp/tekes-completion-only-tool-sink.log`). The finality suite
passed 14 tests (`/tmp/tekes-completion-only-tool-finality.log`). This closes an
actual presentation delay but does not introduce an eager execution flag or
complete the old eager manifest/fan-in contract.

Public process/client read flow also passed on rebuilt binaries:
`/tmp/tekes-completion-only-public-read-20260904-1`. The independent WebSocket
receipt contains exactly one tool-call-delta each for flow-brief and flow-tool;
the latter decodes to the expected docs/seed.md read arguments. These scripted
responses supply arguments only in arguments.done. The receipt audit is retained
as tool-delta-audit.json. No actual external provider acceptance is claimed by
this scripted public-flow result.

### Per-call argument completion signal

Responses now emits ToolCallReady once per arguments-done call, after validating
identity and JSON. The worker sends a tool frame with empty delta and optional
arguments_complete; endpoint projection preserves it as argumentsComplete and
adds assistantFrameId for response grouping. This is presentation-only: provider
terminal validation, tool-result ownership, and turn settlement remain required.
Legacy frames omit the optional field; non-tool markers are rejected.

The strengthened socket test waits for two ToolCallReady callbacks before
releasing terminal bytes. The public scripted read test uses --tool-ready-gate
and withholds each tool response terminal until the external WebSocket client
logs that exact call's argumentsComplete marker. Run
`/tmp/tekes-tool-ready-public-20260904-2` passed. The prior ungated fast-response
run completed but did not retain completion markers after semantic output won
the projection race, so it is not used as realtime completion evidence.

Interface regressions passed: provider + worker-control + endpoint, 132 passed,
17 ignored (`/tmp/tekes-tool-ready-interface-regression-3.log`). The first run
caught the missing fixture manifest entry, now registered. Completion projection
has live/durable identity and retry-dedup assertions; old control frames and
non-tool marker rejection are tested.

Actual CF openai/gpt-5.6-luna parallel test passed with low reasoning, required
choice, two distinct strict schemas, no sampling parameters, and two ready
callbacks exactly matching the terminal calls (A/B). Evidence:
`/tmp/tekes-parallel-ready-live-20260904-1/receipt.json`; retained request digest
95fad41240de8f4836cae51255e950adecf7a9f09fa7269d89d792ffebed4104.
The original eager manifest/fan-in contract and a combined real-provider/public
client test are still not claimed complete by these separate tests.

### Response manifest preserves completed sink calls

Previously the response manifest could diverge from a completed streamed call,
leaving normalized tool execution and sealed replay with different facts.
Responses now checks completed calls against output_item.done and any explicit
terminal output array. Changed identity/name/argument values and dropped calls
fail normalization; semantically equal JSON with different whitespace remains
valid. An absent output array still supports the existing reconstruction path.
The finality test exercises six variants at both item and response boundaries.

Provider regression passed on the manifest checks
(`/tmp/tekes-tool-manifest-regression.log`), including 15 finality tests. Fresh
real CF parallel invocation also passed:
`/tmp/tekes-parallel-manifest-live-20260904-1/receipt.json`, with both ready calls
exactly equal to final manifest calls. This verifies the adapter boundary; the
combined real-provider public-client eager workflow is still open.

### Combined live parallel public-flow test

The public worker/process harness now supports --parallel-tools with two
read-only dynamic helpers, record_left/right, exact A/B input, low session
reasoning, and a fixed final answer. The public audit requires two calls in one
attempt, completion markers associated with the same attempt before durable
calls, one successful result each, and next model attempt after both results.

Run `/tmp/tekes-parallel-public-live-20260904-1` used an invalid helper catalog:
it incorrectly included strict in the dynamic schema's closed three-field
format. The helper declaration is corrected; provider request normalization
already derives strict from the required-only closed object schema. Production
schema validation was not weakened. Corrected run 2 is in progress; neither
run is considered passing until the public audit is inspected.

Run 1 terminated with startup failure from the invalid strict catalog field.
Run 2 reached real A/B tool execution and final settlement but failed the
client's prompt assertion: searching serialized JSON cannot match raw quotes.
The assertion now compares the decoded user/message text. Run 3 froze an
intermediate incorrect message nesting in that correction and is also excluded.
Run 4 uses the corrected data.content path and runs the public fan-in auditor
automatically. Failed harness runs are not counted as passing live evidence.

Run 4 completed tools/turn but failed the completion-marker audit. It exposed a
production race: publish_frame refreshed its cache from the full newer disk
ledger, publishing output while earlier tool frames remained on the pipe.
Cache refresh now stops before that attempt's response records. It does not
reopen already published semantic output. A fixed-timing regression writes the
full settled ledger before delivering the earlier frame and asserts stream seq
precedes output seq (`/tmp/tekes-frame-prefix-causal.log`: 1 passed). Supervisor
library regression passed 95 tests, 11 ignored before the additional cache
attempt-key condition; the causal test and live run include that condition.

Corrected run 5 passes the combined real CF/public-client audit:
`/tmp/tekes-parallel-public-live-20260904-5/parallel-public-audit.json`.
Two calls share daemon-87873-1-attempt-7; completion markers precede durable tool
calls, results are seq 13/14, next model attempt seq 15, and there is one completed
settlement. Legacy eagerDispatch/triggerCompile manifest fields still need an
explicit v3 semantic mapping; the inventory retains that remaining requirement.

### Eager contract interpretation and final frame-prefix regression

The final supervisor source, including the cache attempt-key condition, passes
95 library tests with 11 ignored
(`/tmp/tekes-frame-prefix-final-regression.log`). This replaces the earlier
regression qualification; the fixed-time race test and live run remain evidence.

Further pinned-source inspection corrects the remaining eager gap description.
TekesRuntime Orchestration.Digester.swift lines 645-656 explicitly treat the
manifest as proof that valid calls were already dispatched mid-stream, skip
redispatch, and schedule the shared tool_join. Lines 2492-2525 suppress each
individual result's compile and leave one shared join to continue the frame.
Thus the remaining gap is not simply absent metadata. Current Kernel early
ToolCallReady is presentation only; execution occurs after response validation.
The live combined test proves early presentation, no duplicate results, and
post-result continuation, but not pre-terminal execution. Inventory wording now
states the exact behavior/recovery gap, without claiming semantic equivalence.

### Adopted response preserves already-durable calls

The recovery audit for early dispatch found a current gap: complete_adopt checked
only the queried response's call-ID inventory. It could then skip an existing
call ID even if the adopted tool name/arguments differed from the durable call.
Before appending reasoning, usage, or output, adoption now validates all existing
calls owned by the attempt against the returned response, materializing spilled
arguments from verified assets. Omitted calls and changed names/values fail.
Other attempts are excluded from this comparison. No early execution is enabled.

Worker bin regression: 85 passed, 1 ignored
(`/tmp/tekes-adopt-call-identity-regression.log`). The focused test was then
strengthened to force a real spilled asset and passed
(`/tmp/tekes-adopt-spilled-identity.log`). It checks mismatch failures do not
modify ledger bytes. Existing actual-process asset-only adoption still passes
(`/tmp/tekes-adopt-process-regression.log`: 1 passed), retaining recovery behavior
for responses with no previously appended calls.

### Eager dispatch: pre-terminal execution with durable fan-in (2026-09-05)

The remaining eager gap was a behavior gap: `ToolCallReady` reached the public
sink early, but every call executed only after response validation. The worker
now dispatches a call at its arguments-complete frame, while the response is
still streaming on its own thread:

- `forward_provider_frame` returns the validated call; `eager_dispatch_ready_call`
  rejects an empty id, a duplicate within the response, or any id already
  durable in the ledger (cross-attempt or replayed), then appends the same
  write-ahead `tool_call` the post-terminal path appends (`append_provider_tool_call`,
  spill + side-effect barrier), rings the doorbell, and runs the single call
  through `execute_provider_tool_calls` — policy, hooks, approvals, sandbox,
  safepoints and cancellation unchanged. A parked or stopped dispatch marks
  the response `suspended`: no further eager execution, no request cancellation.
- `validate_terminal_tool_calls` now takes the attempt and the eager set. A
  terminal call may reuse a durable id only when this attempt dispatched it
  eagerly and the name and materialized (spilled) arguments are equal; an
  omitted eager call, a changed fact, a duplicate id, or any other durable id
  fails the terminal. `append_terminal` skips the eager calls; the post-terminal
  batch executes only the remainder, or returns immediately when suspended,
  leaving the whole batch durable for the resumed run. One continuation
  follows after all results and the terminal.
- Recovery reuses the existing machinery: an interruption between the intent
  and its result is `recover_unpaired_tool_calls` (replay-safe re-execution or
  `aborted_by_crash`, exactly once); an interruption before the terminal closes
  the attempt through `reconcile_provider_attempt`/adopt, with
  `validate_adopted_calls` already comparing the queried response against the
  durable eager calls.
- Supervisor: eager records and their doorbell would have tripped
  `publish_frame`'s sealed-output guard for every later frame of the same
  attempt (the refresh stopped before the attempt's first `tool_call`, saw
  newer journaled seqs, and returned without publishing or caching). Worker
  frames now carry `ledger_seq` (worker-control-v1, optional); the refresh cuts
  at `max(ledger_seq, journaled_through)`, so later frames publish and a late
  frame after a published output is still rejected by the projector.

Tests (worker bin 89 passed, 1 ignored; supervisor lib 97 passed, 11 ignored;
worker process suite 15 passed; worker-control + endpoint all passed;
`/tmp/tekes-eager-dispatch/*.log`):

- `eager_dispatch_executes_ready_call_before_response_terminal_bytes`: the
  scripted Responses SSE server withholds `response.completed` until the
  `tool_result` line for the ready call is durable in `main.jsonl`; asserts one
  call record, one result, call < result < usage < output, one continuation
  attempt after the terminal, a ready frame stamped below the call seq and a
  doorbell after it.
- `eager_dispatch_park_suspends_execution_and_resume_pairs_the_whole_batch`:
  an eager `ask_user_questions` parks; the response terminal still lands with
  the second call appended once and unexecuted; the answered resume pairs both
  exactly once.
- `response_terminal_reuse_of_eager_call_requires_identical_facts`: spilled
  eager arguments are materialized for comparison; changed args/name, omitted
  eager call, cross-attempt id, duplicate id, blank id, and a durable id outside
  the eager set all fail.
- `eager_call_interrupted_before_result_is_recovered_once_and_attempt_closes`:
  unpaired eager intent recovers once, then the attempt closes with
  `attempt_recovery`/`usage`/`error`/`settle interrupted` and no second result.
- Supervisor `eager_tool_records_published_by_doorbell_do_not_drop_later_frames`
  and `late_stamped_frame_does_not_reopen_sealed_output`.

`scripts/audit-parallel-public.py` now additionally requires every eager call
and result to precede the attempt's first `usage`/`output` record and exactly
two call records for the attempt. Re-run against the pre-change run 5 evidence
it fails on the result-before-terminal assertion, so it distinguishes early
presentation from early execution.

Real CF combined run with the new worker/supervisor binaries passed:
`/tmp/tekes-parallel-public-live-20260905-1/parallel-public-audit.json`
(openai/gpt-5.6-luna, low reasoning, required choice, two strict dynamic
helpers). Ledger order for `daemon-93515-1-attempt-7`: tool_call 9, tool_result 10,
tool_call 11, tool_result 12, usage 13, output 14, next attempt 15, one completed
settlement; both public completion markers precede the durable tool/call events
and each call has exactly one successful public result. Two dispatched attempts,
zero scripted requests (`matrix.json`). The inventory entry for
`live_openai_cloudflare_eager_parallel_tools` moves to `passed_cloudflare` with
this evidence. Not covered by this run: adapters other than Responses (no
`ToolCallReady` producer yet) and a process-level kill between the eager intent
and its result (unit coverage only).

### Readiness on every streaming adapter (2026-09-05, evening)

`StreamAccumulator::mark_call_ready` announces a call once through the same
argument resolver the terminal uses: Anthropic at `content_block_stop`
(argument deltas after the stop are rejected), Chat Completions at
`finish_reason` (no per-call event exists), Google generation on the whole
`functionCall` part, Interactions at `step.stop`. Provider suite: 33 lib +
finality 17 passed (`/tmp/tekes-eager-producers-provider.log`); worker bin 93
passed, 1 ignored (`/tmp/tekes-eager-producers-worker.log`) including
`eager_dispatch_executes_anthropic_call_before_message_stop_bytes`, which
withholds `message_stop` until the tool result is durable. Live parity on the
non-OpenAI routes is not claimed by this change.

### Process-level eager kill recovery (2026-09-05, evening)

`crates/worker/tests/shell.rs::eager_dispatch_survives_worker_kill_between_intent_and_result`:
a real worker streams a Responses `context_get` call, appends the write-ahead
`tool_call` and sends the `tool_control` request; the test (acting as the
supervisor) kills it there, with no result and no output durable, and withholds
the response terminal. The recovery run (genesis `resume: {bounded: 2}`) pairs
the call exactly once through stable-receipt recovery, closes the interrupted
attempt with a recovery epoch before the re-send, preserves every byte written
before the kill, and completes the turn. A first variant with `resume: never`
showed the reconcile behavior instead: the call still pairs once, then the
attempt closes `unresolved_dispatch` and the turn settles `interrupted`. Worker
process suite: 16 passed (`/tmp/tekes-eager-kill-shell.log`).

### MCP asynchronous task continuation activated (2026-09-05)

The continuation building blocks (request identity, journal, bridge, broker
task operations) are now connected to worker-control and the worker; see the
"Activation" section of `mcp-async-continuation-design.md` for the exact
surfaces. Contract changes: `tool_control_result.pending` (closed third arm),
`tool_continuation`/`tool_continuation_result` messages, event-v1 `state`
subkind `tool_continuation`, mcp-runtime-v1 "Task results bind a
continuation". A task-shaped `tools/call` result no longer becomes a
successful value.

Tests: worker bin 91 passed, 1 ignored; supervisor lib 98 passed, 11 ignored;
supervisor process suite 9 passed; tools/engine/worker-control/mcp all passed
(`/tmp/tekes-mcp-continuation/*.log`). The worker test scripts the control
channel exactly (initial pending result, two pending polls, hold, answered
resume as one update, completed receipt) and the stop test asserts the cancel
step and the error result. The supervisor test uses an in-process task-capable
peer, not the stdio fixture server, and no real worker process crosses the
supervisor reader route in a test. The inventory's two MCP task/SDK entries
stay `not_run`: the official public-tunnel URLs are still absent.

### Native deferred tools: worker automatic routing (2026-09-05)

Capability per exact route (`model-capabilities.canonical.json`
`native_deferred_tools`, digest re-pinned), `ResolvedDialectProfile::native_deferred_tools()`,
worker native catalog/reference/stateless-context/prepare routing, production
`TEKES_KERNEL_LIVE_ARTIFACT` wire capture forwarded by the supervisor, and
`apply_openai` replaying unbound searches as empty `tool_search_output`. Full
detail and live evidence in `native-deferred-tools-gap.md` ("Worker automatic
native routing"). Tests: provider `native_deferred_routing` (7 route cases),
worker `native_route_declares_client_tool_search_and_replays_bound_output`,
provider lib 33 passed; worker bin 92 passed, 1 ignored; supervisor lib 98
passed, 11 ignored; supervisor process suite 9 passed
(`/tmp/tekes-native-worker/*.log`). Live: OpenAI CF route passes the real
worker/public-client two-turn scenario with the native audit
(`/tmp/tekes-native-worker-openai-20260905-2/native-worker-audit.json`);
Anthropic CF route is wire-accepted through the `tool_reference` replay but
the model refuses the scenario prompt (runs 1–3), so it is not counted as a
completed worker proof.

### Anthropic native worker turn and the eager replay-ordering fix (2026-09-05, evening)

The neutral shipping scenario completes one full native Anthropic worker turn
(see the gap audit's Anthropic follow-up); its first run found that eager
dispatch had made stateless replay render a call's result before the sealed
assistant call. `project_provider_context_mode` now keys a result that
precedes its attempt's output at that output's position (after it). Worker bin
94 passed, 1 ignored (`/tmp/tekes-native-worker/worker-bin-2.log`).

### MCP task augmentation for task-required tools (2026-09-05, evening)

See the design doc's "Task augmentation" paragraph. MCP suite 23+9+16 passed
(`/tmp/tekes-mcp-task-aug.log`), supervisor lib 98 passed, 11 ignored
(`/tmp/tekes-mcp-task-aug-supervisor.log`).

### MCP continuation park-and-release (2026-09-05, evening)

See the design doc's "Park-and-release" paragraph and the tail-lifecycle
`continuation_wait_until` predicate. Worker bin 95 passed, 1 ignored
(`/tmp/tekes-park-worker.log`); supervisor lib 98 passed, 11 ignored
(`/tmp/tekes-park-supervisor.log`); schema/engine/tools/conformance suites
passed after the fold and lifecycle changes.

### Real-process MCP task continuation and the spawned-CLI family (2026-09-05, night)

`real_public_mcp_task_continuation` (`crates/supervisor/src/process_live_tests.rs`,
ignored, driven by `scripts/run-live-process.py mcp-task`) runs the actual
supervisor, worker and `mcp-fixture-server --scenario task-augmented` over the
public v3 carrier. Three defects surfaced only at process level and are fixed:

1. Allowed MCP tools were validated against the effective catalog before the
   MCP catalog was merged, so `mcp__fixture__echo` was rejected as absent. The
   launch closure now prepares MCP first and passes its tools into
   `resolve_worker_launch_bindings` (`crates/supervisor/src/dynamic_bindings.rs`).
2. The production secret policy withheld every `pending` tool-control result
   because it scanned only terminal values. `ProductionToolControlPolicy::apply`
   and `apply_continuation` now scan the pending state and continuation
   responses (`crates/supervisor/src/production_tool_control.rs`).
3. Each worker run restarted the fixture because the cached binding kept only a
   `Weak` authority; the park-and-release loop therefore never observed the
   completed task (run 3, eleven parks at `/tmp/tekes-mcp-task-live-20260905-3`).
   `CachedBinding.routes` keeps the routes alive and rebinds when the authority
   is dead (`crates/supervisor/src/mcp_runtime.rs`).

Run 4 (`/tmp/tekes-mcp-task-live-20260905-4/runtime`): `tool_call` → `state`
bind/query/park → run 1 exits with `continuation_wait_until` → run 2 resumes
and appends one `tool_result ok` ("augmented done") → `settle completed`.
Supervisor lib 100 passed, 12 ignored.

The spawned-CLI family (legacy endpoint-v1 stdio routes, catalog-only launch,
stubbed-runtime cases 4/5/6, login-keychain-home, Cloudflare fact events) is
ported onto the scripted and live public-flow runners: `flow-websocket-client.py`
probes `session.models` / `session.selectModel` and the empty pre-turn journal
snapshot before submitting the prompt (receipt `route_probes`), the runner
asserts the bound credential reaches the provider and never the client frames,
and `--no-route-probes` proves catalog-default resolution. Six scripted cases
(`/tmp/tekes-spawned-endpoint-scripted-20260905-3-*`), the catalog-only run and
the live Cloudflare run (`/tmp/tekes-cf-fact-events-20260905-1`) passed. The
legacy login-keychain preservation is an inverse contract in Kernel: the worker
environment is cleared and credentials cross the broker descriptor. Inventory
`not_run` 17 → 10.

### Durable provider admission wait and the compact transition gate (2026-09-05, night)

Legacy `wholesale_queue_is_durable_visible_and_finishes_original_drawing_turn_without_retry`
is ported as a durable admission wait rather than a SQLite queue:
`provider::wholesale_limited` classifies the Cloudflare AI Gateway wholesale
402 as `RateLimited`; the worker's `wait_provider_admission` appends
`state{subkind: provider_admission}` (`next_attempt_at`, `wait_ms`,
`retries_left`, `declared_retry_after`) and sleeps until the instant, cut short
by stop or supervisor loss; the fold exposes `admission_wait_until` and
`LifecycleFacts::durable_wait_until`, which `run_decision` and
`ensure_action_at` evaluate exactly like a parked continuation. Scripted proof
`scripts/run-public-flow.py --case write --wholesale`
(`/tmp/tekes-wholesale-20260905-2`): two 402 responses, the worker SIGKILLed
during the first wait (pid recorded in `matrix.json`), the sweep started an
ordinary run at the due instant, the second wait ran in-process, and the turn
finished through the validator with exact bytes — 6 provider requests, 3 worker
runs, `settle completed`. Run 1 exposed that the public client must accept the
recoverable `response/error` (the legacy "ordinary response error becomes
visible"); it now does so only for that case.

Legacy `skill_live_compact_transition_gate_requires_queued_request_before_new_generation_frame`
is ported as `engine::first_post_compact_attempt` (`crates/engine/src/compact_gate.rs`)
with the three original expectations over ledger events (pass, absent request,
stale generation). Inventory `not_run` 10 → 8.

### Post-turn memory merge — the legacy memory child (2026-09-05, night)

Legacy `live_memory_smoke` forked a memory child after the root finalized and
expected at least one digested memory record for the lineage scope. Kernel
port: `settings.memory_merge` (config-v1) enables a supervisor helper that runs
when a root line exits `settled`/`completed`: transcript from the admitted
inputs and `final_answer` outputs, one no-tools request to the session's model
(`AUTOMATIC_MEMORY_MERGE_SYSTEM`, JSON facts), then
`WorkflowBackend::automatic_memory_merge` stages the facts and memorizes them
together with every candidate the turn noted, keyed per thread and turn. The
first live run exposed that no turn-bound record can follow the settle
(`constraint 8 ... appended after settle`), so the memory log is the store of
record and the ledger is not written — matching the legacy context-store
placement. Live run 2 (`/tmp/tekes-memory-merge-20260905-2`): root answered
`tokyo`, merge staged 1 and memorized 1 fact. Unit coverage:
`slice8_workflow::post_turn_memory_merge_stages_memorizes_and_is_idempotent_per_turn`,
`process_host::tests::{memory_merge_reply_parsing_tolerates_fences_and_drops_empty_facts,post_turn_memory_merge_uses_the_provider_runtime_and_records_one_receipt}`.
Inventory `partial_live_evidence` 14 → 13.

### Legacy SQLite corpus import and replay (2026-09-05, night)

The production `~/Library/Application Support/TekesAppServer/appserver.sqlite3`
is empty (one workspace, no records); the real corpus is the TekesUI-hosted
AppServer store (`~/Library/Application Support/TekesUI/AppServer/appserver.sqlite3`,
45,648 records, 723 sessions, 10 workspaces), snapshotted with `VACUUM INTO`
into the session scratchpad and opened immutable. `scripts/replay-legacy-sqlite.py`
(`/tmp/tekes-legacy-replay-20260905-1/report.json`) proved integrity, every
lineage kind under the legacy grammar (`task.repair(pdf2dxf.py)` ×2,
`task.repair(pdf2dwg.py)` ×2, `branch.repair.coordinator` ×2 among 700), zero
reserved candidates, and exported 60 root threads. The Kernel import test
(`kernel-replay-receipt.json`) rebuilt 1,051 events, projected 960 journal
records, paged 158 turns / 158 user messages / 145 turn ends / 93 assistant
messages (71 legacy finals) / 110 tool calls, idempotent on replay.
Inventory `not_run` 8 → 7 (B5 ×2, SWEbench ×5 remain by ruling).

### Queued input on the spawned host, partial-entry regrade and the crate documentation audit (2026-09-05, night)

`scripts/run-public-flow.py --queue-during-turn` submits a second ordinary
prompt through the external v3 client while the first turn's tool call is in
flight; the client proves turn 2 opens only after turn/end 1 (and after the
validator on the write case), the scripted provider proves the queued prompt
never reaches turn 1's requests, and the flow test counts both settles.
Scripted read and write (`/tmp/tekes-queue-release-20260905-{read,write}`),
live Cloudflare openai/gpt-5.6-luna and Kimi K3
(`/tmp/tekes-queue-release-live-{cf,kimi}-20260905-2`, `--live-provider` /
`--live-secret-name` added) all passed. The first attempt exposed a reused
`rpcId` (`idempotency-conflict`), which is the endpoint working as specified.

Thirteen `partial_live_evidence` entries were re-audited against tonight's
evidence: ten regraded to passed (spawned-host startup via route probes,
queue release with external clients, v3 drain as the legacy shutdown, retired
metadata interfaces, sequence adaptation settled by design, provider-behavior
exceptions recorded as retained failures); three remain partial with narrowed
gaps (simpleWriteFile / simpleWriteRepair: DeepSeek credential absent from the
runners' keys file and cache assertions covered elsewhere; skill projection
after effective compact: a live-model adherence run through the named compact
command). `check-live-test-inventory.py`: 99 registered, 0 missing. Final
counts: `not_run` 7 (B5 ×2 and SWEbench ×5 by ruling), `partial_live_evidence` 3.

Crate documentation audit (21 crates): every README entry point and link
resolves against the sources (scripted check), method-level claims
(`exchange_tool_control`, `dispatch_with_pre_mutation`, `sandboxed_with_scratch`,
`RewriteKind`, `validation_decision`, `has_causal_offer`, `remote_mux_loop`,
`unary_inner`, `MuxHostDescription`) verified. One stale statement fixed
(provider: "automatic worker native-mode routing is not yet implemented"), and
tonight's behaviors added to provider, mcp, supervisor, engine, worker, schema,
tools, worker-control and endpoint READMEs; generated references regenerated
with the architecture venv (`--check --links` clean).

### Closing the last three partial entries: DeepSeek legacy write/repair and the `/compact` command (2026-09-05, later)

Correction first: earlier regrades said the DeepSeek credential was absent from
the runners' keys file. It is present as the field `deepSeekKey` (the scan
that missed it matched lowercase names only); the three entries and this audit
now say so.

`simpleWriteFile` / `simpleWriteRepair` run on the single legacy provider
(DeepSeek). `scripts/run-live-kernel.py` `legacy_write` and `validation_repair`
passed on the configured DeepSeek Responses route
(`/tmp/tekes-legacy-deepseek-responses-20260905-1`) and on a DeepSeek
Chat-dialect route built from the same credential
(`/tmp/tekes-legacy-deepseek-chat-20260905-1`). The harness now also asserts
the legacy repair shape: one or two bounded rounds (`validation.feedback`), the
initial validator plus one per round (`spawn`), and both writes in the same
worker. Observed: write — one validator, one turn, exact file and answer;
repair — one round, two validators, two `apply_patch` writes, one settle. The
Chat route retains the legacy cache pair on every root and validator usage;
the Responses route reports `cache_read` only (no explicit miss counter from
DeepSeek Responses, none inferred).

`workflow_live_skill_projection_resets_and_reloads_after_effective_compact`
needed the missing product piece: the named `compact` command. command-catalog-v1
gains the reserved verb — an expanded body that is exactly `compact` is the
manual compaction request, never input. `SessionDeliveryAuthority::compact`
delivers worker-control `compact` to a live worker or, with none alive,
authors the origin-keyed `compact` under the line lock (`locked_compact`:
checkpoint at the key floor, `engine::plan_context_compaction`, summary inline
or spilled) through the new ledger-owning keyed append
(`ThreadStore::append_keyed_with_ledger_if`, `NativeEndpoint::author_keyed_with_ledger`).
Unit: `compact_verb_maps_to_the_manual_compaction_request_not_an_input`,
`locked_manual_compact_covers_settled_history_and_is_keyed_by_origin`.
Live (`scripts/run-public-flow.py --live --case text --skill-compact`,
`/tmp/tekes-skill-compact-live-20260905-3`): PRE turn skill chain and exact
marker; `commands/run compact` → compact seq 27 (retry deduplicated); POST turn
on compaction epoch 31, first attempt 32 admits input 28 (the gate over the
real ledger), fresh skill chain, exact marker; the first post-compact request
has no pre-compact tool items, only the `[compacted history]` summary. Run 1
exposed that extension methods must be percent-encoded on the single-segment
unary path and that the flow's default tool policy lacked the skill tools; run 2
exposed that the summary quotes covered events (so the id check is structural).
Inventory: `partial_live_evidence` 0, `not_run` 7 (all by ruling).

Regression note: `cargo test --workspace --locked` first failed only in
`capacity_release_starts_a_pending_queue_transaction_on_a_settled_tail`
(`wait_for_worker` timeout) while a live harness was compiling alongside; the
same happened once more in a 34 s loaded run. Six unloaded runs of the
supervisor suite on this tree and two at the previous pushed commit
(`f159581`, separate worktree) all passed (104 / 102 passed, 12 ignored), so
this is the fake-worker startup timing under machine load, not a change in
behavior. Everything else in the workspace run was green.

### B5: official SDK public tunnels (2026-09-05, later)

Frank asked whether the two blocked SDK-tunnel entries could close. Findings
first: no released official SDK implements the 2026-07-28 modern surface the
pinned tests used (`server/discover`, `x-mcp-header`, multi-round
`requestState`/`inputResponses`, modern subscriptions); npm's latest
`@modelcontextprotocol/sdk` is 1.30.0 at protocol 2025-11-25, with SEP-1686
tasks as `ttl`/`pollInterval`, status-only `tasks/get` and `tasks/result`.
Against that shape the Kernel task path did not interoperate (it required
`pollIntervalMs` and an inline `result`). `McpClient` now adopts the official
keys and completes a finished poll with one `tasks/result`
(`crates/mcp/src/client.rs`, spec §Official task shapes, fixture scenario
`official-task-shape`, gate `official_sdk_task_shape_is_adopted_and_completed_with_tasks_result`).

`scripts/mcp-official-sdk/server.mjs` runs the official TypeScript SDK
servers (tasks: `slow_echo`, taskSupport required; conformance: `sse_echo`
behind forced SSE); `scripts/run-live-mcp-official-sdk.py` exposes them through
the user's ngrok agent and runs `crates/mcp/tests/live_official_sdk_tunnel.rs`.
Loopback rehearsals exposed two server-side shape mistakes (tasks capability
must be declared; `createTask` returns `{task}`) and the first public attempt
exposed that the free ngrok agent refuses to run behind the shell's proxy
variables (ERR_NGROK_9009; the agent's direct control connection works, so the
runner strips the variables for it). Public run
`/tmp/tekes-mcp-official-sdk-public-20260905-2` (host
`unranked-dreamland-facsimile.ngrok-free.dev`, torn down afterwards): task
lifecycle working → working → completed, `task:public`; SSE round trip
`sse:round-trip`. Both entries move from `not_run` to passed with the stated
substitutions (TypeScript SDK for C#, legacy protocol mode) and the stated
unrunnable surface. Inventory `not_run` 7 → 5 (SWEbench ×5 by ruling).

### SWE-bench on the Kernel, Docker-free tiers (2026-09-05, later)

Frank asked to try the five SWE-bench entries and then ruled out installing
Docker. `scripts/run-swebench.py` ports the legacy four stages (provision from
a bare-mirror cache, drive one real turn through the spawned host with the
legacy instruction, grade, report) with the legacy fixture copied unchanged to
`fixtures/live/swebench/`. Tier 1 passed on psf__requests-1142
(`/tmp/tekes-swebench-pipeline-20260905-1`, 93 s, a plausible fix in
`requests/models.py`). Tier 1.5 over all ten instances exposed two Kernel
defects, both fixed with unit coverage and confirmed by reruns
(`/tmp/tekes-swebench-predictions-20260905-{1,2,3}`; 10/10 turns completed,
10/10 non-empty predictions):

1. Duplicate JSON member in model tool arguments (`artifact_outputs` twice,
   gpt-5.6-luna) was a `malformed_response` → `provider_terminal` → lost turn.
   Provider-adapter rule 7: unusable argument text becomes the
   invalid-arguments sentinel, identical at readiness and terminal; the worker
   refuses the call with a durable `tool_result` error and the model resends
   (`unusable_tool_arguments_become_a_refused_call_not_a_malformed_stream`).
2. Every worker exit fed the crash-loop restart backoff, so nine clean
   approval-hold resumes in one turn hit `process-host-restart-limit` and the
   approved `apply_patch` never ran. Exit reconciliation now counts only
   recorded failures (protocol failure or non-success exit status); a clean
   exit whose tail asks for a run clears the backoff (tail-lifecycle text
   updated). requests-1724 then completed with six runs and five approvals.

The three Docker-graded entries stay `not_run` with the ruling recorded; the
runner grades through the official harness automatically when `docker` and the
`swebench` module exist. The fixture manifest gate also caught that the
official-SDK Node project (with `node_modules`) had been placed under the
manifest-checked `fixtures/live`; it now lives in `scripts/mcp-official-sdk`
and `fixtures/manifest.json` declares the SWE-bench fixture files.
Inventory: `not_run` 3 (Docker, by ruling), `partial_live_evidence` 0.

### The last eight: what closed, what needs a decision (2026-09-05, evening)

- `workflow_live_direct_deepseek_validation_repair_and_cache_attribution`:
  closed. `scripts/run-public-flow.py --validation-repair` drives the legacy
  repair prompt on the DeepSeek chat route through the spawned host and an
  external client, asserts the repair shape in the ledger (one feedback round,
  two validators, two writes, one settle, exact proof.txt) and the public
  `usage.summary` / `usage.cacheAttribution` routes
  (`/tmp/tekes-validation-repair-cache-20260905-1`: one attribution row,
  cacheReadTokens 6528 of 8058).
- `apiTokenBearerCompletesAuthenticatedModernRoundTrip`: re-verified; the
  configured Cloudflare credential answers 403 `insufficient_scope` ("Token
  lacks required user:read or account:read scope"). An account API token with
  that scope is the owner's to create; the Kernel gate runs as soon as
  `TEKES_LIVE_CLOUDFLARE_API_TOKEN` carries one.
- `listsToolsFromLiveDeepWikiServer`: DeepWiki answers `server/discover` with
  HTTP 400 and a textual -32600 whose id is the placeholder `"server-error"`.
  mcp-runtime-v1 says such textual hints and mismatched identities never
  authorize downgrade, so Auto's refusal is the contract, not a defect. A
  relaxation (accept a -32600 "Unsupported protocol version" enumeration that
  lists an older version, even with a placeholder id) was prototyped with unit
  tests and parked in `/tmp/tekes-deepwiki-textual-downgrade.patch`; it is a
  contract change and waits for Frank's ruling.
- The three `blocked_contract_gap` entries (BriefV2 atoms, compaction v5
  shadow artifact, OAuth refresh-token persistence) are new versioned contracts
  (B6, still blocked by ruling); the three SWE-bench Docker entries stay
  skipped by ruling.

Update (2026-09-05, evening): Frank ruled to change the Auto-downgrade
contract. mcp-runtime-v1 now accepts a `-32600` "unsupported protocol version"
error that enumerates an older supported version as protocol-era evidence, even
with a placeholder response id; the HTTP transport surfaces such a body as the
remote error instead of a bare `HTTP 400` transport failure. Unit:
`textual_protocol_version_rejection_is_legacy_evidence_when_it_lists_an_older_version`;
live: `live_deepwiki_list_and_call` (Auto) passes against the public DeepWiki
service (`/tmp/tekes-turn-live-audit/mcp-http-auto-deepwiki-2.log`). The
`listsToolsFromLiveDeepWikiServer` entry is passed. Remaining non-passed:
3 Docker (ruling), 3 B6 contracts (ruling), 1 Cloudflare bearer (credential scope).

### B6 closed: Brief V2, compaction v5 shadow, OAuth refresh lifecycle (2026-09-05, night)

Frank ruled "do B6 too, plan first, then 1→2→3 with the recommended
decisions" ([plan](b6-contract-plan-2026-09-05.md)). All three landed and
passed live; each is one commit on `origin/main`.

- **Brief V2** (`4fb1ac6`): `policy.brief_schema_version: 2` selects the strict
  `brief` schema under the same name (catalog identity through the tools
  digest); the worker renders the host-owned source alias table (`s1` the
  inputs verbatim, `s2…` antecedents from the ledger) after the admitted
  inputs; `engine::read_brief_v2` checks ids/sources/goal and derives the
  effective completion (goal minus facts). Live on the Cloudflare OpenAI route
  (`run-public-flow.py --live --case text --brief-v2`): intent/fact/constraint
  atoms all citing sources, goal referencing the fact, explicit completion
  empty, input bytes unchanged. The first live attempt had the model exclude
  the fact from the goal and fill completion; the fix was the v2 tool
  description, not the assertion.
- **Compaction v5 shadow** (`97fe20d`): planned covers frozen as a source
  bundle (sha256 over canonical bytes, rendering capped at 60 % of the window),
  one compactor-role request with `summary_artifact` as its only tool
  (`evidence_refs` are `seq` addresses), admission
  (`engine::admit_shadow_artifact`), telemetry on the compact event as
  `shadow` (a `state` record was not possible: the supervisor's manual compact
  has no open turn), `settings.compaction_shadow: off|shadow|promote` (default
  `shadow`). Worker: the request runs inside the overflowing attempt's lease
  before auto-compaction; supervisor: on its own thread before the line lock,
  the bundle re-checked against the plan at write time. Live A/B
  (`run-compaction-shadow.py`, `/tmp/tekes-compaction-shadow-20260905-2`):
  promote 4/4 private codes recalled with the artifact admitted and promoted,
  deterministic 4/4. Not done: the 50 % prefetch band; a worker-authored manual
  compact at a yield carries no shadow (no lease). Two defects found on the
  way: the supervisor's shadow request panicked on the tokio thread ("Cannot
  start a runtime from within a runtime"; now its own thread), and the
  `summary_artifact` fixed schema oracle had to be regenerated.
- **OAuth refresh lifecycle** (`e6643ae`, `d5b78b6`): secret-store-v1 §OAuth
  secret mutation (`SecretMutationAuthority`; memory, Keychain
  `SecItemAdd`/`SecItemUpdate`, and the durable floor); Kernel-minted grants
  (`provider::OAuthGrant`, one item, canonical JSON material; mint/rotate/revoke
  only for Kernel-minted records); `provider::OAuthTokenExchange` (per-generation
  access-token cache, rotation write-back, 429 Retry-After honoured, HTTP 400 =
  exact legacy text `token endpoint returned HTTP 400` + revoked record, sticky
  per connector because the MCP client retries once); wired into the MCP HTTP
  authorization provider (`SecretAccess`), identity stable across rotations.
  Live against Cloudflare (`run-live-mcp-oauth.py`, two browser consents by
  Frank — the gate's own remote revocation consumed the first grant):
  rotation on both connects (floor generation 3), RFC 7009 revocation 200,
  third connector fails with the exact text, record closed. Learned: Cloudflare
  rotates on every refresh and answers an overlapping refresh with 429
  `temporarily_unavailable` + Retry-After; its `search` tool description is
  1760 bytes, above the then 1024-byte dynamic-schema cap — Frank ruled to
  raise it to 4096 (`5957a27`).
- **SWE-bench** (same night): a Docker-free official grading lane was proven
  (official arm64 instance images + official eval scripts and log parser under
  the Apple `container` runtime; 6/10 legacy predictions resolved) and then
  stopped by ruling before the scored suite; the harness changes are parked in
  a git stash, the three inventory entries stay `not_run`.
- **Documentation alignment** (`c563f6d`…`ce4063d`): architecture index
  regenerated, module maps completed, events/context/feature-parity updated,
  and all 21 crate READMEs audited claim by claim against source
  (`crate-readme-semantic-audit-2026-09-05.md`; two wordings corrected).

Inventory after this section: 99 entries, 95 passed-class; remaining 3
SWE-bench Docker grading (stopped by ruling) and 1 Cloudflare bearer
(credential scope, Frank's account token).
