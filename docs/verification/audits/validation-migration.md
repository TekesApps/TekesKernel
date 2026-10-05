# Session final and validation settlement migration

Status: implementation in progress; neither live parity nor runtime acceptance is complete.

## Authoritative legacy contract

The archived TekesRuntime sources are available in the audit checkout at
`/tmp/tekes-turn-live-audit/TekesRuntime`; exact checkout revisions are recorded in
the maintainers' live-test inventory.

- `Docs/sessions.md:88-109`: root owns turn settlement; the executing worker may
  issue many model responses in one session. Its candidate output enters
  validation, and failed validation resumes the original append-only worker.
- `Docs/validation-loop-charter.md:76-93`: every root worker final enters
  validation settlement, including artifact-free output. Exact accepted snapshot
  coverage avoids a new judge; fail coverage does not release the candidate.
- Charter C2/C3: two negative rounds permit one repair; capped failures and
  validator death settle as `inconclusive`, never fabricated `pass`. A unique
  writer-owned settlement promotes an immutable, exactly identified output.
- Charter C4/C5: feedback delivery and wake are durable, generation-bound and
  replayable; only host-authorized candidates with matching turn, output and
  validator provenance can cause routing or settlement.
- `Sources/Orchestration/Orchestration.Facts.swift:563-573`: final-answer metadata
  stops the session loop. Lines 661-686 separately park validation-held output;
  delivered feedback wakes that same session.

## Initial Kernel gaps found by source inspection

These are the initial migration findings, preserved as historical evidence.
They are not a current open-defect checklist: subsequent sections record
implementation and tests. The current A-to-code review is maintained in
[thread-alignment](../thread-alignment.md); the thread definition is in [Thread definition](../../concepts/thread.md).
This clarification does not upgrade any historical live-acceptance status.

1. The provider terminal paths in `crates/worker/src/main.rs` currently append
   `settle(completed)` directly for normalized final answers, including adopt
   recovery. This needs the same validation entry point on both paths.
2. `WorkflowBackend` implements `verify` result formatting, but formatting is not
   validation orchestration or settlement authority.
3. Existing child creation is tied to model tool calls. A validator needs a
   host-owned lineage and frozen candidate/snapshot binding, without fabricating
   a model tool call.
4. `LockedLedger` writes one JSONL event per append. Writing a negative round and
   settlement as unrelated appends would introduce the crash window forbidden
   by the charter. Use a single authoritative compound decision record with
   idempotently recoverable projections, or introduce an actual transaction
   mechanism before splitting these facts.
5. Validator context must contain the exact candidate and frozen artifacts,
   not the main worker's entire conversation. Repairs must preserve the original
   worker profile and append structured feedback.
6. Endpoint completion must follow the promoted output identity and validation
   outcome; physical provider final metadata alone is insufficient.

## Verification obligations

- Multi-response tool/commentary/reasoning loop reaches a candidate only on final.
- No-artifact and already-covered candidates take explicit `not_required`.
- Fresh artifacts invoke a real validator with the same task tools plus `verify`.
- Fail feedback reaches the original worker; changed artifacts are rejudged.
- Unchanged failed snapshots consume the standoff round without another judge.
- Pass, inconclusive, capped fail and validator death have distinct evidence.
- Duplicate/late/foreign verdicts cannot settle or wake another turn.
- Crash recovery at candidate, decision, feedback, wake and promotion boundaries
  neither loses work nor creates a second executor or second settlement.
- Real provider and legacy Runtime/AppServer live scenarios remain individually
  recorded as passed, failed, pending or externally blocked. Pure reducer tests
  do not count as end-to-end validation-loop evidence.

`engine::validation_decision` covers the pure C1-C3 decisions. The writer now
provides `begin_validation`, `commit_validation_decision`, and
`materialize_validation_settlement`: candidates are immutable, decision records
contain the negative round and disposition together, and settlement can be
recovered by exact decision/output references without rewriting the output.

Six pure decision tests and five worker-harness persistence tests pass. The
persistence tests reopen actual JSONL files, reject nonfinal/foreign candidates,
reject model output as control evidence, verify replay without duplicate writes,
and exercise fail followed by unchanged-snapshot inconclusive settlement.

The production provider path now enters this bridge before another request and
after final output; adopted finals also return to this path. Host-owned validator
children use a frozen judge system, the executor tool selection plus `verify`,
and a seed containing only the current request, candidate and artifact snapshot.
Fail feedback is synced to the original worker's ledger. Candidate decisions,
child identities, verdict references, and promoted output references are reused
on recovery. Tool-result coverage must match the host snapshot exactly.

Worker regression before the additional host-spawn check: 55 passed, 1 intentionally ignored live test. Real DeepSeek
HTTP runs in `/tmp/tekes-turn-live-audit/kernel-validation-live-2` passed both
`write_read` and `validation_repair`: the latter has a fail verdict, feedback to
the same worker, a second validator for changed content, and a single pass
settlement. The harness runs the actual child worker loop but supplies supervisor
launch/lease replies; it does not prove daemon process recovery or UI acceptance.
Frozen file assets are secret-scanned, and file hashes are checked again before
accepting a verdict. The latest coverage-rejection refinement still needs a
focused regression beyond compilation.

Outstanding: full daemon crash/relaunch matrix and held-child delivery, endpoint
promotion presentation, delegated/dynamic artifact inventory, large/binary
artifact context, all remaining live-test parity rows and provider matrices.

## Additional execution evidence

- `/tmp/tekes-turn-live-audit/kernel-repair-spill-fixed-1/matrix.json`:
  DeepSeek validation repair passed using a frozen test binary after the large
  reasoning spill fix. The saved SHA identifies the tested implementation.
- `/tmp/tekes-turn-live-audit/validator-host-lineage-1.log`: validator activation
  rejects a mismatched spawn identity, child filename, turn, call or sequence;
  matching only a candidate reference cannot grant the validator role.
- `/tmp/tekes-turn-live-audit/kernel-validator-lineage-1/matrix.json`: real
  write/read and independent validation passed with that stronger host binding.
- `/tmp/tekes-turn-live-audit/kernel-shell-fd-cwd-2/matrix.json`: real shell
  continuation passed after replacing sandbox-denied ambient canonicalization
  with descriptor-relative working-directory resolution.
- `/tmp/tekes-turn-live-audit/mcp-regression-current-2.log`: MCP deterministic
  regressions passed; real Cloudflare docs and DeepWiki round trips passed in
  `mcp-cloudflare-docs-7.log` and `mcp-deepwiki-3.log` respectively.

The memory scenario now stages a note, reconciles its returned candidate ID with
memorize, then recalls the saved key. This is only same-session tool coverage;
it is not parity proof for the legacy post-turn memory child lifecycle.

Latest worker regression (`worker-validation-integration-5.log`): 56 passed,
1 intentionally ignored live test. The revised memory live scenario passed in
`kernel-memory-reconcile-2/matrix.json`, including an assertion on the actual
recalled fact rather than only the model's final marker. None of these results
closes the daemon lifecycle or complete legacy inventory gates above.

## Inventory completeness correction

The initial 63-row list missed inline Swift `@Test … func` declarations and
entire AgentEval/SWEBench scoring suites. The reproducible annotation audit now
registers 99 tests, including all tests from mixed live/offline suites. This is
an inventory count, not a claim that 99 external live tests exist or passed.
A private inventory check against checkouts of the legacy repositories detects
missing rows against pinned source revisions.
The current check reports zero missing declarations in that discovered scope.

The newly recovered `workflow_queued_root_input_waits_for_validator_and_respects_session_projection_boundaries`
test (legacy AppServer workflow source line 1200) launches a real CLI against a
scripted local model server. It queues two root inputs, creates an artifact,
requires a separate validator read/verify, and then runs the second input.
The Kernel in-process child callback harness does not satisfy this process and
queue boundary test. It remains pending rather than being credited to the
existing real-provider write/read scenario. Newly discovered scoring gates also
remain pending; deterministic transport tests in mixed suites need individual
classification and are not counted as provider live evidence.

## Real worker process recovery gate

`real_worker_recovers_final_before_settlement_with_resume_never` launches the
built production worker via `ProductionProcessHost` from a durable final-output
crash boundary. It uses no provider configuration: an artifact-free final must
complete its host-owned `not_required` settlement without obtaining a model.
The test checks the original output sequence, exactly one settlement, and no
remaining runnable recovery obligation after process shutdown.

Run after `cargo build -p tekes-worker -p tools --bins`:
`TEKES_TEST_REAL_WORKER="$PWD/target/debug/tekes-worker" cargo test -p
tekes-supervisor --lib real_worker_recovers_final_before_settlement -- --ignored`.
Evidence: `/tmp/tekes-turn-live-audit/real-worker-validation-recovery-1.log`
(1 executed and passed). This is a real worker process with the production
process host, not a full installed supervisor/UI or artifact-validator process
chain acceptance test.

The gate exposed provider preparation preceding durable validation recovery.
Recovery/validation now runs first; no-provider final recovery also has a focused
worker regression. Latest worker suite: 57 passed, 1 ignored live entrypoint
(`worker-validation-integration-6.log`). A supervisor regression fixture was
missing its workspace profile and failed before reaching its intended spawn
failure; it now creates that profile and its focused test passes.

## Endpoint completion boundary

`projection::tests::worker_final_and_validation_feedback_do_not_end_the_client_turn`
passed (`endpoint-validation-boundary-2.log`, one actual test). Candidate final,
repair feedback and repaired final produce no `turn/end`; settlement alone does.
This checks endpoint completion ownership, not final-answer presentation.

Current client inspection found a separate presentation gap:
`Tekes/Tekes/SessionEndpoint/Client/SessionEventProjector.swift:450` marks every
tool-free assistant message as the answer carrier; at line 571, completed
`turn/end` promotes `lastAssistantByTurn`. Kernel currently does not project
`settle.validation.promoted_output_seq` into the wire event. Exact immutable
candidate promotion therefore still needs a coordinated wire/client contract,
including compatibility and replay tests; the completion-boundary pass above
must not be credited as UI promotion acceptance.

## Settlement record shape

The optional `settle.validation` payload is now schema-checked: only completed
settlements can carry it; terminal validation outcomes are pass, inconclusive,
or not_required; positive output/candidate/decision references must precede the
settlement in causal order. A dedicated malformed/missing-field regression
passed (`validation-settlement-schema-2.log`), along with six worker validation
regressions (`validation-schema-worker-1.log`). This is structural validation;
writer-owned exact-source identity checks remain a separate requirement.

## Recovery binding recheck and schema live execution

`kernel-settlement-schema-live-1/matrix.json` records real DeepSeek write/read
and validation repair passes on the same frozen binary (SHA in each row), after
the provider-preparation ordering and settlement schema changes. This run
precedes the additional settlement materialization binding check below.

Materialization now rechecks host visibility, candidate kind, exact worker/turn,
prior final output, and causal references instead of trusting deserialization
of a persisted decision alone. The recovery regression modifies a candidate's
worker binding, reopens its JSONL and verifies rejection with zero additional
writes (`validation-settlement-binding-1.log`). The later binding check has
regression coverage, not yet a new external-provider run of its own.

## Additional crash boundaries and GLM evidence

The real-worker process test now executes three durable interruption boundaries:
output, candidate, and terminal decision. Each launches the production worker
through ProductionProcessHost with resume=never and no model configuration,
then checks exactly one candidate, one decision, one settlement and the original
output identity. All three cases passed in one executed test
(`real-worker-validation-recovery-2.log`). This still excludes a running external
validator and full supervisor process crash/relaunch.

The pinned Anthropic/GLM recheck completed (`kernel-provider-recheck-1/matrix.json`).
GLM repair reached a passing validator and completed settlement, with no earlier
16 KiB spill failure, but the strict final-marker assertion still failed because
of extra prose. The cell remains failed. Anthropic refusal/inconclusive and GLM
write prose failures are recorded in `live-provider-failure-audit.json`.

## Authenticated MCP live gate

The Cloudflare bearer round trip was actually executed against
`https://mcp.cloudflare.com/mcp`; discovery returned HTTP 403 Forbidden
(`mcp-cloudflare-bearer-1.log`). The configured Cloudflare credential was passed
only through the child environment and redacted from captured output. This gate
is blocked on service-authorized credentials, not passed or skipped.

The test now also checks the negotiated modern version, both search/execute
catalog entries and nonempty text from read-only search. Those additional
assertions compile (`mcp-live-services-build-1.log`) but cannot execute beyond
the rejected handshake. Legacy advertised-supportedVersions equality is still
not exposed by the client, so its parity row remains partial rather than fully
ported. Official SDK conformance/task tunnel environment URLs are absent in
this session; those scenarios still require their server setup and ports.

## Advertised MCP version metadata

McpClient now retains discovery-supported versions and clears them with the
connection generation; legacy initialization exposes its negotiated version.
The bearer test now asserts the exact legacy expected version list as well as
catalog and nonempty text. Its implementation parity is complete, but execution
remains authentication-blocked by the observed 403.
A real public Cloudflare docs round trip passed with the retained metadata check
(`mcp-cloudflare-metadata-live-1.log`); it does not prove bearer authorization.

## Failed-decision feedback crash window

`failed_decision_recovery_delivers_feedback_once_to_the_same_worker` seeds a
host fail verdict and committed feedback decision, closes the JSONL before
feedback delivery, then runs the production validation bridge and reopens twice.
It verifies one model-visible feedback record with the original output/decision
identity and ordinal, no replacement worker spawn, and no premature settlement.
The focused test passed (`validation-feedback-recovery-2.log`). This exercises
durable replay in-process; it is not proof of supervisor-delivered feedback
while an external validator process is active.

## Stop during validator launch/wait

A child launch/wait cancelled by user stop returned a generic error that the
provider-loop wrapper could turn into an internal-error settlement before the
stop receipt. The wrapper now returns control to the existing post_turn_exit
stop owner when cancellation is user-stop; protocol failures retain their
separate handling. The regression drives an actual validator launch request,
delivers stop instead of launch_result, and confirms no error/settle is written
before the stop owner runs (`validator-stop-ownership-1.log`, passed).
This does not independently prove the full installed UI stop cascade.

## Stop with an unresolved validator spawn

The extended launch-stop regression drains the deferred stop through the real
control delivery handler and calls post_turn_exit. Because the validator spawn
remains unresolved, the correct behavior is a durable active stop and a failure
exit for supervisor tail recovery, with no parent settlement yet. The initial
extended assertion incorrectly expected immediate user_stop settlement and
failed; it was corrected to preserve child cleanup ownership. The corrected
regression passes (`validator-stop-ownership-3.log`). Completing user_stop after
actual validator reaping still requires the full process-chain gate; this test
must not be reported as that acceptance.

## Snapshot descriptor-bound read

Artifact freezing now opens with O_NOFOLLOW/O_NONBLOCK, validates the opened
file's metadata, and reads at most 16 MiB plus one sentinel byte. It rejects
nonregular files and oversized/growing content, avoiding separate path metadata
inspection followed by an unbounded open/read. A focused regression covers
regular content, symlink, directory and oversized files
(`validation-snapshot-read-2.log`). Ancestor-path authorization and comprehensive
delegated/dynamic artifact inventory remain separate outstanding work.

## Same-response tool batch live scenario

`parallel_tools` asserts that at least two tool calls share one provider attempt
and every call in that batch has a successful result before the final marker.
Real DeepSeek and OpenAI gateway runs passed in `kernel-parallel-tools-1` and
`kernel-parallel-tools-openai-1`. This uses two think invocations; legacy
`live_openai_cloudflare_eager_parallel_tools` uses two distinct strict tools and
also asserts eager callbacks before the terminal manifest. Those additional
requirements remain pending, so the parity row is partial.

## Live evidence survives process interruption

The live harness now writes its ledger and assets under each cell's
`runtime-thread/` from initialization instead of copying the only ledger after
normal completion. The runner also saves redacted partial stdout on timeout.
A real text run passed (`kernel-durable-live-evidence-1`). A separate forcibly
killed test process retained four initial JSONL records
(`kernel-killed-evidence-2/interruption-check.json`); this verifies preservation
only and is explicitly not a completed provider test. Full-run result and
root-level copied evidence retain their previous locations.

## Exact live-test build artifact selection

The live runner now selects the executable from this invocation's Cargo
compiler-artifact JSON (test profile and tekes-worker target), requiring exactly
one match. It no longer guesses by modification time among old build products or
rewrites the shared deps-directory helper. It still freezes the selected test
binary and helper and records the test binary SHA. A real text scenario passed
with this path (`kernel-cargo-artifact-selection-1/matrix.json`). Zero provider
matches already fail explicitly; no zero-execution pass was found there.

## Wire promotion identity implementation

Kernel now projects explicit output finality as sessionFinal and resolves a
validation settlement's output reference to promotedMessageID plus
validationOutcome. The regression deliberately promotes an earlier output while
a later assistant exists, proving ID resolution is not last-message selection.
Endpoint library tests passed (`endpoint-output-identity-2.log`, 22 tests).
Old records without final_answer/validation retain their old payloads. The
Swift client still requires the coordinated consumption/persistent candidate
map and migration/replay tests; wire support alone is not UI acceptance.

## Swift client exact promotion

The Tekes Swift projector now keeps an optional Codable candidate map for
explicit sessionFinal messages, waits for settlement before marking these
messages final, resolves promotedMessageID in the exact turn, and uses the
selected message for the turn output. Unknown promotion IDs fail projection
instead of silently selecting the last message. The map is cleared per completed
turn; old checkpoints decode without it and old wire events retain legacy behavior.

`validationPromotionSurvivesCheckpointAndSelectsExactMessage` persists/reloads
fold state with two candidates and promotes the earlier one. The whole
SessionEndpointCoreTests suite executed 46 tests and passed
(`client-validation-promotion-suite-1.log`). The earlier single-method Xcode run
reported zero tests and is NOT accepted as test evidence. Installed-app visual
acceptance and full cross-process validation delivery remain outstanding.
Changes are in the sibling Tekes checkout, not just TekesKernel.

## Promotion rejects nonfinal targets

Only explicit final outputs enter the Kernel promotion map or the Swift
candidate map. Swift regression coverage now also rejects both a missing ID and
a known commentary message instead of promoting the last message. The client
suite executed 47 tests and passed (`client-validation-promotion-suite-2.log`);
Kernel endpoint regressions passed (`endpoint-output-identity-3.log`). These
checks supplement the checkpoint/exact-earlier-output test and do not replace
installed-app or full worker/validator process acceptance.

## Real validator child failure chain

The process-host recovery test now has a fourth case with a real artifact
snapshot and no provider configuration. The parent production worker launches
an actual validator worker through ProductionProcessHost. Assertions require a
child run_start and error settlement, one parent child_result, one
validation.death, and an inconclusive root settlement referencing the original
output. The three earlier crash boundaries and this child-failure case all
passed (`real-validator-death-1.log`, one test executing four scenarios).
This proves the negative parent/child process chain with an in-process production
supervisor host. It does not prove a real-model validator pass/repair across
processes, queued-input release, or installed supervisor/UI acceptance.

## Queued input exposed a recovery restart loop

Adding a second pending input behind an unfinished validation candidate caused
repeated run_start records without validation progress. Ordinary recovery was
opening a new turn merely because turn_open_inputs was nonempty, despite the
old turn lacking settlement. The startup path now requires terminal_tail before
opening a new queued-input turn.

The five-case real-worker test now includes queued input behind actual validator
failure. It asserts first settlement precedes second turn_open; the second turn
then settles as provider-unavailable error because this fixture intentionally
has no model configuration. All cases passed (`real-validator-queue-2.log`), and
61 worker tests passed (`worker-validation-integration-10.log`). This covers the
negative validator queue path, not successful real-model process validation or
all legacy projection-boundary assertions. The original failed restart-loop
trace is retained in `real-validator-queue-1.log`.

## Branch artifact parity contract clarified from original source

Legacy `Orchestration.Action.Validate.swift:38-42` explicitly defines the
snapshot as branch-agnostic. `Orchestration.Coverage.swift:54-81` reads root
artifact facts newest-first, selecting each ID's latest globally ordered version
and retaining source_session_id. `Orchestration.Session.swift:362-390` states
that internal write emits a neutral artifact with unconditional upstream_TTL=*,
while the local write result is independent; external/MCP tools emit no artifact
by default.

Current Kernel snapshot collection from local apply_patch/shell results is
therefore insufficient for delegated writes. Scanning completed child ledgers
at join time is not equivalent: completion order differs from write version
order and can overwrite a newer parent artifact with an older child version.
The remaining implementation needs a root-owned, durable artifact identity and
version propagation path, preserving source ownership and replay order before
validation capture. Generic MCP/dynamic outputs should not be treated as
artifacts merely because they contain path/hash-looking text. This replaces the
earlier broad 'dynamic artifact inventory' assumption with the precise legacy
contract; no branch-parity completion is claimed.

## Existing artifact version authority to reuse

Current source inspection found `engine::DurableArtifactVersions` in
`crates/engine/src/system_tools.rs`: it already serializes reserve/commit under
artifact-versions.lock, writes canonical synced JSONL and tracks per-path
versions. Worker backend assembly shares it at storage_root/tool-state.
The propagation work should reuse this authority rather than inventing a second
version domain. Important gaps: records currently carry path/call/version/hash
but no root-turn/source-session ownership; observe also records reads, so all
observed paths cannot be treated as produced artifacts; shell declared artifacts
need equivalent provenance/version treatment. Snapshot membership must remain
root-turn scoped, not every artifact in the shared storage authority.
No implementation of that ownership extension is claimed yet.

## Shell declared artifact version tracking

Successful shell artifact verification now observes its hash through the same
DurableArtifactVersions authority and returns artifact_version in the tool
result. Declared artifacts require that authority before command execution;
commands without declared artifacts preserve their existing behavior. Engine
regressions passed (37 tests, `engine-shell-artifact-version-1.log`).

A new real shell_artifact scenario creates proof.txt with shell, reads it,
asserts the emitted artifact_version and requires validator pass plus the saved
file. DeepSeek passed (`kernel-shell-artifact-version-1/matrix.json`). This
supplies missing version data; root-turn/source ownership, canonical path
identity across branches and propagation/snapshot integration remain pending.

## Reproduced artifact path alias defect (open failing regression)

`artifact_path_aliases_share_one_version_history` creates note.txt through a
relative path (version 1), then reads the same file by absolute path. Current
behavior returns version 0, proving separate history keys. The desired assertion
(version 1) currently fails (`artifact-path-alias-regression-1.log`). This is an
open regression, not a passing characterization test. The prior 37-test engine
pass predates this test and must not be presented as current all-green status.
A fix must canonicalize workspace/root identity and address old raw-path journal
keys without silently resetting existing artifact versions.

### Artifact path identity regression correction

The failing `artifact_path_aliases_share_one_version_history` regression is now fixed: system-tool read, patch, and declared shell artifact observations use the absolute configured mount plus normalized helper-relative path as their durable version key. User-facing path spelling is unchanged. Existing absolute records retain their versions; different workspace roots do not share versions. Bare relative legacy records have no mount provenance, so matching absolute observations/reservations fail with an explicit migration conflict without modifying the journal instead of guessing ownership or restarting at version zero.

Validation: `cargo test -p engine` passed 40 tests, 0 failed (`/tmp/tekes-turn-live-audit/engine-path-identity-1.log`). Added coverage for relative/absolute aliases, absolute-history restart preservation, workspace separation, and ambiguous legacy records preserving journal bytes. This replaces the earlier open path-alias failure status only; it does not establish full live parity.

Further source comparison confirms an outstanding validation-scope gap: legacy `Orchestration.Coverage.swift:54-81` reads root artifact facts newest-first and retains source session identity; `Orchestration.Session.swift:362-390` documents unconditional upstream artifact propagation independent of local tool-result delivery. Current Kernel `validation_runtime.rs::snapshot` reads only local successful patch/shell results, keyed by user-facing path. Cross-session propagation, canonical snapshot identity, and version-aware selection remain incomplete. A child-result completion-order merge would not satisfy the legacy contract.

### Canonical validation snapshot identities and version selection

`validation_runtime.rs` now normalizes relative and absolute artifact paths to the configured workspace identity before constructing the candidate snapshot. A revision fold retains source session and source sequence while selecting the greatest shared artifact version. Equal versions with different hashes are rejected; unversioned legacy history may use sequence only within the same source session, never to order different branches. Cross-session traversal/propagation is still pending; the selection logic is integrated for local results and tested for future multi-source collection.

Evidence:
- `cargo test -p tekes-worker --bin tekes-worker`: 64 passed, 0 failed, 1 deliberately ignored live entry; `/tmp/tekes-turn-live-audit/worker-snapshot-identity-1.log`.
- Real DeepSeek v4 flash `write_read` and `validation_repair`: both passed on frozen worker SHA256 `1ce34ec553cc1ee026155636c95b2295809b17d844d4b4439c3b00c8c434e88f`; `/tmp/tekes-turn-live-audit/kernel-canonical-snapshot-live-1/matrix.json`.
- Repair root ledger: candidate 29, feedback 35, repaired candidate 55, sole completed settlement 61. The harness uses actual provider calls and tool/helper execution but an in-process child-launch callback; this is not proof of real-process validator pass/repair or installed-app acceptance.

### Joined worker artifact tree collection

Candidate snapshot construction now walks the durable spawn tree for the root turn, verifies each child genesis against parent filename, spawn sequence, spawn ID, workspace, and child identity, and folds successful internal patch/shell results from joined ordinary descendants. It reads verified spill assets without acquiring another worker's write lock. Validator branches are excluded only after their host-spawn binding is checked. Paths outside a single child ledger basename, cyclic/repeated traversal, and unmatched child lineage fail rather than silently producing an empty snapshot. Revision selection uses shared artifact versions, not traversal or child completion order.

The existing production task protocol integration fixture now includes an artifact-producing child ledger. Root snapshot collection sees that artifact, while a rewritten foreign spawn ID is rejected. Evidence: `/tmp/tekes-turn-live-audit/worker-artifact-tree-integration-3.log` (one targeted integration test passed), `/tmp/tekes-turn-live-audit/worker-artifact-tree-2.log` (64 passed, 1 ignored), and `/tmp/tekes-turn-live-audit/kernel-artifact-tree-repair-1/matrix.json` (real DeepSeek validation repair passed). The final basename hardening was covered by the targeted integration rerun; the full suite and live run precede that one-line hardening.

Remaining gaps are explicit: this is snapshot-time collection of durable tool results after joins, not the legacy unconditional artifact-fact upstream stream. Writes whose filesystem commit outlives a missing tool-result append still need durable artifact provenance/recovery. Multi-level and competing-branch process integration, real-process successful delegation plus validation, durable source metadata in the candidate binding, and the broader live parity inventory remain open.

### Multi-level artifact tree and damaged-tail validation

Added a schema-validated four-ledger fixture (root, two children, grandchild). The root writes version 6, one child version 4, its child version 7, and the competing child version 5. Both sibling orders select version 7 and retain its grandchild source. A different root-turn scope sees no artifacts. Each fixture asserts that the scanner consumes every byte, preventing a truncated valid-prefix fixture from yielding false test confidence.

Snapshot collection now rejects a descendant ledger requiring tail repair. Previously the generic read helper returned only the valid prefix; for a frozen artifact snapshot this could silently omit the latest record. A deliberately torn grandchild tail now rejects the current scope while leaving unrelated turn scopes unaffected. Live-running generic ledger reads retain their existing prefix behavior.

Evidence: `/tmp/tekes-turn-live-audit/worker-artifact-tree-tail-1.log` reports 65 passed, 0 failed, 1 ignored. A freshly built real worker/helper pair passed the existing five-case process recovery test, `/tmp/tekes-turn-live-audit/real-worker-artifact-tree-recovery-1.log` (one test, five scenarios). This proves the targeted snapshot and recovery behavior; it does not close durable independent artifact publication, true process-level successful multi-branch validation, or the remaining legacy live-test inventory.

### Repeated identical calls live gate

Audited legacy `AppServerWorkflowSwiftTestingTests.swift:944-1045`: three consecutive context_get calls, no host loop block or consumed-summary substitution, model-owned final answer, spawned AppServer notifications and database assertions. Added Kernel `repeated_read` live scenario as partial loop coverage, not equivalent end-to-end parity. It requires exactly three identical read argument objects across three distinct attempts; each successful result must precede the next call and final output, with exactly one settlement. It does not weaken the old context_get/brief/endpoint/queue requirements.

Real DeepSeek passed on frozen SHA256 `710216116ffab4b95d54bd36261bb79f46a5544d5bae1eeea2cad92e29008d46`: calls at seq 9, 16, 23 on separate attempts, sole completed settlement seq 33. Evidence: `/tmp/tekes-turn-live-audit/kernel-repeated-read-live-1/matrix.json` and its retained root ledger. Inventory row is explicitly partial. Next integration needed: production ToolControlSession/ProductionToolControlHandler for supervisor context_get plus endpoint-level notification and queue evidence.

### Production supervisor context_get live integration

The live harness now routes tool_control requests through the real `ToolControlSession`, durable receipt/binding validation, `ProductionToolControlHandler`, and `ProductionToolControlPolicy`. Its retained ledgers use the production `runtime-host/threads/<session>` directory layout; the existing `runtime-thread` evidence path is an alias to that directory. Read-only operations use production behavior; process-mutating runtime/job operations explicitly return unavailable instead of simulated success.

Added `repeated_context_get`: three calls with identical arguments in three distinct responses. Every response must be successful, return exactly record 1 with the actual complete current-session genesis event and matching thread ID, and precede the next call and the final output. Exactly one settlement is required. This closes the tool mismatch in the previous repeated_read partial gate, but does not claim spawned-process/endpoint/brief/queue equivalence.

Evidence: `/tmp/tekes-turn-live-audit/kernel-supervisor-context-live-2/matrix.json` reports real DeepSeek repeated_context_get and validation_repair both passed. `/tmp/tekes-turn-live-audit/worker-live-control-regression-1.log` reports 65 passed, 0 failed, 1 ignored. Legacy inventory row remains partial with the outstanding transport and notification requirements spelled out.

### Endpoint ordering over actual provider ledgers

Live harness now reconciles each individual accepted event through production Endpoint Projector/Journal and retains `endpoint-events.json`. It asserts exactly one turn/end, authored only by the completed settlement, carrying the validation outcome and promoted message identity. Recovery replays the same event identities and must leave journal bytes unchanged. Added the endpoint crate only as a worker test dependency.

The first run exposed an incorrect harness assumption, not a production duplication bug: Projector.reconcile returns existing events on replay. Corrected the oracle to inspect identity equality and unchanged persisted bytes. Both original failures remain retained at `/tmp/tekes-turn-live-audit/kernel-endpoint-order-live-1`; corrected repeated_context_get and validation_repair cases passed at `/tmp/tekes-turn-live-audit/kernel-endpoint-order-live-2/matrix.json`.

This is production projection of a real provider ledger, not proof that a connected client received notifications over process transport. The legacy repeated-call queue-release/brief-delegation requirements remain open and are still marked partial in inventory.

### Six-provider repeated context_get matrix and optional schema fix

Executed repeated_context_get with endpoint-order/recovery assertions against every configured provider: `/tmp/tekes-turn-live-audit/kernel-repeated-context-all-1/matrix.json`. GLM, Google and Kimi passed; Anthropic returned content_filter; OpenAI rejected context_get's schema; DeepSeek reported malformed stream JSON.

The OpenAI rejection identified a Kernel serialization bug: strict mode was disabled only for composition constraints, but context_get has optional properties. Strict compatibility now also requires closed object schemas with every property required, recursively. Original schema fields are preserved; optional properties are not silently rewritten as mandatory or nullable. Tests verify actual Responses and Chat tool renderings plus nested schema behavior. Updated the exact-byte dialect oracle and its canonical digest for non-closed fixture objects whose strict claim also needed correction. `cargo test -p provider` passed 41 tests (`provider-optional-strict-2.log`).

OpenAI and DeepSeek rechecks both passed (`/tmp/tekes-turn-live-audit/kernel-optional-schema-recheck-1/matrix.json`). This proves the OpenAI schema correction; it does not establish the cause of DeepSeek's earlier malformed stream, whose raw failing frame was not captured. Anthropic's content_filter remains an unsuccessful applicable live scenario. Initial failures remain retained; matrix reruns do not erase them.

### Preserve malformed-stream diagnostic evidence

Added explicit opt-in HTTP response-body capture after credential-echo checks and before decoder ingestion. The default production runtime does not enable it. The worker test harness saves attempt-specific `.response.partial.raw` files even when decoding returns a failure, closing the evidence gap observed in the six-provider DeepSeek failure. Capture is bounded by the existing response byte limit, reset per send, and cleared if credential material is detected. Request headers are not captured.

The hermetic HTTP gate proves malformed JSON bytes are retained exactly and credential-echo responses leave an empty capture. Full provider suite: 42 passed (`/tmp/tekes-turn-live-audit/provider-response-capture-regression-1.log`). Real DeepSeek repeated_context_get passed (`/tmp/tekes-turn-live-audit/kernel-response-capture-live-1/matrix.json`); all four captures equal the completed raw responses byte-for-byte (`capture-check.json` in the scenario directory). This establishes future failure evidence collection, not a retrospective cause for the previously lost malformed frame.

### Production queue gate and literal retry-body regression

Extracted the existing startup turn-opening rule into `open_ready_turn`, called by the real worker startup. It re-reads durable lifecycle facts and consumes pending inputs only after settlement in ordinary mode. A new regression supplies literal user body `重试`, verifies no queued turn opens at session final, candidate decision, or reopen, then materializes settlement and verifies exactly one new turn consumes exactly that input. Reconcile mode does not consume it; repeated gate calls do not append another opening.

The real worker recovery fixture now also uses literal `重试` and asserts the next turn's trigger references that exact input record after the first turn's validation settlement. Evidence: `/tmp/tekes-turn-live-audit/queue-gate-worker-regression-1.log` (66 passed, 1 ignored); `/tmp/tekes-turn-live-audit/real-worker-retry-body-queue-1.log` (five recovery scenarios passed in one process test using freshly built worker/helper). The queued case still uses validator death/inconclusive and an unavailable provider for the second turn. Successful real-model validation plus completion of the queued second input remains open.

### Successful validation followed by a queued second provider turn

Added `queued_validation`. During the first write approval, the harness parks a real input delivery and drains it through the production writer path. It asserts the queue gate cannot open a second turn then, the queued marker is absent from the first worker's prepared requests, and the first artifact receives a passing validator. The production queue gate then opens the second turn, which calls the real provider and must finish with its own exact marker and one settlement. Both endpoint turn-end sequences are checked and retained.

The first two DeepSeek runs found a real full-history replay defect: queued input seq 13 was rendered between an earlier tool call and its result. The provider rejected turn 2 with `No tool output found for tool call ...`. Added capture for bounded non-success HTTP bodies (including credential-echo withholding tests), which exposed that underlying error instead of only the adapter's `terminal lacks status` message. The adapter's generic error-detail masking remains a separate diagnostic limitation.

Fixed projection ordering: ordinary inputs are rendered at their owning turn_open position, while original JSONL sequences/identities and steer chronology remain unchanged. A regression checks input is physically before tool_result, withheld in the active turn, and rendered after tool_result when the next turn admits it.

Evidence: Worker 67 passed, 1 ignored (`queued-replay-worker-regression-2.log`). DeepSeek passed (`kernel-queued-validation-live-3/matrix.json`): input 13, first settlement 41, second opening 42, second settlement 49. OpenAI passed (`kernel-queued-validation-openai-1/matrix.json`). All paths are under `/tmp/tekes-turn-live-audit`. Initial failing runs remain retained at `kernel-queued-validation-live-1` and `-2`. This closes successful model-level queue progression in the harness; real independent worker/validator processes and connected-client transport are not yet covered by this case.

### Preserve DeepSeek request-error envelopes

Fixed the diagnostic masking found by queued_validation: DeepSeek Responses/Chat now recognize a non-null top-level error envelope before applying success-envelope requirements (status/output/choices). Request rejection remains ProviderError, carries its original message/code, and cannot be final_answer. Non-success HTTP normalization redacts any credential echo before retaining the normalized terminal; optional raw capture remains withheld when such an echo is detected.

Evidence: `/tmp/tekes-turn-live-audit/provider-error-envelope-redaction-1.log` reports 44 provider tests passed, including a real local HTTP 400 response matching the observed `No tool output found` envelope, Chat error normalization, and credential-redaction/capture assertions. `/tmp/tekes-turn-live-audit/worker-request-error-regression-1.log` reports 68 worker tests passed, 1 ignored. The worker regression checks provider_terminal detail preserves the actual rejection, yields an error settlement disposition with no retry, and emits neither output nor validation candidate state. This resolves the previously documented `terminal lacks status` masking for top-level DeepSeek error envelopes; broader live parity remains open.

### Exact legacy Anthropic gateway schema live gate ported

Ported `gateway_accepts_canonical_nonstrict_tool_schema` to `crates/provider/tests/live_schema_probe.rs`, driven by `scripts/run-live-anthropic-schema.py`. The route and enabled SKU are resolved/checked against local provider configuration; credentials are passed only in environment. The probe reproduces the original claude-opus-5 body, including nullable integer, minimum/maximum, pattern, strict omission, max_tokens 16 and disabled thinking. HTTP 200 for the canonical case is mandatory; strict=true is only an evidence comparison, exactly as in the old test.

The initial Python urllib probe received Cloudflare 403/1010 before a schema verdict (`anthropic-exact-schema-live-1`). Reimplemented transport with the repository's reqwest client. The exact same canonical body SHA256 `d0b97904d585fe912f4c6d5fd2ee5f9ff2f3efb3f0e6fbcc0ab1ac6ab79183ec` then returned HTTP 200 with `ok`; strict=true returned HTTP 400 explicitly rejecting integer minimum/maximum. Retained receipt: `/tmp/tekes-turn-live-audit/anthropic-exact-schema-live-2/receipt.json`. This completes that particular legacy gate, not other models/routes or the still-failing Fable repeated-context scenario. Inventory row is now ported/passed.

### Legacy Chat Completions layout probes ported

Added `live_chat_layout.rs` plus `run-live-chat-layout.py`, reproducing all three raw layouts from ChatCompletionsLayoutLiveTests: system/user/frames, system/frames/user, and system/base-user/frames/user. The exact dummy call/result pair, tool schema, max_tokens 32 and stream=false are preserved. Baseline requires HTTP 200; candidate layouts retain the original 200-or-400 verdict gate. This is protocol-acceptance evidence only and enables no production cache/reordering flag.

Executed nine requests across original default routes/models: DeepSeek deepseek-chat and GLM glm-4.6 returned HTTP 200 for all layouts; Kimi moonshot-v1-8k returned 404 `Not found the model ... or Permission denied` for all layouts. The legacy test supports a model override, so reran the same Moonshot .cn route with configured kimi-k3; all three returned 200. Evidence: `/tmp/tekes-turn-live-audit/legacy-chat-layout-live-1` and `/tmp/tekes-turn-live-audit/legacy-chat-layout-kimi-current-1`. Inventory distinguishes current-model passage from original-model unavailability. The separate five-trial F2 comprehension/causality measurement is still unported, and these acceptance results must not be substituted for it.

### F2 causality measurement and first-root admission regression

The F2 wire measurement now preserves the legacy DeepSeek route/model, one sealed
check-result frame, five trials per layout, and 64-token budget. Both arms measured
5/5 correct in `/tmp/tekes-turn-live-audit/f2-causality-live-2/deepseek/receipt.json`.
This is a raw-wire port, not execution of the old Swift builder, and its accuracy
is not a production-reordering approval gate. The initial compile failure remains
in `f2-causality-live-1`; it issued no provider request.

A new real-process test starts ProductionProcessHost and the actual worker/helper,
requests four sequential context_get calls, queues literal `重试` during the first
attempt, and requires two writer-owned settlements with exact promoted answers.
Its first executable run (`process-context-queue-2`) exposed a root admission
regression: `open_ready_turn` required `terminal_tail`, which is false before the
first root turn. The ledger recorded repeated run_start without any turn_open or
provider attempt. Root admission now also permits latest_turn=None with pending
inputs. Existing turns still require settlement before queued input admission.
The first-root regression and held-validation queue regression both pass; the
real-process rerun is still pending at this entry.

The first-root fix passes the full worker unit suite: 69 passed, 1 explicitly
ignored live test (`worker-first-root-admission-1.log`). Process reruns 3 and 4
failed in the new harness's input submission: wrong origin operation, then direct
ledger delivery while the worker owned the lock. The harness now uses
SessionDeliveryAuthority::prompt, the production forwarding path. Run 5 completed
four context_get calls and root settlement at seq40, but the harness omitted the
periodic sweep that daemon.rs starts in production. The next run includes that
scheduler; these fixture failures are retained and do not count as passes.

`process-context-queue-6` now passes with frozen real worker/helper binaries and
production periodic scheduling. Four context_get calls at seq10/17/24/31 each
received successful results before the next response. Literal `重试` was durably
queued at seq8; the first root turn settled at seq41, the second opened at seq44
with trigger inputs=[8], and settled at seq52. Both promoted the exact explicit
final answer and recorded validation.not_required. Evidence includes test.log,
binaries.json, runtime/receipt.json, and the retained runtime ledger. This closes
the real-process repeated-call/queue gate, not the original external stdio client
notification assertions or artifact-validator process gate.

### Independent validator process gates

`process-validator-pass-1` passed: a pre-existing frozen candidate is resumed by
a real root worker, which launches a distinct real validator worker using the
configured DeepSeek provider. The child reads proof.txt, calls verify with the
exact path/hash and pass verdict, and settles. Root seq16 promotes original
output seq8 only after child_result and validation.decision. Root generation is
seeded in this recovery test; it is not evidence of a live initial mutation.

The corresponding repair gate is being exercised with a WRONG frozen draft.
Runs 1-4 are retained: (1) the fixture's sealed adapter mismatched DeepSeek during
repair replay; (2) approval delivery raced worker exit and returned Broken pipe;
(3) nested runtime/workspace paths were correctly rejected by safepoint; (4) the
model recovered from invalid patch formats and successfully repaired the file,
but later attempted a scratch file that the original narrow harness rejected.
The current harness uses separate storage/workspace directories, exact dialect
sealed data, grants only bounded disposable-workspace mutations, retries approval
with the same origin, and exposes verify only to the validator role. The external
approval delivery exit race still needs its own production/client gate; harness
retry is not proof that a single client submission recovers transparently.

`process-validator-repair-5` passed with the actual worker/helper binaries and
configured DeepSeek model. First validator verify(fail) is followed by root
validation.feedback seq16. Root apply_patch seq28 succeeds, read confirms the
correct content, and explicit candidate output seq44 is held for the second
validator. Its verify(pass) precedes root settlement seq51, which promotes seq44
and references candidate seq45/decision seq50. There is one root turn_open and one
root settle; the repair stays in the original session/turn. The complete child
ledgers retain both exact covered sets and verdicts. Receipt/test.log/binaries.json
are retained with the artifact. This adds real child-process repair evidence;
initial candidate creation is still seeded, cache attribution/external client
transport and one-shot approval exit-race recovery remain separate open gates.

The final shared harness also passed the independent pass-only scenario in
`process-validator-pass-2`; verify is supplied by validator role activation rather
than being exposed to the root worker. Supervisor offline suite remains 80 passed,
4 explicitly ignored live/process tests (`supervisor-validator-process-ports-1.log`).
The ignored gates executed above are accounted for separately; the 99-entry
inventory still proves discovery completeness only, not all-test completion.

### Approval handoff at worker exit

ProductionProcessHost::deliver_if_live previously returned Broken pipe when the
approval holder closed stdin before the process reader noticed its exit. It now
checks actual child exit after a failed write/receipt, bounded to one second.
Only confirmed process exit permits the existing locked Respond author fallback;
a transport error, alive flag, or timeout alone does not. The author retains the
exact origin, so an answer already committed before a lost receipt deduplicates.

The repair harness now invokes ProductionRespondAuthority::author once per
approval instead of implementing its own retry/append loop. It then separately
replays the exact authorization and requires the same semantic sequence.
`process-approval-handoff-2` passed: two approvals were LockedAppend at seq32/42,
each appears exactly once, and root completed the real validator repair chain at
seq61. The runner now freezes and hashes the supervisor test binary as well as
worker/helper; these hashes and the delivery methods are retained in evidence.
External HTTP/client authorization transport remains outside this gate.

Deterministic tests cover a closed stdin while the process is still alive (the
approval author waits for confirmed exit then returns the fallback path), and a
still-running child (no fallback). The former is
approval_closed_stdin_waits_for_actual_exit_before_fallback; it explicitly asserts
the live handle before delivery and actual child exit after fallback. This avoids
relying solely on probabilistic live-model timing to exercise the race fix.

### Legacy text smoke parity

Read runSimpleTextSmoke and each provider configuration in the pinned Runtime
LiveTests.swift. Its requirement is a real orchestration final containing
LIVE_OK_DONE; it allows environment model/base-URL overrides and an explicit
DeepSeek Responses API-style override. Added legacy_text to the kernel runner
with the exact original prompt/marker. `legacy-text-smokes-1` passes all six current
configured routes: Anthropic Cloudflare claude-fable-5, OpenAI Cloudflare
openai/gpt-5.6-luna, DeepSeek Responses deepseek-v4-flash, GLM glm-5.2, Google
gemini-3.6-flash and Kimi kimi-k3. Every result has explicit final_answer=true,
writer validation.not_required, one provider attempt and exact LIVE_OK_DONE.
Endpoint events and full request/ledger evidence remain beside each result.
These cover text semantics only; reasoning signatures/tool choice/schema/vision
checks remain distinct. They do not imply that earlier Anthropic tool refusals
are fixed.

Direct auth is audited separately. The OpenAI direct smoke issued a real request
with the existing proved gpt-5 profile and failed HTTP401 invalid_api_key in
`legacy-text-openai-direct-1`; this is an authentication blocker, not a passed
smoke. Direct Anthropic has neither anthropicKey in the available test key source
nor ANTHROPIC_API_KEY/TEKES_ANTHROPIC_API_KEY/TEKES_KERNEL_LIVE_KEY in the environment,
so no direct request was sent. Existing Cloudflare credentials do not establish
upstream direct authentication. All eight parity rows now record these exact
scopes and outcomes rather than remaining unaudited or borrowing another route.

The subsequent MCP HTTP audit found an unported behavior, not an equivalent green
test: old autoClientChoosesModernAndReservedHeadersCannotBeOverridden selects
modern via HTTP discovery, while current McpClient::connect_once sends every Auto
transport directly to legacy initialize. The old autoFallbackRequiresProtocolEvidence
accepts JSON-RPC method-not-found or unsupported-version explicitly listing legacy,
and rejects generic HTTP failures/timeouts or modern-only supported versions.
Those two rows are now audited_missing rather than incorrectly borrowing the
explicit Modern public service test or authorization/session transport tests.
This is the next concrete protocol parity implementation gap.

User route clarification: both OpenAI and Anthropic must use Cloudflare. Direct
OpenAI/Anthropic authentication is no longer an outstanding gate for this task.
The two historical direct text rows now point to the successful CF results under
an explicit user-authorized route substitution; historical direct 401/missing-key
evidence remains intact and is not represented as direct success. Apply this
route preference to remaining OpenAI/Anthropic live tests as well.

HTTP Auto now probes modern; stdio Auto retains legacy initialization. Remote
JSON-RPC error data survives normalization so structured protocol-version evidence
can be inspected. Modern pin never downgrades, and bare HTTP errors/timeouts do
not become legacy evidence. Mcp-Param-* joins the reserved configured and dynamic
auth-header set. The local socket matrix and unit predicate pass (mcp-http-auto-2,
mcp-http-auto-suite-2). Public Cloudflare Auto discovery/list/call passes;
DeepWiki Auto returns HTTP400/-32600 with only textual supported versions, so it
fails without downgrade. Explicit Legacy still passes separately. Both results
are retained. Broader legacy helper paths (no-mutual-version discovery and parsed
non-auth HTTP RPC errors) remain to port; declared predicate tests alone do not
prove those paths. Supervisor compatibility suite passed in supervisor-mcp-auto-1.

### Complete structured MCP negotiation evidence

Added a typed no-mutual-protocol result retaining discovery's supported versions.
Auto recognizes explicit legacy-era support, matching the old connector helper;
incomplete discovery is rejected before its version list can authorize fallback.
Non-auth 4xx errors may retain a valid matching JSON-RPC error for that decision.
The error reader is bounded to 16 KiB, rejects unmatched ids/versions, ignores
success-shaped bodies on error status, and never uses 401/403/5xx bodies as era
proof. Broken/truncated body streams discard partial evidence. Bare HTTP status
and textual version lists still cannot downgrade (including DeepWiki's observed
400/-32600 response).

The expanded real-local-socket suite passes modern pin, legacy-only complete
discovery, method missing on HTTP400/404/405, structured unsupported version on
HTTP400, and negative authentication/server/textual cases. Evidence:
mcp-http-structured-errors-2.log, mcp-structured-evidence-suite-1.log. Public
Cloudflare Auto discovery/list/call passes again in
mcp-structured-evidence-cloudflare-1.log. The full old HTTP suite still has separate
session-expiry, DELETE, concurrent-cancellation and lifecycle tests outstanding.

### Legacy HTTP session expiry and close parity

The old HttpTransportTests session-expiry test exposed missing server cleanup: Kernel close only discarded its local session id. HTTP close now resolves current authorization, consumes the session once, and attempts bounded best-effort DELETE. Authorization identity rotation suppresses deletion of the prior principal's session. A real loopback server verifies two initializations around a 404 expiry, successful retried tools/list, and exactly one DELETE even after two close calls; a 405 DELETE response does not block local teardown. Current Kernel legacy version is 2025-11-25, so this is semantic parity rather than an assertion of old 2025-06-18 negotiation support.

Validation: `/tmp/tekes-turn-live-audit/mcp-session-close-suite-2.log` passed all 26 non-ignored MCP tests (5 library, 4 HTTP discovery/session, 17 slice gates); 5 public live cases remain ignored in this command. Initial test run `mcp-session-close-1.log` retained: fixture incorrectly hardcoded the old protocol version and was corrected to assert Kernel's actual negotiated version. This does not close outstanding modern concurrency, cancellation, handshake cleanup, or overall 99-test migration work.

### Handshake failure cleanup parity

Old `connectorHandshakeThrowClosesTransportExactlyOnce` exposed missing cleanup in Kernel's early return after handshake reconnect/retry failure, plus decoded discovery protocol validation errors. Client now shares a close-once guard per transport generation across failure and explicit close paths. All terminal connect errors release their transport; read-triggered reconnect handshake errors also release it. Beginning a reconnect resets the guard because even a failed reconnect may acquire resources.

`/tmp/tekes-turn-live-audit/mcp-handshake-cleanup-1.log`: 27 non-ignored MCP tests passed, including counting-transport assertions for remote failure, wire protocol failure, decoded discovery failure, retry failure, and reconnect failure. These tests explicitly check that caller cleanup after failed connect does not double-close. They do not prove cancellation of a dropped handshake future; the old cancellation test remains outstanding.

Supervisor integration validation: `/tmp/tekes-turn-live-audit/supervisor-mcp-handshake-cleanup-2.log` passed 82 tests with 4 ignored process-live tests. Attempt `-1` only failed package selection (`supervisor` rather than actual `tekes-supervisor`), and is not test execution evidence.

### Cooperative handshake cancellation

`McpClient::connect_cancellable` now races the handshake (including retry/reconnect) against an explicit cancellation token, drops the in-flight operation, then awaits close before returning `Cancelled`. Ordinary `connect` delegates to this entry with an uncancelled token. Added counting-transport tests cancel at initialize, initialized notification and reconnect, plus a pre-cancelled case proving no request is sent. Each checks close has already occurred at return and later explicit close cannot repeat it.

`/tmp/tekes-turn-live-audit/mcp-handshake-cancellation-1.log`: all 29 non-ignored MCP tests passed; five public tests were ignored. This is partial parity: unlike Swift task cancellation, aborting/dropping a Rust future does not execute async cleanup. Production `prepare_server` is still synchronous and supplies no handshake cancellation token. Wiring cancellation requires preserving the operation's token generation across `ensure_route_peer` and the eventual tool call; fetching a newly rotated token after reconnect would incorrectly admit cancelled work. Keep that production integration gap explicit.

Integration regression `/tmp/tekes-turn-live-audit/supervisor-mcp-handshake-cancellation-1.log`: 82 passed, 4 ignored. This verifies existing supervisor behavior after the client API change, not production handshake cancellation acceptance.

### Production route recovery cancellation

Dynamic execute/reconcile now capture the operation token before `ensure_route_peer` and pass the same token through cancellable preparation and final broker dispatch. This prevents rotation in `cancel_inflight` from turning the old operation into a fresh uncancelled call. Preparation checks cancellation before spawn, during connect/reconnect, and while listing tools; failure closes the peer. Administrative preparation retains an uncancelled default token because it has no operation cancellation owner.

Real stdio test `preparation_cancellation_reaps_real_stdio_during_handshake_and_catalog` runs Python child servers that stall at initialize or tools/list, cancels only after the child publishes a complete PID, requires `Cancelled` within five seconds, and checks `kill(pid,0)` returns ESRCH after preparation returns. `/tmp/tekes-turn-live-audit/supervisor-real-handshake-cancel-3.log`: all 83 supervisor tests passed, 4 ignored, including prior cancellation-generation regression. Attempt 1 was missing Duration import; attempt 2 exposed a test synchronization race (file created before PID write), fixed by waiting for parseable PID. Both retained. Explicit HTTP socket cancellation and Rust future-drop cleanup remain unproven, so this does not assert complete task-abort parity with Swift.

### Real HTTP handshake cancellation acceptance

`http_handshake_cancellation_closes_socket_and_deletes_allocated_session_once` uses a real loopback TCP server. Initialize allocates `cancel-session`; the server receives initialized but deliberately never responds. Only then does it trigger the cancellation token. The test requires Cancelled within three seconds, matching session DELETE, EOF on the stalled notification socket, and no extra connection after repeated explicit client close. Server completion is separately bounded. This supplies the previously missing real HTTP evidence for cooperative cancellation, not arbitrary future-drop cleanup.

`/tmp/tekes-turn-live-audit/mcp-http-handshake-cancel-1.log`: all 30 non-ignored MCP tests passed (5 library, 3 handshake lifecycle, 5 HTTP, 17 slice gates); five external-service tests were ignored and are not claimed here. Production reconnect/dispatch token wiring remains covered by the previous 83-test supervisor run. Overall legacy inventory and session/validation acceptance work remain open.

### Legacy local HTTP tool round trip

Ported old `connectsAndCallsToolOverLocalHttp` as `legacy_http_connects_lists_and_calls_tool_over_real_socket`. The real TCP fixture returns SSE-framed JSON-RPC for initialize, tools/list and tools/call. It checks initialized/session propagation, tool name and Unicode arguments, exact text result/isError, negotiated metadata and DELETE on close. The whole interaction is bounded to five seconds. Current supported legacy protocol is 2025-11-25; older 2025-06-18 negotiation is not claimed.

`/tmp/tekes-turn-live-audit/mcp-http-roundtrip-1.log`: six HTTP integration tests passed, no ignored tests. This was a test-only change; no claim of rerunning unrelated public services. Separately inspected old terminal-provider-400 workflow: its acceptance includes spawned appserver/client notifications and automatic queued-input release, so existing provider-unit or in-process worker evidence cannot yet close that pending row.

### Permanent provider 400 and queued-body release

Added worker regression `permanent_400_releases_queued_retry_body_after_error_settlement`: queue literal 重试 while turn 2 is open; verify it cannot open early; return actual HTTP 400 with the old test's model-not-available body; require one attempt, durable error body and error settlement; then ordinary admission opens turn 3 with exactly the queued input, after settlement, and cannot open it twice. `/tmp/tekes-turn-live-audit/worker-400-queue-suite-3.log`: 70 passed, 1 ignored.

Retained attempts: `worker-400-queue-1.log` used the wrong lease attempt seq (initial provider epoch precedes attempt); `worker-400-queue-suite-2.log` expected `http_400` in detail, but current normalized provider-terminal error records preserve model error text without HTTP status. The final assertion requires the actual server message. Loss of numeric terminal HTTP status remains an observability gap. This gate does not yet prove spawned client notification delivery, automatic supervisor wakeup, or the second recovered response from the old end-to-end workflow; the inventory row remains explicitly partial.

### HTTP status ownership and finality protection

ProviderTerminal now carries optional transport-owned `http_status`; protocol-only normalization leaves it absent. Non-success HTTP normalized terminals preserve status and cannot report successful finish (context-overflow and content-filter semantics remain intact). `is_final_answer` additionally refuses non-2xx transport evidence. Worker error detail prefixes the numeric status, preserving the server message; malformed non-success bodies also retain status in their diagnostic. Added a real HTTP 400 success-shaped final-answer regression and retained the credential-redaction assertions. Strengthened the queued-400 regression to require both http_400 and model-not-available text.

Expanded test scope exposed stale process integration expectations of output -> settle directly. Updated `slice7_worker_provider_loop_commits_outcome_before_release` to assert output -> validation.candidate -> validation.decision -> settle, not_required validation, exact promoted output seq, and final appended doorbell. This matches the user's session-final then validation requirement.

`/tmp/tekes-turn-live-audit/provider-http-status-preservation-3.log`: complete provider and worker `--tests` run passed, including 70 worker unit tests and 12 worker process integration tests. Ignored live probes were not executed by this command. Attempts 1 and 2 retained the stale process-test failure (expected seq10 versus actual validated settlement seq12); final run checks semantic records as well as seq. The previously documented loss of HTTP 400 status is now fixed. Overall external client workflow/live-test migration remains incomplete.

### Current six-provider real continuation revalidation

Ran `python3 scripts/run-live-kernel.py --providers all --scenarios tool_continuation --output /tmp/tekes-turn-live-audit/tool-continuation-current-1` against current frozen worker test executable SHA256 5b729c99ea62fa6169b2213acb04c8e88d57d43a60427096169a370f228d9081. Matrix has six passed cells: CF Anthropic claude-fable-5, CF OpenAI openai/gpt-5.6-luna, DeepSeek deepseek-v4-flash, GLM glm-5.2, Google gemini-3.6-flash, Kimi kimi-k3. Every result records exactly two attempts, think tool, answer KERNEL_TOOL_OK, explicit final_answer and validation not_required with exact promoted output sequence. Runner exited zero; requests/raw responses/ledgers/projection evidence and result.json retained per cell. No direct OpenAI/Anthropic credentials were used.

This is current real provider/worker loop evidence, not a claim of completion for all legacy declarations. Source audit confirms old thinkContinuation requires three distinct ordered STEP markers and a train meeting-time answer; old DeepSeek bounded continuation requires lookup_file arguments and specific framing checks. Neither old row is closed by this simpler continuation matrix. At run start the 99-row inventory still had 67 not_run entries; broader migration remains outstanding.

### Legacy three-step think scenario execution

Added `legacy_think_three` to the real worker matrix runner, preserving the old train problem and ordered STEP_1_CLOSING_SPEED / STEP_2_MEETING_TIME / STEP_3_DISTANCE_CHECK prefixes. Harness requires exactly three think calls across distinct responses, matching successful results before each next call and before promoted final, and numeric final 3. Every prefix projection must still withhold turn/end until validation settlement.

`/tmp/tekes-turn-live-audit/legacy-think-three-1/matrix.json`: five passed (CF OpenAI, DeepSeek, GLM, Google, Kimi); CF Anthropic claude-fable-5 failed with durable provider_terminal/content_filter then error settlement. Runner exits 1, correctly preserving the failed matrix. Successful cells use four model attempts and validation not_required. Original test's ContinuationThink wrapper dynamically forces the next marker via required tool choice, switching to none after three matching results; that wrapper has not been ported, so the row is partial despite five model passes. The original reasoning-mode intent also needs request-body verification. No failure is relabeled as success or replaced by another route.

### Explicit tool-choice preparation seam

Source audit found no per-response tool-choice input in Kernel prepare; adding prompt instructions alone cannot reproduce old ContinuationThink's required/none scheduling. Added typed `ToolChoice` and `prepare_with_tool_choice`, with existing prepare preserving its exact default request. Explicit choices serialize before request digest calculation: Responses/Chat string auto|required|none, Anthropic type auto|any|none, Google Generation functionCallingConfig AUTO|ANY|NONE. Google Interactions explicit choice is rejected pending proof. Required with an empty catalog is rejected.

`/tmp/tekes-turn-live-audit/provider-tool-choice-1.log`: all provider tests passed. New fixture-wide test verifies default identity unchanged, per-dialect shapes and distinct digests for all three choices, plus empty-catalog rejection. This is encoder coverage, not proof each live model supports required with its configured reasoning mode. Worker test-only marker scheduling and corresponding live request-body assertions remain to be wired before the legacy three-step row can be closed.

### Legacy dynamic think-choice live wiring

Worker test builds now route the legacy three-step scenario through `prepare_live_choice`. It derives the accepted prefix count from successful durable think call/results, injects the old next-marker instruction, requires tools until three accepted prefixes, then disables tools. Production builds continue to call ordinary prepare directly. Live assertions read all four actual request bodies and require required/any/ANY followed by none/NONE plus the proper marker; final numeric and causal result ordering gates remain.

`/tmp/tekes-turn-live-audit/worker-tool-choice-regression-1.log`: 70 worker tests passed, one live test ignored. Live matrix `/tmp/tekes-turn-live-audit/legacy-think-choice-1` explicitly retains CF Anthropic content_filter and DeepSeek HTTP 400 `Thinking mode does not support this tool_choice`. Do not drop thinking or substitute the earlier prompt-only result to claim required-tool parity. The test uses configured reasoning settings; full old high-reasoning intent still needs explicit per-provider equivalence review.

### Dynamic choice matrix completion and Kimi cause

The previously live handle has completed with exit 1; no process remains pending from this matrix. `/tmp/tekes-turn-live-audit/legacy-think-choice-1/matrix.json` is final: CF OpenAI, GLM, Google passed; CF Anthropic, DeepSeek, Kimi failed. Added `causal-audit.json` capturing per-attempt request controls, errors, first think argument, call count and final settlement.

Kimi request 6 actually contains tool_choice required and the STEP_1 next-marker instruction (reasoning_effort max). Its first tool call at seq9 instead starts STEP_2_MEETING_TIME. The legacy contiguous-prefix scheduler therefore continues requesting STEP_1; subsequent correct STEP_1 calls cannot erase the earlier out-of-order first result, matching the pinned Swift loop's break-on-mismatch behavior. Eleven think calls occur before provider response stream failure seq86 and interrupted settlement seq87 at the 180-second runtime limit; live test fails in 180.12 seconds. This is model ordering failure plus the old scheduler's strict prefix semantics, not a successful final or a still-running job. Do not relax ordering or use prior prompt-only passes to replace it. Production think auto policy is unchanged. Other applicable legacy rows remain actionable; these provider-specific failures do not block the overall goal.

### Expanded inventory discovery audit

Rechecked seemingly absent simpleWriteFile/simpleWriteRepair: both were already registered with non-not_run states, so no duplicate rows were added. Expanded discovery beyond filename Live/Smoke/Scored to directory path and annotated suites carrying live configuration markers. Reviewed two newly surfaced files: CIQualityGateTests asserts deterministic CI/script contracts; ModelMigrationTests asserts schema/backend documentation contracts. They join explicit non-provider suite exclusions rather than being silently treated as executed live coverage.

Pinned source scan `/tmp/tekes-turn-live-audit/live-inventory-expanded-discovery-1.json`: 18 suites, 99 declarations, zero missing, four explicit reviewed exclusions. New hermetic test `scripts/test-live-test-inventory.py` creates a temporary committed Git repo, verifies directory-only and body-marker candidates produce a failing omission gate, verifies ordinary unit tests do not, and verifies --update registers only not_run entries. `/tmp/tekes-turn-live-audit/live-inventory-discovery-tests-1.log` passed. Inventory discovery is stronger; execution/semantic parity is still evaluated separately and remains incomplete.

### Legacy nested write smoke migration in progress

Added legacy_write runner scenario for docs/live-smoke.md containing exactly LIVE_OK newline, final LIVE_OK_DONE. Harness requires actual apply_patch/read success, saved bytes, one validator spawn, one root turn, reported usage and passing validation. Started real DeepSeek run `/tmp/tekes-turn-live-audit/legacy-write-deepseek-1` (still running at this entry).

Live evidence revealed a concrete semantic gap: correctly formed create_file diff +LIVE_OK fails NotFound because helper atomic_replace opens but does not create missing parent directories. Old WorkflowProjectSupport.swift:349 resolves target with createParentDirectory true. Do not prepopulate the target file to bypass this. Initial model diff-format mistakes are also retained. Artifact version changes from 0 to 1 after reserve before failed write; source inspection confirms this is a durable reservation, not evidence of successful write, so rollback must not be introduced casually. Subsequent missing-file/version conflicts obstruct retry. Next work: reconcile nested-directory creation and reservation failure semantics with helper confinement, then rerun the same acceptance gate.

Raw DeepSeek Responses usage reports input_tokens and input_tokens_details.cached_tokens, rather than the old Chat-specific hit/miss pair. Per-request cache equivalence needs explicit accounting; current usage presence assertion alone does not close the old DeepSeek cache attribution gate.

### Confined nested parent creation

Fixed helper create_file to create missing parent directories using mkdirat on owned directory descriptors, reopening each component with O_DIRECTORY/O_NOFOLLOW. Existing-file no-replace behavior remains enforced; update paths still require existing parents. Added tests for nested creation, refused overwrite, symlink traversal and dot-dot escape. `/tmp/tekes-turn-live-audit/helper-nested-create-suite-1.log`: tools --tests passed, including helper process/confinement suites and the two new tests.

Initial legacy-write-deepseek-1 ended failed; its original missing-directory evidence is retained. With the fix, legacy-write-deepseek-2 wrote the file and completed a real validator/pass settlement, but the harness failed because it requested/expected an extra newline while file bytes were LIVE_OK. The validator did not detect that mismatch: retained as a validation precision gap. Source audit LiveTests.swift:1361,1797 confirms original expected exact LIVE_OK without newline; the migrated prompt/assertion have now been corrected to original semantics, with a fresh legacy-write-deepseek-3 run (not relabeling -2). Reservation versions are not rolled back; they represent reserved generations, not proof of write completion.

legacy-write-deepseek-3 is terminal, exit1: file bytes match original LIVE_OK and validation passed, but final output includes explanatory prose before LIVE_OK_DONE. Exact-final assertion correctly failed; not changed to substring matching. The per-provider legacy row now records this failure and outstanding cache equivalence. No live process from these runs remains active.

### Candidate-answer validation and rejudgment fix

Live exact-final failures prompted inspection of ensure_validator: its seed already contains root request, candidate text and frozen artifacts. Strengthened judge instructions to check every requirement, exact candidate answer, and exact byte/newline constraints; correct files cannot excuse an incorrect final answer.

New real `validation_repair_final` scenario creates a correct proof.txt but submits WRONG_FINAL, then repairs only the answer after feedback. Initial `/tmp/tekes-turn-live-audit/validator-exact-final-1` showed judge correctly rejecting prose/WRONG_FINAL, but the worker reused snapshot-only coverage and settled the repaired answer inconclusive without a second judge. Fixed ValidationBinding::covered_by to include output_seq: prior verdicts cover only the exact candidate, not new answers with identical files. Same-candidate replay remains idempotent; negative-round cap remains bounded.

`/tmp/tekes-turn-live-audit/validator-exact-final-2`: passed real DeepSeek run in 27.96s, exact final KERNEL_FILE_OK, two candidate snapshots equal, feedback between first wrong answer and repaired output, two validators, final pass with promoted output seq46/decision52. Test intentionally accepts an incorrect first answer containing WRONG_FINAL (including additional incorrect prose); the final answer must be exact. This is genuine file-preserving answer repair rather than forcing artifact mutation to trigger revalidation.

`/tmp/tekes-turn-live-audit/validator-candidate-coverage-2.log`: engine and worker lib/bin tests passed (70 worker tests, one ignored). Updated stale regression that previously expected unchanged-file standoff: it now requires a second validator for a new output, refuses settlement before that verdict, and verifies a second fail reaches ordinal2/inconclusive. Initial coverage run retained the old assertion failure. This fixes candidate identity reuse and supplies a live exact-answer repair gate; it does not prove all validators always judge byte constraints correctly or close the remaining legacy migration work.

### Nested write acceptance and cache attribution separation

After candidate-answer validation fixes, `/tmp/tekes-turn-live-audit/legacy-write-deepseek-4` exited0: exact file LIVE_OK, exact final LIVE_OK_DONE, one root turn, one validator, pass settlement with promoted output34/decision40, 19.98 seconds. This was a meaningful rerun following validator changes, not relabeling prior failed outputs.

Audited six raw response usage samples (worker and validator) in the cell's cache-audit.json. Every sample reports usage; none provides the old explicit prompt_cache_hit_tokens/prompt_cache_miss_tokens pair. Responses instead reports input_tokens_details.cached_tokens. The old mapping only preserves actually supplied provider-specific fields, so the full old explicit-cache gate remains unmet; no inferred subtraction is presented as reported miss tokens.

Source audit also found Kernel omitted prompt_cache_hit_tokens when it is supplied. Added that alias ahead of nested cached tokens, matching legacy priority (including explicit zero). `/tmp/tekes-turn-live-audit/provider-cache-hit-alias-1.log`: provider tests passed, including top-level precedence and missing-value/no-invention regressions. The live -4 binary predates this alias-only change; it proved the nested Responses path and is not claimed as live proof of a top-level field the service did not return.

### Parent-creation boundary closeout

Reviewed helper root binding and whole-path normalization after nested create support. Added targeted regression ensuring parent creation is exclusive to CreateMode::New (Replace/Upsert still fail without parents), malformed later path components produce no partial directories, and an intermediate regular file is never overwritten. `/tmp/tekes-turn-live-audit/helper-parent-boundaries-1.log`: three helper unit tests passed. Together with prior tools integration and live nested-write evidence, this covers the concrete behavior changed by mkdirat. Updated builtin-tools-v1.md to describe directory creation, no-follow reopening, existing-target rejection and reservation-versus-commit semantics. No additional general testing gate or expanded filesystem authority was introduced.

### Real worker completed-commentary continuation gate

Extended existing spawned worker/credential-channel/loopback HTTP integration fixture with a two-response variant. First response is provider-completed but contains an assistant message with phase commentary and text Still working. Test drives the real worker's second lease request, reads the live ledger at that boundary and asserts no settlement has occurred. The first output must have final_answer false. Second response supplies normal final text; exact event sequence requires two attempts/two usage pairs and only then candidate, decision and settle. Final doorbell covers the actual settlement and promotes the second output.

`/tmp/tekes-turn-live-audit/worker-process-commentary-1.log`: 13 shell process integration tests passed. New test `real_worker_continues_after_completed_commentary_before_final_validation` exercises production process control rather than an in-process loop. This provides direct process evidence for the original completed-response/non-final-message bug. It does not by itself close external endpoint client transport or all legacy live suites.

### Worker event-time progression

Fixed production worker events that reused the launch timestamp throughout the run. Options now retains a monotonic Instant origin and derives each event timestamp as the supplied launch RFC3339 anchor plus elapsed monotonic time. This preserves launch attribution while reflecting elapsed provider/tool/validation time and avoiding wall-clock jumps. Validation bridge writes use the same clock. Test builds keep their deterministic fixture timestamps; spawned production-binary integration tests exercise the real clock.

The real completed-commentary integration server delays each response; its process test now requires the two output timestamps to advance while still asserting no early settlement and final validation ordering. `/tmp/tekes-turn-live-audit/worker-event-clock-2.log`: 70 worker unit tests passed (one ignored live test), 13 shell integration tests passed. Initial -1 compile failure from missing/overbroad constructor edits was corrected and retained. No historical ledger is rewritten; this fixes future event timing and does not establish the original user's exact elapsed runtime solely from its previously constant timestamps.

### Current independent-process repair and event-clock proof

`/tmp/tekes-turn-live-audit/process-repair-current-clock-1` completed exit0. Receipt records three production AllowedOnce approval deliveries; JSONL records two independent validators, feedback and repair in one root turn, then pass settlement seq71 promoting repaired output64 through candidate65/decision70. Child settlement timestamps progress from 05:20:57.950Z to 05:22:27.389Z; root settles at 05:22:27.418Z. Per-event extraction is retained in runtime/clock-audit.json. Frozen executables and SHA manifest accompany the run. Initial root candidate remains seeded; this does not establish a from-zero external client workflow.

Found a harness coverage gap during evidence review: it stopped the host immediately upon observing root settlement, before the asynchronous endpoint journal necessarily caught up. Added an explicit wait for production endpoint turn/end, exact semantic settle sequence, the promoted assistant message bound to the candidate output sequence, sessionFinal, and message-before-end ordering. No forced projection call is used. The previous run is not retroactively counted as proof of this new delivery gate.

### Production endpoint journal delivery after independent repair

`/tmp/tekes-turn-live-audit/process-repair-endpoint-delivery-1` passed with the new delivery gate. One AllowedOnce response was delivered to the live worker at semantic seq32 (exact duplicate remains idempotent). The root promotes repaired output46 at settle53. Endpoint assistant/message seq815 binds kernel output46 and sessionFinal true; step/end816 precedes the single turn/end817 bound to settle53 with validationOutcome pass and the exact promoted message ID. This confirms asynchronous production projection catches up before teardown, without manually invoking publish/replay. Initial candidate is still seeded and no external client process consumes this journal, so those original workflow boundaries remain open.

### Current workspace regression validation

`cargo test --workspace --locked` completed exit0 after the candidate-identity, helper confinement, event-clock and endpoint-delivery test changes. Full log: `/tmp/tekes-turn-live-audit/workspace-validation-current-1.log`; command receipt: adjacent .json. Default unit/integration/doc targets passed. Ignored live tests were not run by this command, and nested process harness summaries must not be counted as unique declarations. Separate live repair evidence remains above. `git diff --check`, the hermetic inventory discovery test, and pinned legacy scan passed; 99 registered declarations still include 65 not_run rows. Overall migration goal remains active and incomplete.

### Legacy Google nullable union declaration live port

Ported LiveTests.swift:702-751 from pinned TekesRuntime: identical ask_user_questions object with required question/options/progress and options type [array,null], AUTO selection, nonstream request and nonempty successful response. New provider test uses production resolve_profile/epoch_profile/prepare_with_tool_choice and HttpRuntime, not hand-authored HTTP JSON. Kernel uses parametersJsonSchema and preserves the union; old Swift used scalar parameters with nullable. The actual acceptance invariant is preserved without restoring obsolete encoding.

`/tmp/tekes-turn-live-audit/google-legacy-union-1` passed; tightened acceptance to explicitly exclude non-2xx tool-shaped errors and reran as `google-legacy-union-2`, also passed (Gemini 3.6 Flash, normal STOP response 42). Exact request/raw response/receipt/test log retained. A subsequent receipt-only change removes bulky Debug raw-byte duplication; no live behavior changed, and `google-schema-compile-3.log` confirms compilation. Both earlier receipts remain immutable evidence. Inventory row now passed_configured_model; 64 of 99 declarations remain not_run. This test does not cover the separate Google thinking/signature continuation gate.

### Google thinking continuation live port

Pinned LiveTests.swift:625-690 requires high thinking, required lookup for 21*2, nonempty thought signature, positive reasoning usage, then tool result 42 with preserved reasoning data and no further calls. Added google_thinking_tool_result_preserves_signature and runner --scenario thinking. It uses production preparation/HTTP normalization, asserts first response is non-final, preserves the complete sealed carrier exactly in second request, and requires final text42.

`/tmp/tekes-turn-live-audit/google-thinking-continuation-1` completed exit0, one test passed, two actual requests. Explicit high control is thinkingBudget4096 in both request bodies. Both wire bodies and responses plus receipt retained. This closes the live signature replay and response-continuation behavior on configured Gemini3.6Flash, but Kernel Usage lacks normalized reasoning tokens; positive raw thoughtsTokenCount is only partial parity with old usage.reasoningTokens. Inventory records partial_live_evidence rather than full pass, with that precise remaining gap. Now 63 of 99 declarations are not_run; additional partial/failure rows also remain unfinished.

### Reported reasoning usage normalization and persistence

Added optional reasoning_tokens to provider Usage with absent-field deserialization compatibility and omission when missing. Normalizes legacy reasoningTokens, Google thoughtsTokenCount, top-level reasoning_tokens, Responses output_tokens_details and Chat completion_tokens_details. Stream merge preserves prior reported values across partial updates, including explicit zero. Worker usage events now persist the figure, schema validates it under reported/unavailable rules, and endpoint maps it to reasoningTokens with existing safe-integer checks. It is an independent reported figure, not added to output totals or inferred from other counts. Updated event specification.

`reasoning-usage-path-1.log`: provider/schema/endpoint/worker regression command exited0. Added absence/zero/partial merge and endpoint safe-integer regressions passed separately in reasoning-usage-unit-2.log and reasoning-usage-endpoint-2.log. Actual Google run google-thinking-normalized-1 passed with an assertion that normalized reasoning_tokens equals the positive raw thoughtsTokenCount; signature replay and final42 assertions also pass. The legacy Google thinking row is now passed_configured_model. This is not a claim of installed Swift client UI presentation; live provider evidence and downstream code/test boundaries are recorded separately. Other 63 not_run rows and partial/failed rows remain outstanding.

### Anthropic CF thinking migration exposed generation-policy gap

Added real-route test/runner for old LiveTests.swift:500-602. First attempt wrongly assumed all Anthropic models use budgeted thinking; source review of Network.ModelAPI.Anthropic.ThinkingPolicy.swift shows Fable/Mythos must omit thinking and use output_config.effort, Opus5 uses adaptive. Corrected test to require those exact generation shapes and retained required first tool choice (only legacy budgeted models drop it), auto second tool choice and sealed replay. The configured CF Fable5 case fails before dispatch: epoch_profile rejects high reasoning with UnsupportedControl. Runs anthropic-cf-thinking-1 and -2 are terminal failures, not credential failures; no HTTP response was received. No defaults substituted to manufacture a pass.

Source trace: model_capability_for performs exact model-profile matching and yields no supported reasoning levels for this configured target; request.rs currently emits budget_tokens for native Anthropic when reasoning is present. Both capability and generation request encoding require repair before this old live test can pass. Inventory now records this actionable failure rather than not_run (62 rows still not_run). Final test edit restores required/auto semantics after -2; it has not been live-executed past the unchanged pre-dispatch failure.

### Native CF Anthropic generation controls and credential routing fixed

Enabled low/medium/high explicit reasoning for exact configured Fable5 and Opus5 profiles while retaining default=None; updated catalog integrity digest. Native Anthropic Fable5 now sends output_config.effort without thinking; Opus5 sends adaptive thinking plus effort. Other models retain existing policy, without speculative generation inference. Found native CF Anthropic reused direct x-api-key authentication fallback; exact cloudflare/cloudflare-native-anthropic route now uses cf-aig-authorization Bearer. Formal route proof_verified is not promoted by this policy change.

`anthropic-cf-thinking-generation-5` (Fable5) and `anthropic-cf-thinking-opus-2` both passed actual two-call tests with production encoding and HTTP transport. First response contains one lookup tool call; second returns completed with no calls after exact sealed carrier replay. Request assertions cover high effort, generation-specific thinking, required/auto choice and CF credential selection. Earlier capability, catalog hash and stale auth assertion failures are retained; they did not dispatch HTTP. Successful receipts contain an outdated scope description mentioning budgeted behavior; actual request assertions/bodies prove modern generation behavior, and the source description has been corrected without rewriting historical receipts.

Provider regression -3 revealed the old CF route fixture expected x-api-key; corrected that explicit expectation, preserving unverified route assertions. Provider regression -4 passed. Native direct-route fixtures remain covered. The legacy thinking inventory row is now passed_cloudflare; 62 not_run rows and other partial/failed rows remain.

### Production worker checks after CF route repair

`anthropic-worker-after-route-fix-1` is terminal with two failed cells. legacy_think_three gets content_filter on first response. legacy_write generates an artifact candidate but independently validated child request fails HTTP400: tools.2.custom.input_schema rejects top-level oneOf/allOf/anyOf. Actual verify schema has one top-level allOf wrapping if verdict=fail then failures.minItems1 else maxItems0. Root correctly settles inconclusive via validator_death, so live gate fails rather than claiming pass. Raw validator response retained in the cell. This exposes an actual provider-schema migration gap, not auth failure.

Ported old ContinuationThink's explicit high reasoning into the live worker session settings (production default untouched), including settings revision. `anthropic-think-explicit-high-1` also terminates content_filter. Its actual request proves output_config.effort high, absent thinking and tool_choice any; default-reasoning prior runs are not substituted as equivalent evidence. Other providers' high-mode reruns remain pending. Next repair is semantics-preserving lowering of the single conditional allOf used by verify, with local validation retained.

### Anthropic validator conditional schema compatibility restored

Production Anthropic tool rendering now hoists a sole allOf conjunct only when it contains exclusively an isolated if/then/else group and none of those root keys already exists. The if predicate and both branch constraints remain byte-value identical; object properties, required list and additionalProperties remain unchanged. Arbitrary compositions, duplicate conditionals, mixed property schemas and multiple branches are deliberately not flattened because their semantics may differ. Local argument validation keeps the original canonical schema.

`anthropic-conditional-provider-1.log`: full provider tests passed, including bounded-hoisting and unsafe-shape preservation regression. `anthropic-validator-conditional-1` passed actual CF Fable worker + validator run: exact docs/live-smoke.md LIVE_OK and final LIVE_OK_DONE, four root attempts, candidate32/output31, passing decision37, 34.29 seconds. Both validator requests carry root if/then/else and no allOf; the previous HTTP400 is resolved, and the validator actually completes its verify call. This live gate uses in-process child execution; independent-process and external-client scopes are not inferred. Full legacy migration remains incomplete.

### CF answer-only repair after conditional-schema fix

`anthropic-answer-repair-1` completed exit0 with actual CF Fable provider, root worker loop and two live validators (in-process child harness). First validator verifies fail: artifact correct but WRONG_FINAL answer violates exact final requirement; guidance explicitly requests unchanged file and corrected final. Root resumes in the same turn, submits KERNEL_FILE_OK, and second validator verifies pass. Harness checks both candidate snapshots equal, feedback between wrong and repaired answer, one root turn and one settlement. Final promoted output42, candidate43, decision48; 52.58 seconds and five root attempts. Additional repair-audit.json retains exact candidate/feedback/settle bindings.

This proves both branches of verify's conditional schema are accepted on CF, and unchanged files do not cause stale candidate-verdict reuse. It does not close original file-mutation repair/cache-attribution tests or independent-process external-client scopes. Pinned legacy inventory scan after Anthropic schema work remains 99 declarations with no missing discovery entries; all remaining incomplete execution rows stay open.

### Kimi exact-SKU schema warning live port and fix

Ported pinned LiveTests.swift:839-896: kimi-k3 on api.moonshot.cn, read tool with required path and nullable integer offset/minimum1, AUTO choice, nonstream prompt, nonempty response and absent msh-schema-warning header. New test uses production preparation/normalization and a real reqwest response to inspect the header. Initial kimi-schema-warning-1 got HTTP200 and an answer but failed correctly: minimum cannot accompany multiple types.

Compared old Network.ProviderToolContractCompiler.swift:590-663 and ported nullable structural constraints into a concrete-type anyOf branch plus null branch; enum and other sibling constraints remain outside. Existing anyOf collision returns a preparation error rather than overwriting it. Numeric/string/array/object structural keyword sets follow the pinned compiler. Canonical local validation is unchanged. kimi-schema-warning-2 passed HTTP200, nonempty result, warning absent. Full provider suite passed in kimi-nullable-provider-1.log; targeted null/enum/constraint-collision regression passed in kimi-nullable-constraints-2.log. Inventory row now passed_configured_model; 61 declarations remain not_run, plus partial and failed rows.

### DeepSeek Responses tool-choice parity correction

While auditing old bounded continuation, inspected Network.ModelAPI.DeepSeek.Response.swift:154-191: responsesWireProfile explicitly sets toolChoice:nil and preserves input reasoning. Prior three-step migration forced required/none into DeepSeek Responses and therefore did not match the old adapter; its thinking+required HTTP400 is retained as a port mismatch, not treated as an inherent legacy continuation failure. Corrected only the legacy test seam for this dialect to omit choice while keeping high session reasoning, dynamic marker instructions, distinct successful call/result ordering, exact final3 and settlement gates. Production default preparation is unchanged.

First corrected run deepseek-legacy-choice-parity-1 completed the workflow but failed a stale unconditional unwrap of absent choice; fixed the test and reran after terminal completion. deepseek-legacy-choice-parity-2 passed four actual requests, every one high reasoning and absent tool_choice, three separate ordered think calls and final3. Worker unit regression deepseek-choice-worker-1.log passed. This corrects the historical failure attribution. Separate lookup_file bounded scenario and its requested/effective reasoning metadata assertions are still not ported; no row was closed for that unexecuted test. Other providers' high-reasoning matrix and remaining 61 not_run declarations remain open.

### DeepSeek bounded lookup adapter continuation

Added live_deepseek_continuation.rs and runner preserving pinned LiveTests.swift:764-827 prompts/schema and selected DeepSeek Responses wire behavior. First call requires exactly lookup_file with two string fields, exact path docs/known.md and reason under512 characters; it is not a final. Second call replays sealed output and tool result KNOWN_CONTENT with no tools, requires zero calls and final text containing the marker. Both actual requests require high reasoning and omit tool_choice per the old Responses serializer. Shared total120s deadline constrains production HTTP transport.

`deepseek-bounded-lookup-1` passed in2.35s with real configured DeepSeek V4Flash. Exact bodies and responses plus receipt retained. No inferred effective-reasoning or repair provenance fields added: old argumentRepair whitelist and requested_reasoning/effective_reasoning metadata remain a normalized-result interface gap. Thus row is partial_live_evidence, not full parity. Inventory now has60 not_run declarations plus remaining partial/failed rows. No external-client or worker-process proof inferred from this adapter test.
