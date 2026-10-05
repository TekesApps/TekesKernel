# spec: Skill package v1

Status: **normative executable contract**. This contract owns user/project
skill package identity, precedence, immutable companion resources, model load,
and the versioned Client `skills/list` projection. Plugin and MCP sources are
not part of v1.

## Package roots and frozen bytes

The input is one validated `InstructionSnapshot` from
[instruction-snapshot](instruction-snapshot.md). A package is exactly:

```text
skills/<name>/SKILL.md
skills/<name>/<companion-relative-path>...
```

`<name>` is 1–64 ASCII bytes, begins with lowercase ASCII or a digit, and the
remaining bytes are lowercase ASCII, digits, `-`, or `_`. A companion path is
nonempty, relative, `/`-separated, contains no NUL, empty, `.` or `..`
component, and is already NFC. The instruction resolver's no-follow,
regular-file, UTF-8, per-file, total-size and double-scan rules apply to every
package file. Runtime code reads only frozen Source content; it never reopens a
package directory.

Flat legacy sources such as `skills/name.md` remain readable by the pre-Slice-11
model-tool compatibility path, but are not package-compatible and are omitted
from `skills/list`.

## Manifest and identity

`SKILL.md` begins with a `---` fenced header of unique `key: value` lines.
Blank/comment lines are ignored; one balanced matching quote pair is removed.
`name` and nonempty `description` are required. Optional `schema_version`, when
present, is `1`. Unknown keys are retained as forward-compatible source bytes
but have no v1 behavior. The header `name` must equal the package directory.

The effective package is selected as a whole directory in ordinary instruction
source order: user first, then projects by workspace index; a later directory
replaces every lower-precedence file. Files are never merged across origins.
An invalid winning package fails catalog construction rather than exposing a
lower-precedence package under the same identity.

Resources sort by relative-path UTF-8 bytes. `content_digest` is lowercase
SHA-256 of RFC-8785 canonical bytes (without JSONL LF) of:

```text
{format:1,name:<name>,resources:[
  {path:<relative>,content_sha256:<source digest>}, ...
]}
```

Changing any companion file changes package identity. `SKILL.md` is included
as resource `SKILL.md`. A resource read is package-name plus exact frozen
relative path; missing and traversal paths fail closed.

## Model and Client projections

The existing fixed `skill_explorer`/`skill` tools use the same catalog. A
package load returns `{format:1,name,description,body,resources}` from the
snapshot. Companion contents remain data owned by the loaded package; they are
not independent skills, commands, tools, or executable grants.

The separate Client resource capability `skills/list` returns:

```text
{format:1,skills:[{
  id:str,name:str,description:str,source:"user"|"project:<index>",
  content_digest:lowercase-64-hex
}, ...]}
```

Rows sort by name UTF-8 bytes. This capability is not added to the
Session Endpoint base registry. The Client negotiates the
versioned resource capability independently; an older driver sees no method.

## Errors and migration

Closed semantic classes are `invalid-path`, `invalid-name`, `invalid-skill`,
`skill-not-found`, and `skill-resource-not-found`. Diagnostics may include
package/path identity but never resource content.

`fixtures/resources/migration/` freezes the AppServer/Runtime revisions and
source files used for migration. Gates prove whole-directory user/project
precedence, identity, companion access and model/Client catalog agreement.
