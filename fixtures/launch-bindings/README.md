# Launch bindings fixtures

`dynamic-catalog.canonical.json` is the resolved, host-supplied catalog byte
oracle. It is intentionally not user configuration: the schema comes from a
plugin or MCP `tools/list`, but the worker receives only this immutable launch
document and never reconstructs it from process state. Every dynamic tool is
executed through supervisor control; local tools are MCP stdio servers.
