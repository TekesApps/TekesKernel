# TekesKernel

TekesKernel is a local execution kernel for AI coding and general-purpose
agents. It records every session in an append-only JSONL ledger, runs each
active line of work in its own worker process, and uses a supervisor to manage
workers, tools, model providers and client connections. Clients talk to it
over the Session Endpoint (HTTP plus one WebSocket stream).

**Status: alpha.** The ledger format and the Session Endpoint are versioned,
but APIs and on-disk layouts can still change between releases.

**Platforms.** macOS on Apple silicon is the supported platform: the shell
sandbox (Seatbelt), credential storage (Keychain), the LaunchAgent and the
installer are macOS-specific, and some recovery tests run only there. The
workspace also builds on Linux, where those components are absent; Linux and
Windows are not supported targets.

**Clients.** The native Tekes client is maintained separately and is not part
of this repository. A browser client and the full protocol contract are, so
any client can be built against the [Session Endpoint](spec/session-endpoint.md).
Some fixtures and documents record provenance from earlier, closed-source Tekes
repositories (names and pinned revisions only).

The central rule is simple: a live worker holds a file lock; durable events
record what happened. A turn is settled only after its terminal event is durable
**and** the writer has exited. Client streams and projections are derived from
the ledger, except for narrowly scoped protocol carriers that preserve
externally visible bytes and identities. See [thread fundamentals](docs/concepts/thread.md)
and the [tail lifecycle contract](spec/tail-lifecycle.md) for the precise rules.

## What is in this repository

The Session Endpoint server (protocol version 3), the provider and tool
runtimes, an MCP client, plugin, schedule and thread-search services, a
browser client, and the macOS deployment components (installer and selector).
The endpoint registers 16 unary methods plus one multiplexed WebSocket stream
(`remote.mux`); optional capabilities are registered separately. See the
[Session Endpoint contract](spec/session-endpoint.md#routes-and-streams) and
[Client extensions](spec/client-extensions.md).

Passing the tests does not by itself qualify a signed production release. The
[deployment contract](spec/deployment.md) and
[production UAT](packaging/macos/PRODUCTION-UAT.md) describe the separate
installation, signing, reboot and target-user checks.

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
Installing the signed macOS product is a separate workflow; see
[Packaging](packaging/README.md).

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
