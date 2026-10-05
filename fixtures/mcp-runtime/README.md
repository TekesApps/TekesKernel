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
