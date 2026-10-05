# thread-search — rebuildable session title search

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Builds a title index from authoritative logs, providing pagination, archive filtering and workspace-scoped queries.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `lib` | ThreadSearchAuthority, SearchRequest, SearchResult, matching and cursors |

## Interfaces and calls

Main entry points: ThreadSearchAuthority, SearchRequest, ArchiveVisibility.

Management entry point → search authority → read/rebuild cache → title matching → stable pagination.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Search covers titles, not full conversation text; cache invalidation does not change session semantic history.

Behavior contract: [thread-search.md](../../../spec/thread-search.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
