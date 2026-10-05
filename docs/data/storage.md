# Data storage: directories, execution logs and assets

[Data and reliability · documentation home](../README.md) · [Thread concepts](../concepts/thread.md) · [Next: event format](events.md)

## Persistence scope

One thread = one directory. Copying the directory yields a complete,
independently replayable snapshot — for reading, backup, and audit.
**Live use of a copy is never a second writer**: it is a fork (new identity)
or a quiescent transfer, per D-47. Self-containment beats deduplication.

## Layout

```
~/.agents/
  settings/                      # user preferences; never project-authored
  skills/                        # user-level skill packages
  client/ClientMirror.sqlite3    # Tekes-owned rebuildable UI mirror
  runtime/composer/              # draft attachment staging
  not-in-project/                # client fallback workspace
  threads/
    <thread-id>/                 # UUID; never renamed (titles are metadata)
      main.jsonl                 # the mainline line file
      <child-id>.jsonl           # zero or more child line files (flat, beside main)
      assets/
        sha256-<hex>             # content-addressed blobs (images, PDFs, big tool output, rendered provider requests)
      .thread-search.json        # rebuildable title-search projection; deletable
      endpoint.jsonl             # endpoint journal: projection of main.jsonl; never worker input
      endpoint.lock              # serializes projection allocation/emission
      control/                   # D-70 exact-retry carrier: intent, then receipt, per request id; GC after terminal tool_result
      orphans/                   # designed sweep quarantine; not produced today
  archive/
    <thread-id>/                 # archived threads: same shape, moved wholesale
  workspaces/
    <workspace-id>/
      workspace.json           # one workspace authority; stable folder binding ids
      state/                   # workspace-scoped runtime state
      skills/                  # optional workspace-level skill packages
memory/
  log.jsonl                    # global memory event log (note/merge/recall),
                               # named-lock tool transactions, LWW + tombstones
goals/
  log.jsonl                    # host-bound goal transactions, keyed by
                               # (thread, call); append-only and barrier-synced
tool-state/
  artifact-versions.jsonl      # apply_patch version/SHA reservations + commits
  artifact-versions.lock       # cross-process serialization for that log
credential-state/
  .lock                        # serializes provider-secret generation floors
  provider-secret-generations.json # durable anti-rollback authority; no secrets
jobs/
  <job-id>/                    # durable ownerless JobBroker records/output tails;
                               # supervisor-created 0700, never a thread folder
config/
  providers.json               # providers/endpoints/enabled models — secrets by key id only
  mcp-servers.json             # MCP server registry (mcp-runtime)
  settings.json                # global settings
cache/                         # rebuildable probe results (catalogs, capability tables)
staging/                       # live rewrite op.json records + payload builds
endpoint-management/          # endpoint-scoped management/rpc protocol authority
  rpc.lock
  rpc/                         # per-rpc canonical JSONL exact-retry carriers
  workspaces/                  # durable workspace metadata sidecars
  operations/                  # crash-recoverable config/folder operations
.create-staging/               # keyed-create recovery; never rewrite-GC'd
.rewrite-trash/                # redact tombstones owned by live op.json
.rewrite.lock                  # serializes rewrite create/advance/debris GC
.thread-search.lock            # serializes rebuildable search-cache writers
.thread-catalog.lock           # serializes active/archive membership changes
```

**The instruction plane has three explicit scopes (D-55 as amended by
D-72):** user level = `~/.agents/` (`AGENTS.md`, `skills/`, `commands/`,
`hooks/`, `settings.json`); workspace level =
`~/.agents/workspaces/<workspace-id>/`; project level = `<cwd>/AGENTS.md` +
`<cwd>/.agents/`, one per bound workspace folder. The legacy project
`<cwd>/.agent/` layout remains a read-only discovery fallback only when the
project has no `.agents/` directory. All sources are loaded at spawn and
labeled `user`, `workspace`, or `project:<folder-index>`.
`AGENTS.md` is the open standard name (interop with other tools); nested
per-subdirectory files are deferred. These are user-config authority.
**Resolution materializes one content-addressed snapshot at launch**
(R3-20/D-55): deterministic scope/folder order, conservative policy meet;
the snapshot is passed to the worker by
descriptor and its digest lands in `run_start` and the epoch profile — no
worker reads live instruction paths mid-run, so an edit during spawn cannot
produce a mixed or irreproducible profile. An instruction change is a
system-source change and opens a named epoch at the next spawn ([Model context](context.md)). Project
`.agents/` files are ordinary workspace files: agents edit them with ordinary
file tools under writable roots (user-visible customization, not autonomous
promotion — R3-16); no special write machinery exists. A skill name present
in more than one scope is an explicit configuration error; it is never
silently replaced. Project `.agents/tools/` is reserved project-authored input
but format 1 does not auto-execute arbitrary files there: executable tools
still require the closed builtin, configured helper, MCP, or plugin contract.

`config/` files are user-authored authority like workspaces: supervisor-
written, atomic-swapped, versioned into spawn snapshots. They reference
secrets **by key id only** — values live in the platform secret store behind
the `SecretStore` seam ([platform boundaries](../architecture/platform.md)), and reach workers via descriptor (D-46). Format 1
does not deliver integration `credential_env`: such an integration is omitted
until a closed integration-secret protocol exists. `cache/` is deletable; a
fact that survives only in `cache/` is a bug (D-1).

A workspace maps to many threads and many working directories. Its definition
file is the second legitimate truth class — user-authored config, written only
by the supervisor (atomic temp + rename), never derived and never rebuildable
from events. A thread binds its workspace by id in `genesis` (rebind is a
`meta` event) and records one stable `folder_binding`. That binding selects
the execution cwd; it does **not**
reorder or narrow the workspace's authored folder list. Instruction capture
still includes every bound folder in authored order, and workspace resource
catalogs remain isolated from other workspaces. A legacy single-folder thread
without `folder_binding` resolves as `folder-0001`; a legacy thread that now
maps to more than one folder is ambiguous and fails closed before execution.
The worker never reads workspace files — it receives the
resolved cwd/writable-roots set as spawn config (D-36).

`memory/log.jsonl`, `goals/log.jsonl`, `tool-state/artifact-versions.jsonl`,
`credential-state/provider-secret-generations.json`, and the records below
`jobs/` are D-1 named durable authorities,
not cache. Tool transactions append canonical records under their adjacent
named lock, sync each record before returning success, and deduplicate by the
durable `(thread, call)` identity. No backend-private `offers` log exists:
causal skill/dynamic-tool offers are proved by the originating thread's
durable `tool_result`. These global logs are outside thread folders so fork,
archive, and rewrite never copy or silently rewrite external goal/memory/file
version state. The credential-generation authority follows
`secret-store`: it stores only generation/digest commitments, survives an
ordinary uninstall with retained provider secrets/config, and is consulted
before broker scope construction.

## Rules

1. **A file is a line of work, not a session.** `main.jsonl` lives as long as the
   thread; successive worker processes append to it in relay (flock serializes).
   Child files each hold one delegated task. Every file is schema-identical and
   run by the same worker binary — "root" is a position in the process tree, not
   a type. Parentage is recorded in events (genesis + spawn), never in structure.
2. **Creation is keyed and serialized** (R3-1): `genesis` records the
   creator's `origin_key`; the supervisor serializes creates through its
   single creation path and answers a retried key with the existing thread
   id (the receipt index is a rebuildable cache over genesis scans). Two
   concurrent same-key creates cannot both mint folders.
3. **Folder name is the thread id** (D-64: lower-case canonical UUID; new native
   ids use a process-safe UUIDv7 generator). Display ordering uses genesis
   `ts` plus the UUID tie-breaker; raw UUID order is never semantic. The display
   title and other thread-level facts are `meta` events in `main.jsonl`
   (supersedable); renaming a thread appends an event and never moves the
   folder. The manifest only caches them.
4. **Assets are content-addressed.** Events reference
   `{"asset":"sha256-…","mime":…,"bytes":…}`; binaries never inline in JSONL.
   Write via temp file + `rename`, following the durable-reference protocol
   (file sync → rename → parent-dir sync before the referencing event syncs,
   D-45). Fork copies assets with `clonefile` where the volume supports it,
   verified copy+sync elsewhere (D-11). A genesis seed snapshot is a
   schema-declared carrier: its own blob and every asset it references are
   live, copied, and content-scanned on rewrite/redact (event §Seed
   snapshot asset format). The config and instruction snapshot digests in
   `genesis`/`run_start` are also implicit asset references under
   [config](../../spec/config.md) and
   [instruction-snapshot](../../spec/instruction-snapshot.md). GC, fork, archive, and
   redact retain/process every carrier class. GC deletes
   only a blob no event or declared carrier references — the recursive
   reference scan is itself a rebuildable projection.
5. **manifest.json is a cache, never truth.** Atomic snapshot (temp + rename),
   written only by the supervisor, caching: title (from `meta` events), created
   (from `genesis` ts), per-file `{name, last_seq, settled, seq→offset
   samples}`, the child tree. Deleting every manifest and rescanning must
   reproduce the system exactly. If any fact survives only in a manifest, that
   fact is a bug in this contract — promote it to an event.
   `.thread-search-v1.json` is the equally deletable, per-folder search cache
   defined by [thread-search](../../spec/thread-search.md). It moves with
   archive, is omitted from fork/redact materialization, and is validated
   against the semantic ledger before use.
   `endpoint.jsonl` is the executable
   [endpoint projection journal](../../spec/session-endpoint.md#journal-projection): every
   row is a deterministic projection of a semantic ledger event, and the file
   is rebuilt by reconciliation (a journal under the earlier
   `endpoint-v2.jsonl` name is retired on open). It owns the public endpoint
   seqs clients cursor on, never participates in worker replay or provider
   input, and never holds provider chunks — those are transient.
6. **Archive = `mv` under the catalog and folder lifecycle locks.** The
   supervisor takes root `.thread-catalog.lock` exclusive, then the folder
   lifecycle lock. The lifecycle lock is
   a flock on the thread *directory* fd: every worker holds it **shared** for
   its lifetime (taken at startup, before the file lock); the supervisor
   takes it **exclusive** for archive, delete, and rewrite publication — so a
   folder can never move under a live process, and a candidate that opened a
   path pre-rename fails inode revalidation ([Event definitions](events.md)) and retries. With a live
   process the endpoint request returns typed `session-running`; the caller may
   retry after exit. An explicit stop remains a separate durable operation.
   Unarchive is the reverse `mv`.
7. **Fork/redact is a materialized projection, not a byte copy** (D-40/47/50).
   The executable authority is
   [rewrite-publication](../../spec/rewrite-publication.md): it fixes the
   source-prefix digest, complete file/id map, seq/parent remapping,
   historical provider-row normalization, supersede/compact application,
   declared carrier closure, and selector-based redact. Attempts,
   checkpoints, continuation and sealed carriers never enter the destination;
   the endpoint projection journal and endpoint seqs never enter the
   destination; it starts empty and rebuilds deterministically from the
   materialized semantic prefix (answerable requests are derived from that
   prefix's holds);
   root-scoped endpoint rpc/operation records never enter the destination;
   redact scans source-targeted records and replaces secret-bearing complete
   authorities with session-endpoint §Management authority and recovery's content-free retired tombstones;
   D-70 control receipts are scanned by redact but never enter the destination;
   it starts an empty journal over the materialized ledger;
   a fresh epoch carries only the latest clean provider profile. Publication
   uses a synced `staging/<op-id>/op.json` and the type-specific phase machines
   `fork: prepared→built→validated→published→closed` and
   `redact: …→published→source_retired→source_removed→closed`. Sweep resumes
   records idempotently. Rewrite GC deletes only recordless entries in
   `staging/`; keyed creates live in `.create-staging/` and are out of scope.
8. **Cross-thread communication** never writes another folder directly. All
   cross-folder delivery goes through the supervisor ([supervisor](../runtime/supervisor.md)): target alive →
   stdin; target dead → supervisor appends under lock.

## Placement

- Data root: `~/.agents/`; active thread root: `~/.agents/threads/` (per-user default;
  server deployments configure an explicit root). **Single host, supported
  local filesystem only** — flock/O_APPEND semantics are not guaranteed on
  NFS/SMB; the supervisor probes lock/append capability at boot and refuses
  network or unknown volumes (F10).
- **Never under an iCloud-synced path** (iCloud Drive, Desktop/Documents sync).
  Dataless-file materialization stalls reads and can wedge replay. The
  supervisor asserts at boot that the root is not inside an iCloud container and
  refuses to start otherwise.
- Backup: Time Machine (do not exclude). Sync/migration between machines is
  explicit export = copy the folder; the format is the interchange.
- Multiple backends/instances each get their own root; folders never interleave.

## Manifest and indexes

- `manifest.json` per thread ([Thread definition](../concepts/thread.md)): title, timestamps, file list with
  `last_seq`/settled flags, child tree, `seq→offset` samples. Supervisor-owned,
  atomic-swapped, deletable.
- Global sidebar index, usage aggregates, full-text search index: same rule —
  derived, deletable, rebuilt by scan. Search starts as ripgrep-over-JSONL;
  add an FTS index only when measured too slow, and even then it is a
  disposable accelerator.
