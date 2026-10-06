# Feature parity and qualification

[Verification · documentation home](../README.md) · [Verification entry point](README.md) · [Next: provider parity](provider-parity.md)

The status matrix is tied to its explicit comparison baseline below; it is not an
acceptance report for arbitrary later worktree changes. Current package ownership,
V3 transport and validation call paths are in [Code architecture](../architecture/README.md).
Historical V2 registration-count claims apply to their original Slice baseline.

This document records the **implemented** feature relationship between the
Kernel and the stack it replaces. It is a status and closure registry, not a
second event or wire authority. Executable contracts under `spec/` remain
normative.

The comparison baseline is:

- TekesKernel `daa009cce8deea30af7c335758181e1fb5577bd7`;
- TekesAppServer `e4df9dc94e60025d0e8f62bcf98c95241181793a`;
- TekesRuntime `995d8ffeece88c84502e721c5cf9c30d286a17bd`.

Moving either baseline requires re-running the inventory audit and updating
this document. Narrative migration intent in 09 is not evidence that a
capability is implemented.

## Status vocabulary and parity bar

- **aligned** — the Kernel has an implemented path and conformance evidence
  for the same product behavior.
- **intentional replacement** — the old API/name is absent by design, but the
  supported behavior has a documented replacement.
- **partial** — a useful subset is implemented, but an AppServer/Runtime
  behavior or management surface is absent.
- **deferred** — the Kernel neither implements nor advertises the capability.

Feature parity is behavioral, not a class-name or RPC-count comparison. A
capability is aligned only when its discovery, execution, lifecycle,
permissions/secrets, failure behavior, management surface when product-visible,
and fixtures/gates are all closed. A schema type, snapshot field, migration
row, or test-only fake does not establish parity. Optional capabilities may
remain deferred, but Endpoint registration/capability projection and every
catalog MUST omit them; an empty success response is not a substitute.

## Current capability matrix

| capability | current Kernel mechanism | status | remaining work / boundary |
|---|---|---|---|
| Durable turns, recovery, provider attempts, context projection, compaction, usage | event/tail/provider contracts and Slices 1–10 | **aligned** for the native Kernel architecture | External Gates 72 and 76 belonged to the retired launchd deployment; this does not change feature semantics |
| Fixed model-facing tools | one implemented 27-name catalog and dispatcher | **aligned** | The parity fixture records every legacy fixed name and disposition |
| `write`, `goal_completed`, `compact` | `apply_patch`, `set_goal_state`, host compaction | **intentional replacement** | No aliases or tombstones are exposed |
| Git/GitHub model tools | `shell` invoking system `git`/`gh`; UI Git remains Client-side | **intentional replacement** | No fixed `git_*`/`github_*` names return |
| Filesystem/exec, shell/job, Web fetch, approval, hooks, memory/goal, task/subagent/report | helper/sandbox, ownerless jobs, tool pipeline and worker-control | **aligned** for the fixed catalog | Generic plugin and MCP execution are covered by their own rows below; no product-specific dispatch branch is implied |
| Web search/provider chain | `web-tools`: bounded public fetch + deterministic local extraction, and credential-gated `tavily_v1` search | **aligned with explicit retirements** | Public fetch/SSRF/HTML extraction/search hits are production-qualified by Gates 109–112; Jina/Tavily-extract fallbacks, synthesized-answer metadata and environment-owned policy/credentials are permanently retired and absent |
| User/project instruction files | deterministic two-level immutable instruction snapshot | **aligned** for `AGENTS.md`, settings, and hook capture | Snapshot bytes and precedence are already normative |
| User/project skills | immutable whole-directory packages, companion resources, `skill_explorer`/`skill`, `skills/list`, and unified `resources.v1` reads | **aligned** for user/project sources | Gates 77–80 and 113–116 bind immutable snapshots and Client reads; package-defined skill components are not silently treated as user/project packages |
| Slash commands | immutable command catalog/expander plus versioned `commands/list`/`commands/run` through ordinary keyed input | **aligned** for user/project files | MCP prompts remain an MCP capability rather than a slash-command alias; v1 intentionally leaves file/shell directives literal |
| Local external tools | MCP stdio servers (`mcp-runtime`); the `dynamic-helper-v1` discovery/execution lane is removed | **aligned** | The deferred (`tool_search`) and parallel-tools live gates run against the fixture MCP stdio server's `local-tools` scenario (`mcp__local__*`) |
| Tool introspection | `tools.v1` over immutable fixed and dynamic catalogs | **aligned** | `tools/list` and `tools/resolve` are read-only Client extensions; they expose no credentials and revoked generations fail closed |
| Plugin packages and marketplace | `plugin-package` lifecycle plus `plugins.v1` Client management | **aligned with explicit retirements** | Gates 81–85 and 113–116 close inspect/install/remove/enable/grants/components/recovery; marketplace fetching and distribution-policy administration are permanently retired |
| MCP runtime | `mcp-runtime` stdio and HTTP/OAuth pools plus `mcp.v1` management | **aligned** | Gates 86–92 and 113–116 close negotiation, tools/resources/prompts/tasks, cancellation, credentials, reconnect and all seven management methods |
| Plugin-owned MCP servers, tools, and resources | generic `(plugin,component)` generation binding into the MCP runtime | **aligned** | Only an enabled, fully granted, revalidated `mcp-server` component launches; tool/resource identities remain generation-bound and collision checked |
| Other plugin component kinds: skills/commands/hooks/external tools/apps/native helpers | validated manifest entries and inert component projection | **partial by explicit carrier boundary** | No runtime path is inferred from a component type. v1 claims execution only for the generic MCP relation; other package components remain non-launchable unless a separately versioned carrier exists |
| Brief (strict, reference-native brief) | one `brief` schema, no version field and no policy switch; the worker renders the host-owned source alias table whenever a turn admits inputs and `brief` is callable; `engine::read_brief` derives the effective completion | **aligned** | Live: `run-public-flow.py --live --case text --brief-sources` on the Cloudflare OpenAI route (2026-09-05) |
| Compaction summary | frozen source bundle, compactor-role `summary_artifact` request, evidence-address admission, `compact.summary_request` telemetry; the admitted continuation is the summary and the deterministic quoted history the fallback (no mode setting) | **aligned** | Live `run-public-flow.py --live --case text --compaction` (four private codes recalled across the supervisor-authored manual compact, 2026-09-06); the 50 % prefetch band is not implemented and a worker-authored manual compact at a yield carries no summary request |
| OAuth refresh-token lifecycle for MCP | Production injects platform-installed tokens as-is; the launching application owns minting, rotation and revocation. Kernel-minted grants (`SecretMutationAuthority`, `provider::OAuthTokenExchange` in the HTTP authorization provider, rotation write-back, HTTP 400 → revoked record) run only where a mutation authority is installed: tests and explicit embedders | **partial** | Production startup installs no mutation authority since credentials became application-owned; the live `run-live-mcp-oauth.py` run against Cloudflare (2026-09-05: rotation ×2, RFC 7009 revocation) predates that change |
| Computer Use reference MCP | signed TekesComputerUse 0.1.6 executable configured as an ordinary local stdio MCP server with 12 tools | **aligned through the generic MCP host** | Gates 117–120 and the signed safe preflight close configuration, discovery/call, denial, lifecycle and recovery without a plugin binding or TCU branch; interactive user-authorized TCC grant/call remains release UAT |
| Safepoint shadow-git create/list/restore | removed 2026-09-06 (simplification item 8): DSH keeps no file snapshots and the Tekes client never called the Kernel routes | **removed** | the user's own VCS owns file history |
| Schedule/cron service | `schedule` authority plus `schedule.v1` Client capability | **aligned** | Gates 101–104 and 113–116 close persistence, cron/IANA evaluation, run-now, redrive and policy/profile validation |
| Thread search | `thread-search` authority plus `threadSearch.v1` Client capability | **aligned** | Gates 105–108 and 113–116 close title search, active/archive visibility, stable cursor bytes and rebuildable projection; transcript/body search is not implied |
| Session Endpoint inventory/history/live/models/prompt/queue/cancel/rename/archive/unarchive/fork | exact native 20-registration v2 carrier | **aligned** | The base remains byte-frozen and isolated from extensions |
| Session Endpoint optional product extensions | nine atomic versioned capability groups with 40 methods | **aligned** | Gates 113–116 prove `.tekes` Client/production-registry parity, closed DTOs, authority binding, idempotency and exhaustive predecessor disposition without changing the base 20 or `SessionEvent` |
| Agent presets/types and thread-agent inventory | instruction snapshots, workspace policy and explicit child topology | **intentional retirement** | Opaque preset/type routes are absent from both Client and server catalogs; the disposition fixture permanently retires them |
| Provider/model wire dialects | twelve exact proof rows, eleven advertised route tuples | **aligned for the closed v1 registry** | Gates 93–96 bind exact request/stream/terminal/recovery/control bytes; the generic chat fixture is test-only and opt-in live route smoke remains supplemental |
| Provider connection and model-profile administration | the launching application supplies provider configuration in the launch document; `providerAdmin.v1` was removed with the launchd deployment | **intentional replacement** | No Kernel route edits providers or the default model |
| Permission mode and workspace capability overrides | immutable enforcement plus `workspacePolicy.v1` get/set | **aligned with explicit retirement** | Workspace policy is implemented; the predecessor's opaque permission-mode/preset catalog is permanently retired and cannot grant beyond deployment, plugin, credential, TCC or OS limits |
| Resources and composer references | `resources.v1` unified immutable skill and generation-bound MCP list/read | **aligned** | Gates 77–80, 86–92 and 113–116 close resource identity, paging, stale generation and bounded content; no Kernel-private URI rewrite is introduced |
| Sidechat | `session.fork {ephemeral:true}` plus ordinary keyed `session.prompt`, closed by `session.discard` | **aligned** | An ephemeral fork is a scratch ledger: never workspace membership, refused by archive, swept at daemon startup; no separate sidechat authority exists |
| Feedback list/put/delete | Client-owned local product metadata | **intentional retirement** | Feedback is not AS durable truth and no empty-success endpoint placeholder exists |
| Git Changes/branch/commit/PR Client surface | Client-local Git UI and model-facing `shell` | **intentional retirement for the built-in local Kernel** | Remote Kernel Git UI/RPC parity is not claimed and no remote Git extension is advertised |
| Usage summary and cache attribution queries | durable usage ledger projections plus `usage.v1` | **aligned** | Gates 113–116 close archived reads, exact route attribution, decimal aggregation and corrupt/unavailable behavior |
| Thread debug and diagnostics query | readiness diagnostics; the Slice-10 support bundle was removed with the launchd deployment | **intentional replacement** | The predecessor's interactive debug query shapes are absent |
| Legacy freeze/read, offload/restore, context query, direct shell and compatibility aliases | v2 history/mux, always-file folders, fork/prompt/recovery and model-facing shell | **intentional replacement** | Gate 115 fixture-locks every predecessor route as base, implemented, replaced or retired; no compatibility alias returns an empty success |

## Skill package compatibility

Slice 11 implements [skill-package](../../spec/skill-package.md): one named
directory contains `SKILL.md` plus scripts, references and assets; precedence
replaces that directory as a unit and all reads use one immutable snapshot.
Flat pre-Slice-11 skill sources remain model-tool compatibility inputs but are
not advertised as packages. `skills/list` belongs to the independent resource
capability and does not alter the native endpoint's frozen 20 registrations.

## Command execution

[command-catalog](../../spec/command-catalog.md) now freezes parsing, front
matter, argument substitution/escaping, precedence, list/run DTOs and
expansion-to-keyed-input idempotency. `commands/run` calls the same production
delivery authority as an ordinary non-steer prompt and creates no side channel.
MCP prompts, plugin commands, filesystem interpolation and shell execution are
not inferred from legacy syntax.

## Plugin and MCP closure

The generic words `plugin` and `mcp` in launch-bindings and builtin-tools
reserve collision/provenance space; they do not by themselves make a source
executable. Slices 12 and 13 close the only package-owned executable relation
claimed by v1:
an enabled, fully granted, revalidated `mcp-server` component bound by immutable
plugin and server generations.

Parity requires two independent executable contracts:

1. `spec/plugin-package.md`: immutable package/receipt bytes, signature and
   trust policy, install/update/remove/enable/grants transactions, component
   types, owner/collision rules, data directories, rollback, capability
   projection, and management fixtures.
2. `spec/mcp-runtime.md`: stdio and HTTP/OAuth ownership, initialization and
   capability negotiation, tools/resources/prompts/tasks, schema deferral,
   deadlines/cancellation/reconnect, credential delivery, daemon pooling,
   management operations, and crash/restart fixtures.

The existing Computer Use executable is a black-box reference consumer of the
generic MCP contract and the signed-process/TCC lifecycle. It is not a plugin
package and must not be used as the mechanism that defines either contract.
Its migration boundary is frozen as follows:

- the existing `TekesComputerUse` tool schemas, executable identity, and 12 MCP
  tools are migration inputs, not redesign inputs;
- after Slices 12, 13, and 14F completed, Slice 14A configured and launched
  the unchanged executable through only ordinary local stdio MCP configuration and
  black-box qualifies its 12 tools, signature, TCC denial, stop/restart, and
  recovery; the real lane pairs the production executable's LaunchServices
  lifecycle with a distinct, non-distributable signing-identity canary for the
  denied path, without requesting or resetting TCC;
- a qualification failure may repair only generic MCP/process/TCC lifecycle;
  a TCU-specific protocol, DTO, dispatcher, execution route, or branch is a
  failed architecture gate;
- Rust MUST NOT reimplement Accessibility traversal, screenshots, click,
  drag, key press, scroll, text/value mutation, or secondary actions. Those
  operations remain owned by the existing native helper.

## Capability-closure sequence after Slice 10 (completed)

The plan stops at **Slice 14**. Slices 11–14F and the Slice-14A generic
reference-MCP lane now have their named contracts, canonical fixtures,
negative corpora and doc-14 gates. This is a closure record, not authority to
invent a Slice 15 or broaden an implemented contract without versioning it.

The completed scheduling graph kept Slice 11 independent, developed Slices 12
and 13 from a common published baseline, converged 14B–14E into 14F, and ran
14A last. Parallel development did not mean parallel publication: merges were
serialized and each lane updated to the latest `main` before its cross-lane
gates.

### Slice 11 — Skills + Commands (implemented)

Gates 77–80 close whole-directory user/project skill packages, companion
reads, command expansion and ordinary keyed-input submission. The independent
resource capability does not change the frozen Session Endpoint v2 base.

### Slice 12 — Plugin lifecycle (implemented)

Gates 81–85 close `.tekesplugin` validation, signing/trust, install/update/
remove/enable/grants, collision ownership, immutable component generations and
recovery. Slice 14F supplies the separate `plugins.v1` Client management
surface; only the later generic MCP relation makes an executable component
launchable.

### Slice 13 — MCP runtime + management (implemented)

Gates 86–92 close stdio and HTTP/OAuth transports, daemon pooling,
initialization, tools/resources/prompts/tasks, credentials, cancellation,
reconnect and recovery. Slice 14F supplies `mcp.v1`; no product-specific server
path exists.

### Slice 14 — Optional product parity closure (implemented)

- **14B–14E:** Gates 101–112 close schedule, thread search and the
  retained Web provider chain. Jina/Tavily-extract fallbacks,
  synthesized-answer metadata and environment-owned reader credentials remain
  explicit retirements.
- **14F:** Gates 113–116 bind nine atomic capability groups and 40 methods to
  production authorities, preserve the exact base 20, and fixture-lock every
  predecessor route as base, implemented, replaced or retired.
- **14A:** Gates 117–120 qualify signed TekesComputerUse 0.1.6 as an ordinary
  local stdio MCP server: exact 12-tool discovery/call, signing, hermetic TCC
  states, real denial, stop/restart and peer-loss recovery all use the generic
  MCP path without a plugin binding. The safe real lane locks the external source revision,
  production signing tuple, migration-tool contract and exact projected catalog
  digest without pinning timestamped archive bytes. Interactive user-authorized
  TCC grant/call remains release UAT and is not claimed by these automated gates.

Every Slice-14 item is now either implemented with executable contracts and
gates or explicitly retired with its old capability omitted and migration
behavior documented. Release qualification still separately requires the
named external Gates 72/76 and the interactive Computer Use granted-path UAT.

## Re-audit rule

Before declaring replacement parity, generate and diff all of the following
against the pinned source revisions:

- AppServer public routes and capability identifiers;
- Runtime and AppServer model-facing tool names;
- skill, command, hook, external-tool, plugin and MCP configuration producers;
- first-party plugin component inventories;
- Kernel advertised endpoint methods, fixed/dynamic catalogs and production
  backend registrations.

The route inventory is checked in both directions. Grouping several routes
under “Client extensions” without one disposition per behavior fails the
audit.

Every source item must resolve to **aligned**, **intentional replacement**, an
explicit retirement, or a narrowly stated non-advertised carrier boundary. An
undocumented omission blocks a parity claim even when all numbered gates
remain green.
