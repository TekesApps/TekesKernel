# Thread search v1

Status: executable contract for Slice 14D.

This contract owns the Kernel's internal, language-neutral thread-title search
authority. It does **not** add a Session Endpoint method or capability. Slice
14F may bind this seam to a separately versioned Client-owned route only after
its DTOs and negotiation are frozen.

## Scope and authority

Search covers the latest effective `meta.title` in each root `main.jsonl`.
It does not search transcript text, child files, tool output, assets, workspace
files, or endpoint carrier journals. The stable result identity is the pair
`(workspace_id, session_id)`; native `session_id` remains the lower-case
canonical UUID from `genesis.thread` and the folder name. Search never invents
another thread id.

The semantic ledger and the folder's active/archive placement are the only
authorities. `.thread-search.json` is a per-folder, rebuildable projection.
It may improve query work but cannot authorize a result, preserve a rename,
or decide archive visibility. Every query first obtains one catalog snapshot
from ledger bytes and validates cache rows against that snapshot. Missing,
stale, or malformed cache bytes fall back to the scanned row and are repaired
best-effort; inability to rewrite a cache cannot turn a correct scan into a
failed query. Deleting every search cache preserves exact results.

Archive moves the cache with the folder. Fork/redact materialization never
copies it: the destination rebuilds from its rewritten ledger, and retirement
removes the source cache with the source folder. Thus the cache adds no carrier
or redaction exception.

## Source snapshot and locking

The authority serializes its own cache writers with root
`.thread-search.lock`, then takes `.rewrite.lock` shared and
`.thread-catalog.lock` shared, in that order, while enumerating both
`threads/` and `archive/`. Create and every active/archive folder move take
`.thread-catalog.lock` exclusive across directory publication and directory
sync. The existing nonblocking archive path also acquires the catalog lock
before it tries the lifecycle lock; catalog contention is an internal wait and
must not be misreported as `session-running`. The lifecycle acquisition
remains nonblocking. Rewrite
writers take `.rewrite.lock` exclusive and then `.thread-catalog.lock`
exclusive across preparation, publication and retirement; create rejects a
destination reserved by a live rewrite. No writer acquires these two locks in
the opposite order. This produces one collision-free membership snapshot; seeing the
same UUID in both areas is corruption, never a duplicate result.

For each folder, read `main.jsonl` and accept its longest schema-valid,
LF-terminated prefix. A trailing partial line is ignored, as in the store tail
contract. An invalid **complete** line after that prefix is corruption and
fails the query closed; search does not repair semantic ledgers. The first
event must be `genesis`, `genesis.thread` must equal the folder UUID, and
`genesis.workspace` supplies the workspace scope. The source row is:

```text
{session_id:uuid, workspace_id:str, title?:str, normalized_title?:str,
 updated_at:str, archived:bool, as_of_seq:int}
```

`title` is the newest unsuperseded `meta` event carrying `title`; every later
`supersedes` range is applied before selecting it. No live title means the
thread is not matchable. `updated_at` is the last valid event's `ts`. Rows are sorted
by UTF-8 bytes of `session_id`. A query's `catalog_digest` is
`"sha256-" + lowercase_sha256(JCS(scoped_rows))`, after workspace and archive
visibility filtering but before text matching. Concurrent appends may make a
later query observe a longer valid prefix; a page cursor detects that catalog
change instead of mixing snapshots.

## Text normalization and ranking

Normalization is exact and locale-independent:

1. apply Unicode NFKC;
2. map only ASCII `A`...`Z` to `a`...`z` (non-ASCII case is preserved);
3. map every Unicode White_Space scalar from Unicode 16.0
   (`0009..000D, 0020, 0085, 00A0, 1680, 2000..200A, 2028, 2029, 202F,
   205F, 3000`) to one ASCII space between non-space runs; drop leading and
   trailing space.

The request query is valid only when its UTF-8 input is at most 512 bytes,
contains no control scalar outside that explicit White_Space set, and
normalizes nonempty. `workspace_id` uses config's existing identity grammar:
1...128 ASCII characters from `[A-Za-z0-9._-]`, not starting with `.`.
`limit` is 1...50. Invalid values fail closed; they never become an
empty-success query.

A title matches in exactly one highest class:

| score | class | predicate over normalized bytes/scalars |
|---:|---|---|
| 3000 | `exact` | title equals query |
| 2000 | `prefix` | title starts with query |
| 1000 | `tokens` | every space-separated query token is a substring of title |

Results sort by `(score descending, session_id UTF-8 ascending)`. No fuzzy,
locale, recency, or index-engine score participates. Each result copies the
source identity, title, updated timestamp, archive bit, `as_of_seq`, score and
match class. Therefore independent implementations produce the same ordering.

## Archive visibility

Every internal request carries exactly one closed value:

- `active` — only folders currently in `threads/`;
- `archived` — only folders currently in `archive/`;
- `all` — both, with no score bias.

There is no implicit default. Archive/unarchive changes the scoped catalog
digest and invalidates an older cursor. Searching archived threads is an
explicit caller choice; a later Client capability must preserve that choice
and its authorization policy.

## Cursor and paging

The cursor is opaque text:

```text
base64url-no-padding(JCS(body)) "." lowercase_sha256(JCS(body))
```

The closed body is:

```text
{v:1, workspace_id:str, visibility:"active"|"archived"|"all",
 normalized_query:str, catalog_digest:"sha256-"+hex64,
 score:1000|2000|3000, session_id:uuid}
```

Decode requires canonical JCS bytes, one dot, exact unpadded base64url, a
matching 64-lowercase-hex checksum, exact field set, `v:1`, and a cursor row
that exists at the named score in the current ranked result. Structural
failure is `malformed-cursor`; request scope/query differences are
`cursor-scope`; a catalog digest difference is `cursor-stale`; a missing
ranked position is `cursor-position`. None restarts silently.

The first page begins at rank zero. A continuation begins immediately after
the named `(score, session_id)` row. Return at most `limit` rows. If more rows
remain, `next_cursor` names the last returned row and `reached_end` is false;
otherwise `next_cursor` is absent and `reached_end` is true. An empty result is
`[]`, no cursor, and `reached_end:true`.

## Cache bytes and recovery

Each `.thread-search.json` is one RFC-8785/JCS object plus LF:

```text
{format:1, source_digest:"sha256-"+hex64, entry:source-row}
```

`source_digest` is the digest rule above over a one-element row array. The
decoder rejects unknown fields, noncanonical bytes, missing final LF, multiple
lines, unsupported format, noncanonical UUID, invalid config workspace
identity, zero `as_of_seq`, wrong normalized title, or digest mismatch.
Every rejection is cache-local: use the scanned source row and attempt atomic
temp+sync+rename+directory-sync repair. Explicit `rebuild_index` performs the
same scan and publication for every active and archived folder and returns the
digest of the complete source row array; unlike query-time repair, an explicit
rebuild reports a write failure.

## Internal authority seam

The production seam is logically:

```text
search({workspace_id, query, limit, visibility, after?})
  -> {results, next_cursor?, reached_end, catalog_digest, index_diagnostics}
rebuild_index() -> complete_catalog_digest
```

The closed error classes are: `invalid-workspace`, `invalid-query`,
`invalid-limit`, `malformed-cursor`, `cursor-scope`, `cursor-stale`,
`cursor-position`, `source-identity`, `source-corrupt`, and internal I/O/store
failure. `index_diagnostics` is an internal count of hit/missing/stale/corrupt/
write-failed rows and is not automatically a Client DTO. Calls are read-only
with respect to semantic truth and endpoint carriers.

## Required proof

Slice 14D gates prove:

1. canonical request/result/index/cursor bytes and exact normalization/ranking;
2. stable paging for all three visibility modes, cursor scope/staleness, and
   held membership locks across archive/create transitions;
3. missing/stale/corrupt cache scan fallback, best-effort repair, explicit
   rebuild, and no result change after deleting every cache;
4. malformed query/cursor/source fail-closed behavior, no semantic ledger or
   endpoint-journal mutation, and no public route/capability registration.
