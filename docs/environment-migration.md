# Session, Environment and Folder migration

This describes the architecture being prepared for Kernel 0.8.0. It does not
announce a release or change a package version.

## Authority and compatibility

- A Session is the primary entity. Its preset and creation seed are frozen by
  value. Its Environment is a reference resolved at each launch; absent means
  `local`. A running worker retains its immutable launch descriptors.
- `session.create {cwd}` selects only that Folder. Explicit `workspaceId`
  copies a compatibility template once. Later workspace policy or Folder edits
  do not change detached sessions. Old workspace fields and routes remain
  compatibility addresses, including in historical snapshot wire shapes.
- Folder ids are stable logical identities in `config/folders.json`. Relocation
  updates paths, never ledger history or physical trust. Reusing the old path
  creates a separate identity. Grouping is presentation, not an access boundary.
- Folder edits are per-session ledger events, effective at the next launch.
  Call `session.listFolders` for the scope revision; normal conversation seqs
  do not advance it. A running worker causes `workspace-busy`. Retry an
  ambiguous result with the same RPC id and payload; never replay it as a fresh
  mutation. See [the extension contract](../spec/client-extensions.md#session-folder-administration).
- A new session scope or scope-change event requires ledger reader **4**.
  This is separate from endpoint protocol version **3** and private bootstrap
  version **1**. Older readers must not write records requiring a newer reader.
  Legacy sessions lazily use their checksum-verified original creation asset;
  records without a usable asset retain the workspace adapter and fail closed
  when that adapter is unavailable. There is no eager startup ledger rewrite.
- Cross-session context reads/messages and explicit global search do not
  require shared folders. Control actions use the acting session's approval
  policy; received content uses the receiver's permissions.
- Folder trust is granted by independent host authorization. Client RPCs cannot
  grant it. Project hooks, executable skills and stdio MCP require trust.
  `/` is rejected; HOME can be a Folder, but Kernel data and credential roots
  remain write-protected. See [Folder trust](folder-trust.md).

## Launcher adoption

Applications must adopt the [private pipe bootstrap](builtin-launch.md) before
embedding this Kernel. Tokens and credential values must not enter the initial
environment, argv or launch JSON. Legacy secret environment inputs are rejected.
Keep the pipe open for the existing application lifetime contract.

The Kernel Swift contract harness compiles contract DTOs and
`NativeSessionEndpoint*.swift`; it does **not** compile or exercise an
application's `KernelManagedServer` launcher. Passing that harness is not proof
that an application launcher or its UI has adopted this migration. Real
application startup, reconnect, shutdown, trust authorization and session Folder
controls remain separate consumer acceptance work.

## Kernel acceptance

Run `scripts/ci.sh` with the pinned architecture dependencies installed, and
run the built-in process/Swift contract harness with the intended client source
checkout. Linux CI additionally checks both x86_64 and aarch64 sandbox backends.
The macOS local suite alone does not establish Linux or application acceptance.

Failure-path coverage must include catalog preparation before append, rejected
configuration, partial or absent ledger append, a different event at the
prepared seq, committed append before catalog publication, publication failure,
and recovery after publication or session archival. Only the exact prepared
valid ledger event may publish its catalog state. MCP shutdown must cover an
already-exited group leader, persistent group-signal denial, descendant-owned
stderr and normal broker removal. External provider and ignored platform tests
must be reported separately from the hermetic regression suite.
