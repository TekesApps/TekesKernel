#!/usr/bin/env bash
# Exercise the applied Linux sandbox, not just its policy compiler.
set -euo pipefail
cd "$(dirname "$0")/.."

backend=all
if [[ $# -gt 0 ]]; then
  case "$1" in
    --help|-h)
      echo "Usage: $0 [--backend all|bubblewrap|landlock|auto]"
      exit 0
      ;;
    --backend)
      if [[ $# -ne 2 ]]; then
        echo "Usage: $0 [--backend all|bubblewrap|landlock|auto]" >&2
        exit 2
      fi
      backend=$2
      ;;
    *)
      echo "Usage: $0 [--backend all|bubblewrap|landlock|auto]" >&2
      exit 2
      ;;
  esac
fi
case "$backend" in
  all) backends=(bubblewrap landlock auto) ;;
  bubblewrap|landlock|auto) backends=("$backend") ;;
  *)
    echo "Unknown Linux sandbox backend: $backend" >&2
    exit 2
    ;;
esac

case "$(uname -s)/$(uname -m)" in
  Linux/x86_64|Linux/aarch64) ;;
  *)
    echo "Linux sandbox tests require a native Linux x86_64 or aarch64 host" >&2
    exit 1
    ;;
esac

printf 'Linux sandbox host: '
uname -srmo
if [[ -r /sys/kernel/security/lsm ]]; then
  printf 'Enabled LSMs: '
  cat /sys/kernel/security/lsm
fi

# Public runtime tests exercise the helper/job entry points and escape attempts.
# A misconfigured target or renamed test module must not pass with zero tests.
tests=$(cargo test -p tools --locked --test linux_sandbox_runtime -- --list)
if ! grep -Eq '^linux_[^ ]+: test$' <<< "$tests"; then
  echo "Linux sandbox public runtime tests are missing from the selected target" >&2
  exit 1
fi
# Cover the production helper launcher and approval binding as well.
gates=(
  slice4_gate_11_sandbox_backend_probe
  slice4_gate_44_exec_helper_process_protocol
  slice4_gate_45_sandbox_probe_and_approval_binding
)
tests=$(cargo test -p tools --locked --test slice4_gates -- --list)
for gate in "${gates[@]}"; do
  if ! grep -Fxq "$gate: test" <<< "$tests"; then
    echo "Linux sandbox integration gate is missing: $gate" >&2
    exit 1
  fi
done
for selected in "${backends[@]}"; do
  printf '\nLinux sandbox backend: %s\n' "$selected"
  # This selector exists only in test code, never in production selection.
  export TEKES_SANDBOX_TEST_BACKEND=$selected
  case "$selected" in
    bubblewrap) module=bubblewrap_sandbox::tests:: ;;
    landlock) module=linux_sandbox::tests:: ;;
    auto) module= ;;
  esac
  if [[ -n "$module" ]]; then
    tests=$(cargo test -p tools --locked --lib "$module" -- --list)
    if ! grep -Eq "^${module}[^ ]+: test$" <<< "$tests"; then
      echo "Linux sandbox backend tests are missing: $module" >&2
      exit 1
    fi
    # Unsupported hosts fail; ignored backend tests must still execute.
    cargo test -p tools --locked --lib "$module" -- --include-ignored --nocapture
  fi
  # The ignored confined_probe_process is a child entry point launched by the
  # parent tests only after the sandbox has been applied.
  cargo test -p tools --locked --test linux_sandbox_runtime -- --nocapture
  if [[ "$selected" == bubblewrap ]]; then
    tests=$(cargo test -p tools --locked --test linux_sandbox_runtime -- --list)
    if ! grep -Eq '^bubblewrap_lifecycle_[^ ]+: test$' <<< "$tests"; then
      echo "Bubblewrap lifecycle tests are missing" >&2
      exit 1
    fi
    cargo test -p tools --locked --test linux_sandbox_runtime bubblewrap_lifecycle_ -- \
      --include-ignored --nocapture
    tests=$(cargo test -p tools --locked --lib runtime_backends::namespace_job_identity_tests:: -- --list)
    if ! grep -Eq '^runtime_backends::namespace_job_identity_tests::[^ ]+: test$' <<< "$tests"; then
      echo "Namespace job identity tests are missing" >&2
      exit 1
    fi
    cargo test -p tools --locked --lib runtime_backends::namespace_job_identity_tests:: -- \
      --include-ignored --nocapture
  fi
  cargo test -p tools --locked --test slice4_gates -- \
    --exact --include-ignored --nocapture "${gates[@]}"
  printf 'Linux sandbox backend %s: passed\n' "$selected"
done

printf '\nLinux sandbox runtime gates (%s): passed\n' "$backend"
