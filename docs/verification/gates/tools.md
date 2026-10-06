# Tool, plugin and MCP gates

[All gates](README.md) · [Running tests](../running-tests.md)

This page retains the gates' original numbers and applicable baselines. It defines conformance rules, not the results of this run.

## Production tool runtime

59. ⬢ **ToolDispatcherCatalogCoverage** — Setup: the fixed manifest and
    tool-runtime case registry. Action: construct every availability profile
    and dispatch every advertised name. Assert: exactly one backend/effect per
    fixed entry, all 27 covered in both directions, dynamic collisions reject,
    and no unavailable tool advertises or returns fake success. `[tool-runtime-
    v1 §One dispatcher/§Backend mapping; builtin-tools]`
60. ⬢ **ToolWriteAheadCrashMatrix** — Setup: one call for each backend/effect
    class. Action: crash at validate → tool_call barrier → hooks → conditional
    effective_execution barrier → revalidation/approval → backend → post-hook
    → total scan → asset → result → next attempt/settle barrier. Assert: one
    result or a durable recoverable unpaired call, no blind side-effect replay,
    and process-local completion proves nothing. `[tool-runtime §Invocation
    identity and durable order; D-19]`
61. ⬢ **ToolHoldStopResume** — Setup: ask/plan call with queued input. Action:
    answer, deny, stop, park/respawn and duplicate response. Assert: request and
    response barriers precede exposure/receipt, same-turn resume is exact, stop
    aborts the call once, and queued non-answer input never releases the hold.
    `[tool-runtime §Fixed-family obligations; tail-lifecycle]`
62. ⬢ **SupervisorToolControlDedup** — Setup: context/task/subagent/report
    supervisor-control requests. Action: drop replies, restart supervisor and
    retry identical/conflicting request ids. Assert: identical retry returns
    the first response, conflict rejects, supervisor authors no tool_result,
    and EOF leaves worker recovery governed by the ledger. `[tool-runtime
    §Supervisor-control extension; worker-control; D-4/D-61]`
63. ⬢ **JobAndHelperLifetime** — Setup: filesystem, shell and detached job
    calls, including absent job cwd and requested writable roots inside/outside
    the immutable policy ceiling. Action: timeout/cancel/worker exit/supervisor
    restart. Assert: absent cwd is the injected primary workspace cwd, only the
    accepted requested roots reach the launcher policy, rejected roots create
    no job directory, descriptor confinement, complete process-group reap for foreground calls,
    stable queryable ownerless job ids for detached calls, and no unknown exit
    becomes success. `[tool-runtime §Backend mapping/§Fixed-family
    obligations; exec-helper]`
64. ⬢ **ToolPolicySecretAndNetwork** — Setup: policy deny, missing sandbox,
    redirect/oversize HTTP, pre-hook mutation and secret-bearing output.
    Action: execute through the single pipeline. Assert: derived policy cannot
    be caller-overridden, mutations are durable, network bounds hold, and
    withheld output never reaches provider/endpoint/log. `[tool-runtime
    §Resource bounds and secrets; tool-hook; sandbox-profile]`

## Plugin lifecycle

81. ◈ **HermeticPluginPackageCarrier** — Setup: the frozen migration manifest
    plus synthetic directory and `.tekesplugin` carriers. Action: inspect,
    descriptor-read, fd-relative expand, validate and install with explicit
    grants. Assert: the manifest
    fields and three component rows are preserved, all projections say
    `launchable:false`, and no helper or MCP tool starts. This hermetic gate
    proves carrier compatibility; it does not qualify the private reference
    archive. `[plugin-package §Package bytes/§Component projection]`
82. ◈ **HermeticPluginSignatureTrustAndGrants** — Setup: synthetic
    accepted/rejected native helpers, unsigned-local and Ed25519
    publisher-attested packages, allowed
    publisher sets, missing and undeclared grants. Action: install and enable.
    Assert: strict helper verification always runs, signed content and
    publisher fingerprint are pinned, required trust fails closed, signed
    manifest-derived receipt fields reject canonical-registry tampering, a
    package cannot self-grant, and enable requires the complete requested grant set.
    `[plugin-package §Integrity and trust/§Store and receipts]`
83. ◈ **PluginLifecycleAndOwnership** — Setup: initial, update, downgrade,
    changed same-precedence versions (including distinct build metadata) and
    two enabled packages claiming one component key.
    Action: install/update/enable/change grants/remove. Assert: downgrade and
    equal-precedence replacement require distinct authorization, grant removal
    disables, remove is idempotent, data remains outside package bytes, and
    `(type,id)` collisions name both owners and fail the whole projection.
    `[plugin-package §Store and receipts/§Component projection]`
84. ◈ **PluginTransactionRecovery** — Setup: crash injection after immutable
    package/data publication but before the operation journal, after that
    journal, and after registry commit but before its phase update for
    install/update/remove/enable/grants, plus controlled publication debris and
    a staging symlink. Action: inspect each fault-window disk state, then reopen
    and recover before another mutation. Assert: exact next receipt means
    commit, every other state rolls back, old/new packages are never jointly
    active, enabled/granted state is exact, data cleanup follows removal,
    controlled temporaries and inert staging/orphans are collected with durable
    parent updates, and symlink cleanup never touches its external target.
    `[plugin-package §Transactions and recovery]`
85. ◈ **PluginTypedManagementNoLaunch** — Setup: every typed management arm
    and canonical registry/operation bytes, plus a generic executable component
    tuple. Action: install, list, set-enabled, set-grants, components, resolve
    `(plugin,component)`, recover and remove. Assert: typed results are
    deterministic, durable files are canonical-plus-LF, enabled components are
    revalidated, resolution returns immutable executable plus package-digest
    generation without launching it, all Slice-12 rows remain inert, and neither builtin catalog nor
    Session Endpoint v2 registrations change. `[plugin-package §Component projection]`

## MCP runtime and management

86. ⬣ **MCPProtocolHandshakeAndFraming** — Setup: legacy and modern fixture
    peers over bounded stdio. Action: negotiate each mode, inject malformed,
    oversized, duplicate-id and unsupported-version frames. Assert: only a
    mutual version becomes ready, initialized precedes catalog calls, and
    every malformed/unknown-required case closes the peer and fails in-flight
    requests. `[mcp-runtime §JSON-RPC and transport bounds/§Handshake]`
87. ⬣ **MCPOperationsAndCancellation** — Setup: peers advertising each subset
    of tools/prompts/resources/tasks. Action: page each catalog, call/read/get,
    continue one multi-round call, cancel one request, and exhaust every
    bound. Assert: unsupported differs from successful-empty, opaque cursors
    cannot repeat, content is preserved, cancellation is terminal locally,
    and no mutation is blindly resent. `[mcp-runtime §Catalogs/§Operations]`
88. ⬣ **MCPStdioAndDaemonPoolOwnership** — Setup: stdio, plugin-native stdio
    and HTTP entries under the one supervisor-owned pool. Action: race peer
    creation, worker exit, last-client release, standalone config replacement/
    disable/remove, plugin update/grant revoke/uninstall, explicit generation
    reconcile and supervisor drain. Exercise both the explicit reconcile API
    and the independent per-effect fail-close check; the implemented Slice-14F
    plugin mutations call reconcile after their durable commit.
    Assert: ownership follows the contract, one generation wins per complete
    pool key, every child is reaped, and a dead generation cannot remain
    advertised. `[mcp-runtime §stdio/§Operations and recovery]`
89. ⬣ **MCPHttpOAuthIsolation** — Setup: two authorization identities and a
    loopback HTTP oracle. Action: send, reconnect, rotate/revoke the sentinel
    credential, hold one same-name OAuth reference pending, complete its exact
    token-free bound transition, attempt redirects and cache reuse. Assert: credentials exist
    only on the send boundary, redirects fail closed, sessions/caches never
    cross identity or generation, and no secret reaches durable bytes or
    diagnostics. `[mcp-runtime §Streamable HTTP/§Configuration]`
90. ⬣ **MCPCatalogProjectionAndSearch** — Setup: resident/deferred catalogs,
    annotations, UTF-8 names, fixed/dynamic collisions, and a Slice-12
    `(plugin,component)` MCP relation. Action: resolve the immutable package
    generation through the automatic read-only plugin projection, project and freeze launch bindings,
    invalidate the live catalog generation and binding cache, and invoke
    `tool_search`. Assert: escaping/digests/effects are byte-exact, collisions
    fail the whole launch, the running binding is immutable, and deferred
    offers are causal. `[mcp-runtime §Catalogs, names, and dynamic launch binding]`
91. ⬣ **MCPManagementPublicationRecovery** — Setup: user/project/plugin
    entries and every publication fault point. Action: list/get/save/remove/
    probe/OAuth reference lifecycle through the production daemon composite,
    retry rpcIds, and restart with an intent. Assert: all seven methods are
    mounted and closed including every nested union; precedence, trust and plugin ownership hold; field
    preserve/replace/remove never embeds secret bytes; responses are secret-free; and recovery
    completes or rolls back the one recorded registry/credential transaction.
    `[mcp-runtime §Management capability]`
92. ⬣ **MCPRecoveryAndPredecessorParity** — Setup: every row of the closed
    loss table plus the frozen AppServer MCP inventory. Action: lose transport
    at every boundary and reconcile. Assert: reads reconnect at most once,
    unproved effects become unknown, resumable identities are queried,
    config/plugin/grant/credential and `list_changed` authority changes close the stale pool key
    and force a fresh generation, and every predecessor
    tools/resources/prompts/tasks/management behavior is implemented or has a
    named intentional replacement. `[mcp-runtime §Operations and recovery;
    Feature parity §Plugin and MCP closure]`
97–100. ⟲ *Removed 2026-09-06 (simplification item 8):* the safepoint
    shadow-git authority, its pre-mutation seam and Client routes were deleted;
    the user's own VCS owns file history. Numbers are retained so later gates
    keep their identities.
101. ⏱ **ScheduleContractOracleCronTimezone** — Setup: the canonical
    definition and cron corpus, including spring-forward omission, fall-back
    repetition and day-of-month/weekday OR. Action: parse and evaluate every
    case against the pinned IANA zone. Assert: bytes, validation, exact UTC
    next occurrence and DST behavior match the oracle. `[schedule
    §Definition and cron]`
102. ⏱ **ScheduleKeyedManagementDurability** — Setup: empty authority and
    durable-failure injection around save/delete/run-now. Action: repeat equal
    and conflicting origin tuples, bind a launch, park/complete it and retry
    historical requests. Assert: receipt follows full sync, equal retries
    return the original result after later changes, conflicts fail closed,
    active runs exclude save/delete/overlap, and list order is stable.
    `[schedule §Files, writer and durability/§Keyed management]`
103. ⏱ **ScheduleClaimRecoveryAndMissedPolicy** — Setup: one due schedule,
    one active run and one overdue inactive definition. Action: crash after
    claim, after keyed launch and after bind; restart and tick through another
    due instant. Assert: the same unbound claim redrives, a bound claim never
    relaunches, an active occurrence is recorded missed, startup skips and
    records the earliest downtime occurrence, and every next deadline is
    strictly after now. `[schedule §Claim-before-launch and recovery/§Poll,
    missed occurrences and timer ownership]`
104. ⏱ **ScheduleFailClosedInternalBoundary** — Setup: every invalid class,
    a torn final line, a malformed complete line and sentinels over thread and
    endpoint bytes. Action: attempt mutations/reopen and scan public route
    registration. Assert: validation and claim mismatches are non-mutating,
    exactly one final partial line repairs before a later append, complete
    corruption fails closed, semantic/client files are untouched, and no
    schedule management route/capability appears before 14F. `[schedule
    §Files, writer and durability/§Errors and boundaries]`
105. ⌕ **ThreadSearchContractOracle** — Setup: the canonical Slice-14D
    request/result/index corpus, including NFKC/full-width input, exact,
    prefix and token matches. Action: scan semantic ledgers, search twice and
    compare the per-thread cache bytes. Assert: stable `(workspace, UUID)`
    identities, byte-exact normalization/ranking/order/cursor, first query
    reports cache misses, second query reports only validated hits, and no
    transcript or child fact is searchable. `[thread-search §Scope and
    authority/§Text normalization and ranking]`
106. ⌕ **ThreadSearchPagingArchiveSnapshot** — Setup: active and archived
    matching threads in two workspaces. Action: page each explicit visibility
    mode, hold the shared catalog lock while archive/create race, exercise the
    rewrite-before-catalog lock order, then continue an old page. Assert:
    score-desc/UUID-asc pages neither duplicate nor skip, membership changes
    cannot cross the snapshot, no lock-order inversion occurs, archived rows
    appear only in their requested mode, and the old cursor fails
    `cursor-stale` after the move. `[thread-search §Source snapshot and locking/§Archive visibility/
    §Cursor and paging]`
107. ⌕ **ThreadSearchRebuildFallback** — Setup: one missing, one stale and one
    malformed per-thread cache. Action: query, delete every cache, query
    again, then run explicit rebuild. Assert: all query results derive from
    the same ledger rows, fallback repairs best-effort, cache deletion changes
    no result, explicit rebuild validates every write, and the next query uses
    only matching caches. `[thread-search §Cache bytes and recovery]`
108. ⌕ **ThreadSearchFailClosedAndInert** — Setup: malformed/foreign/stale
    cursors, invalid queries/limits, a corrupt complete ledger line and
    sentinel semantic/endpoint bytes. Action: search each case. Assert: the
    exact closed error class is returned, no malformed input becomes empty
    success, corrupt source never falls back to cache, semantic and endpoint
    files are byte-identical afterward, a live rewrite reservation excludes a
    colliding create and materialization never copies the rebuildable search
    cache, and no public Client route or
    capability is registered before 14F. `[thread-search §Cursor and
    paging/§Internal authority seam]`
109. ⌁ **WebFetchExtractionChain** — Setup: every canonical HTML/text/non-text
    case and the 24,000/24,001-scalar boundary. Action: run the production
    local extractor. Assert: title/script/entity/line behavior and exact
    result bytes match the oracle, JS shells are diagnostic-only
    `under_rendered`, non-text never embeds raw body bytes, and truncation does
    not split a scalar. `[web-tools §Local extraction]`
110. ⌁ **WebNetworkPolicyAndBounds** — Setup: every special-use IPv4/IPv6
    class, invalid scheme, zero/over-limit bounds, redirect cap and
    cancellation. Action: enter the production HTTP backend. Assert: every
    forbidden address rejects before send, DNS answers are all-public or the
    hop fails, the selected address is pinned, each redirect re-enters the
    same gate, and exactly one bounded attempt occurs. `[web-tools
    §web_fetch production chain]`
111. ⌁ **WebSearchExactWireAndNegativeMatrix** — Setup: the Tavily request,
    response, status and invalid-shape corpora. Action: run the production
    request encoder, response parser, URL classifier and status mapper.
    Assert: bytes and hits are exact, every malformed/private-result case
    rejects, redirects never follow, and unavailable differs from auth,
    invalid and terminal-unavailable. `[web-search-provider §HTTP
    request/§Response and errors; web-tools §web_search production chain]`
112. ⌁ **WebParityRetirementAndSecretLifecycle** — Setup: the pinned
    predecessor disposition plus rotating/revoked broker generations. Action:
    execute two production search calls, revoke between them, and inventory
    production/config surfaces. Assert: each call resolves the current exact
    scope, a revoked id fails the next call closed without credential leakage, all
    retained behaviors have production proof, and Jina/Tavily-extract,
    synthesized-answer metadata, environment credentials and every hidden
    reader capability remain absent. `[web-tools §Predecessor
    disposition/§Oracle and internal seam]`
113. ⊕ **ClientExtensionCatalogNegotiation** — Setup: the frozen 16-method V3
    base, every v1 extension group and the Client driver catalog. Action:
    compose the production registry in different orders and remove/collide one
    method. Assert: complete groups match byte-for-byte in both directions,
    partial groups and collisions fail startup, unavailable methods never
    return empty success, and no SessionEvent/mux family changes.
    `[client-extensions §Frozen base and negotiation]`
114. ⊕ **ClientExtensionDTOAuthorityAndIdempotency** — Setup: every canonical
    request/result/error case and every authority fault point. Action: execute
    reads and mutations, crash after the durable commit before response, then
    retry equal and conflicting rpcIds. Assert: closed DTOs and error sets are
    exact, success follows the owning barrier, equal retries re-ack, changed
    bytes conflict, and no extension invents a second durable truth.
    `[client-extensions §Resource and tool catalogs through §Usage
    projection]`
115. ⊕ **ClientExtensionPredecessorDisposition** — Setup: the pinned
    AppServer/Runtime route inventory and both production/Client registries.
    Action: diff all three in both directions. Assert: every predecessor row
    is base, implemented, replaced, or retired; every retained method has one
    owner; every retired name is absent and fails typed; and no Computer Use,
    marketplace, remote-Git, compatibility alias, or fake-empty route enters
    the Kernel catalog. `[client-extensions §Permanent dispositions]`
116. ⊕ **ClientExtensionCrossCapabilityLifecycle** — Setup: an enabled plugin
    MCP peer, a scheduled definition with policy/profile choices, archived
    search/usage state, and revoked config/grant generations. Action: mutate
    plugin state, save/run the schedule, query archive projections, and revoke
    each authority. Assert: MCP reconcile follows the plugin commit, schedule
    values are validated and applied rather than ignored, archive behavior is
    exact, connection readiness is independent of zero/multiple/disabled
    profiles, a workspace policy may relax within but never exceed its
    effective ceiling, and stale generations fail closed before an effect.
    `[client-extensions §Plugin and MCP management/§Schedule and
    search/§Provider and workspace administration]`
