# Changelog

Notable changes per release. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow the
workspace version in `Cargo.toml`.

## Unreleased

### Removed
- The launchd deployment. The host application launches the Kernel and
  supplies its endpoint token and credentials through the environment, as in
  [Application-owned launch](docs/builtin-launch.md). Removed: the `selector`,
  `product-installer` and `deployment-tests` crates, the signed release and UAT
  packaging, the deployment fixtures and `ci-slice10.sh`; and from
  `tekes-supervisor` the `--install-root` daemon mode, `support-bundle`,
  `--web-launch-url`, `--describe-web-client`, and the operational metrics,
  access log and support bundle that only the daemon wrote.
  `--describe-build` no longer reports `authority_registry_sha256`, and the
  storage preflight no longer creates `credential-state/`
  ([#28](https://github.com/TekesApps/TekesKernel/issues/28)).
- The `providerAdmin.v1` client-extension group (`providers.list`,
  `providers.verify`, `providers.connections`, `providers.connection.*` and
  `providers.profile*`). The launching application owns provider
  configuration; abandoned provider and settings operation records are ignored
  on recovery. The client-extension fixtures drop the group's catalog entry,
  method and negative cases, error codes and readiness value cases.
- The runtime provider route allow-list. The proof registry
  (`fixtures/provider-dialects/profiles.canonical.json`) is no longer compiled
  into the Kernel: a configured provider on a supported dialect runs with the
  endpoint, gateway and model it is given, and an upstream that rejects the
  route fails at request time. Credential headers come from the dialect, and
  the configured endpoint is no longer completed from a proved URL. Readiness
  drops `dialect-unproved` (an unknown dialect is `misconfigured`).
  `--models-available` now prints `[{dialect_id, protocol_family}]` instead of
  proof rows. Removed from the `provider` crate: `AdvertisedDialectProof`,
  `advertised_dialect_proofs`, `configured_route_is_verified`,
  `validate_endpoint`, and `ResolvedDialectProfile::{proof_verified,
  proved_endpoint}`; added `supported_dialects` and `validate_provider`.
  `scripts/add-provider-model-proof-alias.py` is removed.

### Added
- Linux tool sandbox. The `linux-landlock-seccomp` backend now applies its
  plan: Landlock confines the filesystem to the policy roots and seccomp
  refuses sockets under `deny` network, process creation without
  `allow_process`, and namespace, mount, keyring, `io_uring` and
  terminal-injection interfaces. Shell, edit and job tools therefore run on
  Linux instead of being refused. `loopback` network is refused on Linux
  because neither mechanism can enforce it
  ([#25](https://github.com/TekesApps/TekesKernel/issues/25)).
- `web_listen` in the built-in launch document starts the browser Web Client on
  a loopback address and adds `webUrl` to the readiness record. It is off when
  omitted.

### Fixed
- Built-in launch now starts on Linux. The storage preflight required APFS on
  every platform; outside macOS it now relies on the local-filesystem probe.

## 0.2.2 — 2026-10-06

### Added
- Release builds record their Git commit. With `TEKES_SOURCE_REVISION` set at
  compile time, `tekes-supervisor --describe-build` adds `source_revision`.
  `build-signed-release.sh` requires `--source-revision` and rejects a
  supervisor that reports another commit; `scripts/ci-slice10.sh` refuses a
  tree with local changes ([#5](https://github.com/TekesApps/TekesKernel/issues/5)).
- `tekes-selector prune` removes published versions that are neither the
  current nor the previous selection, with their observations, so old
  development versions do not accumulate and a pruned version can be staged
  again ([#4](https://github.com/TekesApps/TekesKernel/issues/4)).

### Fixed
- The workspace-service `process_files` tests no longer fail intermittently on
  Linux with ETXTBSY ("Text file busy"); the fixture warm-up exec now retries.

## 0.2.1 — 2026-10-05

### Added
- Optional native context composition projections use the actual prepared request,
  safe tool inventories and independent byte bounds. Usage/details share a clock;
  model changes and compaction invalidate stale request readings.


### Changed
- `tekes-selector stage` of a different build under an already published
  version returns `invalid-state` with state `version-already-published`
  instead of `invalid-bundle`. Each build needs its own version; see
  `packaging/README.md` ([#4](https://github.com/TekesApps/TekesKernel/issues/4)).

### Fixed
- Private data roots open inside the macOS App Sandbox. When the sandbox denies
  the descriptor walk from `/`, the profile opens the canonical path and still
  rejects a path that traverses a symlink.
- The job sandbox error names the probe failure class and detail.
- The test suite builds and passes on Linux; CI now requires the Linux job.

## 0.2.0 — 2026-10-05

First public release. It also contains everything in 0.1.4.

Versions 0.1.1 to 0.1.4 were released from the earlier closed-source history;
their tags are not in this repository. Version 0.1.3 was skipped.

### Added
- `scripts/ci.sh`, the single hermetic check entry point, and a GitHub Actions
  workflow that runs it on macOS.
- `session.discard` transport contract example and its error codes in the
  endpoint fixture checker.
- Contributor documentation: `CONTRIBUTING.md`, `SECURITY.md`,
  `CODE_OF_CONDUCT.md`, `scripts/README.md`.
- Cost benchmark report against pi 1.0 on DeepSeek Flash
  (`docs/benchmarks/deepseek-cost-2026-10.md`) and its reproduction kit
  (`scripts/bench/`); the pi runner gains `--no-skills`.
- `docs/glossary.md`.

### Changed
- Live and benchmark scripts read `TEKES_PROVIDERS_CONFIG` and
  `TEKES_LIVE_KEYS_FILE` (or explicit flags) instead of fixed home-directory
  paths; `run-public-flow.py` needs no key file when `TEKES_KERNEL_LIVE_KEY`
  is set.
- Deployment-specific provider aliases are no longer shipped as proof rows;
  supply them in the runtime provider configuration. Unlisted DeepSeek
  Responses models now inherit the dialect's uniform reasoning levels.
- No crate is published to crates.io (`publish = false`).
- The 29 MB machine-readable architecture inventory
  (`docs/architecture/generated/inventory.json`) is no longer tracked; run
  `scripts/code-architecture.py` to write it locally. The generated Markdown
  atlas stays in the repository.
- The whole workspace is formatted with `cargo fmt` and passes
  `clippy -D warnings`.

### Fixed
- A supervisor test mutated the process environment while other tests ran.
- workspace-service tests failed intermittently under a full parallel run
  because macOS serializes the first execution of new executables.
- Two fixture checks and the management gate did not know `initialPresets.v1`
  and `session.discard`.

## 0.1.4 — 2026-10-05

### Added
- Session model catalogs expose the configured effective context window.
- A `contextUsage` control projection distinguishes conservative request bounds
  from unknown occupancy, independently of cumulative billing usage.
- Context refresh on ledger append, configuration changes, and control reconnect;
  publication sequence persistence prevents restart from replaying an older value.

### Changed
- Compaction, route/policy changes, opaque media and remote continuation state
  invalidate occupancy until a compatible request can be measured conservatively.

## 0.1.2 — 2026-10-04

### Added
- Built-in `coding` and `general` session profiles
  (`crates/tools/prompts/`). The coding profile adds four behavior rules that
  stop unrequested verification loops.
- Native initial-preset catalog (`session.initialPresets`).
- `deepseek-flash` model capability for the DeepSeek Responses dialect.

### Changed
- Automatic title requests always disable reasoning and cap output at 64
  tokens.
- Clients are never offered a `none` reasoning effort; the lowest is `low`.

## 0.1.1 — 2026-10-04

First tagged release.
