# Security policy

TekesKernel runs model-directed shell commands and file edits, holds provider
credentials, and serves a loopback-only local endpoint. Please report
vulnerabilities privately so they can be fixed before disclosure.

## Reporting a vulnerability

Use GitHub's private vulnerability reporting: on the repository page, open
**Security → Report a vulnerability**. Do not open a public issue for a
security problem.

Include the affected version or commit, the platform, the steps to reproduce,
and the impact you observed. You will get an acknowledgement within seven
days. Please allow up to 90 days for a fix before public disclosure; we will
coordinate the date with you.

## In scope

Reports apply to the supported platform, macOS on Apple silicon. On Linux the
shell sandbox is absent, and commands that require it are refused rather than
run unsandboxed.

- Escaping the shell sandbox or the workspace root (file or Git operations
  outside the resolved workspace).
- Leaking provider credentials, the endpoint token, or secret material into
  ledgers, logs, tool results, or provider requests.
- Bypassing Session Endpoint authentication or origin checks, or reaching the
  endpoint from a non-loopback address.
- Executing tools or MCP servers without the required approval.
- Corrupting or forging ledger history in a way that survives validation.

## Out of scope

- Behavior of third-party model providers or MCP servers themselves.
- Actions a user explicitly approved, or that their configured permission
  policy allows.
- Issues that require an attacker who already controls the user's account or
  machine.

## Supported versions

Only the latest tagged version, the newest release entry in
[CHANGELOG.md](CHANGELOG.md), receives security fixes while the project is in
alpha.
