#!/bin/sh
set -eu

repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"

# Slices 14C, 14D and 14E prove supervisor lifecycle, rewrite/catalog lock
# composition and Web/tool dispatch isolation; the Slice-14B safepoint lane
# was removed on 2026-09-06 (simplification item 8).
scripts/ci-slice14c.sh
scripts/ci-slice14d.sh
scripts/ci-slice14e.sh
cargo test -p tekes-worker --locked
cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
