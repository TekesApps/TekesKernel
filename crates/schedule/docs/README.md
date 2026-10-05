# schedule — durable scheduling

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Stores cron/timezone/task definitions, due claims and run status for the host to launch tasks.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `lib` | ScheduleAuthority, CronSchedule, LaunchClaim and status log |

## Interfaces and calls

Main entry points: ScheduleAuthority, CronSchedule, ScheduleDefinition, LaunchClaim.

supervisor timer → poll_due/recover → claim → host creates and binds execution → record_status.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

This is a library, not a separate cron process; claim, actual launch and completion are distinct states.

Behavior contract: [schedule.md](../../../spec/schedule.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
