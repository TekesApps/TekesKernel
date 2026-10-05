# Contributing to TekesKernel

Thanks for considering a contribution. This page lists what a pull request
needs to be merged.

## Before you start

- For a bug, open an issue with the steps to reproduce, the expected and actual
  behavior, and the `tekes-supervisor --version` (or commit) you ran.
- For a feature or a protocol change, open an issue first. Contracts in `spec/`
  are the authority for behavior; a change that alters a contract needs that
  discussion before code.

## Development setup

The toolchain is pinned in `rust-toolchain.toml`; `rustup` installs it on first
use. macOS on Apple silicon is the primary platform. Python 3 runs the fixture
checks. To regenerate the architecture atlas, install its pinned dependencies:

```sh
python3 -m pip install -r scripts/architecture-requirements.txt
```

## The pull request checklist

1. `scripts/ci.sh` passes. It runs `cargo fmt --check`, `cargo clippy
   --workspace --all-targets -D warnings`, `cargo test --workspace --locked`,
   every `scripts/check-*.py`, and the dependency-free Python tests.
2. A bug fix comes with a test that fails without the fix.
3. A behavior change updates the contract in `spec/` and the explanation in
   `docs/` in the same pull request.
4. A fixture change keeps the fixture byte-canonical (one JSON object plus one
   LF, sorted keys) and updates `fixtures/manifest.json` with
   `python3 scripts/update-fixture-manifest.py`. Pinned digests in the code
   (for example in `crates/provider/src/dialect.rs`) change with it.
5. A change to Rust source or Cargo configuration regenerates the architecture
   atlas: `python3 scripts/code-architecture.py`, then
   `python3 scripts/code-architecture.py --check` and `--links`.
6. No credentials, personal paths, or deployment-specific values (gateway
   hosts, account-specific provider ids, signing identities) in code,
   fixtures, or docs. Consumers supply those at runtime.

## Tests that call real providers

Live tests are `#[ignore]`d and never run in CI. They need your own provider
configuration and keys (`TEKES_PROVIDERS_CONFIG`, `TEKES_LIVE_KEYS_FILE`) and
spend tokens; see `scripts/README.md`. When a change affects provider
requests, say in the pull request which live runs you did, if any.

## Commit messages

Use a short conventional prefix and a scope when one fits, for example
`fix(supervisor): ...`, `feat(provider): ...`, `docs: ...`, `chore: ...`. The
body explains what changed and why, in plain sentences.

## License

By contributing you agree that your contribution is licensed under the
[MIT License](LICENSE).
