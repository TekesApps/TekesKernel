# endpoint::management

[Package atlas](index.md) · [Source](../../src/management.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::management::VERSION](../../src/management.rs#L26) | const_item | `private` |  |
| [endpoint::management::WorkspaceView](../../src/management.rs#L30) | struct_item | `pub` |  |
| [endpoint::management::WorkspaceFolderView](../../src/management.rs#L42) | struct_item | `pub` |  |
| [endpoint::management::WorkspaceFolders](../../src/management.rs#L50) | struct_item | `pub` |  |
| [endpoint::management::WorkspaceList](../../src/management.rs#L57) | struct_item | `pub` |  |
| [endpoint::management::WorkspaceMetadata](../../src/management.rs#L64) | struct_item | `private` |  |
| [endpoint::management::OperationPhase](../../src/management.rs#L75) | enum_item | `private` |  |
| [endpoint::management::WorkspaceIntent](../../src/management.rs#L85) | struct_item | `private` |  |
| [endpoint::management::FolderMoveIntent](../../src/management.rs#L106) | struct_item | `private` |  |
| [endpoint::management::DiscardIntent](../../src/management.rs#L118) | struct_item | `private` |  |
| [endpoint::management::SessionCreateIntent](../../src/management.rs#L127) | struct_item | `private` |  |
| [endpoint::management::SelectModelIntent](../../src/management.rs#L144) | struct_item | `private` |  |
| [endpoint::management::ForkIntent](../../src/management.rs#L157) | struct_item | `private` |  |
| [endpoint::management::QueueTransactionIntent](../../src/management.rs#L176) | struct_item | `private` |  |
| [endpoint::management::QueueTailSnapshot](../../src/management.rs#L189) | struct_item | `private` |  |
| [endpoint::management::ORIGIN_CLIENT](../../src/management.rs#L197) | const_item | `pub` |  |
| [endpoint::management::LEGACY_ORIGIN_CLIENT](../../src/management.rs#L200) | const_item | `pub` |  |
| [endpoint::management::is_endpoint_origin_client](../../src/management.rs#L203) | function_item | `pub` |  |
| [endpoint::management::OperationRecord](../../src/management.rs#L209) | struct_item | `private` |  |
| [endpoint::management::WorkspaceOperationWrite](../../src/management.rs#L221) | struct_item | `private` |  |
| [endpoint::management::PresetMcpSource](../../src/management.rs#L236) | trait_item | `pub` |  |
| [endpoint::management::PresetMcpSource::server_digest](../../src/management.rs#L239) | function_signature_item | `private` |  |
| [endpoint::management::PresetMcpSource::tool_name](../../src/management.rs#L241) | function_signature_item | `private` |  |
| [endpoint::management::PresetModelSource](../../src/management.rs#L246) | trait_item | `pub` |  |
| [endpoint::management::PresetModelSource::model_available](../../src/management.rs#L249) | function_signature_item | `private` |  |
| [endpoint::management::SessionPresetEnvironment](../../src/management.rs#L261) | struct_item | `pub` |  |
| [endpoint::management::SessionPresetEnvironment::new](../../src/management.rs#L273) | function_item | `pub` |  |
| [endpoint::management::SessionPresetEnvironment::with_models](../../src/management.rs#L294) | function_item | `pub` |  |
| [endpoint::management::SessionPresetEnvironment::permitted_tools](../../src/management.rs#L305) | function_item | `private` |  |
| [endpoint::management::SessionPresetEnvironment::mcp_server_digest](../../src/management.rs#L309) | function_item | `private` |  |
| [endpoint::management::SessionPresetEnvironment::mcp_tool_name](../../src/management.rs#L313) | function_item | `private` |  |
| [endpoint::management::SessionPresetEnvironment::skill_known](../../src/management.rs#L317) | function_item | `private` |  |
| [endpoint::management::SessionPresetEnvironment::model_available](../../src/management.rs#L321) | function_item | `private` |  |
| [endpoint::management::SessionCreateOperation](../../src/management.rs#L327) | struct_item | `pub` |  |
| [endpoint::management::SelectModelOperation](../../src/management.rs#L345) | struct_item | `pub` |  |
| [endpoint::management::ForkSessionOperation](../../src/management.rs#L355) | struct_item | `pub` |  |
| [endpoint::management::ForkLineage](../../src/management.rs#L374) | struct_item | `pub` |  |
| [endpoint::management::QueueTransactionOperation](../../src/management.rs#L379) | struct_item | `pub` |  |
| [endpoint::management::PendingQueueTransaction](../../src/management.rs#L392) | struct_item | `pub` |  |
| [endpoint::management::QueueTransactionDecision](../../src/management.rs#L403) | enum_item | `pub` |  |
| [endpoint::management::QueueTransactionCompletion](../../src/management.rs#L412) | enum_item | `pub` |  |
| [endpoint::management::QueueTransactionState](../../src/management.rs#L423) | enum_item | `pub` |  |
| [endpoint::management::QueueRecoveryDriver](../../src/management.rs#L428) | trait_item | `pub` |  |
| [endpoint::management::QueueRecoveryDriver::execute](../../src/management.rs#L429) | function_signature_item | `private` |  |
| [endpoint::management::SelectedModel](../../src/management.rs#L436) | struct_item | `pub` |  |
| [endpoint::management::CompleteOperations](../../src/management.rs#L445) | struct_item | `pub` |  |
| [endpoint::management::file_identity](../../src/management.rs#L449) | function_item | `private` |  |
| [endpoint::management::ManagementStore](../../src/management.rs#L455) | struct_item | `pub` |  |
| [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) | function_item | `pub(crate)` | test; #[cfg(test)] |
| [endpoint::management::ManagementStore::open_at_with_queue_driver](../../src/management.rs#L473) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::open_recovering_again](../../src/management.rs#L484) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::open_recovering](../../src/management.rs#L498) | function_item | `private` |  |
| [endpoint::management::ManagementStore::open_light](../../src/management.rs#L533) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::open_light_at](../../src/management.rs#L537) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::heal_workspace_metadata_locked](../../src/management.rs#L568) | function_item | `private` |  |
| [endpoint::management::ManagementStore::lock_exclusive](../../src/management.rs#L605) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::root](../../src/management.rs#L612) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::recover](../../src/management.rs#L617) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::recover_with_queue_driver](../../src/management.rs#L621) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::recover_skipping](../../src/management.rs#L628) | function_item | `private` |  |
| [endpoint::management::ManagementStore::recover_record](../../src/management.rs#L653) | function_item | `private` |  |
| [endpoint::management::ManagementStore::quarantine_corrupt_workspace](../../src/management.rs#L795) | function_item | `private` |  |
| [endpoint::management::ManagementStore::quarantine_operation](../../src/management.rs#L812) | function_item | `private` |  |
| [endpoint::management::ManagementStore::seed_missing_workspace_policies](../../src/management.rs#L861) | function_item | `private` |  |
| [endpoint::management::ManagementStore::reconcile_workspace_metadata](../../src/management.rs#L883) | function_item | `private` |  |
| [endpoint::management::ManagementStore::list_workspaces](../../src/management.rs#L888) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::workspace_paths](../../src/management.rs#L950) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::workspace_path](../../src/management.rs#L960) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::create_session](../../src/management.rs#L973) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::select_model](../../src/management.rs#L1111) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::fork_session](../../src/management.rs#L1193) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::apply_preset_defaults](../../src/management.rs#L1289) | function_item | `private` |  |
| [endpoint::management::ManagementStore::resolve_fork_preset](../../src/management.rs#L1319) | function_item | `private` |  |
| [endpoint::management::ManagementStore::prepare_queue_transaction](../../src/management.rs#L1347) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::complete_queue_transaction](../../src/management.rs#L1448) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::has_incomplete_session_operation](../../src/management.rs#L1464) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::completed_fork_lineage](../../src/management.rs#L1481) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::create_workspace](../../src/management.rs#L1502) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::create_workspace_locked](../../src/management.rs#L1523) | function_item | `private` |  |
| [endpoint::management::ManagementStore::guard_folder_entry](../../src/management.rs#L1613) | function_item | `private` |  |
| [endpoint::management::ManagementStore::register_session_cwd](../../src/management.rs#L1629) | function_item | `private` |  |
| [endpoint::management::ManagementStore::rename_workspace](../../src/management.rs#L1658) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::relocate_workspace](../../src/management.rs#L1704) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::workspace_folders](../../src/management.rs#L1780) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::add_workspace_folder](../../src/management.rs#L1795) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::remove_workspace_folder](../../src/management.rs#L1856) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::resume_folder_operation](../../src/management.rs#L1914) | function_item | `private` |  |
| [endpoint::management::ManagementStore::drive_folder_operation](../../src/management.rs#L1938) | function_item | `private` |  |
| [endpoint::management::ManagementStore::workspace_session_bindings](../../src/management.rs#L1948) | function_item | `private` |  |
| [endpoint::management::ManagementStore::archive_session](../../src/management.rs#L2011) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::unarchive_session](../../src/management.rs#L2041) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::discard_session](../../src/management.rs#L2062) | function_item | `pub` |  |
| [endpoint::management::ManagementStore::drive_discard](../../src/management.rs#L2123) | function_item | `private` |  |
| [endpoint::management::ManagementStore::folder_move](../../src/management.rs#L2178) | function_item | `private` |  |
| [endpoint::management::ManagementStore::begin_workspace_operation](../../src/management.rs#L2254) | function_item | `private` |  |
| [endpoint::management::ManagementStore::drive_workspace_operation](../../src/management.rs#L2311) | function_item | `private` |  |
| [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) | function_item | `private` |  |
| [endpoint::management::ManagementStore::operation_created](../../src/management.rs#L2430) | function_item | `private` |  |
| [endpoint::management::ManagementStore::rebuild_candidate](../../src/management.rs#L2464) | function_item | `private` |  |
| [endpoint::management::ManagementStore::drive_session_create](../../src/management.rs#L2589) | function_item | `private` |  |
| [endpoint::management::ManagementStore::session_id_from_response](../../src/management.rs#L2664) | function_item | `private` |  |
| [endpoint::management::ManagementStore::drive_select_model](../../src/management.rs#L2678) | function_item | `private` |  |
| [endpoint::management::ManagementStore::fork_was_quarantined](../../src/management.rs#L2744) | function_item | `private` |  |
| [endpoint::management::ManagementStore::drive_fork](../../src/management.rs#L2773) | function_item | `private` |  |
| [endpoint::management::ManagementStore::drive_queue_completion](../../src/management.rs#L2864) | function_item | `private` |  |
| [endpoint::management::ManagementStore::remove_recordless_payloads](../../src/management.rs#L2923) | function_item | `private` |  |
| [endpoint::management::ManagementStore::remove_recordless_payload](../../src/management.rs#L2950) | function_item | `private` |  |
| [endpoint::management::ManagementStore::drive_folder_move](../../src/management.rs#L2969) | function_item | `private` |  |
| [endpoint::management::ManagementStore::operation_paths](../../src/management.rs#L3039) | function_item | `private` |  |
| [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) | function_item | `private` |  |
| [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) | function_item | `private` |  |
| [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) | function_item | `private` |  |
| [endpoint::management::ManagementStore::read_metadata](../../src/management.rs#L3082) | function_item | `private` |  |
| [endpoint::management::ManagementStore::read_metadata_optional](../../src/management.rs#L3090) | function_item | `private` |  |
| [endpoint::management::primary_path](../../src/management.rs#L3103) | function_item | `private` |  |
| [endpoint::management::binding_for_path](../../src/management.rs#L3112) | function_item | `private` |  |
| [endpoint::management::binding_for_canonical_path](../../src/management.rs#L3118) | function_item | `private` |  |
| [endpoint::management::execution_path_from_snapshot](../../src/management.rs#L3133) | function_item | `private` |  |
| [endpoint::management::folder_binding_matches_snapshot](../../src/management.rs#L3139) | function_item | `private` |  |
| [endpoint::management::resolve_session_workspace](../../src/management.rs#L3143) | function_item | `private` |  |
| [endpoint::management::existing_session_cwd](../../src/management.rs#L3205) | function_item | `private` |  |
| [endpoint::management::session_preset](../../src/management.rs#L3228) | function_item | `private` |  |
| [endpoint::management::genesis_preset_defaults](../../src/management.rs#L3252) | function_item | `private` |  |
| [endpoint::management::create_genesis](../../src/management.rs#L3265) | function_item | `private` |  |
| [endpoint::management::canonical_event_line](../../src/management.rs#L3305) | function_item | `private` |  |
| [endpoint::management::read_canonical_event](../../src/management.rs#L3311) | function_item | `private` |  |
| [endpoint::management::verify_session_create_genesis](../../src/management.rs#L3322) | function_item | `private` |  |
| [endpoint::management::copy_payload_tree](../../src/management.rs#L3373) | function_item | `private` |  |
| [endpoint::management::sync_operation_payload](../../src/management.rs#L3394) | function_item | `private` |  |
| [endpoint::management::is_hex_digest](../../src/management.rs#L3407) | function_item | `private` |  |
| [endpoint::management::default_workspace_policy](../../src/management.rs#L3421) | function_item | `pub` |  |
| [endpoint::management::relocate_policy_roots](../../src/management.rs#L3436) | function_item | `private` |  |
| [endpoint::management::materialized_folders](../../src/management.rs#L3461) | function_item | `private` |  |
| [endpoint::management::next_folder_id](../../src/management.rs#L3477) | function_item | `private` |  |
| [endpoint::management::with_added_folder](../../src/management.rs#L3488) | function_item | `private` |  |
| [endpoint::management::with_removed_folder](../../src/management.rs#L3512) | function_item | `private` |  |
| [endpoint::management::folders_view](../../src/management.rs#L3545) | function_item | `private` |  |
| [endpoint::management::folder_response](../../src/management.rs#L3559) | function_item | `private` |  |
| [endpoint::management::canonical_workspace_path](../../src/management.rs#L3571) | function_item | `private` |  |
| [endpoint::management::unavailable_absolute_path](../../src/management.rs#L3580) | function_item | `private` |  |
| [endpoint::management::allocate_uuid_v7](../../src/management.rs#L3585) | function_item | `private` |  |
| [endpoint::management::current_timestamp](../../src/management.rs#L3604) | function_item | `private` |  |
| [endpoint::management::validate_title](../../src/management.rs#L3613) | function_item | `private` |  |
| [endpoint::management::TitleChoice](../../src/management.rs#L3626) | enum_item | `private` |  |
| [endpoint::management::WorkspaceCreate](../../src/management.rs#L3634) | struct_item | `private` |  |
| [endpoint::management::first_free_title](../../src/management.rs#L3643) | function_item | `private` |  |
| [endpoint::management::ensure_unique_title](../../src/management.rs#L3661) | function_item | `private` |  |
| [endpoint::management::verify_metadata](../../src/management.rs#L3675) | function_item | `private` |  |
| [endpoint::management::workspace_view](../../src/management.rs#L3693) | function_item | `private` |  |
| [endpoint::management::timestamp_from_millis](../../src/management.rs#L3719) | function_item | `private` |  |
| [endpoint::management::publish_canonical](../../src/management.rs#L3743) | function_item | `private` |  |
| [endpoint::management::read_canonical](../../src/management.rs#L3748) | function_item | `private` |  |
| [endpoint::management::decode_canonical](../../src/management.rs#L3752) | function_item | `private` |  |
| [endpoint::management::canonical_line](../../src/management.rs#L3773) | function_item | `private` |  |
| [endpoint::management::to_ijson](../../src/management.rs#L3780) | function_item | `private` |  |
| [endpoint::management::intent_kind](../../src/management.rs#L3788) | function_item | `private` |  |
| [endpoint::management::workspace_intent](../../src/management.rs#L3799) | function_item | `private` |  |
| [endpoint::management::discard_intent](../../src/management.rs#L3831) | function_item | `private` |  |
| [endpoint::management::folder_is_ephemeral](../../src/management.rs#L3843) | function_item | `private` |  |
| [endpoint::management::folder_move_intent](../../src/management.rs#L3849) | function_item | `private` |  |
| [endpoint::management::session_create_intent](../../src/management.rs#L3860) | function_item | `private` |  |
| [endpoint::management::select_model_intent](../../src/management.rs#L3885) | function_item | `private` |  |
| [endpoint::management::fork_intent](../../src/management.rs#L3905) | function_item | `private` |  |
| [endpoint::management::queue_transaction_intent](../../src/management.rs#L3922) | function_item | `private` |  |
| [endpoint::management::pending_queue_transaction](../../src/management.rs#L3950) | function_item | `private` |  |
| [endpoint::management::verify_queue_payload](../../src/management.rs#L3965) | function_item | `private` |  |
| [endpoint::management::completion_for_decision](../../src/management.rs#L4000) | function_item | `private` |  |
| [endpoint::management::queue_completion_value](../../src/management.rs#L4012) | function_item | `private` |  |
| [endpoint::management::queue_completion_from_record](../../src/management.rs#L4050) | function_item | `private` |  |
| [endpoint::management::operation_target_session](../../src/management.rs#L4097) | function_item | `private` |  |
| [endpoint::management::is_uid_principal](../../src/management.rs#L4113) | function_item | `private` |  |
| [endpoint::management::fork_id_from_response](../../src/management.rs#L4119) | function_item | `private` |  |
| [endpoint::management::selected_model_from_response](../../src/management.rs#L4131) | function_item | `private` |  |
| [endpoint::management::active_session_folder](../../src/management.rs#L4169) | function_item | `private` |  |
| [endpoint::management::valid_folder_projection](../../src/management.rs#L4180) | function_item | `private` |  |
| [endpoint::management::folder_reservation](../../src/management.rs#L4193) | function_item | `private` |  |
| [endpoint::management::verify_folder_reservation](../../src/management.rs#L4210) | function_item | `private` |  |
| [endpoint::management::folder_workspace](../../src/management.rs#L4223) | function_item | `private` |  |
| [endpoint::management::active_session_count](../../src/management.rs#L4235) | function_item | `private` |  |
| [endpoint::management::hex_digest](../../src/management.rs#L4245) | function_item | `private` |  |
| [endpoint::management::ManagementError](../../src/management.rs#L4250) | enum_item | `pub` |  |
| [endpoint::management::tests::omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed](../../src/management.rs#L4349) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::preset_genesis_replaces_identity_and_needs_reader_two](../../src/management.rs#L4383) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::preset_skills_resolve_against_package_names](../../src/management.rs#L4470) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::legacy_session_create_without_folder_binding_remains_recoverable](../../src/management.rs#L4504) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::quarantined_fork_does_not_block_recovery_or_claim_success](../../src/management.rs#L4519) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::open_light_heals_metadata_after_any_config_writer_without_reading_threads](../../src/management.rs#L4594) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::unavailable_workspace_keeps_its_management_operation_pending](../../src/management.rs#L4683) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::a_select_model_payload_staged_before_the_rename_is_read_on_recovery](../../src/management.rs#L4722) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder](../../src/management.rs#L4790) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once](../../src/management.rs#L4835) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::relocation_moves_writable_roots_with_their_folder](../../src/management.rs#L4887) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::folder_store](../../src/management.rs#L4923) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::sibling](../../src/management.rs#L4941) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::write_session](../../src/management.rs#L4952) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::folders_are_added_listed_and_removed_with_their_writable_roots](../../src/management.rs#L4994) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::a_folder_bound_to_a_session_is_not_removed](../../src/management.rs#L5110) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::a_legacy_unbound_session_blocks_a_second_folder](../../src/management.rs#L5156) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::folder_edits_materialize_a_legacy_cwd_workspace_with_its_synthesized_ids](../../src/management.rs#L5189) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::completed_workspace_operation_never_reopens_its_old_path](../../src/management.rs#L5221) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::create_in](../../src/management.rs#L5242) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::session_create_registers_an_unknown_cwd_once](../../src/management.rs#L5262) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::session_create_reads_workspace_skills_where_the_worker_launch_does](../../src/management.rs#L5320) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::tests::folders_never_name_the_filesystem_root_or_kernel_storage](../../src/management.rs#L5373) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::BOOT](../../src/management.rs#L5467) | const_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::TICK](../../src/management.rs#L5468) | const_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::LATER](../../src/management.rs#L5469) | const_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::WORKSPACE](../../src/management.rs#L5470) | const_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::fixture](../../src/management.rs#L5472) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::policy_with](../../src/management.rs#L5497) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::set_policy](../../src/management.rs#L5503) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::quarantine_root](../../src/management.rs#L5510) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::stale_metadata](../../src/management.rs#L5516) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::metadata_matches](../../src/management.rs#L5523) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::stuck_carriers_record](../../src/management.rs#L5530) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::tick_heals_stale_metadata_without_staging_an_operation](../../src/management.rs#L5564) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::tick_never_fails_while_policy_set_publishes](../../src/management.rs#L5594) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::parked_synthetic_rename](../../src/management.rs#L5639) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::boot_quarantines_the_parked_carriers_record_and_succeeds](../../src/management.rs#L5688) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::record_of_another_version_is_left_in_place_not_quarantined](../../src/management.rs#L5743) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::quarantine_keeps_the_payload_and_never_redrives_the_operation](../../src/management.rs#L5765) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::complete_workspace_record_without_response_is_quarantined](../../src/management.rs#L5828) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::management::k1_management_race::one_failing_record_does_not_stop_the_others_from_recovering](../../src/management.rs#L5864) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `fs` | `std::fs` | `private` |
| `File` | `std::fs::File` | `private` |
| `Read` | `std::io::Read` | `private` |
| `MetadataExt` | `std::os::unix::fs::MetadataExt` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `SystemTime` | `std::time::SystemTime` | `private` |
| `UNIX_EPOCH` | `std::time::UNIX_EPOCH` | `private` |
| `ConfigRepository` | `profile::ConfigRepository` | `private` |
| `WorkspaceConfig` | `profile::WorkspaceConfig` | `private` |
| `WorkspaceFolder` | `profile::WorkspaceFolder` | `private` |
| `WorkspacePolicy` | `profile::WorkspacePolicy` | `private` |
| `Event` | `schema::Event` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |
| `ResumePolicy` | `schema::ResumePolicy` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `AssetStore` | `store::AssetStore` | `private` |
| `AtomicPublisher` | `store::AtomicPublisher` | `private` |
| `DirectoryLock` | `store::DirectoryLock` | `private` |
| `NamedLock` | `store::NamedLock` | `private` |
| `ThreadStore` | `store::ThreadStore` | `private` |
| `scan_valid_prefix` | `store::scan_valid_prefix` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `Uuid` | `uuid::Uuid` | `private` |
| `SessionInventoryItem` | `crate::SessionInventoryItem` | `private` |
| `ForkIntent` | `super::ForkIntent` | `private` |
| `ManagementError` | `super::ManagementError` | `private` |
| `ManagementStore` | `super::ManagementStore` | `private` |
| `OperationPhase` | `super::OperationPhase` | `private` |
| `OperationRecord` | `super::OperationRecord` | `private` |
| `VERSION` | `super::VERSION` | `private` |
| `WorkspaceMetadata` | `super::WorkspaceMetadata` | `private` |
| `WorkspaceOperationWrite` | `super::WorkspaceOperationWrite` | `private` |
| `WorkspaceView` | `super::WorkspaceView` | `private` |
| `canonical_event_line` | `super::canonical_event_line` | `private` |
| `create_genesis` | `super::create_genesis` | `private` |
| `default_workspace_policy` | `super::default_workspace_policy` | `private` |
| `folder_binding_matches_snapshot` | `super::folder_binding_matches_snapshot` | `private` |
| `publish_canonical` | `super::publish_canonical` | `private` |
| `read_canonical` | `super::read_canonical` | `private` |
| `session_preset` | `super::session_preset` | `private` |
| `to_ijson` | `super::to_ijson` | `private` |
| `SessionInventoryItem` | `crate::SessionInventoryItem` | `private` |
| `ConfigRepository` | `profile::ConfigRepository` | `private` |
| `WorkspaceConfig` | `profile::WorkspaceConfig` | `private` |
| `WorkspaceFolder` | `profile::WorkspaceFolder` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `fs` | `std::fs` | `private` |
| `Path` | `std::path::Path` | `private` |
| `ThreadStore` | `store::ThreadStore` | `private` |
| `*` | `super::*` | `private` |
| `ConfigRepository` | `profile::ConfigRepository` | `private` |
| `WorkspaceConfig` | `profile::WorkspaceConfig` | `private` |
| `WorkspaceFolder` | `profile::WorkspaceFolder` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `AtomicBool` | `std::sync::atomic::AtomicBool` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `endpoint::management::tests` | `private` | #[cfg(test)] |
| `endpoint::management::k1_management_race` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 28 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::is_endpoint_origin_client"]
  n1["endpoint::management::ManagementStore::workspace_configs"]
  n2["endpoint::management::SessionPresetEnvironment::new"]
  n3["endpoint::management::ManagementStore::remove_recordless_payloads"]
  n4["endpoint::management::SessionPresetEnvironment::with_models"]
  n5["endpoint::management::ManagementStore::operation_paths"]
  n6["endpoint::management::SessionPresetEnvironment::permitted_tools"]
  n7["endpoint::management::ManagementStore::metadata_path"]
  n8["endpoint::management::ManagementStore::read_metadata_optional"]
  n9["endpoint::management::SessionPresetEnvironment::mcp_server_digest"]
  n10["endpoint::management::primary_path"]
  n11["endpoint::management::SessionPresetEnvironment::mcp_tool_name"]
  n12["endpoint::management::SessionPresetEnvironment::skill_known"]
  n13["endpoint::management::SessionPresetEnvironment::model_available"]
  n14["endpoint::management::current_timestamp"]
  n15["endpoint::management::verify_metadata"]
  n16["endpoint::management::publish_canonical"]
  n17["endpoint::management::canonical_line"]
  n18["endpoint::management::hex_digest"]
  n19["endpoint::management::file_identity"]
  n20["endpoint::management::ManagementStore::open_at_with_queue_driver"]
  n21["endpoint::management::ManagementStore::open_recovering_again"]
  n22["endpoint::management::ManagementStore::open_recovering"]
  n23["endpoint::management::ManagementStore::open_light"]
  n24["endpoint::management::ManagementStore::open_light_at"]
  n25["endpoint::management::ManagementStore::heal_workspace_metadata_locked"]
  n26["endpoint::management::ManagementStore::lock_exclusive"]
  n27["endpoint::management::ManagementStore::root"]
  n28["endpoint::management::ManagementStore::recover"]
  n29["endpoint::management::ManagementStore::recover_with_queue_driver"]
  n30["endpoint::management::ManagementStore::recover_skipping"]
  n31["endpoint::management::ManagementStore::recover_record"]
  n32["profile::config::ConfigRepository::open"]
  n33["profile::resources::ResourceCatalog::skill_package_names"]
  n34["store::folder::ThreadStore::open"]
  n35["store::management_root::endpoint_management_root"]
  n36["store::platform::NamedLock::exclusive"]
  n37["store::platform::NamedLock::shared"]
  n2 --> n33
  n20 --> n22
  n21 --> n22
  n22 --> n32
  n22 --> n34
  n22 --> n35
  n23 --> n14
  n23 --> n24
  n24 --> n32
  n24 --> n35
  n24 --> n36
  n25 --> n1
  n25 --> n7
  n25 --> n8
  n25 --> n10
  n25 --> n15
  n25 --> n16
  n25 --> n17
  n25 --> n18
  n25 --> n37
  n26 --> n35
  n26 --> n36
  n28 --> n29
  n29 --> n30
  n30 --> n3
  n30 --> n5
  n30 --> n31
  n30 --> n36
```

</details>

<details><summary>Functions 21–40: 151 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::journal::EndpointJournal::open"]
  n1["endpoint::management::ManagementStore::select_model"]
  n2["endpoint::management::ManagementStore::fork_session"]
  n3["endpoint::management::ManagementStore::apply_preset_defaults"]
  n4["endpoint::management::ManagementStore::resolve_fork_preset"]
  n5["endpoint::management::ManagementStore::prepare_queue_transaction"]
  n6["endpoint::management::ManagementStore::complete_queue_transaction"]
  n7["endpoint::management::ManagementStore::has_incomplete_session_operation"]
  n8["endpoint::management::ManagementStore::completed_fork_lineage"]
  n9["endpoint::management::ManagementStore::create_workspace"]
  n10["endpoint::management::ManagementStore::create_workspace_locked"]
  n11["endpoint::management::ManagementStore::guard_folder_entry"]
  n12["endpoint::management::ManagementStore::register_session_cwd"]
  n13["endpoint::management::ManagementStore::begin_workspace_operation"]
  n14["endpoint::management::ManagementStore::drive_workspace_operation"]
  n15["endpoint::management::ManagementStore::workspace_configs"]
  n16["endpoint::management::ManagementStore::drive_session_create"]
  n17["endpoint::management::ManagementStore::session_id_from_response"]
  n18["endpoint::management::ManagementStore::drive_select_model"]
  n19["endpoint::management::SessionPresetEnvironment::new"]
  n20["endpoint::management::ManagementStore::fork_was_quarantined"]
  n21["endpoint::management::ManagementStore::drive_fork"]
  n22["endpoint::management::ManagementStore::drive_queue_completion"]
  n23["endpoint::management::ManagementStore::remove_recordless_payload"]
  n24["endpoint::management::ManagementStore::drive_folder_move"]
  n25["endpoint::management::ManagementStore::operation_paths"]
  n26["endpoint::management::ManagementStore::operation_path"]
  n27["endpoint::management::ManagementStore::operation_payload_path"]
  n28["endpoint::management::ManagementStore::read_metadata"]
  n29["endpoint::management::ManagementStore::read_metadata_optional"]
  n30["endpoint::management::primary_path"]
  n31["endpoint::management::resolve_session_workspace"]
  n32["endpoint::management::existing_session_cwd"]
  n33["endpoint::management::session_preset"]
  n34["endpoint::management::create_genesis"]
  n35["endpoint::management::canonical_event_line"]
  n36["endpoint::management::copy_payload_tree"]
  n37["endpoint::management::sync_operation_payload"]
  n38["endpoint::management::is_hex_digest"]
  n39["endpoint::management::default_workspace_policy"]
  n40["endpoint::management::canonical_workspace_path"]
  n41["endpoint::management::unavailable_absolute_path"]
  n42["endpoint::management::allocate_uuid_v7"]
  n43["endpoint::management::current_timestamp"]
  n44["endpoint::management::validate_title"]
  n45["endpoint::management::first_free_title"]
  n46["endpoint::management::ensure_unique_title"]
  n47["endpoint::management::verify_metadata"]
  n48["endpoint::management::workspace_view"]
  n49["endpoint::management::publish_canonical"]
  n50["endpoint::management::read_canonical"]
  n51["endpoint::management::canonical_line"]
  n52["endpoint::management::to_ijson"]
  n53["endpoint::management::intent_kind"]
  n54["endpoint::management::workspace_intent"]
  n55["endpoint::management::folder_move_intent"]
  n56["endpoint::management::session_create_intent"]
  n57["endpoint::management::select_model_intent"]
  n58["endpoint::management::fork_intent"]
  n59["endpoint::management::queue_transaction_intent"]
  n60["endpoint::management::pending_queue_transaction"]
  n61["endpoint::management::verify_queue_payload"]
  n62["endpoint::management::queue_completion_from_record"]
  n63["endpoint::management::operation_target_session"]
  n64["endpoint::management::fork_id_from_response"]
  n65["endpoint::management::selected_model_from_response"]
  n66["endpoint::management::active_session_folder"]
  n67["endpoint::management::valid_folder_projection"]
  n68["endpoint::management::active_session_count"]
  n69["endpoint::management::hex_digest"]
  n70["endpoint::management::file_identity"]
  n71["endpoint::management::ManagementStore::heal_workspace_metadata_locked"]
  n72["endpoint::management::ManagementStore::recover_record"]
  n73["endpoint::management::ManagementStore::quarantine_corrupt_workspace"]
  n74["endpoint::management::ManagementStore::quarantine_operation"]
  n75["endpoint::management::ManagementStore::seed_missing_workspace_policies"]
  n76["endpoint::management::ManagementStore::reconcile_workspace_metadata"]
  n77["endpoint::management::ManagementStore::list_workspaces"]
  n78["endpoint::management::ManagementStore::workspace_paths"]
  n79["endpoint::management::ManagementStore::workspace_path"]
  n80["endpoint::management::ManagementStore::create_session"]
  n81["endpoint::service::NativeEndpoint::open"]
  n82["endpoint::types::validate_session_id"]
  n83["profile::scope::workspace_instructions"]
  n84["profile::scope::SessionScope::for_session_folder"]
  n85["store::asset::AssetStore::new"]
  n86["store::atomic::AtomicPublisher::replace"]
  n87["store::folder::ThreadStore::open"]
  n88["store::platform::NamedLock::exclusive"]
  n89["store::platform::NamedLock::shared"]
  n90["store::tail::scan_valid_prefix"]
  n91["tools::sandbox::kernel_protected_roots"]
  n92["tools::sandbox::protected_root_containing"]
  n1 --> n18
  n1 --> n23
  n1 --> n26
  n1 --> n27
  n1 --> n37
  n1 --> n49
  n1 --> n50
  n1 --> n52
  n1 --> n65
  n1 --> n66
  n1 --> n67
  n1 --> n82
  n1 --> n86
  n1 --> n87
  n1 --> n88
  n2 --> n0
  n2 --> n4
  n2 --> n21
  n2 --> n26
  n2 --> n42
  n2 --> n49
  n2 --> n50
  n2 --> n52
  n2 --> n64
  n2 --> n66
  n2 --> n68
  n2 --> n81
  n2 --> n82
  n2 --> n88
  n4 --> n19
  n4 --> n33
  n4 --> n84
  n5 --> n23
  n5 --> n26
  n5 --> n27
  n5 --> n37
  n5 --> n49
  n5 --> n50
  n5 --> n51
  n5 --> n52
  n5 --> n59
  n5 --> n60
  n5 --> n61
  n5 --> n62
  n5 --> n66
  n5 --> n82
  n5 --> n86
  n5 --> n88
  n5 --> n90
  n6 --> n22
  n6 --> n26
  n6 --> n50
  n6 --> n88
  n7 --> n25
  n7 --> n50
  n7 --> n63
  n7 --> n82
  n7 --> n89
  n8 --> n25
  n8 --> n50
  n8 --> n53
  n8 --> n58
  n8 --> n89
  n9 --> n10
  n9 --> n40
  n9 --> n88
  n10 --> n11
  n10 --> n13
  n10 --> n14
  n10 --> n15
  n10 --> n30
  n10 --> n39
  n10 --> n42
  n10 --> n44
  n10 --> n45
  n10 --> n46
  n11 --> n91
  n11 --> n92
  n72 --> n14
  n72 --> n16
  n72 --> n18
  n72 --> n20
  n72 --> n21
  n72 --> n22
  n72 --> n24
  n72 --> n27
  n72 --> n41
  n72 --> n50
  n72 --> n53
  n72 --> n54
  n72 --> n55
  n72 --> n56
  n72 --> n57
  n72 --> n58
  n72 --> n59
  n72 --> n60
  n72 --> n61
  n72 --> n62
  n72 --> n70
  n72 --> n73
  n72 --> n88
  n73 --> n74
  n74 --> n36
  n74 --> n38
  n74 --> n86
  n75 --> n15
  n75 --> n30
  n75 --> n39
  n75 --> n88
  n76 --> n71
  n76 --> n88
  n77 --> n15
  n77 --> n28
  n77 --> n29
  n77 --> n30
  n77 --> n43
  n77 --> n47
  n77 --> n48
  n77 --> n71
  n77 --> n88
  n78 --> n15
  n78 --> n30
  n78 --> n89
  n79 --> n15
  n79 --> n30
  n79 --> n88
  n80 --> n12
  n80 --> n15
  n80 --> n16
  n80 --> n17
  n80 --> n19
  n80 --> n23
  n80 --> n26
  n80 --> n27
  n80 --> n31
  n80 --> n32
  n80 --> n33
  n80 --> n34
  n80 --> n35
  n80 --> n37
  n80 --> n42
  n80 --> n49
  n80 --> n50
  n80 --> n52
  n80 --> n68
  n80 --> n69
  n80 --> n82
  n80 --> n83
  n80 --> n85
  n80 --> n86
  n80 --> n88
```

</details>

<details><summary>Functions 41–60: 130 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::ManagementStore::apply_preset_defaults"]
  n1["endpoint::management::ManagementStore::create_workspace_locked"]
  n2["endpoint::management::ManagementStore::guard_folder_entry"]
  n3["endpoint::management::ManagementStore::register_session_cwd"]
  n4["endpoint::management::ManagementStore::rename_workspace"]
  n5["endpoint::management::ManagementStore::relocate_workspace"]
  n6["endpoint::management::ManagementStore::workspace_folders"]
  n7["endpoint::management::ManagementStore::add_workspace_folder"]
  n8["endpoint::management::ManagementStore::remove_workspace_folder"]
  n9["endpoint::management::ManagementStore::resume_folder_operation"]
  n10["endpoint::management::ManagementStore::drive_folder_operation"]
  n11["endpoint::management::ManagementStore::workspace_session_bindings"]
  n12["endpoint::management::ManagementStore::archive_session"]
  n13["endpoint::management::ManagementStore::unarchive_session"]
  n14["endpoint::management::ManagementStore::discard_session"]
  n15["endpoint::management::ManagementStore::drive_discard"]
  n16["endpoint::management::ManagementStore::folder_move"]
  n17["endpoint::management::ManagementStore::begin_workspace_operation"]
  n18["endpoint::management::ManagementStore::drive_workspace_operation"]
  n19["endpoint::management::ManagementStore::workspace_configs"]
  n20["endpoint::management::ManagementStore::operation_created"]
  n21["endpoint::management::ManagementStore::rebuild_candidate"]
  n22["endpoint::management::ManagementStore::drive_session_create"]
  n23["endpoint::management::ManagementStore::remove_recordless_payload"]
  n24["endpoint::management::ManagementStore::drive_folder_move"]
  n25["endpoint::management::ManagementStore::operation_paths"]
  n26["endpoint::management::ManagementStore::operation_path"]
  n27["endpoint::management::ManagementStore::operation_payload_path"]
  n28["endpoint::management::ManagementStore::metadata_path"]
  n29["endpoint::management::ManagementStore::read_metadata"]
  n30["endpoint::management::ManagementStore::read_metadata_optional"]
  n31["endpoint::management::primary_path"]
  n32["endpoint::management::binding_for_canonical_path"]
  n33["endpoint::management::execution_path_from_snapshot"]
  n34["endpoint::management::folder_binding_matches_snapshot"]
  n35["endpoint::management::genesis_preset_defaults"]
  n36["endpoint::management::read_canonical_event"]
  n37["endpoint::management::verify_session_create_genesis"]
  n38["endpoint::management::sync_operation_payload"]
  n39["endpoint::management::relocate_policy_roots"]
  n40["endpoint::management::materialized_folders"]
  n41["endpoint::management::next_folder_id"]
  n42["endpoint::management::with_added_folder"]
  n43["endpoint::management::with_removed_folder"]
  n44["endpoint::management::folders_view"]
  n45["endpoint::management::folder_response"]
  n46["endpoint::management::canonical_workspace_path"]
  n47["endpoint::management::validate_title"]
  n48["endpoint::management::ensure_unique_title"]
  n49["endpoint::management::workspace_view"]
  n50["endpoint::management::publish_canonical"]
  n51["endpoint::management::read_canonical"]
  n52["endpoint::management::decode_canonical"]
  n53["endpoint::management::canonical_line"]
  n54["endpoint::management::to_ijson"]
  n55["endpoint::management::workspace_intent"]
  n56["endpoint::management::discard_intent"]
  n57["endpoint::management::folder_is_ephemeral"]
  n58["endpoint::management::session_create_intent"]
  n59["endpoint::management::fork_id_from_response"]
  n60["endpoint::management::folder_reservation"]
  n61["endpoint::management::active_session_count"]
  n62["endpoint::management::hex_digest"]
  n63["endpoint::service::NativeEndpoint::open"]
  n64["endpoint::types::validate_session_id"]
  n65["profile::config::ConfigSnapshot::decode"]
  n66["profile::instruction::InstructionSnapshot::decode"]
  n67["schema::ijson::IJsonValue::parse"]
  n68["store::asset::AssetStore::new"]
  n69["store::atomic::AtomicPublisher::replace"]
  n70["store::folder::ThreadStore::open"]
  n71["store::platform::NamedLock::exclusive"]
  n72["store::platform::DirectoryLock::try_exclusive"]
  n73["store::tail::scan_valid_prefix"]
  n3 --> n1
  n3 --> n32
  n3 --> n46
  n4 --> n17
  n4 --> n18
  n4 --> n19
  n4 --> n47
  n4 --> n48
  n4 --> n71
  n5 --> n2
  n5 --> n17
  n5 --> n18
  n5 --> n19
  n5 --> n39
  n5 --> n46
  n5 --> n71
  n6 --> n19
  n6 --> n44
  n7 --> n2
  n7 --> n9
  n7 --> n10
  n7 --> n11
  n7 --> n19
  n7 --> n40
  n7 --> n41
  n7 --> n42
  n7 --> n45
  n7 --> n46
  n7 --> n71
  n8 --> n9
  n8 --> n10
  n8 --> n11
  n8 --> n19
  n8 --> n40
  n8 --> n43
  n8 --> n45
  n8 --> n71
  n9 --> n18
  n9 --> n26
  n9 --> n51
  n10 --> n17
  n10 --> n18
  n10 --> n45
  n11 --> n25
  n11 --> n51
  n11 --> n58
  n11 --> n64
  n11 --> n73
  n12 --> n16
  n13 --> n16
  n14 --> n15
  n14 --> n26
  n14 --> n50
  n14 --> n51
  n14 --> n54
  n14 --> n57
  n14 --> n60
  n14 --> n64
  n14 --> n70
  n14 --> n71
  n14 --> n72
  n15 --> n50
  n15 --> n54
  n15 --> n56
  n15 --> n59
  n15 --> n60
  n15 --> n70
  n16 --> n24
  n16 --> n26
  n16 --> n50
  n16 --> n51
  n16 --> n54
  n16 --> n57
  n16 --> n60
  n16 --> n61
  n16 --> n64
  n16 --> n71
  n16 --> n72
  n17 --> n23
  n17 --> n26
  n17 --> n27
  n17 --> n31
  n17 --> n38
  n17 --> n50
  n17 --> n51
  n17 --> n53
  n17 --> n54
  n17 --> n62
  n17 --> n69
  n18 --> n19
  n18 --> n20
  n18 --> n21
  n18 --> n27
  n18 --> n28
  n18 --> n29
  n18 --> n30
  n18 --> n31
  n18 --> n49
  n18 --> n50
  n18 --> n53
  n18 --> n55
  n18 --> n62
  n18 --> n63
  n18 --> n67
  n18 --> n69
  n20 --> n19
  n20 --> n27
  n20 --> n50
  n20 --> n52
  n21 --> n19
  n21 --> n42
  n21 --> n43
  n21 --> n53
  n21 --> n62
  n22 --> n0
  n22 --> n27
  n22 --> n28
  n22 --> n29
  n22 --> n33
  n22 --> n34
  n22 --> n35
  n22 --> n36
  n22 --> n37
  n22 --> n50
  n22 --> n54
  n22 --> n58
  n22 --> n65
  n22 --> n66
  n22 --> n68
  n22 --> n70
```

</details>

<details><summary>Functions 61–80: 50 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::ManagementStore::apply_preset_defaults"]
  n1["endpoint::management::ManagementStore::session_id_from_response"]
  n2["endpoint::management::ManagementStore::drive_select_model"]
  n3["endpoint::management::ManagementStore::fork_was_quarantined"]
  n4["endpoint::management::ManagementStore::drive_fork"]
  n5["endpoint::management::ManagementStore::drive_queue_completion"]
  n6["endpoint::management::ManagementStore::remove_recordless_payloads"]
  n7["endpoint::management::ManagementStore::remove_recordless_payload"]
  n8["endpoint::management::ManagementStore::drive_folder_move"]
  n9["endpoint::management::ManagementStore::operation_paths"]
  n10["endpoint::management::ManagementStore::operation_path"]
  n11["endpoint::management::ManagementStore::operation_payload_path"]
  n12["endpoint::management::ManagementStore::metadata_path"]
  n13["endpoint::management::ManagementStore::read_metadata"]
  n14["endpoint::management::ManagementStore::read_metadata_optional"]
  n15["endpoint::management::primary_path"]
  n16["endpoint::management::binding_for_path"]
  n17["endpoint::management::binding_for_canonical_path"]
  n18["endpoint::management::execution_path_from_snapshot"]
  n19["endpoint::management::folder_binding_matches_snapshot"]
  n20["endpoint::management::resolve_session_workspace"]
  n21["endpoint::management::is_hex_digest"]
  n22["endpoint::management::canonical_workspace_path"]
  n23["endpoint::management::publish_canonical"]
  n24["endpoint::management::decode_canonical"]
  n25["endpoint::management::to_ijson"]
  n26["endpoint::management::folder_move_intent"]
  n27["endpoint::management::select_model_intent"]
  n28["endpoint::management::fork_intent"]
  n29["endpoint::management::queue_transaction_intent"]
  n30["endpoint::management::verify_queue_payload"]
  n31["endpoint::management::completion_for_decision"]
  n32["endpoint::management::queue_completion_value"]
  n33["endpoint::management::queue_completion_from_record"]
  n34["endpoint::management::active_session_folder"]
  n35["endpoint::management::verify_folder_reservation"]
  n36["endpoint::management::folder_workspace"]
  n37["endpoint::management::hex_digest"]
  n38["schema::ijson::IJsonValue::parse"]
  n39["store::folder::ThreadStore::open"]
  n40["store::folder::move_thread_folder"]
  n41["store::management_root::session_settings_path"]
  n42["store::platform::DirectoryLock::try_exclusive"]
  n43["store::rewrite::RewriteOperation::decode_canonical"]
  n2 --> n11
  n2 --> n12
  n2 --> n13
  n2 --> n23
  n2 --> n25
  n2 --> n27
  n2 --> n34
  n2 --> n36
  n2 --> n41
  n3 --> n28
  n3 --> n43
  n4 --> n0
  n4 --> n3
  n4 --> n12
  n4 --> n13
  n4 --> n23
  n4 --> n25
  n4 --> n28
  n4 --> n36
  n4 --> n39
  n5 --> n11
  n5 --> n23
  n5 --> n29
  n5 --> n30
  n5 --> n31
  n5 --> n32
  n5 --> n33
  n6 --> n21
  n7 --> n10
  n7 --> n11
  n8 --> n12
  n8 --> n14
  n8 --> n23
  n8 --> n26
  n8 --> n35
  n8 --> n36
  n8 --> n38
  n8 --> n40
  n8 --> n42
  n10 --> n37
  n11 --> n37
  n13 --> n14
  n14 --> n12
  n14 --> n24
  n17 --> n16
  n17 --> n22
  n20 --> n15
  n20 --> n16
  n20 --> n17
  n20 --> n22
```

</details>

<details><summary>Functions 81–100: 13 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::is_endpoint_origin_client"]
  n1["endpoint::management::existing_session_cwd"]
  n2["endpoint::management::session_preset"]
  n3["endpoint::management::genesis_preset_defaults"]
  n4["endpoint::management::create_genesis"]
  n5["endpoint::management::canonical_event_line"]
  n6["endpoint::management::read_canonical_event"]
  n7["endpoint::management::verify_session_create_genesis"]
  n8["endpoint::management::copy_payload_tree"]
  n9["endpoint::management::sync_operation_payload"]
  n10["endpoint::management::is_hex_digest"]
  n11["endpoint::management::default_workspace_policy"]
  n12["endpoint::management::relocate_policy_roots"]
  n13["endpoint::management::materialized_folders"]
  n14["endpoint::management::next_folder_id"]
  n15["endpoint::management::with_added_folder"]
  n16["endpoint::management::with_removed_folder"]
  n17["endpoint::management::folders_view"]
  n18["endpoint::management::folder_response"]
  n19["endpoint::management::canonical_workspace_path"]
  n20["endpoint::management::unavailable_absolute_path"]
  n21["endpoint::management::folder_workspace"]
  n22["schema::event::Event::decode_canonical"]
  n23["schema::event::Event::from_value"]
  n24["schema::ijson::IJsonValue::parse"]
  n25["tools::builtin::BuiltinManifest::compiled"]
  n26["tools::preset::ResolvedPreset::resolve_request"]
  n27["tools::preset::ResolvedPreset::from_identity"]
  n28["tools::preset::ResolvedPreset::from_genesis"]
  n1 --> n21
  n2 --> n26
  n2 --> n27
  n3 --> n28
  n4 --> n23
  n4 --> n24
  n6 --> n22
  n7 --> n0
  n8 --> n8
  n11 --> n25
  n15 --> n13
  n16 --> n13
  n17 --> n13
```

</details>

<details><summary>Functions 101–120: 17 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::primary_path"]
  n1["endpoint::management::is_hex_digest"]
  n2["endpoint::management::allocate_uuid_v7"]
  n3["endpoint::management::current_timestamp"]
  n4["endpoint::management::validate_title"]
  n5["endpoint::management::first_free_title"]
  n6["endpoint::management::ensure_unique_title"]
  n7["endpoint::management::verify_metadata"]
  n8["endpoint::management::workspace_view"]
  n9["endpoint::management::timestamp_from_millis"]
  n10["endpoint::management::publish_canonical"]
  n11["endpoint::management::read_canonical"]
  n12["endpoint::management::decode_canonical"]
  n13["endpoint::management::canonical_line"]
  n14["endpoint::management::to_ijson"]
  n15["endpoint::management::intent_kind"]
  n16["endpoint::management::workspace_intent"]
  n17["endpoint::management::discard_intent"]
  n18["endpoint::management::folder_is_ephemeral"]
  n19["endpoint::management::folder_move_intent"]
  n20["endpoint::management::session_create_intent"]
  n21["endpoint::management::select_model_intent"]
  n22["endpoint::management::hex_digest"]
  n23["endpoint::types::validate_session_id"]
  n24["schema::ijson::IJsonValue::parse"]
  n25["store::atomic::AtomicPublisher::replace"]
  n26["store::tail::read_genesis"]
  n3 --> n9
  n5 --> n4
  n7 --> n0
  n7 --> n13
  n7 --> n22
  n8 --> n0
  n8 --> n9
  n10 --> n13
  n10 --> n25
  n11 --> n12
  n12 --> n24
  n14 --> n24
  n17 --> n23
  n18 --> n26
  n20 --> n1
  n20 --> n23
  n21 --> n23
```

</details>

<details><summary>Functions 121–138: 18 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::read_canonical"]
  n1["endpoint::management::to_ijson"]
  n2["endpoint::management::intent_kind"]
  n3["endpoint::management::folder_move_intent"]
  n4["endpoint::management::session_create_intent"]
  n5["endpoint::management::select_model_intent"]
  n6["endpoint::management::fork_intent"]
  n7["endpoint::management::queue_transaction_intent"]
  n8["endpoint::management::pending_queue_transaction"]
  n9["endpoint::management::verify_queue_payload"]
  n10["endpoint::management::completion_for_decision"]
  n11["endpoint::management::queue_completion_value"]
  n12["endpoint::management::queue_completion_from_record"]
  n13["endpoint::management::operation_target_session"]
  n14["endpoint::management::is_uid_principal"]
  n15["endpoint::management::fork_id_from_response"]
  n16["endpoint::management::selected_model_from_response"]
  n17["endpoint::management::active_session_folder"]
  n18["endpoint::management::valid_folder_projection"]
  n19["endpoint::management::folder_reservation"]
  n20["endpoint::management::verify_folder_reservation"]
  n21["endpoint::management::folder_workspace"]
  n22["endpoint::management::active_session_count"]
  n23["endpoint::management::hex_digest"]
  n24["endpoint::types::validate_session_id"]
  n25["store::tail::read_ledger_repaired"]
  n26["store::tail::read_genesis"]
  n6 --> n14
  n6 --> n24
  n7 --> n24
  n8 --> n7
  n9 --> n0
  n11 --> n1
  n12 --> n7
  n12 --> n11
  n13 --> n2
  n13 --> n3
  n13 --> n4
  n13 --> n5
  n13 --> n6
  n13 --> n7
  n18 --> n25
  n19 --> n25
  n20 --> n19
  n21 --> n26
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `new` | `instruction                 .effective                 .policy                 .allowed_tools                 .as_ref()                 .map` | [280](../../src/management.rs#L280) | receiver-type-required |
| `new` | `instruction                 .effective                 .policy                 .allowed_tools                 .as_ref` | [280](../../src/management.rs#L280) | receiver-type-required |
| `new` | `tools.iter().cloned().collect` | [285](../../src/management.rs#L285) | receiver-type-required |
| `new` | `tools.iter().cloned` | [285](../../src/management.rs#L285) | receiver-type-required |
| `new` | `tools.iter` | [285](../../src/management.rs#L285) | receiver-type-required |
| `new` | `profile::ResourceCatalog::skill_package_names` | [286](../../src/management.rs#L286) | [profile::resources::ResourceCatalog::skill_package_names](../../../profile/src/resources.rs#L109) |
| `with_models` | `models.map` | [299](../../src/management.rs#L299) | receiver-type-required |
| `permitted_tools` | `self.permitted.as_ref` | [306](../../src/management.rs#L306) | receiver-type-required |
| `mcp_server_digest` | `self.mcp?.server_digest` | [310](../../src/management.rs#L310) | receiver-type-required |
| `mcp_tool_name` | `self.mcp?.tool_name` | [314](../../src/management.rs#L314) | receiver-type-required |
| `skill_known` | `self.skills.contains` | [318](../../src/management.rs#L318) | receiver-type-required |
| `model_available` | `self.models             .is_some_and` | [322](../../src/management.rs#L322) | receiver-type-required |
| `model_available` | `models.model_available` | [323](../../src/management.rs#L323) | receiver-type-required |
| `file_identity` | `fs::metadata(path).ok` | [450](../../src/management.rs#L450) | receiver-type-required |
| `file_identity` | `fs::metadata` | [450](../../src/management.rs#L450) | external-constructor-callback-or-unresolved |
| `file_identity` | `Some` | [451](../../src/management.rs#L451) | external-constructor-callback-or-unresolved |
| `file_identity` | `metadata.len` | [451](../../src/management.rs#L451) | receiver-type-required |
| `file_identity` | `metadata.modified().ok` | [451](../../src/management.rs#L451) | receiver-type-required |
| `file_identity` | `metadata.modified` | [451](../../src/management.rs#L451) | receiver-type-required |
| `open_at` | `Self::open_at_with_queue_driver` | [470](../../src/management.rs#L470) | [endpoint::management::ManagementStore::open_at_with_queue_driver](../../src/management.rs#L473) |
| `open_at_with_queue_driver` | `Self::open_recovering` | [478](../../src/management.rs#L478) | [endpoint::management::ManagementStore::open_recovering](../../src/management.rs#L498) |
| `open_recovering_again` | `Self::open_recovering` | [490](../../src/management.rs#L490) | [endpoint::management::ManagementStore::open_recovering](../../src/management.rs#L498) |
| `open_recovering_again` | `Some` | [494](../../src/management.rs#L494) | external-constructor-callback-or-unresolved |
| `open_recovering` | `storage_root.as_ref().to_path_buf` | [504](../../src/management.rs#L504) | receiver-type-required |
| `open_recovering` | `storage_root.as_ref` | [504](../../src/management.rs#L504) | receiver-type-required |
| `open_recovering` | `ThreadStore::open` | [505](../../src/management.rs#L505) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `open_recovering` | `store::endpoint_management_root` | [506](../../src/management.rs#L506) | [store::management_root::endpoint_management_root](../../../store/src/management_root.rs#L47) |
| `open_recovering` | `fs::create_dir_all` | [507](../../src/management.rs#L507), [508](../../src/management.rs#L508) | external-constructor-callback-or-unresolved |
| `open_recovering` | `root.join` | [507](../../src/management.rs#L507), [508](../../src/management.rs#L508), [512](../../src/management.rs#L512) | receiver-type-required |
| `open_recovering` | `fs::OpenOptions::new()             .create(true)             .append(true)             .open` | [509](../../src/management.rs#L509) | receiver-type-required |
| `open_recovering` | `fs::OpenOptions::new()             .create(true)             .append` | [509](../../src/management.rs#L509) | receiver-type-required |
| `open_recovering` | `fs::OpenOptions::new()             .create` | [509](../../src/management.rs#L509) | receiver-type-required |
| `open_recovering` | `fs::OpenOptions::new` | [509](../../src/management.rs#L509) | external-constructor-callback-or-unresolved |
| `open_recovering` | `ConfigRepository::open` | [514](../../src/management.rs#L514) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `open_recovering` | `store.recover_skipping` | [518](../../src/management.rs#L518) | receiver-type-required |
| `open_recovering` | `store.seed_missing_workspace_policies` | [519](../../src/management.rs#L519) | receiver-type-required |
| `open_recovering` | `store.reconcile_workspace_metadata` | [520](../../src/management.rs#L520) | receiver-type-required |
| `open_recovering` | `Ok` | [524](../../src/management.rs#L524) | external-constructor-callback-or-unresolved |
| `open_light` | `Self::open_light_at` | [534](../../src/management.rs#L534) | [endpoint::management::ManagementStore::open_light_at](../../src/management.rs#L537) |
| `open_light` | `current_timestamp` | [534](../../src/management.rs#L534) | [endpoint::management::current_timestamp](../../src/management.rs#L3604) |
| `open_light_at` | `storage_root.as_ref().to_path_buf` | [541](../../src/management.rs#L541) | receiver-type-required |
| `open_light_at` | `storage_root.as_ref` | [541](../../src/management.rs#L541) | receiver-type-required |
| `open_light_at` | `store::endpoint_management_root` | [542](../../src/management.rs#L542) | [store::management_root::endpoint_management_root](../../../store/src/management_root.rs#L47) |
| `open_light_at` | `fs::create_dir_all` | [543](../../src/management.rs#L543), [544](../../src/management.rs#L544) | external-constructor-callback-or-unresolved |
| `open_light_at` | `root.join` | [543](../../src/management.rs#L543), [544](../../src/management.rs#L544), [548](../../src/management.rs#L548) | receiver-type-required |
| `open_light_at` | `fs::OpenOptions::new()             .create(true)             .append(true)             .open` | [545](../../src/management.rs#L545) | receiver-type-required |
| `open_light_at` | `fs::OpenOptions::new()             .create(true)             .append` | [545](../../src/management.rs#L545) | receiver-type-required |
| `open_light_at` | `fs::OpenOptions::new()             .create` | [545](../../src/management.rs#L545) | receiver-type-required |
| `open_light_at` | `fs::OpenOptions::new` | [545](../../src/management.rs#L545) | external-constructor-callback-or-unresolved |
| `open_light_at` | `ConfigRepository::open` | [550](../../src/management.rs#L550) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `open_light_at` | `NamedLock::exclusive` | [554](../../src/management.rs#L554) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `open_light_at` | `store.root.join` | [554](../../src/management.rs#L554) | receiver-type-required |
| `open_light_at` | `store.heal_workspace_metadata_locked` | [555](../../src/management.rs#L555) | receiver-type-required |
| `open_light_at` | `drop` | [556](../../src/management.rs#L556) | external-constructor-callback-or-unresolved |
| `open_light_at` | `Ok` | [557](../../src/management.rs#L557) | external-constructor-callback-or-unresolved |
| `heal_workspace_metadata_locked` | `NamedLock::shared` | [569](../../src/management.rs#L569) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `heal_workspace_metadata_locked` | `self.storage_root.join` | [569](../../src/management.rs#L569) | receiver-type-required |
| `heal_workspace_metadata_locked` | `self.workspace_configs()?.into_values` | [570](../../src/management.rs#L570) | receiver-type-required |
| `heal_workspace_metadata_locked` | `self.workspace_configs` | [570](../../src/management.rs#L570) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `heal_workspace_metadata_locked` | `primary_path(&config).map` | [572](../../src/management.rs#L572) | receiver-type-required |
| `heal_workspace_metadata_locked` | `primary_path` | [572](../../src/management.rs#L572) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `heal_workspace_metadata_locked` | `self.read_metadata_optional` | [579](../../src/management.rs#L579) | [endpoint::management::ManagementStore::read_metadata_optional](../../src/management.rs#L3090) |
| `heal_workspace_metadata_locked` | `existing                 .as_ref()                 .is_some_and` | [580](../../src/management.rs#L580) | receiver-type-required |
| `heal_workspace_metadata_locked` | `existing                 .as_ref` | [580](../../src/management.rs#L580) | receiver-type-required |
| `heal_workspace_metadata_locked` | `verify_metadata(&config, metadata).is_ok` | [582](../../src/management.rs#L582) | receiver-type-required |
| `heal_workspace_metadata_locked` | `verify_metadata` | [582](../../src/management.rs#L582) | [endpoint::management::verify_metadata](../../src/management.rs#L3675) |
| `heal_workspace_metadata_locked` | `config.id.clone` | [588](../../src/management.rs#L588) | receiver-type-required |
| `heal_workspace_metadata_locked` | `existing                     .map_or_else` | [590](../../src/management.rs#L590) | receiver-type-required |
| `heal_workspace_metadata_locked` | `started_at.to_owned` | [591](../../src/management.rs#L591), [592](../../src/management.rs#L592) | receiver-type-required |
| `heal_workspace_metadata_locked` | `hex_digest` | [593](../../src/management.rs#L593) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `heal_workspace_metadata_locked` | `canonical_line` | [593](../../src/management.rs#L593) | [endpoint::management::canonical_line](../../src/management.rs#L3773) |
| `heal_workspace_metadata_locked` | `publish_canonical` | [595](../../src/management.rs#L595) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `heal_workspace_metadata_locked` | `self.metadata_path` | [595](../../src/management.rs#L595) | [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) |
| `heal_workspace_metadata_locked` | `Ok` | [597](../../src/management.rs#L597) | external-constructor-callback-or-unresolved |
| `lock_exclusive` | `store::endpoint_management_root` | [606](../../src/management.rs#L606) | [store::management_root::endpoint_management_root](../../../store/src/management_root.rs#L47) |
| `lock_exclusive` | `storage_root.as_ref` | [606](../../src/management.rs#L606) | receiver-type-required |
| `lock_exclusive` | `fs::create_dir_all` | [607](../../src/management.rs#L607) | external-constructor-callback-or-unresolved |
| `lock_exclusive` | `Ok` | [608](../../src/management.rs#L608) | external-constructor-callback-or-unresolved |
| `lock_exclusive` | `NamedLock::exclusive` | [608](../../src/management.rs#L608) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `lock_exclusive` | `root.join` | [608](../../src/management.rs#L608) | receiver-type-required |
| `recover` | `self.recover_with_queue_driver` | [618](../../src/management.rs#L618) | [endpoint::management::ManagementStore::recover_with_queue_driver](../../src/management.rs#L621) |
| `recover_with_queue_driver` | `self.recover_skipping` | [625](../../src/management.rs#L625) | [endpoint::management::ManagementStore::recover_skipping](../../src/management.rs#L628) |
| `recover_skipping` | `NamedLock::exclusive` | [637](../../src/management.rs#L637) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `recover_skipping` | `self.root.join` | [637](../../src/management.rs#L637) | receiver-type-required |
| `recover_skipping` | `self.remove_recordless_payloads` | [638](../../src/management.rs#L638) | [endpoint::management::ManagementStore::remove_recordless_payloads](../../src/management.rs#L2923) |
| `recover_skipping` | `self.operation_paths` | [639](../../src/management.rs#L639) | [endpoint::management::ManagementStore::operation_paths](../../src/management.rs#L3039) |
| `recover_skipping` | `self.recover_record` | [645](../../src/management.rs#L645) | [endpoint::management::ManagementStore::recover_record](../../src/management.rs#L653) |
| `recover_skipping` | `validated.as_deref_mut` | [645](../../src/management.rs#L645) | receiver-type-required |
| `recover_skipping` | `first_error.get_or_insert` | [646](../../src/management.rs#L646) | receiver-type-required |
| `recover_skipping` | `first_error.map_or` | [649](../../src/management.rs#L649) | receiver-type-required |
| `recover_skipping` | `Ok` | [649](../../src/management.rs#L649) | external-constructor-callback-or-unresolved |
| `recover_record` | `Some` | [659](../../src/management.rs#L659), [762](../../src/management.rs#L762) | external-constructor-callback-or-unresolved |
| `recover_record` | `NamedLock::exclusive` | [659](../../src/management.rs#L659), [762](../../src/management.rs#L762) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `recover_record` | `self.root.join` | [659](../../src/management.rs#L659), [762](../../src/management.rs#L762) | receiver-type-required |
| `recover_record` | `path.exists` | [660](../../src/management.rs#L660) | receiver-type-required |
| `recover_record` | `Ok` | [661](../../src/management.rs#L661), [667](../../src/management.rs#L667), [679](../../src/management.rs#L679), [707](../../src/management.rs#L707), [725](../../src/management.rs#L725), [747](../../src/management.rs#L747), [765](../../src/management.rs#L765), [792](../../src/management.rs#L792) | external-constructor-callback-or-unresolved |
| `recover_record` | `file_identity` | [663](../../src/management.rs#L663) | [endpoint::management::file_identity](../../src/management.rs#L449) |
| `recover_record` | `validated.as_deref` | [664](../../src/management.rs#L664) | receiver-type-required |
| `recover_record` | `identity.as_ref` | [664](../../src/management.rs#L664) | receiver-type-required |
| `recover_record` | `known.0.contains` | [665](../../src/management.rs#L665) | receiver-type-required |
| `recover_record` | `path.clone` | [665](../../src/management.rs#L665), [683](../../src/management.rs#L683) | receiver-type-required |
| `recover_record` | `read_canonical::<OperationRecord>` | [669](../../src/management.rs#L669), [763](../../src/management.rs#L763) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `recover_record` | `intent_kind` | [670](../../src/management.rs#L670) | [endpoint::management::intent_kind](../../src/management.rs#L3788) |
| `recover_record` | `known.0.insert` | [683](../../src/management.rs#L683) | receiver-type-required |
| `recover_record` | `kind.as_str` | [685](../../src/management.rs#L685), [727](../../src/management.rs#L727) | receiver-type-required |
| `recover_record` | `workspace_intent` | [687](../../src/management.rs#L687), [728](../../src/management.rs#L728) | [endpoint::management::workspace_intent](../../src/management.rs#L3799) |
| `recover_record` | `self.quarantine_corrupt_workspace` | [688](../../src/management.rs#L688), [720](../../src/management.rs#L720), [788](../../src/management.rs#L788) | [endpoint::management::ManagementStore::quarantine_corrupt_workspace](../../src/management.rs#L795) |
| `recover_record` | `folder_move_intent` | [692](../../src/management.rs#L692), [732](../../src/management.rs#L732) | [endpoint::management::folder_move_intent](../../src/management.rs#L3849) |
| `recover_record` | `session_create_intent` | [695](../../src/management.rs#L695), [736](../../src/management.rs#L736) | [endpoint::management::session_create_intent](../../src/management.rs#L3860) |
| `recover_record` | `select_model_intent` | [698](../../src/management.rs#L698), [740](../../src/management.rs#L740) | [endpoint::management::select_model_intent](../../src/management.rs#L3885) |
| `recover_record` | `fork_intent` | [701](../../src/management.rs#L701) | [endpoint::management::fork_intent](../../src/management.rs#L3905) |
| `recover_record` | `queue_transaction_intent` | [704](../../src/management.rs#L704), [752](../../src/management.rs#L752) | [endpoint::management::queue_transaction_intent](../../src/management.rs#L3922) |
| `recover_record` | `verify_queue_payload` | [705](../../src/management.rs#L705), [753](../../src/management.rs#L753) | [endpoint::management::verify_queue_payload](../../src/management.rs#L3965) |
| `recover_record` | `self.operation_payload_path` | [705](../../src/management.rs#L705), [753](../../src/management.rs#L753) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `recover_record` | `queue_completion_from_record` | [706](../../src/management.rs#L706) | [endpoint::management::queue_completion_from_record](../../src/management.rs#L4050) |
| `recover_record` | `Err` | [710](../../src/management.rs#L710), [722](../../src/management.rs#L722), [771](../../src/management.rs#L771), [790](../../src/management.rs#L790) | external-constructor-callback-or-unresolved |
| `recover_record` | `ManagementError::CorruptOperation` | [710](../../src/management.rs#L710), [716](../../src/management.rs#L716), [771](../../src/management.rs#L771) | external-constructor-callback-or-unresolved |
| `recover_record` | `record.response.is_none` | [715](../../src/management.rs#L715) | receiver-type-required |
| `recover_record` | `"complete operation lacks response".to_owned` | [717](../../src/management.rs#L717) | receiver-type-required |
| `recover_record` | `workspace_intent(&record)                 .and_then(&#124;_&#124; self.drive_workspace_operation(&path, record))                 .map` | [728](../../src/management.rs#L728) | receiver-type-required |
| `recover_record` | `workspace_intent(&record)                 .and_then` | [728](../../src/management.rs#L728) | receiver-type-required |
| `recover_record` | `self.drive_workspace_operation` | [729](../../src/management.rs#L729) | [endpoint::management::ManagementStore::drive_workspace_operation](../../src/management.rs#L2311) |
| `recover_record` | `self.drive_folder_move(&path, record, None).map` | [733](../../src/management.rs#L733) | receiver-type-required |
| `recover_record` | `self.drive_folder_move` | [733](../../src/management.rs#L733) | [endpoint::management::ManagementStore::drive_folder_move](../../src/management.rs#L2969) |
| `recover_record` | `self.drive_session_create(&path, record).map` | [737](../../src/management.rs#L737) | receiver-type-required |
| `recover_record` | `self.drive_session_create` | [737](../../src/management.rs#L737) | [endpoint::management::ManagementStore::drive_session_create](../../src/management.rs#L2589) |
| `recover_record` | `self.drive_select_model(&path, record).map` | [741](../../src/management.rs#L741) | receiver-type-required |
| `recover_record` | `self.drive_select_model` | [741](../../src/management.rs#L741) | [endpoint::management::ManagementStore::drive_select_model](../../src/management.rs#L2678) |
| `recover_record` | `self.fork_was_quarantined` | [744](../../src/management.rs#L744) | [endpoint::management::ManagementStore::fork_was_quarantined](../../src/management.rs#L2744) |
| `recover_record` | `self.drive_fork(&path, record).map` | [749](../../src/management.rs#L749) | receiver-type-required |
| `recover_record` | `self.drive_fork` | [749](../../src/management.rs#L749) | [endpoint::management::ManagementStore::drive_fork](../../src/management.rs#L2773) |
| `recover_record` | `pending_queue_transaction` | [754](../../src/management.rs#L754) | [endpoint::management::pending_queue_transaction](../../src/management.rs#L3950) |
| `recover_record` | `queue_driver.ok_or_else` | [755](../../src/management.rs#L755) | receiver-type-required |
| `recover_record` | `ManagementError::QueueRecoveryRequired` | [756](../../src/management.rs#L756) | external-constructor-callback-or-unresolved |
| `recover_record` | `pending.session_id.clone` | [756](../../src/management.rs#L756) | receiver-type-required |
| `recover_record` | `drop` | [760](../../src/management.rs#L760) | external-constructor-callback-or-unresolved |
| `recover_record` | `lock.take` | [760](../../src/management.rs#L760) | receiver-type-required |
| `recover_record` | `driver.execute` | [761](../../src/management.rs#L761) | receiver-type-required |
| `recover_record` | `self.drive_queue_completion(&path, record, decision)                     .map` | [767](../../src/management.rs#L767) | receiver-type-required |
| `recover_record` | `self.drive_queue_completion` | [767](../../src/management.rs#L767) | [endpoint::management::ManagementStore::drive_queue_completion](../../src/management.rs#L2864) |
| `recover_record` | `unavailable_absolute_path` | [779](../../src/management.rs#L779) | [endpoint::management::unavailable_absolute_path](../../src/management.rs#L3580) |
| `quarantine_corrupt_workspace` | `self.quarantine_operation` | [801](../../src/management.rs#L801) | [endpoint::management::ManagementStore::quarantine_operation](../../src/management.rs#L812) |
| `quarantine_corrupt_workspace` | `Err` | [802](../../src/management.rs#L802) | external-constructor-callback-or-unresolved |
| `quarantine_operation` | `path             .file_stem()             .and_then(&#124;value&#124; value.to_str())             .filter(&#124;value&#124; is_hex_digest(value))             .ok_or_else(&#124;&#124; {                 ManagementError::CorruptOperation("operation record has no digest name".to_owned())             })?             .to_owned` | [813](../../src/management.rs#L813) | receiver-type-required |
| `quarantine_operation` | `path             .file_stem()             .and_then(&#124;value&#124; value.to_str())             .filter(&#124;value&#124; is_hex_digest(value))             .ok_or_else` | [813](../../src/management.rs#L813) | receiver-type-required |
| `quarantine_operation` | `path             .file_stem()             .and_then(&#124;value&#124; value.to_str())             .filter` | [813](../../src/management.rs#L813) | receiver-type-required |
| `quarantine_operation` | `path             .file_stem()             .and_then` | [813](../../src/management.rs#L813) | receiver-type-required |
| `quarantine_operation` | `path             .file_stem` | [813](../../src/management.rs#L813) | receiver-type-required |
| `quarantine_operation` | `value.to_str` | [815](../../src/management.rs#L815) | receiver-type-required |
| `quarantine_operation` | `is_hex_digest` | [816](../../src/management.rs#L816) | [endpoint::management::is_hex_digest](../../src/management.rs#L3407) |
| `quarantine_operation` | `ManagementError::CorruptOperation` | [818](../../src/management.rs#L818), [845](../../src/management.rs#L845) | external-constructor-callback-or-unresolved |
| `quarantine_operation` | `"operation record has no digest name".to_owned` | [818](../../src/management.rs#L818) | receiver-type-required |
| `quarantine_operation` | `self.root.join` | [821](../../src/management.rs#L821) | receiver-type-required |
| `quarantine_operation` | `fs::create_dir_all` | [822](../../src/management.rs#L822), [834](../../src/management.rs#L834) | external-constructor-callback-or-unresolved |
| `quarantine_operation` | `quarantine.join` | [824](../../src/management.rs#L824), [827](../../src/management.rs#L827) | receiver-type-required |
| `quarantine_operation` | `target.join("record.json").exists` | [826](../../src/management.rs#L826) | receiver-type-required |
| `quarantine_operation` | `target.join` | [826](../../src/management.rs#L826), [837](../../src/management.rs#L837), [839](../../src/management.rs#L839), [842](../../src/management.rs#L842) | receiver-type-required |
| `quarantine_operation` | `target.exists` | [831](../../src/management.rs#L831) | receiver-type-required |
| `quarantine_operation` | `fs::remove_dir_all` | [832](../../src/management.rs#L832), [849](../../src/management.rs#L849) | external-constructor-callback-or-unresolved |
| `quarantine_operation` | `path.with_extension` | [835](../../src/management.rs#L835) | receiver-type-required |
| `quarantine_operation` | `payload_root.is_dir` | [836](../../src/management.rs#L836), [848](../../src/management.rs#L848) | receiver-type-required |
| `quarantine_operation` | `copy_payload_tree` | [837](../../src/management.rs#L837) | [endpoint::management::copy_payload_tree](../../src/management.rs#L3373) |
| `quarantine_operation` | `AtomicPublisher::replace` | [839](../../src/management.rs#L839) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `quarantine_operation` | `reason.as_bytes` | [839](../../src/management.rs#L839) | receiver-type-required |
| `quarantine_operation` | `File::open(&target)?.sync_all` | [840](../../src/management.rs#L840), [843](../../src/management.rs#L843) | receiver-type-required |
| `quarantine_operation` | `File::open` | [840](../../src/management.rs#L840), [841](../../src/management.rs#L841), [843](../../src/management.rs#L843), [847](../../src/management.rs#L847), [850](../../src/management.rs#L850) | external-constructor-callback-or-unresolved |
| `quarantine_operation` | `File::open(&quarantine)?.sync_all` | [841](../../src/management.rs#L841) | receiver-type-required |
| `quarantine_operation` | `fs::rename` | [842](../../src/management.rs#L842) | external-constructor-callback-or-unresolved |
| `quarantine_operation` | `path.parent().ok_or_else` | [844](../../src/management.rs#L844) | receiver-type-required |
| `quarantine_operation` | `path.parent` | [844](../../src/management.rs#L844) | receiver-type-required |
| `quarantine_operation` | `"operation record has no prefix".to_owned` | [845](../../src/management.rs#L845) | receiver-type-required |
| `quarantine_operation` | `File::open(prefix)?.sync_all` | [847](../../src/management.rs#L847), [850](../../src/management.rs#L850) | receiver-type-required |
| `quarantine_operation` | `Ok` | [853](../../src/management.rs#L853) | external-constructor-callback-or-unresolved |
| `seed_missing_workspace_policies` | `NamedLock::exclusive` | [862](../../src/management.rs#L862) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `seed_missing_workspace_policies` | `self.root.join` | [862](../../src/management.rs#L862) | receiver-type-required |
| `seed_missing_workspace_policies` | `self.workspace_configs()?.into_values` | [863](../../src/management.rs#L863) | receiver-type-required |
| `seed_missing_workspace_policies` | `self.workspace_configs` | [863](../../src/management.rs#L863) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `seed_missing_workspace_policies` | `config.policy.is_some` | [864](../../src/management.rs#L864) | receiver-type-required |
| `seed_missing_workspace_policies` | `primary_path(&config).map` | [867](../../src/management.rs#L867) | receiver-type-required |
| `seed_missing_workspace_policies` | `primary_path` | [867](../../src/management.rs#L867) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `seed_missing_workspace_policies` | `self.config.publish_workspace_policy` | [870](../../src/management.rs#L870) | receiver-type-required |
| `seed_missing_workspace_policies` | `default_workspace_policy` | [873](../../src/management.rs#L873) | [endpoint::management::default_workspace_policy](../../src/management.rs#L3421) |
| `seed_missing_workspace_policies` | `Ok` | [876](../../src/management.rs#L876) | external-constructor-callback-or-unresolved |
| `reconcile_workspace_metadata` | `NamedLock::exclusive` | [884](../../src/management.rs#L884) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `reconcile_workspace_metadata` | `self.root.join` | [884](../../src/management.rs#L884) | receiver-type-required |
| `reconcile_workspace_metadata` | `self.heal_workspace_metadata_locked` | [885](../../src/management.rs#L885) | [endpoint::management::ManagementStore::heal_workspace_metadata_locked](../../src/management.rs#L568) |
| `list_workspaces` | `NamedLock::exclusive` | [892](../../src/management.rs#L892) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `list_workspaces` | `self.root.join` | [892](../../src/management.rs#L892) | receiver-type-required |
| `list_workspaces` | `self.workspace_configs` | [893](../../src/management.rs#L893), [926](../../src/management.rs#L926) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `list_workspaces` | `configs.retain` | [894](../../src/management.rs#L894) | receiver-type-required |
| `list_workspaces` | `primary_path(config).is_ok` | [895](../../src/management.rs#L895), [927](../../src/management.rs#L927) | receiver-type-required |
| `list_workspaces` | `primary_path` | [895](../../src/management.rs#L895), [927](../../src/management.rs#L927) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `list_workspaces` | `BTreeMap::<String, Vec<&SessionInventoryItem>>::new` | [901](../../src/management.rs#L901) | external-constructor-callback-or-unresolved |
| `list_workspaces` | `Vec::new` | [902](../../src/management.rs#L902) | external-constructor-callback-or-unresolved |
| `list_workspaces` | `archived_session_ids.push` | [905](../../src/management.rs#L905) | receiver-type-required |
| `list_workspaces` | `session.session_id.clone` | [905](../../src/management.rs#L905) | receiver-type-required |
| `list_workspaces` | `active                     .entry(session.workspace_id.clone())                     .or_default()                     .push` | [907](../../src/management.rs#L907) | receiver-type-required |
| `list_workspaces` | `active                     .entry(session.workspace_id.clone())                     .or_default` | [907](../../src/management.rs#L907) | receiver-type-required |
| `list_workspaces` | `active                     .entry` | [907](../../src/management.rs#L907) | receiver-type-required |
| `list_workspaces` | `session.workspace_id.clone` | [908](../../src/management.rs#L908) | receiver-type-required |
| `list_workspaces` | `archived_session_ids.sort` | [913](../../src/management.rs#L913) | receiver-type-required |
| `list_workspaces` | `configs.values().all` | [917](../../src/management.rs#L917) | receiver-type-required |
| `list_workspaces` | `configs.values` | [917](../../src/management.rs#L917), [931](../../src/management.rs#L931) | receiver-type-required |
| `list_workspaces` | `self.read_metadata_optional(&config.id)                 .ok()                 .flatten()                 .is_some_and` | [918](../../src/management.rs#L918) | receiver-type-required |
| `list_workspaces` | `self.read_metadata_optional(&config.id)                 .ok()                 .flatten` | [918](../../src/management.rs#L918) | receiver-type-required |
| `list_workspaces` | `self.read_metadata_optional(&config.id)                 .ok` | [918](../../src/management.rs#L918) | receiver-type-required |
| `list_workspaces` | `self.read_metadata_optional` | [918](../../src/management.rs#L918) | [endpoint::management::ManagementStore::read_metadata_optional](../../src/management.rs#L3090) |
| `list_workspaces` | `verify_metadata(config, &metadata).is_ok` | [921](../../src/management.rs#L921) | receiver-type-required |
| `list_workspaces` | `verify_metadata` | [921](../../src/management.rs#L921), [933](../../src/management.rs#L933) | [endpoint::management::verify_metadata](../../src/management.rs#L3675) |
| `list_workspaces` | `self.heal_workspace_metadata_locked` | [925](../../src/management.rs#L925) | [endpoint::management::ManagementStore::heal_workspace_metadata_locked](../../src/management.rs#L568) |
| `list_workspaces` | `current_timestamp` | [925](../../src/management.rs#L925) | [endpoint::management::current_timestamp](../../src/management.rs#L3604) |
| `list_workspaces` | `healed.retain` | [927](../../src/management.rs#L927) | receiver-type-required |
| `list_workspaces` | `Vec::with_capacity` | [930](../../src/management.rs#L930) | external-constructor-callback-or-unresolved |
| `list_workspaces` | `configs.len` | [930](../../src/management.rs#L930) | receiver-type-required |
| `list_workspaces` | `self.read_metadata` | [932](../../src/management.rs#L932) | [endpoint::management::ManagementStore::read_metadata](../../src/management.rs#L3082) |
| `list_workspaces` | `items.push` | [934](../../src/management.rs#L934) | receiver-type-required |
| `list_workspaces` | `workspace_view` | [934](../../src/management.rs#L934) | [endpoint::management::workspace_view](../../src/management.rs#L3693) |
| `list_workspaces` | `active.get(&config.id).cloned().unwrap_or_default` | [937](../../src/management.rs#L937) | receiver-type-required |
| `list_workspaces` | `active.get(&config.id).cloned` | [937](../../src/management.rs#L937) | receiver-type-required |
| `list_workspaces` | `active.get` | [937](../../src/management.rs#L937) | receiver-type-required |
| `list_workspaces` | `items.sort_by` | [940](../../src/management.rs#L940) | receiver-type-required |
| `list_workspaces` | `left.workspace_id.cmp` | [940](../../src/management.rs#L940) | receiver-type-required |
| `list_workspaces` | `Ok` | [941](../../src/management.rs#L941) | external-constructor-callback-or-unresolved |
| `workspace_paths` | `NamedLock::shared` | [951](../../src/management.rs#L951) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `workspace_paths` | `self.root.join` | [951](../../src/management.rs#L951) | receiver-type-required |
| `workspace_paths` | `Ok` | [953](../../src/management.rs#L953) | external-constructor-callback-or-unresolved |
| `workspace_paths` | `self             .workspace_configs()?             .into_iter()             .filter_map(&#124;(id, config)&#124; primary_path(&config).ok().map(&#124;path&#124; (id, path.to_owned())))             .collect` | [953](../../src/management.rs#L953) | receiver-type-required |
| `workspace_paths` | `self             .workspace_configs()?             .into_iter()             .filter_map` | [953](../../src/management.rs#L953) | receiver-type-required |
| `workspace_paths` | `self             .workspace_configs()?             .into_iter` | [953](../../src/management.rs#L953) | receiver-type-required |
| `workspace_paths` | `self             .workspace_configs` | [953](../../src/management.rs#L953) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `workspace_paths` | `primary_path(&config).ok().map` | [956](../../src/management.rs#L956) | receiver-type-required |
| `workspace_paths` | `primary_path(&config).ok` | [956](../../src/management.rs#L956) | receiver-type-required |
| `workspace_paths` | `primary_path` | [956](../../src/management.rs#L956) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `workspace_paths` | `path.to_owned` | [956](../../src/management.rs#L956) | receiver-type-required |
| `workspace_path` | `NamedLock::exclusive` | [961](../../src/management.rs#L961) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `workspace_path` | `self.root.join` | [961](../../src/management.rs#L961) | receiver-type-required |
| `workspace_path` | `self             .workspace_configs()?             .remove(workspace_id)             .ok_or_else` | [962](../../src/management.rs#L962) | receiver-type-required |
| `workspace_path` | `self             .workspace_configs()?             .remove` | [962](../../src/management.rs#L962) | receiver-type-required |
| `workspace_path` | `self             .workspace_configs` | [962](../../src/management.rs#L962) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `workspace_path` | `ManagementError::WorkspaceNotFound` | [965](../../src/management.rs#L965) | external-constructor-callback-or-unresolved |
| `workspace_path` | `workspace_id.to_owned` | [965](../../src/management.rs#L965) | receiver-type-required |
| `workspace_path` | `Ok` | [966](../../src/management.rs#L966) | external-constructor-callback-or-unresolved |
| `workspace_path` | `primary_path(&config)?.to_owned` | [966](../../src/management.rs#L966) | receiver-type-required |
| `workspace_path` | `primary_path` | [966](../../src/management.rs#L966) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `create_session` | `NamedLock::exclusive` | [977](../../src/management.rs#L977) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `create_session` | `self.root.join` | [977](../../src/management.rs#L977) | receiver-type-required |
| `create_session` | `self.operation_path` | [978](../../src/management.rs#L978) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `create_session` | `operation_path.exists` | [979](../../src/management.rs#L979) | receiver-type-required |
| `create_session` | `read_canonical::<OperationRecord>` | [980](../../src/management.rs#L980) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `create_session` | `Err` | [985](../../src/management.rs#L985), [995](../../src/management.rs#L995), [1015](../../src/management.rs#L1015), [1075](../../src/management.rs#L1075) | external-constructor-callback-or-unresolved |
| `create_session` | `request.rpc_id.to_owned` | [986](../../src/management.rs#L986), [1053](../../src/management.rs#L1053), [1084](../../src/management.rs#L1084) | receiver-type-required |
| `create_session` | `"session.create".to_owned` | [987](../../src/management.rs#L987), [1052](../../src/management.rs#L1052), [1085](../../src/management.rs#L1085) | receiver-type-required |
| `create_session` | `self                 .session_id_from_response` | [990](../../src/management.rs#L990) | [endpoint::management::ManagementStore::session_id_from_response](../../src/management.rs#L2664) |
| `create_session` | `self.drive_session_create` | [991](../../src/management.rs#L991), [1108](../../src/management.rs#L1108) | [endpoint::management::ManagementStore::drive_session_create](../../src/management.rs#L2589) |
| `create_session` | `active_session_count` | [994](../../src/management.rs#L994) | [endpoint::management::active_session_count](../../src/management.rs#L4235) |
| `create_session` | `self.workspace_configs` | [997](../../src/management.rs#L997), [1001](../../src/management.rs#L1001) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `create_session` | `self.register_session_cwd` | [999](../../src/management.rs#L999) | [endpoint::management::ManagementStore::register_session_cwd](../../src/management.rs#L1629) |
| `create_session` | `resolve_session_workspace` | [1004](../../src/management.rs#L1004) | [endpoint::management::resolve_session_workspace](../../src/management.rs#L3143) |
| `create_session` | `crate::validate_session_id` | [1007](../../src/management.rs#L1007) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `create_session` | `value.to_owned` | [1008](../../src/management.rs#L1008) | receiver-type-required |
| `create_session` | `allocate_uuid_v7` | [1010](../../src/management.rs#L1010) | [endpoint::management::allocate_uuid_v7](../../src/management.rs#L3585) |
| `create_session` | `self.storage_root.join("threads").join` | [1012](../../src/management.rs#L1012) | receiver-type-required |
| `create_session` | `self.storage_root.join` | [1012](../../src/management.rs#L1012), [1013](../../src/management.rs#L1013) | receiver-type-required |
| `create_session` | `self.storage_root.join("archive").join` | [1013](../../src/management.rs#L1013) | receiver-type-required |
| `create_session` | `active.exists` | [1014](../../src/management.rs#L1014) | receiver-type-required |
| `create_session` | `archived.exists` | [1014](../../src/management.rs#L1014) | receiver-type-required |
| `create_session` | `existing_session_cwd` | [1018](../../src/management.rs#L1018) | [endpoint::management::existing_session_cwd](../../src/management.rs#L3205) |
| `create_session` | `self             .config             .resolve_for_binding` | [1022](../../src/management.rs#L1022) | receiver-type-required |
| `create_session` | `profile::workspace_instructions(             &self.config,             &workspace.id,             request.user_agent_dir,             &config,         )?         .capture` | [1025](../../src/management.rs#L1025) | receiver-type-required |
| `create_session` | `profile::workspace_instructions` | [1025](../../src/management.rs#L1025) | [profile::scope::workspace_instructions](../../../profile/src/scope.rs#L142) |
| `create_session` | `instruction.validate_against_config` | [1032](../../src/management.rs#L1032) | receiver-type-required |
| `create_session` | `SessionPresetEnvironment::new(&workspace.id, &instruction, request.preset_mcp)                 .with_models` | [1034](../../src/management.rs#L1034) | receiver-type-required |
| `create_session` | `SessionPresetEnvironment::new` | [1034](../../src/management.rs#L1034) | [endpoint::management::SessionPresetEnvironment::new](../../src/management.rs#L273) |
| `create_session` | `session_preset` | [1036](../../src/management.rs#L1036) | [endpoint::management::session_preset](../../src/management.rs#L3228) |
| `create_session` | `config.workspace.policy.base.unwrap_or_default` | [1039](../../src/management.rs#L1039) | receiver-type-required |
| `create_session` | `config.canonical_bytes` | [1042](../../src/management.rs#L1042) | receiver-type-required |
| `create_session` | `instruction.canonical_bytes` | [1043](../../src/management.rs#L1043) | receiver-type-required |
| `create_session` | `hex_digest` | [1044](../../src/management.rs#L1044), [1045](../../src/management.rs#L1045) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `create_session` | `request.principal.to_owned` | [1049](../../src/management.rs#L1049) | receiver-type-required |
| `create_session` | `ORIGIN_CLIENT.to_owned` | [1050](../../src/management.rs#L1050) | receiver-type-required |
| `create_session` | `session_id.clone` | [1051](../../src/management.rs#L1051), [1091](../../src/management.rs#L1091) | receiver-type-required |
| `create_session` | `create_genesis` | [1055](../../src/management.rs#L1055) | [endpoint::management::create_genesis](../../src/management.rs#L3265) |
| `create_session` | `preset.as_ref` | [1062](../../src/management.rs#L1062), [1103](../../src/management.rs#L1103) | receiver-type-required |
| `create_session` | `canonical_event_line` | [1066](../../src/management.rs#L1066) | [endpoint::management::canonical_event_line](../../src/management.rs#L3305) |
| `create_session` | `self.operation_payload_path` | [1067](../../src/management.rs#L1067) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `create_session` | `self.remove_recordless_payload` | [1068](../../src/management.rs#L1068) | [endpoint::management::ManagementStore::remove_recordless_payload](../../src/management.rs#L2950) |
| `create_session` | `AssetStore::new` | [1069](../../src/management.rs#L1069) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `create_session` | `payload.join` | [1069](../../src/management.rs#L1069), [1079](../../src/management.rs#L1079) | receiver-type-required |
| `create_session` | `assets.publish` | [1070](../../src/management.rs#L1070), [1071](../../src/management.rs#L1071) | receiver-type-required |
| `create_session` | `ManagementError::CorruptOperation` | [1075](../../src/management.rs#L1075) | external-constructor-callback-or-unresolved |
| `create_session` | `"session snapshot publication changed digest identity".to_owned` | [1076](../../src/management.rs#L1076) | receiver-type-required |
| `create_session` | `AtomicPublisher::replace` | [1079](../../src/management.rs#L1079) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `create_session` | `sync_operation_payload` | [1080](../../src/management.rs#L1080) | [endpoint::management::sync_operation_payload](../../src/management.rs#L3394) |
| `create_session` | `request.request_sha256.to_owned` | [1086](../../src/management.rs#L1086) | receiver-type-required |
| `create_session` | `request.started_at.to_owned` | [1088](../../src/management.rs#L1088) | receiver-type-required |
| `create_session` | `to_ijson` | [1089](../../src/management.rs#L1089) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `create_session` | `"session-create".to_owned` | [1090](../../src/management.rs#L1090) | receiver-type-required |
| `create_session` | `workspace.id.clone` | [1092](../../src/management.rs#L1092) | receiver-type-required |
| `create_session` | `preset.is_none().then` | [1097](../../src/management.rs#L1097) | receiver-type-required |
| `create_session` | `preset.is_none` | [1097](../../src/management.rs#L1097) | receiver-type-required |
| `create_session` | `request                         .identity_profile                         .map_or("auto", tools::IdentityProfile::as_str)                         .to_owned` | [1098](../../src/management.rs#L1098) | receiver-type-required |
| `create_session` | `request                         .identity_profile                         .map_or` | [1098](../../src/management.rs#L1098) | receiver-type-required |
| `create_session` | `preset.as_ref().map` | [1103](../../src/management.rs#L1103) | receiver-type-required |
| `create_session` | `preset.digest.clone` | [1103](../../src/management.rs#L1103) | receiver-type-required |
| `create_session` | `publish_canonical` | [1107](../../src/management.rs#L1107) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `create_session` | `self.session_id_from_response` | [1108](../../src/management.rs#L1108) | [endpoint::management::ManagementStore::session_id_from_response](../../src/management.rs#L2664) |
| `select_model` | `crate::validate_session_id` | [1115](../../src/management.rs#L1115) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `select_model` | `NamedLock::exclusive` | [1116](../../src/management.rs#L1116) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `select_model` | `self.root.join` | [1116](../../src/management.rs#L1116) | receiver-type-required |
| `select_model` | `self.operation_path` | [1117](../../src/management.rs#L1117) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `select_model` | `operation_path.exists` | [1118](../../src/management.rs#L1118) | receiver-type-required |
| `select_model` | `read_canonical::<OperationRecord>` | [1119](../../src/management.rs#L1119) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `select_model` | `Err` | [1124](../../src/management.rs#L1124), [1135](../../src/management.rs#L1135), [1153](../../src/management.rs#L1153) | external-constructor-callback-or-unresolved |
| `select_model` | `request.rpc_id.to_owned` | [1125](../../src/management.rs#L1125), [1173](../../src/management.rs#L1173) | receiver-type-required |
| `select_model` | `"session.selectModel".to_owned` | [1126](../../src/management.rs#L1126), [1174](../../src/management.rs#L1174) | receiver-type-required |
| `select_model` | `selected_model_from_response` | [1129](../../src/management.rs#L1129), [1190](../../src/management.rs#L1190) | [endpoint::management::selected_model_from_response](../../src/management.rs#L4131) |
| `select_model` | `self.drive_select_model` | [1129](../../src/management.rs#L1129), [1190](../../src/management.rs#L1190) | [endpoint::management::ManagementStore::drive_select_model](../../src/management.rs#L2678) |
| `select_model` | `active_session_folder` | [1131](../../src/management.rs#L1131) | [endpoint::management::active_session_folder](../../src/management.rs#L4169) |
| `select_model` | `ThreadStore::open(&self.storage_root)?             .session_has_live_line_holder` | [1132](../../src/management.rs#L1132) | receiver-type-required |
| `select_model` | `ThreadStore::open` | [1132](../../src/management.rs#L1132) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `select_model` | `ManagementError::SessionRunning` | [1135](../../src/management.rs#L1135), [1153](../../src/management.rs#L1153) | external-constructor-callback-or-unresolved |
| `select_model` | `request.session_id.to_owned` | [1136](../../src/management.rs#L1136), [1154](../../src/management.rs#L1154), [1180](../../src/management.rs#L1180) | receiver-type-required |
| `select_model` | `valid_folder_projection` | [1139](../../src/management.rs#L1139) | [endpoint::management::valid_folder_projection](../../src/management.rs#L4180) |
| `select_model` | `projection.lifecycle.latest_turn.is_some` | [1145](../../src/management.rs#L1145) | receiver-type-required |
| `select_model` | `projection.lifecycle.live_inputs.is_empty` | [1150](../../src/management.rs#L1150) | receiver-type-required |
| `select_model` | `self.config.session_settings` | [1157](../../src/management.rs#L1157) | receiver-type-required |
| `select_model` | `current.as_ref().map_or` | [1158](../../src/management.rs#L1158) | receiver-type-required |
| `select_model` | `current.as_ref` | [1158](../../src/management.rs#L1158) | receiver-type-required |
| `select_model` | `request.provider.to_owned` | [1162](../../src/management.rs#L1162) | receiver-type-required |
| `select_model` | `request.model.to_owned` | [1163](../../src/management.rs#L1163) | receiver-type-required |
| `select_model` | `request.reasoning_effort.map` | [1164](../../src/management.rs#L1164) | receiver-type-required |
| `select_model` | `candidate.canonical_bytes` | [1166](../../src/management.rs#L1166) | receiver-type-required |
| `select_model` | `self.remove_recordless_payload` | [1167](../../src/management.rs#L1167) | [endpoint::management::ManagementStore::remove_recordless_payload](../../src/management.rs#L2950) |
| `select_model` | `self.operation_payload_path` | [1168](../../src/management.rs#L1168) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `select_model` | `AtomicPublisher::replace` | [1169](../../src/management.rs#L1169) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `select_model` | `payload.join` | [1169](../../src/management.rs#L1169) | receiver-type-required |
| `select_model` | `sync_operation_payload` | [1170](../../src/management.rs#L1170) | [endpoint::management::sync_operation_payload](../../src/management.rs#L3394) |
| `select_model` | `request.request_sha256.to_owned` | [1175](../../src/management.rs#L1175) | receiver-type-required |
| `select_model` | `request.started_at.to_owned` | [1177](../../src/management.rs#L1177) | receiver-type-required |
| `select_model` | `to_ijson` | [1178](../../src/management.rs#L1178) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `select_model` | `"select-model".to_owned` | [1179](../../src/management.rs#L1179) | receiver-type-required |
| `select_model` | `candidate.provider.clone` | [1183](../../src/management.rs#L1183) | receiver-type-required |
| `select_model` | `candidate.model.clone` | [1184](../../src/management.rs#L1184) | receiver-type-required |
| `select_model` | `candidate.reasoning_effort.clone` | [1185](../../src/management.rs#L1185) | receiver-type-required |
| `select_model` | `publish_canonical` | [1189](../../src/management.rs#L1189) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `fork_session` | `crate::validate_session_id` | [1197](../../src/management.rs#L1197) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `fork_session` | `NamedLock::exclusive` | [1198](../../src/management.rs#L1198) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `fork_session` | `self.root.join` | [1198](../../src/management.rs#L1198) | receiver-type-required |
| `fork_session` | `self.operation_path` | [1199](../../src/management.rs#L1199) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `fork_session` | `operation_path.exists` | [1200](../../src/management.rs#L1200) | receiver-type-required |
| `fork_session` | `read_canonical::<OperationRecord>` | [1201](../../src/management.rs#L1201) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `fork_session` | `Err` | [1206](../../src/management.rs#L1206), [1214](../../src/management.rs#L1214) | external-constructor-callback-or-unresolved |
| `fork_session` | `request.rpc_id.to_owned` | [1207](../../src/management.rs#L1207), [1274](../../src/management.rs#L1274) | receiver-type-required |
| `fork_session` | `"session.fork".to_owned` | [1208](../../src/management.rs#L1208), [1275](../../src/management.rs#L1275) | receiver-type-required |
| `fork_session` | `fork_id_from_response` | [1211](../../src/management.rs#L1211), [1284](../../src/management.rs#L1284) | [endpoint::management::fork_id_from_response](../../src/management.rs#L4119) |
| `fork_session` | `self.drive_fork` | [1211](../../src/management.rs#L1211), [1283](../../src/management.rs#L1283) | [endpoint::management::ManagementStore::drive_fork](../../src/management.rs#L2773) |
| `fork_session` | `active_session_count` | [1213](../../src/management.rs#L1213) | [endpoint::management::active_session_count](../../src/management.rs#L4235) |
| `fork_session` | `active_session_folder` | [1216](../../src/management.rs#L1216) | [endpoint::management::active_session_folder](../../src/management.rs#L4169) |
| `fork_session` | `crate::NativeEndpoint::open(&self.storage_root)?             .reconcile_projection` | [1217](../../src/management.rs#L1217) | receiver-type-required |
| `fork_session` | `crate::NativeEndpoint::open` | [1217](../../src/management.rs#L1217) | [endpoint::service::NativeEndpoint::open](../../src/service.rs#L263) |
| `fork_session` | `crate::EndpointJournal::open` | [1219](../../src/management.rs#L1219) | [endpoint::journal::EndpointJournal::open](../../src/journal.rs#L165) |
| `fork_session` | `journal.last_settled_endpoint_seq()?.ok_or_else` | [1225](../../src/management.rs#L1225) | receiver-type-required |
| `fork_session` | `journal.last_settled_endpoint_seq` | [1225](../../src/management.rs#L1225) | receiver-type-required |
| `fork_session` | `request.source_session_id.to_owned` | [1227](../../src/management.rs#L1227), [1238](../../src/management.rs#L1238), [1263](../../src/management.rs#L1263) | receiver-type-required |
| `fork_session` | `journal             .kernel_anchor_for_endpoint_seq(selected_endpoint_seq)             .map_err` | [1232](../../src/management.rs#L1232) | receiver-type-required |
| `fork_session` | `journal             .kernel_anchor_for_endpoint_seq` | [1232](../../src/management.rs#L1232) | receiver-type-required |
| `fork_session` | `ManagementError::Journal` | [1242](../../src/management.rs#L1242) | external-constructor-callback-or-unresolved |
| `fork_session` | `drop` | [1246](../../src/management.rs#L1246) | external-constructor-callback-or-unresolved |
| `fork_session` | `request             .preset             .map(&#124;raw&#124; {                 self.resolve_fork_preset(                     &source_folder,                     raw,                     request.user_agent_dir,                     request.preset_mcp,                     request.preset_models,                 )             })             .transpose` | [1247](../../src/management.rs#L1247) | receiver-type-required |
| `fork_session` | `request             .preset             .map` | [1247](../../src/management.rs#L1247) | receiver-type-required |
| `fork_session` | `self.resolve_fork_preset` | [1250](../../src/management.rs#L1250) | [endpoint::management::ManagementStore::resolve_fork_preset](../../src/management.rs#L1319) |
| `fork_session` | `allocate_uuid_v7` | [1259](../../src/management.rs#L1259) | [endpoint::management::allocate_uuid_v7](../../src/management.rs#L3585) |
| `fork_session` | `"fork".to_owned` | [1262](../../src/management.rs#L1262) | receiver-type-required |
| `fork_session` | `destination.clone` | [1264](../../src/management.rs#L1264) | receiver-type-required |
| `fork_session` | `request.principal.to_owned` | [1268](../../src/management.rs#L1268) | receiver-type-required |
| `fork_session` | `request.request_sha256.to_owned` | [1276](../../src/management.rs#L1276) | receiver-type-required |
| `fork_session` | `request.started_at.to_owned` | [1278](../../src/management.rs#L1278) | receiver-type-required |
| `fork_session` | `to_ijson` | [1279](../../src/management.rs#L1279) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `fork_session` | `publish_canonical` | [1282](../../src/management.rs#L1282) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `apply_preset_defaults` | `self.config.session_settings` | [1294](../../src/management.rs#L1294) | receiver-type-required |
| `apply_preset_defaults` | `current.as_ref().is_some_and` | [1295](../../src/management.rs#L1295) | receiver-type-required |
| `apply_preset_defaults` | `current.as_ref` | [1295](../../src/management.rs#L1295), [1302](../../src/management.rs#L1302) | receiver-type-required |
| `apply_preset_defaults` | `Ok` | [1300](../../src/management.rs#L1300), [1314](../../src/management.rs#L1314) | external-constructor-callback-or-unresolved |
| `apply_preset_defaults` | `current.as_ref().map_or` | [1302](../../src/management.rs#L1302) | receiver-type-required |
| `apply_preset_defaults` | `self.config.publish_session_settings` | [1303](../../src/management.rs#L1303) | receiver-type-required |
| `apply_preset_defaults` | `defaults.provider.clone` | [1309](../../src/management.rs#L1309) | receiver-type-required |
| `apply_preset_defaults` | `defaults.model.clone` | [1310](../../src/management.rs#L1310) | receiver-type-required |
| `apply_preset_defaults` | `defaults.reasoning_effort.clone` | [1311](../../src/management.rs#L1311) | receiver-type-required |
| `resolve_fork_preset` | `profile::SessionScope::for_session_folder` | [1327](../../src/management.rs#L1327) | [profile::scope::SessionScope::for_session_folder](../../../profile/src/scope.rs#L35) |
| `resolve_fork_preset` | `scope.workspace_id().to_owned` | [1328](../../src/management.rs#L1328) | receiver-type-required |
| `resolve_fork_preset` | `scope.workspace_id` | [1328](../../src/management.rs#L1328) | receiver-type-required |
| `resolve_fork_preset` | `scope.workspace_config` | [1329](../../src/management.rs#L1329) | receiver-type-required |
| `resolve_fork_preset` | `scope             .instructions(&self.config, user_agent_dir, &config)?             .capture` | [1330](../../src/management.rs#L1330) | receiver-type-required |
| `resolve_fork_preset` | `scope             .instructions` | [1330](../../src/management.rs#L1330) | receiver-type-required |
| `resolve_fork_preset` | `SessionPresetEnvironment::new(&workspace_id, &instruction, mcp)             .with_models` | [1333](../../src/management.rs#L1333) | receiver-type-required |
| `resolve_fork_preset` | `SessionPresetEnvironment::new` | [1333](../../src/management.rs#L1333) | [endpoint::management::SessionPresetEnvironment::new](../../src/management.rs#L273) |
| `resolve_fork_preset` | `session_preset(             Some(raw),             None,             config.workspace.policy.base.unwrap_or_default(),             &environment,         )?         .ok_or_else` | [1335](../../src/management.rs#L1335) | receiver-type-required |
| `resolve_fork_preset` | `session_preset` | [1335](../../src/management.rs#L1335) | [endpoint::management::session_preset](../../src/management.rs#L3228) |
| `resolve_fork_preset` | `Some` | [1336](../../src/management.rs#L1336) | external-constructor-callback-or-unresolved |
| `resolve_fork_preset` | `config.workspace.policy.base.unwrap_or_default` | [1338](../../src/management.rs#L1338) | receiver-type-required |
| `resolve_fork_preset` | `ManagementError::CorruptOperation` | [1341](../../src/management.rs#L1341) | external-constructor-callback-or-unresolved |
| `resolve_fork_preset` | `"fork preset did not resolve".to_owned` | [1341](../../src/management.rs#L1341) | receiver-type-required |
| `prepare_queue_transaction` | `crate::validate_session_id` | [1351](../../src/management.rs#L1351) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `prepare_queue_transaction` | `request.replacement_origin.is_some_and` | [1356](../../src/management.rs#L1356) | receiver-type-required |
| `prepare_queue_transaction` | `Err` | [1362](../../src/management.rs#L1362), [1374](../../src/management.rs#L1374) | external-constructor-callback-or-unresolved |
| `prepare_queue_transaction` | `ManagementError::CorruptOperation` | [1362](../../src/management.rs#L1362), [1398](../../src/management.rs#L1398), [1403](../../src/management.rs#L1403) | external-constructor-callback-or-unresolved |
| `prepare_queue_transaction` | `"queue transaction binding is invalid".to_owned` | [1363](../../src/management.rs#L1363) | receiver-type-required |
| `prepare_queue_transaction` | `NamedLock::exclusive` | [1366](../../src/management.rs#L1366) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `prepare_queue_transaction` | `self.root.join` | [1366](../../src/management.rs#L1366) | receiver-type-required |
| `prepare_queue_transaction` | `self.operation_path` | [1367](../../src/management.rs#L1367) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `prepare_queue_transaction` | `operation_path.exists` | [1368](../../src/management.rs#L1368) | receiver-type-required |
| `prepare_queue_transaction` | `read_canonical::<OperationRecord>` | [1369](../../src/management.rs#L1369) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `prepare_queue_transaction` | `request.rpc_id.to_owned` | [1375](../../src/management.rs#L1375), [1433](../../src/management.rs#L1433) | receiver-type-required |
| `prepare_queue_transaction` | `"session.updateQueue".to_owned` | [1376](../../src/management.rs#L1376), [1434](../../src/management.rs#L1434) | receiver-type-required |
| `prepare_queue_transaction` | `verify_queue_payload` | [1380](../../src/management.rs#L1380), [1442](../../src/management.rs#L1442) | [endpoint::management::verify_queue_payload](../../src/management.rs#L3965) |
| `prepare_queue_transaction` | `self.operation_payload_path` | [1381](../../src/management.rs#L1381), [1405](../../src/management.rs#L1405) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `prepare_queue_transaction` | `queue_transaction_intent` | [1382](../../src/management.rs#L1382) | [endpoint::management::queue_transaction_intent](../../src/management.rs#L3922) |
| `prepare_queue_transaction` | `Ok` | [1384](../../src/management.rs#L1384), [1388](../../src/management.rs#L1388), [1443](../../src/management.rs#L1443) | external-constructor-callback-or-unresolved |
| `prepare_queue_transaction` | `QueueTransactionState::Complete` | [1384](../../src/management.rs#L1384) | external-constructor-callback-or-unresolved |
| `prepare_queue_transaction` | `queue_completion_from_record` | [1385](../../src/management.rs#L1385) | [endpoint::management::queue_completion_from_record](../../src/management.rs#L4050) |
| `prepare_queue_transaction` | `QueueTransactionState::Pending` | [1388](../../src/management.rs#L1388), [1443](../../src/management.rs#L1443) | external-constructor-callback-or-unresolved |
| `prepare_queue_transaction` | `pending_queue_transaction` | [1388](../../src/management.rs#L1388), [1443](../../src/management.rs#L1443) | [endpoint::management::pending_queue_transaction](../../src/management.rs#L3950) |
| `prepare_queue_transaction` | `active_session_folder` | [1394](../../src/management.rs#L1394) | [endpoint::management::active_session_folder](../../src/management.rs#L4169) |
| `prepare_queue_transaction` | `fs::read` | [1395](../../src/management.rs#L1395) | external-constructor-callback-or-unresolved |
| `prepare_queue_transaction` | `folder.join` | [1395](../../src/management.rs#L1395) | receiver-type-required |
| `prepare_queue_transaction` | `scan_valid_prefix` | [1396](../../src/management.rs#L1396) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `prepare_queue_transaction` | `scan.projection.ok_or_else` | [1397](../../src/management.rs#L1397) | receiver-type-required |
| `prepare_queue_transaction` | `"queue target has no valid semantic ledger prefix".to_owned` | [1399](../../src/management.rs#L1399) | receiver-type-required |
| `prepare_queue_transaction` | `usize::try_from(scan.valid_bytes).map_err` | [1402](../../src/management.rs#L1402) | receiver-type-required |
| `prepare_queue_transaction` | `usize::try_from` | [1402](../../src/management.rs#L1402) | external-constructor-callback-or-unresolved |
| `prepare_queue_transaction` | `"queue tail length exceeds this host".to_owned` | [1403](../../src/management.rs#L1403) | receiver-type-required |
| `prepare_queue_transaction` | `self.remove_recordless_payload` | [1406](../../src/management.rs#L1406) | [endpoint::management::ManagementStore::remove_recordless_payload](../../src/management.rs#L2950) |
| `prepare_queue_transaction` | `AtomicPublisher::replace` | [1407](../../src/management.rs#L1407), [1411](../../src/management.rs#L1411) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `prepare_queue_transaction` | `payload.join` | [1408](../../src/management.rs#L1408), [1412](../../src/management.rs#L1412) | receiver-type-required |
| `prepare_queue_transaction` | `request.action.canonical_bytes` | [1409](../../src/management.rs#L1409) | receiver-type-required |
| `prepare_queue_transaction` | `canonical_line` | [1413](../../src/management.rs#L1413) | [endpoint::management::canonical_line](../../src/management.rs#L3773) |
| `prepare_queue_transaction` | `request.session_id.to_owned` | [1415](../../src/management.rs#L1415), [1424](../../src/management.rs#L1424) | receiver-type-required |
| `prepare_queue_transaction` | `sync_operation_payload` | [1421](../../src/management.rs#L1421) | [endpoint::management::sync_operation_payload](../../src/management.rs#L3394) |
| `prepare_queue_transaction` | `"queue-transaction".to_owned` | [1423](../../src/management.rs#L1423) | receiver-type-required |
| `prepare_queue_transaction` | `request.action.clone` | [1426](../../src/management.rs#L1426) | receiver-type-required |
| `prepare_queue_transaction` | `request.retract_origin.clone` | [1427](../../src/management.rs#L1427) | receiver-type-required |
| `prepare_queue_transaction` | `request.replacement_origin.cloned` | [1428](../../src/management.rs#L1428) | receiver-type-required |
| `prepare_queue_transaction` | `request.asset_digests.to_vec` | [1429](../../src/management.rs#L1429) | receiver-type-required |
| `prepare_queue_transaction` | `request.request_sha256.to_owned` | [1435](../../src/management.rs#L1435) | receiver-type-required |
| `prepare_queue_transaction` | `request.started_at.to_owned` | [1437](../../src/management.rs#L1437) | receiver-type-required |
| `prepare_queue_transaction` | `to_ijson` | [1438](../../src/management.rs#L1438) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `prepare_queue_transaction` | `publish_canonical` | [1441](../../src/management.rs#L1441) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `complete_queue_transaction` | `NamedLock::exclusive` | [1453](../../src/management.rs#L1453) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `complete_queue_transaction` | `self.root.join` | [1453](../../src/management.rs#L1453) | receiver-type-required |
| `complete_queue_transaction` | `self.operation_path` | [1454](../../src/management.rs#L1454) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `complete_queue_transaction` | `read_canonical::<OperationRecord>` | [1455](../../src/management.rs#L1455) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `complete_queue_transaction` | `Err` | [1457](../../src/management.rs#L1457) | external-constructor-callback-or-unresolved |
| `complete_queue_transaction` | `ManagementError::CorruptOperation` | [1457](../../src/management.rs#L1457) | external-constructor-callback-or-unresolved |
| `complete_queue_transaction` | `"queue completion does not match its operation".to_owned` | [1458](../../src/management.rs#L1458) | receiver-type-required |
| `complete_queue_transaction` | `self.drive_queue_completion` | [1461](../../src/management.rs#L1461) | [endpoint::management::ManagementStore::drive_queue_completion](../../src/management.rs#L2864) |
| `has_incomplete_session_operation` | `crate::validate_session_id` | [1468](../../src/management.rs#L1468) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `has_incomplete_session_operation` | `NamedLock::shared` | [1469](../../src/management.rs#L1469) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `has_incomplete_session_operation` | `self.root.join` | [1469](../../src/management.rs#L1469) | receiver-type-required |
| `has_incomplete_session_operation` | `self.operation_paths` | [1470](../../src/management.rs#L1470) | [endpoint::management::ManagementStore::operation_paths](../../src/management.rs#L3039) |
| `has_incomplete_session_operation` | `read_canonical::<OperationRecord>` | [1471](../../src/management.rs#L1471) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `has_incomplete_session_operation` | `operation_target_session(&record)?.as_deref` | [1473](../../src/management.rs#L1473) | receiver-type-required |
| `has_incomplete_session_operation` | `operation_target_session` | [1473](../../src/management.rs#L1473) | [endpoint::management::operation_target_session](../../src/management.rs#L4097) |
| `has_incomplete_session_operation` | `Some` | [1473](../../src/management.rs#L1473) | external-constructor-callback-or-unresolved |
| `has_incomplete_session_operation` | `Ok` | [1475](../../src/management.rs#L1475), [1478](../../src/management.rs#L1478) | external-constructor-callback-or-unresolved |
| `completed_fork_lineage` | `NamedLock::shared` | [1482](../../src/management.rs#L1482) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `completed_fork_lineage` | `self.root.join` | [1482](../../src/management.rs#L1482) | receiver-type-required |
| `completed_fork_lineage` | `BTreeMap::new` | [1483](../../src/management.rs#L1483) | external-constructor-callback-or-unresolved |
| `completed_fork_lineage` | `self.operation_paths` | [1484](../../src/management.rs#L1484) | [endpoint::management::ManagementStore::operation_paths](../../src/management.rs#L3039) |
| `completed_fork_lineage` | `read_canonical::<OperationRecord>` | [1485](../../src/management.rs#L1485) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `completed_fork_lineage` | `intent_kind` | [1486](../../src/management.rs#L1486) | [endpoint::management::intent_kind](../../src/management.rs#L3788) |
| `completed_fork_lineage` | `fork_intent` | [1487](../../src/management.rs#L1487) | [endpoint::management::fork_intent](../../src/management.rs#L3905) |
| `completed_fork_lineage` | `lineage.insert(intent.dest, entry).is_some` | [1492](../../src/management.rs#L1492) | receiver-type-required |
| `completed_fork_lineage` | `lineage.insert` | [1492](../../src/management.rs#L1492) | receiver-type-required |
| `completed_fork_lineage` | `Err` | [1493](../../src/management.rs#L1493) | external-constructor-callback-or-unresolved |
| `completed_fork_lineage` | `ManagementError::CorruptOperation` | [1493](../../src/management.rs#L1493) | external-constructor-callback-or-unresolved |
| `completed_fork_lineage` | `"fork destination has more than one completed lineage".to_owned` | [1494](../../src/management.rs#L1494) | receiver-type-required |
| `completed_fork_lineage` | `Ok` | [1499](../../src/management.rs#L1499) | external-constructor-callback-or-unresolved |
| `create_workspace` | `NamedLock::exclusive` | [1509](../../src/management.rs#L1509) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `create_workspace` | `self.root.join` | [1509](../../src/management.rs#L1509) | receiver-type-required |
| `create_workspace` | `canonical_workspace_path` | [1510](../../src/management.rs#L1510) | [endpoint::management::canonical_workspace_path](../../src/management.rs#L3571) |
| `create_workspace` | `self.create_workspace_locked` | [1511](../../src/management.rs#L1511) | [endpoint::management::ManagementStore::create_workspace_locked](../../src/management.rs#L1523) |
| `create_workspace_locked` | `canonical.to_string_lossy().into_owned` | [1535](../../src/management.rs#L1535) | receiver-type-required |
| `create_workspace_locked` | `canonical.to_string_lossy` | [1535](../../src/management.rs#L1535) | receiver-type-required |
| `create_workspace_locked` | `self.workspace_configs` | [1536](../../src/management.rs#L1536) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `create_workspace_locked` | `configs             .values()             .find` | [1537](../../src/management.rs#L1537) | receiver-type-required |
| `create_workspace_locked` | `configs             .values` | [1537](../../src/management.rs#L1537) | receiver-type-required |
| `create_workspace_locked` | `primary_path(config).ok` | [1539](../../src/management.rs#L1539) | receiver-type-required |
| `create_workspace_locked` | `primary_path` | [1539](../../src/management.rs#L1539) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `create_workspace_locked` | `Some` | [1539](../../src/management.rs#L1539), [1572](../../src/management.rs#L1572), [1589](../../src/management.rs#L1589) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `canonical_string.as_str` | [1539](../../src/management.rs#L1539), [1549](../../src/management.rs#L1549) | receiver-type-required |
| `create_workspace_locked` | `existing.is_none` | [1540](../../src/management.rs#L1540) | receiver-type-required |
| `create_workspace_locked` | `self.guard_folder_entry` | [1541](../../src/management.rs#L1541) | [endpoint::management::ManagementStore::guard_folder_entry](../../src/management.rs#L1613) |
| `create_workspace_locked` | `config.clone` | [1544](../../src/management.rs#L1544) | receiver-type-required |
| `create_workspace_locked` | `configs                 .values()                 .any` | [1547](../../src/management.rs#L1547) | receiver-type-required |
| `create_workspace_locked` | `configs                 .values` | [1547](../../src/management.rs#L1547) | receiver-type-required |
| `create_workspace_locked` | `config.folder_paths().contains` | [1549](../../src/management.rs#L1549) | receiver-type-required |
| `create_workspace_locked` | `config.folder_paths` | [1549](../../src/management.rs#L1549) | receiver-type-required |
| `create_workspace_locked` | `Err` | [1551](../../src/management.rs#L1551) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `ManagementError::InvalidPath` | [1551](../../src/management.rs#L1551) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `path.to_owned` | [1551](../../src/management.rs#L1551), [1558](../../src/management.rs#L1558) | receiver-type-required |
| `create_workspace_locked` | `canonical.file_name().and_then` | [1554](../../src/management.rs#L1554) | receiver-type-required |
| `create_workspace_locked` | `canonical.file_name` | [1554](../../src/management.rs#L1554) | receiver-type-required |
| `create_workspace_locked` | `name.to_str` | [1554](../../src/management.rs#L1554) | receiver-type-required |
| `create_workspace_locked` | `folder_name                             .ok_or_else` | [1557](../../src/management.rs#L1557) | receiver-type-required |
| `create_workspace_locked` | `ManagementError::InvalidTitle` | [1558](../../src/management.rs#L1558) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `validate_title` | [1559](../../src/management.rs#L1559) | [endpoint::management::validate_title](../../src/management.rs#L3613) |
| `create_workspace_locked` | `ensure_unique_title` | [1560](../../src/management.rs#L1560) | [endpoint::management::ensure_unique_title](../../src/management.rs#L3661) |
| `create_workspace_locked` | `title.to_owned` | [1561](../../src/management.rs#L1561) | receiver-type-required |
| `create_workspace_locked` | `first_free_title` | [1563](../../src/management.rs#L1563) | [endpoint::management::first_free_title](../../src/management.rs#L3643) |
| `create_workspace_locked` | `allocate_uuid_v7` | [1569](../../src/management.rs#L1569) | [endpoint::management::allocate_uuid_v7](../../src/management.rs#L3585) |
| `create_workspace_locked` | `Vec::new` | [1571](../../src/management.rs#L1571) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `default_workspace_policy` | [1572](../../src/management.rs#L1572) | [endpoint::management::default_workspace_policy](../../src/management.rs#L3421) |
| `create_workspace_locked` | `self.begin_workspace_operation` | [1582](../../src/management.rs#L1582) | [endpoint::management::ManagementStore::begin_workspace_operation](../../src/management.rs#L2254) |
| `create_workspace_locked` | `self.drive_workspace_operation` | [1593](../../src/management.rs#L1593) | [endpoint::management::ManagementStore::drive_workspace_operation](../../src/management.rs#L2311) |
| `create_workspace_locked` | `response             .get("workspace")             .ok_or` | [1594](../../src/management.rs#L1594) | receiver-type-required |
| `create_workspace_locked` | `response             .get` | [1594](../../src/management.rs#L1594), [1599](../../src/management.rs#L1599) | receiver-type-required |
| `create_workspace_locked` | `ManagementError::CorruptOperation` | [1596](../../src/management.rs#L1596), [1603](../../src/management.rs#L1603) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `"missing workspace response".to_owned` | [1597](../../src/management.rs#L1597) | receiver-type-required |
| `create_workspace_locked` | `response             .get("created")             .and_then(Value::as_bool)             .ok_or_else` | [1599](../../src/management.rs#L1599) | receiver-type-required |
| `create_workspace_locked` | `response             .get("created")             .and_then` | [1599](../../src/management.rs#L1599) | receiver-type-required |
| `create_workspace_locked` | `"missing create decision".to_owned` | [1603](../../src/management.rs#L1603) | receiver-type-required |
| `create_workspace_locked` | `Ok` | [1605](../../src/management.rs#L1605) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `serde_json::from_value` | [1605](../../src/management.rs#L1605) | external-constructor-callback-or-unresolved |
| `create_workspace_locked` | `workspace.clone` | [1605](../../src/management.rs#L1605) | receiver-type-required |
| `guard_folder_entry` | `tools::kernel_protected_roots` | [1614](../../src/management.rs#L1614) | [tools::sandbox::kernel_protected_roots](../../../tools/src/sandbox.rs#L150) |
| `guard_folder_entry` | `canonical.parent().is_none` | [1615](../../src/management.rs#L1615) | receiver-type-required |
| `guard_folder_entry` | `canonical.parent` | [1615](../../src/management.rs#L1615) | receiver-type-required |
| `guard_folder_entry` | `canonical.is_dir` | [1616](../../src/management.rs#L1616) | receiver-type-required |
| `guard_folder_entry` | `tools::protected_root_containing(&protected, canonical).is_some` | [1617](../../src/management.rs#L1617) | receiver-type-required |
| `guard_folder_entry` | `tools::protected_root_containing` | [1617](../../src/management.rs#L1617) | [tools::sandbox::protected_root_containing](../../../tools/src/sandbox.rs#L199) |
| `guard_folder_entry` | `Err` | [1619](../../src/management.rs#L1619) | external-constructor-callback-or-unresolved |
| `guard_folder_entry` | `ManagementError::InvalidPath` | [1619](../../src/management.rs#L1619) | external-constructor-callback-or-unresolved |
| `guard_folder_entry` | `requested.to_owned` | [1619](../../src/management.rs#L1619) | receiver-type-required |
| `guard_folder_entry` | `Ok` | [1621](../../src/management.rs#L1621) | external-constructor-callback-or-unresolved |
| `register_session_cwd` | `canonical_workspace_path` | [1635](../../src/management.rs#L1635) | [endpoint::management::canonical_workspace_path](../../src/management.rs#L3571) |
| `register_session_cwd` | `canonical.to_string_lossy().into_owned` | [1636](../../src/management.rs#L1636) | receiver-type-required |
| `register_session_cwd` | `canonical.to_string_lossy` | [1636](../../src/management.rs#L1636) | receiver-type-required |
| `register_session_cwd` | `configs.values` | [1637](../../src/management.rs#L1637) | receiver-type-required |
| `register_session_cwd` | `binding_for_canonical_path(workspace, &canonical_string)?.is_some` | [1638](../../src/management.rs#L1638) | receiver-type-required |
| `register_session_cwd` | `binding_for_canonical_path` | [1638](../../src/management.rs#L1638) | [endpoint::management::binding_for_canonical_path](../../src/management.rs#L3118) |
| `register_session_cwd` | `Ok` | [1639](../../src/management.rs#L1639), [1655](../../src/management.rs#L1655) | external-constructor-callback-or-unresolved |
| `register_session_cwd` | `self.create_workspace_locked` | [1647](../../src/management.rs#L1647) | [endpoint::management::ManagementStore::create_workspace_locked](../../src/management.rs#L1523) |
| `rename_workspace` | `validate_title` | [1666](../../src/management.rs#L1666) | [endpoint::management::validate_title](../../src/management.rs#L3613) |
| `rename_workspace` | `NamedLock::exclusive` | [1667](../../src/management.rs#L1667) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `rename_workspace` | `self.root.join` | [1667](../../src/management.rs#L1667) | receiver-type-required |
| `rename_workspace` | `self.workspace_configs` | [1668](../../src/management.rs#L1668) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `rename_workspace` | `ensure_unique_title` | [1669](../../src/management.rs#L1669) | [endpoint::management::ensure_unique_title](../../src/management.rs#L3661) |
| `rename_workspace` | `Some` | [1669](../../src/management.rs#L1669) | external-constructor-callback-or-unresolved |
| `rename_workspace` | `configs             .get(workspace_id)             .ok_or_else` | [1670](../../src/management.rs#L1670) | receiver-type-required |
| `rename_workspace` | `configs             .get` | [1670](../../src/management.rs#L1670) | receiver-type-required |
| `rename_workspace` | `ManagementError::WorkspaceNotFound` | [1672](../../src/management.rs#L1672) | external-constructor-callback-or-unresolved |
| `rename_workspace` | `workspace_id.to_owned` | [1672](../../src/management.rs#L1672) | receiver-type-required |
| `rename_workspace` | `current.id.clone` | [1676](../../src/management.rs#L1676) | receiver-type-required |
| `rename_workspace` | `title.to_owned` | [1677](../../src/management.rs#L1677) | receiver-type-required |
| `rename_workspace` | `current.cwd.clone` | [1678](../../src/management.rs#L1678) | receiver-type-required |
| `rename_workspace` | `current.folders.clone` | [1679](../../src/management.rs#L1679) | receiver-type-required |
| `rename_workspace` | `current.policy.clone` | [1680](../../src/management.rs#L1680) | receiver-type-required |
| `rename_workspace` | `self.begin_workspace_operation` | [1682](../../src/management.rs#L1682) | [endpoint::management::ManagementStore::begin_workspace_operation](../../src/management.rs#L2254) |
| `rename_workspace` | `self.drive_workspace_operation` | [1693](../../src/management.rs#L1693) | [endpoint::management::ManagementStore::drive_workspace_operation](../../src/management.rs#L2311) |
| `rename_workspace` | `serde_json::from_value(response.get("workspace").cloned().ok_or_else(&#124;&#124; {             ManagementError::CorruptOperation("missing workspace response".to_owned())         })?)         .map_err` | [1694](../../src/management.rs#L1694) | receiver-type-required |
| `rename_workspace` | `serde_json::from_value` | [1694](../../src/management.rs#L1694) | external-constructor-callback-or-unresolved |
| `rename_workspace` | `response.get("workspace").cloned().ok_or_else` | [1694](../../src/management.rs#L1694) | receiver-type-required |
| `rename_workspace` | `response.get("workspace").cloned` | [1694](../../src/management.rs#L1694) | receiver-type-required |
| `rename_workspace` | `response.get` | [1694](../../src/management.rs#L1694) | receiver-type-required |
| `rename_workspace` | `ManagementError::CorruptOperation` | [1695](../../src/management.rs#L1695) | external-constructor-callback-or-unresolved |
| `rename_workspace` | `"missing workspace response".to_owned` | [1695](../../src/management.rs#L1695) | receiver-type-required |
| `relocate_workspace` | `canonical_workspace_path` | [1713](../../src/management.rs#L1713) | [endpoint::management::canonical_workspace_path](../../src/management.rs#L3571) |
| `relocate_workspace` | `self.guard_folder_entry` | [1714](../../src/management.rs#L1714) | [endpoint::management::ManagementStore::guard_folder_entry](../../src/management.rs#L1613) |
| `relocate_workspace` | `canonical.to_string_lossy().into_owned` | [1715](../../src/management.rs#L1715) | receiver-type-required |
| `relocate_workspace` | `canonical.to_string_lossy` | [1715](../../src/management.rs#L1715) | receiver-type-required |
| `relocate_workspace` | `NamedLock::exclusive` | [1716](../../src/management.rs#L1716) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `relocate_workspace` | `self.root.join` | [1716](../../src/management.rs#L1716) | receiver-type-required |
| `relocate_workspace` | `self.workspace_configs` | [1717](../../src/management.rs#L1717) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `relocate_workspace` | `configs             .get(workspace_id)             .ok_or_else` | [1718](../../src/management.rs#L1718) | receiver-type-required |
| `relocate_workspace` | `configs             .get` | [1718](../../src/management.rs#L1718) | receiver-type-required |
| `relocate_workspace` | `ManagementError::WorkspaceNotFound` | [1720](../../src/management.rs#L1720) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `workspace_id.to_owned` | [1720](../../src/management.rs#L1720) | receiver-type-required |
| `relocate_workspace` | `current.folders.clone` | [1722](../../src/management.rs#L1722) | receiver-type-required |
| `relocate_workspace` | `folders             .iter_mut()             .find(&#124;folder&#124; folder.path == previous_path)             .ok_or_else` | [1723](../../src/management.rs#L1723) | receiver-type-required |
| `relocate_workspace` | `folders             .iter_mut()             .find` | [1723](../../src/management.rs#L1723) | receiver-type-required |
| `relocate_workspace` | `folders             .iter_mut` | [1723](../../src/management.rs#L1723) | receiver-type-required |
| `relocate_workspace` | `ManagementError::InvalidPath` | [1726](../../src/management.rs#L1726) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `previous_path.to_owned` | [1726](../../src/management.rs#L1726) | receiver-type-required |
| `relocate_workspace` | `target.id.clone` | [1727](../../src/management.rs#L1727) | receiver-type-required |
| `relocate_workspace` | `configs.values().any` | [1728](../../src/management.rs#L1728) | receiver-type-required |
| `relocate_workspace` | `configs.values` | [1728](../../src/management.rs#L1728) | receiver-type-required |
| `relocate_workspace` | `config.folders.iter().any` | [1729](../../src/management.rs#L1729) | receiver-type-required |
| `relocate_workspace` | `config.folders.iter` | [1729](../../src/management.rs#L1729) | receiver-type-required |
| `relocate_workspace` | `Err` | [1734](../../src/management.rs#L1734) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `ManagementError::WorkspaceAmbiguous` | [1734](../../src/management.rs#L1734) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `canonical_string.clone` | [1736](../../src/management.rs#L1736), [1742](../../src/management.rs#L1742) | receiver-type-required |
| `relocate_workspace` | `current             .cwd             .iter()             .map(&#124;candidate&#124; {                 if candidate == previous_path {                     canonical_string.clone()                 } else {                     candidate.clone()                 }             })             .collect` | [1737](../../src/management.rs#L1737) | receiver-type-required |
| `relocate_workspace` | `current             .cwd             .iter()             .map` | [1737](../../src/management.rs#L1737) | receiver-type-required |
| `relocate_workspace` | `current             .cwd             .iter` | [1737](../../src/management.rs#L1737) | receiver-type-required |
| `relocate_workspace` | `candidate.clone` | [1744](../../src/management.rs#L1744) | receiver-type-required |
| `relocate_workspace` | `current.id.clone` | [1751](../../src/management.rs#L1751) | receiver-type-required |
| `relocate_workspace` | `current.name.clone` | [1752](../../src/management.rs#L1752) | receiver-type-required |
| `relocate_workspace` | `current                 .policy                 .clone()                 .map` | [1755](../../src/management.rs#L1755) | receiver-type-required |
| `relocate_workspace` | `current                 .policy                 .clone` | [1755](../../src/management.rs#L1755) | receiver-type-required |
| `relocate_workspace` | `relocate_policy_roots` | [1758](../../src/management.rs#L1758) | [endpoint::management::relocate_policy_roots](../../src/management.rs#L3436) |
| `relocate_workspace` | `self.begin_workspace_operation` | [1760](../../src/management.rs#L1760) | [endpoint::management::ManagementStore::begin_workspace_operation](../../src/management.rs#L2254) |
| `relocate_workspace` | `Some` | [1768](../../src/management.rs#L1768) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `self.drive_workspace_operation` | [1771](../../src/management.rs#L1771) | [endpoint::management::ManagementStore::drive_workspace_operation](../../src/management.rs#L2311) |
| `relocate_workspace` | `serde_json::from_value(response.get("workspace").cloned().ok_or_else(&#124;&#124; {             ManagementError::CorruptOperation("missing workspace response".to_owned())         })?)         .map_err` | [1772](../../src/management.rs#L1772) | receiver-type-required |
| `relocate_workspace` | `serde_json::from_value` | [1772](../../src/management.rs#L1772) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `response.get("workspace").cloned().ok_or_else` | [1772](../../src/management.rs#L1772) | receiver-type-required |
| `relocate_workspace` | `response.get("workspace").cloned` | [1772](../../src/management.rs#L1772) | receiver-type-required |
| `relocate_workspace` | `response.get` | [1772](../../src/management.rs#L1772) | receiver-type-required |
| `relocate_workspace` | `ManagementError::CorruptOperation` | [1773](../../src/management.rs#L1773) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `"missing workspace response".to_owned` | [1773](../../src/management.rs#L1773) | receiver-type-required |
| `workspace_folders` | `self             .workspace_configs()?             .remove(workspace_id)             .ok_or_else` | [1784](../../src/management.rs#L1784) | receiver-type-required |
| `workspace_folders` | `self             .workspace_configs()?             .remove` | [1784](../../src/management.rs#L1784) | receiver-type-required |
| `workspace_folders` | `self             .workspace_configs` | [1784](../../src/management.rs#L1784) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `workspace_folders` | `ManagementError::WorkspaceNotFound` | [1787](../../src/management.rs#L1787) | external-constructor-callback-or-unresolved |
| `workspace_folders` | `workspace_id.to_owned` | [1787](../../src/management.rs#L1787) | receiver-type-required |
| `workspace_folders` | `Ok` | [1788](../../src/management.rs#L1788) | external-constructor-callback-or-unresolved |
| `workspace_folders` | `folders_view` | [1788](../../src/management.rs#L1788) | [endpoint::management::folders_view](../../src/management.rs#L3545) |
| `add_workspace_folder` | `NamedLock::exclusive` | [1803](../../src/management.rs#L1803) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `add_workspace_folder` | `self.root.join` | [1803](../../src/management.rs#L1803) | receiver-type-required |
| `add_workspace_folder` | `self.resume_folder_operation` | [1805](../../src/management.rs#L1805) | [endpoint::management::ManagementStore::resume_folder_operation](../../src/management.rs#L1914) |
| `add_workspace_folder` | `folder_response` | [1807](../../src/management.rs#L1807) | [endpoint::management::folder_response](../../src/management.rs#L3559) |
| `add_workspace_folder` | `canonical_workspace_path` | [1809](../../src/management.rs#L1809) | [endpoint::management::canonical_workspace_path](../../src/management.rs#L3571) |
| `add_workspace_folder` | `self.guard_folder_entry` | [1810](../../src/management.rs#L1810) | [endpoint::management::ManagementStore::guard_folder_entry](../../src/management.rs#L1613) |
| `add_workspace_folder` | `canonical_path.to_string_lossy().into_owned` | [1811](../../src/management.rs#L1811) | receiver-type-required |
| `add_workspace_folder` | `canonical_path.to_string_lossy` | [1811](../../src/management.rs#L1811) | receiver-type-required |
| `add_workspace_folder` | `self.workspace_configs` | [1812](../../src/management.rs#L1812) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `add_workspace_folder` | `configs             .get(workspace_id)             .ok_or_else` | [1813](../../src/management.rs#L1813) | receiver-type-required |
| `add_workspace_folder` | `configs             .get` | [1813](../../src/management.rs#L1813) | receiver-type-required |
| `add_workspace_folder` | `ManagementError::WorkspaceNotFound` | [1815](../../src/management.rs#L1815) | external-constructor-callback-or-unresolved |
| `add_workspace_folder` | `workspace_id.to_owned` | [1815](../../src/management.rs#L1815), [1832](../../src/management.rs#L1832) | receiver-type-required |
| `add_workspace_folder` | `configs             .values()             .any` | [1816](../../src/management.rs#L1816) | receiver-type-required |
| `add_workspace_folder` | `configs             .values` | [1816](../../src/management.rs#L1816) | receiver-type-required |
| `add_workspace_folder` | `config.folder_paths().contains` | [1818](../../src/management.rs#L1818) | receiver-type-required |
| `add_workspace_folder` | `config.folder_paths` | [1818](../../src/management.rs#L1818) | receiver-type-required |
| `add_workspace_folder` | `canonical.as_str` | [1818](../../src/management.rs#L1818) | receiver-type-required |
| `add_workspace_folder` | `Err` | [1820](../../src/management.rs#L1820), [1831](../../src/management.rs#L1831) | external-constructor-callback-or-unresolved |
| `add_workspace_folder` | `ManagementError::WorkspaceAmbiguous` | [1820](../../src/management.rs#L1820) | external-constructor-callback-or-unresolved |
| `add_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter()             .filter(&#124;(_, binding)&#124; binding.is_none())             .map(&#124;(session_id, _)&#124; session_id)             .collect::<Vec<_>>` | [1824](../../src/management.rs#L1824) | receiver-type-required |
| `add_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter()             .filter(&#124;(_, binding)&#124; binding.is_none())             .map` | [1824](../../src/management.rs#L1824) | receiver-type-required |
| `add_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter()             .filter` | [1824](../../src/management.rs#L1824) | receiver-type-required |
| `add_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter` | [1824](../../src/management.rs#L1824) | receiver-type-required |
| `add_workspace_folder` | `self             .workspace_session_bindings` | [1824](../../src/management.rs#L1824) | [endpoint::management::ManagementStore::workspace_session_bindings](../../src/management.rs#L1948) |
| `add_workspace_folder` | `binding.is_none` | [1827](../../src/management.rs#L1827) | receiver-type-required |
| `add_workspace_folder` | `legacy.is_empty` | [1830](../../src/management.rs#L1830) | receiver-type-required |
| `add_workspace_folder` | `next_folder_id` | [1836](../../src/management.rs#L1836) | [endpoint::management::next_folder_id](../../src/management.rs#L3477) |
| `add_workspace_folder` | `materialized_folders` | [1836](../../src/management.rs#L1836) | [endpoint::management::materialized_folders](../../src/management.rs#L3461) |
| `add_workspace_folder` | `with_added_folder` | [1837](../../src/management.rs#L1837) | [endpoint::management::with_added_folder](../../src/management.rs#L3488) |
| `add_workspace_folder` | `self.drive_folder_operation` | [1838](../../src/management.rs#L1838) | [endpoint::management::ManagementStore::drive_folder_operation](../../src/management.rs#L1938) |
| `add_workspace_folder` | `Some` | [1847](../../src/management.rs#L1847) | external-constructor-callback-or-unresolved |
| `remove_workspace_folder` | `NamedLock::exclusive` | [1864](../../src/management.rs#L1864) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `remove_workspace_folder` | `self.root.join` | [1864](../../src/management.rs#L1864) | receiver-type-required |
| `remove_workspace_folder` | `self.resume_folder_operation` | [1866](../../src/management.rs#L1866) | [endpoint::management::ManagementStore::resume_folder_operation](../../src/management.rs#L1914) |
| `remove_workspace_folder` | `folder_response` | [1868](../../src/management.rs#L1868) | [endpoint::management::folder_response](../../src/management.rs#L3559) |
| `remove_workspace_folder` | `self.workspace_configs` | [1870](../../src/management.rs#L1870) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `remove_workspace_folder` | `configs             .get(workspace_id)             .ok_or_else` | [1871](../../src/management.rs#L1871) | receiver-type-required |
| `remove_workspace_folder` | `configs             .get` | [1871](../../src/management.rs#L1871) | receiver-type-required |
| `remove_workspace_folder` | `ManagementError::WorkspaceNotFound` | [1873](../../src/management.rs#L1873) | external-constructor-callback-or-unresolved |
| `remove_workspace_folder` | `workspace_id.to_owned` | [1873](../../src/management.rs#L1873), [1881](../../src/management.rs#L1881), [1891](../../src/management.rs#L1891) | receiver-type-required |
| `remove_workspace_folder` | `materialized_folders` | [1874](../../src/management.rs#L1874) | [endpoint::management::materialized_folders](../../src/management.rs#L3461) |
| `remove_workspace_folder` | `folders             .iter()             .find(&#124;folder&#124; folder.path == path)             .map(&#124;folder&#124; folder.id.clone())             .ok_or_else` | [1875](../../src/management.rs#L1875) | receiver-type-required |
| `remove_workspace_folder` | `folders             .iter()             .find(&#124;folder&#124; folder.path == path)             .map` | [1875](../../src/management.rs#L1875) | receiver-type-required |
| `remove_workspace_folder` | `folders             .iter()             .find` | [1875](../../src/management.rs#L1875) | receiver-type-required |
| `remove_workspace_folder` | `folders             .iter` | [1875](../../src/management.rs#L1875) | receiver-type-required |
| `remove_workspace_folder` | `folder.id.clone` | [1878](../../src/management.rs#L1878) | receiver-type-required |
| `remove_workspace_folder` | `ManagementError::InvalidPath` | [1879](../../src/management.rs#L1879) | external-constructor-callback-or-unresolved |
| `remove_workspace_folder` | `path.to_owned` | [1879](../../src/management.rs#L1879), [1892](../../src/management.rs#L1892) | receiver-type-required |
| `remove_workspace_folder` | `folders.len` | [1880](../../src/management.rs#L1880) | receiver-type-required |
| `remove_workspace_folder` | `Err` | [1881](../../src/management.rs#L1881), [1890](../../src/management.rs#L1890) | external-constructor-callback-or-unresolved |
| `remove_workspace_folder` | `ManagementError::LastFolder` | [1881](../../src/management.rs#L1881) | external-constructor-callback-or-unresolved |
| `remove_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter()             .filter(&#124;(_, binding)&#124; binding.as_deref() == Some(folder_id.as_str()))             .map(&#124;(session_id, _)&#124; session_id)             .collect::<Vec<_>>` | [1883](../../src/management.rs#L1883) | receiver-type-required |
| `remove_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter()             .filter(&#124;(_, binding)&#124; binding.as_deref() == Some(folder_id.as_str()))             .map` | [1883](../../src/management.rs#L1883) | receiver-type-required |
| `remove_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter()             .filter` | [1883](../../src/management.rs#L1883) | receiver-type-required |
| `remove_workspace_folder` | `self             .workspace_session_bindings(workspace_id)?             .into_iter` | [1883](../../src/management.rs#L1883) | receiver-type-required |
| `remove_workspace_folder` | `self             .workspace_session_bindings` | [1883](../../src/management.rs#L1883) | [endpoint::management::ManagementStore::workspace_session_bindings](../../src/management.rs#L1948) |
| `remove_workspace_folder` | `binding.as_deref` | [1886](../../src/management.rs#L1886) | receiver-type-required |
| `remove_workspace_folder` | `Some` | [1886](../../src/management.rs#L1886), [1908](../../src/management.rs#L1908) | external-constructor-callback-or-unresolved |
| `remove_workspace_folder` | `folder_id.as_str` | [1886](../../src/management.rs#L1886) | receiver-type-required |
| `remove_workspace_folder` | `bound.is_empty` | [1889](../../src/management.rs#L1889) | receiver-type-required |
| `remove_workspace_folder` | `with_removed_folder(current, &folder_id, path).ok_or_else` | [1896](../../src/management.rs#L1896) | receiver-type-required |
| `remove_workspace_folder` | `with_removed_folder` | [1896](../../src/management.rs#L1896) | [endpoint::management::with_removed_folder](../../src/management.rs#L3512) |
| `remove_workspace_folder` | `ManagementError::CorruptOperation` | [1897](../../src/management.rs#L1897) | external-constructor-callback-or-unresolved |
| `remove_workspace_folder` | `"located folder vanished".to_owned` | [1897](../../src/management.rs#L1897) | receiver-type-required |
| `remove_workspace_folder` | `self.drive_folder_operation` | [1899](../../src/management.rs#L1899) | [endpoint::management::ManagementStore::drive_folder_operation](../../src/management.rs#L1938) |
| `resume_folder_operation` | `self.operation_path` | [1920](../../src/management.rs#L1920) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `resume_folder_operation` | `operation_path.exists` | [1921](../../src/management.rs#L1921) | receiver-type-required |
| `resume_folder_operation` | `Ok` | [1922](../../src/management.rs#L1922) | external-constructor-callback-or-unresolved |
| `resume_folder_operation` | `read_canonical::<OperationRecord>` | [1924](../../src/management.rs#L1924) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `resume_folder_operation` | `Err` | [1929](../../src/management.rs#L1929) | external-constructor-callback-or-unresolved |
| `resume_folder_operation` | `rpc_id.to_owned` | [1930](../../src/management.rs#L1930) | receiver-type-required |
| `resume_folder_operation` | `operation.to_owned` | [1931](../../src/management.rs#L1931) | receiver-type-required |
| `resume_folder_operation` | `self.drive_workspace_operation(&operation_path, record)             .map` | [1934](../../src/management.rs#L1934) | receiver-type-required |
| `resume_folder_operation` | `self.drive_workspace_operation` | [1934](../../src/management.rs#L1934) | [endpoint::management::ManagementStore::drive_workspace_operation](../../src/management.rs#L2311) |
| `drive_folder_operation` | `self.begin_workspace_operation` | [1942](../../src/management.rs#L1942) | [endpoint::management::ManagementStore::begin_workspace_operation](../../src/management.rs#L2254) |
| `drive_folder_operation` | `folder_response` | [1943](../../src/management.rs#L1943) | [endpoint::management::folder_response](../../src/management.rs#L3559) |
| `drive_folder_operation` | `self.drive_workspace_operation` | [1943](../../src/management.rs#L1943) | [endpoint::management::ManagementStore::drive_workspace_operation](../../src/management.rs#L2311) |
| `workspace_session_bindings` | `Vec::new` | [1952](../../src/management.rs#L1952), [1969](../../src/management.rs#L1969) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `fs::read_dir` | [1954](../../src/management.rs#L1954) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `self.storage_root.join` | [1954](../../src/management.rs#L1954) | receiver-type-required |
| `workspace_session_bindings` | `error.kind` | [1956](../../src/management.rs#L1956), [1978](../../src/management.rs#L1978) | receiver-type-required |
| `workspace_session_bindings` | `Err` | [1957](../../src/management.rs#L1957), [1979](../../src/management.rs#L1979) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `error.into` | [1957](../../src/management.rs#L1957), [1979](../../src/management.rs#L1979) | receiver-type-required |
| `workspace_session_bindings` | `entry.file_name().to_str().map` | [1961](../../src/management.rs#L1961) | receiver-type-required |
| `workspace_session_bindings` | `entry.file_name().to_str` | [1961](../../src/management.rs#L1961) | receiver-type-required |
| `workspace_session_bindings` | `entry.file_name` | [1961](../../src/management.rs#L1961) | receiver-type-required |
| `workspace_session_bindings` | `entry.file_type()?.is_dir` | [1964](../../src/management.rs#L1964) | receiver-type-required |
| `workspace_session_bindings` | `entry.file_type` | [1964](../../src/management.rs#L1964) | receiver-type-required |
| `workspace_session_bindings` | `crate::validate_session_id(&session_id).is_err` | [1964](../../src/management.rs#L1964) | receiver-type-required |
| `workspace_session_bindings` | `crate::validate_session_id` | [1964](../../src/management.rs#L1964) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `workspace_session_bindings` | `fs::File::open` | [1970](../../src/management.rs#L1970) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `entry.path().join` | [1970](../../src/management.rs#L1970) | receiver-type-required |
| `workspace_session_bindings` | `entry.path` | [1970](../../src/management.rs#L1970) | receiver-type-required |
| `workspace_session_bindings` | `std::io::BufRead::read_until` | [1972](../../src/management.rs#L1972) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `std::io::BufReader::new` | [1973](../../src/management.rs#L1973) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `scan_valid_prefix(&bytes, schema::READER_VERSION)                     .projection                     .ok_or_else` | [1981](../../src/management.rs#L1981) | receiver-type-required |
| `workspace_session_bindings` | `scan_valid_prefix` | [1981](../../src/management.rs#L1981) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `workspace_session_bindings` | `ManagementError::CorruptOperation` | [1984](../../src/management.rs#L1984), [1987](../../src/management.rs#L1987) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `"session ledger lacks genesis".to_owned` | [1984](../../src/management.rs#L1984), [1987](../../src/management.rs#L1987) | receiver-type-required |
| `workspace_session_bindings` | `projection.events.first().ok_or_else` | [1986](../../src/management.rs#L1986) | receiver-type-required |
| `workspace_session_bindings` | `projection.events.first` | [1986](../../src/management.rs#L1986) | receiver-type-required |
| `workspace_session_bindings` | `genesis.string_field` | [1989](../../src/management.rs#L1989), [1990](../../src/management.rs#L1990) | receiver-type-required |
| `workspace_session_bindings` | `Some` | [1989](../../src/management.rs#L1989) | external-constructor-callback-or-unresolved |
| `workspace_session_bindings` | `genesis.string_field("folder_binding").map` | [1990](../../src/management.rs#L1990) | receiver-type-required |
| `workspace_session_bindings` | `sessions.push` | [1991](../../src/management.rs#L1991), [2003](../../src/management.rs#L2003) | receiver-type-required |
| `workspace_session_bindings` | `self.operation_paths` | [1995](../../src/management.rs#L1995) | [endpoint::management::ManagementStore::operation_paths](../../src/management.rs#L3039) |
| `workspace_session_bindings` | `read_canonical::<OperationRecord>` | [1996](../../src/management.rs#L1996) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `workspace_session_bindings` | `session_create_intent` | [2000](../../src/management.rs#L2000) | [endpoint::management::session_create_intent](../../src/management.rs#L3860) |
| `workspace_session_bindings` | `(!intent.folder_binding.is_empty()).then_some` | [2002](../../src/management.rs#L2002) | receiver-type-required |
| `workspace_session_bindings` | `intent.folder_binding.is_empty` | [2002](../../src/management.rs#L2002) | receiver-type-required |
| `workspace_session_bindings` | `sessions.sort` | [2006](../../src/management.rs#L2006) | receiver-type-required |
| `workspace_session_bindings` | `sessions.dedup` | [2007](../../src/management.rs#L2007) | receiver-type-required |
| `workspace_session_bindings` | `Ok` | [2008](../../src/management.rs#L2008) | external-constructor-callback-or-unresolved |
| `archive_session` | `self.folder_move` | [2018](../../src/management.rs#L2018) | [endpoint::management::ManagementStore::folder_move](../../src/management.rs#L2178) |
| `archive_session` | `value             .get("archivedSessionIds")             .and_then(Value::as_array)             .map(&#124;values&#124; {                 values                     .iter()                     .map(&#124;value&#124; {                         value.as_str().map(str::to_owned).ok_or_else(&#124;&#124; {                             ManagementError::CorruptOperation(                                 "archive response contains a non-string id".to_owned(),                             )                         })                     })                     .collect()             })             .ok_or_else` | [2019](../../src/management.rs#L2019) | receiver-type-required |
| `archive_session` | `value             .get("archivedSessionIds")             .and_then(Value::as_array)             .map` | [2019](../../src/management.rs#L2019) | receiver-type-required |
| `archive_session` | `value             .get("archivedSessionIds")             .and_then` | [2019](../../src/management.rs#L2019) | receiver-type-required |
| `archive_session` | `value             .get` | [2019](../../src/management.rs#L2019) | receiver-type-required |
| `archive_session` | `values                     .iter()                     .map(&#124;value&#124; {                         value.as_str().map(str::to_owned).ok_or_else(&#124;&#124; {                             ManagementError::CorruptOperation(                                 "archive response contains a non-string id".to_owned(),                             )                         })                     })                     .collect` | [2023](../../src/management.rs#L2023) | receiver-type-required |
| `archive_session` | `values                     .iter()                     .map` | [2023](../../src/management.rs#L2023) | receiver-type-required |
| `archive_session` | `values                     .iter` | [2023](../../src/management.rs#L2023) | receiver-type-required |
| `archive_session` | `value.as_str().map(str::to_owned).ok_or_else` | [2026](../../src/management.rs#L2026) | receiver-type-required |
| `archive_session` | `value.as_str().map` | [2026](../../src/management.rs#L2026) | receiver-type-required |
| `archive_session` | `value.as_str` | [2026](../../src/management.rs#L2026) | receiver-type-required |
| `archive_session` | `ManagementError::CorruptOperation` | [2027](../../src/management.rs#L2027), [2035](../../src/management.rs#L2035) | external-constructor-callback-or-unresolved |
| `archive_session` | `"archive response contains a non-string id".to_owned` | [2028](../../src/management.rs#L2028) | receiver-type-required |
| `archive_session` | `"archive response lacks archivedSessionIds".to_owned` | [2036](../../src/management.rs#L2036) | receiver-type-required |
| `unarchive_session` | `self.folder_move` | [2049](../../src/management.rs#L2049) | [endpoint::management::ManagementStore::folder_move](../../src/management.rs#L2178) |
| `unarchive_session` | `value             .get("sessionId")             .and_then(Value::as_str)             .map(str::to_owned)             .ok_or_else` | [2050](../../src/management.rs#L2050) | receiver-type-required |
| `unarchive_session` | `value             .get("sessionId")             .and_then(Value::as_str)             .map` | [2050](../../src/management.rs#L2050) | receiver-type-required |
| `unarchive_session` | `value             .get("sessionId")             .and_then` | [2050](../../src/management.rs#L2050) | receiver-type-required |
| `unarchive_session` | `value             .get` | [2050](../../src/management.rs#L2050) | receiver-type-required |
| `unarchive_session` | `ManagementError::CorruptOperation` | [2055](../../src/management.rs#L2055) | external-constructor-callback-or-unresolved |
| `unarchive_session` | `"unarchive response lacks sessionId".to_owned` | [2055](../../src/management.rs#L2055) | receiver-type-required |
| `discard_session` | `crate::validate_session_id` | [2069](../../src/management.rs#L2069) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `discard_session` | `NamedLock::exclusive` | [2070](../../src/management.rs#L2070) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `discard_session` | `self.root.join` | [2070](../../src/management.rs#L2070) | receiver-type-required |
| `discard_session` | `self.operation_path` | [2071](../../src/management.rs#L2071) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `discard_session` | `operation_path.exists` | [2072](../../src/management.rs#L2072) | receiver-type-required |
| `discard_session` | `read_canonical::<OperationRecord>` | [2073](../../src/management.rs#L2073) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `discard_session` | `Err` | [2078](../../src/management.rs#L2078), [2086](../../src/management.rs#L2086), [2090](../../src/management.rs#L2090), [2093](../../src/management.rs#L2093), [2100](../../src/management.rs#L2100) | external-constructor-callback-or-unresolved |
| `discard_session` | `rpc_id.to_owned` | [2079](../../src/management.rs#L2079), [2106](../../src/management.rs#L2106) | receiver-type-required |
| `discard_session` | `"session.discard".to_owned` | [2080](../../src/management.rs#L2080), [2107](../../src/management.rs#L2107) | receiver-type-required |
| `discard_session` | `self.drive_discard` | [2083](../../src/management.rs#L2083), [2120](../../src/management.rs#L2120) | [endpoint::management::ManagementStore::drive_discard](../../src/management.rs#L2123) |
| `discard_session` | `self.storage_root.join("archive").join(session_id).is_dir` | [2085](../../src/management.rs#L2085) | receiver-type-required |
| `discard_session` | `self.storage_root.join("archive").join` | [2085](../../src/management.rs#L2085) | receiver-type-required |
| `discard_session` | `self.storage_root.join` | [2085](../../src/management.rs#L2085), [2088](../../src/management.rs#L2088) | receiver-type-required |
| `discard_session` | `ManagementError::SessionArchived` | [2086](../../src/management.rs#L2086) | external-constructor-callback-or-unresolved |
| `discard_session` | `session_id.to_owned` | [2086](../../src/management.rs#L2086), [2090](../../src/management.rs#L2090), [2093](../../src/management.rs#L2093), [2096](../../src/management.rs#L2096), [2100](../../src/management.rs#L2100), [2113](../../src/management.rs#L2113) | receiver-type-required |
| `discard_session` | `self.storage_root.join("threads").join` | [2088](../../src/management.rs#L2088) | receiver-type-required |
| `discard_session` | `source.is_dir` | [2089](../../src/management.rs#L2089) | receiver-type-required |
| `discard_session` | `ManagementError::SessionNotFound` | [2090](../../src/management.rs#L2090) | external-constructor-callback-or-unresolved |
| `discard_session` | `folder_is_ephemeral` | [2092](../../src/management.rs#L2092) | [endpoint::management::folder_is_ephemeral](../../src/management.rs#L3843) |
| `discard_session` | `ManagementError::SessionNotEphemeral` | [2093](../../src/management.rs#L2093) | external-constructor-callback-or-unresolved |
| `discard_session` | `DirectoryLock::try_exclusive(&source).map_err` | [2095](../../src/management.rs#L2095) | receiver-type-required |
| `discard_session` | `DirectoryLock::try_exclusive` | [2095](../../src/management.rs#L2095) | [store::platform::DirectoryLock::try_exclusive](../../../store/src/platform.rs#L66) |
| `discard_session` | `ManagementError::SessionRunning` | [2096](../../src/management.rs#L2096), [2100](../../src/management.rs#L2100) | external-constructor-callback-or-unresolved |
| `discard_session` | `ManagementError::Store` | [2097](../../src/management.rs#L2097) | external-constructor-callback-or-unresolved |
| `discard_session` | `ThreadStore::open(&self.storage_root)?.session_has_live_line_holder` | [2099](../../src/management.rs#L2099) | receiver-type-required |
| `discard_session` | `ThreadStore::open` | [2099](../../src/management.rs#L2099) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `discard_session` | `folder_reservation` | [2102](../../src/management.rs#L2102) | [endpoint::management::folder_reservation](../../src/management.rs#L4193) |
| `discard_session` | `drop` | [2103](../../src/management.rs#L2103) | external-constructor-callback-or-unresolved |
| `discard_session` | `request_sha256.to_owned` | [2108](../../src/management.rs#L2108) | receiver-type-required |
| `discard_session` | `started_at.to_owned` | [2110](../../src/management.rs#L2110) | receiver-type-required |
| `discard_session` | `to_ijson` | [2111](../../src/management.rs#L2111) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `discard_session` | `"discard".to_owned` | [2112](../../src/management.rs#L2112) | receiver-type-required |
| `discard_session` | `publish_canonical` | [2119](../../src/management.rs#L2119) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `drive_discard` | `discard_intent` | [2128](../../src/management.rs#L2128) | [endpoint::management::discard_intent](../../src/management.rs#L3831) |
| `drive_discard` | `Err` | [2130](../../src/management.rs#L2130), [2143](../../src/management.rs#L2143) | external-constructor-callback-or-unresolved |
| `drive_discard` | `ManagementError::CorruptOperation` | [2130](../../src/management.rs#L2130), [2143](../../src/management.rs#L2143), [2173](../../src/management.rs#L2173) | external-constructor-callback-or-unresolved |
| `drive_discard` | `"unsupported discard operation".to_owned` | [2131](../../src/management.rs#L2131) | receiver-type-required |
| `drive_discard` | `publish_canonical` | [2136](../../src/management.rs#L2136), [2160](../../src/management.rs#L2160), [2165](../../src/management.rs#L2165), [2170](../../src/management.rs#L2170) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `drive_discard` | `self.storage_root.join("threads").join` | [2139](../../src/management.rs#L2139) | receiver-type-required |
| `drive_discard` | `self.storage_root.join` | [2139](../../src/management.rs#L2139) | receiver-type-required |
| `drive_discard` | `source.is_dir` | [2140](../../src/management.rs#L2140) | receiver-type-required |
| `drive_discard` | `folder_reservation` | [2141](../../src/management.rs#L2141) | [endpoint::management::folder_reservation](../../src/management.rs#L4193) |
| `drive_discard` | `"discard reservation no longer matches target".to_owned` | [2144](../../src/management.rs#L2144) | receiver-type-required |
| `drive_discard` | `ThreadStore::open(&self.storage_root)?                     .discard_ephemeral(&intent.session_id)                     .map_err` | [2147](../../src/management.rs#L2147) | receiver-type-required |
| `drive_discard` | `ThreadStore::open(&self.storage_root)?                     .discard_ephemeral` | [2147](../../src/management.rs#L2147) | receiver-type-required |
| `drive_discard` | `ThreadStore::open` | [2147](../../src/management.rs#L2147) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `drive_discard` | `ManagementError::SessionRunning` | [2151](../../src/management.rs#L2151) | external-constructor-callback-or-unresolved |
| `drive_discard` | `intent.session_id.clone` | [2151](../../src/management.rs#L2151), [2154](../../src/management.rs#L2154) | receiver-type-required |
| `drive_discard` | `ManagementError::SessionNotEphemeral` | [2154](../../src/management.rs#L2154) | external-constructor-callback-or-unresolved |
| `drive_discard` | `ManagementError::Store` | [2156](../../src/management.rs#L2156) | external-constructor-callback-or-unresolved |
| `drive_discard` | `Some` | [2169](../../src/management.rs#L2169) | external-constructor-callback-or-unresolved |
| `drive_discard` | `to_ijson` | [2169](../../src/management.rs#L2169) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `drive_discard` | `record.response.ok_or_else` | [2172](../../src/management.rs#L2172) | receiver-type-required |
| `drive_discard` | `"complete discard lacks response".to_owned` | [2173](../../src/management.rs#L2173) | receiver-type-required |
| `drive_discard` | `fork_id_from_response` | [2175](../../src/management.rs#L2175) | [endpoint::management::fork_id_from_response](../../src/management.rs#L4119) |
| `drive_discard` | `serde_json::from_slice` | [2175](../../src/management.rs#L2175) | external-constructor-callback-or-unresolved |
| `drive_discard` | `response.canonical_bytes` | [2175](../../src/management.rs#L2175) | receiver-type-required |
| `folder_move` | `crate::validate_session_id` | [2186](../../src/management.rs#L2186) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `folder_move` | `NamedLock::exclusive` | [2187](../../src/management.rs#L2187) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `folder_move` | `self.root.join` | [2187](../../src/management.rs#L2187) | receiver-type-required |
| `folder_move` | `self.operation_path` | [2188](../../src/management.rs#L2188) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `folder_move` | `operation_path.exists` | [2194](../../src/management.rs#L2194) | receiver-type-required |
| `folder_move` | `read_canonical::<OperationRecord>` | [2195](../../src/management.rs#L2195) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `folder_move` | `Err` | [2200](../../src/management.rs#L2200), [2208](../../src/management.rs#L2208), [2217](../../src/management.rs#L2217), [2220](../../src/management.rs#L2220) | external-constructor-callback-or-unresolved |
| `folder_move` | `rpc_id.to_owned` | [2201](../../src/management.rs#L2201), [2229](../../src/management.rs#L2229) | receiver-type-required |
| `folder_move` | `requested_operation.to_owned` | [2202](../../src/management.rs#L2202) | receiver-type-required |
| `folder_move` | `self.drive_folder_move` | [2205](../../src/management.rs#L2205), [2251](../../src/management.rs#L2251) | [endpoint::management::ManagementStore::drive_folder_move](../../src/management.rs#L2969) |
| `folder_move` | `active_session_count` | [2207](../../src/management.rs#L2207) | [endpoint::management::active_session_count](../../src/management.rs#L4235) |
| `folder_move` | `self.storage_root.join(source_area).join` | [2215](../../src/management.rs#L2215) | receiver-type-required |
| `folder_move` | `self.storage_root.join` | [2215](../../src/management.rs#L2215) | receiver-type-required |
| `folder_move` | `source.is_dir` | [2216](../../src/management.rs#L2216) | receiver-type-required |
| `folder_move` | `ManagementError::SessionNotFound` | [2217](../../src/management.rs#L2217) | external-constructor-callback-or-unresolved |
| `folder_move` | `session_id.to_owned` | [2217](../../src/management.rs#L2217), [2220](../../src/management.rs#L2220), [2223](../../src/management.rs#L2223), [2237](../../src/management.rs#L2237) | receiver-type-required |
| `folder_move` | `folder_is_ephemeral` | [2219](../../src/management.rs#L2219) | [endpoint::management::folder_is_ephemeral](../../src/management.rs#L3843) |
| `folder_move` | `ManagementError::SessionEphemeral` | [2220](../../src/management.rs#L2220) | external-constructor-callback-or-unresolved |
| `folder_move` | `DirectoryLock::try_exclusive(&source).map_err` | [2222](../../src/management.rs#L2222) | receiver-type-required |
| `folder_move` | `DirectoryLock::try_exclusive` | [2222](../../src/management.rs#L2222) | [store::platform::DirectoryLock::try_exclusive](../../../store/src/platform.rs#L66) |
| `folder_move` | `ManagementError::SessionRunning` | [2223](../../src/management.rs#L2223) | external-constructor-callback-or-unresolved |
| `folder_move` | `ManagementError::Store` | [2224](../../src/management.rs#L2224) | external-constructor-callback-or-unresolved |
| `folder_move` | `folder_reservation` | [2226](../../src/management.rs#L2226) | [endpoint::management::folder_reservation](../../src/management.rs#L4193) |
| `folder_move` | `operation.to_owned` | [2230](../../src/management.rs#L2230) | receiver-type-required |
| `folder_move` | `request_sha256.to_owned` | [2231](../../src/management.rs#L2231) | receiver-type-required |
| `folder_move` | `started_at.to_owned` | [2233](../../src/management.rs#L2233) | receiver-type-required |
| `folder_move` | `to_ijson` | [2234](../../src/management.rs#L2234) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `folder_move` | `"folder-move".to_owned` | [2235](../../src/management.rs#L2235) | receiver-type-required |
| `folder_move` | `action.to_owned` | [2236](../../src/management.rs#L2236) | receiver-type-required |
| `folder_move` | `source.to_string_lossy().into_owned` | [2238](../../src/management.rs#L2238) | receiver-type-required |
| `folder_move` | `source.to_string_lossy` | [2238](../../src/management.rs#L2238) | receiver-type-required |
| `folder_move` | `self                     .storage_root                     .join(destination_area)                     .join(session_id)                     .to_string_lossy()                     .into_owned` | [2239](../../src/management.rs#L2239) | receiver-type-required |
| `folder_move` | `self                     .storage_root                     .join(destination_area)                     .join(session_id)                     .to_string_lossy` | [2239](../../src/management.rs#L2239) | receiver-type-required |
| `folder_move` | `self                     .storage_root                     .join(destination_area)                     .join` | [2239](../../src/management.rs#L2239) | receiver-type-required |
| `folder_move` | `self                     .storage_root                     .join` | [2239](../../src/management.rs#L2239) | receiver-type-required |
| `folder_move` | `publish_canonical` | [2250](../../src/management.rs#L2250) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `folder_move` | `Some` | [2251](../../src/management.rs#L2251) | external-constructor-callback-or-unresolved |
| `begin_workspace_operation` | `self.operation_path` | [2258](../../src/management.rs#L2258) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `begin_workspace_operation` | `operation_path.exists` | [2259](../../src/management.rs#L2259) | receiver-type-required |
| `begin_workspace_operation` | `read_canonical::<OperationRecord>` | [2260](../../src/management.rs#L2260) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `begin_workspace_operation` | `Err` | [2265](../../src/management.rs#L2265) | external-constructor-callback-or-unresolved |
| `begin_workspace_operation` | `write.rpc_id.to_owned` | [2266](../../src/management.rs#L2266), [2287](../../src/management.rs#L2287) | receiver-type-required |
| `begin_workspace_operation` | `write.operation.to_owned` | [2267](../../src/management.rs#L2267), [2288](../../src/management.rs#L2288) | receiver-type-required |
| `begin_workspace_operation` | `Ok` | [2270](../../src/management.rs#L2270), [2308](../../src/management.rs#L2308) | external-constructor-callback-or-unresolved |
| `begin_workspace_operation` | `canonical_line` | [2272](../../src/management.rs#L2272) | [endpoint::management::canonical_line](../../src/management.rs#L3773) |
| `begin_workspace_operation` | `hex_digest` | [2273](../../src/management.rs#L2273) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `begin_workspace_operation` | `self.remove_recordless_payload` | [2274](../../src/management.rs#L2274) | [endpoint::management::ManagementStore::remove_recordless_payload](../../src/management.rs#L2950) |
| `begin_workspace_operation` | `self.operation_payload_path` | [2275](../../src/management.rs#L2275) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `begin_workspace_operation` | `payload.join` | [2276](../../src/management.rs#L2276), [2280](../../src/management.rs#L2280) | receiver-type-required |
| `begin_workspace_operation` | `AtomicPublisher::replace` | [2277](../../src/management.rs#L2277) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `begin_workspace_operation` | `publish_canonical` | [2279](../../src/management.rs#L2279), [2307](../../src/management.rs#L2307) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `begin_workspace_operation` | `sync_operation_payload` | [2284](../../src/management.rs#L2284) | [endpoint::management::sync_operation_payload](../../src/management.rs#L3394) |
| `begin_workspace_operation` | `write.request_sha256.to_owned` | [2289](../../src/management.rs#L2289) | receiver-type-required |
| `begin_workspace_operation` | `write.started_at.to_owned` | [2291](../../src/management.rs#L2291) | receiver-type-required |
| `begin_workspace_operation` | `to_ijson` | [2292](../../src/management.rs#L2292) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `begin_workspace_operation` | `"workspace".to_owned` | [2293](../../src/management.rs#L2293) | receiver-type-required |
| `begin_workspace_operation` | `write.action.to_owned` | [2294](../../src/management.rs#L2294) | receiver-type-required |
| `begin_workspace_operation` | `write.config.id.clone` | [2295](../../src/management.rs#L2295) | receiver-type-required |
| `begin_workspace_operation` | `primary_path(write.config)?.to_owned` | [2296](../../src/management.rs#L2296) | receiver-type-required |
| `begin_workspace_operation` | `primary_path` | [2296](../../src/management.rs#L2296) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `begin_workspace_operation` | `write.config.name.clone` | [2297](../../src/management.rs#L2297) | receiver-type-required |
| `begin_workspace_operation` | `write.relocation.map` | [2299](../../src/management.rs#L2299), [2300](../../src/management.rs#L2300), [2301](../../src/management.rs#L2301) | receiver-type-required |
| `begin_workspace_operation` | `value.0.to_owned` | [2299](../../src/management.rs#L2299), [2302](../../src/management.rs#L2302) | receiver-type-required |
| `begin_workspace_operation` | `value.1.to_owned` | [2300](../../src/management.rs#L2300), [2303](../../src/management.rs#L2303) | receiver-type-required |
| `begin_workspace_operation` | `value.2.to_owned` | [2301](../../src/management.rs#L2301) | receiver-type-required |
| `begin_workspace_operation` | `write.folder.map` | [2302](../../src/management.rs#L2302), [2303](../../src/management.rs#L2303) | receiver-type-required |
| `drive_workspace_operation` | `Err` | [2317](../../src/management.rs#L2317), [2332](../../src/management.rs#L2332), [2339](../../src/management.rs#L2339), [2356](../../src/management.rs#L2356) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `ManagementError::CorruptOperation` | [2317](../../src/management.rs#L2317), [2339](../../src/management.rs#L2339), [2356](../../src/management.rs#L2356), [2413](../../src/management.rs#L2413) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `"unsupported management operation record".to_owned` | [2318](../../src/management.rs#L2318) | receiver-type-required |
| `drive_workspace_operation` | `workspace_intent` | [2321](../../src/management.rs#L2321) | [endpoint::management::workspace_intent](../../src/management.rs#L3799) |
| `drive_workspace_operation` | `self             .operation_payload_path(&record.rpc_id)             .join` | [2322](../../src/management.rs#L2322) | receiver-type-required |
| `drive_workspace_operation` | `self             .operation_payload_path` | [2322](../../src/management.rs#L2322) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `drive_workspace_operation` | `fs::read` | [2325](../../src/management.rs#L2325) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `WorkspaceConfig::decode` | [2326](../../src/management.rs#L2326) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `error.kind` | [2327](../../src/management.rs#L2327) | receiver-type-required |
| `drive_workspace_operation` | `self.rebuild_candidate` | [2328](../../src/management.rs#L2328) | [endpoint::management::ManagementStore::rebuild_candidate](../../src/management.rs#L2464) |
| `drive_workspace_operation` | `AtomicPublisher::replace` | [2329](../../src/management.rs#L2329) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `drive_workspace_operation` | `canonical_line` | [2329](../../src/management.rs#L2329), [2337](../../src/management.rs#L2337) | [endpoint::management::canonical_line](../../src/management.rs#L3773) |
| `drive_workspace_operation` | `error.into` | [2332](../../src/management.rs#L2332) | receiver-type-required |
| `drive_workspace_operation` | `primary_path` | [2336](../../src/management.rs#L2336), [2371](../../src/management.rs#L2371), [2379](../../src/management.rs#L2379) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `drive_workspace_operation` | `hex_digest` | [2337](../../src/management.rs#L2337) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `drive_workspace_operation` | `"staged workspace does not match immutable intent".to_owned` | [2340](../../src/management.rs#L2340) | receiver-type-required |
| `drive_workspace_operation` | `publish_canonical` | [2345](../../src/management.rs#L2345), [2362](../../src/management.rs#L2362), [2385](../../src/management.rs#L2385), [2387](../../src/management.rs#L2387), [2410](../../src/management.rs#L2410) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `drive_workspace_operation` | `self.workspace_configs` | [2348](../../src/management.rs#L2348) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `drive_workspace_operation` | `configs.get` | [2349](../../src/management.rs#L2349) | receiver-type-required |
| `drive_workspace_operation` | `self.config.publish_workspace` | [2352](../../src/management.rs#L2352), [2354](../../src/management.rs#L2354) | receiver-type-required |
| `drive_workspace_operation` | `"workspace publication does not match prepared revision".to_owned` | [2357](../../src/management.rs#L2357) | receiver-type-required |
| `drive_workspace_operation` | `self.operation_created` | [2365](../../src/management.rs#L2365), [2401](../../src/management.rs#L2401) | [endpoint::management::ManagementStore::operation_created](../../src/management.rs#L2430) |
| `drive_workspace_operation` | `self.read_metadata_optional` | [2366](../../src/management.rs#L2366) | [endpoint::management::ManagementStore::read_metadata_optional](../../src/management.rs#L3090) |
| `drive_workspace_operation` | `config.id.clone` | [2370](../../src/management.rs#L2370), [2378](../../src/management.rs#L2378) | receiver-type-required |
| `drive_workspace_operation` | `primary_path(&config)?.to_owned` | [2371](../../src/management.rs#L2371), [2379](../../src/management.rs#L2379) | receiver-type-required |
| `drive_workspace_operation` | `record.started_at.clone` | [2373](../../src/management.rs#L2373), [2380](../../src/management.rs#L2380), [2381](../../src/management.rs#L2381) | receiver-type-required |
| `drive_workspace_operation` | `intent.config_digest.clone` | [2374](../../src/management.rs#L2374), [2382](../../src/management.rs#L2382) | receiver-type-required |
| `drive_workspace_operation` | `self.metadata_path` | [2385](../../src/management.rs#L2385) | [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) |
| `drive_workspace_operation` | `self.read_metadata` | [2390](../../src/management.rs#L2390) | [endpoint::management::ManagementStore::read_metadata](../../src/management.rs#L3082) |
| `drive_workspace_operation` | `crate::NativeEndpoint::open(&self.storage_root)?                 .list_sessions` | [2391](../../src/management.rs#L2391) | receiver-type-required |
| `drive_workspace_operation` | `crate::NativeEndpoint::open` | [2391](../../src/management.rs#L2391) | [endpoint::service::NativeEndpoint::open](../../src/service.rs#L263) |
| `drive_workspace_operation` | `std::collections::HashSet::new` | [2392](../../src/management.rs#L2392) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `sessions                 .iter()                 .filter(&#124;session&#124; !session.archived && session.workspace_id == config.id)                 .collect::<Vec<_>>` | [2393](../../src/management.rs#L2393) | receiver-type-required |
| `drive_workspace_operation` | `sessions                 .iter()                 .filter` | [2393](../../src/management.rs#L2393) | receiver-type-required |
| `drive_workspace_operation` | `sessions                 .iter` | [2393](../../src/management.rs#L2393) | receiver-type-required |
| `drive_workspace_operation` | `workspace_view` | [2397](../../src/management.rs#L2397) | [endpoint::management::workspace_view](../../src/management.rs#L3693) |
| `drive_workspace_operation` | `intent.folder_id.is_some` | [2403](../../src/management.rs#L2403) | receiver-type-required |
| `drive_workspace_operation` | `Some` | [2409](../../src/management.rs#L2409) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `IJsonValue::parse` | [2409](../../src/management.rs#L2409) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `drive_workspace_operation` | `serde_json::to_vec` | [2409](../../src/management.rs#L2409) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `record.response.ok_or_else` | [2412](../../src/management.rs#L2412) | receiver-type-required |
| `drive_workspace_operation` | `"complete operation lacks response".to_owned` | [2413](../../src/management.rs#L2413) | receiver-type-required |
| `drive_workspace_operation` | `serde_json::from_slice(&response.canonical_bytes()?).map_err` | [2415](../../src/management.rs#L2415) | receiver-type-required |
| `drive_workspace_operation` | `serde_json::from_slice` | [2415](../../src/management.rs#L2415) | external-constructor-callback-or-unresolved |
| `drive_workspace_operation` | `response.canonical_bytes` | [2415](../../src/management.rs#L2415) | receiver-type-required |
| `workspace_configs` | `BTreeMap::new` | [2419](../../src/management.rs#L2419) | external-constructor-callback-or-unresolved |
| `workspace_configs` | `self.config.workspaces` | [2420](../../src/management.rs#L2420) | receiver-type-required |
| `workspace_configs` | `values.insert(config.id.clone(), config).is_some` | [2421](../../src/management.rs#L2421) | receiver-type-required |
| `workspace_configs` | `values.insert` | [2421](../../src/management.rs#L2421) | receiver-type-required |
| `workspace_configs` | `config.id.clone` | [2421](../../src/management.rs#L2421) | receiver-type-required |
| `workspace_configs` | `Err` | [2422](../../src/management.rs#L2422) | external-constructor-callback-or-unresolved |
| `workspace_configs` | `ManagementError::CorruptOperation` | [2422](../../src/management.rs#L2422) | external-constructor-callback-or-unresolved |
| `workspace_configs` | `"duplicate workspace id".to_owned` | [2423](../../src/management.rs#L2423) | receiver-type-required |
| `workspace_configs` | `Ok` | [2427](../../src/management.rs#L2427) | external-constructor-callback-or-unresolved |
| `operation_created` | `Ok` | [2436](../../src/management.rs#L2436) | external-constructor-callback-or-unresolved |
| `operation_created` | `self             .operation_payload_path(&record.rpc_id)             .join` | [2438](../../src/management.rs#L2438) | receiver-type-required |
| `operation_created` | `self             .operation_payload_path` | [2438](../../src/management.rs#L2438) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `operation_created` | `fs::read` | [2441](../../src/management.rs#L2441) | external-constructor-callback-or-unresolved |
| `operation_created` | `decode_canonical::<Value>` | [2442](../../src/management.rs#L2442) | [endpoint::management::decode_canonical](../../src/management.rs#L3752) |
| `operation_created` | `error.kind` | [2443](../../src/management.rs#L2443) | receiver-type-required |
| `operation_created` | `self.workspace_configs()?.get(&intent.workspace_id).cloned` | [2444](../../src/management.rs#L2444) | receiver-type-required |
| `operation_created` | `self.workspace_configs()?.get` | [2444](../../src/management.rs#L2444) | receiver-type-required |
| `operation_created` | `self.workspace_configs` | [2444](../../src/management.rs#L2444) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `operation_created` | `existing.is_none` | [2445](../../src/management.rs#L2445) | receiver-type-required |
| `operation_created` | `publish_canonical` | [2447](../../src/management.rs#L2447) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `operation_created` | `Err` | [2450](../../src/management.rs#L2450) | external-constructor-callback-or-unresolved |
| `operation_created` | `error.into` | [2450](../../src/management.rs#L2450) | receiver-type-required |
| `operation_created` | `value             .as_object()             .filter(&#124;object&#124; object.len() == 1)             .and_then(&#124;object&#124; object.get("created"))             .and_then(Value::as_bool)             .ok_or_else` | [2452](../../src/management.rs#L2452) | receiver-type-required |
| `operation_created` | `value             .as_object()             .filter(&#124;object&#124; object.len() == 1)             .and_then(&#124;object&#124; object.get("created"))             .and_then` | [2452](../../src/management.rs#L2452) | receiver-type-required |
| `operation_created` | `value             .as_object()             .filter(&#124;object&#124; object.len() == 1)             .and_then` | [2452](../../src/management.rs#L2452) | receiver-type-required |
| `operation_created` | `value             .as_object()             .filter` | [2452](../../src/management.rs#L2452) | receiver-type-required |
| `operation_created` | `value             .as_object` | [2452](../../src/management.rs#L2452) | receiver-type-required |
| `operation_created` | `object.len` | [2454](../../src/management.rs#L2454) | receiver-type-required |
| `operation_created` | `object.get` | [2455](../../src/management.rs#L2455) | receiver-type-required |
| `operation_created` | `ManagementError::CorruptOperation` | [2458](../../src/management.rs#L2458) | external-constructor-callback-or-unresolved |
| `operation_created` | `"workspace create stage lacks exact created decision".to_owned` | [2459](../../src/management.rs#L2459) | receiver-type-required |
| `rebuild_candidate` | `intent.workspace_id.clone` | [2472](../../src/management.rs#L2472) | receiver-type-required |
| `rebuild_candidate` | `intent.title.clone` | [2473](../../src/management.rs#L2473), [2499](../../src/management.rs#L2499) | receiver-type-required |
| `rebuild_candidate` | `Vec::new` | [2474](../../src/management.rs#L2474) | external-constructor-callback-or-unresolved |
| `rebuild_candidate` | `self                 .workspace_configs()?                 .remove(&intent.workspace_id)                 .ok_or_else` | [2482](../../src/management.rs#L2482), [2506](../../src/management.rs#L2506), [2549](../../src/management.rs#L2549) | receiver-type-required |
| `rebuild_candidate` | `self                 .workspace_configs()?                 .remove` | [2482](../../src/management.rs#L2482), [2506](../../src/management.rs#L2506), [2549](../../src/management.rs#L2549) | receiver-type-required |
| `rebuild_candidate` | `self                 .workspace_configs` | [2482](../../src/management.rs#L2482), [2506](../../src/management.rs#L2506), [2549](../../src/management.rs#L2549) | [endpoint::management::ManagementStore::workspace_configs](../../src/management.rs#L2418) |
| `rebuild_candidate` | `ManagementError::CorruptOperation` | [2486](../../src/management.rs#L2486), [2510](../../src/management.rs#L2510), [2518](../../src/management.rs#L2518), [2521](../../src/management.rs#L2521), [2526](../../src/management.rs#L2526), [2535](../../src/management.rs#L2535), [2553](../../src/management.rs#L2553), [2562](../../src/management.rs#L2562), [2570](../../src/management.rs#L2570), [2577](../../src/management.rs#L2577), [2582](../../src/management.rs#L2582) | external-constructor-callback-or-unresolved |
| `rebuild_candidate` | `"rename recovery cannot find source workspace".to_owned` | [2487](../../src/management.rs#L2487) | receiver-type-required |
| `rebuild_candidate` | `hex_digest` | [2491](../../src/management.rs#L2491), [2514](../../src/management.rs#L2514), [2557](../../src/management.rs#L2557), [2581](../../src/management.rs#L2581) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `rebuild_candidate` | `canonical_line` | [2491](../../src/management.rs#L2491), [2514](../../src/management.rs#L2514), [2557](../../src/management.rs#L2557), [2581](../../src/management.rs#L2581) | [endpoint::management::canonical_line](../../src/management.rs#L3773) |
| `rebuild_candidate` | `"relocate recovery cannot find source workspace".to_owned` | [2511](../../src/management.rs#L2511) | receiver-type-required |
| `rebuild_candidate` | `intent.relocate_folder_id.as_deref().ok_or_else` | [2517](../../src/management.rs#L2517) | receiver-type-required |
| `rebuild_candidate` | `intent.relocate_folder_id.as_deref` | [2517](../../src/management.rs#L2517) | receiver-type-required |
| `rebuild_candidate` | `"relocate intent lacks folder id".to_owned` | [2518](../../src/management.rs#L2518) | receiver-type-required |
| `rebuild_candidate` | `intent.previous_path.as_deref().ok_or_else` | [2520](../../src/management.rs#L2520) | receiver-type-required |
| `rebuild_candidate` | `intent.previous_path.as_deref` | [2520](../../src/management.rs#L2520) | receiver-type-required |
| `rebuild_candidate` | `"relocate intent lacks previous path".to_owned` | [2522](../../src/management.rs#L2522) | receiver-type-required |
| `rebuild_candidate` | `intent.relocated_path.as_deref().ok_or_else` | [2525](../../src/management.rs#L2525) | receiver-type-required |
| `rebuild_candidate` | `intent.relocated_path.as_deref` | [2525](../../src/management.rs#L2525) | receiver-type-required |
| `rebuild_candidate` | `"relocate intent lacks destination path".to_owned` | [2527](../../src/management.rs#L2527) | receiver-type-required |
| `rebuild_candidate` | `current                     .folders                     .iter_mut()                     .find(&#124;folder&#124; folder.id == folder_id && folder.path == previous_path)                     .ok_or_else` | [2530](../../src/management.rs#L2530) | receiver-type-required |
| `rebuild_candidate` | `current                     .folders                     .iter_mut()                     .find` | [2530](../../src/management.rs#L2530) | receiver-type-required |
| `rebuild_candidate` | `current                     .folders                     .iter_mut` | [2530](../../src/management.rs#L2530) | receiver-type-required |
| `rebuild_candidate` | `"relocate recovery source no longer matches intent".to_owned` | [2536](../../src/management.rs#L2536) | receiver-type-required |
| `rebuild_candidate` | `relocated_path.to_owned` | [2539](../../src/management.rs#L2539), [2542](../../src/management.rs#L2542) | receiver-type-required |
| `rebuild_candidate` | `"folder recovery cannot find source workspace".to_owned` | [2554](../../src/management.rs#L2554) | receiver-type-required |
| `rebuild_candidate` | `Err` | [2562](../../src/management.rs#L2562), [2577](../../src/management.rs#L2577), [2582](../../src/management.rs#L2582) | external-constructor-callback-or-unresolved |
| `rebuild_candidate` | `"folder intent lacks its folder".to_owned` | [2563](../../src/management.rs#L2563) | receiver-type-required |
| `rebuild_candidate` | `with_added_folder` | [2567](../../src/management.rs#L2567) | [endpoint::management::with_added_folder](../../src/management.rs#L3488) |
| `rebuild_candidate` | `with_removed_folder(&current, folder_id, folder_path).ok_or_else` | [2569](../../src/management.rs#L2569) | receiver-type-required |
| `rebuild_candidate` | `with_removed_folder` | [2569](../../src/management.rs#L2569) | [endpoint::management::with_removed_folder](../../src/management.rs#L3512) |
| `rebuild_candidate` | `"folder removal recovery source no longer matches intent".to_owned` | [2571](../../src/management.rs#L2571) | receiver-type-required |
| `rebuild_candidate` | `"unknown workspace action".to_owned` | [2578](../../src/management.rs#L2578) | receiver-type-required |
| `rebuild_candidate` | `"rebuilt candidate does not match prepared digest".to_owned` | [2583](../../src/management.rs#L2583) | receiver-type-required |
| `rebuild_candidate` | `Ok` | [2586](../../src/management.rs#L2586) | external-constructor-callback-or-unresolved |
| `drive_session_create` | `session_create_intent` | [2594](../../src/management.rs#L2594) | [endpoint::management::session_create_intent](../../src/management.rs#L3860) |
| `drive_session_create` | `Err` | [2596](../../src/management.rs#L2596), [2616](../../src/management.rs#L2616) | external-constructor-callback-or-unresolved |
| `drive_session_create` | `ManagementError::CorruptOperation` | [2596](../../src/management.rs#L2596), [2616](../../src/management.rs#L2616), [2657](../../src/management.rs#L2657) | external-constructor-callback-or-unresolved |
| `drive_session_create` | `"unsupported session-create operation".to_owned` | [2597](../../src/management.rs#L2597) | receiver-type-required |
| `drive_session_create` | `self.operation_payload_path` | [2600](../../src/management.rs#L2600) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `drive_session_create` | `AssetStore::new` | [2601](../../src/management.rs#L2601) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `drive_session_create` | `payload.join` | [2601](../../src/management.rs#L2601), [2620](../../src/management.rs#L2620) | receiver-type-required |
| `drive_session_create` | `asset_store.read_verified` | [2602](../../src/management.rs#L2602), [2603](../../src/management.rs#L2603) | receiver-type-required |
| `drive_session_create` | `profile::ConfigSnapshot::decode` | [2604](../../src/management.rs#L2604) | [profile::config::ConfigSnapshot::decode](../../../profile/src/config.rs#L293) |
| `drive_session_create` | `profile::InstructionSnapshot::decode` | [2605](../../src/management.rs#L2605) | [profile::instruction::InstructionSnapshot::decode](../../../profile/src/instruction.rs#L180) |
| `drive_session_create` | `instruction.validate_against_config` | [2606](../../src/management.rs#L2606) | receiver-type-required |
| `drive_session_create` | `execution_path_from_snapshot` | [2608](../../src/management.rs#L2608) | [endpoint::management::execution_path_from_snapshot](../../src/management.rs#L3133) |
| `drive_session_create` | `folder_binding_matches_snapshot` | [2609](../../src/management.rs#L2609) | [endpoint::management::folder_binding_matches_snapshot](../../src/management.rs#L3139) |
| `drive_session_create` | `config.workspace.folder_binding.as_deref` | [2610](../../src/management.rs#L2610) | receiver-type-required |
| `drive_session_create` | `"session-create snapshots disagree with immutable intent".to_owned` | [2617](../../src/management.rs#L2617) | receiver-type-required |
| `drive_session_create` | `read_canonical_event` | [2620](../../src/management.rs#L2620) | [endpoint::management::read_canonical_event](../../src/management.rs#L3311) |
| `drive_session_create` | `verify_session_create_genesis` | [2621](../../src/management.rs#L2621) | [endpoint::management::verify_session_create_genesis](../../src/management.rs#L3322) |
| `drive_session_create` | `genesis_preset_defaults` | [2622](../../src/management.rs#L2622) | [endpoint::management::genesis_preset_defaults](../../src/management.rs#L3252) |
| `drive_session_create` | `publish_canonical` | [2626](../../src/management.rs#L2626), [2635](../../src/management.rs#L2635), [2646](../../src/management.rs#L2646), [2648](../../src/management.rs#L2648), [2654](../../src/management.rs#L2654) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `drive_session_create` | `ThreadStore::open(&self.storage_root)?.create_thread_with_assets` | [2629](../../src/management.rs#L2629) | receiver-type-required |
| `drive_session_create` | `ThreadStore::open` | [2629](../../src/management.rs#L2629) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `drive_session_create` | `self.apply_preset_defaults` | [2639](../../src/management.rs#L2639) | [endpoint::management::ManagementStore::apply_preset_defaults](../../src/management.rs#L1289) |
| `drive_session_create` | `self.storage_root.join("threads").join` | [2640](../../src/management.rs#L2640) | receiver-type-required |
| `drive_session_create` | `self.storage_root.join` | [2640](../../src/management.rs#L2640) | receiver-type-required |
| `drive_session_create` | `self.read_metadata` | [2644](../../src/management.rs#L2644) | [endpoint::management::ManagementStore::read_metadata](../../src/management.rs#L3082) |
| `drive_session_create` | `record.started_at.clone` | [2645](../../src/management.rs#L2645) | receiver-type-required |
| `drive_session_create` | `self.metadata_path` | [2646](../../src/management.rs#L2646) | [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) |
| `drive_session_create` | `Some` | [2653](../../src/management.rs#L2653) | external-constructor-callback-or-unresolved |
| `drive_session_create` | `to_ijson` | [2653](../../src/management.rs#L2653) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `drive_session_create` | `record.response.ok_or_else` | [2656](../../src/management.rs#L2656) | receiver-type-required |
| `drive_session_create` | `"complete session-create operation lacks response".to_owned` | [2658](../../src/management.rs#L2658) | receiver-type-required |
| `drive_session_create` | `serde_json::from_slice(&response.canonical_bytes()?).map_err` | [2661](../../src/management.rs#L2661) | receiver-type-required |
| `drive_session_create` | `serde_json::from_slice` | [2661](../../src/management.rs#L2661) | external-constructor-callback-or-unresolved |
| `drive_session_create` | `response.canonical_bytes` | [2661](../../src/management.rs#L2661) | receiver-type-required |
| `session_id_from_response` | `response             .as_object()             .filter(&#124;object&#124; object.len() == 1)             .and_then(&#124;object&#124; object.get("sessionId"))             .and_then(Value::as_str)             .map(str::to_owned)             .ok_or_else` | [2665](../../src/management.rs#L2665) | receiver-type-required |
| `session_id_from_response` | `response             .as_object()             .filter(&#124;object&#124; object.len() == 1)             .and_then(&#124;object&#124; object.get("sessionId"))             .and_then(Value::as_str)             .map` | [2665](../../src/management.rs#L2665) | receiver-type-required |
| `session_id_from_response` | `response             .as_object()             .filter(&#124;object&#124; object.len() == 1)             .and_then(&#124;object&#124; object.get("sessionId"))             .and_then` | [2665](../../src/management.rs#L2665) | receiver-type-required |
| `session_id_from_response` | `response             .as_object()             .filter(&#124;object&#124; object.len() == 1)             .and_then` | [2665](../../src/management.rs#L2665) | receiver-type-required |
| `session_id_from_response` | `response             .as_object()             .filter` | [2665](../../src/management.rs#L2665) | receiver-type-required |
| `session_id_from_response` | `response             .as_object` | [2665](../../src/management.rs#L2665) | receiver-type-required |
| `session_id_from_response` | `object.len` | [2667](../../src/management.rs#L2667) | receiver-type-required |
| `session_id_from_response` | `object.get` | [2668](../../src/management.rs#L2668) | receiver-type-required |
| `session_id_from_response` | `ManagementError::CorruptOperation` | [2672](../../src/management.rs#L2672) | external-constructor-callback-or-unresolved |
| `session_id_from_response` | `"session-create response lacks exact sessionId".to_owned` | [2673](../../src/management.rs#L2673) | receiver-type-required |
| `drive_select_model` | `select_model_intent` | [2683](../../src/management.rs#L2683) | [endpoint::management::select_model_intent](../../src/management.rs#L3885) |
| `drive_select_model` | `Err` | [2685](../../src/management.rs#L2685), [2699](../../src/management.rs#L2699) | external-constructor-callback-or-unresolved |
| `drive_select_model` | `ManagementError::CorruptOperation` | [2685](../../src/management.rs#L2685), [2699](../../src/management.rs#L2699), [2737](../../src/management.rs#L2737) | external-constructor-callback-or-unresolved |
| `drive_select_model` | `"unsupported select-model operation".to_owned` | [2686](../../src/management.rs#L2686) | receiver-type-required |
| `drive_select_model` | `self.operation_payload_path` | [2689](../../src/management.rs#L2689) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `drive_select_model` | `store::session_settings_path` | [2692](../../src/management.rs#L2692) | [store::management_root::session_settings_path](../../../store/src/management_root.rs#L57) |
| `drive_select_model` | `profile::SessionSettings::decode` | [2693](../../src/management.rs#L2693) | external-constructor-callback-or-unresolved |
| `drive_select_model` | `fs::read` | [2693](../../src/management.rs#L2693) | external-constructor-callback-or-unresolved |
| `drive_select_model` | `"select-model candidate disagrees with immutable intent".to_owned` | [2700](../../src/management.rs#L2700) | receiver-type-required |
| `drive_select_model` | `publish_canonical` | [2705](../../src/management.rs#L2705), [2712](../../src/management.rs#L2712), [2719](../../src/management.rs#L2719), [2721](../../src/management.rs#L2721), [2734](../../src/management.rs#L2734) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `drive_select_model` | `active_session_folder` | [2708](../../src/management.rs#L2708), [2715](../../src/management.rs#L2715) | [endpoint::management::active_session_folder](../../src/management.rs#L4169) |
| `drive_select_model` | `self.config                 .publish_session_settings` | [2709](../../src/management.rs#L2709) | receiver-type-required |
| `drive_select_model` | `folder_workspace` | [2716](../../src/management.rs#L2716) | [endpoint::management::folder_workspace](../../src/management.rs#L4223) |
| `drive_select_model` | `self.read_metadata` | [2717](../../src/management.rs#L2717) | [endpoint::management::ManagementStore::read_metadata](../../src/management.rs#L3082) |
| `drive_select_model` | `record.started_at.clone` | [2718](../../src/management.rs#L2718) | receiver-type-required |
| `drive_select_model` | `self.metadata_path` | [2719](../../src/management.rs#L2719) | [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) |
| `drive_select_model` | `Value::String` | [2729](../../src/management.rs#L2729) | external-constructor-callback-or-unresolved |
| `drive_select_model` | `Some` | [2733](../../src/management.rs#L2733) | external-constructor-callback-or-unresolved |
| `drive_select_model` | `to_ijson` | [2733](../../src/management.rs#L2733) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `drive_select_model` | `record.response.ok_or_else` | [2736](../../src/management.rs#L2736) | receiver-type-required |
| `drive_select_model` | `"complete select-model operation lacks response".to_owned` | [2738](../../src/management.rs#L2738) | receiver-type-required |
| `drive_select_model` | `serde_json::from_slice(&response.canonical_bytes()?).map_err` | [2741](../../src/management.rs#L2741) | receiver-type-required |
| `drive_select_model` | `serde_json::from_slice` | [2741](../../src/management.rs#L2741) | external-constructor-callback-or-unresolved |
| `drive_select_model` | `response.canonical_bytes` | [2741](../../src/management.rs#L2741) | receiver-type-required |
| `fork_was_quarantined` | `fork_intent` | [2745](../../src/management.rs#L2745) | [endpoint::management::fork_intent](../../src/management.rs#L3905) |
| `fork_was_quarantined` | `Ok` | [2750](../../src/management.rs#L2750), [2758](../../src/management.rs#L2758), [2761](../../src/management.rs#L2761) | external-constructor-callback-or-unresolved |
| `fork_was_quarantined` | `self             .storage_root             .join(".rewrite-quarantine")             .join(&intent.rewrite_op_id)             .join` | [2752](../../src/management.rs#L2752) | receiver-type-required |
| `fork_was_quarantined` | `self             .storage_root             .join(".rewrite-quarantine")             .join` | [2752](../../src/management.rs#L2752) | receiver-type-required |
| `fork_was_quarantined` | `self             .storage_root             .join` | [2752](../../src/management.rs#L2752) | receiver-type-required |
| `fork_was_quarantined` | `path.is_file` | [2757](../../src/management.rs#L2757) | receiver-type-required |
| `fork_was_quarantined` | `store::RewriteOperation::decode_canonical` | [2760](../../src/management.rs#L2760) | [store::rewrite::RewriteOperation::decode_canonical](../../../store/src/rewrite.rs#L107) |
| `fork_was_quarantined` | `fs::read` | [2760](../../src/management.rs#L2760) | external-constructor-callback-or-unresolved |
| `fork_was_quarantined` | `self                 .storage_root                 .join("threads")                 .join(&intent.dest)                 .exists` | [2766](../../src/management.rs#L2766) | receiver-type-required |
| `fork_was_quarantined` | `self                 .storage_root                 .join("threads")                 .join` | [2766](../../src/management.rs#L2766) | receiver-type-required |
| `fork_was_quarantined` | `self                 .storage_root                 .join` | [2766](../../src/management.rs#L2766) | receiver-type-required |
| `drive_fork` | `fork_intent` | [2778](../../src/management.rs#L2778) | [endpoint::management::fork_intent](../../src/management.rs#L3905) |
| `drive_fork` | `Err` | [2780](../../src/management.rs#L2780), [2785](../../src/management.rs#L2785), [2827](../../src/management.rs#L2827) | external-constructor-callback-or-unresolved |
| `drive_fork` | `ManagementError::CorruptOperation` | [2780](../../src/management.rs#L2780), [2785](../../src/management.rs#L2785), [2827](../../src/management.rs#L2827), [2859](../../src/management.rs#L2859) | external-constructor-callback-or-unresolved |
| `drive_fork` | `"unsupported fork operation".to_owned` | [2781](../../src/management.rs#L2781) | receiver-type-required |
| `drive_fork` | `self.fork_was_quarantined` | [2784](../../src/management.rs#L2784) | [endpoint::management::ManagementStore::fork_was_quarantined](../../src/management.rs#L2744) |
| `drive_fork` | `"fork could not recover its assets; operation preserved in .rewrite-quarantine"                     .to_owned` | [2786](../../src/management.rs#L2786) | receiver-type-required |
| `drive_fork` | `self             .storage_root             .join("staging")             .join` | [2790](../../src/management.rs#L2790) | receiver-type-required |
| `drive_fork` | `self             .storage_root             .join` | [2790](../../src/management.rs#L2790) | receiver-type-required |
| `drive_fork` | `self.storage_root.join("threads").join` | [2794](../../src/management.rs#L2794) | receiver-type-required |
| `drive_fork` | `self.storage_root.join` | [2794](../../src/management.rs#L2794) | receiver-type-required |
| `drive_fork` | `rewrite_stage.is_dir` | [2796](../../src/management.rs#L2796) | receiver-type-required |
| `drive_fork` | `destination.is_dir` | [2796](../../src/management.rs#L2796), [2826](../../src/management.rs#L2826) | receiver-type-required |
| `drive_fork` | `intent.principal.clone` | [2798](../../src/management.rs#L2798) | receiver-type-required |
| `drive_fork` | `ORIGIN_CLIENT.to_owned` | [2799](../../src/management.rs#L2799) | receiver-type-required |
| `drive_fork` | `intent.dest.clone` | [2800](../../src/management.rs#L2800), [2809](../../src/management.rs#L2809) | receiver-type-required |
| `drive_fork` | `"session.fork".to_owned` | [2801](../../src/management.rs#L2801) | receiver-type-required |
| `drive_fork` | `record.rpc_id.clone` | [2802](../../src/management.rs#L2802) | receiver-type-required |
| `drive_fork` | `ThreadStore::open(&self.storage_root)?.begin_fork_at_kernel_anchor` | [2804](../../src/management.rs#L2804) | receiver-type-required |
| `drive_fork` | `ThreadStore::open` | [2804](../../src/management.rs#L2804), [2825](../../src/management.rs#L2825) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `drive_fork` | `intent                             .preset                             .as_ref()                             .map(serde_json::to_value)                             .transpose` | [2812](../../src/management.rs#L2812) | receiver-type-required |
| `drive_fork` | `intent                             .preset                             .as_ref()                             .map` | [2812](../../src/management.rs#L2812) | receiver-type-required |
| `drive_fork` | `intent                             .preset                             .as_ref` | [2812](../../src/management.rs#L2812) | receiver-type-required |
| `drive_fork` | `publish_canonical` | [2822](../../src/management.rs#L2822), [2832](../../src/management.rs#L2832), [2848](../../src/management.rs#L2848), [2851](../../src/management.rs#L2851), [2856](../../src/management.rs#L2856) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `drive_fork` | `ThreadStore::open(&self.storage_root)?.recover_rewrites` | [2825](../../src/management.rs#L2825) | receiver-type-required |
| `drive_fork` | `"fork rewrite did not publish its destination".to_owned` | [2828](../../src/management.rs#L2828) | receiver-type-required |
| `drive_fork` | `intent                 .preset                 .as_ref()                 .and_then` | [2835](../../src/management.rs#L2835) | receiver-type-required |
| `drive_fork` | `intent                 .preset                 .as_ref` | [2835](../../src/management.rs#L2835) | receiver-type-required |
| `drive_fork` | `preset.defaults.as_ref` | [2838](../../src/management.rs#L2838) | receiver-type-required |
| `drive_fork` | `self.apply_preset_defaults` | [2840](../../src/management.rs#L2840) | [endpoint::management::ManagementStore::apply_preset_defaults](../../src/management.rs#L1289) |
| `drive_fork` | `folder_workspace` | [2845](../../src/management.rs#L2845) | [endpoint::management::folder_workspace](../../src/management.rs#L4223) |
| `drive_fork` | `self.read_metadata` | [2846](../../src/management.rs#L2846) | [endpoint::management::ManagementStore::read_metadata](../../src/management.rs#L3082) |
| `drive_fork` | `record.started_at.clone` | [2847](../../src/management.rs#L2847) | receiver-type-required |
| `drive_fork` | `self.metadata_path` | [2848](../../src/management.rs#L2848) | [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) |
| `drive_fork` | `Some` | [2855](../../src/management.rs#L2855) | external-constructor-callback-or-unresolved |
| `drive_fork` | `to_ijson` | [2855](../../src/management.rs#L2855) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `drive_fork` | `record.response.ok_or_else` | [2858](../../src/management.rs#L2858) | receiver-type-required |
| `drive_fork` | `"complete fork operation lacks response".to_owned` | [2859](../../src/management.rs#L2859) | receiver-type-required |
| `drive_fork` | `serde_json::from_slice(&response.canonical_bytes()?).map_err` | [2861](../../src/management.rs#L2861) | receiver-type-required |
| `drive_fork` | `serde_json::from_slice` | [2861](../../src/management.rs#L2861) | external-constructor-callback-or-unresolved |
| `drive_fork` | `response.canonical_bytes` | [2861](../../src/management.rs#L2861) | receiver-type-required |
| `drive_queue_completion` | `queue_transaction_intent` | [2870](../../src/management.rs#L2870) | [endpoint::management::queue_transaction_intent](../../src/management.rs#L3922) |
| `drive_queue_completion` | `Err` | [2872](../../src/management.rs#L2872), [2879](../../src/management.rs#L2879), [2889](../../src/management.rs#L2889) | external-constructor-callback-or-unresolved |
| `drive_queue_completion` | `ManagementError::CorruptOperation` | [2872](../../src/management.rs#L2872), [2879](../../src/management.rs#L2879), [2889](../../src/management.rs#L2889) | external-constructor-callback-or-unresolved |
| `drive_queue_completion` | `"unsupported queue transaction operation".to_owned` | [2873](../../src/management.rs#L2873) | receiver-type-required |
| `drive_queue_completion` | `queue_completion_from_record` | [2877](../../src/management.rs#L2877), [2918](../../src/management.rs#L2918) | [endpoint::management::queue_completion_from_record](../../src/management.rs#L4050) |
| `drive_queue_completion` | `completion_for_decision` | [2878](../../src/management.rs#L2878) | [endpoint::management::completion_for_decision](../../src/management.rs#L4000) |
| `drive_queue_completion` | `"queue completion decision changed after durability".to_owned` | [2880](../../src/management.rs#L2880) | receiver-type-required |
| `drive_queue_completion` | `Ok` | [2883](../../src/management.rs#L2883), [2897](../../src/management.rs#L2897) | external-constructor-callback-or-unresolved |
| `drive_queue_completion` | `verify_queue_payload` | [2885](../../src/management.rs#L2885) | [endpoint::management::verify_queue_payload](../../src/management.rs#L3965) |
| `drive_queue_completion` | `self.operation_payload_path` | [2885](../../src/management.rs#L2885) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `drive_queue_completion` | `"rejected queue transaction advanced before its decision".to_owned` | [2890](../../src/management.rs#L2890) | receiver-type-required |
| `drive_queue_completion` | `Some` | [2895](../../src/management.rs#L2895), [2915](../../src/management.rs#L2915) | external-constructor-callback-or-unresolved |
| `drive_queue_completion` | `queue_completion_value` | [2895](../../src/management.rs#L2895), [2915](../../src/management.rs#L2915) | [endpoint::management::queue_completion_value](../../src/management.rs#L4012) |
| `drive_queue_completion` | `publish_canonical` | [2896](../../src/management.rs#L2896), [2902](../../src/management.rs#L2902), [2906](../../src/management.rs#L2906), [2910](../../src/management.rs#L2910), [2916](../../src/management.rs#L2916) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `remove_recordless_payloads` | `self.root.join` | [2924](../../src/management.rs#L2924) | receiver-type-required |
| `remove_recordless_payloads` | `fs::read_dir` | [2925](../../src/management.rs#L2925), [2930](../../src/management.rs#L2930) | external-constructor-callback-or-unresolved |
| `remove_recordless_payloads` | `prefix.file_type()?.is_dir` | [2927](../../src/management.rs#L2927) | receiver-type-required |
| `remove_recordless_payloads` | `prefix.file_type` | [2927](../../src/management.rs#L2927) | receiver-type-required |
| `remove_recordless_payloads` | `prefix.path` | [2930](../../src/management.rs#L2930), [2941](../../src/management.rs#L2941), [2943](../../src/management.rs#L2943) | receiver-type-required |
| `remove_recordless_payloads` | `entry.file_type()?.is_dir` | [2932](../../src/management.rs#L2932) | receiver-type-required |
| `remove_recordless_payloads` | `entry.file_type` | [2932](../../src/management.rs#L2932) | receiver-type-required |
| `remove_recordless_payloads` | `entry.file_name().to_string_lossy().into_owned` | [2935](../../src/management.rs#L2935) | receiver-type-required |
| `remove_recordless_payloads` | `entry.file_name().to_string_lossy` | [2935](../../src/management.rs#L2935) | receiver-type-required |
| `remove_recordless_payloads` | `entry.file_name` | [2935](../../src/management.rs#L2935) | receiver-type-required |
| `remove_recordless_payloads` | `is_hex_digest` | [2936](../../src/management.rs#L2936) | [endpoint::management::is_hex_digest](../../src/management.rs#L3407) |
| `remove_recordless_payloads` | `Err` | [2937](../../src/management.rs#L2937) | external-constructor-callback-or-unresolved |
| `remove_recordless_payloads` | `ManagementError::CorruptOperation` | [2937](../../src/management.rs#L2937) | external-constructor-callback-or-unresolved |
| `remove_recordless_payloads` | `"operation payload directory has invalid digest name".to_owned` | [2938](../../src/management.rs#L2938) | receiver-type-required |
| `remove_recordless_payloads` | `prefix.path().join(format!("{digest}.json")).is_file` | [2941](../../src/management.rs#L2941) | receiver-type-required |
| `remove_recordless_payloads` | `prefix.path().join` | [2941](../../src/management.rs#L2941) | receiver-type-required |
| `remove_recordless_payloads` | `fs::remove_dir_all` | [2942](../../src/management.rs#L2942) | external-constructor-callback-or-unresolved |
| `remove_recordless_payloads` | `entry.path` | [2942](../../src/management.rs#L2942) | receiver-type-required |
| `remove_recordless_payloads` | `File::open(prefix.path())?.sync_all` | [2943](../../src/management.rs#L2943) | receiver-type-required |
| `remove_recordless_payloads` | `File::open` | [2943](../../src/management.rs#L2943) | external-constructor-callback-or-unresolved |
| `remove_recordless_payloads` | `Ok` | [2947](../../src/management.rs#L2947) | external-constructor-callback-or-unresolved |
| `remove_recordless_payload` | `self.operation_path` | [2951](../../src/management.rs#L2951) | [endpoint::management::ManagementStore::operation_path](../../src/management.rs#L3059) |
| `remove_recordless_payload` | `self             .operation_payload_path(rpc_id)             .parent()             .ok_or_else(&#124;&#124; {                 ManagementError::CorruptOperation("operation payload has no root".to_owned())             })?             .to_path_buf` | [2952](../../src/management.rs#L2952) | receiver-type-required |
| `remove_recordless_payload` | `self             .operation_payload_path(rpc_id)             .parent()             .ok_or_else` | [2952](../../src/management.rs#L2952) | receiver-type-required |
| `remove_recordless_payload` | `self             .operation_payload_path(rpc_id)             .parent` | [2952](../../src/management.rs#L2952) | receiver-type-required |
| `remove_recordless_payload` | `self             .operation_payload_path` | [2952](../../src/management.rs#L2952) | [endpoint::management::ManagementStore::operation_payload_path](../../src/management.rs#L3067) |
| `remove_recordless_payload` | `ManagementError::CorruptOperation` | [2956](../../src/management.rs#L2956), [2962](../../src/management.rs#L2962) | external-constructor-callback-or-unresolved |
| `remove_recordless_payload` | `"operation payload has no root".to_owned` | [2956](../../src/management.rs#L2956) | receiver-type-required |
| `remove_recordless_payload` | `operation_path.exists` | [2959](../../src/management.rs#L2959) | receiver-type-required |
| `remove_recordless_payload` | `payload_root.exists` | [2959](../../src/management.rs#L2959) | receiver-type-required |
| `remove_recordless_payload` | `fs::remove_dir_all` | [2960](../../src/management.rs#L2960) | external-constructor-callback-or-unresolved |
| `remove_recordless_payload` | `File::open(payload_root.parent().ok_or_else(&#124;&#124; {                 ManagementError::CorruptOperation("operation payload has no prefix".to_owned())             })?)?             .sync_all` | [2961](../../src/management.rs#L2961) | receiver-type-required |
| `remove_recordless_payload` | `File::open` | [2961](../../src/management.rs#L2961) | external-constructor-callback-or-unresolved |
| `remove_recordless_payload` | `payload_root.parent().ok_or_else` | [2961](../../src/management.rs#L2961) | receiver-type-required |
| `remove_recordless_payload` | `payload_root.parent` | [2961](../../src/management.rs#L2961) | receiver-type-required |
| `remove_recordless_payload` | `"operation payload has no prefix".to_owned` | [2962](../../src/management.rs#L2962) | receiver-type-required |
| `remove_recordless_payload` | `Ok` | [2966](../../src/management.rs#L2966) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `folder_move_intent` | [2975](../../src/management.rs#L2975) | [endpoint::management::folder_move_intent](../../src/management.rs#L3849) |
| `drive_folder_move` | `Err` | [2977](../../src/management.rs#L2977), [2995](../../src/management.rs#L2995), [3006](../../src/management.rs#L3006) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `ManagementError::CorruptOperation` | [2977](../../src/management.rs#L2977), [2995](../../src/management.rs#L2995), [3006](../../src/management.rs#L3006), [3034](../../src/management.rs#L3034) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `"unsupported folder move operation".to_owned` | [2978](../../src/management.rs#L2978) | receiver-type-required |
| `drive_folder_move` | `publish_canonical` | [2983](../../src/management.rs#L2983), [3011](../../src/management.rs#L3011), [3018](../../src/management.rs#L3018), [3021](../../src/management.rs#L3021), [3031](../../src/management.rs#L3031) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `drive_folder_move` | `Path::new` | [2986](../../src/management.rs#L2986), [2987](../../src/management.rs#L2987), [3014](../../src/management.rs#L3014) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `source.is_dir` | [2988](../../src/management.rs#L2988) | receiver-type-required |
| `drive_folder_move` | `DirectoryLock::try_exclusive` | [2991](../../src/management.rs#L2991) | [store::platform::DirectoryLock::try_exclusive](../../../store/src/platform.rs#L66) |
| `drive_folder_move` | `verify_folder_reservation` | [2993](../../src/management.rs#L2993), [3004](../../src/management.rs#L3004) | [endpoint::management::verify_folder_reservation](../../src/management.rs#L4210) |
| `drive_folder_move` | `destination.exists` | [2994](../../src/management.rs#L2994) | receiver-type-required |
| `drive_folder_move` | `"folder move destination already exists".to_owned` | [2996](../../src/management.rs#L2996) | receiver-type-required |
| `drive_folder_move` | `store::move_thread_folder` | [3001](../../src/management.rs#L3001) | [store::folder::move_thread_folder](../../../store/src/folder.rs#L986) |
| `drive_folder_move` | `drop` | [3002](../../src/management.rs#L3002) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `destination.is_dir` | [3003](../../src/management.rs#L3003) | receiver-type-required |
| `drive_folder_move` | `"folder move lost both source and destination".to_owned` | [3007](../../src/management.rs#L3007) | receiver-type-required |
| `drive_folder_move` | `folder_workspace` | [3015](../../src/management.rs#L3015) | [endpoint::management::folder_workspace](../../src/management.rs#L4223) |
| `drive_folder_move` | `self.read_metadata_optional` | [3016](../../src/management.rs#L3016) | [endpoint::management::ManagementStore::read_metadata_optional](../../src/management.rs#L3090) |
| `drive_folder_move` | `record.started_at.clone` | [3017](../../src/management.rs#L3017) | receiver-type-required |
| `drive_folder_move` | `self.metadata_path` | [3018](../../src/management.rs#L3018) | [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) |
| `drive_folder_move` | `Some` | [3030](../../src/management.rs#L3030) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `IJsonValue::parse` | [3030](../../src/management.rs#L3030) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `drive_folder_move` | `serde_json::to_vec` | [3030](../../src/management.rs#L3030) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `record.response.ok_or_else` | [3033](../../src/management.rs#L3033) | receiver-type-required |
| `drive_folder_move` | `"complete folder move lacks response".to_owned` | [3034](../../src/management.rs#L3034) | receiver-type-required |
| `drive_folder_move` | `serde_json::from_slice(&response.canonical_bytes()?).map_err` | [3036](../../src/management.rs#L3036) | receiver-type-required |
| `drive_folder_move` | `serde_json::from_slice` | [3036](../../src/management.rs#L3036) | external-constructor-callback-or-unresolved |
| `drive_folder_move` | `response.canonical_bytes` | [3036](../../src/management.rs#L3036) | receiver-type-required |
| `operation_paths` | `Vec::new` | [3040](../../src/management.rs#L3040) | external-constructor-callback-or-unresolved |
| `operation_paths` | `fs::read_dir` | [3041](../../src/management.rs#L3041), [3046](../../src/management.rs#L3046) | external-constructor-callback-or-unresolved |
| `operation_paths` | `self.root.join` | [3041](../../src/management.rs#L3041) | receiver-type-required |
| `operation_paths` | `prefix.file_type()?.is_dir` | [3043](../../src/management.rs#L3043) | receiver-type-required |
| `operation_paths` | `prefix.file_type` | [3043](../../src/management.rs#L3043) | receiver-type-required |
| `operation_paths` | `prefix.path` | [3046](../../src/management.rs#L3046) | receiver-type-required |
| `operation_paths` | `entry.file_type()?.is_file` | [3048](../../src/management.rs#L3048) | receiver-type-required |
| `operation_paths` | `entry.file_type` | [3048](../../src/management.rs#L3048) | receiver-type-required |
| `operation_paths` | `entry.path().extension().and_then` | [3049](../../src/management.rs#L3049) | receiver-type-required |
| `operation_paths` | `entry.path().extension` | [3049](../../src/management.rs#L3049) | receiver-type-required |
| `operation_paths` | `entry.path` | [3049](../../src/management.rs#L3049), [3051](../../src/management.rs#L3051) | receiver-type-required |
| `operation_paths` | `value.to_str` | [3049](../../src/management.rs#L3049) | receiver-type-required |
| `operation_paths` | `Some` | [3049](../../src/management.rs#L3049) | external-constructor-callback-or-unresolved |
| `operation_paths` | `paths.push` | [3051](../../src/management.rs#L3051) | receiver-type-required |
| `operation_paths` | `paths.sort` | [3055](../../src/management.rs#L3055) | receiver-type-required |
| `operation_paths` | `Ok` | [3056](../../src/management.rs#L3056) | external-constructor-callback-or-unresolved |
| `operation_path` | `hex_digest` | [3060](../../src/management.rs#L3060) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `operation_path` | `rpc_id.as_bytes` | [3060](../../src/management.rs#L3060) | receiver-type-required |
| `operation_path` | `self.root             .join("operations")             .join(&digest[..2])             .join` | [3061](../../src/management.rs#L3061) | receiver-type-required |
| `operation_path` | `self.root             .join("operations")             .join` | [3061](../../src/management.rs#L3061) | receiver-type-required |
| `operation_path` | `self.root             .join` | [3061](../../src/management.rs#L3061) | receiver-type-required |
| `operation_payload_path` | `hex_digest` | [3068](../../src/management.rs#L3068) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `operation_payload_path` | `rpc_id.as_bytes` | [3068](../../src/management.rs#L3068) | receiver-type-required |
| `operation_payload_path` | `self.root             .join("operations")             .join(&digest[..2])             .join(digest)             .join` | [3069](../../src/management.rs#L3069) | receiver-type-required |
| `operation_payload_path` | `self.root             .join("operations")             .join(&digest[..2])             .join` | [3069](../../src/management.rs#L3069) | receiver-type-required |
| `operation_payload_path` | `self.root             .join("operations")             .join` | [3069](../../src/management.rs#L3069) | receiver-type-required |
| `operation_payload_path` | `self.root             .join` | [3069](../../src/management.rs#L3069) | receiver-type-required |
| `metadata_path` | `self.root             .join("workspaces")             .join` | [3077](../../src/management.rs#L3077) | receiver-type-required |
| `metadata_path` | `self.root             .join` | [3077](../../src/management.rs#L3077) | receiver-type-required |
| `read_metadata` | `self.read_metadata_optional(workspace_id)?.ok_or_else` | [3083](../../src/management.rs#L3083) | receiver-type-required |
| `read_metadata` | `self.read_metadata_optional` | [3083](../../src/management.rs#L3083) | [endpoint::management::ManagementStore::read_metadata_optional](../../src/management.rs#L3090) |
| `read_metadata` | `ManagementError::CorruptOperation` | [3084](../../src/management.rs#L3084) | external-constructor-callback-or-unresolved |
| `read_metadata_optional` | `self.metadata_path` | [3094](../../src/management.rs#L3094) | [endpoint::management::ManagementStore::metadata_path](../../src/management.rs#L3076) |
| `read_metadata_optional` | `fs::read` | [3095](../../src/management.rs#L3095) | external-constructor-callback-or-unresolved |
| `read_metadata_optional` | `Ok` | [3096](../../src/management.rs#L3096), [3097](../../src/management.rs#L3097) | external-constructor-callback-or-unresolved |
| `read_metadata_optional` | `Some` | [3096](../../src/management.rs#L3096) | external-constructor-callback-or-unresolved |
| `read_metadata_optional` | `decode_canonical` | [3096](../../src/management.rs#L3096) | [endpoint::management::decode_canonical](../../src/management.rs#L3752) |
| `read_metadata_optional` | `error.kind` | [3097](../../src/management.rs#L3097) | receiver-type-required |
| `read_metadata_optional` | `Err` | [3098](../../src/management.rs#L3098) | external-constructor-callback-or-unresolved |
| `read_metadata_optional` | `error.into` | [3098](../../src/management.rs#L3098) | receiver-type-required |
| `primary_path` | `config         .folders         .first()         .map(&#124;folder&#124; folder.path.as_str())         .or_else(&#124;&#124; config.cwd.first().map(String::as_str))         .ok_or_else` | [3104](../../src/management.rs#L3104) | receiver-type-required |
| `primary_path` | `config         .folders         .first()         .map(&#124;folder&#124; folder.path.as_str())         .or_else` | [3104](../../src/management.rs#L3104) | receiver-type-required |
| `primary_path` | `config         .folders         .first()         .map` | [3104](../../src/management.rs#L3104) | receiver-type-required |
| `primary_path` | `config         .folders         .first` | [3104](../../src/management.rs#L3104) | receiver-type-required |
| `primary_path` | `folder.path.as_str` | [3107](../../src/management.rs#L3107) | receiver-type-required |
| `primary_path` | `config.cwd.first().map` | [3108](../../src/management.rs#L3108) | receiver-type-required |
| `primary_path` | `config.cwd.first` | [3108](../../src/management.rs#L3108) | receiver-type-required |
| `primary_path` | `ManagementError::CorruptOperation` | [3109](../../src/management.rs#L3109) | external-constructor-callback-or-unresolved |
| `primary_path` | `"workspace cwd is empty".to_owned` | [3109](../../src/management.rs#L3109) | receiver-type-required |
| `binding_for_path` | `config.binding_for_path(path).ok_or_else` | [3113](../../src/management.rs#L3113) | receiver-type-required |
| `binding_for_path` | `config.binding_for_path` | [3113](../../src/management.rs#L3113) | receiver-type-required |
| `binding_for_path` | `ManagementError::CorruptOperation` | [3114](../../src/management.rs#L3114) | external-constructor-callback-or-unresolved |
| `binding_for_path` | `"workspace path has no folder binding".to_owned` | [3114](../../src/management.rs#L3114) | receiver-type-required |
| `binding_for_canonical_path` | `config.folder_paths` | [3122](../../src/management.rs#L3122) | receiver-type-required |
| `binding_for_canonical_path` | `canonical_workspace_path(authored)?             .to_string_lossy()             .into_owned` | [3123](../../src/management.rs#L3123) | receiver-type-required |
| `binding_for_canonical_path` | `canonical_workspace_path(authored)?             .to_string_lossy` | [3123](../../src/management.rs#L3123) | receiver-type-required |
| `binding_for_canonical_path` | `canonical_workspace_path` | [3123](../../src/management.rs#L3123) | [endpoint::management::canonical_workspace_path](../../src/management.rs#L3571) |
| `binding_for_canonical_path` | `binding_for_path(config, authored).map` | [3127](../../src/management.rs#L3127) | receiver-type-required |
| `binding_for_canonical_path` | `binding_for_path` | [3127](../../src/management.rs#L3127) | [endpoint::management::binding_for_path](../../src/management.rs#L3112) |
| `binding_for_canonical_path` | `Ok` | [3130](../../src/management.rs#L3130) | external-constructor-callback-or-unresolved |
| `execution_path_from_snapshot` | `config.execution_cwd().ok_or_else` | [3134](../../src/management.rs#L3134) | receiver-type-required |
| `execution_path_from_snapshot` | `config.execution_cwd` | [3134](../../src/management.rs#L3134) | receiver-type-required |
| `execution_path_from_snapshot` | `ManagementError::CorruptOperation` | [3135](../../src/management.rs#L3135) | external-constructor-callback-or-unresolved |
| `execution_path_from_snapshot` | `"snapshot workspace cwd is empty".to_owned` | [3135](../../src/management.rs#L3135) | receiver-type-required |
| `folder_binding_matches_snapshot` | `intent.is_empty` | [3140](../../src/management.rs#L3140) | receiver-type-required |
| `folder_binding_matches_snapshot` | `Some` | [3140](../../src/management.rs#L3140) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `configs                 .get(workspace_id)                 .ok_or_else` | [3150](../../src/management.rs#L3150) | receiver-type-required |
| `resolve_session_workspace` | `configs                 .get` | [3150](../../src/management.rs#L3150) | receiver-type-required |
| `resolve_session_workspace` | `ManagementError::WorkspaceNotFound` | [3152](../../src/management.rs#L3152) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `workspace_id.to_owned` | [3152](../../src/management.rs#L3152) | receiver-type-required |
| `resolve_session_workspace` | `primary_path` | [3153](../../src/management.rs#L3153), [3193](../../src/management.rs#L3193) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `resolve_session_workspace` | `canonical_workspace_path(cwd)?                     .to_string_lossy()                     .into_owned` | [3155](../../src/management.rs#L3155) | receiver-type-required |
| `resolve_session_workspace` | `canonical_workspace_path(cwd)?                     .to_string_lossy` | [3155](../../src/management.rs#L3155) | receiver-type-required |
| `resolve_session_workspace` | `canonical_workspace_path` | [3155](../../src/management.rs#L3155), [3162](../../src/management.rs#L3162), [3176](../../src/management.rs#L3176), [3194](../../src/management.rs#L3194) | [endpoint::management::canonical_workspace_path](../../src/management.rs#L3571) |
| `resolve_session_workspace` | `binding_for_canonical_path(workspace, &canonical)?                     .ok_or_else` | [3158](../../src/management.rs#L3158) | receiver-type-required |
| `resolve_session_workspace` | `binding_for_canonical_path` | [3158](../../src/management.rs#L3158), [3168](../../src/management.rs#L3168), [3181](../../src/management.rs#L3181) | [endpoint::management::binding_for_canonical_path](../../src/management.rs#L3118) |
| `resolve_session_workspace` | `ManagementError::InvalidPath` | [3159](../../src/management.rs#L3159), [3187](../../src/management.rs#L3187) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `cwd.to_owned` | [3159](../../src/management.rs#L3159), [3187](../../src/management.rs#L3187) | receiver-type-required |
| `resolve_session_workspace` | `Ok` | [3160](../../src/management.rs#L3160), [3165](../../src/management.rs#L3165), [3186](../../src/management.rs#L3186), [3198](../../src/management.rs#L3198) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `canonical_workspace_path(primary)?                 .to_string_lossy()                 .into_owned` | [3162](../../src/management.rs#L3162) | receiver-type-required |
| `resolve_session_workspace` | `canonical_workspace_path(primary)?                 .to_string_lossy` | [3162](../../src/management.rs#L3162) | receiver-type-required |
| `resolve_session_workspace` | `primary.clone` | [3167](../../src/management.rs#L3167) | receiver-type-required |
| `resolve_session_workspace` | `binding_for_canonical_path(workspace, &primary)?.ok_or_else` | [3168](../../src/management.rs#L3168) | receiver-type-required |
| `resolve_session_workspace` | `ManagementError::CorruptOperation` | [3169](../../src/management.rs#L3169) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `"primary workspace path has no folder binding".to_owned` | [3170](../../src/management.rs#L3170) | receiver-type-required |
| `resolve_session_workspace` | `canonical_workspace_path(cwd)?                 .to_string_lossy()                 .into_owned` | [3176](../../src/management.rs#L3176) | receiver-type-required |
| `resolve_session_workspace` | `canonical_workspace_path(cwd)?                 .to_string_lossy` | [3176](../../src/management.rs#L3176) | receiver-type-required |
| `resolve_session_workspace` | `Vec::new` | [3179](../../src/management.rs#L3179) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `configs.values` | [3180](../../src/management.rs#L3180), [3191](../../src/management.rs#L3191) | receiver-type-required |
| `resolve_session_workspace` | `matches.push` | [3182](../../src/management.rs#L3182) | receiver-type-required |
| `resolve_session_workspace` | `matches.as_slice` | [3185](../../src/management.rs#L3185) | receiver-type-required |
| `resolve_session_workspace` | `binding.clone` | [3186](../../src/management.rs#L3186) | receiver-type-required |
| `resolve_session_workspace` | `Err` | [3187](../../src/management.rs#L3187), [3188](../../src/management.rs#L3188), [3200](../../src/management.rs#L3200) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `ManagementError::WorkspaceAmbiguous` | [3188](../../src/management.rs#L3188) | external-constructor-callback-or-unresolved |
| `resolve_session_workspace` | `configs.values().collect::<Vec<_>>().as_slice` | [3191](../../src/management.rs#L3191) | receiver-type-required |
| `resolve_session_workspace` | `configs.values().collect::<Vec<_>>` | [3191](../../src/management.rs#L3191) | receiver-type-required |
| `resolve_session_workspace` | `canonical_workspace_path(authored)?                     .to_string_lossy()                     .into_owned` | [3194](../../src/management.rs#L3194) | receiver-type-required |
| `resolve_session_workspace` | `canonical_workspace_path(authored)?                     .to_string_lossy` | [3194](../../src/management.rs#L3194) | receiver-type-required |
| `resolve_session_workspace` | `binding_for_path` | [3197](../../src/management.rs#L3197) | [endpoint::management::binding_for_path](../../src/management.rs#L3112) |
| `existing_session_cwd` | `active.is_dir` | [3210](../../src/management.rs#L3210) | receiver-type-required |
| `existing_session_cwd` | `archived.is_dir` | [3212](../../src/management.rs#L3212) | receiver-type-required |
| `existing_session_cwd` | `Ok` | [3215](../../src/management.rs#L3215) | external-constructor-callback-or-unresolved |
| `existing_session_cwd` | `folder_workspace` | [3217](../../src/management.rs#L3217) | [endpoint::management::folder_workspace](../../src/management.rs#L4223) |
| `existing_session_cwd` | `configs         .get(&workspace_id)         .map(primary_path)         .transpose()         .map` | [3218](../../src/management.rs#L3218) | receiver-type-required |
| `existing_session_cwd` | `configs         .get(&workspace_id)         .map(primary_path)         .transpose` | [3218](../../src/management.rs#L3218) | receiver-type-required |
| `existing_session_cwd` | `configs         .get(&workspace_id)         .map` | [3218](../../src/management.rs#L3218) | receiver-type-required |
| `existing_session_cwd` | `configs         .get` | [3218](../../src/management.rs#L3218) | receiver-type-required |
| `existing_session_cwd` | `value.map` | [3222](../../src/management.rs#L3222) | receiver-type-required |
| `session_preset` | `tools::ResolvedPreset::resolve_request(request, base, environment)             .map(Some)             .map_err` | [3235](../../src/management.rs#L3235) | receiver-type-required |
| `session_preset` | `tools::ResolvedPreset::resolve_request(request, base, environment)             .map` | [3235](../../src/management.rs#L3235) | receiver-type-required |
| `session_preset` | `tools::ResolvedPreset::resolve_request` | [3235](../../src/management.rs#L3235) | [tools::preset::ResolvedPreset::resolve_request](../../../tools/src/preset.rs#L721) |
| `session_preset` | `ManagementError::PresetBadRequest` | [3239](../../src/management.rs#L3239) | external-constructor-callback-or-unresolved |
| `session_preset` | `ManagementError::PresetInvalid` | [3241](../../src/management.rs#L3241) | external-constructor-callback-or-unresolved |
| `session_preset` | `Ok` | [3243](../../src/management.rs#L3243), [3247](../../src/management.rs#L3247) | external-constructor-callback-or-unresolved |
| `session_preset` | `Some` | [3243](../../src/management.rs#L3243) | external-constructor-callback-or-unresolved |
| `session_preset` | `tools::ResolvedPreset::from_identity` | [3243](../../src/management.rs#L3243) | [tools::preset::ResolvedPreset::from_identity](../../../tools/src/preset.rs#L770) |
| `genesis_preset_defaults` | `serde_json::from_slice` | [3255](../../src/management.rs#L3255) | external-constructor-callback-or-unresolved |
| `genesis_preset_defaults` | `genesis.canonical_bytes` | [3255](../../src/management.rs#L3255) | receiver-type-required |
| `genesis_preset_defaults` | `value.get` | [3256](../../src/management.rs#L3256) | receiver-type-required |
| `genesis_preset_defaults` | `Ok` | [3257](../../src/management.rs#L3257) | external-constructor-callback-or-unresolved |
| `genesis_preset_defaults` | `tools::ResolvedPreset::from_genesis(preset)         .map(&#124;preset&#124; preset.defaults)         .map_err` | [3259](../../src/management.rs#L3259) | receiver-type-required |
| `genesis_preset_defaults` | `tools::ResolvedPreset::from_genesis(preset)         .map` | [3259](../../src/management.rs#L3259) | receiver-type-required |
| `genesis_preset_defaults` | `tools::ResolvedPreset::from_genesis` | [3259](../../src/management.rs#L3259) | [tools::preset::ResolvedPreset::from_genesis](../../../tools/src/preset.rs#L855) |
| `create_genesis` | `value.as_object_mut().expect` | [3297](../../src/management.rs#L3297) | receiver-type-required |
| `create_genesis` | `value.as_object_mut` | [3297](../../src/management.rs#L3297) | receiver-type-required |
| `create_genesis` | `object.remove` | [3298](../../src/management.rs#L3298) | receiver-type-required |
| `create_genesis` | `object.insert` | [3299](../../src/management.rs#L3299), [3300](../../src/management.rs#L3300) | receiver-type-required |
| `create_genesis` | `"min_reader".to_owned` | [3299](../../src/management.rs#L3299) | receiver-type-required |
| `create_genesis` | `"preset".to_owned` | [3300](../../src/management.rs#L3300) | receiver-type-required |
| `create_genesis` | `serde_json::to_value` | [3300](../../src/management.rs#L3300) | external-constructor-callback-or-unresolved |
| `create_genesis` | `Event::from_value(IJsonValue::parse(&serde_json::to_vec(&value)?)?).map_err` | [3302](../../src/management.rs#L3302) | receiver-type-required |
| `create_genesis` | `Event::from_value` | [3302](../../src/management.rs#L3302) | [schema::event::Event::from_value](../../../schema/src/event.rs#L179) |
| `create_genesis` | `IJsonValue::parse` | [3302](../../src/management.rs#L3302) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `create_genesis` | `serde_json::to_vec` | [3302](../../src/management.rs#L3302) | external-constructor-callback-or-unresolved |
| `canonical_event_line` | `event.canonical_bytes` | [3306](../../src/management.rs#L3306) | receiver-type-required |
| `canonical_event_line` | `bytes.push` | [3307](../../src/management.rs#L3307) | receiver-type-required |
| `canonical_event_line` | `Ok` | [3308](../../src/management.rs#L3308) | external-constructor-callback-or-unresolved |
| `read_canonical_event` | `fs::read` | [3312](../../src/management.rs#L3312) | external-constructor-callback-or-unresolved |
| `read_canonical_event` | `bytes.ends_with` | [3313](../../src/management.rs#L3313) | receiver-type-required |
| `read_canonical_event` | `bytes.len` | [3313](../../src/management.rs#L3313), [3319](../../src/management.rs#L3319) | receiver-type-required |
| `read_canonical_event` | `Err` | [3314](../../src/management.rs#L3314) | external-constructor-callback-or-unresolved |
| `read_canonical_event` | `ManagementError::CorruptOperation` | [3314](../../src/management.rs#L3314) | external-constructor-callback-or-unresolved |
| `read_canonical_event` | `Event::decode_canonical(&bytes[..bytes.len() - 1]).map_err` | [3319](../../src/management.rs#L3319) | receiver-type-required |
| `read_canonical_event` | `Event::decode_canonical` | [3319](../../src/management.rs#L3319) | [schema::event::Event::decode_canonical](../../../schema/src/event.rs#L168) |
| `verify_session_create_genesis` | `genesis.origin_tuple()?.ok_or_else` | [3327](../../src/management.rs#L3327) | receiver-type-required |
| `verify_session_create_genesis` | `genesis.origin_tuple` | [3327](../../src/management.rs#L3327) | receiver-type-required |
| `verify_session_create_genesis` | `ManagementError::CorruptOperation` | [3328](../../src/management.rs#L3328), [3332](../../src/management.rs#L3332), [3365](../../src/management.rs#L3365) | external-constructor-callback-or-unresolved |
| `verify_session_create_genesis` | `"session-create genesis lacks origin".to_owned` | [3328](../../src/management.rs#L3328) | receiver-type-required |
| `verify_session_create_genesis` | `serde_json::from_slice` | [3330](../../src/management.rs#L3330) | external-constructor-callback-or-unresolved |
| `verify_session_create_genesis` | `genesis.canonical_bytes` | [3330](../../src/management.rs#L3330) | receiver-type-required |
| `verify_session_create_genesis` | `genesis_value.as_object().ok_or_else` | [3331](../../src/management.rs#L3331) | receiver-type-required |
| `verify_session_create_genesis` | `genesis_value.as_object` | [3331](../../src/management.rs#L3331) | receiver-type-required |
| `verify_session_create_genesis` | `"session-create genesis is not an object".to_owned` | [3332](../../src/management.rs#L3332) | receiver-type-required |
| `verify_session_create_genesis` | `object         .get("config")         .and_then(Value::as_object)         .and_then(&#124;value&#124; value.get("digest"))         .and_then` | [3334](../../src/management.rs#L3334) | receiver-type-required |
| `verify_session_create_genesis` | `object         .get("config")         .and_then(Value::as_object)         .and_then` | [3334](../../src/management.rs#L3334) | receiver-type-required |
| `verify_session_create_genesis` | `object         .get("config")         .and_then` | [3334](../../src/management.rs#L3334) | receiver-type-required |
| `verify_session_create_genesis` | `object         .get` | [3334](../../src/management.rs#L3334), [3339](../../src/management.rs#L3339) | receiver-type-required |
| `verify_session_create_genesis` | `value.get` | [3337](../../src/management.rs#L3337), [3342](../../src/management.rs#L3342) | receiver-type-required |
| `verify_session_create_genesis` | `object         .get("instruction")         .and_then(Value::as_object)         .and_then(&#124;value&#124; value.get("digest"))         .and_then` | [3339](../../src/management.rs#L3339) | receiver-type-required |
| `verify_session_create_genesis` | `object         .get("instruction")         .and_then(Value::as_object)         .and_then` | [3339](../../src/management.rs#L3339) | receiver-type-required |
| `verify_session_create_genesis` | `object         .get("instruction")         .and_then` | [3339](../../src/management.rs#L3339) | receiver-type-required |
| `verify_session_create_genesis` | `genesis.seq` | [3344](../../src/management.rs#L3344) | receiver-type-required |
| `verify_session_create_genesis` | `genesis.string_field` | [3345](../../src/management.rs#L3345), [3346](../../src/management.rs#L3346), [3348](../../src/management.rs#L3348), [3349](../../src/management.rs#L3349), [3353](../../src/management.rs#L3353) | receiver-type-required |
| `verify_session_create_genesis` | `Some` | [3345](../../src/management.rs#L3345), [3346](../../src/management.rs#L3346), [3348](../../src/management.rs#L3348), [3349](../../src/management.rs#L3349), [3353](../../src/management.rs#L3353) | external-constructor-callback-or-unresolved |
| `verify_session_create_genesis` | `intent.session_id.as_str` | [3345](../../src/management.rs#L3345) | receiver-type-required |
| `verify_session_create_genesis` | `intent.workspace_id.as_str` | [3346](../../src/management.rs#L3346) | receiver-type-required |
| `verify_session_create_genesis` | `intent.folder_binding.is_empty` | [3347](../../src/management.rs#L3347) | receiver-type-required |
| `verify_session_create_genesis` | `intent.folder_binding.as_str` | [3348](../../src/management.rs#L3348) | receiver-type-required |
| `verify_session_create_genesis` | `record.started_at.as_str` | [3349](../../src/management.rs#L3349) | receiver-type-required |
| `verify_session_create_genesis` | `intent.config_asset.strip_prefix` | [3350](../../src/management.rs#L3350) | receiver-type-required |
| `verify_session_create_genesis` | `intent.instruction_asset.strip_prefix` | [3351](../../src/management.rs#L3351) | receiver-type-required |
| `verify_session_create_genesis` | `intent.identity_profile.as_ref().is_some_and` | [3352](../../src/management.rs#L3352) | receiver-type-required |
| `verify_session_create_genesis` | `intent.identity_profile.as_ref` | [3352](../../src/management.rs#L3352) | receiver-type-required |
| `verify_session_create_genesis` | `profile.as_str` | [3353](../../src/management.rs#L3353) | receiver-type-required |
| `verify_session_create_genesis` | `intent.preset_digest.as_deref` | [3355](../../src/management.rs#L3355) | receiver-type-required |
| `verify_session_create_genesis` | `object                 .get("preset")                 .and_then(&#124;preset&#124; preset.get("digest"))                 .and_then` | [3356](../../src/management.rs#L3356) | receiver-type-required |
| `verify_session_create_genesis` | `object                 .get("preset")                 .and_then` | [3356](../../src/management.rs#L3356) | receiver-type-required |
| `verify_session_create_genesis` | `object                 .get` | [3356](../../src/management.rs#L3356) | receiver-type-required |
| `verify_session_create_genesis` | `preset.get` | [3358](../../src/management.rs#L3358) | receiver-type-required |
| `verify_session_create_genesis` | `is_endpoint_origin_client` | [3360](../../src/management.rs#L3360) | [endpoint::management::is_endpoint_origin_client](../../src/management.rs#L203) |
| `verify_session_create_genesis` | `Err` | [3365](../../src/management.rs#L3365) | external-constructor-callback-or-unresolved |
| `verify_session_create_genesis` | `"session-create genesis disagrees with immutable intent".to_owned` | [3366](../../src/management.rs#L3366) | receiver-type-required |
| `verify_session_create_genesis` | `Ok` | [3369](../../src/management.rs#L3369) | external-constructor-callback-or-unresolved |
| `copy_payload_tree` | `fs::create_dir_all` | [3374](../../src/management.rs#L3374) | external-constructor-callback-or-unresolved |
| `copy_payload_tree` | `fs::read_dir` | [3375](../../src/management.rs#L3375) | external-constructor-callback-or-unresolved |
| `copy_payload_tree` | `target.join` | [3377](../../src/management.rs#L3377) | receiver-type-required |
| `copy_payload_tree` | `entry.file_name` | [3377](../../src/management.rs#L3377) | receiver-type-required |
| `copy_payload_tree` | `entry.file_type` | [3378](../../src/management.rs#L3378) | receiver-type-required |
| `copy_payload_tree` | `kind.is_dir` | [3379](../../src/management.rs#L3379) | receiver-type-required |
| `copy_payload_tree` | `copy_payload_tree` | [3380](../../src/management.rs#L3380) | [endpoint::management::copy_payload_tree](../../src/management.rs#L3373) |
| `copy_payload_tree` | `entry.path` | [3380](../../src/management.rs#L3380), [3382](../../src/management.rs#L3382) | receiver-type-required |
| `copy_payload_tree` | `kind.is_file` | [3381](../../src/management.rs#L3381) | receiver-type-required |
| `copy_payload_tree` | `fs::copy` | [3382](../../src/management.rs#L3382) | external-constructor-callback-or-unresolved |
| `copy_payload_tree` | `File::open(&destination)?.sync_all` | [3383](../../src/management.rs#L3383) | receiver-type-required |
| `copy_payload_tree` | `File::open` | [3383](../../src/management.rs#L3383), [3390](../../src/management.rs#L3390) | external-constructor-callback-or-unresolved |
| `copy_payload_tree` | `Err` | [3385](../../src/management.rs#L3385) | external-constructor-callback-or-unresolved |
| `copy_payload_tree` | `ManagementError::CorruptOperation` | [3385](../../src/management.rs#L3385) | external-constructor-callback-or-unresolved |
| `copy_payload_tree` | `"operation payload holds a non-regular entry".to_owned` | [3386](../../src/management.rs#L3386) | receiver-type-required |
| `copy_payload_tree` | `File::open(target)?.sync_all` | [3390](../../src/management.rs#L3390) | receiver-type-required |
| `copy_payload_tree` | `Ok` | [3391](../../src/management.rs#L3391) | external-constructor-callback-or-unresolved |
| `sync_operation_payload` | `File::open(payload)?.sync_all` | [3395](../../src/management.rs#L3395) | receiver-type-required |
| `sync_operation_payload` | `File::open` | [3395](../../src/management.rs#L3395), [3399](../../src/management.rs#L3399), [3403](../../src/management.rs#L3403) | external-constructor-callback-or-unresolved |
| `sync_operation_payload` | `payload.parent().ok_or_else` | [3396](../../src/management.rs#L3396) | receiver-type-required |
| `sync_operation_payload` | `payload.parent` | [3396](../../src/management.rs#L3396) | receiver-type-required |
| `sync_operation_payload` | `ManagementError::CorruptOperation` | [3397](../../src/management.rs#L3397), [3401](../../src/management.rs#L3401) | external-constructor-callback-or-unresolved |
| `sync_operation_payload` | `"operation payload has no rpc root".to_owned` | [3397](../../src/management.rs#L3397) | receiver-type-required |
| `sync_operation_payload` | `File::open(rpc_root)?.sync_all` | [3399](../../src/management.rs#L3399) | receiver-type-required |
| `sync_operation_payload` | `rpc_root.parent().ok_or_else` | [3400](../../src/management.rs#L3400) | receiver-type-required |
| `sync_operation_payload` | `rpc_root.parent` | [3400](../../src/management.rs#L3400) | receiver-type-required |
| `sync_operation_payload` | `"operation payload has no prefix".to_owned` | [3401](../../src/management.rs#L3401) | receiver-type-required |
| `sync_operation_payload` | `File::open(prefix)?.sync_all` | [3403](../../src/management.rs#L3403) | receiver-type-required |
| `sync_operation_payload` | `Ok` | [3404](../../src/management.rs#L3404) | external-constructor-callback-or-unresolved |
| `is_hex_digest` | `value.len` | [3408](../../src/management.rs#L3408) | receiver-type-required |
| `is_hex_digest` | `value             .bytes()             .all` | [3409](../../src/management.rs#L3409) | receiver-type-required |
| `is_hex_digest` | `value             .bytes` | [3409](../../src/management.rs#L3409) | receiver-type-required |
| `is_hex_digest` | `byte.is_ascii_hexdigit` | [3411](../../src/management.rs#L3411) | receiver-type-required |
| `is_hex_digest` | `byte.is_ascii_uppercase` | [3411](../../src/management.rs#L3411) | receiver-type-required |
| `default_workspace_policy` | `Vec::new` | [3424](../../src/management.rs#L3424) | external-constructor-callback-or-unresolved |
| `default_workspace_policy` | `tools::BuiltinManifest::compiled().interactive_names` | [3426](../../src/management.rs#L3426) | receiver-type-required |
| `default_workspace_policy` | `tools::BuiltinManifest::compiled` | [3426](../../src/management.rs#L3426) | [tools::builtin::BuiltinManifest::compiled](../../../tools/src/builtin.rs#L315) |
| `relocate_policy_roots` | `Path::new` | [3441](../../src/management.rs#L3441), [3443](../../src/management.rs#L3443), [3447](../../src/management.rs#L3447) | external-constructor-callback-or-unresolved |
| `relocate_policy_roots` | `Path::new(root.as_str()).strip_prefix` | [3443](../../src/management.rs#L3443) | receiver-type-required |
| `relocate_policy_roots` | `root.as_str` | [3443](../../src/management.rs#L3443) | receiver-type-required |
| `relocate_policy_roots` | `suffix.as_os_str().is_empty` | [3444](../../src/management.rs#L3444) | receiver-type-required |
| `relocate_policy_roots` | `suffix.as_os_str` | [3444](../../src/management.rs#L3444) | receiver-type-required |
| `relocate_policy_roots` | `new_path.to_owned` | [3445](../../src/management.rs#L3445) | receiver-type-required |
| `relocate_policy_roots` | `Path::new(new_path)                     .join(suffix)                     .to_string_lossy()                     .into_owned` | [3447](../../src/management.rs#L3447) | receiver-type-required |
| `relocate_policy_roots` | `Path::new(new_path)                     .join(suffix)                     .to_string_lossy` | [3447](../../src/management.rs#L3447) | receiver-type-required |
| `relocate_policy_roots` | `Path::new(new_path)                     .join` | [3447](../../src/management.rs#L3447) | receiver-type-required |
| `relocate_policy_roots` | `policy.writable_roots.sort` | [3454](../../src/management.rs#L3454) | receiver-type-required |
| `relocate_policy_roots` | `policy.writable_roots.dedup` | [3455](../../src/management.rs#L3455) | receiver-type-required |
| `materialized_folders` | `config.folders.is_empty` | [3462](../../src/management.rs#L3462) | receiver-type-required |
| `materialized_folders` | `config             .cwd             .iter()             .enumerate()             .map(&#124;(index, path)&#124; WorkspaceFolder {                 id: format!("folder-{:04}", index + 1),                 path: path.clone(),             })             .collect` | [3463](../../src/management.rs#L3463) | receiver-type-required |
| `materialized_folders` | `config             .cwd             .iter()             .enumerate()             .map` | [3463](../../src/management.rs#L3463) | receiver-type-required |
| `materialized_folders` | `config             .cwd             .iter()             .enumerate` | [3463](../../src/management.rs#L3463) | receiver-type-required |
| `materialized_folders` | `config             .cwd             .iter` | [3463](../../src/management.rs#L3463) | receiver-type-required |
| `materialized_folders` | `path.clone` | [3469](../../src/management.rs#L3469) | receiver-type-required |
| `materialized_folders` | `config.folders.clone` | [3473](../../src/management.rs#L3473) | receiver-type-required |
| `next_folder_id` | `folders         .iter()         .filter_map(&#124;folder&#124; folder.id.strip_prefix("folder-")?.parse::<u64>().ok())         .max()         .unwrap_or(0)         .checked_add(1)         .ok_or_else` | [3478](../../src/management.rs#L3478) | receiver-type-required |
| `next_folder_id` | `folders         .iter()         .filter_map(&#124;folder&#124; folder.id.strip_prefix("folder-")?.parse::<u64>().ok())         .max()         .unwrap_or(0)         .checked_add` | [3478](../../src/management.rs#L3478) | receiver-type-required |
| `next_folder_id` | `folders         .iter()         .filter_map(&#124;folder&#124; folder.id.strip_prefix("folder-")?.parse::<u64>().ok())         .max()         .unwrap_or` | [3478](../../src/management.rs#L3478) | receiver-type-required |
| `next_folder_id` | `folders         .iter()         .filter_map(&#124;folder&#124; folder.id.strip_prefix("folder-")?.parse::<u64>().ok())         .max` | [3478](../../src/management.rs#L3478) | receiver-type-required |
| `next_folder_id` | `folders         .iter()         .filter_map` | [3478](../../src/management.rs#L3478) | receiver-type-required |
| `next_folder_id` | `folders         .iter` | [3478](../../src/management.rs#L3478) | receiver-type-required |
| `next_folder_id` | `folder.id.strip_prefix("folder-")?.parse::<u64>().ok` | [3480](../../src/management.rs#L3480) | receiver-type-required |
| `next_folder_id` | `folder.id.strip_prefix("folder-")?.parse::<u64>` | [3480](../../src/management.rs#L3480) | receiver-type-required |
| `next_folder_id` | `folder.id.strip_prefix` | [3480](../../src/management.rs#L3480) | receiver-type-required |
| `next_folder_id` | `ManagementError::CorruptOperation` | [3484](../../src/management.rs#L3484) | external-constructor-callback-or-unresolved |
| `next_folder_id` | `"folder ids are exhausted".to_owned` | [3484](../../src/management.rs#L3484) | receiver-type-required |
| `next_folder_id` | `Ok` | [3485](../../src/management.rs#L3485) | external-constructor-callback-or-unresolved |
| `with_added_folder` | `materialized_folders` | [3489](../../src/management.rs#L3489) | [endpoint::management::materialized_folders](../../src/management.rs#L3461) |
| `with_added_folder` | `folders.push` | [3490](../../src/management.rs#L3490) | receiver-type-required |
| `with_added_folder` | `folder_id.to_owned` | [3491](../../src/management.rs#L3491) | receiver-type-required |
| `with_added_folder` | `path.to_owned` | [3492](../../src/management.rs#L3492), [3495](../../src/management.rs#L3495) | receiver-type-required |
| `with_added_folder` | `current.policy.clone().map` | [3494](../../src/management.rs#L3494) | receiver-type-required |
| `with_added_folder` | `current.policy.clone` | [3494](../../src/management.rs#L3494) | receiver-type-required |
| `with_added_folder` | `policy.writable_roots.push` | [3495](../../src/management.rs#L3495) | receiver-type-required |
| `with_added_folder` | `policy.writable_roots.sort` | [3496](../../src/management.rs#L3496) | receiver-type-required |
| `with_added_folder` | `policy.writable_roots.dedup` | [3497](../../src/management.rs#L3497) | receiver-type-required |
| `with_added_folder` | `current.id.clone` | [3503](../../src/management.rs#L3503) | receiver-type-required |
| `with_added_folder` | `current.name.clone` | [3504](../../src/management.rs#L3504) | receiver-type-required |
| `with_added_folder` | `Vec::new` | [3505](../../src/management.rs#L3505) | external-constructor-callback-or-unresolved |
| `with_removed_folder` | `materialized_folders` | [3517](../../src/management.rs#L3517) | [endpoint::management::materialized_folders](../../src/management.rs#L3461) |
| `with_removed_folder` | `folders.len` | [3518](../../src/management.rs#L3518), [3520](../../src/management.rs#L3520) | receiver-type-required |
| `with_removed_folder` | `folders.retain` | [3519](../../src/management.rs#L3519) | receiver-type-required |
| `with_removed_folder` | `Path::new` | [3523](../../src/management.rs#L3523), [3526](../../src/management.rs#L3526), [3530](../../src/management.rs#L3530) | external-constructor-callback-or-unresolved |
| `with_removed_folder` | `current.policy.clone().map` | [3524](../../src/management.rs#L3524) | receiver-type-required |
| `with_removed_folder` | `current.policy.clone` | [3524](../../src/management.rs#L3524) | receiver-type-required |
| `with_removed_folder` | `policy.writable_roots.retain` | [3525](../../src/management.rs#L3525) | receiver-type-required |
| `with_removed_folder` | `root.as_str` | [3526](../../src/management.rs#L3526) | receiver-type-required |
| `with_removed_folder` | `root.starts_with` | [3527](../../src/management.rs#L3527), [3530](../../src/management.rs#L3530) | receiver-type-required |
| `with_removed_folder` | `folders                     .iter()                     .any` | [3528](../../src/management.rs#L3528) | receiver-type-required |
| `with_removed_folder` | `folders                     .iter` | [3528](../../src/management.rs#L3528) | receiver-type-required |
| `with_removed_folder` | `Some` | [3534](../../src/management.rs#L3534) | external-constructor-callback-or-unresolved |
| `with_removed_folder` | `current.id.clone` | [3537](../../src/management.rs#L3537) | receiver-type-required |
| `with_removed_folder` | `current.name.clone` | [3538](../../src/management.rs#L3538) | receiver-type-required |
| `with_removed_folder` | `Vec::new` | [3539](../../src/management.rs#L3539) | external-constructor-callback-or-unresolved |
| `folders_view` | `config.id.clone` | [3547](../../src/management.rs#L3547) | receiver-type-required |
| `folders_view` | `materialized_folders(config)             .into_iter()             .map(&#124;folder&#124; WorkspaceFolderView {                 folder_id: folder.id,                 path: folder.path,             })             .collect` | [3549](../../src/management.rs#L3549) | receiver-type-required |
| `folders_view` | `materialized_folders(config)             .into_iter()             .map` | [3549](../../src/management.rs#L3549) | receiver-type-required |
| `folders_view` | `materialized_folders(config)             .into_iter` | [3549](../../src/management.rs#L3549) | receiver-type-required |
| `folders_view` | `materialized_folders` | [3549](../../src/management.rs#L3549) | [endpoint::management::materialized_folders](../../src/management.rs#L3461) |
| `folder_response` | `response.get(name).cloned().ok_or_else` | [3561](../../src/management.rs#L3561) | receiver-type-required |
| `folder_response` | `response.get(name).cloned` | [3561](../../src/management.rs#L3561) | receiver-type-required |
| `folder_response` | `response.get` | [3561](../../src/management.rs#L3561) | receiver-type-required |
| `folder_response` | `ManagementError::CorruptOperation` | [3562](../../src/management.rs#L3562) | external-constructor-callback-or-unresolved |
| `folder_response` | `Ok` | [3565](../../src/management.rs#L3565) | external-constructor-callback-or-unresolved |
| `folder_response` | `serde_json::from_value` | [3566](../../src/management.rs#L3566), [3567](../../src/management.rs#L3567) | external-constructor-callback-or-unresolved |
| `folder_response` | `field` | [3566](../../src/management.rs#L3566), [3567](../../src/management.rs#L3567) | external-constructor-callback-or-unresolved |
| `canonical_workspace_path` | `Path::new` | [3572](../../src/management.rs#L3572) | external-constructor-callback-or-unresolved |
| `canonical_workspace_path` | `path.is_absolute` | [3573](../../src/management.rs#L3573) | receiver-type-required |
| `canonical_workspace_path` | `Err` | [3574](../../src/management.rs#L3574) | external-constructor-callback-or-unresolved |
| `canonical_workspace_path` | `ManagementError::InvalidPath` | [3574](../../src/management.rs#L3574), [3577](../../src/management.rs#L3577) | external-constructor-callback-or-unresolved |
| `canonical_workspace_path` | `value.to_owned` | [3574](../../src/management.rs#L3574), [3577](../../src/management.rs#L3577) | receiver-type-required |
| `canonical_workspace_path` | `path.canonicalize()         .map_err` | [3576](../../src/management.rs#L3576) | receiver-type-required |
| `canonical_workspace_path` | `path.canonicalize` | [3576](../../src/management.rs#L3576) | receiver-type-required |
| `unavailable_absolute_path` | `Path::new` | [3581](../../src/management.rs#L3581) | external-constructor-callback-or-unresolved |
| `unavailable_absolute_path` | `path.is_absolute` | [3582](../../src/management.rs#L3582) | receiver-type-required |
| `unavailable_absolute_path` | `path.exists` | [3582](../../src/management.rs#L3582) | receiver-type-required |
| `allocate_uuid_v7` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err(&#124;error&#124; ManagementError::Clock(error.to_string()))?         .as_millis` | [3586](../../src/management.rs#L3586) | receiver-type-required |
| `allocate_uuid_v7` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err` | [3586](../../src/management.rs#L3586) | receiver-type-required |
| `allocate_uuid_v7` | `SystemTime::now()         .duration_since` | [3586](../../src/management.rs#L3586) | receiver-type-required |
| `allocate_uuid_v7` | `SystemTime::now` | [3586](../../src/management.rs#L3586) | external-constructor-callback-or-unresolved |
| `allocate_uuid_v7` | `ManagementError::Clock` | [3588](../../src/management.rs#L3588), [3591](../../src/management.rs#L3591) | external-constructor-callback-or-unresolved |
| `allocate_uuid_v7` | `error.to_string` | [3588](../../src/management.rs#L3588) | receiver-type-required |
| `allocate_uuid_v7` | `u128::from` | [3590](../../src/management.rs#L3590) | external-constructor-callback-or-unresolved |
| `allocate_uuid_v7` | `Err` | [3591](../../src/management.rs#L3591) | external-constructor-callback-or-unresolved |
| `allocate_uuid_v7` | `"system clock exceeds UUIDv7 timestamp width".to_owned` | [3592](../../src/management.rs#L3592) | receiver-type-required |
| `allocate_uuid_v7` | `fs::File::open("/dev/urandom")?.read_exact` | [3596](../../src/management.rs#L3596) | receiver-type-required |
| `allocate_uuid_v7` | `fs::File::open` | [3596](../../src/management.rs#L3596) | external-constructor-callback-or-unresolved |
| `allocate_uuid_v7` | `(millis as u64).to_be_bytes` | [3597](../../src/management.rs#L3597) | receiver-type-required |
| `allocate_uuid_v7` | `bytes[..6].copy_from_slice` | [3598](../../src/management.rs#L3598) | receiver-type-required |
| `allocate_uuid_v7` | `Ok` | [3601](../../src/management.rs#L3601) | external-constructor-callback-or-unresolved |
| `allocate_uuid_v7` | `Uuid::from_bytes(bytes).hyphenated().to_string` | [3601](../../src/management.rs#L3601) | receiver-type-required |
| `allocate_uuid_v7` | `Uuid::from_bytes(bytes).hyphenated` | [3601](../../src/management.rs#L3601) | receiver-type-required |
| `allocate_uuid_v7` | `Uuid::from_bytes` | [3601](../../src/management.rs#L3601) | external-constructor-callback-or-unresolved |
| `current_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err` | [3605](../../src/management.rs#L3605) | receiver-type-required |
| `current_timestamp` | `SystemTime::now()         .duration_since` | [3605](../../src/management.rs#L3605) | receiver-type-required |
| `current_timestamp` | `SystemTime::now` | [3605](../../src/management.rs#L3605) | external-constructor-callback-or-unresolved |
| `current_timestamp` | `ManagementError::Clock` | [3607](../../src/management.rs#L3607), [3609](../../src/management.rs#L3609) | external-constructor-callback-or-unresolved |
| `current_timestamp` | `error.to_string` | [3607](../../src/management.rs#L3607) | receiver-type-required |
| `current_timestamp` | `timestamp_from_millis` | [3608](../../src/management.rs#L3608) | [endpoint::management::timestamp_from_millis](../../src/management.rs#L3719) |
| `current_timestamp` | `u64::try_from(duration.as_millis()).map_err` | [3608](../../src/management.rs#L3608) | receiver-type-required |
| `current_timestamp` | `u64::try_from` | [3608](../../src/management.rs#L3608) | external-constructor-callback-or-unresolved |
| `current_timestamp` | `duration.as_millis` | [3608](../../src/management.rs#L3608) | receiver-type-required |
| `current_timestamp` | `"system clock exceeds supported millisecond range".to_owned` | [3609](../../src/management.rs#L3609) | receiver-type-required |
| `validate_title` | `title.trim_matches` | [3614](../../src/management.rs#L3614) | receiver-type-required |
| `validate_title` | `title.is_empty` | [3616](../../src/management.rs#L3616) | receiver-type-required |
| `validate_title` | `title.len` | [3617](../../src/management.rs#L3617) | receiver-type-required |
| `validate_title` | `title.chars().any` | [3618](../../src/management.rs#L3618) | receiver-type-required |
| `validate_title` | `title.chars` | [3618](../../src/management.rs#L3618) | receiver-type-required |
| `validate_title` | `Err` | [3620](../../src/management.rs#L3620) | external-constructor-callback-or-unresolved |
| `validate_title` | `ManagementError::InvalidTitle` | [3620](../../src/management.rs#L3620) | external-constructor-callback-or-unresolved |
| `validate_title` | `title.to_owned` | [3620](../../src/management.rs#L3620) | receiver-type-required |
| `validate_title` | `Ok` | [3622](../../src/management.rs#L3622) | external-constructor-callback-or-unresolved |
| `first_free_title` | `folder_name         .filter(&#124;name&#124; validate_title(name).is_ok())         .unwrap_or` | [3647](../../src/management.rs#L3647) | receiver-type-required |
| `first_free_title` | `folder_name         .filter` | [3647](../../src/management.rs#L3647) | receiver-type-required |
| `first_free_title` | `validate_title(name).is_ok` | [3648](../../src/management.rs#L3648) | receiver-type-required |
| `first_free_title` | `validate_title` | [3648](../../src/management.rs#L3648), [3656](../../src/management.rs#L3656) | [endpoint::management::validate_title](../../src/management.rs#L3613) |
| `first_free_title` | `(1..=configs.len() + 1)         .map(&#124;index&#124; match index {             1 => base.to_owned(),             index => format!("{base} ({index})"),         })         .find(&#124;title&#124; {             validate_title(title).is_ok() && !configs.values().any(&#124;config&#124; &config.name == title)         })         .ok_or_else` | [3650](../../src/management.rs#L3650) | receiver-type-required |
| `first_free_title` | `(1..=configs.len() + 1)         .map(&#124;index&#124; match index {             1 => base.to_owned(),             index => format!("{base} ({index})"),         })         .find` | [3650](../../src/management.rs#L3650) | receiver-type-required |
| `first_free_title` | `(1..=configs.len() + 1)         .map` | [3650](../../src/management.rs#L3650) | receiver-type-required |
| `first_free_title` | `configs.len` | [3650](../../src/management.rs#L3650) | receiver-type-required |
| `first_free_title` | `base.to_owned` | [3652](../../src/management.rs#L3652), [3658](../../src/management.rs#L3658) | receiver-type-required |
| `first_free_title` | `validate_title(title).is_ok` | [3656](../../src/management.rs#L3656) | receiver-type-required |
| `first_free_title` | `configs.values().any` | [3656](../../src/management.rs#L3656) | receiver-type-required |
| `first_free_title` | `configs.values` | [3656](../../src/management.rs#L3656) | receiver-type-required |
| `first_free_title` | `ManagementError::NameConflict` | [3658](../../src/management.rs#L3658) | external-constructor-callback-or-unresolved |
| `ensure_unique_title` | `configs         .values()         .any` | [3666](../../src/management.rs#L3666) | receiver-type-required |
| `ensure_unique_title` | `configs         .values` | [3666](../../src/management.rs#L3666) | receiver-type-required |
| `ensure_unique_title` | `Some` | [3668](../../src/management.rs#L3668) | external-constructor-callback-or-unresolved |
| `ensure_unique_title` | `config.id.as_str` | [3668](../../src/management.rs#L3668) | receiver-type-required |
| `ensure_unique_title` | `Err` | [3670](../../src/management.rs#L3670) | external-constructor-callback-or-unresolved |
| `ensure_unique_title` | `ManagementError::NameConflict` | [3670](../../src/management.rs#L3670) | external-constructor-callback-or-unresolved |
| `ensure_unique_title` | `title.to_owned` | [3670](../../src/management.rs#L3670) | receiver-type-required |
| `ensure_unique_title` | `Ok` | [3672](../../src/management.rs#L3672) | external-constructor-callback-or-unresolved |
| `verify_metadata` | `hex_digest` | [3679](../../src/management.rs#L3679) | [endpoint::management::hex_digest](../../src/management.rs#L4245) |
| `verify_metadata` | `canonical_line` | [3679](../../src/management.rs#L3679) | [endpoint::management::canonical_line](../../src/management.rs#L3773) |
| `verify_metadata` | `primary_path` | [3682](../../src/management.rs#L3682) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `verify_metadata` | `Err` | [3685](../../src/management.rs#L3685) | external-constructor-callback-or-unresolved |
| `verify_metadata` | `ManagementError::CorruptOperation` | [3685](../../src/management.rs#L3685) | external-constructor-callback-or-unresolved |
| `verify_metadata` | `Ok` | [3690](../../src/management.rs#L3690) | external-constructor-callback-or-unresolved |
| `workspace_view` | `members         .iter()         .map(&#124;session&#124; session.session_id.clone())         .collect::<Vec<_>>` | [3698](../../src/management.rs#L3698) | receiver-type-required |
| `workspace_view` | `members         .iter()         .map` | [3698](../../src/management.rs#L3698) | receiver-type-required |
| `workspace_view` | `members         .iter` | [3698](../../src/management.rs#L3698), [3703](../../src/management.rs#L3703) | receiver-type-required |
| `workspace_view` | `session.session_id.clone` | [3700](../../src/management.rs#L3700) | receiver-type-required |
| `workspace_view` | `session_ids.sort` | [3702](../../src/management.rs#L3702) | receiver-type-required |
| `workspace_view` | `members         .iter()         .filter_map(&#124;session&#124; timestamp_from_millis(session.updated_at).ok())         .chain(std::iter::once(metadata.metadata_updated_at.clone()))         .max()         .expect` | [3703](../../src/management.rs#L3703) | receiver-type-required |
| `workspace_view` | `members         .iter()         .filter_map(&#124;session&#124; timestamp_from_millis(session.updated_at).ok())         .chain(std::iter::once(metadata.metadata_updated_at.clone()))         .max` | [3703](../../src/management.rs#L3703) | receiver-type-required |
| `workspace_view` | `members         .iter()         .filter_map(&#124;session&#124; timestamp_from_millis(session.updated_at).ok())         .chain` | [3703](../../src/management.rs#L3703) | receiver-type-required |
| `workspace_view` | `members         .iter()         .filter_map` | [3703](../../src/management.rs#L3703) | receiver-type-required |
| `workspace_view` | `timestamp_from_millis(session.updated_at).ok` | [3705](../../src/management.rs#L3705) | receiver-type-required |
| `workspace_view` | `timestamp_from_millis` | [3705](../../src/management.rs#L3705) | [endpoint::management::timestamp_from_millis](../../src/management.rs#L3719) |
| `workspace_view` | `std::iter::once` | [3706](../../src/management.rs#L3706) | external-constructor-callback-or-unresolved |
| `workspace_view` | `metadata.metadata_updated_at.clone` | [3706](../../src/management.rs#L3706) | receiver-type-required |
| `workspace_view` | `Ok` | [3709](../../src/management.rs#L3709) | external-constructor-callback-or-unresolved |
| `workspace_view` | `config.id.clone` | [3710](../../src/management.rs#L3710) | receiver-type-required |
| `workspace_view` | `primary_path(config)?.to_owned` | [3711](../../src/management.rs#L3711) | receiver-type-required |
| `workspace_view` | `primary_path` | [3711](../../src/management.rs#L3711) | [endpoint::management::primary_path](../../src/management.rs#L3103) |
| `workspace_view` | `config.name.clone` | [3712](../../src/management.rs#L3712) | receiver-type-required |
| `timestamp_from_millis` | `i64::try_from(seconds / 86_400)         .map_err` | [3722](../../src/management.rs#L3722) | receiver-type-required |
| `timestamp_from_millis` | `i64::try_from` | [3722](../../src/management.rs#L3722) | external-constructor-callback-or-unresolved |
| `timestamp_from_millis` | `ManagementError::CorruptOperation` | [3723](../../src/management.rs#L3723) | external-constructor-callback-or-unresolved |
| `timestamp_from_millis` | `"timestamp overflow".to_owned` | [3723](../../src/management.rs#L3723) | receiver-type-required |
| `timestamp_from_millis` | `i64::from` | [3734](../../src/management.rs#L3734) | external-constructor-callback-or-unresolved |
| `timestamp_from_millis` | `Ok` | [3735](../../src/management.rs#L3735) | external-constructor-callback-or-unresolved |
| `publish_canonical` | `AtomicPublisher::replace` | [3744](../../src/management.rs#L3744) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_canonical` | `canonical_line` | [3744](../../src/management.rs#L3744) | [endpoint::management::canonical_line](../../src/management.rs#L3773) |
| `publish_canonical` | `Ok` | [3745](../../src/management.rs#L3745) | external-constructor-callback-or-unresolved |
| `read_canonical` | `decode_canonical` | [3749](../../src/management.rs#L3749) | [endpoint::management::decode_canonical](../../src/management.rs#L3752) |
| `read_canonical` | `fs::read` | [3749](../../src/management.rs#L3749) | external-constructor-callback-or-unresolved |
| `decode_canonical` | `bytes.ends_with` | [3756](../../src/management.rs#L3756) | receiver-type-required |
| `decode_canonical` | `bytes[..bytes.len().saturating_sub(1)].ends_with` | [3756](../../src/management.rs#L3756) | receiver-type-required |
| `decode_canonical` | `bytes.len().saturating_sub` | [3756](../../src/management.rs#L3756) | receiver-type-required |
| `decode_canonical` | `bytes.len` | [3756](../../src/management.rs#L3756), [3762](../../src/management.rs#L3762) | receiver-type-required |
| `decode_canonical` | `Err` | [3757](../../src/management.rs#L3757), [3765](../../src/management.rs#L3765) | external-constructor-callback-or-unresolved |
| `decode_canonical` | `ManagementError::CorruptOperation` | [3757](../../src/management.rs#L3757), [3765](../../src/management.rs#L3765) | external-constructor-callback-or-unresolved |
| `decode_canonical` | `IJsonValue::parse` | [3763](../../src/management.rs#L3763) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `decode_canonical` | `value.canonical_bytes` | [3764](../../src/management.rs#L3764) | receiver-type-required |
| `decode_canonical` | `serde_json::from_slice(body).map_err` | [3770](../../src/management.rs#L3770) | receiver-type-required |
| `decode_canonical` | `serde_json::from_slice` | [3770](../../src/management.rs#L3770) | external-constructor-callback-or-unresolved |
| `canonical_line` | `serde_json_canonicalizer::to_vec(value)         .map_err` | [3774](../../src/management.rs#L3774) | receiver-type-required |
| `canonical_line` | `serde_json_canonicalizer::to_vec` | [3774](../../src/management.rs#L3774) | external-constructor-callback-or-unresolved |
| `canonical_line` | `ManagementError::Canonical` | [3775](../../src/management.rs#L3775) | external-constructor-callback-or-unresolved |
| `canonical_line` | `error.to_string` | [3775](../../src/management.rs#L3775) | receiver-type-required |
| `canonical_line` | `bytes.push` | [3776](../../src/management.rs#L3776) | receiver-type-required |
| `canonical_line` | `Ok` | [3777](../../src/management.rs#L3777) | external-constructor-callback-or-unresolved |
| `to_ijson` | `IJsonValue::parse(         &serde_json_canonicalizer::to_vec(value)             .map_err(&#124;error&#124; ManagementError::Canonical(error.to_string()))?,     )     .map_err` | [3781](../../src/management.rs#L3781) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse` | [3781](../../src/management.rs#L3781) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `to_ijson` | `serde_json_canonicalizer::to_vec(value)             .map_err` | [3782](../../src/management.rs#L3782) | receiver-type-required |
| `to_ijson` | `serde_json_canonicalizer::to_vec` | [3782](../../src/management.rs#L3782) | external-constructor-callback-or-unresolved |
| `to_ijson` | `ManagementError::Canonical` | [3783](../../src/management.rs#L3783) | external-constructor-callback-or-unresolved |
| `to_ijson` | `error.to_string` | [3783](../../src/management.rs#L3783) | receiver-type-required |
| `intent_kind` | `record.intent.canonical_bytes` | [3789](../../src/management.rs#L3789) | receiver-type-required |
| `intent_kind` | `serde_json::from_slice` | [3790](../../src/management.rs#L3790) | external-constructor-callback-or-unresolved |
| `intent_kind` | `value         .as_object()         .and_then(&#124;object&#124; object.get("kind"))         .and_then(Value::as_str)         .map(str::to_owned)         .ok_or_else` | [3791](../../src/management.rs#L3791) | receiver-type-required |
| `intent_kind` | `value         .as_object()         .and_then(&#124;object&#124; object.get("kind"))         .and_then(Value::as_str)         .map` | [3791](../../src/management.rs#L3791) | receiver-type-required |
| `intent_kind` | `value         .as_object()         .and_then(&#124;object&#124; object.get("kind"))         .and_then` | [3791](../../src/management.rs#L3791) | receiver-type-required |
| `intent_kind` | `value         .as_object()         .and_then` | [3791](../../src/management.rs#L3791) | receiver-type-required |
| `intent_kind` | `value         .as_object` | [3791](../../src/management.rs#L3791) | receiver-type-required |
| `intent_kind` | `object.get` | [3793](../../src/management.rs#L3793) | receiver-type-required |
| `intent_kind` | `ManagementError::CorruptOperation` | [3796](../../src/management.rs#L3796) | external-constructor-callback-or-unresolved |
| `intent_kind` | `"operation intent has no kind".to_owned` | [3796](../../src/management.rs#L3796) | receiver-type-required |
| `workspace_intent` | `record.intent.canonical_bytes` | [3800](../../src/management.rs#L3800) | receiver-type-required |
| `workspace_intent` | `serde_json::from_slice` | [3801](../../src/management.rs#L3801) | external-constructor-callback-or-unresolved |
| `workspace_intent` | `intent.action.as_str` | [3808](../../src/management.rs#L3808) | receiver-type-required |
| `workspace_intent` | `relocation.iter().all` | [3810](../../src/management.rs#L3810), [3814](../../src/management.rs#L3814), [3818](../../src/management.rs#L3818) | receiver-type-required |
| `workspace_intent` | `relocation.iter` | [3810](../../src/management.rs#L3810), [3814](../../src/management.rs#L3814), [3818](../../src/management.rs#L3818) | receiver-type-required |
| `workspace_intent` | `field.is_none` | [3810](../../src/management.rs#L3810), [3811](../../src/management.rs#L3811), [3815](../../src/management.rs#L3815), [3818](../../src/management.rs#L3818) | receiver-type-required |
| `workspace_intent` | `folder.iter().all` | [3811](../../src/management.rs#L3811), [3815](../../src/management.rs#L3815), [3819](../../src/management.rs#L3819) | receiver-type-required |
| `workspace_intent` | `folder.iter` | [3811](../../src/management.rs#L3811), [3815](../../src/management.rs#L3815), [3819](../../src/management.rs#L3819) | receiver-type-required |
| `workspace_intent` | `field.is_some` | [3814](../../src/management.rs#L3814), [3819](../../src/management.rs#L3819) | receiver-type-required |
| `workspace_intent` | `Err` | [3824](../../src/management.rs#L3824) | external-constructor-callback-or-unresolved |
| `workspace_intent` | `ManagementError::CorruptOperation` | [3824](../../src/management.rs#L3824) | external-constructor-callback-or-unresolved |
| `workspace_intent` | `"operation is not a workspace intent".to_owned` | [3825](../../src/management.rs#L3825) | receiver-type-required |
| `workspace_intent` | `Ok` | [3828](../../src/management.rs#L3828) | external-constructor-callback-or-unresolved |
| `discard_intent` | `record.intent.canonical_bytes` | [3832](../../src/management.rs#L3832) | receiver-type-required |
| `discard_intent` | `serde_json::from_slice` | [3833](../../src/management.rs#L3833) | external-constructor-callback-or-unresolved |
| `discard_intent` | `Err` | [3835](../../src/management.rs#L3835) | external-constructor-callback-or-unresolved |
| `discard_intent` | `ManagementError::CorruptOperation` | [3835](../../src/management.rs#L3835) | external-constructor-callback-or-unresolved |
| `discard_intent` | `"operation is not a discard intent".to_owned` | [3836](../../src/management.rs#L3836) | receiver-type-required |
| `discard_intent` | `crate::validate_session_id` | [3839](../../src/management.rs#L3839) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `discard_intent` | `Ok` | [3840](../../src/management.rs#L3840) | external-constructor-callback-or-unresolved |
| `folder_is_ephemeral` | `Ok` | [3844](../../src/management.rs#L3844) | external-constructor-callback-or-unresolved |
| `folder_is_ephemeral` | `store::read_genesis(folder.join("main.jsonl"))?         .as_ref()         .is_some_and` | [3844](../../src/management.rs#L3844) | receiver-type-required |
| `folder_is_ephemeral` | `store::read_genesis(folder.join("main.jsonl"))?         .as_ref` | [3844](../../src/management.rs#L3844) | receiver-type-required |
| `folder_is_ephemeral` | `store::read_genesis` | [3844](../../src/management.rs#L3844) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `folder_is_ephemeral` | `folder.join` | [3844](../../src/management.rs#L3844) | receiver-type-required |
| `folder_move_intent` | `record.intent.canonical_bytes` | [3850](../../src/management.rs#L3850) | receiver-type-required |
| `folder_move_intent` | `serde_json::from_slice` | [3851](../../src/management.rs#L3851) | external-constructor-callback-or-unresolved |
| `folder_move_intent` | `Err` | [3853](../../src/management.rs#L3853) | external-constructor-callback-or-unresolved |
| `folder_move_intent` | `ManagementError::CorruptOperation` | [3853](../../src/management.rs#L3853) | external-constructor-callback-or-unresolved |
| `folder_move_intent` | `"operation is not a folder-move intent".to_owned` | [3854](../../src/management.rs#L3854) | receiver-type-required |
| `folder_move_intent` | `Ok` | [3857](../../src/management.rs#L3857) | external-constructor-callback-or-unresolved |
| `session_create_intent` | `record.intent.canonical_bytes` | [3861](../../src/management.rs#L3861) | receiver-type-required |
| `session_create_intent` | `serde_json::from_slice` | [3862](../../src/management.rs#L3862) | external-constructor-callback-or-unresolved |
| `session_create_intent` | `is_hex_digest` | [3864](../../src/management.rs#L3864), [3870](../../src/management.rs#L3870) | [endpoint::management::is_hex_digest](../../src/management.rs#L3407) |
| `session_create_intent` | `intent                 .config_asset                 .strip_prefix("sha256-")                 .unwrap_or_default` | [3865](../../src/management.rs#L3865) | receiver-type-required |
| `session_create_intent` | `intent                 .config_asset                 .strip_prefix` | [3865](../../src/management.rs#L3865) | receiver-type-required |
| `session_create_intent` | `intent                 .instruction_asset                 .strip_prefix("sha256-")                 .unwrap_or_default` | [3871](../../src/management.rs#L3871) | receiver-type-required |
| `session_create_intent` | `intent                 .instruction_asset                 .strip_prefix` | [3871](../../src/management.rs#L3871) | receiver-type-required |
| `session_create_intent` | `Err` | [3877](../../src/management.rs#L3877) | external-constructor-callback-or-unresolved |
| `session_create_intent` | `ManagementError::CorruptOperation` | [3877](../../src/management.rs#L3877) | external-constructor-callback-or-unresolved |
| `session_create_intent` | `"operation is not a valid session-create intent".to_owned` | [3878](../../src/management.rs#L3878) | receiver-type-required |
| `session_create_intent` | `crate::validate_session_id` | [3881](../../src/management.rs#L3881) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `session_create_intent` | `Ok` | [3882](../../src/management.rs#L3882) | external-constructor-callback-or-unresolved |
| `select_model_intent` | `record.intent.canonical_bytes` | [3886](../../src/management.rs#L3886) | receiver-type-required |
| `select_model_intent` | `serde_json::from_slice` | [3887](../../src/management.rs#L3887) | external-constructor-callback-or-unresolved |
| `select_model_intent` | `intent.provider.is_empty` | [3890](../../src/management.rs#L3890) | receiver-type-required |
| `select_model_intent` | `intent.model.is_empty` | [3891](../../src/management.rs#L3891) | receiver-type-required |
| `select_model_intent` | `intent             .reasoning_effort             .as_deref()             .is_some_and` | [3892](../../src/management.rs#L3892) | receiver-type-required |
| `select_model_intent` | `intent             .reasoning_effort             .as_deref` | [3892](../../src/management.rs#L3892) | receiver-type-required |
| `select_model_intent` | `Err` | [3897](../../src/management.rs#L3897) | external-constructor-callback-or-unresolved |
| `select_model_intent` | `ManagementError::CorruptOperation` | [3897](../../src/management.rs#L3897) | external-constructor-callback-or-unresolved |
| `select_model_intent` | `"operation is not a valid select-model intent".to_owned` | [3898](../../src/management.rs#L3898) | receiver-type-required |
| `select_model_intent` | `crate::validate_session_id` | [3901](../../src/management.rs#L3901) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `select_model_intent` | `Ok` | [3902](../../src/management.rs#L3902) | external-constructor-callback-or-unresolved |
| `fork_intent` | `record.intent.canonical_bytes` | [3906](../../src/management.rs#L3906) | receiver-type-required |
| `fork_intent` | `serde_json::from_slice` | [3907](../../src/management.rs#L3907) | external-constructor-callback-or-unresolved |
| `fork_intent` | `intent.rewrite_op_id.is_empty` | [3910](../../src/management.rs#L3910) | receiver-type-required |
| `fork_intent` | `is_uid_principal` | [3911](../../src/management.rs#L3911) | [endpoint::management::is_uid_principal](../../src/management.rs#L4113) |
| `fork_intent` | `Err` | [3913](../../src/management.rs#L3913) | external-constructor-callback-or-unresolved |
| `fork_intent` | `ManagementError::CorruptOperation` | [3913](../../src/management.rs#L3913) | external-constructor-callback-or-unresolved |
| `fork_intent` | `"operation is not a valid fork intent".to_owned` | [3914](../../src/management.rs#L3914) | receiver-type-required |
| `fork_intent` | `crate::validate_session_id` | [3917](../../src/management.rs#L3917), [3918](../../src/management.rs#L3918) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `fork_intent` | `Ok` | [3919](../../src/management.rs#L3919) | external-constructor-callback-or-unresolved |
| `queue_transaction_intent` | `record.intent.canonical_bytes` | [3925](../../src/management.rs#L3925) | receiver-type-required |
| `queue_transaction_intent` | `serde_json::from_slice` | [3926](../../src/management.rs#L3926) | external-constructor-callback-or-unresolved |
| `queue_transaction_intent` | `intent.replacement_origin.as_ref().is_some_and` | [3932](../../src/management.rs#L3932) | receiver-type-required |
| `queue_transaction_intent` | `intent.replacement_origin.as_ref` | [3932](../../src/management.rs#L3932) | receiver-type-required |
| `queue_transaction_intent` | `intent             .asset_digests             .iter()             .any` | [3937](../../src/management.rs#L3937) | receiver-type-required |
| `queue_transaction_intent` | `intent             .asset_digests             .iter` | [3937](../../src/management.rs#L3937) | receiver-type-required |
| `queue_transaction_intent` | `asset.strip_prefix("sha256-").is_some_and` | [3940](../../src/management.rs#L3940) | receiver-type-required |
| `queue_transaction_intent` | `asset.strip_prefix` | [3940](../../src/management.rs#L3940) | receiver-type-required |
| `queue_transaction_intent` | `Err` | [3942](../../src/management.rs#L3942) | external-constructor-callback-or-unresolved |
| `queue_transaction_intent` | `ManagementError::CorruptOperation` | [3942](../../src/management.rs#L3942) | external-constructor-callback-or-unresolved |
| `queue_transaction_intent` | `"operation is not a valid queue-transaction intent".to_owned` | [3943](../../src/management.rs#L3943) | receiver-type-required |
| `queue_transaction_intent` | `crate::validate_session_id` | [3946](../../src/management.rs#L3946) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `queue_transaction_intent` | `Ok` | [3947](../../src/management.rs#L3947) | external-constructor-callback-or-unresolved |
| `pending_queue_transaction` | `queue_transaction_intent` | [3953](../../src/management.rs#L3953) | [endpoint::management::queue_transaction_intent](../../src/management.rs#L3922) |
| `pending_queue_transaction` | `Ok` | [3954](../../src/management.rs#L3954) | external-constructor-callback-or-unresolved |
| `pending_queue_transaction` | `record.rpc_id.clone` | [3955](../../src/management.rs#L3955) | receiver-type-required |
| `verify_queue_payload` | `payload.join` | [3969](../../src/management.rs#L3969), [3976](../../src/management.rs#L3976), [3991](../../src/management.rs#L3991) | receiver-type-required |
| `verify_queue_payload` | `fs::read` | [3970](../../src/management.rs#L3970) | external-constructor-callback-or-unresolved |
| `verify_queue_payload` | `intent.action.canonical_bytes` | [3971](../../src/management.rs#L3971) | receiver-type-required |
| `verify_queue_payload` | `Err` | [3972](../../src/management.rs#L3972), [3986](../../src/management.rs#L3986), [3992](../../src/management.rs#L3992) | external-constructor-callback-or-unresolved |
| `verify_queue_payload` | `ManagementError::CorruptOperation` | [3972](../../src/management.rs#L3972), [3986](../../src/management.rs#L3986), [3992](../../src/management.rs#L3992) | external-constructor-callback-or-unresolved |
| `verify_queue_payload` | `"queue action staging bytes disagree with prepared intent".to_owned` | [3973](../../src/management.rs#L3973) | receiver-type-required |
| `verify_queue_payload` | `read_canonical` | [3976](../../src/management.rs#L3976) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `verify_queue_payload` | `snapshot             .valid_prefix_digest             .strip_prefix("sha256-")             .is_some_and` | [3981](../../src/management.rs#L3981) | receiver-type-required |
| `verify_queue_payload` | `snapshot             .valid_prefix_digest             .strip_prefix` | [3981](../../src/management.rs#L3981) | receiver-type-required |
| `verify_queue_payload` | `"queue tail snapshot disagrees with prepared intent".to_owned` | [3987](../../src/management.rs#L3987) | receiver-type-required |
| `verify_queue_payload` | `payload.join("assets").join(asset).is_file` | [3991](../../src/management.rs#L3991) | receiver-type-required |
| `verify_queue_payload` | `payload.join("assets").join` | [3991](../../src/management.rs#L3991) | receiver-type-required |
| `verify_queue_payload` | `Ok` | [3997](../../src/management.rs#L3997) | external-constructor-callback-or-unresolved |
| `completion_for_decision` | `code.clone` | [4005](../../src/management.rs#L4005) | receiver-type-required |
| `completion_for_decision` | `reason.clone` | [4006](../../src/management.rs#L4006) | receiver-type-required |
| `queue_completion_value` | `code.as_str` | [4023](../../src/management.rs#L4023) | receiver-type-required |
| `queue_completion_value` | `reason.as_deref` | [4023](../../src/management.rs#L4023) | receiver-type-required |
| `queue_completion_value` | `reason.is_empty` | [4032](../../src/management.rs#L4032) | receiver-type-required |
| `queue_completion_value` | `Err` | [4036](../../src/management.rs#L4036) | external-constructor-callback-or-unresolved |
| `queue_completion_value` | `ManagementError::CorruptOperation` | [4036](../../src/management.rs#L4036) | external-constructor-callback-or-unresolved |
| `queue_completion_value` | `"queue rejection is outside the closed union".to_owned` | [4037](../../src/management.rs#L4037) | receiver-type-required |
| `queue_completion_value` | `to_ijson` | [4047](../../src/management.rs#L4047) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `queue_completion_from_record` | `Err` | [4054](../../src/management.rs#L4054), [4090](../../src/management.rs#L4090) | external-constructor-callback-or-unresolved |
| `queue_completion_from_record` | `ManagementError::CorruptOperation` | [4054](../../src/management.rs#L4054), [4060](../../src/management.rs#L4060), [4071](../../src/management.rs#L4071), [4076](../../src/management.rs#L4076), [4090](../../src/management.rs#L4090) | external-constructor-callback-or-unresolved |
| `queue_completion_from_record` | `"queue operation is not complete".to_owned` | [4055](../../src/management.rs#L4055) | receiver-type-required |
| `queue_completion_from_record` | `queue_transaction_intent` | [4058](../../src/management.rs#L4058) | [endpoint::management::queue_transaction_intent](../../src/management.rs#L3922) |
| `queue_completion_from_record` | `record.response.as_ref().ok_or_else` | [4059](../../src/management.rs#L4059) | receiver-type-required |
| `queue_completion_from_record` | `record.response.as_ref` | [4059](../../src/management.rs#L4059) | receiver-type-required |
| `queue_completion_from_record` | `"complete queue operation lacks response".to_owned` | [4060](../../src/management.rs#L4060) | receiver-type-required |
| `queue_completion_from_record` | `response.canonical_bytes` | [4062](../../src/management.rs#L4062) | receiver-type-required |
| `queue_completion_from_record` | `serde_json::from_slice` | [4063](../../src/management.rs#L4063) | external-constructor-callback-or-unresolved |
| `queue_completion_from_record` | `value.get` | [4064](../../src/management.rs#L4064) | receiver-type-required |
| `queue_completion_from_record` | `Some` | [4064](../../src/management.rs#L4064) | external-constructor-callback-or-unresolved |
| `queue_completion_from_record` | `Value::Bool` | [4064](../../src/management.rs#L4064) | external-constructor-callback-or-unresolved |
| `queue_completion_from_record` | `value             .get("error")             .and_then(Value::as_object)             .ok_or_else` | [4067](../../src/management.rs#L4067) | receiver-type-required |
| `queue_completion_from_record` | `value             .get("error")             .and_then` | [4067](../../src/management.rs#L4067) | receiver-type-required |
| `queue_completion_from_record` | `value             .get` | [4067](../../src/management.rs#L4067) | receiver-type-required |
| `queue_completion_from_record` | `"queue completion lacks its exact error".to_owned` | [4072](../../src/management.rs#L4072) | receiver-type-required |
| `queue_completion_from_record` | `error.get("code").and_then(Value::as_str).ok_or_else` | [4075](../../src/management.rs#L4075) | receiver-type-required |
| `queue_completion_from_record` | `error.get("code").and_then` | [4075](../../src/management.rs#L4075) | receiver-type-required |
| `queue_completion_from_record` | `error.get` | [4075](../../src/management.rs#L4075) | receiver-type-required |
| `queue_completion_from_record` | `"queue completion lacks error code".to_owned` | [4076](../../src/management.rs#L4076) | receiver-type-required |
| `queue_completion_from_record` | `error             .get("details")             .and_then(Value::as_object)             .and_then(&#124;details&#124; details.get("reason"))             .and_then(Value::as_str)             .map` | [4078](../../src/management.rs#L4078) | receiver-type-required |
| `queue_completion_from_record` | `error             .get("details")             .and_then(Value::as_object)             .and_then(&#124;details&#124; details.get("reason"))             .and_then` | [4078](../../src/management.rs#L4078) | receiver-type-required |
| `queue_completion_from_record` | `error             .get("details")             .and_then(Value::as_object)             .and_then` | [4078](../../src/management.rs#L4078) | receiver-type-required |
| `queue_completion_from_record` | `error             .get("details")             .and_then` | [4078](../../src/management.rs#L4078) | receiver-type-required |
| `queue_completion_from_record` | `error             .get` | [4078](../../src/management.rs#L4078) | receiver-type-required |
| `queue_completion_from_record` | `details.get` | [4081](../../src/management.rs#L4081) | receiver-type-required |
| `queue_completion_from_record` | `code.to_owned` | [4085](../../src/management.rs#L4085) | receiver-type-required |
| `queue_completion_from_record` | `queue_completion_value` | [4089](../../src/management.rs#L4089) | [endpoint::management::queue_completion_value](../../src/management.rs#L4012) |
| `queue_completion_from_record` | `"queue completion response is not exact".to_owned` | [4091](../../src/management.rs#L4091) | receiver-type-required |
| `queue_completion_from_record` | `Ok` | [4094](../../src/management.rs#L4094) | external-constructor-callback-or-unresolved |
| `operation_target_session` | `Ok` | [4098](../../src/management.rs#L4098) | external-constructor-callback-or-unresolved |
| `operation_target_session` | `intent_kind(record)?.as_str` | [4098](../../src/management.rs#L4098) | receiver-type-required |
| `operation_target_session` | `intent_kind` | [4098](../../src/management.rs#L4098) | [endpoint::management::intent_kind](../../src/management.rs#L3788) |
| `operation_target_session` | `Some` | [4099](../../src/management.rs#L4099), [4100](../../src/management.rs#L4100), [4101](../../src/management.rs#L4101), [4102](../../src/management.rs#L4102), [4103](../../src/management.rs#L4103) | external-constructor-callback-or-unresolved |
| `operation_target_session` | `session_create_intent` | [4099](../../src/management.rs#L4099) | [endpoint::management::session_create_intent](../../src/management.rs#L3860) |
| `operation_target_session` | `select_model_intent` | [4100](../../src/management.rs#L4100) | [endpoint::management::select_model_intent](../../src/management.rs#L3885) |
| `operation_target_session` | `folder_move_intent` | [4101](../../src/management.rs#L4101) | [endpoint::management::folder_move_intent](../../src/management.rs#L3849) |
| `operation_target_session` | `fork_intent` | [4102](../../src/management.rs#L4102) | [endpoint::management::fork_intent](../../src/management.rs#L3905) |
| `operation_target_session` | `queue_transaction_intent` | [4103](../../src/management.rs#L4103) | [endpoint::management::queue_transaction_intent](../../src/management.rs#L3922) |
| `operation_target_session` | `Err` | [4106](../../src/management.rs#L4106) | external-constructor-callback-or-unresolved |
| `operation_target_session` | `ManagementError::CorruptOperation` | [4106](../../src/management.rs#L4106) | external-constructor-callback-or-unresolved |
| `is_uid_principal` | `value.strip_prefix("uid:").is_some_and` | [4114](../../src/management.rs#L4114) | receiver-type-required |
| `is_uid_principal` | `value.strip_prefix` | [4114](../../src/management.rs#L4114) | receiver-type-required |
| `is_uid_principal` | `digits.is_empty` | [4115](../../src/management.rs#L4115) | receiver-type-required |
| `is_uid_principal` | `digits.bytes().all` | [4115](../../src/management.rs#L4115) | receiver-type-required |
| `is_uid_principal` | `digits.bytes` | [4115](../../src/management.rs#L4115) | receiver-type-required |
| `is_uid_principal` | `byte.is_ascii_digit` | [4115](../../src/management.rs#L4115) | receiver-type-required |
| `fork_id_from_response` | `response         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then(&#124;object&#124; object.get("sessionId"))         .and_then(Value::as_str)         .map(str::to_owned)         .ok_or_else` | [4120](../../src/management.rs#L4120) | receiver-type-required |
| `fork_id_from_response` | `response         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then(&#124;object&#124; object.get("sessionId"))         .and_then(Value::as_str)         .map` | [4120](../../src/management.rs#L4120) | receiver-type-required |
| `fork_id_from_response` | `response         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then(&#124;object&#124; object.get("sessionId"))         .and_then` | [4120](../../src/management.rs#L4120) | receiver-type-required |
| `fork_id_from_response` | `response         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then` | [4120](../../src/management.rs#L4120) | receiver-type-required |
| `fork_id_from_response` | `response         .as_object()         .filter` | [4120](../../src/management.rs#L4120) | receiver-type-required |
| `fork_id_from_response` | `response         .as_object` | [4120](../../src/management.rs#L4120) | receiver-type-required |
| `fork_id_from_response` | `object.len` | [4122](../../src/management.rs#L4122) | receiver-type-required |
| `fork_id_from_response` | `object.get` | [4123](../../src/management.rs#L4123) | receiver-type-required |
| `fork_id_from_response` | `ManagementError::CorruptOperation` | [4127](../../src/management.rs#L4127) | external-constructor-callback-or-unresolved |
| `fork_id_from_response` | `"fork response lacks exact sessionId".to_owned` | [4127](../../src/management.rs#L4127) | receiver-type-required |
| `selected_model_from_response` | `response         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then(&#124;object&#124; object.get("selected"))         .and_then(Value::as_object)         .ok_or_else` | [4132](../../src/management.rs#L4132) | receiver-type-required |
| `selected_model_from_response` | `response         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then(&#124;object&#124; object.get("selected"))         .and_then` | [4132](../../src/management.rs#L4132) | receiver-type-required |
| `selected_model_from_response` | `response         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then` | [4132](../../src/management.rs#L4132) | receiver-type-required |
| `selected_model_from_response` | `response         .as_object()         .filter` | [4132](../../src/management.rs#L4132) | receiver-type-required |
| `selected_model_from_response` | `response         .as_object` | [4132](../../src/management.rs#L4132) | receiver-type-required |
| `selected_model_from_response` | `object.len` | [4134](../../src/management.rs#L4134) | receiver-type-required |
| `selected_model_from_response` | `object.get` | [4135](../../src/management.rs#L4135) | receiver-type-required |
| `selected_model_from_response` | `ManagementError::CorruptOperation` | [4138](../../src/management.rs#L4138), [4146](../../src/management.rs#L4146), [4151](../../src/management.rs#L4151), [4156](../../src/management.rs#L4156) | external-constructor-callback-or-unresolved |
| `selected_model_from_response` | `"select-model response lacks exact selected value".to_owned` | [4139](../../src/management.rs#L4139) | receiver-type-required |
| `selected_model_from_response` | `selected         .get("provider")         .and_then(Value::as_str)         .ok_or_else` | [4142](../../src/management.rs#L4142) | receiver-type-required |
| `selected_model_from_response` | `selected         .get("provider")         .and_then` | [4142](../../src/management.rs#L4142) | receiver-type-required |
| `selected_model_from_response` | `selected         .get` | [4142](../../src/management.rs#L4142), [4148](../../src/management.rs#L4148), [4152](../../src/management.rs#L4152) | receiver-type-required |
| `selected_model_from_response` | `"selected provider is missing".to_owned` | [4146](../../src/management.rs#L4146) | receiver-type-required |
| `selected_model_from_response` | `selected         .get("model")         .and_then(Value::as_str)         .ok_or_else` | [4148](../../src/management.rs#L4148) | receiver-type-required |
| `selected_model_from_response` | `selected         .get("model")         .and_then` | [4148](../../src/management.rs#L4148) | receiver-type-required |
| `selected_model_from_response` | `"selected model is missing".to_owned` | [4151](../../src/management.rs#L4151) | receiver-type-required |
| `selected_model_from_response` | `selected         .get("reasoningEffort")         .map(&#124;value&#124; {             value.as_str().map(str::to_owned).ok_or_else(&#124;&#124; {                 ManagementError::CorruptOperation(                     "selected reasoning effort is not a string".to_owned(),                 )             })         })         .transpose` | [4152](../../src/management.rs#L4152) | receiver-type-required |
| `selected_model_from_response` | `selected         .get("reasoningEffort")         .map` | [4152](../../src/management.rs#L4152) | receiver-type-required |
| `selected_model_from_response` | `value.as_str().map(str::to_owned).ok_or_else` | [4155](../../src/management.rs#L4155) | receiver-type-required |
| `selected_model_from_response` | `value.as_str().map` | [4155](../../src/management.rs#L4155) | receiver-type-required |
| `selected_model_from_response` | `value.as_str` | [4155](../../src/management.rs#L4155) | receiver-type-required |
| `selected_model_from_response` | `"selected reasoning effort is not a string".to_owned` | [4157](../../src/management.rs#L4157) | receiver-type-required |
| `selected_model_from_response` | `Ok` | [4162](../../src/management.rs#L4162) | external-constructor-callback-or-unresolved |
| `selected_model_from_response` | `provider.to_owned` | [4163](../../src/management.rs#L4163) | receiver-type-required |
| `selected_model_from_response` | `model.to_owned` | [4164](../../src/management.rs#L4164) | receiver-type-required |
| `active_session_folder` | `root.join("archive").join(session_id).is_dir` | [4170](../../src/management.rs#L4170) | receiver-type-required |
| `active_session_folder` | `root.join("archive").join` | [4170](../../src/management.rs#L4170) | receiver-type-required |
| `active_session_folder` | `root.join` | [4170](../../src/management.rs#L4170), [4173](../../src/management.rs#L4173) | receiver-type-required |
| `active_session_folder` | `Err` | [4171](../../src/management.rs#L4171), [4175](../../src/management.rs#L4175) | external-constructor-callback-or-unresolved |
| `active_session_folder` | `ManagementError::SessionArchived` | [4171](../../src/management.rs#L4171) | external-constructor-callback-or-unresolved |
| `active_session_folder` | `session_id.to_owned` | [4171](../../src/management.rs#L4171), [4175](../../src/management.rs#L4175) | receiver-type-required |
| `active_session_folder` | `root.join("threads").join` | [4173](../../src/management.rs#L4173) | receiver-type-required |
| `active_session_folder` | `folder.is_dir` | [4174](../../src/management.rs#L4174) | receiver-type-required |
| `active_session_folder` | `ManagementError::SessionNotFound` | [4175](../../src/management.rs#L4175) | external-constructor-callback-or-unresolved |
| `active_session_folder` | `Ok` | [4177](../../src/management.rs#L4177) | external-constructor-callback-or-unresolved |
| `valid_folder_projection` | `store::read_ledger_repaired` | [4182](../../src/management.rs#L4182) | [store::tail::read_ledger_repaired](../../../store/src/tail.rs#L651) |
| `valid_folder_projection` | `folder.join` | [4182](../../src/management.rs#L4182) | receiver-type-required |
| `valid_folder_projection` | `Err` | [4184](../../src/management.rs#L4184) | external-constructor-callback-or-unresolved |
| `valid_folder_projection` | `ManagementError::CorruptOperation` | [4184](../../src/management.rs#L4184), [4189](../../src/management.rs#L4189) | external-constructor-callback-or-unresolved |
| `valid_folder_projection` | `"session ledger has a torn or invalid tail".to_owned` | [4185](../../src/management.rs#L4185) | receiver-type-required |
| `valid_folder_projection` | `scan.projection.ok_or_else` | [4188](../../src/management.rs#L4188) | receiver-type-required |
| `valid_folder_projection` | `"session ledger has no valid genesis".to_owned` | [4189](../../src/management.rs#L4189) | receiver-type-required |
| `folder_reservation` | `fs::metadata` | [4194](../../src/management.rs#L4194) | external-constructor-callback-or-unresolved |
| `folder_reservation` | `store::read_ledger_repaired` | [4195](../../src/management.rs#L4195) | [store::tail::read_ledger_repaired](../../../store/src/tail.rs#L651) |
| `folder_reservation` | `folder.join` | [4195](../../src/management.rs#L4195) | receiver-type-required |
| `folder_reservation` | `scan.projection.is_none` | [4196](../../src/management.rs#L4196) | receiver-type-required |
| `folder_reservation` | `Err` | [4197](../../src/management.rs#L4197) | external-constructor-callback-or-unresolved |
| `folder_reservation` | `ManagementError::CorruptOperation` | [4197](../../src/management.rs#L4197), [4202](../../src/management.rs#L4202) | external-constructor-callback-or-unresolved |
| `folder_reservation` | `"folder move source has no complete valid ledger prefix".to_owned` | [4198](../../src/management.rs#L4198) | receiver-type-required |
| `folder_reservation` | `usize::try_from(scan.valid_bytes).map_err` | [4201](../../src/management.rs#L4201) | receiver-type-required |
| `folder_reservation` | `usize::try_from` | [4201](../../src/management.rs#L4201) | external-constructor-callback-or-unresolved |
| `folder_reservation` | `"valid prefix length exceeds usize".to_owned` | [4202](../../src/management.rs#L4202) | receiver-type-required |
| `folder_reservation` | `Ok` | [4204](../../src/management.rs#L4204) | external-constructor-callback-or-unresolved |
| `verify_folder_reservation` | `folder_reservation` | [4214](../../src/management.rs#L4214) | [endpoint::management::folder_reservation](../../src/management.rs#L4193) |
| `verify_folder_reservation` | `Err` | [4216](../../src/management.rs#L4216) | external-constructor-callback-or-unresolved |
| `verify_folder_reservation` | `ManagementError::CorruptOperation` | [4216](../../src/management.rs#L4216) | external-constructor-callback-or-unresolved |
| `verify_folder_reservation` | `"folder move reservation no longer matches target".to_owned` | [4217](../../src/management.rs#L4217) | receiver-type-required |
| `verify_folder_reservation` | `Ok` | [4220](../../src/management.rs#L4220) | external-constructor-callback-or-unresolved |
| `folder_workspace` | `store::read_genesis(folder.join("main.jsonl"))?.ok_or_else` | [4224](../../src/management.rs#L4224) | receiver-type-required |
| `folder_workspace` | `store::read_genesis` | [4224](../../src/management.rs#L4224) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `folder_workspace` | `folder.join` | [4224](../../src/management.rs#L4224) | receiver-type-required |
| `folder_workspace` | `ManagementError::CorruptOperation` | [4225](../../src/management.rs#L4225), [4231](../../src/management.rs#L4231) | external-constructor-callback-or-unresolved |
| `folder_workspace` | `"folder ledger lacks genesis".to_owned` | [4225](../../src/management.rs#L4225) | receiver-type-required |
| `folder_workspace` | `genesis         .string_field("workspace")         .map(str::to_owned)         .ok_or_else` | [4227](../../src/management.rs#L4227) | receiver-type-required |
| `folder_workspace` | `genesis         .string_field("workspace")         .map` | [4227](../../src/management.rs#L4227) | receiver-type-required |
| `folder_workspace` | `genesis         .string_field` | [4227](../../src/management.rs#L4227) | receiver-type-required |
| `folder_workspace` | `"folder genesis lacks workspace".to_owned` | [4231](../../src/management.rs#L4231) | receiver-type-required |
| `active_session_count` | `fs::read_dir` | [4237](../../src/management.rs#L4237) | external-constructor-callback-or-unresolved |
| `active_session_count` | `root.join` | [4237](../../src/management.rs#L4237) | receiver-type-required |
| `active_session_count` | `entry?.file_type()?.is_dir` | [4238](../../src/management.rs#L4238) | receiver-type-required |
| `active_session_count` | `entry?.file_type` | [4238](../../src/management.rs#L4238) | receiver-type-required |
| `active_session_count` | `Ok` | [4242](../../src/management.rs#L4242) | external-constructor-callback-or-unresolved |
| `omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed` | `"test".to_owned` | [4351](../../src/management.rs#L4351) | receiver-type-required |
| `omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed` | `"session-endpoint".to_owned` | [4352](../../src/management.rs#L4352) | receiver-type-required |
| `omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed` | `"018f0000-0000-7000-8000-000000000003".to_owned` | [4353](../../src/management.rs#L4353) | receiver-type-required |
| `omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed` | `"session.create".to_owned` | [4354](../../src/management.rs#L4354) | receiver-type-required |
| `omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed` | `"create".to_owned` | [4355](../../src/management.rs#L4355) | receiver-type-required |
| `omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed` | `create_genesis(                 &origin.target,                 "ws",                 "folder-1",                 "cfg",                 "ins",                 identity,                 None,                 "2026-09-27T00:00:00.000Z",                 &origin,             )             .unwrap` | [4358](../../src/management.rs#L4358) | receiver-type-required |
| `omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed` | `create_genesis` | [4358](../../src/management.rs#L4358) | [endpoint::management::create_genesis](../../src/management.rs#L3265) |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `"test".to_owned` | [4385](../../src/management.rs#L4385) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `"session-endpoint".to_owned` | [4386](../../src/management.rs#L4386) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `"018f0000-0000-7000-8000-000000000003".to_owned` | [4387](../../src/management.rs#L4387) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `"session.create".to_owned` | [4388](../../src/management.rs#L4388) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `"create".to_owned` | [4389](../../src/management.rs#L4389) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `session_preset(             Some(&request),             None,             tools::PresetBase::General,             &tools::NoPresetEnvironment,         )         .unwrap()         .expect` | [4393](../../src/management.rs#L4393) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `session_preset(             Some(&request),             None,             tools::PresetBase::General,             &tools::NoPresetEnvironment,         )         .unwrap` | [4393](../../src/management.rs#L4393) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `session_preset` | [4393](../../src/management.rs#L4393) | [endpoint::management::session_preset](../../src/management.rs#L3228) |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `Some` | [4394](../../src/management.rs#L4394), [4408](../../src/management.rs#L4408) | external-constructor-callback-or-unresolved |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `create_genesis(             &origin.target,             "ws",             "folder-1",             "cfg",             "ins",             None,             Some(&preset),             "2026-09-27T00:00:00.000Z",             &origin,         )         .unwrap` | [4401](../../src/management.rs#L4401) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `create_genesis` | [4401](../../src/management.rs#L4401) | [endpoint::management::create_genesis](../../src/management.rs#L3265) |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `serde_json::from_slice(&genesis.canonical_bytes().unwrap()).unwrap` | [4415](../../src/management.rs#L4415) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `serde_json::from_slice` | [4415](../../src/management.rs#L4415) | external-constructor-callback-or-unresolved |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `genesis.canonical_bytes().unwrap` | [4415](../../src/management.rs#L4415) | receiver-type-required |
| `preset_genesis_replaces_identity_and_needs_reader_two` | `genesis.canonical_bytes` | [4415](../../src/management.rs#L4415) | receiver-type-required |
| `preset_skills_resolve_against_package_names` | `profile::EffectiveInstructions::default` | [4481](../../src/management.rs#L4481) | external-constructor-callback-or-unresolved |
| `preset_skills_resolve_against_package_names` | `super::SessionPresetEnvironment::new` | [4483](../../src/management.rs#L4483) | [endpoint::management::SessionPresetEnvironment::new](../../src/management.rs#L273) |
| `preset_skills_resolve_against_package_names` | `session_preset` | [4485](../../src/management.rs#L4485) | [endpoint::management::session_preset](../../src/management.rs#L3228) |
| `preset_skills_resolve_against_package_names` | `Some` | [4486](../../src/management.rs#L4486) | external-constructor-callback-or-unresolved |
| `preset_skills_resolve_against_package_names` | `resolve(json!(["review"])).unwrap().expect` | [4492](../../src/management.rs#L4492) | receiver-type-required |
| `preset_skills_resolve_against_package_names` | `resolve(json!(["review"])).unwrap` | [4492](../../src/management.rs#L4492) | receiver-type-required |
| `preset_skills_resolve_against_package_names` | `resolve` | [4492](../../src/management.rs#L4492) | external-constructor-callback-or-unresolved |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `tempfile::tempdir().unwrap` | [4520](../../src/management.rs#L4520) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `tempfile::tempdir` | [4520](../../src/management.rs#L4520) | external-constructor-callback-or-unresolved |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `ManagementStore::open_at(root.path(), timestamp).unwrap` | [4522](../../src/management.rs#L4522), [4575](../../src/management.rs#L4575) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `ManagementStore::open_at` | [4522](../../src/management.rs#L4522), [4575](../../src/management.rs#L4575) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `root.path` | [4522](../../src/management.rs#L4522), [4523](../../src/management.rs#L4523), [4526](../../src/management.rs#L4526), [4575](../../src/management.rs#L4575) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `ThreadStore::open(root.path()).unwrap` | [4523](../../src/management.rs#L4523) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `ThreadStore::open` | [4523](../../src/management.rs#L4523) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `root.path().join("threads").join` | [4526](../../src/management.rs#L4526) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `root.path().join` | [4526](../../src/management.rs#L4526) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `fs::create_dir(&folder).unwrap` | [4527](../../src/management.rs#L4527) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `fs::create_dir` | [4527](../../src/management.rs#L4527) | external-constructor-callback-or-unresolved |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `baseline             .split(&#124;b&#124; *b == b'\n')             .filter(&#124;line&#124; !line.is_empty())             .map(&#124;line&#124; serde_json::from_slice(line).unwrap())             .collect` | [4529](../../src/management.rs#L4529) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `baseline             .split(&#124;b&#124; *b == b'\n')             .filter(&#124;line&#124; !line.is_empty())             .map` | [4529](../../src/management.rs#L4529) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `baseline             .split(&#124;b&#124; *b == b'\n')             .filter` | [4529](../../src/management.rs#L4529) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `baseline             .split` | [4529](../../src/management.rs#L4529) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `line.is_empty` | [4531](../../src/management.rs#L4531) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `serde_json::from_slice(line).unwrap` | [4532](../../src/management.rs#L4532) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `serde_json::from_slice` | [4532](../../src/management.rs#L4532) | external-constructor-callback-or-unresolved |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `events             .iter_mut()             .find(&#124;event&#124; event["kind"] == "run_start")             .unwrap()             .as_object_mut()             .unwrap()             .insert` | [4534](../../src/management.rs#L4534) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `events             .iter_mut()             .find(&#124;event&#124; event["kind"] == "run_start")             .unwrap()             .as_object_mut()             .unwrap` | [4534](../../src/management.rs#L4534) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `events             .iter_mut()             .find(&#124;event&#124; event["kind"] == "run_start")             .unwrap()             .as_object_mut` | [4534](../../src/management.rs#L4534) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `events             .iter_mut()             .find(&#124;event&#124; event["kind"] == "run_start")             .unwrap` | [4534](../../src/management.rs#L4534) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `events             .iter_mut()             .find` | [4534](../../src/management.rs#L4534) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `events             .iter_mut` | [4534](../../src/management.rs#L4534) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `"launch_bindings_digest".into` | [4540](../../src/management.rs#L4540) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `Vec::new` | [4541](../../src/management.rs#L4541) | external-constructor-callback-or-unresolved |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `ledger.extend` | [4543](../../src/management.rs#L4543) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `serde_json_canonicalizer::to_vec(&event).unwrap` | [4543](../../src/management.rs#L4543) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `serde_json_canonicalizer::to_vec` | [4543](../../src/management.rs#L4543) | external-constructor-callback-or-unresolved |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `ledger.push` | [4544](../../src/management.rs#L4544) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `fs::write(folder.join("main.jsonl"), &ledger).unwrap` | [4546](../../src/management.rs#L4546) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `fs::write` | [4546](../../src/management.rs#L4546) | external-constructor-callback-or-unresolved |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `folder.join` | [4546](../../src/management.rs#L4546) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `store             .begin_fork("failed-fork", source, dest, timestamp)             .unwrap` | [4547](../../src/management.rs#L4547) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `store             .begin_fork` | [4547](../../src/management.rs#L4547) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `"failed-fork-rpc".into` | [4552](../../src/management.rs#L4552) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `"session.fork".into` | [4553](../../src/management.rs#L4553) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `"0".repeat` | [4554](../../src/management.rs#L4554) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `timestamp.into` | [4556](../../src/management.rs#L4556) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `to_ijson(&ForkIntent {                 kind: "fork".into(),                 source: source.into(),                 dest: dest.into(),                 at_endpoint_seq: None,                 kernel_anchor: 8,                 rewrite_op_id: "failed-fork".into(),                 principal: "uid:501".into(),                 ephemeral: false,                 preset: None,             })             .unwrap` | [4558](../../src/management.rs#L4558) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `to_ijson` | [4558](../../src/management.rs#L4558) | [endpoint::management::to_ijson](../../src/management.rs#L3780) |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `"fork".into` | [4559](../../src/management.rs#L4559) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `source.into` | [4560](../../src/management.rs#L4560) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `dest.into` | [4561](../../src/management.rs#L4561) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `"failed-fork".into` | [4564](../../src/management.rs#L4564) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `"uid:501".into` | [4565](../../src/management.rs#L4565) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `management.operation_path` | [4571](../../src/management.rs#L4571) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `publish_canonical(&path, &record).unwrap` | [4572](../../src/management.rs#L4572) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `publish_canonical` | [4572](../../src/management.rs#L4572) | [endpoint::management::publish_canonical](../../src/management.rs#L3743) |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `store.recover_rewrites_for_startup().unwrap` | [4573](../../src/management.rs#L4573) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `store.recover_rewrites_for_startup` | [4573](../../src/management.rs#L4573) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `management.recover().unwrap` | [4574](../../src/management.rs#L4574) | receiver-type-required |
| `quarantined_fork_does_not_block_recovery_or_claim_success` | `management.recover` | [4574](../../src/management.rs#L4574) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `tempfile::tempdir().expect` | [4596](../../src/management.rs#L4596) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `tempfile::tempdir` | [4596](../../src/management.rs#L4596) | external-constructor-callback-or-unresolved |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `root.path().join` | [4597](../../src/management.rs#L4597) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `root.path` | [4597](../../src/management.rs#L4597), [4611](../../src/management.rs#L4611), [4614](../../src/management.rs#L4614), [4637](../../src/management.rs#L4637), [4650](../../src/management.rs#L4650), [4660](../../src/management.rs#L4660), [4673](../../src/management.rs#L4673) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::create_dir(&project).expect` | [4598](../../src/management.rs#L4598) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::create_dir` | [4598](../../src/management.rs#L4598) | external-constructor-callback-or-unresolved |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `"ws-light".to_owned` | [4602](../../src/management.rs#L4602) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `"Light".to_owned` | [4603](../../src/management.rs#L4603) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `Vec::new` | [4604](../../src/management.rs#L4604), [4616](../../src/management.rs#L4616) | external-constructor-callback-or-unresolved |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ConfigRepository::open(root.path()).expect` | [4611](../../src/management.rs#L4611) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ConfigRepository::open` | [4611](../../src/management.rs#L4611) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository.publish_workspace(0, &config).expect` | [4612](../../src/management.rs#L4612) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository.publish_workspace` | [4612](../../src/management.rs#L4612) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_at(root.path(), "2026-10-12T00:00:00.000Z")             .expect` | [4614](../../src/management.rs#L4614) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_at` | [4614](../../src/management.rs#L4614) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `recovering             .list_workspaces(&sessions)             .expect` | [4617](../../src/management.rs#L4617) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `recovering             .list_workspaces` | [4617](../../src/management.rs#L4617) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `root             .path()             .join` | [4622](../../src/management.rs#L4622) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `root             .path` | [4622](../../src/management.rs#L4622) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::create_dir_all(&trap).expect` | [4625](../../src/management.rs#L4625) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::create_dir_all` | [4625](../../src/management.rs#L4625) | external-constructor-callback-or-unresolved |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::set_permissions(&trap, std::fs::Permissions::from_mode(0o000)).expect` | [4626](../../src/management.rs#L4626) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::set_permissions` | [4626](../../src/management.rs#L4626), [4679](../../src/management.rs#L4679) | external-constructor-callback-or-unresolved |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::Permissions::from_mode` | [4626](../../src/management.rs#L4626), [4679](../../src/management.rs#L4679) | external-constructor-callback-or-unresolved |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository.workspace("ws-light").expect` | [4629](../../src/management.rs#L4629), [4644](../../src/management.rs#L4644), [4657](../../src/management.rs#L4657) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository.workspace` | [4629](../../src/management.rs#L4629), [4644](../../src/management.rs#L4644), [4657](../../src/management.rs#L4657) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository             .publish_workspace_policy(                 "ws-light",                 current.revision,                 default_workspace_policy(project.to_str().unwrap()),             )             .expect` | [4630](../../src/management.rs#L4630) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository             .publish_workspace_policy` | [4630](../../src/management.rs#L4630) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `default_workspace_policy` | [4634](../../src/management.rs#L4634) | [endpoint::management::default_workspace_policy](../../src/management.rs#L3421) |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `project.to_str().unwrap` | [4634](../../src/management.rs#L4634) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `project.to_str` | [4634](../../src/management.rs#L4634) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light(root.path()).expect` | [4637](../../src/management.rs#L4637), [4660](../../src/management.rs#L4660) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light` | [4637](../../src/management.rs#L4637), [4650](../../src/management.rs#L4650), [4660](../../src/management.rs#L4660) | [endpoint::management::ManagementStore::open_light](../../src/management.rs#L533) |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `light             .list_workspaces(&sessions)             .expect` | [4638](../../src/management.rs#L4638) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `light             .list_workspaces` | [4638](../../src/management.rs#L4638) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `"Renamed".to_owned` | [4645](../../src/management.rs#L4645) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository             .publish_workspace(renamed.revision - 1, &renamed)             .expect` | [4647](../../src/management.rs#L4647) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository             .publish_workspace` | [4647](../../src/management.rs#L4647), [4661](../../src/management.rs#L4661) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light(root.path())             .expect("light open")             .list_workspaces(&sessions)             .expect` | [4650](../../src/management.rs#L4650) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light(root.path())             .expect("light open")             .list_workspaces` | [4650](../../src/management.rs#L4650) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light(root.path())             .expect` | [4650](../../src/management.rs#L4650) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `"Third".to_owned` | [4658](../../src/management.rs#L4658) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `repository             .publish_workspace(third.revision - 1, &third)             .expect` | [4661](../../src/management.rs#L4661) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `stale_view             .list_workspaces(&sessions)             .expect` | [4664](../../src/management.rs#L4664) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `stale_view             .list_workspaces` | [4664](../../src/management.rs#L4664) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `light.metadata_path` | [4670](../../src/management.rs#L4670) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `read_canonical(&metadata_path).expect` | [4671](../../src/management.rs#L4671) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `read_canonical` | [4671](../../src/management.rs#L4671) | [endpoint::management::read_canonical](../../src/management.rs#L3748) |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::remove_file(&metadata_path).expect` | [4672](../../src/management.rs#L4672) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::remove_file` | [4672](../../src/management.rs#L4672) | external-constructor-callback-or-unresolved |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light_at(root.path(), "2026-10-12T05:00:00.000Z")             .expect("light open")             .read_metadata("ws-light")             .expect` | [4673](../../src/management.rs#L4673) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light_at(root.path(), "2026-10-12T05:00:00.000Z")             .expect("light open")             .read_metadata` | [4673](../../src/management.rs#L4673) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light_at(root.path(), "2026-10-12T05:00:00.000Z")             .expect` | [4673](../../src/management.rs#L4673) | receiver-type-required |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `ManagementStore::open_light_at` | [4673](../../src/management.rs#L4673) | [endpoint::management::ManagementStore::open_light_at](../../src/management.rs#L537) |
| `open_light_heals_metadata_after_any_config_writer_without_reading_threads` | `std::fs::set_permissions(&trap, std::fs::Permissions::from_mode(0o700)).expect` | [4679](../../src/management.rs#L4679) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `tempfile::tempdir().expect` | [4684](../../src/management.rs#L4684) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `tempfile::tempdir` | [4684](../../src/management.rs#L4684) | external-constructor-callback-or-unresolved |
| `unavailable_workspace_keeps_its_management_operation_pending` | `root.path().join` | [4685](../../src/management.rs#L4685) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `root.path` | [4685](../../src/management.rs#L4685), [4687](../../src/management.rs#L4687), [4716](../../src/management.rs#L4716) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `std::fs::create_dir(&project).expect` | [4686](../../src/management.rs#L4686) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `std::fs::create_dir` | [4686](../../src/management.rs#L4686) | external-constructor-callback-or-unresolved |
| `unavailable_workspace_keeps_its_management_operation_pending` | `ManagementStore::open_at(root.path(), "2026-09-01T00:00:00.000Z")             .expect` | [4687](../../src/management.rs#L4687) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `ManagementStore::open_at` | [4687](../../src/management.rs#L4687), [4716](../../src/management.rs#L4716) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `unavailable_workspace_keeps_its_management_operation_pending` | `"workspace-offline".to_owned` | [4692](../../src/management.rs#L4692) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `"Offline".to_owned` | [4693](../../src/management.rs#L4693) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `Vec::new` | [4694](../../src/management.rs#L4694) | external-constructor-callback-or-unresolved |
| `unavailable_workspace_keeps_its_management_operation_pending` | `store             .begin_workspace_operation(WorkspaceOperationWrite {                 rpc_id: "workspace-offline-create",                 request_sha256: &"0".repeat(64),                 operation: "workspace.create",                 action: "create",                 config: &config,                 started_at: "2026-09-01T00:00:00.000Z",                 created: Some(true),                 relocation: None,                 folder: None,             })             .expect` | [4701](../../src/management.rs#L4701) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `store             .begin_workspace_operation` | [4701](../../src/management.rs#L4701) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `"0".repeat` | [4704](../../src/management.rs#L4704) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `Some` | [4709](../../src/management.rs#L4709) | external-constructor-callback-or-unresolved |
| `unavailable_workspace_keeps_its_management_operation_pending` | `std::fs::remove_dir(&project).expect` | [4714](../../src/management.rs#L4714) | receiver-type-required |
| `unavailable_workspace_keeps_its_management_operation_pending` | `std::fs::remove_dir` | [4714](../../src/management.rs#L4714) | external-constructor-callback-or-unresolved |
| `unavailable_workspace_keeps_its_management_operation_pending` | `ManagementStore::open_at(root.path(), "2026-09-01T00:00:01.000Z")             .expect` | [4716](../../src/management.rs#L4716) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `tempfile::tempdir().expect` | [4723](../../src/management.rs#L4723) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `tempfile::tempdir` | [4723](../../src/management.rs#L4723) | external-constructor-callback-or-unresolved |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `ManagementStore::open_at(root.path(), "2026-09-01T00:00:00.000Z")             .expect` | [4724](../../src/management.rs#L4724) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `ManagementStore::open_at` | [4724](../../src/management.rs#L4724), [4766](../../src/management.rs#L4766) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `root.path` | [4724](../../src/management.rs#L4724), [4766](../../src/management.rs#L4766) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `store.operation_payload_path` | [4727](../../src/management.rs#L4727) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `std::fs::create_dir_all(&payload).expect` | [4728](../../src/management.rs#L4728) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `std::fs::create_dir_all` | [4728](../../src/management.rs#L4728), [4758](../../src/management.rs#L4758) | external-constructor-callback-or-unresolved |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `serde_json_canonicalizer::to_vec(&candidate).expect` | [4735](../../src/management.rs#L4735) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `serde_json_canonicalizer::to_vec` | [4735](../../src/management.rs#L4735), [4759](../../src/management.rs#L4759) | external-constructor-callback-or-unresolved |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `bytes.push` | [4736](../../src/management.rs#L4736) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `std::fs::write(payload.join("session-settings-v1.json"), &candidate_bytes)             .expect` | [4739](../../src/management.rs#L4739) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `std::fs::write` | [4739](../../src/management.rs#L4739), [4761](../../src/management.rs#L4761) | external-constructor-callback-or-unresolved |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `payload.join` | [4739](../../src/management.rs#L4739) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `store.operation_path` | [4757](../../src/management.rs#L4757) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `std::fs::create_dir_all(operation_path.parent().expect("prefix")).expect` | [4758](../../src/management.rs#L4758) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `operation_path.parent().expect` | [4758](../../src/management.rs#L4758) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `operation_path.parent` | [4758](../../src/management.rs#L4758) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `serde_json_canonicalizer::to_vec(&record).expect` | [4759](../../src/management.rs#L4759) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `record_bytes.push` | [4760](../../src/management.rs#L4760) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `std::fs::write(&operation_path, &record_bytes).expect` | [4761](../../src/management.rs#L4761) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `recovered             .err()             .map(&#124;error&#124; error.to_string())             .unwrap_or_default` | [4769](../../src/management.rs#L4769) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `recovered             .err()             .map` | [4769](../../src/management.rs#L4769) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `recovered             .err` | [4769](../../src/management.rs#L4769) | receiver-type-required |
| `a_select_model_payload_staged_before_the_rename_is_read_on_recovery` | `error.to_string` | [4771](../../src/management.rs#L4771) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `tempfile::tempdir().expect` | [4791](../../src/management.rs#L4791) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `tempfile::tempdir` | [4791](../../src/management.rs#L4791) | external-constructor-callback-or-unresolved |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `root.path().join` | [4792](../../src/management.rs#L4792), [4795](../../src/management.rs#L4795), [4820](../../src/management.rs#L4820) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `root.path` | [4792](../../src/management.rs#L4792), [4795](../../src/management.rs#L4795), [4820](../../src/management.rs#L4820) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `std::fs::create_dir(&project).expect` | [4793](../../src/management.rs#L4793) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `std::fs::create_dir` | [4793](../../src/management.rs#L4793) | external-constructor-callback-or-unresolved |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `ManagementStore::open_at(root.path().join("kernel"), "2026-09-14T00:00:00.000Z")                 .expect` | [4795](../../src/management.rs#L4795) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `ManagementStore::open_at` | [4795](../../src/management.rs#L4795) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `store             .create_workspace(                 "seeded-create",                 &"2".repeat(64),                 &project.to_string_lossy(),                 "2026-09-14T00:00:00.000Z",             )             .expect` | [4797](../../src/management.rs#L4797) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `store             .create_workspace` | [4797](../../src/management.rs#L4797) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `"2".repeat` | [4800](../../src/management.rs#L4800) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `project.to_string_lossy` | [4801](../../src/management.rs#L4801) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `store.workspace_configs().expect("configs")[&view.workspace_id].clone` | [4806](../../src/management.rs#L4806) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `store.workspace_configs().expect` | [4806](../../src/management.rs#L4806) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `store.workspace_configs` | [4806](../../src/management.rs#L4806) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `config.policy.expect` | [4807](../../src/management.rs#L4807) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `profile::ConfigRepository::open(root.path().join("kernel"))             .expect("repository")             .resolve(&view.workspace_id)             .expect` | [4820](../../src/management.rs#L4820) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `profile::ConfigRepository::open(root.path().join("kernel"))             .expect("repository")             .resolve` | [4820](../../src/management.rs#L4820) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `profile::ConfigRepository::open(root.path().join("kernel"))             .expect` | [4820](../../src/management.rs#L4820) | receiver-type-required |
| `a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder` | `profile::ConfigRepository::open` | [4820](../../src/management.rs#L4820) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `tempfile::tempdir().expect` | [4836](../../src/management.rs#L4836) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `tempfile::tempdir` | [4836](../../src/management.rs#L4836) | external-constructor-callback-or-unresolved |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `root.path().join` | [4837](../../src/management.rs#L4837), [4840](../../src/management.rs#L4840), [4866](../../src/management.rs#L4866), [4880](../../src/management.rs#L4880) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `root.path` | [4837](../../src/management.rs#L4837), [4840](../../src/management.rs#L4840), [4866](../../src/management.rs#L4866), [4880](../../src/management.rs#L4880) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `std::fs::create_dir(&project).expect` | [4838](../../src/management.rs#L4838) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `std::fs::create_dir` | [4838](../../src/management.rs#L4838) | external-constructor-callback-or-unresolved |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `ManagementStore::open_at(root.path().join("kernel"), "2026-09-14T00:00:00.000Z")                 .expect` | [4840](../../src/management.rs#L4840) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `ManagementStore::open_at` | [4840](../../src/management.rs#L4840), [4866](../../src/management.rs#L4866), [4880](../../src/management.rs#L4880) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `store             .create_workspace(                 "legacy-create",                 &"5".repeat(64),                 &project.to_string_lossy(),                 "2026-09-14T00:00:00.000Z",             )             .expect` | [4842](../../src/management.rs#L4842) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `store             .create_workspace` | [4842](../../src/management.rs#L4842) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `"5".repeat` | [4845](../../src/management.rs#L4845) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `project.to_string_lossy` | [4846](../../src/management.rs#L4846) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `root             .path()             .join("kernel")             .join("workspaces")             .join(&view.workspace_id)             .join` | [4851](../../src/management.rs#L4851) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `root             .path()             .join("kernel")             .join("workspaces")             .join` | [4851](../../src/management.rs#L4851) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `root             .path()             .join("kernel")             .join` | [4851](../../src/management.rs#L4851) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `root             .path()             .join` | [4851](../../src/management.rs#L4851) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `root             .path` | [4851](../../src/management.rs#L4851) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `serde_json::from_slice(&std::fs::read(&path).expect("document")).expect` | [4858](../../src/management.rs#L4858) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `serde_json::from_slice` | [4858](../../src/management.rs#L4858) | external-constructor-callback-or-unresolved |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `std::fs::read(&path).expect` | [4858](../../src/management.rs#L4858) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `std::fs::read` | [4858](../../src/management.rs#L4858) | external-constructor-callback-or-unresolved |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `document.as_object_mut().expect("object").remove` | [4859](../../src/management.rs#L4859) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `document.as_object_mut().expect` | [4859](../../src/management.rs#L4859) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `document.as_object_mut` | [4859](../../src/management.rs#L4859) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `serde_json_canonicalizer::to_vec(&document).expect` | [4860](../../src/management.rs#L4860) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `serde_json_canonicalizer::to_vec` | [4860](../../src/management.rs#L4860) | external-constructor-callback-or-unresolved |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `bytes.push` | [4861](../../src/management.rs#L4861) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `std::fs::write(&path, bytes).expect` | [4862](../../src/management.rs#L4862) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `std::fs::write` | [4862](../../src/management.rs#L4862) | external-constructor-callback-or-unresolved |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `drop` | [4863](../../src/management.rs#L4863), [4876](../../src/management.rs#L4876) | external-constructor-callback-or-unresolved |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `ManagementStore::open_at(root.path().join("kernel"), "2026-09-14T00:00:01.000Z")                 .expect` | [4866](../../src/management.rs#L4866) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `reopened.workspace_configs().expect("configs")[&view.workspace_id].clone` | [4868](../../src/management.rs#L4868) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `reopened.workspace_configs().expect` | [4868](../../src/management.rs#L4868) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `reopened.workspace_configs` | [4868](../../src/management.rs#L4868) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `config.policy.expect` | [4869](../../src/management.rs#L4869) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `ManagementStore::open_at(root.path().join("kernel"), "2026-09-14T00:00:02.000Z")                 .expect` | [4880](../../src/management.rs#L4880) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `again.workspace_configs().expect("configs")[&view.workspace_id].clone` | [4882](../../src/management.rs#L4882) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `again.workspace_configs().expect` | [4882](../../src/management.rs#L4882) | receiver-type-required |
| `opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once` | `again.workspace_configs` | [4882](../../src/management.rs#L4882) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `tempfile::tempdir().expect` | [4888](../../src/management.rs#L4888) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `tempfile::tempdir` | [4888](../../src/management.rs#L4888) | external-constructor-callback-or-unresolved |
| `relocation_moves_writable_roots_with_their_folder` | `root.path().join` | [4889](../../src/management.rs#L4889), [4892](../../src/management.rs#L4892), [4902](../../src/management.rs#L4902) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `root.path` | [4889](../../src/management.rs#L4889), [4892](../../src/management.rs#L4892), [4902](../../src/management.rs#L4902) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `std::fs::create_dir(&before).expect` | [4890](../../src/management.rs#L4890) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `std::fs::create_dir` | [4890](../../src/management.rs#L4890) | external-constructor-callback-or-unresolved |
| `relocation_moves_writable_roots_with_their_folder` | `ManagementStore::open_at(root.path().join("kernel"), "2026-09-14T00:00:00.000Z")                 .expect` | [4892](../../src/management.rs#L4892) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `ManagementStore::open_at` | [4892](../../src/management.rs#L4892) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `relocation_moves_writable_roots_with_their_folder` | `store             .create_workspace(                 "relocate-create",                 &"3".repeat(64),                 &before.to_string_lossy(),                 "2026-09-14T00:00:00.000Z",             )             .expect` | [4894](../../src/management.rs#L4894) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `store             .create_workspace` | [4894](../../src/management.rs#L4894) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `"3".repeat` | [4897](../../src/management.rs#L4897) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `before.to_string_lossy` | [4898](../../src/management.rs#L4898) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `std::fs::rename(&before, &after).expect` | [4903](../../src/management.rs#L4903) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `std::fs::rename` | [4903](../../src/management.rs#L4903) | external-constructor-callback-or-unresolved |
| `relocation_moves_writable_roots_with_their_folder` | `store             .relocate_workspace(                 "relocate-move",                 &"4".repeat(64),                 &view.workspace_id,                 &view.path,                 &after.to_string_lossy(),                 "2026-09-14T00:00:01.000Z",             )             .expect` | [4904](../../src/management.rs#L4904) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `store             .relocate_workspace` | [4904](../../src/management.rs#L4904) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `"4".repeat` | [4907](../../src/management.rs#L4907) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `after.to_string_lossy` | [4910](../../src/management.rs#L4910) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `store.workspace_configs().expect("configs")[&view.workspace_id].clone` | [4914](../../src/management.rs#L4914) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `store.workspace_configs().expect` | [4914](../../src/management.rs#L4914) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `store.workspace_configs` | [4914](../../src/management.rs#L4914) | receiver-type-required |
| `relocation_moves_writable_roots_with_their_folder` | `config.policy.expect` | [4915](../../src/management.rs#L4915) | receiver-type-required |
| `folder_store` | `tempfile::tempdir().expect` | [4924](../../src/management.rs#L4924) | receiver-type-required |
| `folder_store` | `tempfile::tempdir` | [4924](../../src/management.rs#L4924) | external-constructor-callback-or-unresolved |
| `folder_store` | `root.path().join` | [4925](../../src/management.rs#L4925), [4928](../../src/management.rs#L4928) | receiver-type-required |
| `folder_store` | `root.path` | [4925](../../src/management.rs#L4925), [4928](../../src/management.rs#L4928) | receiver-type-required |
| `folder_store` | `std::fs::create_dir(&project).expect` | [4926](../../src/management.rs#L4926) | receiver-type-required |
| `folder_store` | `std::fs::create_dir` | [4926](../../src/management.rs#L4926) | external-constructor-callback-or-unresolved |
| `folder_store` | `ManagementStore::open_at(root.path().join("kernel"), "2026-10-08T00:00:00.000Z")                 .expect` | [4928](../../src/management.rs#L4928) | receiver-type-required |
| `folder_store` | `ManagementStore::open_at` | [4928](../../src/management.rs#L4928) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `folder_store` | `store             .create_workspace(                 &format!("{name}-create"),                 &"5".repeat(64),                 &project.to_string_lossy(),                 "2026-10-08T00:00:00.000Z",             )             .expect` | [4930](../../src/management.rs#L4930) | receiver-type-required |
| `folder_store` | `store             .create_workspace` | [4930](../../src/management.rs#L4930) | receiver-type-required |
| `folder_store` | `"5".repeat` | [4933](../../src/management.rs#L4933) | receiver-type-required |
| `folder_store` | `project.to_string_lossy` | [4934](../../src/management.rs#L4934) | receiver-type-required |
| `sibling` | `root.path().join` | [4942](../../src/management.rs#L4942) | receiver-type-required |
| `sibling` | `root.path` | [4942](../../src/management.rs#L4942) | receiver-type-required |
| `sibling` | `std::fs::create_dir_all(&path).expect` | [4943](../../src/management.rs#L4943) | receiver-type-required |
| `sibling` | `std::fs::create_dir_all` | [4943](../../src/management.rs#L4943) | external-constructor-callback-or-unresolved |
| `sibling` | `path.canonicalize()             .expect("canonical")             .to_string_lossy()             .into_owned` | [4944](../../src/management.rs#L4944) | receiver-type-required |
| `sibling` | `path.canonicalize()             .expect("canonical")             .to_string_lossy` | [4944](../../src/management.rs#L4944) | receiver-type-required |
| `sibling` | `path.canonicalize()             .expect` | [4944](../../src/management.rs#L4944) | receiver-type-required |
| `sibling` | `path.canonicalize` | [4944](../../src/management.rs#L4944) | receiver-type-required |
| `write_session` | `"test".to_owned` | [4954](../../src/management.rs#L4954) | receiver-type-required |
| `write_session` | `"session-endpoint".to_owned` | [4955](../../src/management.rs#L4955) | receiver-type-required |
| `write_session` | `session_id.to_owned` | [4956](../../src/management.rs#L4956), [4958](../../src/management.rs#L4958) | receiver-type-required |
| `write_session` | `"session.create".to_owned` | [4957](../../src/management.rs#L4957) | receiver-type-required |
| `write_session` | `create_genesis(             session_id,             workspace_id,             binding.unwrap_or("folder-0001"),             &format!("sha256-{}", "a".repeat(64)),             &format!("sha256-{}", "b".repeat(64)),             None,             None,             "2026-10-08T00:00:00.000Z",             &origin,         )         .expect` | [4960](../../src/management.rs#L4960) | receiver-type-required |
| `write_session` | `create_genesis` | [4960](../../src/management.rs#L4960) | [endpoint::management::create_genesis](../../src/management.rs#L3265) |
| `write_session` | `binding.unwrap_or` | [4963](../../src/management.rs#L4963) | receiver-type-required |
| `write_session` | `serde_json::from_slice(&genesis.canonical_bytes().expect("bytes")).expect` | [4973](../../src/management.rs#L4973) | receiver-type-required |
| `write_session` | `serde_json::from_slice` | [4973](../../src/management.rs#L4973) | external-constructor-callback-or-unresolved |
| `write_session` | `genesis.canonical_bytes().expect` | [4973](../../src/management.rs#L4973) | receiver-type-required |
| `write_session` | `genesis.canonical_bytes` | [4973](../../src/management.rs#L4973) | receiver-type-required |
| `write_session` | `binding.is_none` | [4974](../../src/management.rs#L4974) | receiver-type-required |
| `write_session` | `value                 .as_object_mut()                 .expect("object")                 .remove` | [4975](../../src/management.rs#L4975) | receiver-type-required |
| `write_session` | `value                 .as_object_mut()                 .expect` | [4975](../../src/management.rs#L4975) | receiver-type-required |
| `write_session` | `value                 .as_object_mut` | [4975](../../src/management.rs#L4975) | receiver-type-required |
| `write_session` | `schema::Event::from_value(             schema::IJsonValue::parse(&serde_json::to_vec(&value).expect("json")).expect("ijson"),         )         .expect` | [4980](../../src/management.rs#L4980) | receiver-type-required |
| `write_session` | `schema::Event::from_value` | [4980](../../src/management.rs#L4980) | [schema::event::Event::from_value](../../../schema/src/event.rs#L179) |
| `write_session` | `schema::IJsonValue::parse(&serde_json::to_vec(&value).expect("json")).expect` | [4981](../../src/management.rs#L4981) | receiver-type-required |
| `write_session` | `schema::IJsonValue::parse` | [4981](../../src/management.rs#L4981) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `write_session` | `serde_json::to_vec(&value).expect` | [4981](../../src/management.rs#L4981) | receiver-type-required |
| `write_session` | `serde_json::to_vec` | [4981](../../src/management.rs#L4981) | external-constructor-callback-or-unresolved |
| `write_session` | `root.join("threads").join` | [4984](../../src/management.rs#L4984) | receiver-type-required |
| `write_session` | `root.join` | [4984](../../src/management.rs#L4984) | receiver-type-required |
| `write_session` | `std::fs::create_dir_all(&folder).expect` | [4985](../../src/management.rs#L4985) | receiver-type-required |
| `write_session` | `std::fs::create_dir_all` | [4985](../../src/management.rs#L4985) | external-constructor-callback-or-unresolved |
| `write_session` | `std::fs::write(             folder.join("main.jsonl"),             canonical_event_line(&event).expect("line"),         )         .expect` | [4986](../../src/management.rs#L4986) | receiver-type-required |
| `write_session` | `std::fs::write` | [4986](../../src/management.rs#L4986) | external-constructor-callback-or-unresolved |
| `write_session` | `folder.join` | [4987](../../src/management.rs#L4987) | receiver-type-required |
| `write_session` | `canonical_event_line(&event).expect` | [4988](../../src/management.rs#L4988) | receiver-type-required |
| `write_session` | `canonical_event_line` | [4988](../../src/management.rs#L4988) | [endpoint::management::canonical_event_line](../../src/management.rs#L3305) |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `folder_store` | [4995](../../src/management.rs#L4995) | [endpoint::management::tests::folder_store](../../src/management.rs#L4923) |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `sibling` | [4996](../../src/management.rs#L4996) | [endpoint::management::tests::sibling](../../src/management.rs#L4941) |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store             .add_workspace_folder(                 "folders-add",                 &"6".repeat(64),                 &view.workspace_id,                 &second,                 "2026-10-08T00:00:01.000Z",             )             .expect` | [4997](../../src/management.rs#L4997) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store             .add_workspace_folder` | [4997](../../src/management.rs#L4997), [5030](../../src/management.rs#L5030) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `"6".repeat` | [5000](../../src/management.rs#L5000), [5033](../../src/management.rs#L5033) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store.workspace_configs().unwrap()[&view.workspace_id].clone` | [5023](../../src/management.rs#L5023), [5085](../../src/management.rs#L5085) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store.workspace_configs().unwrap` | [5023](../../src/management.rs#L5023), [5085](../../src/management.rs#L5085) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store.workspace_configs` | [5023](../../src/management.rs#L5023), [5085](../../src/management.rs#L5085) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `expected_roots.sort` | [5025](../../src/management.rs#L5025) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store             .add_workspace_folder(                 "folders-add",                 &"6".repeat(64),                 &view.workspace_id,                 &second,                 "2026-10-08T00:00:09.000Z",             )             .expect` | [5030](../../src/management.rs#L5030) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store             .remove_workspace_folder(                 "folders-remove",                 &"a".repeat(64),                 &view.workspace_id,                 &view.path,                 "2026-10-08T00:00:03.000Z",             )             .expect` | [5072](../../src/management.rs#L5072) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `store             .remove_workspace_folder` | [5072](../../src/management.rs#L5072) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `"a".repeat` | [5075](../../src/management.rs#L5075) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `drop` | [5099](../../src/management.rs#L5099) | external-constructor-callback-or-unresolved |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `ManagementStore::open_at(root.path().join("kernel"), "2026-10-08T00:01:00.000Z")                 .expect` | [5101](../../src/management.rs#L5101) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `ManagementStore::open_at` | [5101](../../src/management.rs#L5101) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `root.path().join` | [5101](../../src/management.rs#L5101) | receiver-type-required |
| `folders_are_added_listed_and_removed_with_their_writable_roots` | `root.path` | [5101](../../src/management.rs#L5101) | receiver-type-required |
| `a_folder_bound_to_a_session_is_not_removed` | `folder_store` | [5111](../../src/management.rs#L5111) | [endpoint::management::tests::folder_store](../../src/management.rs#L4923) |
| `a_folder_bound_to_a_session_is_not_removed` | `sibling` | [5112](../../src/management.rs#L5112) | [endpoint::management::tests::sibling](../../src/management.rs#L4941) |
| `a_folder_bound_to_a_session_is_not_removed` | `store             .add_workspace_folder(                 "bound-add",                 &"6".repeat(64),                 &view.workspace_id,                 &second,                 "2026-10-08T00:00:01.000Z",             )             .expect` | [5113](../../src/management.rs#L5113) | receiver-type-required |
| `a_folder_bound_to_a_session_is_not_removed` | `store             .add_workspace_folder` | [5113](../../src/management.rs#L5113) | receiver-type-required |
| `a_folder_bound_to_a_session_is_not_removed` | `"6".repeat` | [5116](../../src/management.rs#L5116) | receiver-type-required |
| `a_folder_bound_to_a_session_is_not_removed` | `write_session` | [5123](../../src/management.rs#L5123) | [endpoint::management::tests::write_session](../../src/management.rs#L4952) |
| `a_folder_bound_to_a_session_is_not_removed` | `root.path().join` | [5124](../../src/management.rs#L5124) | receiver-type-required |
| `a_folder_bound_to_a_session_is_not_removed` | `root.path` | [5124](../../src/management.rs#L5124) | receiver-type-required |
| `a_folder_bound_to_a_session_is_not_removed` | `Some` | [5127](../../src/management.rs#L5127) | external-constructor-callback-or-unresolved |
| `a_folder_bound_to_a_session_is_not_removed` | `store.remove_workspace_folder` | [5129](../../src/management.rs#L5129) | receiver-type-required |
| `a_folder_bound_to_a_session_is_not_removed` | `"a".repeat` | [5131](../../src/management.rs#L5131) | receiver-type-required |
| `a_legacy_unbound_session_blocks_a_second_folder` | `folder_store` | [5157](../../src/management.rs#L5157) | [endpoint::management::tests::folder_store](../../src/management.rs#L4923) |
| `a_legacy_unbound_session_blocks_a_second_folder` | `sibling` | [5158](../../src/management.rs#L5158) | [endpoint::management::tests::sibling](../../src/management.rs#L4941) |
| `a_legacy_unbound_session_blocks_a_second_folder` | `write_session` | [5160](../../src/management.rs#L5160) | [endpoint::management::tests::write_session](../../src/management.rs#L4952) |
| `a_legacy_unbound_session_blocks_a_second_folder` | `root.path().join` | [5161](../../src/management.rs#L5161) | receiver-type-required |
| `a_legacy_unbound_session_blocks_a_second_folder` | `root.path` | [5161](../../src/management.rs#L5161) | receiver-type-required |
| `a_legacy_unbound_session_blocks_a_second_folder` | `store.add_workspace_folder` | [5166](../../src/management.rs#L5166) | receiver-type-required |
| `a_legacy_unbound_session_blocks_a_second_folder` | `"6".repeat` | [5168](../../src/management.rs#L5168) | receiver-type-required |
| `folder_edits_materialize_a_legacy_cwd_workspace_with_its_synthesized_ids` | `"legacy".to_owned` | [5193](../../src/management.rs#L5193), [5194](../../src/management.rs#L5194) | receiver-type-required |
| `folder_edits_materialize_a_legacy_cwd_workspace_with_its_synthesized_ids` | `Vec::new` | [5196](../../src/management.rs#L5196) | external-constructor-callback-or-unresolved |
| `folder_edits_materialize_a_legacy_cwd_workspace_with_its_synthesized_ids` | `super::with_added_folder` | [5199](../../src/management.rs#L5199) | [endpoint::management::with_added_folder](../../src/management.rs#L3488) |
| `completed_workspace_operation_never_reopens_its_old_path` | `tempfile::tempdir().expect` | [5222](../../src/management.rs#L5222) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `tempfile::tempdir` | [5222](../../src/management.rs#L5222) | external-constructor-callback-or-unresolved |
| `completed_workspace_operation_never_reopens_its_old_path` | `root.path().join` | [5223](../../src/management.rs#L5223), [5226](../../src/management.rs#L5226), [5238](../../src/management.rs#L5238) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `root.path` | [5223](../../src/management.rs#L5223), [5226](../../src/management.rs#L5226), [5238](../../src/management.rs#L5238) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `std::fs::create_dir(&project).expect` | [5224](../../src/management.rs#L5224) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `std::fs::create_dir` | [5224](../../src/management.rs#L5224) | external-constructor-callback-or-unresolved |
| `completed_workspace_operation_never_reopens_its_old_path` | `ManagementStore::open_at(root.path().join("kernel"), "2026-09-01T00:00:00.000Z")                 .expect` | [5226](../../src/management.rs#L5226) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `ManagementStore::open_at` | [5226](../../src/management.rs#L5226), [5238](../../src/management.rs#L5238) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `completed_workspace_operation_never_reopens_its_old_path` | `store             .create_workspace(                 "completed-workspace-create",                 &"1".repeat(64),                 &project.to_string_lossy(),                 "2026-09-01T00:00:00.000Z",             )             .expect` | [5228](../../src/management.rs#L5228) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `store             .create_workspace` | [5228](../../src/management.rs#L5228) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `"1".repeat` | [5231](../../src/management.rs#L5231) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `project.to_string_lossy` | [5232](../../src/management.rs#L5232) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `std::fs::remove_dir(&project).expect` | [5236](../../src/management.rs#L5236) | receiver-type-required |
| `completed_workspace_operation_never_reopens_its_old_path` | `std::fs::remove_dir` | [5236](../../src/management.rs#L5236) | external-constructor-callback-or-unresolved |
| `completed_workspace_operation_never_reopens_its_old_path` | `ManagementStore::open_at(root.path().join("kernel"), "2026-09-01T00:00:01.000Z")             .expect` | [5238](../../src/management.rs#L5238) | receiver-type-required |
| `create_in` | `store             .create_session(super::SessionCreateOperation {                 rpc_id,                 request_sha256: &format!("{:0>64}", rpc_id.len()),                 requested_session_id: None,                 workspace_id: None,                 cwd: Some(cwd),                 identity_profile: None,                 preset: None,                 preset_mcp: None,                 preset_models: None,                 user_agent_dir: agents,                 started_at: "2026-10-15T00:00:00.000Z",                 principal: "test",             })             .expect` | [5243](../../src/management.rs#L5243) | receiver-type-required |
| `create_in` | `store             .create_session` | [5243](../../src/management.rs#L5243) | receiver-type-required |
| `create_in` | `Some` | [5249](../../src/management.rs#L5249) | external-constructor-callback-or-unresolved |
| `session_create_registers_an_unknown_cwd_once` | `tempfile::tempdir().expect` | [5263](../../src/management.rs#L5263) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `tempfile::tempdir` | [5263](../../src/management.rs#L5263) | external-constructor-callback-or-unresolved |
| `session_create_registers_an_unknown_cwd_once` | `root.path().join` | [5264](../../src/management.rs#L5264), [5265](../../src/management.rs#L5265), [5266](../../src/management.rs#L5266), [5267](../../src/management.rs#L5267) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `root.path` | [5264](../../src/management.rs#L5264), [5265](../../src/management.rs#L5265), [5266](../../src/management.rs#L5266), [5267](../../src/management.rs#L5267) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `root.path().join("one").join` | [5266](../../src/management.rs#L5266) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `root.path().join("two").join` | [5267](../../src/management.rs#L5267) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `fs::create_dir_all(&alpha).expect` | [5268](../../src/management.rs#L5268) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `fs::create_dir_all` | [5268](../../src/management.rs#L5268), [5269](../../src/management.rs#L5269) | external-constructor-callback-or-unresolved |
| `session_create_registers_an_unknown_cwd_once` | `fs::create_dir_all(&beta).expect` | [5269](../../src/management.rs#L5269) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `ManagementStore::open_at(&storage, "2026-10-15T00:00:00.000Z").expect` | [5270](../../src/management.rs#L5270) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `ManagementStore::open_at` | [5270](../../src/management.rs#L5270) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `session_create_registers_an_unknown_cwd_once` | `alpha.to_string_lossy` | [5271](../../src/management.rs#L5271) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `create_in` | [5273](../../src/management.rs#L5273), [5290](../../src/management.rs#L5290), [5295](../../src/management.rs#L5295) | [endpoint::management::tests::create_in](../../src/management.rs#L5242) |
| `session_create_registers_an_unknown_cwd_once` | `store.workspace_configs().expect` | [5274](../../src/management.rs#L5274) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `store.workspace_configs` | [5274](../../src/management.rs#L5274) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `configs.values().next().expect("registered").clone` | [5276](../../src/management.rs#L5276) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `configs.values().next().expect` | [5276](../../src/management.rs#L5276) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `configs.values().next` | [5276](../../src/management.rs#L5276) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `configs.values` | [5276](../../src/management.rs#L5276) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `alpha.canonicalize().expect` | [5277](../../src/management.rs#L5277) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `alpha.canonicalize` | [5277](../../src/management.rs#L5277) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `beta.to_string_lossy` | [5295](../../src/management.rs#L5295) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `store             .workspace_configs()             .expect("configs")             .values()             .map(&#124;config&#124; config.name.clone())             .collect::<Vec<_>>` | [5296](../../src/management.rs#L5296) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `store             .workspace_configs()             .expect("configs")             .values()             .map` | [5296](../../src/management.rs#L5296) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `store             .workspace_configs()             .expect("configs")             .values` | [5296](../../src/management.rs#L5296) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `store             .workspace_configs()             .expect` | [5296](../../src/management.rs#L5296) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `store             .workspace_configs` | [5296](../../src/management.rs#L5296) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `config.name.clone` | [5300](../../src/management.rs#L5300) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `names.sort` | [5302](../../src/management.rs#L5302) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `crate::NativeEndpoint::open(&storage)             .expect("endpoint")             .list_sessions(&std::collections::HashSet::new())             .expect` | [5305](../../src/management.rs#L5305) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `crate::NativeEndpoint::open(&storage)             .expect("endpoint")             .list_sessions` | [5305](../../src/management.rs#L5305) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `crate::NativeEndpoint::open(&storage)             .expect` | [5305](../../src/management.rs#L5305) | receiver-type-required |
| `session_create_registers_an_unknown_cwd_once` | `crate::NativeEndpoint::open` | [5305](../../src/management.rs#L5305) | [endpoint::service::NativeEndpoint::open](../../src/service.rs#L263) |
| `session_create_registers_an_unknown_cwd_once` | `std::collections::HashSet::new` | [5307](../../src/management.rs#L5307) | external-constructor-callback-or-unresolved |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `tempfile::tempdir().expect` | [5321](../../src/management.rs#L5321) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `tempfile::tempdir` | [5321](../../src/management.rs#L5321) | external-constructor-callback-or-unresolved |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `root.path().join` | [5322](../../src/management.rs#L5322), [5323](../../src/management.rs#L5323), [5324](../../src/management.rs#L5324) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `root.path` | [5322](../../src/management.rs#L5322), [5323](../../src/management.rs#L5323), [5324](../../src/management.rs#L5324) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `fs::create_dir_all(&project).expect` | [5325](../../src/management.rs#L5325) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `fs::create_dir_all` | [5325](../../src/management.rs#L5325), [5338](../../src/management.rs#L5338) | external-constructor-callback-or-unresolved |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `ManagementStore::open_at(&storage, "2026-10-15T00:00:00.000Z").expect` | [5326](../../src/management.rs#L5326) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `ManagementStore::open_at` | [5326](../../src/management.rs#L5326) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `store             .create_workspace(                 "skills-workspace",                 &"7".repeat(64),                 &project.to_string_lossy(),                 "2026-10-15T00:00:00.000Z",             )             .expect` | [5327](../../src/management.rs#L5327) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `store             .create_workspace` | [5327](../../src/management.rs#L5327) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `"7".repeat` | [5330](../../src/management.rs#L5330) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `project.to_string_lossy` | [5331](../../src/management.rs#L5331) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `dir.join("skills").join` | [5337](../../src/management.rs#L5337) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `dir.join` | [5337](../../src/management.rs#L5337) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `fs::create_dir_all(&skill).expect` | [5338](../../src/management.rs#L5338) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `fs::write(skill.join("SKILL.md"), manifest).expect` | [5339](../../src/management.rs#L5339) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `fs::write` | [5339](../../src/management.rs#L5339) | external-constructor-callback-or-unresolved |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `skill.join` | [5339](../../src/management.rs#L5339) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `store.create_session` | [5343](../../src/management.rs#L5343) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `"8".repeat` | [5345](../../src/management.rs#L5345) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `Some` | [5347](../../src/management.rs#L5347), [5350](../../src/management.rs#L5350) | external-constructor-callback-or-unresolved |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `write_skill` | [5360](../../src/management.rs#L5360), [5368](../../src/management.rs#L5368) | external-constructor-callback-or-unresolved |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `agents.join("workspaces").join` | [5360](../../src/management.rs#L5360) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `agents.join` | [5360](../../src/management.rs#L5360) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `storage.join("workspaces").join` | [5368](../../src/management.rs#L5368) | receiver-type-required |
| `session_create_reads_workspace_skills_where_the_worker_launch_does` | `storage.join` | [5368](../../src/management.rs#L5368) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `tempfile::tempdir().expect` | [5374](../../src/management.rs#L5374) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `tempfile::tempdir` | [5374](../../src/management.rs#L5374) | external-constructor-callback-or-unresolved |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `root.path().join` | [5376](../../src/management.rs#L5376), [5378](../../src/management.rs#L5378) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `root.path` | [5376](../../src/management.rs#L5376), [5378](../../src/management.rs#L5378) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `home.join` | [5377](../../src/management.rs#L5377), [5379](../../src/management.rs#L5379) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `ManagementStore::open_at(&storage, "2026-10-15T00:00:00.000Z").expect` | [5380](../../src/management.rs#L5380) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `ManagementStore::open_at` | [5380](../../src/management.rs#L5380) | [endpoint::management::ManagementStore::open_at](../../src/management.rs#L466) |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `fs::write(&file, b"notes").expect` | [5381](../../src/management.rs#L5381) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `fs::write` | [5381](../../src/management.rs#L5381) | external-constructor-callback-or-unresolved |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `"/".to_owned` | [5383](../../src/management.rs#L5383) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `storage.to_string_lossy().into_owned` | [5384](../../src/management.rs#L5384) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `storage.to_string_lossy` | [5384](../../src/management.rs#L5384) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `storage.join("threads").to_string_lossy().into_owned` | [5385](../../src/management.rs#L5385) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `storage.join("threads").to_string_lossy` | [5385](../../src/management.rs#L5385) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `storage.join` | [5385](../../src/management.rs#L5385), [5431](../../src/management.rs#L5431) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `file.to_string_lossy().into_owned` | [5386](../../src/management.rs#L5386) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `file.to_string_lossy` | [5386](../../src/management.rs#L5386) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `rejected.iter().enumerate` | [5388](../../src/management.rs#L5388) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `rejected.iter` | [5388](../../src/management.rs#L5388) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `store.create_session` | [5401](../../src/management.rs#L5401) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `"2".repeat` | [5403](../../src/management.rs#L5403) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `Some` | [5406](../../src/management.rs#L5406) | external-constructor-callback-or-unresolved |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `created.err` | [5415](../../src/management.rs#L5415) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `home.to_string_lossy` | [5423](../../src/management.rs#L5423) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `create_in` | [5424](../../src/management.rs#L5424) | [endpoint::management::tests::create_in](../../src/management.rs#L5242) |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `store             .workspace_configs()             .expect("configs")             .into_keys()             .next()             .expect` | [5425](../../src/management.rs#L5425) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `store             .workspace_configs()             .expect("configs")             .into_keys()             .next` | [5425](../../src/management.rs#L5425) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `store             .workspace_configs()             .expect("configs")             .into_keys` | [5425](../../src/management.rs#L5425) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `store             .workspace_configs()             .expect` | [5425](../../src/management.rs#L5425) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `store             .workspace_configs` | [5425](../../src/management.rs#L5425) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `fs::create_dir_all(&inside).expect` | [5432](../../src/management.rs#L5432) | receiver-type-required |
| `folders_never_name_the_filesystem_root_or_kernel_storage` | `fs::create_dir_all` | [5432](../../src/management.rs#L5432) | external-constructor-callback-or-unresolved |
| `fixture` | `root.join` | [5473](../../src/management.rs#L5473) | receiver-type-required |
| `fixture` | `fs::create_dir(&project).expect` | [5474](../../src/management.rs#L5474) | receiver-type-required |
| `fixture` | `fs::create_dir` | [5474](../../src/management.rs#L5474) | external-constructor-callback-or-unresolved |
| `fixture` | `project.to_string_lossy().into_owned` | [5475](../../src/management.rs#L5475) | receiver-type-required |
| `fixture` | `project.to_string_lossy` | [5475](../../src/management.rs#L5475) | receiver-type-required |
| `fixture` | `ConfigRepository::open(root).expect` | [5476](../../src/management.rs#L5476) | receiver-type-required |
| `fixture` | `ConfigRepository::open` | [5476](../../src/management.rs#L5476) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `fixture` | `repository             .publish_workspace(                 0,                 &WorkspaceConfig {                     format: 1,                     revision: 1,                     id: WORKSPACE.to_owned(),                     name: "K1".to_owned(),                     cwd: Vec::new(),                     folders: vec![WorkspaceFolder {                         id: "folder-0001".to_owned(),                         path: folder.clone(),                     }],                     policy: Some(default_workspace_policy(&folder)),                 },             )             .expect` | [5477](../../src/management.rs#L5477) | receiver-type-required |
| `fixture` | `repository             .publish_workspace` | [5477](../../src/management.rs#L5477) | receiver-type-required |
| `fixture` | `WORKSPACE.to_owned` | [5483](../../src/management.rs#L5483) | receiver-type-required |
| `fixture` | `"K1".to_owned` | [5484](../../src/management.rs#L5484) | receiver-type-required |
| `fixture` | `Vec::new` | [5485](../../src/management.rs#L5485) | external-constructor-callback-or-unresolved |
| `fixture` | `Some` | [5490](../../src/management.rs#L5490) | external-constructor-callback-or-unresolved |
| `fixture` | `default_workspace_policy` | [5490](../../src/management.rs#L5490) | external-constructor-callback-or-unresolved |
| `policy_with` | `default_workspace_policy` | [5498](../../src/management.rs#L5498) | external-constructor-callback-or-unresolved |
| `policy_with` | `project.to_string_lossy` | [5498](../../src/management.rs#L5498) | receiver-type-required |
| `policy_with` | `policy.allowed_tools.push` | [5499](../../src/management.rs#L5499) | receiver-type-required |
| `policy_with` | `extra.to_owned` | [5499](../../src/management.rs#L5499) | receiver-type-required |
| `set_policy` | `repository.workspace(WORKSPACE).expect` | [5504](../../src/management.rs#L5504) | receiver-type-required |
| `set_policy` | `repository.workspace` | [5504](../../src/management.rs#L5504) | receiver-type-required |
| `set_policy` | `repository             .publish_workspace_policy(WORKSPACE, revision, policy_with(project, extra))             .expect` | [5505](../../src/management.rs#L5505) | receiver-type-required |
| `set_policy` | `repository             .publish_workspace_policy` | [5505](../../src/management.rs#L5505) | receiver-type-required |
| `set_policy` | `policy_with` | [5506](../../src/management.rs#L5506) | [endpoint::management::k1_management_race::policy_with](../../src/management.rs#L5497) |
| `quarantine_root` | `store::endpoint_management_root(root)             .expect("management root")             .join` | [5511](../../src/management.rs#L5511) | receiver-type-required |
| `quarantine_root` | `store::endpoint_management_root(root)             .expect` | [5511](../../src/management.rs#L5511) | receiver-type-required |
| `quarantine_root` | `store::endpoint_management_root` | [5511](../../src/management.rs#L5511) | [store::management_root::endpoint_management_root](../../../store/src/management_root.rs#L47) |
| `stale_metadata` | `store.metadata_path` | [5517](../../src/management.rs#L5517) | receiver-type-required |
| `stale_metadata` | `read_canonical(&path).expect` | [5518](../../src/management.rs#L5518) | receiver-type-required |
| `stale_metadata` | `read_canonical` | [5518](../../src/management.rs#L5518) | external-constructor-callback-or-unresolved |
| `stale_metadata` | `"0".repeat` | [5519](../../src/management.rs#L5519) | receiver-type-required |
| `stale_metadata` | `publish_canonical(&path, &metadata).expect` | [5520](../../src/management.rs#L5520) | receiver-type-required |
| `stale_metadata` | `publish_canonical` | [5520](../../src/management.rs#L5520) | external-constructor-callback-or-unresolved |
| `metadata_matches` | `store.workspace_configs().expect("configs")[id].clone` | [5524](../../src/management.rs#L5524) | receiver-type-required |
| `metadata_matches` | `store.workspace_configs().expect` | [5524](../../src/management.rs#L5524) | receiver-type-required |
| `metadata_matches` | `store.workspace_configs` | [5524](../../src/management.rs#L5524) | receiver-type-required |
| `metadata_matches` | `verify_metadata(&config, &store.read_metadata(id).expect("metadata")).is_ok` | [5525](../../src/management.rs#L5525) | receiver-type-required |
| `metadata_matches` | `verify_metadata` | [5525](../../src/management.rs#L5525) | external-constructor-callback-or-unresolved |
| `metadata_matches` | `store.read_metadata(id).expect` | [5525](../../src/management.rs#L5525) | receiver-type-required |
| `metadata_matches` | `store.read_metadata` | [5525](../../src/management.rs#L5525) | receiver-type-required |
| `stuck_carriers_record` | `store.workspace_configs().expect("configs")[WORKSPACE].clone` | [5536](../../src/management.rs#L5536) | receiver-type-required |
| `stuck_carriers_record` | `store.workspace_configs().expect` | [5536](../../src/management.rs#L5536) | receiver-type-required |
| `stuck_carriers_record` | `store.workspace_configs` | [5536](../../src/management.rs#L5536) | receiver-type-required |
| `stuck_carriers_record` | `"K1 renamed".to_owned` | [5538](../../src/management.rs#L5538) | receiver-type-required |
| `stuck_carriers_record` | `hex_digest` | [5539](../../src/management.rs#L5539) | external-constructor-callback-or-unresolved |
| `stuck_carriers_record` | `canonical_line(&config).expect` | [5539](../../src/management.rs#L5539) | receiver-type-required |
| `stuck_carriers_record` | `canonical_line` | [5539](../../src/management.rs#L5539) | external-constructor-callback-or-unresolved |
| `stuck_carriers_record` | `store             .begin_workspace_operation(WorkspaceOperationWrite {                 rpc_id,                 request_sha256: &digest,                 operation: "workspace.rename",                 action: "rename",                 config: &config,                 started_at: TICK,                 created: None,                 relocation: None,                 folder: None,             })             .expect` | [5540](../../src/management.rs#L5540) | receiver-type-required |
| `stuck_carriers_record` | `store             .begin_workspace_operation` | [5540](../../src/management.rs#L5540) | receiver-type-required |
| `stuck_carriers_record` | `publish_canonical(&path, &record).expect` | [5554](../../src/management.rs#L5554) | receiver-type-required |
| `stuck_carriers_record` | `publish_canonical` | [5554](../../src/management.rs#L5554) | external-constructor-callback-or-unresolved |
| `stuck_carriers_record` | `set_policy` | [5555](../../src/management.rs#L5555), [5556](../../src/management.rs#L5556) | [endpoint::management::k1_management_race::set_policy](../../src/management.rs#L5503) |
| `tick_heals_stale_metadata_without_staging_an_operation` | `tempfile::tempdir().expect` | [5565](../../src/management.rs#L5565) | receiver-type-required |
| `tick_heals_stale_metadata_without_staging_an_operation` | `tempfile::tempdir` | [5565](../../src/management.rs#L5565) | external-constructor-callback-or-unresolved |
| `tick_heals_stale_metadata_without_staging_an_operation` | `fixture` | [5566](../../src/management.rs#L5566) | [endpoint::management::k1_management_race::fixture](../../src/management.rs#L5472) |
| `tick_heals_stale_metadata_without_staging_an_operation` | `root.path` | [5566](../../src/management.rs#L5566), [5567](../../src/management.rs#L5567), [5574](../../src/management.rs#L5574), [5588](../../src/management.rs#L5588) | receiver-type-required |
| `tick_heals_stale_metadata_without_staging_an_operation` | `ManagementStore::open_at(root.path(), BOOT).expect` | [5567](../../src/management.rs#L5567) | receiver-type-required |
| `tick_heals_stale_metadata_without_staging_an_operation` | `ManagementStore::open_at` | [5567](../../src/management.rs#L5567) | external-constructor-callback-or-unresolved |
| `tick_heals_stale_metadata_without_staging_an_operation` | `store.operation_paths().expect` | [5568](../../src/management.rs#L5568) | receiver-type-required |
| `tick_heals_stale_metadata_without_staging_an_operation` | `store.operation_paths` | [5568](../../src/management.rs#L5568) | receiver-type-required |
| `tick_heals_stale_metadata_without_staging_an_operation` | `set_policy` | [5569](../../src/management.rs#L5569), [5587](../../src/management.rs#L5587) | [endpoint::management::k1_management_race::set_policy](../../src/management.rs#L5503) |
| `tick_heals_stale_metadata_without_staging_an_operation` | `stale_metadata` | [5570](../../src/management.rs#L5570) | [endpoint::management::k1_management_race::stale_metadata](../../src/management.rs#L5516) |
| `tick_heals_stale_metadata_without_staging_an_operation` | `CompleteOperations::default` | [5573](../../src/management.rs#L5573) | external-constructor-callback-or-unresolved |
| `tick_heals_stale_metadata_without_staging_an_operation` | `ManagementStore::open_recovering_again(root.path(), TICK, None, &mut validated)             .expect` | [5574](../../src/management.rs#L5574) | receiver-type-required |
| `tick_heals_stale_metadata_without_staging_an_operation` | `ManagementStore::open_recovering_again` | [5574](../../src/management.rs#L5574) | external-constructor-callback-or-unresolved |
| `tick_heals_stale_metadata_without_staging_an_operation` | `ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect` | [5588](../../src/management.rs#L5588) | receiver-type-required |
| `tick_heals_stale_metadata_without_staging_an_operation` | `ManagementStore::open_at_with_queue_driver` | [5588](../../src/management.rs#L5588) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `tempfile::tempdir().expect` | [5595](../../src/management.rs#L5595) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `tempfile::tempdir` | [5595](../../src/management.rs#L5595) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `fixture` | [5596](../../src/management.rs#L5596) | [endpoint::management::k1_management_race::fixture](../../src/management.rs#L5472) |
| `tick_never_fails_while_policy_set_publishes` | `root.path` | [5596](../../src/management.rs#L5596), [5597](../../src/management.rs#L5597), [5601](../../src/management.rs#L5601), [5620](../../src/management.rs#L5620), [5631](../../src/management.rs#L5631) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `ManagementStore::open_at(root.path(), BOOT).expect` | [5597](../../src/management.rs#L5597) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `ManagementStore::open_at` | [5597](../../src/management.rs#L5597) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `Arc::new` | [5598](../../src/management.rs#L5598) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `AtomicBool::new` | [5598](../../src/management.rs#L5598) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `Arc::clone` | [5600](../../src/management.rs#L5600) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `root.path().to_path_buf` | [5601](../../src/management.rs#L5601) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `std::thread::spawn` | [5602](../../src/management.rs#L5602) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `ConfigRepository::open(&root).expect` | [5603](../../src/management.rs#L5603) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `ConfigRepository::open` | [5603](../../src/management.rs#L5603) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `tick_never_fails_while_policy_set_publishes` | `stop.load` | [5605](../../src/management.rs#L5605) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `repository.workspace(WORKSPACE).expect` | [5606](../../src/management.rs#L5606) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `repository.workspace` | [5606](../../src/management.rs#L5606) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `repository.publish_workspace_policy` | [5607](../../src/management.rs#L5607) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `policy_with` | [5610](../../src/management.rs#L5610) | [endpoint::management::k1_management_race::policy_with](../../src/management.rs#L5497) |
| `tick_never_fails_while_policy_set_publishes` | `CompleteOperations::default` | [5617](../../src/management.rs#L5617) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `ManagementStore::open_recovering_again(root.path(), TICK, None, &mut validated)                 .is_err` | [5620](../../src/management.rs#L5620) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `ManagementStore::open_recovering_again` | [5620](../../src/management.rs#L5620) | external-constructor-callback-or-unresolved |
| `tick_never_fails_while_policy_set_publishes` | `stop.store` | [5626](../../src/management.rs#L5626) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `writer.join().expect` | [5627](../../src/management.rs#L5627) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `writer.join` | [5627](../../src/management.rs#L5627) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect` | [5631](../../src/management.rs#L5631) | receiver-type-required |
| `tick_never_fails_while_policy_set_publishes` | `ManagementStore::open_at_with_queue_driver` | [5631](../../src/management.rs#L5631) | external-constructor-callback-or-unresolved |
| `parked_synthetic_rename` | `fixture` | [5640](../../src/management.rs#L5640) | [endpoint::management::k1_management_race::fixture](../../src/management.rs#L5472) |
| `parked_synthetic_rename` | `ManagementStore::open_at(root, BOOT).expect` | [5641](../../src/management.rs#L5641) | receiver-type-required |
| `parked_synthetic_rename` | `ManagementStore::open_at` | [5641](../../src/management.rs#L5641) | external-constructor-callback-or-unresolved |
| `parked_synthetic_rename` | `default_workspace_policy` | [5643](../../src/management.rs#L5643) | external-constructor-callback-or-unresolved |
| `parked_synthetic_rename` | `project.to_string_lossy` | [5643](../../src/management.rs#L5643) | receiver-type-required |
| `parked_synthetic_rename` | `(0..232)             .map(&#124;index&#124; format!("mcp__fixture_server__tool_{index:03}"))             .collect` | [5644](../../src/management.rs#L5644) | receiver-type-required |
| `parked_synthetic_rename` | `(0..232)             .map` | [5644](../../src/management.rs#L5644) | receiver-type-required |
| `parked_synthetic_rename` | `repository.workspace(WORKSPACE).expect` | [5647](../../src/management.rs#L5647) | receiver-type-required |
| `parked_synthetic_rename` | `repository.workspace` | [5647](../../src/management.rs#L5647) | receiver-type-required |
| `parked_synthetic_rename` | `repository             .publish_workspace_policy(WORKSPACE, revision, policy)             .expect` | [5648](../../src/management.rs#L5648) | receiver-type-required |
| `parked_synthetic_rename` | `repository             .publish_workspace_policy` | [5648](../../src/management.rs#L5648) | receiver-type-required |
| `parked_synthetic_rename` | `stale_metadata` | [5651](../../src/management.rs#L5651) | [endpoint::management::k1_management_race::stale_metadata](../../src/management.rs#L5516) |
| `parked_synthetic_rename` | `store.workspace_configs().expect("configs")[WORKSPACE].clone` | [5652](../../src/management.rs#L5652) | receiver-type-required |
| `parked_synthetic_rename` | `store.workspace_configs().expect` | [5652](../../src/management.rs#L5652) | receiver-type-required |
| `parked_synthetic_rename` | `store.workspace_configs` | [5652](../../src/management.rs#L5652) | receiver-type-required |
| `parked_synthetic_rename` | `hex_digest` | [5654](../../src/management.rs#L5654) | external-constructor-callback-or-unresolved |
| `parked_synthetic_rename` | `canonical_line(&config).expect` | [5654](../../src/management.rs#L5654) | receiver-type-required |
| `parked_synthetic_rename` | `canonical_line` | [5654](../../src/management.rs#L5654) | external-constructor-callback-or-unresolved |
| `parked_synthetic_rename` | `store             .begin_workspace_operation(WorkspaceOperationWrite {                 rpc_id: &rpc_id,                 request_sha256: &digest,                 operation: "workspace.rename",                 action: "rename",                 config: &config,                 started_at: TICK,                 created: None,                 relocation: None,                 folder: None,             })             .expect` | [5657](../../src/management.rs#L5657) | receiver-type-required |
| `parked_synthetic_rename` | `store             .begin_workspace_operation` | [5657](../../src/management.rs#L5657) | receiver-type-required |
| `parked_synthetic_rename` | `publish_canonical(&path, &record).expect` | [5671](../../src/management.rs#L5671) | receiver-type-required |
| `parked_synthetic_rename` | `publish_canonical` | [5671](../../src/management.rs#L5671) | external-constructor-callback-or-unresolved |
| `parked_synthetic_rename` | `set_policy` | [5677](../../src/management.rs#L5677) | [endpoint::management::k1_management_race::set_policy](../../src/management.rs#L5503) |
| `parked_synthetic_rename` | `fs::read(store.operation_payload_path(&rpc_id).join("workspace.json")).expect` | [5680](../../src/management.rs#L5680) | receiver-type-required |
| `parked_synthetic_rename` | `fs::read` | [5680](../../src/management.rs#L5680), [5681](../../src/management.rs#L5681) | external-constructor-callback-or-unresolved |
| `parked_synthetic_rename` | `store.operation_payload_path(&rpc_id).join` | [5680](../../src/management.rs#L5680) | receiver-type-required |
| `parked_synthetic_rename` | `store.operation_payload_path` | [5680](../../src/management.rs#L5680) | receiver-type-required |
| `parked_synthetic_rename` | `fs::read(&path).expect` | [5681](../../src/management.rs#L5681) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `tempfile::tempdir().expect` | [5689](../../src/management.rs#L5689) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `tempfile::tempdir` | [5689](../../src/management.rs#L5689) | external-constructor-callback-or-unresolved |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `parked_synthetic_rename` | [5690](../../src/management.rs#L5690) | [endpoint::management::k1_management_race::parked_synthetic_rename](../../src/management.rs#L5639) |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `root.path` | [5690](../../src/management.rs#L5690), [5691](../../src/management.rs#L5691), [5700](../../src/management.rs#L5700), [5704](../../src/management.rs#L5704), [5731](../../src/management.rs#L5731) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `ConfigRepository::open(root.path()).expect` | [5691](../../src/management.rs#L5691) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `ConfigRepository::open` | [5691](../../src/management.rs#L5691) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `path             .file_stem()             .expect("stem")             .to_string_lossy()             .into_owned` | [5693](../../src/management.rs#L5693) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `path             .file_stem()             .expect("stem")             .to_string_lossy` | [5693](../../src/management.rs#L5693) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `path             .file_stem()             .expect` | [5693](../../src/management.rs#L5693) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `path             .file_stem` | [5693](../../src/management.rs#L5693) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `ManagementStore::open_at_with_queue_driver(root.path(), LATER, None)             .expect` | [5700](../../src/management.rs#L5700) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `ManagementStore::open_at_with_queue_driver` | [5700](../../src/management.rs#L5700), [5731](../../src/management.rs#L5731) | external-constructor-callback-or-unresolved |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `quarantine_root(root.path()).join` | [5704](../../src/management.rs#L5704) | receiver-type-required |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `quarantine_root` | [5704](../../src/management.rs#L5704) | [endpoint::management::k1_management_race::quarantine_root](../../src/management.rs#L5510) |
| `boot_quarantines_the_parked_carriers_record_and_succeeds` | `ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect` | [5731](../../src/management.rs#L5731) | receiver-type-required |
| `record_of_another_version_is_left_in_place_not_quarantined` | `tempfile::tempdir().expect` | [5744](../../src/management.rs#L5744) | receiver-type-required |
| `record_of_another_version_is_left_in_place_not_quarantined` | `tempfile::tempdir` | [5744](../../src/management.rs#L5744) | external-constructor-callback-or-unresolved |
| `record_of_another_version_is_left_in_place_not_quarantined` | `parked_synthetic_rename` | [5745](../../src/management.rs#L5745) | [endpoint::management::k1_management_race::parked_synthetic_rename](../../src/management.rs#L5639) |
| `record_of_another_version_is_left_in_place_not_quarantined` | `root.path` | [5745](../../src/management.rs#L5745), [5751](../../src/management.rs#L5751) | receiver-type-required |
| `record_of_another_version_is_left_in_place_not_quarantined` | `read_canonical(&path).expect` | [5746](../../src/management.rs#L5746) | receiver-type-required |
| `record_of_another_version_is_left_in_place_not_quarantined` | `read_canonical` | [5746](../../src/management.rs#L5746) | external-constructor-callback-or-unresolved |
| `record_of_another_version_is_left_in_place_not_quarantined` | `publish_canonical(&path, &record).expect` | [5748](../../src/management.rs#L5748) | receiver-type-required |
| `record_of_another_version_is_left_in_place_not_quarantined` | `publish_canonical` | [5748](../../src/management.rs#L5748) | external-constructor-callback-or-unresolved |
| `record_of_another_version_is_left_in_place_not_quarantined` | `fs::read(&path).expect` | [5749](../../src/management.rs#L5749) | receiver-type-required |
| `record_of_another_version_is_left_in_place_not_quarantined` | `fs::read` | [5749](../../src/management.rs#L5749) | external-constructor-callback-or-unresolved |
| `record_of_another_version_is_left_in_place_not_quarantined` | `ManagementStore::open_at_with_queue_driver(root.path(), LATER, None)             .expect` | [5751](../../src/management.rs#L5751) | receiver-type-required |
| `record_of_another_version_is_left_in_place_not_quarantined` | `ManagementStore::open_at_with_queue_driver` | [5751](../../src/management.rs#L5751) | external-constructor-callback-or-unresolved |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `tempfile::tempdir().expect` | [5766](../../src/management.rs#L5766) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `tempfile::tempdir` | [5766](../../src/management.rs#L5766) | external-constructor-callback-or-unresolved |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fixture` | [5767](../../src/management.rs#L5767) | [endpoint::management::k1_management_race::fixture](../../src/management.rs#L5472) |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `root.path` | [5767](../../src/management.rs#L5767), [5768](../../src/management.rs#L5768), [5783](../../src/management.rs#L5783), [5788](../../src/management.rs#L5788), [5790](../../src/management.rs#L5790), [5807](../../src/management.rs#L5807), [5816](../../src/management.rs#L5816), [5817](../../src/management.rs#L5817) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `ManagementStore::open_at(root.path(), BOOT).expect` | [5768](../../src/management.rs#L5768) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `ManagementStore::open_at` | [5768](../../src/management.rs#L5768) | external-constructor-callback-or-unresolved |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `stuck_carriers_record` | [5769](../../src/management.rs#L5769), [5814](../../src/management.rs#L5814) | [endpoint::management::k1_management_race::stuck_carriers_record](../../src/management.rs#L5530) |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `path             .file_stem()             .expect("stem")             .to_string_lossy()             .into_owned` | [5770](../../src/management.rs#L5770) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `path             .file_stem()             .expect("stem")             .to_string_lossy` | [5770](../../src/management.rs#L5770) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `path             .file_stem()             .expect` | [5770](../../src/management.rs#L5770) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `path             .file_stem` | [5770](../../src/management.rs#L5770) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fs::read(&path).expect` | [5775](../../src/management.rs#L5775) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fs::read` | [5775](../../src/management.rs#L5775), [5776](../../src/management.rs#L5776) | external-constructor-callback-or-unresolved |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fs::read(             store                 .operation_payload_path("rpc-stuck")                 .join("workspace.json"),         )         .expect` | [5776](../../src/management.rs#L5776) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `store                 .operation_payload_path("rpc-stuck")                 .join` | [5777](../../src/management.rs#L5777) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `store                 .operation_payload_path` | [5777](../../src/management.rs#L5777) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `quarantine_root(root.path()).join` | [5783](../../src/management.rs#L5783), [5790](../../src/management.rs#L5790), [5817](../../src/management.rs#L5817) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `quarantine_root` | [5783](../../src/management.rs#L5783), [5790](../../src/management.rs#L5790), [5817](../../src/management.rs#L5817) | [endpoint::management::k1_management_race::quarantine_root](../../src/management.rs#L5510) |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fs::create_dir_all(partial.join("payload-root/stale")).expect` | [5784](../../src/management.rs#L5784) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fs::create_dir_all` | [5784](../../src/management.rs#L5784) | external-constructor-callback-or-unresolved |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `partial.join` | [5784](../../src/management.rs#L5784), [5785](../../src/management.rs#L5785) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fs::write(partial.join("payload-root/stale/leftover"), b"x").expect` | [5785](../../src/management.rs#L5785) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `fs::write` | [5785](../../src/management.rs#L5785) | external-constructor-callback-or-unresolved |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `repository.workspace(WORKSPACE).expect` | [5786](../../src/management.rs#L5786) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `repository.workspace` | [5786](../../src/management.rs#L5786) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect` | [5788](../../src/management.rs#L5788), [5807](../../src/management.rs#L5807), [5816](../../src/management.rs#L5816) | receiver-type-required |
| `quarantine_keeps_the_payload_and_never_redrives_the_operation` | `ManagementStore::open_at_with_queue_driver` | [5788](../../src/management.rs#L5788), [5807](../../src/management.rs#L5807), [5816](../../src/management.rs#L5816) | external-constructor-callback-or-unresolved |
| `complete_workspace_record_without_response_is_quarantined` | `tempfile::tempdir().expect` | [5829](../../src/management.rs#L5829) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `tempfile::tempdir` | [5829](../../src/management.rs#L5829) | external-constructor-callback-or-unresolved |
| `complete_workspace_record_without_response_is_quarantined` | `fixture` | [5830](../../src/management.rs#L5830) | [endpoint::management::k1_management_race::fixture](../../src/management.rs#L5472) |
| `complete_workspace_record_without_response_is_quarantined` | `root.path` | [5830](../../src/management.rs#L5830), [5831](../../src/management.rs#L5831), [5849](../../src/management.rs#L5849) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `ManagementStore::open_at(root.path(), BOOT).expect` | [5831](../../src/management.rs#L5831) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `ManagementStore::open_at` | [5831](../../src/management.rs#L5831) | external-constructor-callback-or-unresolved |
| `complete_workspace_record_without_response_is_quarantined` | `store.workspace_configs().expect("configs")[WORKSPACE].clone` | [5832](../../src/management.rs#L5832) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `store.workspace_configs().expect` | [5832](../../src/management.rs#L5832) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `store.workspace_configs` | [5832](../../src/management.rs#L5832) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `store             .begin_workspace_operation(WorkspaceOperationWrite {                 rpc_id: "rpc-complete",                 request_sha256: &"1".repeat(64),                 operation: "workspace.rename",                 action: "rename",                 config: &config,                 started_at: TICK,                 created: None,                 relocation: None,                 folder: None,             })             .expect` | [5833](../../src/management.rs#L5833) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `store             .begin_workspace_operation` | [5833](../../src/management.rs#L5833) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `"1".repeat` | [5836](../../src/management.rs#L5836) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `publish_canonical(&path, &record).expect` | [5847](../../src/management.rs#L5847) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `publish_canonical` | [5847](../../src/management.rs#L5847) | external-constructor-callback-or-unresolved |
| `complete_workspace_record_without_response_is_quarantined` | `ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect` | [5849](../../src/management.rs#L5849) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `ManagementStore::open_at_with_queue_driver` | [5849](../../src/management.rs#L5849) | external-constructor-callback-or-unresolved |
| `complete_workspace_record_without_response_is_quarantined` | `path.file_stem().expect` | [5852](../../src/management.rs#L5852) | receiver-type-required |
| `complete_workspace_record_without_response_is_quarantined` | `path.file_stem` | [5852](../../src/management.rs#L5852) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `tempfile::tempdir().expect` | [5865](../../src/management.rs#L5865) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `tempfile::tempdir` | [5865](../../src/management.rs#L5865) | external-constructor-callback-or-unresolved |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `fixture` | [5866](../../src/management.rs#L5866) | [endpoint::management::k1_management_race::fixture](../../src/management.rs#L5472) |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `root.path` | [5866](../../src/management.rs#L5866), [5867](../../src/management.rs#L5867) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `ManagementStore::open_at(root.path(), BOOT).expect` | [5867](../../src/management.rs#L5867) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `ManagementStore::open_at` | [5867](../../src/management.rs#L5867) | external-constructor-callback-or-unresolved |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store.workspace_configs().expect("configs")[WORKSPACE].clone` | [5868](../../src/management.rs#L5868) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store.workspace_configs().expect` | [5868](../../src/management.rs#L5868) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store.workspace_configs` | [5868](../../src/management.rs#L5868) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `"K1 recovered".to_owned` | [5870](../../src/management.rs#L5870) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `hex_digest` | [5871](../../src/management.rs#L5871) | external-constructor-callback-or-unresolved |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `canonical_line(&config).expect` | [5871](../../src/management.rs#L5871) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `canonical_line` | [5871](../../src/management.rs#L5871) | external-constructor-callback-or-unresolved |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store             .begin_workspace_operation(WorkspaceOperationWrite {                 rpc_id: "rpc-pending",                 request_sha256: &digest,                 operation: "workspace.rename",                 action: "rename",                 config: &config,                 started_at: TICK,                 created: None,                 relocation: None,                 folder: None,             })             .expect` | [5872](../../src/management.rs#L5872) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store             .begin_workspace_operation` | [5872](../../src/management.rs#L5872) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `(0..)             .map(&#124;index&#124; format!("rpc-bogus-{index}"))             .find(&#124;rpc_id&#124; store.operation_path(rpc_id) < pending)             .expect` | [5886](../../src/management.rs#L5886) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `(0..)             .map(&#124;index&#124; format!("rpc-bogus-{index}"))             .find` | [5886](../../src/management.rs#L5886) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `(0..)             .map` | [5886](../../src/management.rs#L5886) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store.operation_path` | [5888](../../src/management.rs#L5888), [5890](../../src/management.rs#L5890) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `fs::create_dir_all(bogus_path.parent().expect("prefix")).expect` | [5891](../../src/management.rs#L5891) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `fs::create_dir_all` | [5891](../../src/management.rs#L5891) | external-constructor-callback-or-unresolved |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `bogus_path.parent().expect` | [5891](../../src/management.rs#L5891) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `bogus_path.parent` | [5891](../../src/management.rs#L5891) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `publish_canonical(             &bogus_path,             &OperationRecord {                 v: VERSION,                 rpc_id: bogus,                 operation: "bogus".to_owned(),                 request_sha256: "0".repeat(64),                 phase: OperationPhase::Prepared,                 started_at: TICK.to_owned(),                 intent: to_ijson(&json!({"kind":"bogus"})).expect("intent"),                 response: None,             },         )         .expect` | [5892](../../src/management.rs#L5892) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `publish_canonical` | [5892](../../src/management.rs#L5892) | external-constructor-callback-or-unresolved |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `"bogus".to_owned` | [5897](../../src/management.rs#L5897) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `"0".repeat` | [5898](../../src/management.rs#L5898) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `TICK.to_owned` | [5900](../../src/management.rs#L5900) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `to_ijson(&json!({"kind":"bogus"})).expect` | [5901](../../src/management.rs#L5901) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `to_ijson` | [5901](../../src/management.rs#L5901) | external-constructor-callback-or-unresolved |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store             .recover()             .expect_err` | [5907](../../src/management.rs#L5907) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `store             .recover` | [5907](../../src/management.rs#L5907) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `read_canonical(&pending).expect` | [5917](../../src/management.rs#L5917) | receiver-type-required |
| `one_failing_record_does_not_stop_the_others_from_recovering` | `read_canonical` | [5917](../../src/management.rs#L5917) | external-constructor-callback-or-unresolved |
