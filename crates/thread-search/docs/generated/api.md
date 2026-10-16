# thread-search — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [thread-search::host::SessionSearchHit](../../src/host.rs#L6) | `pub` | not a function |
| [thread-search::host::SessionSearchResults](../../src/host.rs#L13) | `pub` | not a function |
| [thread-search::host::ThreadSearchAuthority::search_sessions](../../src/host.rs#L21) | `pub` | no resolved direct caller |
| [thread-search::ArchiveVisibility](../../src/lib.rs#L33) | `pub` | not a function |
| [thread-search::SearchRequest](../../src/lib.rs#L40) | `pub` | not a function |
| [thread-search::MatchClass](../../src/lib.rs#L50) | `pub` | not a function |
| [thread-search::SearchResult](../../src/lib.rs#L58) | `pub` | not a function |
| [thread-search::IndexUse](../../src/lib.rs#L70) | `pub` | not a function |
| [thread-search::SearchPage](../../src/lib.rs#L80) | `pub` | not a function |
| [thread-search::ThreadSearchAuthority](../../src/lib.rs#L89) | `pub` | not a function |
| [thread-search::ThreadSearchAuthority::open](../../src/lib.rs#L94) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::open](../../../supervisor/src/client_extensions.rs#L315); [thread-search::tests::slice14d_gates::slice14d_gate_105_contract_oracle_and_stable_identity](../../tests/slice14d_gates.rs#L21); [thread-search::tests::slice14d_gates::slice14d_gate_107_rebuild_and_stale_corrupt_fallback](../../tests/slice14d_gates.rs#L232); [thread-search::tests::slice14d_gates::slice14d_gate_108_fail_closed_and_semantic_non_mutation](../../tests/slice14d_gates.rs#L290); [thread-search::tests::slice14d_gates::host_search_matches_content_across_workspaces_and_archive](../../tests/slice14d_gates.rs#L527); [thread-search::tests::slice14d_gates::global_title_search_is_explicit_and_cursor_filters_cannot_be_widened](../../tests/slice14d_gates.rs#L574); [thread-search::tests::slice14d_gates::slice14d_gate_106_paging_archive_visibility_and_membership_lock](../../tests/slice14d_gates.rs#L80) |
| [thread-search::ThreadSearchAuthority::search](../../src/lib.rs#L103) | `pub` | no resolved direct caller |
| [thread-search::ThreadSearchAuthority::rebuild_index](../../src/lib.rs#L175) | `pub` | no resolved direct caller |
| [thread-search::skipped_sessions](../../src/lib.rs#L323) | `pub` | [thread-search::tests::slice14d_gates::slice14d_gate_108_fail_closed_and_semantic_non_mutation](../../tests/slice14d_gates.rs#L290) |
| [thread-search::SearchError](../../src/lib.rs#L635) | `pub` | not a function |
