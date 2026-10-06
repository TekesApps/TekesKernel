# Glossary

[Documentation home](README.md) · [Thread fundamentals](concepts/thread.md)

Core runtime terms (thread, line, session, worker, run, turn, role, settlement)
are defined in [Thread fundamentals](concepts/thread.md#terminology-and-boundaries).
This page explains the other terms that appear across `docs/`, `spec/`, test
names and commit messages, most of which come from how the project was built.

## Project history terms

| Term | Meaning |
|---|---|
| **Slice N** (Slice 1 … Slice 14F) | An implementation milestone. Each slice delivered one area (for example Slice 13 is the MCP client) together with its conformance gates. Test files named `sliceN_*.rs` and scripts named `ci-sliceN.sh` belong to that milestone. The plan is in [Implementation plan](history/implementation-plan.md); slices do not describe the current architecture. |
| **gate**, **Gate N** | A numbered conformance test that must pass for a slice to count as done, for example `slice13_gate_91_production_assembly_mounts_and_executes_management`. Gate numbers are stable identifiers, not an order of importance. The mapping is in [Gate rollout](history/gate-rollout.md). |
| **D-NN** (for example D-46, D-70) | A design decision record. The number identifies the decision and its rationale in [Design decisions](history/decisions.md). |
| **R1 … R17** | Architecture review rounds during the design. Their records are not part of the public documentation; decisions they produced are in the D-NN records. |
| **legacy**, **predecessor** | The earlier closed-source Swift runtime (TekesRuntime) and app server (TekesAppServer) that TekesKernel replaced. Fixtures that pin their names and revisions record where a behavior was ported from. |

## Protocol and client terms

| Term | Meaning |
|---|---|
| **Session Endpoint** | The HTTP and WebSocket protocol clients use to talk to the supervisor. Contract: [spec/session-endpoint.md](../spec/session-endpoint.md). |
| **V2**, **V3** | Versions of the Session Endpoint protocol. V3 is current (`protocolVersion: 3`); V2 names remain on some durable carriers whose bytes predate V3. |
| **`remote.mux`** | The multiplexed WebSocket route of the Session Endpoint. It carries the five synchronization streams: workspace, session inventory, session journal, session control, and actionables. |
| **unary method** | A request/response call `POST /api/{method}`; the endpoint registers 16. |
| **client extension**, **capability** (`*.v1`) | An optional group of methods registered beside the 16 unary methods, for example `tools.v1` or `initialPresets.v1`. Contract: [spec/client-extensions.md](../spec/client-extensions.md). |
| **carrier** | A durable record whose exact bytes and identity must be preserved for a protocol or provider, rather than recomputed from the ledger, for example an exact-retry intent or a native provider item. |
| **DSH** | DeepSeek Harness, an open-source (MIT) agent harness by DeepSeek. Parts of the Session Endpoint were aligned with its client protocol ("DSH-compatible"), and it served as a reference in design comparisons. |
| **native client** | The Tekes desktop application, maintained separately and not open source. |

## Runtime terms

| Term | Meaning |
|---|---|
| **ledger** | A thread's append-only JSONL event log (`main.jsonl` and child logs). The durable source of truth; see [Storage](data/storage.md). |
| **epoch** | A period of a line in which the provider, model, system text and tool catalog stay fixed. Changing any of them (for example a model switch or a system prompt change) opens a new epoch; see [Model context](data/context.md). |
| **projection** | A view derived from the ledger (client history, model context, inventory). Projections are rebuilt, never the source of truth. |
| **settle**, **settlement** | The single durable event that ends a turn with `completed`, `interrupted` or `error`. |
| **validator** | An optional child execution that checks a candidate answer before settlement. |
| **compaction**, **compactor** | Replacing older history in the model context with a summary when the context grows too large; the compactor is the role that writes the summary. |
| **dialect**, **profile** | A dialect is one provider wire protocol (for example `deepseek_responses_v1`); a model profile binds a dialect to an exact model id and its capabilities. See [spec/provider-dialect-profiles.md](../spec/provider-dialect-profiles.md). |
| **identity profile** | The `coding` or `general` text a session's system prompt starts with; see [System prompt](runtime/system-prompt.md). |
| **workspace service** | A child process that performs file and Git operations confined to one resolved workspace root. |
| **selector**, **installer** | macOS deployment executables: the selector selects and supervises the installed deployment; the installer installs and upgrades it. See [Packaging](../packaging/README.md). |
