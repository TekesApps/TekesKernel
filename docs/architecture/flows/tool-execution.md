# Tool execution, approval, and routing

[Architecture entry point](../README.md) · [engine](../../../crates/engine/docs/README.md) · [tools](../../../crates/tools/docs/README.md)

Fixed tools enter through `dispatch_with_pre_mutation`. Dynamic tools have a separate dispatcher but also require explicit execution bindings;
a tool name must not be treated as an arbitrary executable command.

```mermaid
sequenceDiagram
    participant W as worker
    participant D as engine.ToolDispatcher
    participant P as tools.ToolPipeline
    participant A as hook / policy
    participant B as selected backend
    participant H as helper or supervisor / MCP
    participant S as store/schema
    W->>S: durable tool_call
    W->>D: dispatch
    D->>D: catalog / backend / argument checks
    D->>P: verify_durable_execution and execute pipeline
    P->>A: pre-hook
    opt hook mutates invocation and processing continues
        P->>S: persist effective_execution
    end
    P->>A: policy gate on effective invocation
    alt approval is required
        P->>S: durable approval_request
        P-->>W: hold
        Note over W,S: Client answers v3 actionable, response enters durable worker control
        W->>P: resume with durable approval_response
    else denied or invalid
        P->>S: paired terminal result where pipeline owns execution
        P-->>W: failure decision
    end
    opt authorized executable invocation
        opt workspace mutation
        end
        D->>B: execute or resume_after_approval
        B->>H: helper protocol OR supervisor tool_control
        Note over B,H: supervisor can route MCP call / job / child operation
        H-->>B: correlated terminal result
        opt workspace mutation
        end
        B-->>P: backend terminal
        P->>A: post-hook / secret scan as applicable
        P->>S: durable tool_result
        P-->>W: pipeline decision
    end
```

In the diagram, `opt authorized` excludes the earlier rejection path; it does not imply execution after rejection. See the pipeline for exact hook, rejection and recovery branches.
Some system/Web backends run inside the worker without a helper; helper and supervisor are also different targets.

## IPC variants

| Backend | Crossing | Evidence |
|---|---|---|
| helper | worker → tekes-helper → optional command | [helper](../../../crates/tools/src/helper.rs) |
| supervisor control | worker stdio tool_control → host handler → correlated result | [worker exchange](../../../crates/worker/src/main.rs), [production handler](../../../crates/supervisor/src/production_tool_control.rs) |
| MCP | supervisor runtime → pooled client → stdio/HTTP server | [runtime](../../../crates/supervisor/src/mcp_runtime.rs), [MCP transport](../../../crates/mcp/src/transport.rs) |
| job | supervisor JobBroker → tekes-helper --job-runner → command | [runtime backends](../../../crates/tools/src/runtime_backends.rs) |

## Source evidence

[dispatcher](../../../crates/engine/src/dispatcher.rs) ·
[dynamic dispatcher](../../../crates/engine/src/dynamic_catalog.rs) ·
[pipeline](../../../crates/tools/src/pipeline.rs) ·
[worker tool batch](../../../crates/worker/src/tool_calls.rs) ·
[tool contract](../../../spec/tool-runtime.md)
