# TekesKernel

TekesKernel is a local execution kernel for AI coding and general-purpose
agents. It records every session in an append-only JSONL ledger, runs each
active line of work in its own worker process, and uses a supervisor to manage
workers, tools, model providers and client connections. Clients talk to it
over the Session Endpoint (HTTP plus one WebSocket stream).

**Status: alpha.** The ledger format and the Session Endpoint are versioned,
but APIs and on-disk layouts can still change between releases.

**Platforms.** macOS on Apple silicon is the primary platform, and some
recovery tests run only there. Linux on x86_64 and aarch64 builds and passes
the same tests in CI and confines tools with its own sandbox (Landlock plus
seccomp, which needs a kernel with Landlock enabled). On both, the host
application starts the Kernel; see [Application-owned launch](docs/builtin-launch.md).
Windows is not supported.

**Clients.** The native Tekes client is maintained separately and is not part
of this repository; its Kernel integration is currently suspended. A browser
client and the full protocol contract are, so any client can be built against
the [Session Endpoint](spec/session-endpoint.md).
Some fixtures and documents record provenance from earlier, closed-source Tekes
repositories (names and pinned revisions only).

## Design principles

1. **A service, not a client.** The supervisor runs each active line of work
   in its own worker process and serves clients over the Session Endpoint.
   A client can disconnect and reconnect without stopping the work, because
   no client hosts the agent loop. See the
   [supervisor](docs/runtime/supervisor.md) and the
   [Session Endpoint](spec/session-endpoint.md).
2. **The ledger is the truth, and a crash loses nothing.** A live worker holds
   a file lock; durable events record what happened. A tool call is written
   before it runs and a provider attempt before it is sent, so a crash can
   leave a recorded call that never ran but never an effect without a record.
   A turn is settled only after its terminal event is durable **and** the
   writer has exited, and recovery is simply a new worker run over the same
   ledger. Client streams, indexes and projections are derived from the
   ledger and can be rebuilt, except for narrowly scoped protocol carriers
   that preserve externally visible bytes and identities. See
   [thread fundamentals](docs/concepts/thread.md),
   [durability](docs/data/durability.md) and the
   [tail lifecycle contract](spec/tail-lifecycle.md).
3. **The sandbox is mandatory and fails closed.** Tools run under an OS profile
   derived from the same policy as the permission checks: Seatbelt on macOS,
   Landlock plus seccomp on Linux. When the profile cannot be applied, the
   tool is refused; it never runs unconfined. See the
   [sandbox profile](spec/sandbox-profile.md) and
   [tool permissions](docs/runtime/tool-permissions.md).
4. **Approvals are durable.** An approval request or a question to the user is
   a ledger event. A long wait parks the worker without settling the turn, and
   the answer resumes the same turn after any restart. Running a command
   unsandboxed needs a grant bound to that exact run, call and policy; it is
   never a standing setting. See the
   [approval gate](docs/runtime/tool-permissions.md#approval-gate) and
   [approval policy](spec/session-endpoint.md#approval-policy).
5. **A substrate, not a platform.** Long-term memory is a separate MCP service.
   Evolving prompts, tools and binaries goes through git, CI and the host
   application that chooses which build to launch, rather than through Kernel
   machinery. The Kernel contributes what
   those need: each run records the binary, configuration and instruction
   snapshot it used. See [evolution boundaries](docs/history/evolution.md).
6. **Contracts come before code.** Event formats and protocols are specified in
   `spec/` with language-neutral fixtures, and Rust is the first reference
   implementation (decision D-63 in the [decision records](docs/history/decisions.md)).
   When code and contract disagree, the discrepancy is recorded and resolved
   against the contract rather than by redefining it.
7. **Prompts are lean and measured.** Each built-in profile is under 500
   characters. Their behavior rules were added or removed according to
   controlled benchmarks: on DeepSeek Flash the coding rules cut cost by more
   than half without lowering test quality, and per-tool usage paragraphs that showed no effect
   were dropped. See the [cost benchmark](docs/benchmarks/deepseek-cost-2026-10.md)
   and the [system prompt](docs/runtime/system-prompt.md).

## What is in this repository

The Session Endpoint server (protocol version 3), the provider and tool
runtimes, an MCP client, plugin, schedule and thread-search services, a
browser client.
The endpoint registers 16 unary methods plus one multiplexed WebSocket stream
(`remote.mux`); optional capabilities are registered separately. See the
[Session Endpoint contract](spec/session-endpoint.md#routes-and-streams) and
[Client extensions](spec/client-extensions.md).

The host application launches the Kernel as its own child process and supplies
the endpoint token and provider credentials. The earlier launchd service, installer and
selector have been removed.

## Components

| Component | Responsibility |
|---|---|
| [Schema and store](docs/data/storage.md) | Validate, append and replay durable events and assets |
| [Worker and engine](docs/flows/turn.md) | Build model context, call providers, run tools and settle turns |
| [Supervisor](docs/runtime/supervisor.md) | Launch and reconcile workers, serve Client requests and coordinate integrations |
| [Endpoint and transport](docs/interfaces/client.md) | Project ledger history and deliver the V3 Client protocol |
| [Workspace service](docs/workspace-service-wse.md) | Run file and Git operations in a child process under a resolved workspace root |

New sessions choose a `coding` or `general` identity from the first user request
before the first provider call; an ambiguous request defaults to `coding`.
The choice is durable for that session. A separately stored active goal can
continue work across settled turns, subject to its phase, round bound and user
input. See [identity selection](crates/worker/src/identity.rs) and the
[goal contract](spec/session-endpoint.md#goals).

## Quick start

The toolchain is pinned in [rust-toolchain.toml](rust-toolchain.toml)
(Rust 1.85.1). Build the four executables and run the launch smoke test, which
starts a supervisor with a synthetic provider, creates a workspace and a
session, and checks every stream baseline:

```sh
cargo build --locked -p tekes-supervisor -p tekes-worker -p tools -p workspace-service \
  --bin tekes-supervisor --bin tekes-worker --bin tekes-helper --bin tekes-workspace-service
python3 scripts/test-builtin-launch.py
```

To run it yourself, give `tekes-supervisor --built-in launch.json` a launch
document with your provider routes and the environment variable that holds
each key; `tekes-supervisor --models-available` lists the provider and model
routes the build supports. The launch document, authentication token and
readiness record are described in [Application-owned launch](docs/builtin-launch.md).

## Build and verify

```sh
scripts/ci.sh
```

runs everything a pull request must pass: `cargo fmt --check`, `clippy -D
warnings`, `cargo test --workspace`, the fixture checks and the Python tests.
None of it needs network access or provider keys. Tests that call real
providers are `#[ignore]`d and driven by `scripts/run-live-*.py`; see
[scripts/README.md](scripts/README.md) and [Running tests](docs/verification/running-tests.md).
The test harness finds `fixtures/` automatically, or reads an absolute
`TEKES_KERNEL_FIXTURES` path.

## Where to read

| Need | Start here |
|---|---|
| Understand threads, turns, workers and settlement | [Documentation overview](docs/README.md) → [Thread fundamentals](docs/concepts/thread.md) |
| Follow a request through execution and delivery | [Turn flow](docs/flows/turn.md) → [Client interface](docs/interfaces/client.md) |
| Integrate a Client or extension | [Session Endpoint](spec/session-endpoint.md) → [Client extensions](spec/client-extensions.md) |
| Find implementation by process, crate or symbol | [Architecture](docs/architecture/README.md) → [Crate map](docs/architecture/crates.md) |
| Check persistence and recovery rules | [Storage](docs/data/storage.md) → [Durability](docs/data/durability.md) |
| Review tests and evidence | [Verification](docs/verification/README.md) |
| See measured cost against another agent | [Cost benchmark, October 2026](docs/benchmarks/deepseek-cost-2026-10.md) |
| Understand earlier designs and migrations | [History](docs/history/README.md) |

`spec/` contains precise contracts; `docs/` explains the system and records
verification. Crate guides sit beside code in `crates/<name>/docs/`. The
generated architecture atlas is refreshed and checked with
`scripts/code-architecture.py`; its [method page](docs/architecture/method.md)
lists the Python dependencies and limits of static call graphs. Historical
plans and reviews describe their time of writing, not the present runtime.

## Working on the code

Start with the relevant contract and source owner, then run the focused tests
and update its documentation. The [crate map](docs/architecture/crates.md)
identifies each package. Changes to Rust source or Cargo configuration may
require regenerating the architecture atlas; use the check and link commands
in the [maintenance guide](docs/architecture/method.md). See
[CONTRIBUTING.md](CONTRIBUTING.md) for the pull request checklist and
[SECURITY.md](SECURITY.md) for reporting vulnerabilities.

TekesKernel is licensed under the [MIT License](LICENSE).
