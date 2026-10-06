## What and why

<!-- What changes, and the problem it solves. Link the issue if there is one. -->

## Checklist

- [ ] `scripts/ci.sh` passes locally
- [ ] Bug fixes include a test that fails without the fix
- [ ] Contract changes update `spec/` and `docs/` in this pull request
- [ ] Fixture changes are byte-canonical and `fixtures/manifest.json` is regenerated with `python3 scripts/update-fixture-manifest.py`
- [ ] Rust or Cargo changes regenerate the architecture atlas
- [ ] macOS-only code is behind `#[cfg(target_os = "macos")]` (CI also runs `cargo test` on Linux)
- [ ] No credentials, personal paths, or deployment-specific values

## Live runs

<!-- If this affects provider requests: which live runs did you do, if any? -->
