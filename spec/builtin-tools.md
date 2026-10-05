# Builtin tools v1 — unified catalog, schemas, and execution classes

Status: **normative executable contract**. This file owns the fixed TekesKernel
tool namespace, catalog composition, model exposure, and the mapping from each
fixed tool to an execution class. The exact inventory is the canonical
`fixtures/tools/builtin-tools.canonical.json` object. A narrative document,
language implementation, plugin, MCP server, or configuration file cannot add,
remove, shadow, or silently change one of those entries.

The pre-Kernel repository names, revisions, evidence paths, and removed-name
dispositions live only in the separate
`fixtures/tools/builtin-tool-migration.canonical.json` audit fixture. They
are provenance, not ownership or runtime authorities. TekesKernel has one fixed
catalog: after migration no tool is classified as a Runtime tool or an
AppServer tool. This file plus its builtin fixture are authoritative.

## Terms and namespace

- **fixed tool** — a name whose existence, top-level argument order, semantic
  schema rules, and execution class are owned by this contract. Fixed tools
  form one Kernel catalog regardless of their pre-migration repository.
- **dynamic tool** — a tool supplied by a local manifest, plugin, or MCP
  catalog. Its name and schema are external configuration, not entries in the
  fixed manifest.
- **registered** — an implementation exists in the worker's complete tool map.
  Registration does not imply model visibility.
- **resident** — eligible for the initial model-visible tool projection.
- **role tool** — visible only in a named Kernel execution role or workflow.
- **discoverable** — registered but schema-deferred until `tool_search`
  projects a causal offer.

Fixed names and dynamic names share one namespace. Name comparison is exact,
case-sensitive Unicode scalar equality after rejecting empty names and ASCII
control characters. A dynamic tool whose name collides with any fixed name is
rejected before the worker starts. There is no "last registration wins" rule.
Two dynamic sources with the same name are also a configuration error unless a
separate configuration contract defines deterministic tier precedence before
catalog construction; after resolution, the effective catalog is unique by
name.

## Canonical manifest

The canonical fixture is one RFC-8785 JSON object followed by exactly one LF:

```text
BuiltinManifest = {
  format: 1,
  tools: [Builtin, ...]
}
Builtin = {
  arguments: [string, ...],
  availability: Availability,
  backend: Backend,
  effect: Effect,
  name: string
}
```

Objects are closed. `tools` is strictly UTF-8 byte-sorted by unique `name` and
has no owner/source field. `arguments` is the top-level model-schema property
order; every listed name occurs exactly once and no top-level property is
omitted. Conditional requiredness and value constraints belong to the tool
rules below. The manifest does not duplicate every nested JSON Schema node or
description string: typed per-tool schema definitions materialize those closed
trees, publish a canonical digest, and pass the shape/digest checks below.
Every manifest entry is a callable fixed-schema tool. Host operations and
replaced legacy names do not occupy tombstone entries.

The closed classifications are:

```text
Availability =
  "default" | "selection_controlled" | "plan_mode" |
  "role_compactor" | "role_subagent" |
  "role_validator" | "conditional_credential" |
  "conditional_deferred_catalog"

Backend =
  "in_process" | "worker_hold" | "supervisor_control" |
  "helper_fs" | "helper_exec" | "background_exec" |
  "worker_http"

Effect =
  "none" | "read_only" | "user_interaction" | "goal_state" |
  "workspace_write" | "external_process" |
  "network_read" | "thread_control" | "child_spawn"
```

Visibility is derived from `availability`; a second `exposure` field would
encode the same decision twice. `default` is resident, `selection_controlled`
is explicitly selected, role/plan values are workflow-only, and conditional
values are absent until their condition holds.

## Catalog construction

For one immutable launch profile, the worker constructs the catalog in this
order:

1. Load and validate this fixed manifest. A missing entry, duplicate, unknown
   classification, or implementation/schema mismatch is a packaging error and
   aborts launch.
2. Instantiate every fixed implementation in the single Kernel registry,
   including workflow tools that are not currently visible.
3. Apply platform and launch conditions. Absence of a condition (`web_search`
   credential, deferred catalog, plan role) means the tool is
   not exposed; it never leaves an advertised-but-unrunnable schema.
4. Resolve local-manifest, plugin, and MCP catalogs; reject fixed-name and
   effective dynamic-name collisions; then apply configuration policy.
5. Build the model projection. Resident fixed tools come first in manifest
   order, followed by role-selected fixed tools in manifest order, followed by
   explicitly always-on dynamic tools sorted by `(source, name)`. Deferred
   dynamic schemas are absent until a durable `tool_search` offer activates
   them for the next continuation.

`allowed_tools` and workspace capability policy may remove availability but
cannot add a name, replace a fixed schema, relax approval, or change an effect
classification. After fixed plus configured dynamic catalogs resolve and
before provider work, a policy reference to no effective name rejects launch;
the config snapshot itself does not invent an MCP catalog before connection.
Revoking a tool is a privilege reduction under config and requests
stop/respawn.

The provider receives only schemas selected for that request. Complete schemas
are never placed in a search document. A search entry contains bounded name,
summary, aliases/keywords/intents, namespace context, authorization class,
approval class, and schema digest. The exact schema is resolved from the same
catalog only after a causal offer. Cache files are rebuildable and never
authority.

The fixed catalog identity written as `tools_digest` is exactly
`"sha256-" + lowercase_hex(SHA-256(RFC8785(tools)))`, where `tools` is the
decoded runtime manifest's exact array and `RFC8785(tools)` is its canonical
JSON byte sequence with **no trailing LF**. The enclosing `format` field and
the separate migration/extension fixtures are not in this preimage. Each
closed array item contributes all five actual fields — `name`, `arguments`,
`availability`, `backend`, and `effect` — so changing any one of them changes
the preimage and therefore must change `tools_digest`. The expected format-1
digest and preimage length are locked by
`fixtures/tools/builtin-tools.digest.canonical.json`.

## Schema rules shared by every fixed tool

Every model-facing fixed schema is a function tool with `name` equal to the
manifest entry, a nonempty description, object parameters, a closed property
set, and explicit requiredness. Provider adapters may perform a documented
lossless dialect transform but may not change semantic types, enum values,
requiredness, bounds, or descriptions used for tool choice. Argument order is
preserved because it is model-significant.

Validation occurs before hooks and execution. Invalid arguments produce one
terminal error result and never reach a backend. Tool results use the common
`tool_call`/`tool_result` ABI in doc 06; tool-specific output is JSON-compatible
content and may spill to an asset. An implementation must expose a stable
per-tool schema revision or canonical schema digest in the catalog descriptor.

### Model-facing result presentation

The durable `tool_result` body is the backend's canonical JSON, and every
Kernel reader (validation evidence, edit recording, compaction summaries,
replay) consumes that record. The provider request does not: the context
renderer projects a fixed tool's single-text-block JSON result into plain
text before the dialect serializes it, and a text-carrying dialect sends a
single text block as that string rather than as a JSON-encoded block array.
The projection is a pure function of the result body and is versioned by the
epoch `renderer`, so changing it opens a `renderer_change` epoch.

| Tool | Model-facing text |
|---|---|
| `read` | `N: text` per line (`[line truncated]` suffix when clipped), then one trailer: `[lines a-b of total; artifact_version v]`, or `[lines a-b of total; continue with offset b+1; artifact_version v]` when the window ends early. |
| `shell` | Each step's stdout, then `[stdout truncated]`, `[stderr]` + stderr, `[stderr truncated]`, and `[exit code: N]` only when non-zero; multi-step results prefix `[step i program]`, skipped steps say `[not executed]`; a step the helper killed at its deadline says `[killed after N ms: ...]`, naming whether the default budget, the requested `max_duration_ms`, or the clamped cap applied and pointing at `max_duration_ms` or `job`; declared artifacts append `[artifact path: bytes, artifact_version v]` or their status; an empty result is `(no output)`. Non-UTF-8 output is named by size, not inlined. |
| `grep` | ripgrep's own output, or `[no matches]`. |
| `glob` | One path per line, or `[no matches]`. |
| `write` / `edit` | `Wrote path (bytes, artifact_version v)` / `Edited path (bytes, artifact_version v)`. |
| `apply_patch` | `Patched path: +a -r lines, artifact_version v`. |
| any error outcome | `error (code): message` for the pipeline's `{code, message, retryable}` body, `denied: reason` for an approval denial. |

Any other tool, any multi-block or spilled body, and any body that is not the
shape above pass through unchanged.

### Model-facing tool guidance

The root system instructions of every non-validator request are runtime
owned and composed per request, in this order, joined by blank lines:

1. `You are an AI agent powered by Tekes Kernel.`
2. The session profile text with `{model}` replaced by the request's wire
   model id, trimmed. `coding` uses `crates/tools/prompts/coding.md`: the line
   `You are a coding agent powered by the {model} model.` followed by four
   coding behavior rules (stop and report when the requested change is done and
   the requested tests pass; no extra fuzz, randomized, or differential test
   scripts unless asked; check expected results by running tests rather than by
   long reasoning; be concise). `general` uses `crates/tools/prompts/general.md`:
   `You are a general-purpose agent powered by the {model} model.` The profile
   is frozen in session genesis.
3. `Your working directory is {cwd}.` with the execution cwd, when one is
   bound.
4. The instruction snapshot's `AGENTS.md` scopes, trimmed, when nonempty.
5. For a delegated subagent, the fixed subagent delivery sentence.

The text does not depend on the tool catalog; what each tool does and returns
lives in its schema description alone. Coding-specific behavior belongs in
`coding.md`; tool usage belongs in tool schemas; project rules belong in
`AGENTS.md`.

`fixtures/tools/builtin-tool-guidance.canonical.json` (format 4) is the byte
authority for the harness identity, both profile files (verbatim, including
the final LF), and the working-directory template; its `digest` is SHA-256
over the RFC-8785 bytes of
`{harness_identity, coding_profile, general_profile, working_directory}`.
Changing a profile file changes the digest and, through the epoch system
profile, opens a `system_change` epoch. The validator's judge prompt is
unchanged.

The identity lines are the measured cause of the Kernel/DSH cost gap on the
weighted-ttl corpus with deepseek-v4-flash
(`docs/benchmarks/deepseek-cost-2026-10.md` §Earlier findings): the
two identity lines alone reproduce the whole effect, and per-tool usage
paragraphs (format 1, 2026-09-26) reproduced none of it and were removed. For
gpt-5.6 and kimi-k3 the lines are neutral. The four coding rules cut
deepseek-flash cost by 55% on the batch-ledger-bugfix corpus without lowering
test quality (`docs/benchmarks/deepseek-cost-2026-10.md` §The coding profile rules).

### Complete schema oracle

`fixtures/tools/builtin-tool-schemas.canonical.json` is the byte authority
for every format-1 fixed-tool description, schema revision, complete parameter
tree, required list, nested property set, enum, bound, default, conditional,
and per-tool schema digest. It is one RFC-8785 canonical JSON object followed by
exactly one LF:

```text
BuiltinSchemaOracle = {
  format: 1,
  revision: "builtin-tools/schema-4",
  schemas: [BuiltinSchema, ...]
}
BuiltinSchema = {
  description: nonempty string,
  digest: "sha256-" + 64 lowercase hex digits,
  name: string,
  parameters: closed JSON Schema object,
  revision: "builtin-tools/schema-4"
}
```

`schemas` has exactly the manifest's 25 names in the same order. For each row,
`parameters.properties` has exactly the manifest `arguments` names; their
semantic order is the manifest order even though RFC-8785 sorts object keys.
Every object-valued argument and every object array item is closed with
`additionalProperties:false`. No implementation may derive this fixture from
its Rust types during a test or rewrite it as a test side effect.
Provider projection serializes the typed ordered property vectors, not a
generic map reconstructed from the canonical oracle; the conformance test
checks those vectors against manifest argument order before any dialect
transformation.

The per-tool digest preimage is exactly the RFC-8785 encoding, with no trailing
LF, of this three-field object:

```json
{"description":DESCRIPTION,"name":NAME,"parameters":PARAMETERS}
```

`revision`, `digest`, provider wrapper fields such as `type`/`strict`, manifest
backend/effect/availability, and provider-specific metadata are not in this
preimage. Changing any model-visible description or parameter byte therefore
changes the digest; changing the schema revision also requires a replacement
oracle even when a migration deliberately preserves the same digest.

The oracle closes these previously implicit requiredness decisions:

- strict nullable/default fields remain present and required for
  `ask_user_questions`, `glob`, `grep`, `read`, `set_goal_state`,
  and `web_search`; JSON null selects the documented default;
- `context` always requires `operation` and `reason`; `search` also requires a
  nonempty `query`, thread-scoped search requires `thread_id`, `records`
  requires `turn_id`, `read`/`resume`/`archive`/`interrupt` require `thread_id`,
  and `send` requires nonempty `thread_id` and `message`;
- `context_get` requires `record_ids` and `reason`; cursor, target thread, and
  page bound remain optional;
- `job` requires `action`; `start` requires `program`, and `status`/`stop`
  require `job_id`;
- `shell` requires exactly one non-null command form: `command` or `program`.
  The other top-level fields are optional. `args` is the direct-argv vector,
  `steps` contains closed `{program,args}` objects, and `artifact_outputs`
  contains closed `{path}` objects with project-relative paths. Null optional
  lists select their defaults (schema-3);
- `set_goal_state(state=blocked)` requires non-null, nonempty `reason` and
  `user_action`;
- `task.output` is exactly `{mode,format,name,path,contract}`. File mode
  requires one nonempty project-relative `path`; inline mode requires `path` to
  be the empty string;
- `verify.verdict` is exactly `pass | fail | inconclusive`; `covered_set` is
  nonempty, and `failures` is nonempty iff the verdict is `fail`;
- memory candidates, summary evidence, validation coverage/failures, task
  input sources, and shell steps/artifacts use the closed nested shapes in
  the oracle. Workspace/ledger-dependent predicates are backend validation
  after exact schema validation.

The schema shapes began from the two migration fixture's pinned predecessor
declarations, but this oracle owns the Kernel result. Removed aliases and
replaced fields do not re-enter the fixed namespace merely because they existed
in a predecessor schema.

## Fixed tool catalog

The following two tables are grouped only for readability by execution
semantics. They are not ownership, package, process, or catalog partitions.
Every row belongs to the same fixed namespace and resolves through the same
Kernel catalog.

### Orchestration and workflow tools

The argument names in this table are exhaustive; the stated nested constraints
and semantics are normative and typed implementations may only change them
through a versioned contract change.

| name | availability | arguments | effect and execution |
|---|---|---|---|
| `ask_user_questions` | selection-controlled | `question`, `options` | User-interaction hold. `question` is nonempty; `options` is null or distinct reply strings. The answer resumes and pairs the same call; it is not approval for another action. |
| `new_goal` | selection-controlled | `goal`, `completion_criteria`, `reason` | Creates or replaces a host-bound durable goal. Goal identity is host supplied. |
| `report` | subagent role | `result` | Terminal child-to-parent delivery. It is invalid outside a delegated subagent role. |
| `set_goal_state` | selection-controlled | `state`, `progress`, `reason`, `user_action` | Updates the host-bound goal phase to `active`, `blocked`, `paused`, or `complete`; `blocked` requires nonempty `reason` and `user_action`. `complete`, `blocked` and `paused` stop automatic goal continuation (session-endpoint §Goals). It never edits the objective, and no independent verifier is required. |
| `skill` | selection-controlled | `skill` | Loads exactly one skill offered by `skill_explorer` in the current projection iteration. |
| `subagent` | selection-controlled | `brief`, `report_back_session_id` | Spawns an independent child line from a self-contained brief; host overwrites the report-back identity. |
| `summary_artifact` | compactor role | `continuation`, `evidence_refs` | The compaction summary artifact over the frozen source bundle (event §compact `summary_request`): an admitted continuation is the compact summary, the deterministic quoted history the fallback. `evidence_refs` are exact record addresses: the ledger `seq` numbers of bundle records (integers ≥ 1, unique, at least one). The host admits the artifact against the bundle; it is never a model-visible turn tool. |
| `task` | selection-controlled | `task_name`, `goal`, `instructions`, `input_sources`, `output` | Registers an executable child work unit. `output.mode` is `inline` or `file`; file mode requires one project-relative output path. |
| `think` | selection-controlled | `thought` | Pure in-process reasoning note with no external or durable domain effect beyond its result record. |
| `verify` | validator role | `covered_set`, `verdict`, `failures` | Validator terminal result. `verdict` is the closed validator verdict; failures are empty iff verdict is not fail. |

Migration evidence note: the separate audit fixture maps legacy `compact` to
host-owned compaction and `goal_completed` to `set_goal_state`. `write` is now
a direct builtin rather than a compatibility alias. Missing coverage is a
fatal packaging error.

`note` and `recall` are retired. Existing policy selectors are accepted for
upgrade compatibility but never become executable tools. Historical memory
files remain untouched; see [retirement](../docs/history/retired-kernel-memory.md).

### Capability and integration tools

These are the fixed filesystem, process, network, context, and
discovery capabilities in the same Kernel catalog. Workspace capability policy
can disable one for a later launch, but cannot replace its contract.

| name | availability | arguments | effect and execution |
|---|---|---|---|
| `apply_patch` | default | `operation`, `path`, `diff`, `expected_artifact_version`, `summary` | Contextual `apply_patch.v2` writer. Only `path` and `diff` are required; the host infers create/update from file existence and the current artifact version when omitted. Explicit operation/version fields retain their existing meaning. `diff` is V4A body-only: creation takes `+`-prefixed content lines; updates take context/removal/addition lines without file headers. The helper performs descriptor-beneath SHA compare-and-replace and temp+sync+rename atomically. Outside-workspace paths require approval and an explicit writable root. |
| `context` | default | `operation`, `thread_id`, `message`, `goal`, `reason`, `turn_id`, `cursor`, `limit`, `query`, `scope` | Supervisor-backed historical discovery and thread control. Read operations are `search`, `records`, `threads`, `read`; mutating operations are `start`, `fork`, `send`, `resume`, `archive`, `interrupt`. Every call requires a nonempty audit reason; every mutation requires operation-specific approval. Workspace wall is mandatory. |
| `context_get` | default | `record_ids`, `thread_id`, `reason`, `cursor`, `page_bytes` | Approval-free exact record read through the supervisor. One to sixteen positive record ids; `page_bytes` is at most 16384; returned historical content is untrusted data. |
| `edit` | default | `path`, `old_text`, `new_text` | Replace one exact UTF-8 text span. An absent or repeated `old_text` fails without a write; a concurrent file change is detected before commit. Uses the same workspace edit Approval, sandbox, artifact version, and edit recording path as `write`. |
| `glob` | default | `pattern`, `path` | Read-only helper filesystem walk, newest first; supports `*`, `?`, `**`; maximum 1000 results and 100000 scanned entries. |
| `grep` | default | `pattern`, `path`, `glob`, `output_mode`, `case_insensitive`, `context_lines` | Read-only helper exec/native fallback. Ripgrep regex semantics; output mode is `content`, `files_with_matches`, or `count`; bounded entries, per-file bytes, total bytes, matches, output, and time. |
| `job` | default | `action`, `program`, `args`, `working_directory`, `writable_paths`, `job_id` | Detached process manager. `start` uses argv-direct sandboxed execution and may require approval; absent `working_directory` resolves to the caller-injected immutable primary workspace cwd. Requested writable paths remain in the durable job spec, are revalidated as a subset of the injected policy ceiling for every call, and are passed in the exact effective sandbox policy. `list`/`status` are read-only; `stop` is a process-control side effect. State is durable disk metadata and jobs outlive workers. |
| `plan` | plan-mode | `plan` | Nonempty Markdown plan followed by the plan approval hold. Approval, revision request, or rejection is durably correlated to the same workflow. |
| `read` | default | `path`, `offset`, `limit` | Read-only UTF-8 numbered-line window. Offset is one-based; default limit 2000 lines; max line length 2000; scan cap 32 MiB; nonregular and escaping paths reject. |
| `shell` | default | `working_directory`, `max_duration_ms`, `max_output_tokens`, `command`, `program`, `args`, `steps`, `failure_policy`, `writable_paths`, `artifact_outputs` | Sandboxed external execution. Supply either `command` or `program`; all auxiliary fields are optional. An omitted or null `max_duration_ms` uses the runtime default (300 s); a larger request is clamped to the runtime cap (600 s); both are `SystemToolConfig` fields a profile may set. At the deadline the helper kills the command's process group and the result is a completed, `is_error` shell result whose step is `timed_out`, not a retryable backend error. `max_output_tokens` is an advisory output cap estimated at four bytes per token, bounded by the helper's normal limits. Omitted, null, or empty `writable_paths` uses the effective user-authorized workspace roots; a nonempty list may narrow those roots but cannot expand them. Declared artifacts are verified and hashed. |
| `skill_explorer` | default | `query`, `limit` | Read-only SearchFrame projection over the effective skill catalog. `limit` is 1...10, default 5, and the durable offer controls later `skill` visibility. |
| `tool_search` | deferred-catalog conditional | `query`, `limit` | Read-only SearchFrame projection over deferred dynamic tools. `limit` is 1...10, default 5. Search returns bounded descriptors and creates a causal offer that stays valid while its result is model-visible (until a `compact` covers it); it never executes or permanently enables a tool. |
| `web_fetch` | default | `url` | Public HTTP(S) read with per-hop SSRF validation, timeout, size cap, readable-text extraction, and egress approval policy. Loopback, link-local, LAN/internal destinations, non-HTTP schemes, and unsafe redirects reject. |
| `web_search` | credential-conditional | `query`, `max_results`, `topic` | Public web search through the independent [web-search-provider](web-search-provider.md) authority. It is absent without a supported configuration, active usable credential, broker, and backend. `max_results` is 1...10, default 5; topic is `general` or `news`. |
| `write` | default | `path`, `content` | Write UTF-8 text to one file, creating or replacing it under the same workspace edit Approval and sandbox policy as `apply_patch`. The model does not supply a patch grammar, summary, or artifact version; the runtime records the preimage, guards concurrent replacement, and returns the resulting SHA and durable artifact version. Empty content is valid. |

For `create_file`, missing parent directories are created beneath the authorized
root with descriptor-relative `mkdirat`; each component is reopened without
following symlinks. The complete relative path is validated before any directory
creation. Existing destination files are never replaced by this operation.
`update_file` continues to require an existing parent and target. A reserved
artifact version is not proof that the file effect committed; failed writes may
leave a reserved generation and created parent directories.

`apply_patch` remains available for contextual edits; `edit` handles literal
replacement and `write` handles direct whole-file creation or replacement. A benchmark terminal bridge
removes host filesystem/process/Web tools and supplies its separately
configured MCP terminal mandate; this is a launch profile, not a different
fixed manifest.

Git is not a fixed tool family. The model invokes the system `git` or `gh`
binary through `shell`; there are no `git_*` or `github_pr_create` schemas.
Repository discovery, selected paths, remote choice, approvals, network policy,
and sandboxing are evaluated as part of the concrete `shell` invocation. The
Kernel does not embed a Git library.

Tool names split only across a real model contract, lifecycle, or security
boundary; implementation repository/package boundaries never justify another
tool. The remaining close pairs are intentional: `context` performs broad
discovery/thread operations while `context_get` performs bounded exact reads;
`shell` is a foreground invocation while `job` is durable and detached;
explorer/search
tools create causal offers while `skill` or a dynamic tool consumes one; and
`task`, `subagent`, and `report` represent registration, child creation, and
terminal child return. Combining any of those would produce an operation union
with different authorization or durability rules, not a simpler contract.

### Approval mapping

Approval classification is part of the fixed contract, not inferred from the
English description:

- approval-free/read-only: `read`, `glob`, `grep`, `context_get`, the read arms
  of `context`, `skill_explorer`, `tool_search`, and pure workflow
  tools;
- edit policy: every in-workspace `apply_patch`, `edit`, or `write`; a target under
  `.agent`, `.tekes`, Git hooks, or another privilege-defining path escalates
  to destructive;
- execute policy: `shell`, `job.start`, and `web_fetch`;
- destructive policy: out-of-workspace file writes, additional external
  writable roots, `job.stop`, and every mutating `context` arm;
- workflow holds: `ask_user_questions` and `plan` use their dedicated durable
  answer protocols. An answer to either is not a standing execution grant;
- `web_search` is approval-free at the tool layer but still requires the
  immutable network policy and configured credential. Dynamic tools map their
  declared operation annotations; missing/unknown classification fails safe.

Permission mode maps `read-only/edit/execute/destructive` to allow, deny, or
durable hold. The mode is per session and durable
(`threads/<sessionId>/permission-mode.json`, absent = `workspace-write`; see
`session-endpoint.md` "Approval policy"), re-read by the worker at the start
of every tool batch:

- `read-only`: read-only tools and workflow holds run; edit, execute, and
  destructive calls are denied with `read-only permission mode`;
- `workspace-write`: read-only, workflow, edit, and execute calls run
  (execution is already confined by the immutable sandbox policy);
  destructive calls park behind a durable per-call approval;
- `danger-full-access`: every class runs.

Approval happens before the effect and is bound to the effective invocation.
No classification may be downgraded by configuration, a hook, an MCP
annotation, or the permission mode: the mode changes only the decision per
class, never the class.

## Dynamic external, plugin, and MCP tools

Dynamic tools are not enumerable in this fixed manifest. They obey all of the
following:

1. Local manifest and plugin exec tools use their dynamic exec backend, the same sandbox
   source, clean environment, approval taxonomy, byte/time caps, cancellation,
   secret scan, and write-ahead rules as fixed exec tools.
2. Project-declared executables and MCP servers are inert until trusted by
   user-level configuration. Opening a workspace never executes project
   configuration by itself.
3. MCP `tools/list` supplies the schema. Read-only/destructive annotations map
   to the host operation class; missing or unknown annotations fail safe to
   execute/destructive policy, never read-only.
4. Stdio MCP runs as a worker child when multiply openable. Interactive OAuth
   HTTP MCP and non-multiply-openable native helpers use the supervisor daemon
   pool. Secrets remain in the supervisor/keychain and enter only the named
   integration process; they never enter events, catalog caches, or ambient
   worker environment.
5. `always_on` dynamic tools join the resident projection. All others are
   registered as discoverable and become visible only through `tool_search`.
   `tools/list` may list the complete catalog for clients without making every
   schema resident in model context.
6. Dynamic registration cannot weaken fixed approval, sandbox, schema, or
   collision rules. Live protocol registration state is rebuildable from
   configuration plus live MCP catalogs.

### Historical first-party extension inventory

The predecessor TekesAppServer shipped one tool-bearing first-party plugin,
`com.tekes.computer-use` version `0.1.5`. That identity is migration evidence,
not the current deployment shape and does not make its tools fixed. The current
Computer Use executable is configured as an ordinary local stdio MCP server;
its live `tools/list` is the schema authority. The migration inventory is
preserved separately in
`fixtures/tools/first-party-tools.canonical.json` so "all current tools"
does not silently omit the bundled extension surface.

This is a **reuse, not reimplement** boundary. Kernel launches the unchanged
executable from an ordinary MCP configuration through the generic MCP carrier.
It must not require or synthesize a plugin binding and must not add a TCU-specific
protocol, DTO, dispatcher, execution path, conditional branch, or Rust
implementation of Computer Use operations.

| plugin tool | arguments | operation / effect |
|---|---|---|
| `list_apps` | none | read-only process inventory |
| `permission_status` | none | read-only Accessibility/Screen Recording state |
| `request_permissions` | `accessibility`, `screen_capture` | destructive, explicit user gesture; macOS owns the final consent UI |
| `get_app_state` | `app`, `disableDiff` | read-only Accessibility tree plus front-window capture |
| `click` | `app`, `element_index`, `x`, `y`, `mouse_button`, `click_count` | execute/computer control |
| `drag` | `app`, `from_x`, `from_y`, `to_x`, `to_y` | execute/computer control |
| `press_key` | `app`, `key` | execute/computer control |
| `scroll` | `app`, `element_index`, `direction`, `pages` | execute/computer control |
| `set_value` | `app`, `element_index`, `value` | execute/computer control |
| `select_text` | `app`, `element_index`, `text`, `selection_type`, `prefix`, `suffix` | execute/computer control |
| `type_text` | `app`, `text` | execute/computer control |
| `perform_secondary_action` | `app`, `element_index`, `action` | execute/computer control |

The helper is a macOS 15+ arm64 native-helper daemon because Accessibility and
ScreenCaptureKit/TCC identity cannot be multiply opened as ordinary worker
execs. Plugin installation and workspace capability grants authorize launching
the helper; individual tool annotations still drive read-only/execute/
destructive approval, and `request_permissions` additionally requires an
unreusable user gesture. Removing or upgrading this plugin changes the shipped
extension fixture, not builtin-tools format 1, unless a tool moves into or
collides with the fixed namespace.

## Backend and safety obligations

`helper_fs` uses the exec-helper v1 process boundary. `read`, `glob`, and
`apply_patch` are descriptor-first beneath pre-opened roots. A path string
validated before open is never sufficient. Writes and patches publish through
temp file, full sync, atomic rename, and parent sync.

`helper_exec` is argv-direct. Only `shell.command` deliberately
invokes the platform shell; no other tool interprets shell syntax. Every child
receives a clean allowlisted environment, close-on-exec descriptors, byte and
wall caps, TERM→grace→KILL escalation, full process-group reap, and the sandbox
profile derived from the same effective policy used for authorization.

`background_exec` persists command, pid/start identity, state, and bounded
stdout/stderr tails before returning a job id. Polling never blocks a worker.
PID reuse is detected; stop is idempotent; worker/thread stop does not
implicitly kill detached jobs.

`worker_http` applies the network class from the immutable launch policy and a
tool-specific URL guard. Redirects are revalidated. Response bodies are byte
bounded and secret-scanned before event/asset publication.

`supervisor_control` is the only fixed backend allowed to read or mutate a
different thread folder. Worker calls carry a correlation/origin key; success
is returned only after the target append/control receipt required by the
operation. The worker never opens a sibling thread folder for write.

Effects other than `none` or `read_only` require a durable write-ahead
`tool_call` before execution. A hook mutation of such a call requires durable
`effective_execution` first. Approval is evaluated after validation and before
the effect. Crash recovery never blindly repeats an unpaired side effect.

## Conformance and evolution

Conformance gate `BuiltinToolManifestParity` must prove all of the following:

1. the fixture is canonical, closed, sorted, unique, and uses only the enums
   in this contract;
2. the manifest name set equals the deduplicated legacy union after applying
   every explicit disposition in the separate migration audit fixture, while
   no tool retains a repository owner classification;
3. every entry resolves to exactly one closed schema with the same name and
   top-level property order; every top-level property is represented by
   `arguments`, requiredness/types/enums/bounds satisfy this contract, and its
   canonical digest is stable;
4. every implementation declares the manifest backend/effect class and all
   side-effectful classes traverse write-ahead, policy, sandbox, approval, and
   result-pairing gates;
5. default, missing-credential, benchmark, plan, validator, subagent,
   compactor, and deferred-catalog
   profiles expose exactly the expected subset;
6. fixed/dynamic and dynamic/dynamic collisions fail launch;
7. `tools_digest` equals the locked digest of the canonical five-field `tools`
   array and changes whenever any of those fields changes; a per-tool schema
   digest or effective-projection revision separately changes whenever nested
   model-visible schema or descriptor policy changes;
8. the migration parity check against both named source-repository baselines
   reports no omitted, extra, or undisposed legacy tool without creating two
   catalogs;
9. the separately versioned first-party extension fixture matches every tool,
   top-level argument set, and operation annotation shipped by the current
   Computer Use plugin without admitting those names to the fixed namespace.

The repository migration check (a private tool that needs checkouts of the
legacy repositories, so it is not part of the public CI) independently extracts both migration inventories and compares
their deduplicated name union and every top-level argument order with the one
canonical fixture. It also extracts all shipped Computer Use MCP declarations
and compares their names, argument sets, and read-only/execute/destructive
annotations with the first-party extension fixture.

Adding, removing, renaming, changing argument order/shape, changing backend or
effect classification, or changing availability is a builtin-tools format
change. Update the contract and hand-authored fixture before implementation;
update migration evidence and conformance in the same semantic change. A new
dynamic plugin or MCP instance is not a format change unless it attempts to
occupy the fixed namespace.
