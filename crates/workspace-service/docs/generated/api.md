# workspace-service — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [workspace-service::git::Query](../../src/git.rs#L15) | `pub` | not a function |
| [workspace-service::git::query](../../src/git.rs#L151) | `pub` | [workspace-service::main::main](../../src/main.rs#L13) |
| [workspace-service::git::MutationAuthority](../../src/git.rs#L731) | `pub` | not a function |
| [workspace-service::git::mutate](../../src/git.rs#L796) | `pub` | [workspace-service::main::main](../../src/main.rs#L13) |
| [workspace-service::Failure](../../src/lib.rs#L18) | `pub` | not a function |
| [workspace-service::FilesRequest](../../src/lib.rs#L45) | `pub` | not a function |
| [workspace-service::file_snapshot](../../src/lib.rs#L83) | `pub` | [tekes-supervisor::file_leases::WorkspaceFileTransfer::prepare](../../../supervisor/src/file_leases.rs#L39) |
| [workspace-service::file_text_page](../../src/lib.rs#L128) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::client_file_page](../../../supervisor/src/process_host.rs#L1052) |
| [workspace-service::file_byte_page](../../src/lib.rs#L199) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::client_file_page](../../../supervisor/src/process_host.rs#L1052); [workspace-service::observation::FileObservations::register](../../src/observation.rs#L13); [workspace-service::observation::FileObservations::poll](../../src/observation.rs#L30) |
| [workspace-service::files](../../src/lib.rs#L295) | `pub` | [workspace-service::main::main](../../src/main.rs#L13) |
| [workspace-service::observation::FileObservations](../../src/observation.rs#L8) | `pub` | not a function |
| [workspace-service::observation::FileObservations::register](../../src/observation.rs#L13) | `pub` | no resolved direct caller |
| [workspace-service::observation::FileObservations::poll](../../src/observation.rs#L30) | `pub` | no resolved direct caller |
| [workspace-service::process::invoke](../../src/process.rs#L26) | `pub` | [workspace-service::tests::process_files::cancelling_parent_request_terminates_descendant_process](../../tests/process_files.rs#L186); [workspace-service::tests::process_files::complete_file_crosses_preview_and_helper_output_limits](../../tests/process_files.rs#L236); [workspace-service::tests::process_files::parent_timeout_terminates_and_reaps_child](../../tests/process_files.rs#L26); [workspace-service::tests::process_files::parent_invocation_returns_child_result](../../tests/process_files.rs#L9); [workspace-service::tests::process_git::call](../../tests/process_git.rs#L20) |
| [workspace-service::process::invoke_with_authority](../../src/process.rs#L36) | `pub` | [tekes-supervisor::workspace_routes::WorkspaceRoutes::execute](../../../supervisor/src/workspace_routes.rs#L198); [tekes-worker::workspace_edits::WorkspaceEditRecorder::invoke](../../../worker/src/workspace_edits.rs#L103); [tekes-worker::workspace_edits::tests::actual_patch_helper_to_turn_ledger_round_trip](../../../worker/src/workspace_edits.rs#L205); [workspace-service::process::invoke](../../src/process.rs#L26); [workspace-service::tests::process_turn::call](../../tests/process_turn.rs#L17); [workspace-service::tests::process_write::parent_transports_four_megabyte_save_without_truncation](../../tests/process_write.rs#L104) |
| [workspace-service::turn::Edit](../../src/turn.rs#L14) | `pub` | not a function |
| [workspace-service::turn::Request](../../src/turn.rs#L26) | `pub` | not a function |
| [workspace-service::turn::execute](../../src/turn.rs#L137) | `pub` | no resolved direct caller |
| [workspace-service::turn::execute_with_roots](../../src/turn.rs#L146) | `pub` | [workspace-service::main::main](../../src/main.rs#L13); [workspace-service::turn::execute](../../src/turn.rs#L137) |
| [workspace-service::write::Request](../../src/write.rs#L16) | `pub` | not a function |
| [workspace-service::write::execute](../../src/write.rs#L31) | `pub` | [workspace-service::main::main](../../src/main.rs#L13) |
