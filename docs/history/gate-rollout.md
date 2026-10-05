# Historical stage-to-gate mapping

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

## Slice 1

Build order (R4/R5): the event-schema/replay library, then a minimal
worker shell with a fake provider and a stub supervisor core — canonical
encoding, version gates, append/barrier behavior, torn-tail repair, flock
liveness, keyed create/delivery, `run_start`, attempt/usage settlement,
D-61 recovery runs, protocol negotiation, resume-policy encoding, EOF
recovery. The ▸-marked tests (1–6, 12–14, 17–19, 28–34) gate the slice —
fake ToolBackend/ProcessHost harnesses are explicitly permitted where a
test needs a tool or child (6, 12); no real tools, dialects, instruction
resolver, checkpoint acceleration, or UI.
The language-neutral component graph, gate order, fixture rules, CI semantics,
and deferred-contract registry are planning inputs in
[Phased implementation plan](implementation-plan.md); the current Rust
mapping is [Rust: toolchain and implementation constraints](../architecture/rust.md).

## Slice 2

Slice 2 replaces the injected launch-profile seam with the two executable
contracts: versioned config authority and deterministic instruction capture.
The ◆ gates run in order 35, 37, 36, 20, 21 after the complete Slice-1 gate
set. The combined job must also pass the real supervisor→worker inherited-FD
integration: the worker validates both immutable assets before its line lock
and records their exact digests/effective policy in `run_start`.

## Slice 3

Slice 3 activates [checkpoint-v1](../../spec/event.md) without changing
the ledger's authority or the compatibility path. The ◇ gates run in order
38, 39, 40, 41, 42 after the complete Slice-2 job. The combined job proves
the byte oracle and conditional barrier first, then seeded/full equivalence,
fallback behavior, retention-floor semantics, and the publication crash
boundary. Full replay remains available for legacy and unsupported formats.

## Slice 4

Slice 4 activates the fixed tool catalog and execution substrate contracts:
[builtin-tools-v1](../../spec/builtin-tools.md),
[tool-hook-v1](../../spec/tool-hook.md),
[exec-helper-v1](../../spec/exec-helper.md), and
[sandbox-profile-v1](../../spec/sandbox-profile.md). Gate 43 runs after the
complete Slice-3 job, followed by executable substrate gates 44–46 and
tool-specific semantic tests 7–11.
The combined job must prove the catalog first so a green helper test cannot
mask an omitted, shadowed, or wrongly classified fixed tool. Real process and
platform cases then prove write-ahead pairing, descriptor-first filesystem
access, cancellation/reap, and fail-closed sandbox behavior.

## Slice 5

Slice 5 activates
[rewrite-publication-v1](../../spec/rewrite-publication.md). The ⬟ gates run
15, 16, then 24 after the complete Slice-4 job. They prove the materialized
projection and carrier closure first, every fork/redact phase and action-before-
record crash window second, then operation-record ownership versus recordless
staging GC. A green gate 24 cannot compensate for a projection or retirement
failure in 15/16.

## Slice 6

Slice 6 activates
[endpoint-projection-v2](../../spec/session-endpoint.md). The ● gates run
47, 48, 49, 50, 51, then 52 after the complete Slice-5 job. They prove the
imported Client bytes before Kernel projection, durable 1:N identity before
history/live, typed native mutations and inventory separation, and finally
bounded reconnect/fail-closed behavior. The workspace parity lane also runs
`scripts/check-endpoint-authority.py --require-source`.

## Slice 7

Slice 7 activates [provider-runtime-v1](../../spec/provider-runtime.md),
[credential-broker-v1](../../spec/credential-broker.md), and
[secret-store-v1](../../spec/secret-store.md) over
the existing provider-adapter recovery contract. Slice 7 proves the injectable
store/record and broker behavior; the signed macOS Keychain backend is a Slice
10 Gate-76 obligation. The ◉ gates run 53–58 after
the complete Slice-6 job. No provider/model is advertised until its complete
dialect parity set passes; real credentials are used only in an opt-in smoke
lane after the hermetic byte/recovery gates.

Provider dialect closure runs as gates 93–96 in parallel with Slices 12/13 and
must be green before Slice 14F advertises provider/model administration. It is
not a renumbered Slice and does not turn family gates 53–58 into vendor parity.

## Slice 8

Slice 8 activates [tool-runtime-v1](../../spec/tool-runtime.md) and
[worker-control-v2](../../spec/worker-control.md). The ⬢ gates
run 59–64 after the complete Slice-7 job. Slice 4's catalog/hook/helper/sandbox
substrate remains the prerequisite; Slice 8 is the production worker/supervisor
dispatch closure for all 27 fixed tools and applicable dynamic catalogs.

## Slice 9

Slice 9 activates [endpoint-management-v2](../../spec/session-endpoint.md)
and [endpoint-transport-v2](../../spec/session-endpoint.md).
The ◎ gates run 65–70 after the complete Slice-8 job and the Client authority
byte-sync. The real Tekes Client is part of gates 68 and 70; a transport-only
mock cannot establish readiness.

## Slice 10

Slice 10 activates [deployment-v1](../../spec/deployment.md). The ◆◆ gates run
71–76 after the complete Slice-9 job on macOS 15+ local APFS. In-repository
selector gates own 73–74; the signed repository-built production coordinator
app plus an explicitly injected signed Client UAT app additionally prove 72
and 76 against launchd, Keychain, the installed artifacts, reboot/login and the
real Client. Their closed argv and canonical replies are locked by
`fixtures/deployment/production-uat-contract.canonical.json`; the exact
embedded-profile/application identifiers and runtime-admission probes are
locked by `fixtures/deployment/provisioning-profiles.canonical.json`. All are part of
the Slice-10 release decision; the Kernel still owns no evolution authority.

## Slice 11

Slice 11 activates [skill-package-v1](../../spec/skill-package.md) and
[command-catalog-v1](../../spec/command-catalog.md). The ✦ gates run 77–80
after the complete Slice-10 repository gate set. The resource capability is
versioned independently and the exact 20-registration Session Endpoint v2
authority remains unchanged. Gate 79 proves deterministic catalog/run behavior
with a keyed fake, required `arguments`, and fail-closed metadata aliases;
Gate 80 alone proves the real daemon assembly advertises and executes the
resource routes, the production delivery adapter, frozen-route overlap
rejection, and registry separation. Plugin and MCP sources remain absent.

## Slice 12

Slice 12 activates [plugin-package-v1](../../spec/plugin-package.md). The ◈
gates 81–85 run in `crates/plugins` through `scripts/ci-slice12.sh`. An
explicit `TEKES_APPSERVER_ROOT` adds historical byte comparison with the
predecessor sibling manifest and strict macOS `codesign` validation of its
legacy
`.tekesplugin` archive. Without that input the script prints
`NOT QUALIFIED`; a qualification job sets `TEKES_SLICE12_REQUIRE_REFERENCE=1`
so absence exits nonzero. Ordinary hermetic CI never reports reference
qualification from the frozen local fixture. This optional lane is migration
evidence and creates no current Computer Use plugin binding. These gates prove
package carriage and lifecycle only. Slice 13 supplies MCP runtime activation, Slice
14F now binds Client management, and Slice 14A now closes the ordinary local
MCP Computer Use black-box obligation subject to the separately named
interactive granted-path UAT.

## Slice 13

Slice 13 activates [mcp-runtime-v1](../../spec/mcp-runtime.md). The ⬣ gates run
86–92 after the published Slice-12 baseline and every applicable earlier gate.
Parallel Slice-12/13 development is permitted only from the same published
baseline; the lane merged second rebases and runs the cross-carrier plugin-MCP
ownership cases. Generic MCP became advertisable after all seven gates. Slice
14F now binds its Client management capability, and Slice 14A treats the
unchanged Computer Use executable and tool implementation as an ordinary local
stdio MCP black box; any plugin binding or TCU-specific Kernel path fails the
gate.

## Slice 14B

Slice 14B activates [safepoint-v1](../../spec/tail-lifecycle.md). The ⟲ gates run
97–100 through `scripts/ci-slice14b.sh` after the published Slice-13 baseline.
They close the internal pre-tool and full-tree create/list/restore authority,
private Git ownership, transaction recovery and path/process safety. They do
not extend the frozen Session Endpoint registration set; Slice 14F now binds
the public Client capability separately.

## Slice 14C

Slice 14C activates [schedule-v1](../../spec/schedule.md). The ⏱ gates run
101–104 through `scripts/ci-slice14c.sh`, after the published Slice-13
baseline. They close the internal global JSONL authority, cron/IANA-time-zone
evaluation, keyed management, durable claim-before-launch, restart redrive and
the explicit `skip_and_record` downtime policy. The supervisor owns this seam;
Slice 14F now advertises the separate public Client capability without changing
the base registration set.

## Slice 14D

Slice 14D activates [thread-search-v1](../../spec/thread-search.md). The ⌕
gates run 105–108 through `scripts/ci-slice14d.sh`, after the published
Slice-13 baseline. They close only the internal title-search authority:
stable result identity and cursor bytes, explicit active/archive visibility,
catalog membership locking, and rebuildable stale/corrupt-cache fallback.
The lane does not edit the frozen 20-route Session Endpoint set; Slice 14F now
binds the separate public search DTO, route and negotiation.

## Slice 14E

Slice 14E activates [web-tools-v1](../../spec/web-tools.md) over the existing
[web-search-provider-v1](../../spec/web-search-provider.md). The ⌁ gates run
109–112 through `scripts/ci-slice14e.sh` after the published Slice-13
baseline. They execute the production extractor, HTTP classifier and Tavily
wire/credential paths before checking the closed predecessor disposition.
The lane itself registers no public route; Slice 14F now binds generic Client
tool introspection. Retired external readers cannot be inferred from an
under-rendered diagnostic.

## Slice 14F

Slice 14F activates [client-extensions-v1](../../spec/client-extensions.md).
The ⊕ gates run 113–116 through `scripts/ci-slice14f.sh` only after Slices
11–13, 14B–14E and provider-dialect proof are merged. The contract preserves
the Client-owned twenty-method Session Endpoint v2 set and adds ten atomic,
versioned capability groups with 43 methods. The gate is complete only when
the `.tekes` Client driver and production registry match in both directions;
contract bytes alone do not advertise a route. Gates 113–116 now pass against
both sides; the subsequent Slice-14A gates pass while treating the unchanged
Computer Use executable and tool implementation only as a generic local MCP
black box.

## Slice 14A

Slice 14A is the implemented final generic local-MCP reference
qualification. The four focused gates run as 117–120 through
`scripts/ci-slice14a.sh` after the full Slice-14F lane:

117. save and enable an ordinary user-scope stdio MCP configuration whose
     executable path is explicit; the real executable must carry the verified
     stable signature identity frozen by the source lock;
118. start that configured server through the Slice-13 pool, discover the
     exact 12-tool frozen first-party inventory in deliberately noncanonical
     wire order, prove each projected argument-property set and effect matches
     the canonical migration contract, and call every tool through its
     name-bound ordinary dynamic `supervisor_control` route;
119. exercise hermetic platform-permission denial and grant states, while
     proving `request_permissions` cannot invoke a real consent UI in CI;
120. prove stop, new-process restart, peer-loss recovery, disable/re-enable,
     removal and a source scan rejecting any Computer Use protocol, DTO,
     dispatcher, execution path, operation, conditional or plugin binding in
     Kernel code.

The hermetic executable is a transport/lifecycle oracle and performs no
Accessibility, capture or input operation. An explicit signed macOS executable is
still mandatory for the safe real-reference preflight. That required lane runs
the production 0.1.6 executable from the source-locked external revision through
real LaunchServices stop, restart and peer loss, verifies its production Team
ID/signing identifier and exact projected catalog digest, then runs a separately
signed qualification-only executable through real denied status and denied read.
The canary uses a
distinct signing identifier and designated requirement, never requests consent,
and is forbidden from product distribution, product installation and runtime
manifests. The legacy 0.1.5 plugin archive is migration evidence only and never
enters the current runtime. Slice 14A qualifies the standalone 0.1.6 executable
by ordinary MCP configuration. No Kernel compatibility branch may compensate for
an artifact mismatch. Interactive TCC grant and a real permission-gated call
remain a final user-authorized UAT and are never claimed by the hermetic lane.
