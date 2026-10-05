# host-files — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [host-files::METHODS](../../src/lib.rs#L26) | `pub` | not a function |
| [host-files::DIRECTORY_LIST_CAP](../../src/lib.rs#L34) | `pub` | not a function |
| [host-files::FILE_REFERENCE_LIMIT](../../src/lib.rs#L36) | `pub` | not a function |
| [host-files::FILE_REFERENCE_WALK_BUDGET](../../src/lib.rs#L38) | `pub` | not a function |
| [host-files::SESSION_REFERENCE_LIMIT](../../src/lib.rs#L40) | `pub` | not a function |
| [host-files::Failure](../../src/lib.rs#L47) | `pub` | not a function |
| [host-files::Failure::code](../../src/lib.rs#L65) | `pub` | no resolved direct caller |
| [host-files::Failure::message](../../src/lib.rs#L78) | `pub` | no resolved direct caller |
| [host-files::SessionCandidate](../../src/lib.rs#L106) | `pub` | not a function |
| [host-files::validate](../../src/lib.rs#L145) | `pub` | [tekes-supervisor::client_extensions::validate_payload](../../../supervisor/src/client_extensions.rs#L2834) |
| [host-files::list_directory](../../src/lib.rs#L228) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::host_files](../../../supervisor/src/client_extensions.rs#L1651) |
| [host-files::create_directory](../../src/lib.rs#L276) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::host_files](../../../supervisor/src/client_extensions.rs#L1651) |
| [host-files::file_references](../../src/lib.rs#L321) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::host_files](../../../supervisor/src/client_extensions.rs#L1651) |
| [host-files::session_references](../../src/lib.rs#L412) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::host_files](../../../supervisor/src/client_extensions.rs#L1651) |
