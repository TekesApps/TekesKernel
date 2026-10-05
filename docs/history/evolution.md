# External evolution and extension boundaries

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

*(Rewritten after review R3. The earlier claim — "the kernel needs no new
mechanism to evolve itself" — was false: honest self-hosted evolution needs an
immutable version store, activation records, canary routing, evidence
binding, an independently protected constitution, and a bootstrap watchdog.
That is a fifth subsystem. v1 does not build it; it already exists outside.)*

## The ruling

**Evolvable surfaces are immutable artifacts under config/code authority** —
prompts, skills, seed whitelists, tool binaries and manifests, workflow
scripts, config, worker/supervisor binaries, schema definitions.
Explicitly *not* evolvable: liveness, secret values, thread truth, and the
external world (R3-23).

For v1, the machinery of evolution lives **outside the kernel**, in tools
that already exist and are already trusted:

| Need | External authority |
|---|---|
| Immutable, content-addressed version store | git |
| Candidate isolation | branches / worktrees (outside the storage root — `staging/` is rewrite-only, R3-24) |
| Conformance gate | CI running the conformance suite against a **separately pinned constitution version** — a candidate cannot weaken its own judge because the judge is not in its write authority (R3-18) |
| Promotion / activation | deployment: human-approved for binaries and schema; scripted for low-risk artifact classes |
| Canary routing / weighted rollout | the deployment tool; the kernel only needs attribution (below) |
| Supervisor binary activation + last-known-good fallback | the resident external launcher (`tekes-selector serve`, itself kept alive by launchd) with durable per-launch observation and automatic rollback — a dead supervisor cannot relaunch its own predecessor (R3-19) |

## What the kernel contributes (substrate, not subsystem)

Three pieces, each independently justified:

1. **`run_start` attribution (R3-17).** Appended and synced at every lock
   tenure: `{run_id, binary version, config digest, instruction-snapshot
   digest, policy}`. Attribution is by total order — every event belongs to
   the latest preceding `run_start`; no per-event field. This is what makes
   canary evidence, fitness projections, and plain debugging attributable
   per run, since one line file outlives many worker processes.
2. **Materialized instruction snapshots (R3-20, D-55).** One
   content-addressed snapshot per launch; its digest is in `run_start`.
   Reproducibility for evolution and for ordinary debugging alike.
3. **Downgrade gates (D-49).** Old binaries meeting newer files go
   read-only fail-closed; promoted schema cannot corrupt through a stale
   worker.

Fitness stays what it was: **named, versioned projections over usage/settle
events**, now joined through `run_start` — computable by anything that can
read the logs, including external CI.

For the first macOS packaging, “external” is a strict authority boundary, not
an absent actor: launchd keeps `tekes-selector serve` resident; selector owns
only installed selection, child-process lifetime, attribution observation and
rollback. It passes a frozen selection to the supervisor and may read
health/canary evidence, but it cannot author thread, config, protocol, secret or
other Kernel semantic truth. Activation while serve is live is rejected; the
installer first boots out the job and uses its own durable transaction for the
immutable install signing identity, Keychain credential and lifecycle effects;
selector operations recover only selection changes. The installer then safely
bootstraps the same external launcher again. Candidate canary input
has one explicit external author: the deployment controller uses the ordinary
Client authority, then selector read-only attests that session/run; ordinary
restart does not require or invent a canary event.

## One act, three names

Evolution, extension, and plugins are physically the same act: write files,
let a gate admit them. They differ on three dimensions the gates key on:

- **Authorship (trust)** — plugin: third-party (signature/validation);
  extension: the user; evolution: the system itself, which is exactly why
  its promotion authority sits outside the thing being promoted.
- **Seam** — the artifact must land on an existing seam (ToolBackend, daemon
  socket, hook contract, stdio MCP). Fitting no seam means requesting a new
  seam — a D-51 decision, not an install.
- **Write authority** — an admitted process speaks a protocol through a
  seam; it never gains truth-file authority (D-4). Agent edits to project `.agents/`
  are ordinary supervised work under tool policy and user visibility — a
  customization channel, not autonomous promotion (R3-16); an autonomous
  loop that wants its changes activated goes through the external pipeline
  like any other contributor.

## The internalization path (explicitly deferred)

If self-hosted evolution is ever wanted, it is a named fifth subsystem, not
an extension of existing gates: immutable authority versions + append-only
activation records (source, evidence, constitution hash, routing weights,
origin key), a canary router, a fitness evaluator, and bootstrap recovery —
with its own review rounds. Until then, the honest sentence is: **the kernel
is evolvable; it is not self-evolving.**

## Conformance seed

The normative suite is [Conformance gates by topic](../verification/gates/README.md); tests 25–27
(PromotionCrashMatrix, ConstitutionIsolation, SupervisorFallback) are owned
by the external pipeline's own test surface.
