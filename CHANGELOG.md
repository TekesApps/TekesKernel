# Changelog

Notable changes per release. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versions follow the
workspace version in `Cargo.toml`.

## Unreleased

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

## 0.1.1

First tagged release.
