# tools — tool catalog, execution pipeline and helper

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Provides tool schemas, hooks, approvals, sandboxing and concrete execution backends; also builds tekes-helper.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `builtin / schema_registry` | Fixed tool catalog, visibility conditions, model schemas and argument validation |
| `pipeline` | Durable execution, approvals, hooks and persisted results |
| `hook` | Before/after hook protocol and processes |
| `helper` | Helper request/response and file/exec operations |
| `sandbox` | Darwin/Linux policies and probes |
| `runtime_backends` | Helper, HTTP/Web, job and cancellation backends |
| `bin/tekes-helper` | Program entry point for the stdio helper or --job-runner |

## Interfaces and calls

Main entry points: ToolPipeline, BuiltinManifest, HelperClient, HelperServer, JobBroker, SandboxPolicy.

engine dispatcher → ToolPipeline → hook/policy → backend → helper or HTTP/job; tool results return to the pipeline for persistence.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Job runner is a mode of tekes-helper, not an additional Cargo crate; tool schemas and actual backend availability jointly determine the catalog.

`HelperClient::sandboxed_with_scratch` owns a mode-0700 temporary directory across client clones. Worker shell environments use its path as `TMPDIR`; the final clone releases the directory. Declared toolchain roots are read-only inputs and supply explicit `bin` search paths. They do not become project folders or writable roots.
`summary_artifact` takes `evidence_refs` as ledger `seq`
integers (compaction v5); it is a compactor-role tool, never model-visible in
an ordinary turn.

`BackendTerminal::Pending` carries a remote continuation bound by the
supervisor; `ToolPipeline::append_continuation_step` writes the durable
`state{tool_continuation}` steps (bind/query/update/cancel/park) and
`complete_continuation` pairs the call with exactly one terminal `tool_result`.

Behavior contract: [tool-runtime.md](../../../spec/tool-runtime.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
