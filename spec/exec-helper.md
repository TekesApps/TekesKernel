# Exec helper protocol v1

Status: **normative executable contract**. This file owns the process boundary
used by filesystem and argv-exec tools.

## Process and negotiation

The helper is a one-request process. Roots are bound before negotiation by
repeated `--root <name>=<absolute-canonical-directory>` arguments; names match
`[A-Za-z][A-Za-z0-9_-]{0,31}` and are unique. The helper opens every root as a
directory descriptor before reading stdin. Requests contain only a root name
and a normalized relative path, never an absolute path. The sandbox profile is
installed by the parent before helper code runs.

All frames are one RFC-8785 canonical JSON object plus LF and exactly one
top-level key. The client sends:

```json
{"hello":{"max":1,"min":1,"proto":"tekes-exec-helper"}}
```

The helper replies `{"selected":{"version":1}}` or
`{"reject":{"reason":"..."}}`; no mutual version exits 76. After selection
the client sends one `request` and the helper sends one `result` or `error`,
then exits 0. Malformed input, unknown messages, duplicate ids, or a second
request are protocol errors and exit 76.

## Bytes and request envelope

Byte strings are closed objects `{"encoding":"utf8"|"base64","data":string}`.
Invalid UTF-8/base64 is rejected. A request is:

```text
{"request":{"id":string,"operation": Operation}}
```

`id` is non-empty and is echoed by the terminal response. `Operation` is the
closed union:

```text
{"read":{"root":string,"path":string,"max_bytes":uint}}
{"write":{"root":string,"path":string,"content":Bytes,"create":"new"|"replace"|"upsert"}}
{"patch":{"root":string,"path":string,"expected_sha256":string,"replacement":Bytes}}
{"glob":{"root":string,"pattern":string,"max_entries":uint}}
{"grep":{"root":string,"path":string,"pattern":string,"glob":string|null,"output_mode":string,"case_insensitive":bool,"context_lines":uint,"stdout_bytes":uint,"timeout_ms":uint}}
{"exec":{"argv":[string,...],"stdin"?:Bytes,"cwd"?:{"root":string,"path":string},
         "env":{string:string},"stdout_bytes":uint,"stderr_bytes":uint,
         "timeout_ms":uint|null}}
```

All integer limits are positive I-JSON safe integers and are bounded by the
implementation hard caps. `argv` is non-empty and executes directly; shell
syntax has no meaning unless argv itself names the shell tool. `env` is the
entire child environment and is limited to the allowlisted keys selected by
policy.

## Results and errors

Success is `{"result":{"id":string,"value":...}}`, where value is exactly
one of:

```text
{"read":{"content":Bytes,"bytes":uint,"sha256":string}}
{"write":{"bytes":uint,"sha256":string}}
{"patch":{"bytes":uint,"sha256":string}}
{"glob":{"paths":[string,...]}}
{"exec":{"status":int,"stdout":Bytes,"stderr":Bytes,
         "stdout_truncated":bool,"stderr_truncated":bool}}
```

Failure is
`{"error":{"id":string,"class":Class,"message":string,"retryable":bool}}`.
`Class` is closed: `invalid_request | unknown_root | path_escape | symlink |
not_regular | exists | missing | digest_mismatch | limit | timeout | denied |
io | protocol | sandbox_unavailable`. Messages are diagnostic and never
parsed for behavior.

## Filesystem and cancellation rules

Relative paths contain only non-empty normal components; `.`, `..`, NUL,
absolute paths, and platform prefixes are rejected. Every component is opened
descriptor-first beneath the bound root with no-follow and nonblocking flags;
the opened leaf is `fstat`-checked. Reads refuse symlinks, FIFOs, sockets, and
devices and stop at the byte cap.

Write and patch create a unique temporary regular file in the already-opened
parent directory, write the complete replacement, full-sync it, rename it over
the destination subject to `create`, then sync the parent directory. A killed
helper can therefore leave only an ignorable temporary file or the complete
old/new destination. Patch verifies `expected_sha256` from the opened old
descriptor before publication.

Exec closes stdin, monitors the deadline, sends TERM at timeout, waits a
bounded grace period, then sends KILL and reaps the entire helper-owned process
group. stdout/stderr are drained concurrently and capped without allowing a
pipe deadlock. Cancellation is killing the helper process group; no success
response may follow cancellation.

`fixtures/helper/` is the canonical wire and filesystem-result oracle.


### Native grep fallback

`grep` returns the existing `exec` value shape, with status 0 for matches and
1 for no matches. The engine invokes it when external ripgrep cannot be found.
It uses Rust regex syntax, workspace-local ignore files, optional glob overrides,
and the `content`, `files_with_matches`, and `count` modes. Ambient/global Git
excludes are not loaded. File content is reopened beneath the bound root using
no-follow directory descriptors; traversal results do not authorize reads.

Limits are 100,000 scanned entries and matches, 16 MiB per file, 32 MiB total
read bytes, caller-bounded output and wall time, and a 2,000-byte displayed line.
Output truncation is explicit. Invalid expressions, escaped/nonregular paths,
and exhausted scan budgets are errors, never successful empty searches.

The exec request accepts an optional `timeout_ms` of at least 1, matching the
public shell tool's optional `max_duration_ms`. Absent means the command runs
to completion: a long build or test run is not a hung one, and only the caller
knows which it is, so the turn's stop path ends it rather than a clock. The
client imposes no process deadline of its own when the request carries none.
A timeout that is present is a deadline, not a mandatory wait after a process
exits.

Darwin profiles permit metadata reads on ancestors of explicitly declared
read/write roots to support canonical executable resolution. This grants no
additional file-content reads or directory listing rights on those ancestors.
