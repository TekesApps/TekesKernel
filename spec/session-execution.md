# Session execution (0.9.0)

This is the current execution contract. It replaces the management-backed
creation and scope rules in the 0.8 and 0.7 specifications. Those versions are
not compatibility targets. Existing data is not automatically converted.

## Ownership

The Kernel owns Session identity, its append-only ledger, execution, cancellation,
recovery and enforcement. The host owns named configurations, directory selection,
grouping, filtering, relocation and approval UI. A directory is a path in an
execution value, not a shared Kernel management entity.

`session.create` accepts `execution` and optional `sessionId`, `identityProfile`
or `preset` (the latter two are exclusive). It does not accept `workspaceId`,
top-level `cwd`, `environment`, logical Folder IDs or a `trusted` flag.

```json
{"execution":{"cwd":"/absolute/project","read_roots":[],"writable_roots":[],"toolchain_roots":[],"network":false,"allowed_tools":["read","grep"],"max_wall_seconds":300}}
```

Only `cwd` is required. Network defaults to false; omitted root/tool sets are
empty. `cwd` and `read_roots` form file read authority. Writable roots must lie
within that authority. Toolchain roots are explicit read-only installations.
The time limit is optional and, when supplied, a positive safe integer.
Provider transport retains its independent connection authority.

Paths are canonicalized before creation is journaled. The immutable execution
value is recorded in genesis. Replacing a recorded path with an alias fails
the next launch rather than silently redirecting the Session. Presets and
host-owned instructions can narrow the execution ceiling; they cannot widen it.
Session model selection remains separate from file/process authority.

## Execution and durability

The execution path does not read Environment, workspace documents or Folder
catalog/trust for configuration. It neither registers a directory nor binds
Session lifetime to a shared directory identity. Genesis has no workspace,
environment, folder binding or catalog references. Its required ledger reader is
5, preventing older executors from misinterpreting the authority.

The endpoint advertises protocol version 4; 0.8 clients must not treat this
breaking execution surface as endpoint v3. Private bootstrap stays version 1.

The Session journal lives at `session-operations/`, separate from retired
management transactions. Creation stages config, instructions and genesis before
publishing an operation intent. Retries and recovery use that durable intent;
forks inherit execution by value. Inventory uses the recorded cwd.

Archiving is `session.archive`; restoration is `session.unarchive`. Workspace
creation, rename and relocation are absent from the public base route table.
Application bootstrap does not install Environment/Folder/workspace policy
administration or scan workspace resources. Workspace sync is not a required
or advertised base capability.

## MCP and host boundary

`execution.mcp` selects connections by value. It never contains executable
commands, process environment, directory trust flags or references to a global
server registry.

```json
{"cwd":"/absolute/project","mcp":[{"name":"local","transport":{"type":"stdio","channel":"local"}},{"name":"remote","transport":{"type":"http","url":"https://mcp.example/api","credential_id":"mcp-example"}}]}
```

Kernel owns protocol negotiation, tool discovery/projection, invocation,
cancellation, reconnect contract checks and Session isolation. It accepts HTTP
connections (HTTPS, or HTTP loopback) and host-provided bidirectional stdio
streams. Each Session binding owns its peers; replacing or dropping one binding
does not replace another Session's connections. A reconnect must preserve the
original tool and server contract.

The host owns local process startup, environment/cwd, shutdown/reaping, server
installation, configuration discovery, OAuth and authorization decisions. A
stdio channel is an opaque host capability. The `StdioConnector` interface returns
streams only. The built-in adapter connects to owner-private Unix sockets; it
does not spawn servers. Missing channels fail closed.

HTTP credentials arrive through the private bootstrap secret store. MCP tokens
use a purpose- and destination-bound envelope:
`{"kind":"mcp","url":"https://mcp.example/api","token":"..."}`.
A provider key or a grant for another URL cannot be used. The selected credential
generation is rechecked on requests; rotation/revocation requires a fresh binding.
Forks inherit explicit connection selections by value, including destination-bound
host capabilities. There is no Kernel credential-management registry.

The independent [CLI host](../host/README.md) starts Kernel and optional local MCP
processes without Tekes, ClientKit or GUI dependencies. App hosts can implement
the same stream-provider interface with their own process lifecycle.

Tool catalogs retain supported tools when another tool's schema cannot be
projected. This execution surface exposes MCP tools; generic MCP resources and
prompts remain protocol-library APIs, not Kernel management/browser routes.

## Security and removed management

Private pipe/FD bootstrap, authenticated endpoint admission, protected Kernel,
credential and user-agent roots, worker sandboxing and credential isolation remain
enforced. No client-authored Boolean grants project-code trust. Project text may
be read under explicit roots, but project skills/hooks are not auto-loaded;
host-prepared user/Session sources supply executable instructions.

Environment, Folder catalog/trust, workspace CRUD/relocation, MCP registry/OAuth,
plugin installation and scheduling are absent from Kernel application assembly.
Session operations retain a versioned durable journal with execution digests;
old creation shapes and old journal formats are rejected, never converted.
Standalone host utility crates are not mounted as Kernel management services.
Inventory and title search use Session identity without Folder bindings; context
search supports current, explicit or all Sessions, without a project registry filter.
ClientKit/Tekes consumption is a separate stage. No rendering or workspace/RunView
changes are part of this Kernel work.
