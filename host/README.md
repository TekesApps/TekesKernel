# Standalone Kernel host

This Python 3 standard-library launcher runs Kernel without Tekes or ClientKit.
Kernel connects to MCP streams; this host owns local MCP processes. It is a
headless launcher for authenticated RPC clients, not an interactive chat UI.

Build with `cargo build -p tekes-supervisor -p tekes-worker` and run:

```sh
python3 host/kernel_cli.py --launch /absolute/host.json \
  --supervisor /absolute/target/debug/tekes-supervisor
```

`host.json` has the ordinary built-in Kernel launch fields (`format`,
`bootstrap_version`, `root`, `worker`, `listen`, `providers`,
`credential_bindings`) and an optional `local_mcp` object:

```json
{"local_mcp":{"filesystem":{"command":["/absolute/mcp-server","--stdio"],"cwd":"/absolute/project","environment":{"LANG":"en_US.UTF-8"}}}}
```

Set the file mode to `0600`. Commands must use absolute executable paths;
arguments and environment are passed directly, without a shell or inherited host
secrets. The host creates private Unix sockets and supplies `stdio_channels` to
Kernel. Do not configure `stdio_channels` yourself in this launcher.

An authenticated client creates a Session with this explicit execution selection:

```json
{"execution":{"cwd":"/absolute/project","mcp":[{"name":"filesystem","transport":{"type":"stdio","channel":"filesystem"}}]}}
```

The caller supplies the normal `TEKES-KERNEL-BOOTSTRAP/1` length-prefixed private
frame through stdin and keeps the pipe open for Kernel's lifetime. The host
forwards stdin and readiness stdout unchanged. Credentials never travel in
command arguments or environment variables. Closing stdin shuts Kernel down;
closing each stream terminates and reaps its MCP process group. Reconnect opens
a fresh host process. HTTP-only Sessions need no local server mapping.

Host regression:

```sh
python3 -m unittest discover -s host -p 'test_*.py'
TEKES_TEST_CLI_HOST=1 python3 scripts/test-builtin-launch.py
```

The second command runs the actual supervisor and worker with a local mock model
provider and invokes a real host-started stdio MCP tool. No remote account or App
is required. App hosts may implement the Rust `StdioConnector` interface instead
of using this reference launcher.
