# Kernel 0.9.0 local acceptance

Date: 2026-10-17. This is a breaking Kernel change prepared for PR integration.
The existing worktree was reused. Tagging and consumer release are outside this task.

## Boundary

The contract is [Session execution](../../spec/session-execution.md). Session
creation freezes explicit execution values in reader-5 genesis. Endpoint v4
advertises the breaking surface; private bootstrap remains v1. There is no
Environment/workspace/Folder registry, trust grant or old journal conversion on
the execution path. Session operation intents pin the full execution digest.

Kernel keeps HTTP and host-owned stdio MCP protocol connections, tool projection,
invocation, cancellation, durable effects/tasks and reconnect contract checks.
HTTP secrets are purpose/destination-bound private grants with generation checks.
The host owns server discovery/installation, OAuth and all local subprocess
startup, environment, shutdown and reaping. The independent Python standard-
library [CLI host](../../host/README.md) has no Tekes/ClientKit dependency.

Removed Kernel implementation includes Environment/Folder/workspace management,
Folder trust authority, MCP scope/registry/OAuth admission, plugin administration,
directory management and scheduled launch assembly. Standalone host utility
crates remain outside supervisor's normal dependency graph. Generic protocol
fixture/host utilities are not mounted Kernel management services.

Inventory uses only its own recorded cwd and Session identity. Title search no
longer requires workspace metadata. Context search supports current Session,
explicit Session or all Sessions; the old project registry filter is removed.
Project skills/hooks are not implicitly executable; hosts prepare user/Session
instruction sources. Rendering, workspace UI and RunView were not changed.

## Verification

The final `scripts/ci.sh` run is recorded in `/tmp/kernel090-reviewed-ci.log`.
Result: **PASS** (`ci: passed`), with **1045 Rust tests passed, 0 failed,
47 ignored**. Ignored and platform-disabled tests are not acceptance. The script includes fmt,
all-target Clippy with warnings denied, the complete Cargo workspace test suite,
every `scripts/check-*.py` contract check, five Python audit suites, eight CLI
host tests, generated architecture freshness and link checks.

Focused repairs passed the full supervisor unit target (150 tests; 12 ignored),
six title-search integration tests, Session lifecycle/recovery, real MCP HTTP and
stdio integration, bounded pipe framing, and worker launch/provider regressions.
All ignored tests remain unaccepted rather than being counted as passed.

The real macOS smoke (`TEKES_TEST_CLI_HOST=1 python3
scripts/test-builtin-launch.py`) used the actual built supervisor, worker,
sandbox helper, a local mock model provider and a real host-started stdio MCP
server. `/tmp/kernel090-reviewed-cli.log` records:

- Private bootstrap, malformed frames and legacy environment-secret rejection;
  endpoint/provider secret absence from initial macOS process data.
- Authenticated readiness, duplicate-instance rejection and removed management
  route refusal.
- A real sandboxed file read reaching the provider through a tool result.
- An actual MCP tool call over a host-owned stdio channel reaching the provider.
- Full supervisor/CLI exit and restart, authenticated mux inventory reconnect,
  rename, another actual MCP invocation after restart, and clean shutdown.
- Every MCP child PID was gone after shutdown; corrupt retired configuration
  files remained byte-identical and did not affect execution or restart.

The host unit tests also exercise an uncooperative child, fresh-process reconnect,
failed spawn, explicit environment isolation, frozen launch configuration and
private configuration permissions. The MCP integration tests exercise anonymous
HTTP, purpose/destination grant rejection, rotation/revocation, Session isolation,
unsupported sibling tools and reconnect contract pinning. Pipe tests cover
coalesced frames, wrong response IDs, partial-frame timeout and oversized frames.

Protected unrelated WIP: all 13 ClientKit Composer files matched their original
SHA-256 baseline; the unrelated Tekes design note also matched its baseline.
Temporary logs are development evidence, not release artifacts.

## Unverified and separate work

- ClientKit/Tekes consumption of endpoint v4 and App-specific process-host
  integration were not changed or accepted in this Kernel/standalone CLI task.
- Native Linux sandbox behavior, remote live providers, signed integrations,
  GUI interaction and remote CI were not run here.
- The Kernel execution catalog exposes MCP tools. Resource/prompt browsing stays
  in the generic MCP protocol library and is not a Kernel endpoint capability.
- No compatibility with 0.8/0.7 clients, old management journals or active launch
  configurations is promised. No automatic data migration was added.

## Pre-merge review repairs

Three defects were reproduced before repair and covered by regression tests:

1. The CLI host's blocking child stdin write hid Kernel EOF under backpressure.
   The relay now keeps bounded nonblocking writes while draining the socket for
   EOF, and kills/reaps the child when Kernel disconnects.
2. Creation, launch and preset preview omitted explicit execution roots from
   instruction capture. They now read project AGENTS text under those roots,
   while project skills/hooks remain non-executable. Tests cover both creation's
   published snapshot and later Session instruction resolution.
3. Partial MCP projection dropped rejected names before launch validation. Exact
   policy-selected refusal names now reach both worker and client catalog binding;
   healthy siblings remain usable and genuinely unknown references still fail.

An independent read-only review found the third issue; its follow-up found no
additional P0/P1/P2 blockers in these repairs. Claude review could not authenticate;
the independent review used the local Codex review CLI instead. The retained
legacy web prototype is another unadapted v3 consumer; built-in launch disables
that unauthenticated UI, and no rendering/UI migration is claimed here.

## Linux CI portability repair

The first PR CI run found `held_response_line` unused on both Linux targets. Its
callers are all macOS-only tests; the helper now carries the same platform guard.
No test caller was removed or newly skipped. The follow-up local verification
runs fmt, all-target Clippy and the supervisor unit suite; the PR reruns native
Linux x86_64/aarch64 sandbox and workspace checks before merge.
