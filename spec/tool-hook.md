# Tool hook protocol v1

Status: **normative executable contract**. This file owns hook process bytes
and decisions. `docs/runtime/tools.md` explains intent but cannot change this
protocol.

Host lifecycle events use the separate [format-2 lifecycle hook contract](lifecycle-hook.md).
Format-1 pre/post behavior is unchanged.

## Framing and invocation

A hook binding is itself one RFC-8785 canonical JSON object plus LF in an
effective `hooks/**` instruction source:

```text
{"format":1,"id":string,"phase":"pre"|"post","argv":[string,...],
 "failure":"open"|"closed","timeout_ms":uint,
 "stdout_bytes":uint,"stderr_bytes":uint,"env":{string:string}}
```

`id` equals the source's logical path below `hooks/`; argv is non-empty. The
three limits are positive safe integers within the product hard ceilings.
`env` is a complete allowlisted environment, not additions to ambient state.
Instruction snapshot validation parses every effective and shadowed hook
source, so invalid dormant content cannot become executable after a rewrite.

A hook is spawned argv-direct with a clean, allowlisted environment. The
worker writes exactly one RFC-8785 canonical JSON object followed by one LF to
stdin, closes stdin, and reads exactly one such line from stdout. Any extra
non-whitespace stdout, malformed JSON, duplicate key, unsupported format, or
output beyond the configured cap is a protocol failure. Stderr is diagnostic
only and is byte-capped.

The request is the closed object:

```text
{
  "format": 1,
  "hook_id": string,
  "phase": "pre" | "post",
  "call": string,
  "name": string,
  "invocation": JsonValue,
  "result"?: {"outcome": JsonValue, "content"?: JsonValue, "meta"?: JsonValue}
}
```

`result` is forbidden for `pre` and required for `post`. `invocation` is the
provider-born invocation for the first pre hook and the previous hook's
effective invocation thereafter. Hooks never receive secrets or the worker's
ambient environment.

## Results

The response is the closed object `{"format":1,"hook_id":string,"call":
string,"verdict":...}`. `hook_id` and `call` must equal the request.

Pre-hook verdict is exactly one of:

```text
{"allow": {}}
{"deny": {"reason": string}}
{"mutate": {"invocation": JsonValue, "reason"?: string}}
{"ask": {"scope": string, "question": JsonValue}}
```

Post-hook verdict is exactly one of:

```text
{"allow": {"annotations"?: JsonValue}}
{"deny": {"reason": string, "annotations"?: JsonValue}}
```

Post `deny` withholds the already-produced output as
`tool_result {outcome:{withheld:"quarantined"}}`; it never claims the tool did
not execute. Pre `deny` produces `tool_result {outcome:{denied:reason}}` and
does not execute. Pre `ask` appends the D-45 `approval_request` barrier and
parks. Pre `mutate` changes only execution. For every mutated call the full
effective invocation, inline or `$spill`, is appended as the D-45
`effective_execution` barrier before execution. This keeps replay and
supervisor-control argument binding identical for read-only and side-effectful
backends.

All effective pre hooks run in the instruction-snapshot order. Each mutation
is the next pre hook's invocation. If at least one pre hook mutates, exactly
one `effective_execution` is appended after the complete pre chain, containing
the final invocation, before validation or backend execution. A pre deny, ask,
or closed failure terminates the chain without running the backend.

All effective post hooks run in the same stable order after the backend. Each
post request sees metadata accumulated from earlier hooks. Allow annotations,
deny reasons, and hook failures are retained as ordered entries rather than
overwriting one another. A post deny or closed failure marks the eventual
result quarantined but does not skip later post observers.

## Failure, limits, and secrecy

Each binding freezes `timeout_ms`, `stdout_bytes`, `stderr_bytes`, and
`failure: "open" | "closed"` in the instruction snapshot. Values are positive
safe integers; implementations also enforce product hard ceilings. Timeout
means TERM, bounded wait, then KILL and reap. `closed` maps pre failure to a
denied result and post failure to a quarantined result. `open` preserves the
unmodified invocation/output and records the typed failure in result metadata.

Hook output and tool output pass the same total secret transform before any
event or asset publication: `clean`, `redacted`, or terminal
`withheld {scan_failed | quarantined}`. A scanner failure therefore cannot
leave a durable `tool_call` unpaired.

## Correlation and replay

One hook request has one response. There is no retry inside a running tool
transaction; a process failure is resolved by the binding's failure policy.
Replay never reruns hooks or the wrapped effect. An unpaired side-effectful
`tool_call` is reconciled by the lifecycle contract, using a durable
`effective_execution` when present.

`fixtures/hooks/` is the byte oracle for requests, every verdict arm,
correlation rejection, malformed output, and failure-policy behavior.
