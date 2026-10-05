# Rust implementation milestones

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

## First three Rust increments

1. **Workspace and schema.** Create the Cargo graph, fixture resolver,
   kernel-owned I-JSON/JCS codec, typed event/worker-control payloads, and
   manifest/asset verifier. Acceptance: gates 28, 17, and 32.
2. **Fold and store.** Implement constraints 1–10, full-genesis replay,
   barrier registry, checked append, Darwin locks/F_FULLFSYNC, asset protocol,
   and torn-tail repair. Acceptance: gates 18, 29, and 30 plus all valid and
   invalid ledgers.
3. **Processes and recovery.** Implement worker-control negotiation,
   `tekes-worker`, stub `tekes-supervisor`, fake seams, keyed delivery,
   attempt/outcome transactions, D-61 recovery, stop, and parked holds.
   Acceptance: the remaining thirteen gates, then the entire ordered
   nineteen-gate job from a clean checkout.

The later contracts in doc 16 remain language-neutral and must land before
their slices. Their Rust APIs are implementation details derived from those
contracts, not substitutes for them.

## Slice 2 increments

1. **Profile schema/store.** Add `profile`, config authority types,
   expected-revision atomic publication, resolved snapshot bytes, asset
   retention, and the config rejection oracle. Acceptance: gates 35 and 36.
2. **Instruction resolver.** Add descriptor-first no-follow discovery,
   double-scan stability, deterministic source/effective validation,
   conservative policy meet, and the committed instruction oracle.
   Acceptance: gates 37 and 20.
3. **Spawn binding.** Resolve/capture/publish once per launch; pass both
   immutable assets through inherited read-only descriptors; verify them
   before the worker line lock and record their digests/effective-policy
   digest in `run_start`. Acceptance: gate 21, the real supervisor→worker FD
   integration test, then `scripts/ci-slice2.sh` from a clean checkout.

## Slice 3 increments

1. **Checkpoint contract/oracle.** Add the format-1 closed state schema,
   exact dependency/state digest preimages, conditional barrier membership,
   real state asset, and checkpoint fixture corpus. Acceptance: gate 38.
2. **Seeded replay.** Restore the validation fold from the newest supported
   state asset, fold the suffix, prove equivalence with genesis replay, and
   fall back on invalidation/unsupported formats while failing closed on
   corrupt supported assets. Acceptance: gates 39 and 40.
3. **Retention/publication.** Apply `key_floor` to full origin tuples and
   publish asset-before-event under the line lock with both required sync
   boundaries. Acceptance: gates 41 and 42, then `scripts/ci-slice3.sh` from a
   clean checkout.

## Slice 4 increments

1. **Catalog contract.** Add the closed 27-name manifest, separate migration
   audit, typed classifications, migration-parity reader, deterministic
   role/conditional projections, catalog digest, and collision rejection.
   Acceptance: gate 43.
2. **Execution substrate.** Add canonical pre/post hook processes, the
   negotiated descriptor-first helper, atomic write/patch, bounded argv exec,
   process-group cancellation, secret scan/spill, Darwin Seatbelt derivation,
   Linux fail-closed planning, and approval-bound unsandboxed fallback.
   Acceptance: gates 7, 10, and 11 plus the hook/helper/sandbox oracle tests.
3. **Workflow integration.** Route the engine's sole `ToolBackend` seam through
   write-ahead execution, prove durable approval answers, spawn/validator
   reconciliation ordering, and the complete worker/supervisor-facing gate
   sequence. Acceptance: gates 8 and 9, then `scripts/ci-slice4.sh` from a
   clean checkout.

## Slice 5 increments

1. **Rewrite contract/projection.** Add the closed operation record, file/id
   map, explicit materialization rules, redact inventory, recursive carrier
   closure, and fixture oracle. Acceptance: gate 15.
2. **Publication/recovery.** Add the fork/redact phase machines, source-prefix
   ownership, validated atomic destination publication, type-specific source
   retirement, and idempotent sweep. Acceptance: gate 16.
3. **Ownership GC/integration.** Move keyed creates to `.create-staging/`, GC
   only recordless rewrite staging, resume recorded operations before normal
   sweep work, and close D-open-6. Acceptance: gate 24, then
   `scripts/ci-slice5.sh` from a clean checkout.

## Slice 6 increments

1. **Contract and authority.** Import the pinned Client-v2 corpus, add its
   byte-sync gate, the closed endpoint event/RPC types, and stable UUID public
   identity. Acceptance: gate 47.
2. **Projection and window.** Add `endpoint-v2.jsonl`, deterministic 1:N slots,
   full-sync-before-live stream records, replay/dedup/tail repair,
   message-aligned history, and bounded stitch/mux behavior. Acceptance: gates
   48, 49, and 52.
3. **Native service.** Add inventory without transcript reads, keyed native
   mutations, explicit archive/unarchive, and independent model readiness;
   connect the deployment carrier without changing bytes. Acceptance: gates 50
   and 51, then `scripts/ci-slice6.sh` from a clean checkout.

## Slice 7 increments

0. **Inherited gate wiring.** Make manifest-vs-disk comparison include
   undeclared corpus roots and add Slice-4 gates 44–46 to `ci-slice4.sh`.
   Acceptance: a synthetic extra corpus fails, then `ci-slice6.sh` passes.
1. **Provider crate and oracle.** Add the closed registry, normalized seam,
   completed predecessor request/response/stream fixtures and deterministic
   render/decoder tests. Acceptance: gates 53, 54 and 57.
2. **Bounded HTTP runtime.** Select and workspace-pin one async runtime/client;
   implement the inherited credential-broker channel over the injectable closed
   SecretStore record/result seam, SSE framing, time/byte/backpressure bounds,
   cancellation and the complete classification/recovery matrix. The production
   Keychain backend and entitlements remain Slice-10 work. Acceptance: gates 55
   and 56.
3. **Worker integration.** Replace `FakeProvider` only at the executable
   assembly boundary, connect doc-11 context, endpoint frames, usage/outcome,
   overflow compact/retry and lease release. Acceptance: gate 58, then
   `scripts/ci-slice7.sh` from a clean checkout.

## Slice 8 increments

1. **Single dispatcher.** Keep the Slice-4 `tools` crate as schema/substrate;
   add the engine-owned dispatcher, expand `ToolBackend` to receive the full
   `ToolExecution` plus effective invocation and return the closed
   completed/hold/unavailable terminal, and cover all 27 names. Acceptance:
   gates 59–60.
2. **Control and helper processes.** Implement `worker-control-v2` as an
   additive protocol module over v1, add the
   `tekes-helper` executable, connect hold/context/task/subagent/report, and
   prove retry/EOF semantics. The injectable production handler and receipt
   store land here; the developer stdin supervisor remains v1-only until the
   Slice-10 daemon supplies the complete runtime/job authorities required by
   the v2 capability promise. Acceptance: gates 61–62.
3. **Remaining real backends.** Add ownerless job broker, bounded HTTP,
   the independent credential-brokered `tavily_v1` implementation of
   `web_search`, immutable discovery and declared dynamic catalogs; remove
   executable assembly's fake ToolBackend. Search request/response bytes and
   readiness are gated by `spec/web-search-provider-v1.md`, its fixture checker,
   and the worker rotation/revocation tests. Acceptance: gates 63–64, then
   `scripts/ci-slice8.sh` from a clean checkout.

## Slice 9 increments

1. **Carrier crate.** Add exact v2 unary/WebSocket envelopes, bounded decoder,
   exact `SessionHostDescription` compatibility, driver/server route-registry
   parity, and typed HTTP errors over `endpoint`. Acceptance: gates 65–66.
2. **Mux lifecycle.** Add atomic subscribe, history/live stitch,
   backpressure/gap close, drain and generation recovery without holding store
   locks across socket writes. Acceptance: gates 67–69.
3. **Client integration.** Run the shared Tekes fixtures and real Client
   harness, including archive deletion/unarchive rematerialization. Acceptance:
   gate 70, then `scripts/ci-slice9.sh` from a clean checkout.

## Slice 10 increments

1. **Service package.** Produce the release supervisor inside
   `TekesKernelSupervisor.app` with its exact embedded provisioning profile,
   plus bare `tekes-worker` and `tekes-helper`; add launchd plist generation
   through the stable
   resident `tekes-selector serve` entry (recover/validate, then spawn/observe
   the selected supervisor with frozen selection and closed v1
   authority-registry digest argv, bootstrap-status and launcher-lifetime
   pipes), explicit
   roots/listen address, filesystem preflight,
   ownership/readiness and bounded drain. Acceptance: gates 71–72.
   Packaging also owns immutable install signing identity and the recoverable
   generation/rotation/deletion transaction for the shared Keychain endpoint
   bearer; supervisor and Client read it directly and selector does not receive
   it. The separate provider-secret Keychain group/backend is held only by the
   signed installer and supervisor; ensure-running and the config-mutation hook
   synchronously refresh live brokers from the frozen worker config, with a
   periodic supervisor refresh as a safety net. Before constructing scopes,
   the supervisor records each exact record's highest generation and digest in
   the no-secret `credential-state` authority using the Store atomic publisher;
   restart rollback or same-generation collision fails closed. Build acceptance
   requires both `codesign --verify` and execution of the app's side-effect-free
   build probe; profile/application-id/access-group mismatch fails before
   publication.
2. **External installer/selector.** Implement the `tekes-selector` binary with
   the deployment-v1 CLI/state contract, signed staged publication, directory
   sync, atomic verified stable-selector update, canary attribution, short-lock
   per-launch observation and automatic last-known-good rollback outside Kernel
   crates; the explicit deployment controller submits/attests candidate canary,
   while ordinary restarts need health only. Acceptance: gates 73–74.
   Observation retains the canary session/run pair, serializes/reset promotion,
   and gives automatic rollback a serve-authored operation identity with no
   stdout response.
3. **Operations and UAT.** Add redacted logs/metrics and the production
   offline `tekes-supervisor support-bundle` command, plus the
   isolated clean install/reboot/target-user-login/Client/archive/unarchive/
   uninstall job, including provider-secret ACL, rotation, revocation and
   no-leak evidence, plus generation-floor retention after provider-item
   deletion. The LaunchAgent gate does not claim pre-login residency.
   Acceptance: gates 75–76, then `scripts/ci-slice10.sh` from a clean checkout.

## Slice 11 increments

1. **Package/catalog authority.** Add `profile::ResourceCatalog`, derive
   whole-directory skill winners and companions from `InstructionSnapshot`,
   bind the worker's existing model skill catalog to the same package bytes,
   and lock the migration oracle. Acceptance: gate 77.
2. **Command expansion and capability.** Parse top-level command files, apply
   the closed quoting/substitution grammar, and add the supervisor-owned
   format-1 `ClientResourceService` with exact list/run DTOs. Acceptance:
   gates 78–79.
3. **Production keyed seam.** Adapt command runs to the existing
   `SessionDeliveryAuthority` using the exact origin tuple and non-steer prompt;
   register its exact DTO validator through `ProductionEndpointRoutes`, and
   compose independent Slice handlers with collision-rejecting
   `CompositeProductionEndpointRoutes`. `run_daemon_inner` captures the
   boot-frozen user/configured-project catalog and mounts that service in the
   same composite. Prove additive dispatch never changes or overlaps
   `TEKES_V2_ROUTES`. Acceptance: gate 80, then `scripts/ci-slice11.sh` from a
   clean checkout.

## Slice 13 increments

1. **Contract and peers.** Freeze `mcp-runtime-v1`, implement bounded JSON-RPC,
   stdio and HTTP transports, legacy/modern negotiation, typed catalogs and
   operations. Acceptance: gates 86–87.
2. **Ownership and projection.** Add the supervisor broker/pool, authorization-
   partitioned generations, dynamic launch-binding projection and causal
   search integration. Acceptance: gates 88–90.
3. **Management and recovery.** Add the canonical registry transaction,
   secret-free typed management, plugin ownership seam, OAuth credential
   references, and closed crash/reconnect matrix. Acceptance: gates 91–92,
   then `scripts/ci-slice13.sh` after rebasing onto published Slice 12.
