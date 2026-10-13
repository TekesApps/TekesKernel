# MCP runtime v1 fixtures

These are hand-authored byte oracles for `spec/mcp-runtime.md`. JSON files
are RFC-8785 canonical and end in one LF; JSONL transcripts contain one
canonical JSON-RPC object per line. `sentinel-token` and credential ids are
non-secret test values.

The corpus covers legacy and modern handshake, all catalog families, tool
execution, cancellation/task methods, standalone management publication,
catalog projection, and the closed recovery matrix. Live HTTP/OAuth tests use
the same shapes but inject credentials at request time and must prove that no
fixture, registry, log, or launch binding contains the resolved value.

`management-save.canonical.json` freezes the closed keyed field-operation DTO;
`management-receipt.canonical.json` contains only a sentinel credential id and
digests, never credential material. OAuth fixtures model reference lifecycle,
not an unimplemented token writer.

`recovery-matrix.canonical.json.executable_cases` names the required live
fault/race cases. A table-only enum assertion does not satisfy Gate 92.

`tasks-extension-*.jsonl` freeze the tasks extension (SEP-2663) as implemented
by the official Rust SDK `rmcp` 3.x: an extension-only handshake in which the
client declares `extensions["io.modelcontextprotocol/tasks"]`, a task-creating
`tools/call` answered with a flat `resultType:"task"` object, `tasks/get`
polling to `completed` (inline `result`), `input_required` (`inputRequests`
then `tasks/update`), `cancelled` and `failed` (inline `error`), the `-32021`
refusal when the declaration is missing, and a modern (`2026-07-28`)
transcript carrying the declaration in request `_meta`. Each transcript after
the handshake continues the session of `tasks-extension-handshake.jsonl`
(request ids resume at 2) except the modern one, which opens with its own
`server/discover`. `crates/mcp/tests/tasks_extension.rs` replays every client
line byte-for-byte against `McpClient` and feeds the server lines back; the
`clientInfo.version` of `initialize` is normalised to `0.1.0`.
`legacy-handshake.jsonl` and `modern-discover.jsonl` carry the same client
declaration; their servers still advertise the SEP-1686 top-level `tasks`.
