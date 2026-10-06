# Slice 10 deployment coverage

> **Retired (2026-10-06).** The Kernel is launched only by its host application; see
> [Application-owned launch](../../docs/builtin-launch.md). The launchd service, product
> installer and selector described here are no longer maintained, and their
> Keychain credential model no longer matches the code: the endpoint token and
> provider secrets come from the host's environment
> ([secret-store](../../spec/secret-store.md)). The code remains until it is removed
> ([#28](https://github.com/TekesApps/TekesKernel/issues/28)); this document is kept for reference.

## Gate 71 — DeploymentFilesystemAndOwnership

- `clean-install`
- `duplicate-instance`
- repeated healthy `ensure-running` preserves the resident selector/supervisor
  process identities and live transport generation
- `unsupported-filesystem`
- `cloud-managed-storage`
- `corrupt-tail`
- `locked-config`

## Gate 72 — LaunchdCrashRecovery

- `reboot-start`
- `reboot-target-user-login`
- `selector-sigkill-recovery`
- `live-drain`

## Gate 73 — InstallUpgradePublicationCrashMatrix

- `publication-power-loss`
- `first-activation-power-loss`
- `selector-update-power-loss`
- `upgrade-downgrade-gate`
- `installer-install-recovery`

## Gate 74 — CrashLoopExternalRollback

- `crash-loop-rollback`
- `non-attributable-failure-matrix`
- `promotion-window-boundary`

## Gate 75 — ProductionObservabilityRedaction

- `observability-redaction`

## Gate 76 — CleanInstallClientUninstallE2E

- `archive-preservation`
- `uninstall-retains-data`
- `installer-uninstall-recovery`

The separate selector I/O, selector-update manifest, bundle-manifest, complete
installed-layout, install identity/installer-state-machine, launchd-plist,
bootstrap-status, readiness, publication-state-machine, failure-attribution and
observability oracles support these case traces.
`scripts/check-deployment-fixtures.py` validates their closed schemas and the
registry-to-disk mapping in both directions.
The provisioning-profile oracle additionally closes the application-container
and AMFI-admission precondition for the three entitlement-bearing
UAT/production actors.
