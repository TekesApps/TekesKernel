# spec: Command catalog v1

Status: **normative executable contract**. This contract owns the immutable
user/project slash-command catalog, deterministic argument expansion, and the
versioned Client `commands/list`/`commands/run` capability. Plugin commands and
MCP prompts are absent until their own contracts land.

## Discovery, precedence, and parsing

The input is one validated `InstructionSnapshot`. A v1 command is exactly a
top-level `commands/<name>.md` source; nested paths and non-`.md` sources are
omitted. `<name>` obeys skill-package's 1–64-byte name grammar. The winner is
`InstructionSnapshot.effective.commands`, so later projects replace user
commands by exact filename. Catalog/list/run use frozen Source bytes and never
rescan mutable directories.

A command may begin with the same minimal `---` fenced front matter used by the
predecessor. The body is the trimmed text after the closing fence. Without an
opening fence the complete trimmed file is the body. An opening fence without
a closing fence, a malformed/duplicate field, unbalanced quotes, or an empty
body is `invalid-command`.

Recognized metadata is `description`, `argument-hint` (with
`argument_hint` compatibility), and `model`. Unknown keys remain source bytes
without v1 behavior. `model` is listing metadata only: it never bypasses the
session's selected-model authority. Command `content_digest` is the
Instruction Source SHA-256 of exact file bytes.
If both `argument-hint` and `argument_hint` occur, the command is
`invalid-command` even when their values are identical; aliases never create
an ordering or last-writer-wins rule.

## Argument grammar and expansion

`arguments` is at most 64 KiB UTF-8. Positionals are parsed left-to-right:
unquoted whitespace separates tokens; matching single/double quotes group a
token and are removed; backslash escapes the next scalar outside quotes and in
double quotes; backslash is literal in single quotes. Empty quoted tokens are
retained. Unbalanced quote or final escape is `invalid-arguments`.

The body is scanned once and supports:

- `$ARGUMENTS`: exact raw argument string;
- `$1`…`$99`: parsed positional or empty when absent; `$0` rejects;
- `$$`: one literal `$`;
- `\$`: one literal `$` and no substitution at that position.

Inserted text is never rescanned. `@file` and ``!`shell` `` have no v1 special
meaning and remain literal: command expansion grants neither filesystem reads
nor process execution. `$100` and longer digit runs reject rather than being
partially interpreted. The nonempty expanded text is at most 1 MiB and is the
only text submitted.

## Versioned Client capability

The independent resource capability set is exactly:

```text
skills/list
commands/list
commands/run
```

It does not modify Session Endpoint v3's 16 base unary operations or its
`remote.mux` stream registration. `commands/list`
returns `{format:1,commands:[{name,description,argument_hint?,model?,source,
content_digest}]}` sorted by name. `commands/run` accepts the closed DTO:

```text
{session_id:str,name:str,arguments:str,key:str}
```

All four fields are required. In particular, omitted `arguments` is
`bad-request`; an explicitly supplied empty string remains a valid value.

On the native carrier these methods are additive registrations owned by a
`ProductionEndpointRoutes` extension handler. Each handler must declare the
method class, validate its own closed payload before execution, and validate
its exact typed failures. `CompositeProductionEndpointRoutes` assigns every
method to exactly one handler and rejects collisions, allowing later plugin,
MCP, and Client-extension services to coexist without last-writer-wins
replacement. This registry is compared separately from `TEKES_UNARY_ROUTES` and the
transport base registry; the frozen base remains unchanged. Assembly
rejects any extension that advertises a frozen base name, including a handler
that otherwise supplies a valid class or implementation.

The production daemon captures one catalog at boot from the user agent root
and every configured canonical workspace cwd, ordered by workspace authority
id and then cwd order. It mounts that immutable catalog through
`ClientResourceService`; malformed workspace authority or resource input
fails daemon assembly. Catalog changes become visible after daemon restart and
never trigger an endpoint discovery or a second runtime.

`key` is 1–128 UTF-8 bytes, contains no NUL and does not start `request-`.
After deterministic expansion the supervisor calls the same
`SessionDeliveryAuthority` used by `session.prompt`, with `steer:false` and
origin `{client:"tekes-client-resource",op:"commands/run",target:session_id,
key,principal}`. It authors no separate command/transcript event and never
creates another runtime. Success is:

```text
{format:1,accepted:true,command,content_digest,seq,deduplicated}
```

Retrying the same session/key/text returns the original durable seq with
`deduplicated:true`; changing expanded text under the same key is the ordinary
input authority's idempotency conflict. No success is returned before the
existing input barrier/receipt.

### Reserved verb: `compact`

An expanded text that is exactly `compact` (after trimming) is the manual
compaction request (the predecessor's `/compact` command), never model input.
With the same origin as above the supervisor delivers worker-control `compact`
to the live worker, which applies it at its next yield and receipts it
(worker-control §compact, event R13-4); with no live worker the
supervisor authors it under the line lock — checkpoint at the current key
floor, the pure `engine::plan_context_compaction` plan for the turn after the
settled tail, summary inline or spilled — exactly the worker's manual shape
(tail-lifecycle D-61 whitelist: manual `compact`). Success is the same DTO with
`seq` naming the durable `compact` event; a retry under the same key is
`deduplicated:true`. The next attempt on the line binds a new epoch with
`reason: "compaction"`, and `engine::first_post_compact_attempt` is the gate
that names the first attempt of that generation admitting the inputs queued
behind the request. Any other body that merely contains the word is ordinary
command text.

## Errors and migration

Closed classes are `invalid-path`, `invalid-name`, `invalid-command`,
`invalid-arguments`, `command-not-found`, `invalid-request`, and the existing
typed delivery failure. Catalog construction retains the complete internal
classes; the `commands/run` carrier exposes the reachable subset exactly:

| code | message | details |
|---|---|---|
| `bad-request` | `Request payload is invalid` | `{}` |
| `invalid-command` | `Command is invalid` | `{}` |
| `invalid-arguments` | `Command arguments are invalid` | `{}` |
| `command-not-found` | `Command was not found` | `{name}` |

Existing `session.prompt` delivery failures retain their already frozen exact
message/details. Any out-of-vocabulary or malformed extension failure becomes
the host's closed `internal`; failure authors zero command-side durable state.

`fixtures/resources/migration/` records the pinned predecessor evidence plus
canonical catalog, expansion, negative and keyed-run cases. MCP prompt naming,
context attachment, and shell execution are not silently inferred from the
predecessor; they remain absent until their owning Slice-13/14F contracts.
