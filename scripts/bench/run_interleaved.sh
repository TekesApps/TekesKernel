#!/usr/bin/env bash
# Interleaved Kernel / pi runs of one cache corpus through the recording proxy.
#
# usage: scripts/bench/run_interleaved.sh <corpus> <runs-per-arm> <output-dir>
#
# Environment:
#   TEKES_KERNEL_LIVE_KEY  DeepSeek API key (used by both arms; never written to outputs)
#   PI_BIN                 path to the pi 1.0.x executable (npm i @earendil-works/pi-coding-agent@1.0.0)
#   UPSTREAM               provider base URL (default https://api.deepseek.com)
#   TOOLCHAIN_ROOT         prefix whose bin/ holds python3.14, which the corpus prompts name
#                          (default /opt/homebrew); Kernel's shell sandbox gets read access to it
#
# Each run gets its own proxy directory <output>/<run>-proxy with every request
# body and the provider's raw usage. Price with:
#   python3 scripts/price-proxy-runs.py <output-dir>
#   python3 scripts/bench/stats.py <output-dir> K P
set -euo pipefail
corpus=$1; runs=$2; out=$3
repo=$(cd "$(dirname "$0")/../.." && pwd)
: "${TEKES_KERNEL_LIVE_KEY:?set TEKES_KERNEL_LIVE_KEY}"
: "${PI_BIN:?set PI_BIN to the pi executable}"
upstream=${UPSTREAM:-https://api.deepseek.com}
toolchain=${TOOLCHAIN_ROOT:-/opt/homebrew}
command -v python3.14 >/dev/null || { echo "python3.14 is required by the corpus prompts" >&2; exit 1; }
mkdir -p "$out"

with_proxy() {  # with_proxy <proxy-dir> <command...>; {BASE} in the command becomes the proxy URL
  local dir=$1; shift
  mkdir -p "$dir"
  python3 "$repo/scripts/record-provider-proxy.py" --upstream "$upstream" --output "$dir" \
    --ready-file "$dir/port" > "$dir/proxy.log" 2>&1 &
  local pid=$!
  for _ in $(seq 50); do [ -s "$dir/port" ] && break; sleep 0.2; done
  local base="http://127.0.0.1:$(cat "$dir/port")" args=()
  for arg in "$@"; do args+=("${arg//\{BASE\}/$base}"); done
  local rc=0
  "${args[@]}" || rc=$?
  kill "$pid"; wait "$pid" 2>/dev/null || true
  return $rc
}

for i in $(seq "$runs"); do
  echo "== K$i $(date -u +%FT%TZ)"
  # Point a copy of the example provider config at this run's proxy, then run.
  with_proxy "$out/K$i-proxy" bash -c '
    python3 -c "import json,sys; d=json.load(open(sys.argv[1])); d[\"providers\"][0][\"endpoint\"]=sys.argv[2]; json.dump(d, open(sys.argv[3], \"w\"), indent=2)" \
      "$1/scripts/bench/providers.example.json" "$2" "$3.providers.json"
    cd "$1" && python3 scripts/run-public-flow.py --case text --live \
      --live-provider deepseek-responses --corpus "$4" --config "$3.providers.json" \
      --toolchain-root "$5" --output "$3"
  ' _ "$repo" "{BASE}" "$out/K$i" "$corpus" "$toolchain" > "$out/K$i.log" 2>&1 || echo "K$i failed (see $out/K$i.log)"
  echo "== P$i $(date -u +%FT%TZ)"
  with_proxy "$out/P$i-proxy" python3 "$repo/scripts/run-pi-long-session-cache.py" \
    --output "$out/P$i" --pi-bin "$PI_BIN" --corpus "$corpus" --base-url "{BASE}" --no-skills \
    > "$out/P$i.log" 2>&1 || echo "P$i failed (see $out/P$i.log)"
done
echo "== done $(date -u +%FT%TZ)"
