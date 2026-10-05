# Official TypeScript MCP SDK live servers

`server.mjs` runs two servers on one loopback port with the official
`@modelcontextprotocol/sdk` (pinned in `package.json`):

- `/tasks/mcp` — `tekes-official-ts-tasks`: one tool, `slow_echo`, with
  `taskSupport: required` through the SDK's experimental SEP-1686 tasks
  (`tasks/get` status, `tasks/result` payload, `pollInterval` 400 ms).
- `/conformance/mcp` — `tekes-official-sdk-conformance`: `sse_echo` behind a
  transport that never answers JSON (request-scoped SSE only), plus
  `POST /conformance/notify`.

`scripts/run-live-mcp-official-sdk.py` installs the dependencies, starts the
server, exposes it through the user's ngrok agent (HTTPS), proves the public
URL answers from the internet, and runs `crates/mcp/tests/live_official_sdk_tunnel.rs`
against it. `--loopback` rehearses the same gates over `http://127.0.0.1`.

Deliberately absent, because no released SDK implements them: the 2026-07-28
modern discovery handshake, `x-mcp-header` parameter headers, multi-round
`requestState`/`inputResponses` results, and the modern catalog subscription.
`node_modules/` is not committed.
