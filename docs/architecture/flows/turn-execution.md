# Prompt to execution and client output

[Architecture entry point](../README.md) · [worker](../../../crates/worker/docs/README.md)

This is the ordinary input path in the worktree. Approval, stop, model failure, validation and recovery have their own branches;
a prompt receipt confirms only input acceptance, not completion of model output.

```mermaid
sequenceDiagram
    participant C as Client
    participant T as transport (supervisor)
    participant E as endpoint host/routes (supervisor)
    participant P as ProductionProcessHost
    participant W as worker process
    participant S as store/schema (writer process)
    participant M as provider service
    C->>T: POST /api/session.prompt
    T->>T: authentication / limits / strict envelope
    T->>E: EndpointCarrierHost.unary
    E->>E: materialize prompt / session gate / idempotency
    E->>P: SessionDeliveryAuthority.prompt
    alt worker is live
        P->>W: input over worker-control stdio
        W->>S: validate and append input durably
        W-->>P: receipt
    else no live worker
        P->>S: locked_prompt / durable input
        P->>W: schedule and launch profiled worker
        W->>W: handshake / lock / run_decision
    end
    E-->>T: accepted response after durable handoff
    T-->>C: input acceptance
    W->>S: turn_open
    loop model / tools / continuation within one turn
        W->>W: project_provider_context / provider.prepare
        W->>P: provider lease request
        P-->>W: lease decision
        alt lease granted
            W->>S: attempt and optional attempt_dispatched
            W->>M: HTTP/SSE via provider.HttpRuntime
            M-->>W: chunks and completion
            W-->>P: stream frames over stdout
            W->>S: output or error (carrying usage) / tool calls
            opt tool calls or nonfinal response
                W->>W: execute tools or continue model loop
            end
        else lease denied
            Note over W,S: no new attempt, leave open tail for recovery and exit loop
        end
    end
    opt completion candidate
        W->>W: validation_runtime.advance
        Note over W,S: validation may launch child, request repair, hold, or durably settle
    end
    W-->>P: appended / execution state
    P->>E: Projector / durable carrier / v3 streams
    E-->>T: journal / control / inventory frames
    T-->>C: /api/remote.mux stream frames
```

Request preparation and lease admission precede attempt persistence; see the worker functions and provider-runtime spec for exact commit order.
Stream forwarding and durable journal updates can interleave; the sequence diagram does not combine streamed output and durable results into one fact.
V3 `journal-transient` does not advance the durable journal cursor; final authoritative events or a reconnect snapshot replace transient presentation.

## Concrete call chains

- `transport::unary_inner` → `EndpointCarrierHost::unary` → supervisor endpoint dispatch/routes.
  Production trait bindings come from the daemon's carrier wiring, not from matching method names alone.
- `ProductionProcessHost::prompt` writes stdio and waits for a receipt when the worker is live; otherwise it uses `locked_prompt`.
- `worker::run` → `run_provider_turn` → `run_provider_turn_inner`.
- `run_provider_turn_inner` → `project_provider_context` / `provider::prepare` / `HttpRuntime`，
  Completion then selects `append_terminal`, error, tool or continuation branches.
- In the worktree, root final answers enter `validation_runtime::advance`, which writes the durable completed settlement and exact final-output reference. Existing validation candidates continue their earlier review flow on recovery.

## Source evidence

[transport](../../../crates/transport/src/server.rs) ·
[endpoint routes](../../../crates/supervisor/src/endpoint_host.rs) ·
[process host](../../../crates/supervisor/src/process_host.rs) ·
[worker](../../../crates/worker/src/main.rs) ·
[validation](../../../crates/worker/src/validation_runtime.rs) ·
[projection](../../../crates/endpoint/src/projection.rs) ·
[v3 protocol](../../../spec/session-endpoint.md#routes-and-streams)

Runtime evidence for the validation migration belongs to the [separate audit record](../../verification/audits/validation-migration.md); this diagram does not upgrade its acceptance status.
