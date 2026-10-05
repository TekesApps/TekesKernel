# Thread search v1 fixtures

These files are the language-neutral Slice 14D oracle for
`spec/thread-search.md`. Every `*.canonical.json` file is one RFC-8785/JCS
object plus LF. `cases.canonical.json` freezes stable result identities,
normalization/ranking and an opaque first-page cursor. `index.canonical.json`
freezes the per-folder rebuildable cache shape. `invalid.canonical.json`
contains malformed cursors that must fail closed before source paging.

The Rust gates materialize semantic ledgers from the source rows, rather than
treating this corpus as thread truth, and assert that cache deletion,
corruption and staleness cannot alter results.
