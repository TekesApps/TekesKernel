#!/usr/bin/env bash
# The hermetic check set: everything a pull request must pass without network,
# provider credentials, signing identities or sibling checkouts. Live and
# signed-artifact gates are separate (see scripts/README.md).
set -euo pipefail
cd "$(dirname "$0")/.."

step() { printf '\n== %s\n' "$*"; }

step "cargo fmt"
cargo fmt --all -- --check

step "cargo clippy"
cargo clippy --workspace --all-targets --locked -- -D warnings

step "cargo test"
cargo test --workspace --locked

step "fixture checks"
for check in scripts/check-*.py; do
  python3 "$check"
done

step "python tests"
for test in \
  scripts/test_cache_corpus.py \
  scripts/test_agent_eval_audit.py \
  scripts/test_agent_eval_scored.py \
  scripts/test-audit-live-python-task.py \
  scripts/test-audit-live-task-validation.py
do
  python3 "$test"
done
python3 -m unittest discover -s packaging/macos/tests -p 'test_*.py'

step "architecture atlas"
if python3 -c 'import tree_sitter, tree_sitter_rust' 2>/dev/null; then
  python3 scripts/code-architecture.py --check
  python3 scripts/code-architecture.py --links
else
  echo "skipped: install scripts/architecture-requirements.txt to check the generated atlas"
fi

printf '\nci: passed\n'
