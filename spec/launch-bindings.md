# Launch bindings v1

This contract owns immutable host-resolved values that are specific to one
worker launch and therefore cannot be authored in `config`: the optional
host goal identity and the resolved dynamic plugin/MCP tool catalog.
It is an additive Slice-8 contract and does not change ConfigSnapshot bytes.

## Descriptor and binding

The host supplies one already-open read-only descriptor plus the lowercase
SHA-256 of its exact bytes. The worker verifies the digest, parses exactly one
RFC-8785 canonical object plus LF, and validates `workspace` and
`config_digest` against its already-verified ConfigSnapshot before using any
field:

```text
LaunchBindings = {
  format: 1,
  workspace: string,
  config_digest: 64 lowercase hex digits,
  goal_id?: nonempty string <= 128 UTF-8 bytes without ASCII control,
  dynamic_catalog: DynamicCatalog
}
```

An absent `goal_id` disables `new_goal` and `set_goal_state`. An invalid,
empty, or unbound id aborts launch; the worker never synthesizes goal identity.

## Declared dynamic catalog

```text
DynamicCatalog = {format: 1, tools: [DynamicTool, ...]}
DynamicTool = {
  name: string,
  source: {kind: "plugin" | "mcp", id: string},
  effect: "read_only" | "workspace_write" | "external_process" |
          "network_read" | "computer_control" | "system_permission",
  always_on: bool,
  aliases?: [string, ...],
  schema: {name, description, parameters},
  schema_digest: "sha256-" + 64 lowercase hex digits
}
```

The schema is the exact provider-facing value. Its name must equal the entry
name. Plugin parameters are a closed object schema with explicit properties,
required names, and `additionalProperties:false`. MCP parameters retain remote
object semantics: `required` may be absent, a nested object may omit
`properties`, and `additionalProperties` may be absent, boolean or a schema
for the values. The draft 2020-12 `$schema` identifier is accepted for MCP;
unknown dialects are rejected.

Every node is limited to the keyword contract: `type` (one of object, array,
string, integer, number, boolean, null), `properties`, `required`,
`additionalProperties`, `items`, `allOf`/`anyOf`/`oneOf` (nonempty arrays of
nodes), `enum` (nonempty), `const`, `default`, `description`, `title`,
`examples` (array), `format` and `pattern` (strings), `nullable`,
`uniqueItems` and `deprecated` (booleans), `minimum`/`maximum`,
`exclusiveMinimum`/`exclusiveMaximum`, `minLength`/`maxLength` and
`minItems`/`maxItems` (numbers), `$comment`, and — at the root of an MCP
schema only — `$schema`, `$id`, `$defs` and `definitions`. A `$ref` must be a
local pointer into a root `$defs`/`definitions` entry (`#/$defs/Name`);
external and non-root references are rejected. Every other keyword is
rejected, and the error names the keyword and the JSON pointer of the node
(`/parameters/properties/layout: uses unsupported keyword "not"`). This does
not change the stored schema bytes or its digest. `schema_digest` is SHA-256 over the schema's RFC-8785
bytes without LF. Every dynamic tool is executed through the correlated
supervisor-control route (worker-control `tool_control`): plugin and MCP peers
are supervisor-owned, and local tools are MCP stdio servers
([mcp-runtime](mcp-runtime.md)). There is no worker-side executable
dynamic route. Missing route dependencies omit the entry from projection and
reject dispatch.

Fixed-name and dynamic-name collisions reject the complete launch. Tools are
strictly sorted by `(source.kind, source.id, name)`; fixed tools remain before
dynamic tools in provider projection. Always-on entries are resident. Other
entries remain schema-deferred: only a successful durable `tool_search`
result in the same turn causally admits dispatch. Discovery caches and process
registries are never authority.

Dynamic dispatch verifies the durable invocation, exact arguments, causal
offer when required, ordered hooks, non-downgradable effect-to-approval class,
generic policy/hold, backend result, and secret transform through the same
ToolPipeline used by fixed tools.
