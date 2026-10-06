# Deployment and external pipeline gates

[All gates](README.md) · [Running tests](../running-tests.md)

> **Retired (2026-10-06).** The Kernel is launched only by its host application; see
> [Application-owned launch](../../builtin-launch.md). The launchd service, product
> installer and selector described here are no longer maintained, and their
> Keychain credential model no longer matches the code: the endpoint token and
> provider secrets come from the host's environment
> ([secret-store](../../../spec/secret-store.md)). The code remains until it is removed
> ([#28](https://github.com/TekesApps/TekesKernel/issues/28)); this document is kept for reference.

This page retains the gates' original numbers and applicable baselines. It defines conformance rules, not the results of this run.

## macOS production deployment

71. ◆◆ **DeploymentFilesystemAndOwnership** — Setup: local APFS, network/
    iCloud path, an HFS production-filesystem seam and two supervisor
    candidates. Action: start through the eight readiness steps. Assert:
    installer creates the root/lock, the supervisor acquires ownership before
    any mutable probe, production accepts APFS and rejects HFS even though the
    generic Store seam supports it, only supported local storage
    becomes ready, exactly one owner/listener exists, repeated
    `ensure-running` against that ready build leaves selector/supervisor PIDs
    and the live connection generation unchanged, an unrelated sentinel in
    the launchd user domain is absent from the resident selector, supervisor,
    every worker and their descendants, an upgrade whose readiness socket
    closes before the old process tree releases `.service-lock` waits for the
    lock and then completes activation without a `lock-busy` outage, a retry
    from either `credential-published` or `bundles-published` re-establishes
    that same offline precondition before mutation, and all failures expose
    stable codes.
    `[deployment §Ownership and readiness]`
72. ◆◆ **LaunchdCrashRecovery** — Setup: launchd-owned service with running,
    parked and recovery-needed lines. Action: separately SIGKILL supervisor,
    SIGKILL selector while its child owns the listener, and reboot followed by
    target-user login without opening the Tekes GUI.
    Assert: signed prepare records different selector PID/launch-id after the
    real SIGKILL, then returns reboot-required; the bounded Aqua resume job rejects
    the old boot-session UUID and authors HMAC-bound evidence before ordinary
    CI can verify it. Launchd keeps one resident selector, selector respawns/reaps the
    supervisor from a recovered frozen selection, boot sweep and D-61 recovery
    restore the same durable state, selector-death EOF drains the recorded old
    supervisor and every recorded descendant before the replacement supervisor
    owns the listener/root lock, without counting a candidate failure; exactly one
    owner/listener remains, and neither launcher nor supervisor authors a
    semantic fact. `[deployment §launchd and process lifetime; D-61]`
73. ◆◆ **InstallUpgradePublicationCrashMatrix** — Setup: signed current,
    candidate and last-known-good bundles. Action: power-loss at every stage,
    first-activation, installer identity/Keychain/install/uninstall/rotation,
    selector-update, sync and switch boundary. Assert: one
    complete selected version and one complete old/new stable selector, no
    in-place executable overwrite, recover-before-spawn, frozen run_start
    attribution, offline selector conformance evidence, and exact v1
    authority-registry digest/profile equality while rollback is eligible,
    immutable install-team/access-group equality across installer, Client,
    selector and every bundle;
    unknown durable authorities fail closed. Selector functional failure after an
    atomic update is reported as installer-repair risk, never falsely covered
    by supervisor LKG rollback. `[deployment §Install, upgrade, rollback, uninstall;
    Evolution mechanism §External authority]`
74. ◆◆ **CrashLoopExternalRollback** — Setup: candidate that repeatedly fails
    readiness plus invalid-config, missing-Keychain and fixed-port-conflict
    controls. Action: let resident selector observe three distinct
    candidate-attributable failed launch identities without an installer/test
    rollback call, then separately inject the three environment failures and
    exercise an attributable failure immediately before the 300-second window
    boundary plus three failures after promotion.
    Assert: the real
    selector—not Kernel—stops/reaps candidate, durably switches and syncs
    last-known-good, spawns it, leaves ledgers untouched, publishes no
    incompatible format during the rollback window, and the previous supervisor
    becomes ready; environment/data failures keep the same generation resident
    and not-ready with bounded retry and never count toward rollback; the
    pre-boundary failure counts, promotion occurs only at the boundary, and
    promotion resets the failure counter and post-promotion failures never
    revive the expired predecessor.
    `[deployment
    §launchd/§Install; D-53/D-54]`
75. ◆◆ **ProductionObservabilityRedaction** — Setup: provider, tool, endpoint,
    corruption and rollback failures carrying canary secrets. Action: produce
    logs, metrics, ready errors and support bundle. Assert: every claimed code
    is present with globally unique launch id plus frozen attempt/generation/
    manifest attribution where applicable, forbidden content is absent, labels are bounded, and
    observability never changes recovery. `[deployment §Observability]`
76. ◆◆ **CleanInstallClientUninstallE2E** — Setup: fresh dedicated macOS user
    with the complete Tekes application-support root absent. Action:
    install, start, let the real `.tekes` Client discover the fixed production
    origin `http://127.0.0.1:7347` and read the shared Keychain bearer directly,
    drive it through T1/archive/unarchive,
    publish the signed `provider-uat` SecretStore record, rotate and revoke it,
    reboot, then uninstall. Assert: all Slice 1–10 gates pass, public URI is
    `tekes://tekes/<uuid>`, service/plist/binaries leave on uninstall, and
    endpoint credential leaves, and authoritative data remains unless
    separately erased; only installer/supervisor can read the provider-secret
    access group, readiness follows active/rotated/revoked generations, and no
    provider material appears in ledger, carrier, logs, metrics, bootstrap or
    support-bundle bytes. The real Keychain UAT validates the durable no-secret
    generation/digest floor after active, rotation and revocation and again
    after provider-item deletion/uninstall; the focused restart oracle rejects
    lower-generation rollback and same-generation collision before
    broker-scope construction. The Client reports only endpoint-observable session/
    model facts; the coordinator derives run attestation, process/root-lock and
    secret-scan facts, records and rechecks the exact session file inventory/
    byte digests across idempotent uninstall, and only consume may assert
    uninstall. The controller archives that result before acknowledging
    operation-key deletion. `[deployment
    §Supported product/§Production acceptance; secret-store;
    session-endpoint §Transport listener]`
77. ✦ **SkillPackageMigrationAndPrecedence** — Setup: pinned AppServer/Runtime
    user and project skill directories containing `SKILL.md` plus distinct
    companions. Action: capture one immutable InstructionSnapshot, build the
    package catalog, list and load it. Assert: a project package replaces the
    user directory as a unit, no lower companion leaks into the winner,
    manifest/directory identity and whole-package digest match the oracle,
    traversal/symlink/invalid winning packages fail closed, and model load and
    `skills/list` use the same frozen bytes. `[skill-package]`
78. ✦ **CommandCatalogExpansionAndNegativeCorpus** — Setup: pinned user/project
    command files plus quoting, escaping, positional, missing and malformed
    cases. Action: list and expand from one snapshot. Assert: project precedence,
    metadata/body identity, UTF-8 ordering, `$ARGUMENTS`, `$1`…`$99`, `$$` and
    `\$` equal the oracle; `$0`, three-digit positionals, malformed arguments,
    over-limit expansion and missing commands author no input; `@file` and
    shell syntax remain literal. `[command-catalog]`
79. ✦ **ResourceCapabilitiesAndKeyedCommandSubmission** — Setup: the exact
    three-method format-1 resource capability and a deduplicating input fake.
    Action: list skills/commands and retry `commands/run` with the same and then
    conflicting key. Assert: list DTOs are exact, one expanded prompt enters the
    keyed-input seam, retry returns its original seq, conflict fails closed and
    missing/invalid commands retain the closed typed errors, and there is no
    command-side transcript/state path. `[command-catalog]`
80. ✦ **ProductionInputSeamAndFrozenV2Authority** — Setup: the real
    `SessionDeliveryAuthority` adapter and frozen Session Endpoint registry.
    Action: submit a command and compare capability/route inventories in both
    directions, then compose a second disjoint extension and a conflicting
    owner. Assert: origin client/op/key and `steer:false` are exact, preflight
    precedes the durable prompt receipt, additive carrier dispatch succeeds,
    duplicate ownership fails closed, the Session Endpoint table remains
    exactly 20 and contains none of the resource methods, and the migration
    source revisions/evidence remain pinned. `[skill-package;
    command-catalog; session-endpoint §Transport listener]`

## External pipeline (ownership only — not kernel CI)

25. **PromotionCrashMatrix** — deployment tooling: one active version with
    bound evidence across activation crashes. `[Evolution mechanism §Ruling]`
26. **ConstitutionIsolation** — CI: validation always uses the
    independently pinned constitution hash. `[Evolution mechanism §Ruling; D-54]`
27. **SupervisorFallback** — launchd/systemd: crash-looping candidate
    is monitored and restored by the resident external launcher without an
    implicit installer/test actor or ambiguous activation. `[Evolution mechanism §Ruling;
    R3-19]`
