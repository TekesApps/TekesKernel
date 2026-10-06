# Crates and executable mapping

[Architecture entry point](README.md) · [Process diagram](processes.md) · [Generated dependency graph](generated/index.md)

The workspace has 24 packages. The table uses directory names as reading entry points; the generated index provides Cargo package and target names.

| Directory | Responsibility | Entry point |
|---|---|---|
| [schema](../../crates/schema/docs/README.md) | Event formats, log validation and state folding | [Cargo/targets/modules](../../crates/schema/docs/generated/index.md) |
| [store](../../crates/store/docs/README.md) | Durable logs and assets | [Cargo/targets/modules](../../crates/store/docs/generated/index.md) |
| [worker-control](../../crates/worker-control/docs/README.md) | Internal supervisor-worker protocol | [Cargo/targets/modules](../../crates/worker-control/docs/generated/index.md) |
| [profile](../../crates/profile/docs/README.md) | Configuration and immutable launch snapshots | [Cargo/targets/modules](../../crates/profile/docs/generated/index.md) |
| [engine](../../crates/engine/docs/README.md) | Execution rules and tool coordination | [Cargo/targets/modules](../../crates/engine/docs/generated/index.md) |
| [worker](../../crates/worker/docs/README.md) | Executable for a log execution run | [Cargo/targets/modules](../../crates/worker/docs/generated/index.md) |
| [supervisor](../../crates/supervisor/docs/README.md) | Resident process and production wiring | [Cargo/targets/modules](../../crates/supervisor/docs/generated/index.md) |
| [provider](../../crates/provider/docs/README.md) | Model protocols and transport | [Cargo/targets/modules](../../crates/provider/docs/generated/index.md) |
| [endpoint](../../crates/endpoint/docs/README.md) | Client semantics and durable carriers | [Cargo/targets/modules](../../crates/endpoint/docs/generated/index.md) |
| [transport](../../crates/transport/docs/README.md) | HTTP and WebSocket transport | [Cargo/targets/modules](../../crates/transport/docs/generated/index.md) |
| [mcp](../../crates/mcp/docs/README.md) | MCP client, connection pool and management | [Cargo/targets/modules](../../crates/mcp/docs/generated/index.md) |
| [plugins](../../crates/plugins/docs/README.md) | Plugin package lifecycle | [Cargo/targets/modules](../../crates/plugins/docs/generated/index.md) |
| [schedule](../../crates/schedule/docs/README.md) | Durable scheduling | [Cargo/targets/modules](../../crates/schedule/docs/generated/index.md) |
| [thread-search](../../crates/thread-search/docs/README.md) | Rebuildable session title search | [Cargo/targets/modules](../../crates/thread-search/docs/generated/index.md) |
| [tools](../../crates/tools/docs/README.md) | Tool catalog, execution pipeline and helper | [Cargo/targets/modules](../../crates/tools/docs/generated/index.md) |
| [selector](../../crates/selector/docs/README.md) | External deployment selection and supervision (retired) | [Cargo/targets/modules](../../crates/selector/docs/generated/index.md) |
| [product-installer](../../crates/product-installer/docs/README.md) | Signed product installation (retired) | [Cargo/targets/modules](../../crates/product-installer/docs/generated/index.md) |
| [test-support](../../crates/test-support/docs/README.md) | Shared test infrastructure | [Cargo/targets/modules](../../crates/test-support/docs/generated/index.md) |
| [conformance](../../crates/conformance/docs/README.md) | Cross-crate conformance tests | [Cargo/targets/modules](../../crates/conformance/docs/generated/index.md) |
| [deployment-tests](../../crates/deployment-tests/docs/README.md) | Deployment test harness | [Cargo/targets/modules](../../crates/deployment-tests/docs/generated/index.md) |
| [host-files](../../crates/host-files/docs/README.md) | Host directory and session reference Client methods | [Cargo/targets/modules](../../crates/host-files/docs/generated/index.md) |
| [session-controls](../../crates/session-controls/docs/README.md) | Durable goal controls and delegated child inventory | [Cargo/targets/modules](../../crates/session-controls/docs/generated/index.md) |
| [user-documents](../../crates/user-documents/docs/README.md) | Feedback and user settings documents | [Cargo/targets/modules](../../crates/user-documents/docs/generated/index.md) |
| [workspace-service](../../crates/workspace-service/docs/README.md) | Isolated workspace file, Git and process operations | [Cargo/targets/modules](../../crates/workspace-service/docs/generated/index.md) |

## Process to crate mapping

The table below lists direct local dependencies available to these targets through package configuration. Consult call graphs for actual use and Cargo graphs for transitive dependencies. A package's library and binaries compile separately.

| Executable target | Package | Direct local library dependencies |
|---|---|---|
| `tekes-helper` | `tools` | `tools (same package library)`, `schema`, `store` |
| `mcp-fixture-server` | `mcp` | `mcp (same package library)`, `plugins`, `profile`, `schema`, `store` |
| `tekes-selector` | `tekes-selector` | `tekes-selector (same package library)` |
| `tekes-kernel-installer` | `tekes-kernel-installer` | `tekes-kernel-installer (same package library)` |
| `tekes-supervisor` | `tekes-supervisor` | `tekes-supervisor (same package library)`, `endpoint`, `engine`, `host-files`, `mcp`, `plugins`, `profile`, `provider`, `schedule`, `schema`, `session-controls`, `store`, `thread-search`, `tools`, `transport`, `user-documents`, `worker-control`, `workspace-service` |
| `tekes-worker` | `tekes-worker` | `engine`, `profile`, `provider`, `schema`, `session-controls`, `store`, `tools`, `worker-control`, `workspace-service` |
| `tekes-workspace-service` | `workspace-service` | `workspace-service (same package library)` |

`mcp-fixture-server` is a test tool, not a resident product service. `tekes-helper --job-runner` and the ordinary helper use the same target.

## Reading the graph

Cargo dependency arrows identify dependencies available at compile time, not a guarantee that each is called on every run. The generated index also has a cross-package direct call graph; its counts are source call sites.

Libraries declare modules with mod; pub use can re-export public types from private modules. The interface inventory retains declaration visibility and export locations rather than treating the pub count as the external API count.
