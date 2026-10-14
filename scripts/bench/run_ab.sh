#!/usr/bin/env bash
# Interleaved Kernel-vs-Kernel A/B runs of one corpus through the recording proxy.
#
# usage: scripts/bench/run_ab.sh <corpus> <runs-per-arm> <output-dir> <repo-A> <repo-B> [run-public-flow args...]
#
# <repo-A> and <repo-B> are two checkouts (for example a worktree of the base
# branch and the branch under test). Each arm builds and runs its own checkout,
# alternating KA1, KB1, KA2, KB2, ... Extra arguments go to both arms'
# run-public-flow.py (for example --identity-profile general).
#
# Environment: TEKES_KERNEL_LIVE_KEY, UPSTREAM and TOOLCHAIN_ROOT as in
# run_interleaved.sh. Price and compare with:
#   python3 scripts/price-proxy-runs.py <output-dir>
#   python3 scripts/bench/stats.py <output-dir> KA KB
set -euo pipefail
corpus=$1; runs=$2; out=$3; repo_a=$(cd "$4" && pwd); repo_b=$(cd "$5" && pwd); shift 5
repo=$(cd "$(dirname "$0")/../.." && pwd)
: "${TEKES_KERNEL_LIVE_KEY:?set TEKES_KERNEL_LIVE_KEY}"
upstream=${UPSTREAM:-https://api.deepseek.com}
toolchain=${TOOLCHAIN_ROOT:-/opt/homebrew}
command -v python3.14 >/dev/null || { echo "python3.14 is required by the corpus prompts" >&2; exit 1; }
mkdir -p "$out"

run_arm() {  # run_arm <name> <checkout> [run-public-flow args...]
  local name=$1 checkout=$2; shift 2
  local dir="$out/$name-proxy"
  mkdir -p "$dir"
  python3 "$repo/scripts/record-provider-proxy.py" --upstream "$upstream" --output "$dir" \
    --ready-file "$dir/port" > "$dir/proxy.log" 2>&1 &
  local pid=$!
  for _ in $(seq 50); do [ -s "$dir/port" ] && break; sleep 0.2; done
  local base="http://127.0.0.1:$(cat "$dir/port")" rc=0
  python3 -c "import json,sys; d=json.load(open(sys.argv[1])); d['providers'][0]['endpoint']=sys.argv[2]; json.dump(d, open(sys.argv[3], 'w'), indent=2)" \
    "$repo/scripts/bench/providers.example.json" "$base" "$out/$name.providers.json"
  (cd "$checkout" && python3 scripts/run-public-flow.py --case text --live \
    --live-provider deepseek-responses --corpus "$corpus" --config "$out/$name.providers.json" \
    --toolchain-root "$toolchain" --output "$out/$name" "$@") > "$out/$name.log" 2>&1 || rc=$?
  kill "$pid"; wait "$pid" 2>/dev/null || true
  [ "$rc" -eq 0 ] || echo "$name failed (see $out/$name.log)"
}

for i in $(seq "$runs"); do
  echo "== KA$i $(date -u +%FT%TZ)"; run_arm "KA$i" "$repo_a" "$@"
  echo "== KB$i $(date -u +%FT%TZ)"; run_arm "KB$i" "$repo_b" "$@"
done
echo "== done $(date -u +%FT%TZ)"
