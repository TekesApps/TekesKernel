# Platforms and portability boundaries

[Implementation · documentation home](../README.md) · [Architecture overview](README.md) · [Next: Rust constraints](rust.md)

Target claim: the code ports everywhere; the process model does not. The
complete seam inventory below (D-51) is the contract; seams 1 and 2 are the
portability-critical subset that keeps the platform difference contained.

## Platform reality

| Target | Worker code | Process model | Exec tools |
|---|---|---|---|
| Linux server | best home | real processes | yes |
| macOS | yes | real processes | yes |
| iOS | as an embedded library | **no** — sandbox forbids spawn; background execution suspends | no |
| Android | as an embedded library | technically possible, platform-hostile (W^X, Doze, foreground services) | discouraged |

## Seam 1: process semantics

The kernel speaks four verbs — `spawn / exit / signal / wait` — through an
interface with two implementations:

- **OS processes** (server, macOS): flock leases, SIGTERM, waitpid, setsid jobs.
- **In-process tasks** (mobile): structured tasks inside the app process, same
  four verbs (spawn = start task, signal = cancel, wait = await), an in-process
  lock table instead of flock.

Worker core code never imports POSIX directly.

## Seam 2: tool backends

Exec backend exists only where seam 1 provides real processes. Mobile tool
sets are in-process functions and remote/daemon calls; the event-level ABI
(`tool_call`/`tool_result`) is identical, so transcripts remain
platform-agnostic.

## The complete seam inventory (D-51)

Five traits and three wire protocols; everything else is concrete code. A
trait exists only where a second implementation is *committed* — each one is
a ticket for a second behavior surface, and the current stack's dual context
backend shows what those tickets cost.

| Trait | Verbs | Committed implementations |
|---|---|---|
| `ProcessHost` (seam 1) | spawn / wait / signal / exit + lock table | OS processes; in-process tasks (mobile) |
| `ToolBackend` (seam 2) | execute(tool_call) → tool_result | exec; in-process; daemon socket |
| `ProviderAdapter` | render → wire, stream decode, continuation capability, usage/error mapping | the five v1 protocol families in provider-runtime; a row is enabled only after predecessor parity |
| `SecretStore` | [secret-store](../../spec/secret-store.md) resolve(key id) → value through the credential broker's private descriptor channel (D-46) | macOS/mobile Keychain (Security.framework C API — language-independent); Linux secret-service / systemd credentials / encrypted file |
| `Platform` | clock, entropy, sync policy | real; deterministic test double |

Deliberately **not** seams: storage (the JSONL layer is the only
implementation — the format is the contract; a storage trait would recreate
the dual-backend complexity), the event schema (plain structs), the
supervisor (concrete; bundled services isolate via files + control protocol,
not traits).

Wire protocols defined by this project: the worker stdio control protocol
(inner, bespoke JSONL; the daemon socket reuses its shape), the supervisor
client boundary (the current [Session Endpoint](../../spec/session-endpoint.md#routes-and-streams) value contract, with D-62 and [the historical Client mapping](../history/client-v2.md) preserving the V2 baseline, plus the
Slice-9 HTTP/WebSocket binding), and the hook stdin/stdout contract ([Tool execution](../runtime/tools.md)). MCP
is consumed as an external standard, not defined here.

## What ports without seams

The JSONL layer (O_APPEND, rename-atomicity, and file locks exist in every
target's sandbox), the event schema, every projection, the provider dialects,
the HTTP/SSE layer.

**The deepest portability is session portability, not code portability:** a
thread folder produced on a phone is byte-identical in meaning to one produced
on a server. But a copy is never a second live writer (D-47): handoff is
either a **quiescent move** (ownership generation bump, origin retired) or a
**fork** (new identity via the rewrite projection). Two live folders with the
same identity are a spec violation, not a merge problem.

## Mobile strategy

Default: **thin client**. The worker runs on a server/desktop; the mobile app
is a renderer + control surface over the supervisor's read RPC, syncing by seq
cursor. This covers ~90% of mobile need with zero mobile porting.

Embedded worker (offline, local models, privacy) is the only reason to cross
seam 1 on mobile — defer until that requirement is real.

## Language (decided — D-63; supersedes D-59's implementation choice)

**Contracts are language-neutral; the first reference implementation is
Rust** *(D-63)*. The event schema, wire protocols,
lifecycle rules, and every golden fixture are language-neutral JSON/text and
must produce identical bytes and behavior in another language. Rust is an
implementation/profile decision, not a fifth platform seam and not an excuse
for Rust-shaped protocol fields. Integration with the Swift Client/AppServer
uses the existing process/wire boundaries; no FFI is required by Slice 1.
See [the neutral build plan](../history/implementation-plan.md) and [the Rust workspace profile](rust.md).
