# Installed service startup

[Architecture entry point](../README.md) · [Process diagram](../processes.md)

Scenario: an installed macOS LaunchAgent starts the service. The installation transaction itself is covered by the deployment spec.
Selector and supervisor are processes; process host, endpoint and transport are objects/libraries inside the supervisor.

```mermaid
sequenceDiagram
    participant L as launchd
    participant S as selector process
    participant D as supervisor main/daemon
    participant P as ProductionProcessHost
    participant E as ProductionCarrierAssembly
    participant T as transport
    L->>S: serve
    S->>S: validate selection / bundle / signature
    S->>D: spawn selected executable + inherited FDs
    D->>D: run_production / run_daemon_inner
    D->>D: validate handoff, identity, root lock, storage
    D->>P: open_with_secret_store / preflight
    D->>E: assemble endpoint routes and streams
    D->>T: configure native listener
    D->>D: bind native and optional Web listener
    D-->>S: bootstrap listener_bound through status FD
    D->>T: serve native / optional Web
    Note over D,P: sweep, schedule and metrics are hosted work, not extra daemons
    alt startup validation or bind fails
        D-->>S: failed bootstrap / process exit
        S->>S: classify failure / recovery or rollback policy
    else launcher lifetime closes or termination occurs
        D->>E: begin drain
        D->>P: shutdown workers and runtime
        D->>T: graceful server shutdown
    end
```

Bootstrap listener binding is a specific milestone, not proof that a user request or provider turn succeeded.
The selector's failure policy depends on error category, deployment state and retry history; rollback in the diagram does not occur after every failure.

## Source evidence

- [Selector main](../../../crates/selector/src/main.rs): environment cleanup and command dispatch.
- [Selector implementation](../../../crates/selector/src/selector.rs): `spawn`, bootstrap status and lifetime management.
- [supervisor main](../../../crates/supervisor/src/main.rs): `--install-root` → `run_production`.
- [Daemon](../../../crates/supervisor/src/daemon.rs): `run_daemon_inner` wiring, listeners, Tokio tasks and draining.
- [carrier assembly](../../../crates/supervisor/src/endpoint_carrier.rs): `ProductionCarrierAssembly::assemble`.
- [Deployment contract](../../../spec/deployment.md): signature/restart acceptance belongs to separate release gates.
