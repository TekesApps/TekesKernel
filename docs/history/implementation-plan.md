# Phased implementation plan

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

The Slice sequence below is the staged implementation plan, including historical V2
milestones. It is not the current Cargo inventory: [Code architecture](../architecture/README.md) and
[generated targets/dependencies](../architecture/generated/index.md) own that view.
Current public endpoint behavior is governed by [V3](../../spec/session-endpoint.md).

This document turns the closed design contracts into buildable increments. It
is intentionally language-neutral: it freezes component boundaries,
dependencies, deliverables, conformance gates, and CI semantics, but not a
package manager, test framework, or source-language type system. The current
Rust implementation profile is [Rust: toolchain and implementation constraints](../architecture/rust.md).

The executable files under `spec/` and the fixture oracle remain authoritative. A
language profile may choose modules and libraries; it may not redefine durable
bytes, wire messages, replay, or lifecycle behavior.

## Readiness boundary

Slice 1 may start when this plan and one language profile are present. It uses:

- complete v1 replay from `genesis`, never checkpoint-seeded replay;
- injected config and instruction snapshot descriptors/digests, not live-file
  resolution;
- fake provider, tool, child-process, and supervisor-host seams;
- the four Slice-1 executable contracts and the committed `fixtures/` tree
  unchanged.

Therefore config parsing, instruction resolution, real tools/sandboxing,
checkpoint acceleration, rewrite publication, and Session Endpoint projection
are not hidden Slice-1 work. Config/instructions landed in Slice 2 and
checkpoint acceleration in Slice 3; the remaining surfaces retain explicit
later contract gates below.

## Logical component graph

Every implementation uses these ownership boundaries. They may be packages,
crates, modules, or build targets, but dependencies point only downward:

| component | owns | may depend on |
|---|---|---|
| Schema | I-JSON/JCS values, event and worker-control types, codecs, event-v1 validation fold, checkpoint state schema | no other workspace component; profile libraries may not escape into contracts |
| Store | JSONL framing, checked append, barrier sync, torn-tail scan/repair, asset verification, file/folder locks, full and checkpoint-seeded replay | Schema |
| Protocol | worker-control-v1 negotiation, messages, receipts, correlation/dedup codecs | Schema |
| Profile | config-v1 authority/publication plus instruction-snapshot-v1 capture, merge, hook-binding validation, immutable assets, and effective policy | Schema, Store, Tools |
| Tools | builtin-tools-v1 manifest/catalog, hook pipeline, exec helper, sandbox profiles, fixed backend/effect substrate, dynamic catalog validation | Schema, Store |
| Provider | production adapter registry, exact request rendering, HTTP/SSE execution, normalized frames/terminal, credential/SecretStore seam and provider parity | Schema, Profile |
| Search | title-only thread search, stable paging cursor, active/archive catalog snapshot and rebuildable per-folder cache | Schema, Store |
| Core | tail classification, mode arbitration, attempt/usage transaction, engine-owned tool dispatcher, normalized provider seam, recovery over injected seams | Schema, Store, Protocol, Profile, Tools |
| Worker | worker shell, provider assembly and control loop | Core, Schema, Store, Protocol, Profile, Tools, Provider |
| Supervisor | keyed create/delivery, ensure/reap/sweep, process host, spawn-profile capture, endpoint/transport host, tool and credential brokers | Core, Schema, Store, Protocol, Profile, Tools, Provider, Search, Endpoint, Transport |
| Endpoint | Session Endpoint v2 projection journal, history/live mux, inventory, session-scoped model catalog and typed mutation adapter | Schema, Store, Profile; thin carrier wiring may be hosted by Supervisor |
| Transport | HTTP/WebSocket binding of the imported Session Endpoint v2 shapes | Schema, Endpoint; hosted by Supervisor without importing Worker or Provider |
| Deployment | launchd packaging, shipped external `tekes-selector`, readiness, observability and rollback harness | built executables and Transport; no production component depends on it |
| TestSupport | fixture locator, deterministic clock/ids, crash points, fake provider/tool/process/lease hosts | all above, test builds only |

A lower component cannot import an executable or a higher component. OS calls
sit behind the Platform seams in 08. Tests inject fakes except where a gate
explicitly verifies real Darwin `flock`/`F_FULLFSYNC` or process behavior.

## Fixture loading and test identity

`fixtures/` is read-only input. A language profile must define a fixture-root
resolver with this behavior:

1. use the absolute path in `TEKES_KERNEL_FIXTURES`, if set;
2. otherwise walk from the build/test manifest location toward the repository
   root until both the implementation manifest and `fixtures/manifest.json`
   exist;
3. fail with the searched paths — never synthesize or silently skip.

The loader first compares `manifest.json` with disk in both directions. Event
fixture comparison strips exactly the one JSONL framing LF and compares the
remaining bytes with the implementation's RFC-8785 encoding. Replay, invalid,
torn, wire, and asset corpora are consumed in place; asset name, SHA-256,
declared `bytes`, and carrier references are verified. Tests may never rewrite
the oracle.

Each conformance gate has the stable logical id `Slice1GateNN`. A language
profile maps that id to an exact test symbol and command. One gate may contain
parameterized cases, but implementation-specific unit tests do not reuse gate
ids.

Slice-2 and Slice-3 gates use the corresponding `Slice2GateNN` and
`Slice3GateNN` identities and never reuse an earlier-slice id.

## Slice 1 decomposition

### 1A — Schema and oracle

Deliver:

- closed envelope/kind types and worker-control message types;
- full I-JSON/JCS encoder/decoder with the event-v1 spill rules;
- the single-pass event-v1 validation fold, including extension/min-reader
  behavior and all cross-event constraints;
- the fixture loader and deterministic error classifications.

Acceptance: gates 28, 17, 32, and the positive/negative fixture corpora pass;
all 86 canonical event fixtures round-trip byte-identically, all valid replay
ledgers accept, and each invalid ledger rejects for its declared rule.

### 1B — Durable store and replay

Deliver:

- one checked full-write loop per JSONL event;
- the event-v1/D-45 barrier registry as one exported source of truth;
- local-filesystem capability probe, nonblocking flock, lifecycle lock, and
  path→inode revalidation;
- longest-valid-prefix reader/repair and content-addressed asset protocol;
- full-genesis replay projections for turn state, origin-key dedup,
  checkpoint validity, tail classification, and run ordinal.

Acceptance: gates 18, 29, 30, and the store portion of 1–3 pass under injected
crash points. A valid checkpoint is projected but never authorizes skipping a
v1 prefix; invalidation changes only its future-acceleration eligibility.

### 1C — Worker shell and stub supervisor core

Deliver:

- worker-control handshake before lock acquisition, EOF semantics,
  correlation, receipts, and exit 76;
- `run_start`, turn opening, attempt/lease/dispatch/outcome/usage settlement,
  and D-61 reconcile runs with fake adapters;
- keyed creation and delivery, ensure-running, busy-unknown handling,
  bounded drain/reap/sweep, stop cascade, and parked-hold lifecycle;
- deterministic crash injection at every named conformance boundary.

Acceptance: all nineteen Slice-1 gates pass in the order below with no
network, keychain, real provider, tool process, UI, or live
instruction/config dependency.

## Slice-1 gate map and execution order

The gate set is exactly doc 14's ▸ set: 1–6, 12–14, 17–19, and 28–34.
Test 20 is intentionally not a Slice-1 gate: it requires the deferred
instruction-resolution byte contract, while Slice 1 accepts an injected
immutable descriptor and records its supplied digest.

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 28 CanonicalEnvelopeFixtures | Schema | 1A |
| 2 | 17 DowngradeEvolution | Schema | 1A |
| 3 | 32 ResumePolicyEncoding | Schema | 1A |
| 4 | 18 UpgradeGateBarrier | Store | 1B |
| 5 | 29 TornTailFaultInjection | Store | 1B |
| 6 | 30 CheckpointInvalidationReplay | Store | 1B |
| 7 | 31 ProtocolNegotiationReject | Protocol | 1C |
| 8 | 4 AttemptLeaseCommit | Core | 1C |
| 9 | 5 OutcomeUsageCrashWindow | Core | 1C |
| 10 | 6 CompletedTurnUsageExit | Core | 1C |
| 11 | 33 RunModeArbitration | Core | 1C |
| 12 | 34 SlowHoldLifecycle | Core | 1C |
| 13 | 1 ColdBootCreateRetry | Integration | 1C |
| 14 | 2 DeadTargetKeyedDelivery | Integration | 1C |
| 15 | 3 AliveTargetReceiptCorrelation | Integration | 1C |
| 16 | 13 BusyUnknownWorker | Integration | 1C |
| 17 | 19 CanaryPerRunAttribution | Integration | 1C |
| 18 | 12 SupervisorTotalFailure | Integration | 1C |
| 19 | 14 StopMidGrandchildCrash | Integration | 1C |

## Slice 2 — authoritative launch profiles

Slice 2 replaces Slice 1's injected profile descriptors with production
resolution while leaving provider, tool, checkpoint, rewrite, and endpoint
seams unchanged.

### 2A — Config authority and atomic publication

Deliver `spec/config-v1.md`, its positive/negative fixtures, shared/exclusive
config locking, expected-revision publication, defaults/reference validation,
resolved `ConfigSnapshot` bytes, asset publication, and privilege-reduction
classification. Acceptance: gates 35 and 36.

### 2B — Deterministic instruction snapshot

Deliver `spec/instruction-snapshot-v1.md`, the two-level fixture tree and byte
oracle, descriptor/path safety, stable double scan, deterministic precedence,
the conservative policy meet, snapshot validation from bytes alone, and
content-addressed publication. Acceptance: gates 37, 20, and 21.

### 2C — Launch integration

The supervisor resolves one config snapshot, captures one instruction
snapshot against that exact cwd list, validates their cross-references,
publishes both assets, opens both no-follow/read-only, and inherits the
descriptors into the worker. Before taking its line lock, the worker verifies
both requested digests, validates the instruction workspace indices, derives
the effective policy, and attributes the run with the two digests plus policy
digest. Changes affect only later launches; privilege reductions request a
stop/respawn. Acceptance: all five Slice-2 gates, the real
supervisor→worker descriptor integration, then every Slice-1 gate unchanged.

## Slice-2 gate map and execution order

The combined Slice-2 CI first runs all nineteen Slice-1 gates, then:

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 35 ConfigContractFixtures | Profile schema | 2A |
| 2 | 37 InstructionOracleRejections | Profile schema | 2B |
| 3 | 36 ConfigAtomicPublishRevocation | Profile store | 2A |
| 4 | 20 InstructionSnapshotRace | Profile resolver | 2B |
| 5 | 21 InstructionNextLaunchOnly | Launch integration | 2C |

## Slice 3 — checkpoint-seeded replay

Slice 3 activates [checkpoint-v1](../../spec/event.md). It changes replay
cost, not truth: the JSONL ledger remains authoritative and full replay is the
compatibility/fallback path.

### 3A — State contract and oracle

Deliver the exact format-1 state carrier, digest preimages, conditional
checkpoint barrier, real state asset, and complete `fixtures/checkpoint/`
oracle. Acceptance: gate 38.

### 3B — Seeded replay and fallback

Restore the complete validation fold from the newest supported checkpoint,
retain the decoded event history for projections/backward references, fold the
suffix, and automatically fall back on later invalidation or unsupported
format. Supported-asset corruption remains fail-closed. Acceptance: gates 39
and 40.

### 3C — Retention and publication

Apply full-tuple origin retention at the durable `key_floor`; create state
assets and checkpoint events under the line lock using the reference-before-
event transaction; expose whether replay used genesis or checkpoint seq N.
Acceptance: gates 41 and 42, then all earlier gates unchanged.

## Slice-3 gate map and execution order

The combined Slice-3 CI first runs the complete Slice-2 job, then:

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 38 CheckpointStateOracle | Schema/Store contract | 3A |
| 2 | 39 CheckpointSeededReplayEquivalence | Store replay | 3B |
| 3 | 40 CheckpointInvalidationFallback | Store recovery | 3B |
| 4 | 41 CheckpointKeyRetention | Store/idempotency | 3C |
| 5 | 42 CheckpointPublicationBarrier | Store durability | 3C |

## Slice 4 — fixed catalog and execution substrate

Slice 4 establishes the complete fixed namespace and the real hook/helper/
sandbox execution substrate behind the still-injected `ToolBackend` seam. The catalog contract is proved
before any backend case: passing a filesystem test does not establish that all
legacy source tools were migrated into the one Kernel catalog or classified
safely.

### 4A — Catalog and schema parity

Deliver `spec/builtin-tools-v1.md`, the canonical 27-name
`fixtures/tools/builtin-tools-v1.canonical.json`, the separate migration audit
fixture, a typed closed manifest,
schema resolution/digests, fixed-name collision rejection, stable projection
ordering, role/conditional exposure, dynamic catalog deferral, and a
mechanical migration-parity reader for the pinned source inventories. Preserve
the 12-tool Computer Use shipped-extension inventory separately so optional
plugin names remain covered without becoming fixed.
Acceptance: gate 43.

### 4B — Hook, helper, and sandbox substrate

Deliver tool-hook-v1, exec-helper-v1, sandbox-profile-v1, their committed
fixtures, write-ahead/effective-execution/result pairing, secret scan/spill,
descriptor-first filesystem operations, clean argv exec, timeout/cancellation
and process-group reap, policy-derived Seatbelt/Landlock profiles, and durable
unsandboxed escalation. Acceptance: gates 7, 10, and 11.

### 4C — Workflow and host-tool seam integration

Prove the worker/supervisor control seams for ask/plan and child spawn/report,
and compile backend/effect classes into typed dispatch decisions. Slice 4 does
not claim that all 27 production backends are connected: jobs, Web, exact
cross-thread control, skill/dynamic external/plugin/MCP catalogs and the full
worker dispatcher close in Slice 8 under tool-runtime-v1. Git/GitHub remain
`shell` plus system `git`/`gh`, never a fixed Git family. Acceptance: gates 8
and 9, all Slice-4 gates, and all earlier gates unchanged.

## Slice-4 gate map and execution order

The combined Slice-4 CI first runs the complete Slice-3 job, then the
migration-parity reader against the exact revisions in the audit fixture,
then:

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 43 BuiltinToolManifestParity | Tools catalog/schema | 4A |
| 2 | 7 MutatedToolWriteAhead | Tools pipeline | 4B |
| 3 | 10 DescriptorFirstFileOpen | Helper/filesystem | 4B |
| 4 | 11 SandboxBackendProbe | Sandbox/platform | 4B |
| 5 | 8 AskUserFastAnswer | Worker hold integration | 4C |
| 6 | 9 SpawnValidatorOrdering | Worker/supervisor tool integration | 4C |

## Slice 5 — rewrite publication

Slice 5 activates [rewrite-publication-v1](../../spec/rewrite-publication.md).
It adds no second storage truth: a rewrite is a validated materialized ledger
published through a recoverable folder transaction.

### 5A — Contract and projection

Deliver the closed `op.json` schema, deterministic per-file identity/seq
mapping, supersede/compact application, provider-history normalization,
declared carrier scan, fork copy, and selector-based redact. Acceptance: gate
15 and the rewrite fixture oracle.

### 5B — Publication and recovery

Deliver the two type-specific phase machines, source-prefix binding,
lifecycle exclusion, atomic destination publication, redact retirement, boot
recovery, and recordless-debris GC. Keyed-create staging lives outside the
rewrite namespace. Acceptance: gates 16 and 24.

### 5C — Integration and closure

Wire sweep to operation discovery, expose typed progress/errors, close
D-open-6 by shipping explicit offline redact, and rerun all earlier gates.
Acceptance: the three Slice-5 gates in order, then the complete Slice-4 job
unchanged.

## Slice-5 gate map and execution order

The combined Slice-5 CI first runs the complete Slice-4 job, then:

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 15 RedactRewriteClosure | Rewrite projection/carriers | 5A |
| 2 | 16 RewritePublicationCrashMatrix | Publication/recovery | 5B |
| 3 | 24 StagingOwnershipGC | Sweep/GC ownership | 5B |

## Slice 6 — Session Endpoint v2

Slice 6 activates
[endpoint-projection-v2](../../spec/session-endpoint.md) and imports the
Client-owned v2 corpus without translating it through TekesAppServer v1.

### 6A — Authority and stable projection

Vendor the exact pinned Client corpus with a byte-sync lock/gate. Deliver the
per-thread canonical projection journal, internal Kernel provenance, stable
1:N slots, stream carrier, full-prefix durability, idempotent replay, and
corruption handling. Acceptance: gates 47 and 48.

### 6B — History/live window

Deliver message-aligned raw-range history including chunks, history/live byte
identity, bounded mux buffering, overlap dedup, baseline-ahead and live-gap
refetch decisions, unknown-required fail-closed, and `ignorable:true` skipping.
Acceptance: gates 49 and 52.

### 6C — Native management surface

Deliver canonical UUID public identity, inventory independent of transcript,
session-scoped model readiness, typed keyed create/prompt/queue-edit/cancel/
rename/respond/fork paths, and independent archive/unarchive operations.
Archive keeps the authoritative folder while Client Mirror deletes its active
projection; unarchive enables complete rematerialization. Acceptance: gates 50
and 51, then every earlier gate unchanged.

Physical HTTP/WebSocket listener selection is the Slice 9 transport layer over
this carrier-neutral service and cannot change DTO bytes or ordering. Slice 10
packages that completed transport for production deployment. A deployment
claiming v2 readiness must connect the imported unary and mux carrier to this
service and run the same fixtures end-to-end.

## Slice-6 gate map and execution order

The combined Slice-6 CI first runs the complete Slice-5 job and the pinned
authority byte check, then:

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 47 EndpointAuthorityByteSync | Imported contract/oracle | 6A |
| 2 | 48 StableProjectionJournal | Projection allocation/durability | 6A |
| 3 | 49 ChunkHistoryLiveIdentity | History/live carrier | 6B |
| 4 | 50 EndpointMutationArchiveContract | Typed native mutations | 6C |
| 5 | 51 InventoryModelReadiness | Management-plane separation | 6C |
| 6 | 52 StitchGapOverlapFailClosed | Mux/reconnect/evolution | 6B |

## Slice 7 — production provider/worker runtime

Slice 7 activates [provider-runtime-v1](../../spec/provider-runtime.md),
[credential-broker-v1](../../spec/credential-broker.md), and
[secret-store-v1](../../spec/secret-store.md)'s language-neutral record and
injectable resolution seam. The production macOS Keychain backend, signed
access-group proof, and durable generation/digest anti-rollback authority land
in Slice 10. Slice 7
replaces `FakeProvider` at the Worker boundary while retaining the already-
proved attempt, admission, recovery and context contracts.

Before 7A code, close two inherited conformance-wiring debts without changing
semantics: the fixture verifier must reject an undeclared top-level corpus, and
`ci-slice4.sh` must run documented gates 44–46 by their exact names. The full
Slice-6 job must remain green after that prerequisite commit.

### 7A — Dialect oracle and normalized seam

Complete every raw fixture named by `fixtures/provider-runtime/`, pin the
predecessor source bytes, implement the five-row registry, pure request
rendering and terminal/frame normalization. A row remains disabled until its
complete parity corpus passes. Acceptance: gates 53, 54 and 57.

### 7B — HTTP, admission and recovery

Implement the bounded HTTP/SSE client, an injectable supervisor-held
SecretStore over the closed record/result contract, the inherited private
credential channel, rotation/revocation behavior, exact
attempt/dispatch ordering, cancellation/backpressure, query-by-identity and
asset-only adopt. No secret crosses into ledger, endpoint carrier or logs.
Acceptance: gates 55 and 56.

### 7C — Worker provider loop

Connect provider context projection, dialect-level catalog compilation,
overflow preflight, compaction/epoch retry, streamed endpoint frames, terminal
outcome/usage and lease release. Provider parity compiles an explicitly
supplied catalog, but the production worker advertises an empty catalog until
Slice 8 makes the fixed tools executable; advertising a callable tool before
its dispatcher exists is forbidden. Acceptance: gate 58, all Slice-7 gates,
then every earlier gate.

## Slice-7 gate map and execution order

The combined Slice-7 CI first runs the complete Slice-6 job and provider source
lock, then:

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 53 ProviderDialectParityOracle | Provider schema/parity | 7A |
| 2 | 54 ProviderStreamFramingBounds | Provider transport decoder | 7A |
| 3 | 57 ProviderTerminalNormalization | Provider normalization | 7A |
| 4 | 55 ProviderSendRecoveryMatrix | Worker/provider recovery | 7B |
| 5 | 56 ProviderHttpClassificationAndSecrets | HTTP/security | 7B |
| 6 | 58 ProviderOverflowCompactRetry | Worker/context integration | 7C |

## Slice 8 — production builtin-tool integration

Slice 8 activates [tool-runtime-v1](../../spec/tool-runtime.md). It replaces
the injected fake ToolBackend with one dispatcher over the Slice-4 catalog and
substrate; it does not introduce a second catalog or compatibility aliases.

### 8A — Dispatcher and durable pipeline

Complete per-tool fixtures, implement exact argument validation and the one
engine-owned derived backend/effect/policy path (including the typed hold
terminal), then run all backend/effect crash windows.
Acceptance: gates 59 and 60.

### 8B — Holds and supervisor control

Activate [worker-control-v2](../../spec/worker-control.md) with correlated/
deduplicated tool-control messages and
connect ask/plan, context, task/subagent/report, goal and memory transactions.
The worker remains the only semantic-event author. Slice 8 delivers the
production handler, durable receipt authority, and injectable process/job
authority boundary. The developer stdin supervisor shell is not the Slice-10
daemon: until that daemon supplies the complete process-host and job-launch
authorities it MUST negotiate v1 only, rather than advertise v2 with a partial
or unavailable handler. Gate 62 injects complete deterministic authorities and
proves the v2 behavior; Slice 10 performs the executable daemon assembly.
Acceptance: gates 61 and 62.

### 8C — Process, job, HTTP and dynamic backends

Connect helper filesystem/exec, ownerless jobs, bounded Web, immutable skill/
tool discovery, and config-declared external catalogs through
[dynamic-catalog-discovery-v1](../../spec/launch-bindings.md).
The credential-conditional `web_search` backend is the independent
`tavily_v1` authority in
[web-search-provider-v1](../../spec/web-search-provider.md): it is not a model
provider capability, and only an exact active broker scope enters the frozen
worker catalog.
Advertise only executable dependencies and enforce one secret/policy/hook
path. Plugin/MCP discovery remains fail-closed until a separate executable
contract defines it; an integration declaration alone is not a capability.
Acceptance: gates 63 and 64, all Slice-8 gates, then every earlier gate.

## Slice-8 gate map and execution order

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 59 ToolDispatcherCatalogCoverage | Tool catalog/dispatcher | 8A |
| 2 | 60 ToolWriteAheadCrashMatrix | Tool durability | 8A |
| 3 | 61 ToolHoldStopResume | Worker hold integration | 8B |
| 4 | 62 SupervisorToolControlDedup | Worker/supervisor protocol | 8B |
| 5 | 63 JobAndHelperLifetime | Process/job lifecycle | 8C |
| 6 | 64 ToolPolicySecretAndNetwork | Policy/security | 8C |

The combined job runs these after the complete Slice-7 job and verifies the
builtin and tool-runtime registries in both directions before dispatch tests.

## Slice 9 — physical Session Endpoint transport

Slice 9 activates [endpoint-management-v2](../../spec/session-endpoint.md)
and [endpoint-transport-v2](../../spec/session-endpoint.md).
It binds Slice 6's carrier-neutral endpoint to the Client-owned HTTP/WebSocket
contract without changing any DTO or endpoint seq.

### 9A — Listener and negotiation

Implement loopback HTTP/WebSocket listeners, exact envelope decoding,
`host.describe` version compatibility, `.tekes` driver/server-registry parity,
all 20 request/result/error registrations, crash-recoverable management
operations, and typed carrier errors. Acceptance: gates 65–66.

### 9B — History/live handoff

Bind journal snapshots to mux subscription, bounded backpressure, reconnect,
drain and generation changes. Acceptance: gates 67–69.

### 9C — Client end-to-end

Run the real Tekes Client against create/history/live/mutations and the
archive-delete/unarchive-rematerialize Mirror lifecycle. Acceptance: gate 70,
all Slice-9 gates, then every earlier gate.

## Slice-9 gate map and execution order

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 65 EndpointTransportAuthority | Transport/authority | 9A |
| 2 | 66 EndpointHttpErrorAndIdempotency | Unary transport | 9A |
| 3 | 67 EndpointSubscribeAtomicity | Mux durability | 9B |
| 4 | 68 EndpointRawHistoryReconnect | Client window integration | 9B |
| 5 | 69 EndpointDrainBackpressureSecurity | Transport lifecycle/security | 9B |
| 6 | 70 EndpointArchiveClientRematerialization | Client/AS integration | 9C |

## Slice 10 — macOS production packaging and acceptance

Slice 10 activates [deployment-v1](../../spec/deployment.md). It packages the
completed service; it does not move upgrade/rollback authority into Kernel.

### 10A — Service ownership and launchd

Ship the supervisor as an embedded-profile application bundle plus the bare
worker/helper executables, launchd plist/install layout, local-filesystem
preflight, single-root ownership, readiness and bounded drain. launchd enters
through the fixed external selector's resident `serve` command, which holds the
service lifetime lock, recovers/validates selection, spawns the chosen
supervisor with frozen attribution and the closed v1 authority-registry digest,
bootstrap and launcher-lifetime pipes, then monitors and reaps it; the
plist never dereferences `selector/active` directly. Acceptance: gates
71–72.

The installer owns the shared login-Keychain endpoint bearer lifecycle; signed
supervisor and Client carry the same immutable-install-identity access-group
entitlement and read it directly, selector never handles it, and the
fixed `.tekes` origin is `http://127.0.0.1:7347`.
Separately, the signed installer and supervisor carry the derived
provider-secret group from [secret-store-v1](../../spec/secret-store.md). The
supervisor uses the exact Security.framework backend, freezes provider and
web-search scopes from one ConfigSnapshot, refreshes live brokers on ensure,
the synchronous config-mutation hook and the periodic safety net, and never
reuses the endpoint bearer.
An immutable install identity pins that team/group across upgrades; the signed
installer's separate durable operation recovers install, rotation and uninstall
across every Keychain/filesystem/launchd boundary.
The Tekes product distribution additionally carries a canonical wrapper with
the signed release plus a separately signed installer actor. Tekes.app may call
only that actor's closed status/install-or-upgrade/ensure-running/enable/disable
CLI and cannot mutate Keychain, the install root or launchd itself. Wrapper and
installer contracts are byte-checked by both repositories before publication;
the Gate-72/76 ProductionUAT coordinator remains a separate release-evidence
actor and is not a product fallback.

### 10B — External activation and rollback

Implement the real `tekes-selector` CLI/state contract and its
separate test harness, signed version staging, canary attribution,
atomic stable-selector update, short-lock observation and automatic
last-known-good rollback outside the supervisor. External activation first
boots out and waits for resident serve to stop; bootstrap after a closed
transaction is safely repeatable. The deployment controller authors the fixed
Client canary and selector only attests its exact session/run ledger evidence;
ordinary post-login reboot start requires no new canary and does not require
opening the Tekes GUI; the per-user LaunchAgent makes no pre-login claim.
Automatic rollback uses a serve-authored durable request identity rather than a
fabricated CLI invocation; the sole closed operation retains exact retry bytes
only for the most recent CLI request. Promotion serialization resets the
failure counter and retains the full canary session/run pair.
Acceptance: gates 73–74.

### 10C — Observability and production release

Implement production-wired redacted structured logs/metrics and the offline
installed-supervisor `support-bundle` command, clean install/
reboot/uninstall tests, real Client UAT app and release evidence. The signed Gate
76 lane also proves provider-secret ACL isolation, immediate generation
rotation/revocation and full diagnostic/durable redaction. Acceptance: gates
75–76, then the complete Slice-10 job from a clean checkout.

## Slice-10 gate map and execution order

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 71 DeploymentFilesystemAndOwnership | Platform/readiness | 10A |
| 2 | 72 LaunchdCrashRecovery | Service recovery | 10A |
| 3 | 73 InstallUpgradePublicationCrashMatrix | External activation | 10B |
| 4 | 74 CrashLoopExternalRollback | External rollback | 10B |
| 5 | 75 ProductionObservabilityRedaction | Operations/security | 10C |
| 6 | 76 CleanInstallClientUninstallE2E | Production acceptance | 10C |

## Slice 11 — Skills and commands

Slice 11 activates [skill-package-v1](../../spec/skill-package.md) and
[command-catalog-v1](../../spec/command-catalog.md) over the immutable
instruction snapshot and existing keyed input authority.

### 11A — Whole-directory skills

Derive one package per `skills/<name>/SKILL.md`, preserve every companion as a
frozen resource, replace packages by directory rather than file, bind model
`skill_explorer`/`skill` and Client `skills/list` to the same content digest,
and reject invalid winners without falling back. Acceptance: gate 77.

### 11B — Commands and durable run

Parse the predecessor-compatible command front matter, retain list metadata,
expand only the closed argument grammar, and route `commands/run` through the
existing non-steer `SessionDeliveryAuthority` with the caller key. File/shell
directives remain literal and plugin/MCP commands are absent. Acceptance:
gates 78–79.

### 11C — Capability separation and migration

Expose exactly `skills/list`, `commands/list`, and `commands/run` as the
format-1 Client resource capability, not as additions to Session Endpoint v2.
Register the handler through collision-free `CompositeProductionEndpointRoutes`
so later Slice handlers coexist rather than overwrite one another. Lock the
pinned AppServer/Runtime sources and prove production adapter origin, preflight,
receipt, daemon boot-time catalog assembly, additive carrier dispatch,
frozen-name rejection and 20-route invariants. Acceptance: gate 80, then all
earlier repository gates unchanged.

## Slice-11 gate map and execution order

| order | gate | logical suite | first slice |
|---:|---|---|---|
| 1 | 77 SkillPackageMigrationAndPrecedence | Resource/package | 11A |
| 2 | 78 CommandCatalogExpansionAndNegativeCorpus | Resource/command | 11B |
| 3 | 79 ResourceCapabilitiesAndKeyedCommandSubmission | Client resource capability | 11B |
| 4 | 80 ProductionInputSeamAndFrozenV2Authority | Production/cross-carrier | 11C |

## CI semantics

Every change to implementation sources/tests, `spec/`,
`docs/verification/gates/README.md`, or `fixtures/` must:

1. build all production and test components in debug and optimized modes;
2. pass the language's formatter and static analysis with warnings denied;
3. run every gate through the highest implemented slice individually in the
   documented order, with the fixture
   root explicit so a crash-test timeout identifies its gate;
4. run the full unit/integration suite;
5. verify the fixture manifest/oracle and leave `fixtures/`, `spec/`, and
   doc 14 byte-clean;
6. run without network and enforce a 10-minute per-gate timeout, killing and
   reaping every crash child in teardown.

The authoritative durability lane runs on macOS 15 or newer. Other platforms
may run Schema/Protocol portability lanes but cannot replace macOS: Darwin
`F_FULLFSYNC`, `flock`, APFS/local-volume probes, and process behavior are part
of the acceptance surface.

The Slice-10 production preflight is deliberately narrower than Store: it
requires APFS. Operational logs use no-follow owner/mode/inode validation and
one serialized rotation/append critical section; an append failure immediately
drops endpoint readiness and emits the closed content-free stderr fallback.

## Slice completion and publication

A slice is not complete merely because its implementation exists locally. Its
definition of done is, in order:

1. the slice's ordered gates, all earlier-slice gates, static analysis,
   debug/release builds, and the full workspace suite pass against the
   committed fixture oracle;
2. the implementation, contracts, fixtures, tests, and status/plan updates
   form a scoped semantic commit (generated output and unrelated workspace
   changes are excluded);
3. that commit is pushed to a remote branch and merged to `main` only after
   the same gate evidence is green;
4. local `main` is fast-forwarded to the merged result and its commit id is
   verified equal to `origin/main`.

Slice 10 separates implementation integration from production qualification:
its signed `preflight` must satisfy every repository-owned gate and final-path
signature/profile check before merge, while the target-user reboot halves of
Gates 72 and 76 may run immediately before release. A preflight-only merge is
recorded as **not production qualified**; no tag or distribution may treat the
external gates as skipped, simulated, or green.

No later slice starts from an unpublished or only-locally-passing predecessor.
If two historical slices were accumulated before this rule existed, their
first publication is one explicitly named baseline closeout; every later
slice remains independently committed and merged.

The only scheduling exception is parallel development from the same published
baseline for the explicitly independent lanes named below. It is not parallel
publication: branches remain isolated, merges are serialized, and before each
merge the branch updates onto the latest `main` and reruns every applicable
earlier-slice gate plus any cross-lane dependency gate. A lane may not treat an
unmerged sibling's working tree, local commit, fixture, or wire behavior as an
input.

## Later-slice closure contracts

Slices 11–14 are now implemented. Their publication order still records the
rule that no lane may invent missing bytes or wire behavior before landing its
named contract and fixtures.

| required contract / asset | minimum content it must freeze | blocks | Slice-1 seam |
|---|---|---|---|
| `spec/builtin-tools-v1.md` plus `fixtures/tools/` | complete ownerless fixed namespace, top-level argument order, availability, backend/effect class, single-catalog composition/collision/deferral/evolution, separate two-repository migration audit | Slice 4 catalog and every real tool | fake ToolBackend; no real catalog in Slice 1 |
| `spec/tool-hook-v1.md` | hook request/result unions, mutation and deny semantics, correlation, timeouts, size/secret rules, canonical fixtures | Slice 4 hooks | fake ToolBackend; gates 7–8 do not gate Slice 1 |
| `spec/exec-helper-v1.md` | helper framing/negotiation, operation union, argv/stdin/stdout, fd/root passing, typed errors, byte/time caps, cancellation/kill and atomic write/patch semantics | Slice 4 filesystem/exec tools | no real helper in Slice 1 |
| `spec/sandbox-profile-v1.md` plus platform profiles | policy input schema, Seatbelt and Landlock/seccomp derivation, probes, fail-closed errors, unsandboxed approval binding, golden policy/profile cases | Slice 4 side-effectful tools | no real exec in Slice 1 |
| [rewrite-publication-v1](../../spec/rewrite-publication.md) | implemented in Slice 5: `op.json`, materialized projection, carrier closure, phase recovery, retirement and GC | — | — |
| [endpoint-projection-v2](../../spec/session-endpoint.md) | implemented in Slice 6: binding Client-v2 import, stable endpoint journal, raw-range history/chunks, mux recovery, inventory and typed mutations | — | no endpoint in Slice 1 |
| [provider-runtime-v1](../../spec/provider-runtime.md), [credential-broker-v1](../../spec/credential-broker.md), [secret-store-v1](../../spec/secret-store.md), [web-search-provider-v1](../../spec/web-search-provider.md), and their fixtures | implemented across Slice 7/8/10: model-adapter registry, exact request/stream/terminal bytes, deterministic recovery query key, private credential launch channel, injectable closed secret records, independent `tavily_v1` search configuration/request/result mapping, and the production Keychain identity/backend/live refresh plus durable generation/digest anti-rollback authority in Slice 10 | — | FakeProvider through Slice 6; fake SecretStore before the Slice-10 production lane |
| [provider-dialect-profiles-v1](../../spec/provider-dialect-profiles.md), [Provider parity provider-dialect-parity](../verification/provider-parity.md), and `fixtures/provider-dialects/` | implemented: exact vendor wire dialects, model profiles, production-control wiring and per-route evidence through Gates 93–96 | — | no effect on Slice 1 |
| [tool-runtime-v1](../../spec/tool-runtime.md), [worker-control-v2](../../spec/worker-control.md), [launch-bindings-v1](../../spec/launch-bindings.md), [dynamic-catalog-discovery-v1](../../spec/launch-bindings.md), `fixtures/tool-runtime/`, `fixtures/wire-v2/`, and `fixtures/launch-bindings/` | implemented in Slice 8/10: single dispatcher, 27-tool backend coverage, versioned supervisor-control extension, durable approvals, ownerless jobs, immutable dynamic/goal bindings, config-declared `helper_exec` discovery, helper/network recovery and secret/policy path; Slices 12/13 add the separately versioned plugin/MCP carrier | — | Tool substrate + injected backend through Slice 7; worker-control v1 remains valid |
| [endpoint-management-v2](../../spec/session-endpoint.md), [endpoint-transport-v2](../../spec/session-endpoint.md), plus `fixtures/endpoint-transport/` | implemented in Slice 9: exact 20-route DTO/author/recovery contract, HTTP/WebSocket binding, negotiation/errors, mux lifecycle/backpressure/drain and real Client integration | — | carrier-neutral Endpoint through Slice 8 |
| [deployment-v1](../../spec/deployment.md) plus `fixtures/deployment/` and `fixtures/secret-store/` | implemented in Slice 10: launchd ownership, filesystem/readiness, endpoint/provider Keychain identity and generation floor, install/upgrade selector, external rollback, observability and production UAT | signed external Gates 72/76 before production qualification | developer-run binaries through Slice 9 |
| [skill-package-v1](../../spec/skill-package.md), [command-catalog-v1](../../spec/command-catalog.md), and `fixtures/resources/` | implemented in Slice 11: immutable whole-directory user/project packages, companion reads, command catalog/expansion, independent format-1 Client resource capability and ordinary keyed-input submission; Slices 13/14F add unified MCP reads | — | existing InstructionSnapshot and SessionDeliveryAuthority |
| [client-extensions-v1](../../spec/client-extensions.md) and `fixtures/client-extensions/` | implemented in Slice 14F: frozen base isolation, ten atomic capability groups, 43 closed retained methods, complete predecessor dispositions, authority/idempotency/error rules, production registry and `.tekes` Client parity | — | existing collision-checked `ProductionEndpointRoutes`; no new SessionEvent |

Slices 1–10 close the native Kernel core, not every optional capability of the
replaced AppServer/Runtime. The disk-verified matrix and dependency order are
normative in [Feature parity and qualification](../verification/feature-parity.md). The numbering stops
at Slice 14:

- **Slice 11 — Skills + Commands (implemented):** complete skill packages;
  command catalog, expansion and keyed-input submission; independent
  `skills/list` and `commands/list`/`commands/run`; migration fixtures and
  gates 77–80.
- **Slice 12 — Plugin lifecycle (implemented):** existing `.tekesplugin` and
  `tekes-plugin.json` compatibility; install/update/remove/enable/grants;
  signatures, component projection and transaction recovery through Gates
  81–85. Gates 81/82 are hermetic carrier/policy proof; qualification of a
  plugin additionally requires an explicit private archive and macOS
  `codesign`, otherwise reports `NOT QUALIFIED`. Its generic
  `(plugin,component)` resolver supplies an immutable executable and
  package-digest generation to Slice 13 without launching it. Computer Use is
  not a Slice-12 plugin input.
- **Slice 13 — MCP runtime + management (implemented):** stdio, HTTP/OAuth and daemon pool;
  tools/resources/prompts/tasks; discovery, `tool_search`, credentials,
  reconnect/recovery and management gates 86–92 under
  [mcp-runtime-v1](../../spec/mcp-runtime.md). It remains generic and contains
  no Computer Use special case.
- **Slice 14 — Optional product parity closure (implemented):** independent gate groups 14B
  safepoint (implemented internally by
  [safepoint-v1](../../spec/tail-lifecycle.md), `fixtures/safepoint/`, the
  `safepoint` crate and Gates 97–100; v1 snapshots the session-selected stable
  folder and fails legacy unbound multi-root mutation closed), 14C
  schedule/cron (implemented internally by
  [schedule-v1](../../spec/schedule.md) and Gates 101–104), 14D thread
  search (implemented internally by
  [thread-search-v1](../../spec/thread-search.md) and Gates 105–108), and 14E
  Web-chain (implemented by [web-tools-v1](../../spec/web-tools.md) and Gates
  109–112) converge into the implemented 14F
  versioned Client capabilities and an exhaustive public-route disposition
  audit. After 14F, 14A performs the final black-box qualification of the
  unchanged `TekesComputerUse` executable and tool implementation as an
  ordinary local stdio MCP reference with no plugin binding or Kernel special
  case. The 14F audit includes agent
  presets/types, model connection/profile administration, permissions and
  workspace capability policy, resources/references, tool introspection,
  sidechat, feedback, remote Git UI, usage/cache attribution,
  debug/diagnostics, and explicit replacements or retirement for legacy v1
  freeze/offload/context/direct-shell routes.

Slice 14E retains the production bounded public fetch, deterministic local
HTML/text extraction, and credential-brokered Tavily source-hit search. Its
predecessor inventory permanently retires external Jina/Tavily-extract
fallbacks, synthesized-answer metadata, and environment-owned reader policy
or credentials. The ordered lane is 109 extractor bytes, 110 network bounds,
111 search wire/error matrix, then 112 credential lifecycle and disposition;
`scripts/ci-slice14e.sh` also runs every Slice-13 gate and workspace lint. It
adds no Client route before 14F.

These slices are closed by their executable contracts, fixtures and doc-14
gates. Every Slice-14 group is implemented or explicitly retired; an
unresolved future change cannot be moved to a new Slice 15 without a new
versioned plan.

The execution graph is explicit rather than implied by numeric order. Here,
“parallel” always means the isolated-development exception above; publication
and merge remain serialized:

- Slice 11 is an independent user/project resource lane based on the published
  Client/Slice-10 baseline.
- Slices 12 and 13 may develop in parallel from that common published baseline
  once their own contracts and gates are frozen. Whichever merges second must
  update onto the first merged result and rerun both the earlier-slice suite
  and its applicable cross-carrier gates.
- Slices 14B, 14C, 14D, and 14E may develop as isolated parallel closure lanes
  from their common published baseline once their named contracts exist. Their
  merges are serialized with the same latest-`main` and full applicable-gate
  rule.
- Slice 14F was the Client convergence gate. It started only after Slice 11,
  Slices 12/13, and every 14B–14E lane was merged and published with an
  implemented or explicitly
  retired disposition, so its Client capability bindings and exhaustive
  public-route audit describe the final surface rather than a moving target.
- Slice 14A ran last, after 14F. It saves and starts the unchanged
  `TekesComputerUse` executable through ordinary user-scope stdio MCP
  configuration, then
  black-box checks 12-tool discovery/calls, signature, hermetic TCC
  grant/denial, real denial, stop/restart and recovery. The interactive
  user-authorized granted call remains release UAT. A TCU-specific Kernel
  protocol, DTO, dispatcher,
  execution path, Rust operation, or conditional is forbidden; failures may
  repair only the generic MCP/process/TCC lifecycle.

This capability work started from a green real-Tekes Client baseline: the
Kernel independent smoke, built-in endpoint integration, Settings, automated
Client UAT, and real Tekes launch acceptance are prerequisites rather than
work deferred into Slice 14F. Slice 14F is now implemented; the named external
release/UAT conditions remain separate from capability closure.

`spec/checkpoint-v1.md` is no longer deferred: Slice 3 implements its exact
state carrier, digest algorithms, active-key floor, invalidation fallback, and
publication barrier. Later rotation policy may choose *when* to checkpoint but
may not change those durable bytes or replay semantics.

The endpoint contract uses the accepted Client/DSH behavior in
`Tekes/docs/session-event-conversation-transport-v2.md` (`docs/session-event-conversation-transport-v2.md` in the closed-source Tekes client repository)
and its shared
`session-endpoint-contract-v2/` (`docs/session-endpoint-contract-v2` in the closed-source Tekes client repository)
corpus. TekesKernel references or byte-checks that corpus, never forks it. The
frozen TekesAppServer v1 contract is not an input to the v2 projection.

## First three implementation increments

1. **Build graph + oracle reader.** Add the language profile's modules,
   fixture locator, SHA-256/manifest checks, I-JSON/JCS codec, and
   event/worker-control value types. Accept when gate 28's positive corpus is
   byte-identical and its negative corpus rejects deterministically.
2. **Validation fold + durable store.** Add constraints 1–10, full-genesis
   replay, barrier registry, checked append, locks, and torn-tail repair.
   Accept when gates 17/18/29/30/32 pass and the fixture tree remains clean.
3. **Processes and recovery shell.** Add protocol negotiation, worker shell,
   stub supervisor, fake hosts, keyed delivery, attempts/outcomes, D-61
   recovery, stop, and park flows. Accept when the remaining thirteen gates
   pass, then run the full nineteen-gate job from a clean checkout.

Anything beyond these acceptance checks belongs to a later slice unless it
repairs a contract violation discovered by implementation.
