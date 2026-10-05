# Retired built-in Kernel memory (2026-09-16)

[History entry point](README.md) · [Current system overview](../README.md)

## Decision and scope

The built-in `note` and `recall` tools are removed: their declarations,
schemas and execution backends. The supervisor's automatic `memorize` merge
after a successful turn is removed with them. Long-term memory belongs to a
separate memory service connected through MCP tools and lifecycle hooks. The
Kernel keeps the general lifecycle hooks and goal support.

The old design was: `note` staged candidates; after the main turn completed,
all unconsumed candidates were merged; `recall` read entries exactly by key
and scope. It was not an in-turn scratch pad.

## Configuration and upgrade

- The default tool list of a new workspace no longer contains `note` or
  `recall`.
- If an existing workspace policy still names them, launch is not blocked: the
  worker removes them from the effective tool allowlist and grants no
  execution. Existing configuration snapshots and digests need no rewrite.
- To clean a policy, remove the two names from the workspace's allowed tool
  names and save with `workspace.policy.set` and the current revision; keep
  every other tool and policy field.
- Using a separate memory service still requires configuring the service, its
  MCP tools and lifecycle extension. Retiring the old tools neither enables a
  new service nor imports old memory.
- The change takes effect with a new Kernel binary. A process already running
  the old version keeps the old behavior until it is stopped and restarted
  through the normal update flow; this change does not act on running
  services.

## Data archive

`<storage-root>/memory/log.jsonl` stays in place as a historical archive,
including old facts and unconsumed candidates. The new Kernel does not read,
append, repair, delete or move this file. Old session ledgers and
configuration snapshots are preserved too. Code history is kept in Git; no
second runnable copy of the old memory backend is kept. Importing old memory
into another service needs a separate, explicit migration.

Earlier `live_memory_smoke` results are historical evidence only and are
marked archived in the live-test inventory. `run-public-flow.py --memory` and
the tests that waited for the old merge result are removed.
