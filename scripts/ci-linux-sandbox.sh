#!/usr/bin/env bash
# Exercise the applied Linux sandbox, not just its policy compiler.
set -euo pipefail
cd "$(dirname "$0")/.."

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

# A misconfigured target or renamed test module must not pass with zero tests.
tests=$(cargo test -p tools --locked --lib linux_sandbox::tests:: -- --list)
if ! grep -Eq '^linux_sandbox::tests::[^ ]+: test$' <<< "$tests"; then
  echo "Linux sandbox runtime tests are missing from the selected target" >&2
  exit 1
fi

# Backend tests require a successful probe and launch real confined processes.
# Force execution if a test is ever marked ignored: unsupported hosts fail.
cargo test -p tools --locked --lib linux_sandbox::tests:: -- --include-ignored --nocapture

# Public runtime tests exercise the helper/job entry points and escape attempts.
tests=$(cargo test -p tools --locked --test linux_sandbox_runtime -- --list)
if ! grep -Eq '^linux_[^ ]+: test$' <<< "$tests"; then
  echo "Linux sandbox public runtime tests are missing from the selected target" >&2
  exit 1
fi
# The ignored confined_probe_process is a child entry point launched by the
# parent tests only after the sandbox has been applied.
cargo test -p tools --locked --test linux_sandbox_runtime -- --nocapture

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
cargo test -p tools --locked --test slice4_gates -- \
  --exact --include-ignored --nocapture "${gates[@]}"

printf '\nLinux sandbox runtime gates: passed\n'
