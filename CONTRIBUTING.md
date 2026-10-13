# Contributing to TekesKernel

Thanks for considering a contribution. This page lists what a pull request
needs to be merged.

## Before you start

- For a bug, open an issue with the steps to reproduce, the expected and actual
  behavior, and the release or commit (`git rev-parse --short HEAD`) you ran.
  A built supervisor prints its version with
  `tekes-supervisor --describe-build`; a signed release build also prints the
  Git commit as `source_revision`.
- For a feature or a protocol change, open an issue first. Contracts in `spec/`
  are the authority for behavior; a change that alters a contract needs that
  discussion before code.

## Development setup

The toolchain is pinned in `rust-toolchain.toml`; `rustup` installs it on first
use. macOS on Apple silicon is the primary platform. CI also builds and tests
on Linux and must pass there, so keep macOS-only code behind
`#[cfg(target_os = "macos")]`. Linux has two sandbox backends, both requiring
seccomp filters. Bubblewrap 0.4.0 or later requires unprivileged user, mount,
PID, IPC and UTS namespaces, plus network namespaces for network-denied tools.
It works without Landlock, including on compatible Linux 5.10 hosts.
The Landlock backend requires ABI 3 or later to confine file truncation as
well as writes. Actual launch probes are authoritative; a container or host
security policy that blocks the required namespaces or syscalls fails the
corresponding backend tests.

Run `scripts/ci-linux-sandbox.sh --backend all` on a native Linux host to
check both backends independently, then automatic selection, through the
filesystem/network/process tests and helper/job launchers. CI installs
bubblewrap and requires all three paths on native x86_64 and aarch64 runners.
On a host without Landlock, run `scripts/ci-linux-sandbox.sh --backend bubblewrap`
and `scripts/ci-linux-sandbox.sh --backend auto`; these focused
runs do not claim Landlock coverage. The runner does not change host security
configuration or services. See [scripts/README.md](scripts/README.md) for
prerequisites and diagnostics. Python 3 runs the fixture checks. To regenerate the
architecture atlas, install its pinned dependencies:

```sh
python3 -m pip install -r scripts/architecture-requirements.txt
```

## The pull request checklist

1. `scripts/ci.sh` passes. It runs `cargo fmt --all -- --check`,
   `cargo clippy --workspace --all-targets --locked -- -D warnings`,
   `cargo test --workspace --locked`, every `scripts/check-*.py`, and the
   dependency-free Python tests. When the architecture dependencies are
   installed, it also runs the atlas `--check` and `--links` steps.
2. A bug fix comes with a test that fails without the fix.
3. A behavior change updates the contract in `spec/` and the explanation in
   `docs/` in the same pull request.
4. A fixture change keeps the fixture byte-canonical (one JSON object plus one
   LF, sorted keys, no whitespace between tokens, non-ASCII characters as UTF-8
   rather than `\u` escapes) and updates `fixtures/manifest.json` with
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
