# Folder trust in 0.8

Folder trust is separate from the session's permission mode and environment.
Allowing `$HOME` as a writable folder never makes project code trusted. User
instructions and Kernel workspace instructions remain host-owned resources;
project AGENTS, rules, settings and commands retain their existing read behavior.
Project hooks (formats 1 and 2), skills and stdio MCP require a host grant.
Project HTTP MCP retains its existing transport behavior.

## Authorization

The protected `<storage>/config/folder-authority.json` records global folder
identity, canonical path, filesystem identity, registry revision, folder trust
revision, pending requests and operation receipts in one atomic publication.
Filesystem integers are decimal strings, preserving their exact values through
JCS. Identity is derived from device, inode and birth time, with a domain-separated
opaque SHA-256 id. Aliases share an identity; a replaced directory never inherits
trust. A move keeps the entity id but requires a new request and host approval at
the new canonical path. Missing birth-time support, damaged state and stale
identity/revision fail closed.

The `folderTrust.v1` extension exposes only:

- `folders.trust.status {path}` → `{revision,folder}`.
- `folders.trust.request {path,expectedRevision}` → `{requestId,receipt}`.
- `folders.trust.revoke {folderId,expectedRevision}` →
  `{receipt,effectsDrained,poolCleanup:"pending"}`.

There is no grant RPC and no client `trusted` field. RPC ids identify mutations;
the stored canonical request digest and expected revision prevent conflicting
retries. The latest 4096 operation receipts are retained. Expired retries fail the
monotonic revision check rather than reapplying a write. Up to 1024 identities
may be requested; newer requests for one identity replace its old pending
request. Receipt capacity cannot prevent revoking an existing grant.

The local administrator uses the same protected store from outside the worker
sandbox:

```sh
tekes-supervisor --folder-trust status /absolute/state /absolute/project
tekes-supervisor --folder-trust request /absolute/state /absolute/project request-1 0
tekes-supervisor --folder-trust grant /absolute/state request-1 grant-1 1
tekes-supervisor --folder-trust revoke /absolute/state FOLDER_ID revoke-1 2
```

Grant opens `/dev/tty`, displays the JSON-escaped path, identity and expected
revision, and requires `approve "request-1"`. It then checks that exact pending
request again under the authority mutation lock. There is no `--yes` shortcut.
An exact retained grant retry returns its historical receipt without granting
again; inspect `status` for current trust. The terminal supplies consent. The
security boundary is protected host storage plus the OS sandbox, not possession
of a terminal. Session tools cannot write it through the CLI or file tools.
The built-in endpoint's management bearer is passed through private bootstrap;
it is not present in the initial environment. Host administrators, host-installed
User MCP and intentionally trusted host extensions remain privileged code.

## Execution and revocation

Discovery checks trust before reading or parsing project hook/skill files.
Instruction snapshots capture authoritative folder provenance by project index.
Old snapshots still decode, but missing provenance cannot authorize project
execution. Every tool dispatch, continuation hook, lifecycle hook (including
cached context receipts), skill lookup and resource read checks the captured
identity and revision against current authority.

An admitted effect holds a per-folder shared barrier. Revocation publishes its
new revision immediately, closing admission without waiting on a broker or a
running effect. In-flight effects may finish; `effectsDrained:false` explicitly
reports that state. It is not a pool-cleanup acknowledgment. Idle revoked MCP
peers are checked every 250 ms and closed/reaped; an occupied peer is checked as
soon as it becomes idle. Broker calls check admission after acquiring the peer
mutex, so previously queued calls cannot execute after revocation. Regranting
creates a different generation and cannot revive an old key.

## MCP owners and compatibility

Durable `{workspace_id,scope,name}` references remain compatibility addresses.
Their workspace string is not pool ownership. User rows belong to global
`user`; project rows belong to `folder:<id>`; verified plugins belong to their
plugin owner. Project config accepts optional `folder_id`. A legacy project row
can map only through a single-folder workspace; ambiguous ownership is refused,
without migrating `project_trusted:true` into a grant.

Equivalent enabled User definitions, including credential reference states,
merge across workspace aliases. Unequal definitions report a conflict and do not
spawn a winner. Equivalent project aliases with the same folder id also merge.
Different project owners defining the same visible name conflict. Existing
project-over-user precedence is retained. Repair conflicting aliases explicitly
through management; historical rows and credential pending/bound state are not
silently rewritten.

The pool key is canonical owner, scope, server name, effective configuration
with folder identity/trust revision, credential ids/generations, protocol mode
and verified plugin generation. Compatibility workspace ids and the legacy
`project_trusted` bit are excluded from the execution digest. Creation is
serialized per full key before spawning; each bound scope retains its own lease.
Resource list/read also reuse these generations. Cache keys include the session's
actual folder paths, and cache fingerprints include authoritative folder identity
and trust revision. Optional `sessionId` on resources list/read resolves that
session's recorded folders; workspace-only calls retain compatibility behavior.
Mutation and lifecycle reconciliation inspect idle cached routes as well as live
workers. Closing a stdio generation kills its owned process group and waits for
the direct child and stderr drain; this does not contain descendants that
intentionally detach into another process group.
