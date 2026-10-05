#!/bin/sh
set -eu

repo_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_dir"

# Slice 14B's aggregate lane serially runs 14C, 14D and 14E before its own
# safety gates, so this is the complete pre-14F Kernel baseline.
scripts/ci-slice14b.sh
test -x scripts/ci-provider-dialects.sh || {
  echo "provider-dialect proof lane is not merged" >&2
  exit 1
}
scripts/ci-provider-dialects.sh
python3 scripts/check-client-extension-fixtures.py

# Production route and cross-authority gate tests are added beside the Slice-14F
# route assembly. Exact names make missing implementation a hard failure.
for gate in \
  slice14f_gate_113_catalog_negotiation_and_base_isolation \
  slice14f_gate_114_closed_dtos_authorities_and_idempotency \
  slice14f_gate_115_predecessor_disposition_and_no_special_cases \
  slice14f_gate_116_cross_capability_lifecycle
do
  cargo test -p conformance --locked --test slice14f_gates "$gate" -- --exact
done

cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
