# endpoint — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [endpoint::all_session::AllSessionMux](../../src/all_session.rs#L19) | `pub` | not a function |
| [endpoint::all_session::AllSessionMux::open](../../src/all_session.rs#L27) | `pub` | [conformance::tests::slice9_gates::slice9_gate_70_endpoint_archive_client_rematerialization](../../../conformance/tests/slice9_gates.rs#L277); [conformance::tests::slice9_gates::slice9_gate_67_endpoint_subscribe_atomicity](../../../conformance/tests/slice9_gates.rs#L57); [endpoint::tests::slice9_lower_seam::atomic_unresolved_replay_is_ordered_and_private_to_the_new_mux_generation](../../tests/slice9_lower_seam.rs#L418); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::open_mux_journal](../../../supervisor/src/endpoint_carrier.rs#L2161) |
| [endpoint::all_session::AllSessionMux::open_with_replay](../../src/all_session.rs#L39) | `pub` | [endpoint::tests::slice9_lower_seam::atomic_unresolved_replay_is_ordered_and_private_to_the_new_mux_generation](../../tests/slice9_lower_seam.rs#L418); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::open_legacy_mux](../../../supervisor/src/endpoint_carrier.rs#L1579) |
| [endpoint::all_session::AllSessionMux::attach](../../src/all_session.rs#L51) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::attach_with_replay](../../src/all_session.rs#L85) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::detach_for_archive](../../src/all_session.rs#L120) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::poll](../../src/all_session.rs#L134) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::baselines](../../src/all_session.rs#L143) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::contains](../../src/all_session.rs#L151) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::len](../../src/all_session.rs#L156) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::is_empty](../../src/all_session.rs#L161) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMux::into_stream](../../src/all_session.rs#L170) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMuxHandle](../../src/all_session.rs#L186) | `pub` | not a function |
| [endpoint::all_session::AllSessionMuxHandle::contains](../../src/all_session.rs#L189) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMuxHandle::attach](../../src/all_session.rs#L197) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMuxHandle::attach_with_replay](../../src/all_session.rs#L210) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMuxHandle::detach_for_archive](../../src/all_session.rs#L224) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMuxHandle::baselines](../../src/all_session.rs#L231) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMuxHandle::close](../../src/all_session.rs#L239) | `pub` | no resolved direct caller |
| [endpoint::all_session::AllSessionMuxError](../../src/all_session.rs#L348) | `pub` | not a function |
| [endpoint::approval_policy::PERMISSION_MODE_READ_ONLY](../../src/approval_policy.rs#L12) | `pub` | not a function |
| [endpoint::approval_policy::PERMISSION_MODE_WORKSPACE_WRITE](../../src/approval_policy.rs#L13) | `pub` | not a function |
| [endpoint::approval_policy::PERMISSION_MODE_DANGER_FULL_ACCESS](../../src/approval_policy.rs#L14) | `pub` | not a function |
| [endpoint::approval_policy::PERMISSION_MODE_IDS](../../src/approval_policy.rs#L18) | `pub` | not a function |
| [endpoint::approval_policy::ApprovalOption](../../src/approval_policy.rs#L26) | `pub` | not a function |
| [endpoint::approval_policy::ServerApprovalState](../../src/approval_policy.rs#L34) | `pub` | not a function |
| [endpoint::approval_policy::ApprovalPolicy](../../src/approval_policy.rs#L43) | `pub` | not a function |
| [endpoint::attachment::MAX_ATTACHMENT_BYTES](../../src/attachment.rs#L24) | `pub` | not a function |
| [endpoint::attachment::DEFAULT_FILE_MEDIA_TYPE](../../src/attachment.rs#L25) | `pub` | not a function |
| [endpoint::attachment::UPLOADS_DIR](../../src/attachment.rs#L26) | `pub` | not a function |
| [endpoint::attachment::MAX_IMAGE_DIMENSION](../../src/attachment.rs#L27) | `pub` | not a function |
| [endpoint::attachment::MAX_IMAGE_PIXELS](../../src/attachment.rs#L28) | `pub` | not a function |
| [endpoint::attachment::AttachmentPolicy](../../src/attachment.rs#L35) | `pub` | not a function |
| [endpoint::attachment::ImageMediaType](../../src/attachment.rs#L54) | `pub` | not a function |
| [endpoint::attachment::ImageMediaType::as_str](../../src/attachment.rs#L67) | `pub` | no resolved direct caller |
| [endpoint::attachment::PromptPart](../../src/attachment.rs#L98) | `pub` | not a function |
| [endpoint::attachment::AttachmentErrorReason](../../src/attachment.rs#L114) | `pub` | not a function |
| [endpoint::attachment::ImageAttachmentRef](../../src/attachment.rs#L129) | `pub` | not a function |
| [endpoint::attachment::ImageAttachment](../../src/attachment.rs#L141) | `pub` | not a function |
| [endpoint::attachment::FileAttachmentRef](../../src/attachment.rs#L149) | `pub` | not a function |
| [endpoint::attachment::FileAttachment](../../src/attachment.rs#L158) | `pub` | not a function |
| [endpoint::attachment::UploadedFile](../../src/attachment.rs#L166) | `pub` | not a function |
| [endpoint::attachment::UploadReceipt](../../src/attachment.rs#L174) | `pub` | not a function |
| [endpoint::attachment::MaterializedPrompt](../../src/attachment.rs#L193) | `pub` | not a function |
| [endpoint::attachment::PromptMaterializeError](../../src/attachment.rs#L200) | `pub` | not a function |
| [endpoint::attachment::AttachmentReadError](../../src/attachment.rs#L220) | `pub` | not a function |
| [endpoint::attachment::AttachmentAuthority](../../src/attachment.rs#L236) | `pub` | not a function |
| [endpoint::attachment::AttachmentAuthority::open](../../src/attachment.rs#L242) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::open](../../../supervisor/src/client_extensions.rs#L318); [tekes-supervisor::client_extensions::ProductionClientExtensions::attachments](../../../supervisor/src/client_extensions.rs#L337); [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../../supervisor/src/endpoint_host.rs#L684); [tekes-supervisor::file_leases::tests::attachment_transfer_preserves_metadata_and_requires_session_reference](../../../supervisor/src/file_leases.rs#L213); [tekes-supervisor::file_leases::tests::attachment_transfer_serves_uploaded_files_through_the_same_path](../../../supervisor/src/file_leases.rs#L268); [tekes-supervisor::file_leases::SessionFileTransfer::open](../../../supervisor/src/file_leases.rs#L29) |
| [endpoint::attachment::AttachmentAuthority::materialize_prompt_parts](../../src/attachment.rs#L251) | `pub` | no resolved direct caller |
| [endpoint::attachment::AttachmentAuthority::upload_file](../../src/attachment.rs#L354) | `pub` | no resolved direct caller |
| [endpoint::attachment::AttachmentAuthority::read_authorized_file](../../src/attachment.rs#L433) | `pub` | no resolved direct caller |
| [endpoint::attachment::AttachmentAuthority::read_authorized](../../src/attachment.rs#L515) | `pub` | no resolved direct caller |
| [endpoint::attachment::project_content_blocks](../../src/attachment.rs#L1154) | `pub` | no resolved direct caller |
| [endpoint::carrier_adapter::SESSION_ENDPOINT_PUBLIC_METHODS](../../src/carrier_adapter.rs#L14) | `pub` | not a function |
| [endpoint::carrier_adapter::RespondAuthority](../../src/carrier_adapter.rs#L31) | `pub` | not a function |
| [endpoint::carrier_adapter::RespondDelivery](../../src/carrier_adapter.rs#L47) | `pub` | not a function |
| [endpoint::carrier_adapter::RespondDelivery::as_str](../../src/carrier_adapter.rs#L55) | `pub` | no resolved direct caller |
| [endpoint::carrier_adapter::RespondAuthorReceipt](../../src/carrier_adapter.rs#L65) | `pub` | not a function |
| [endpoint::carrier_adapter::LocatedRespond](../../src/carrier_adapter.rs#L71) | `pub` | not a function |
| [endpoint::carrier_adapter::CarrierRespondHandler](../../src/carrier_adapter.rs#L82) | `pub` | not a function |
| [endpoint::carrier_adapter::JournalRespondHandler](../../src/carrier_adapter.rs#L98) | `pub` | not a function |
| [endpoint::carrier_adapter::JournalRespondHandler::new](../../src/carrier_adapter.rs#L105) | `pub` | [conformance::tests::slice9_gates::slice9_respond_exact_retry_recovers_from_both_commit_windows](../../../conformance/tests/slice9_gates.rs#L413); [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../../supervisor/src/endpoint_carrier.rs#L2875) |
| [endpoint::carrier_adapter::CarrierStreamHandler](../../src/carrier_adapter.rs#L391) | `pub` | not a function |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost](../../src/carrier_adapter.rs#L420) | `pub` | not a function |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::new](../../src/carrier_adapter.rs#L430) | `pub` | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../../supervisor/src/endpoint_carrier.rs#L2875) |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::set_readiness](../../src/carrier_adapter.rs#L440) | `pub` | no resolved direct caller |
| [endpoint::client_wire::client_sync_frames](../../src/client_wire.rs#L20) | `pub` | [endpoint::client_wire::client_mux_frames](../../src/client_wire.rs#L107) |
| [endpoint::client_wire::client_mux_frames](../../src/client_wire.rs#L107) | `pub` | [transport::server::send_mux](../../../transport/src/server.rs#L1060) |
| [endpoint::history::HistoryPage](../../src/history.rs#L8) | `pub` | not a function |
| [endpoint::history::history_page](../../src/history.rs#L13) | `pub` | [conformance::tests::slice9_gates::slice9_gate_68_endpoint_raw_history_reconnect](../../../conformance/tests/slice9_gates.rs#L150); [endpoint::mux::frozen_history_page](../../src/mux.rs#L713); [endpoint::service::NativeEndpoint::history](../../src/service.rs#L971); [endpoint::tests::legacy_replay::legacy_corpus_export_imports_and_projects_through_kernel](../../tests/legacy_replay.rs#L26); [endpoint::tests::slice6_gates::slice6_gate_49_chunks_are_transient_and_never_journaled](../../tests/slice6_gates.rs#L105); [tekes-supervisor::bounded_child_history::read](../../../supervisor/src/bounded_child_history.rs#L80); [tekes-supervisor::endpoint_carrier::legacy_history::FrozenLegacyHistory::page](../../../supervisor/src/endpoint_carrier/legacy_history.rs#L15) |
| [endpoint::history::HistoryError](../../src/history.rs#L128) | `pub` | not a function |
| [endpoint::host::EndpointHostFuture](../../src/host.rs#L22) | `pub` | not a function |
| [endpoint::host::EndpointHostCall](../../src/host.rs#L25) | `pub` | not a function |
| [endpoint::host::EndpointHost](../../src/host.rs#L40) | `pub` | not a function |
| [endpoint::host::CarrierHostFuture](../../src/host.rs#L67) | `pub` | not a function |
| [endpoint::host::HostReadiness](../../src/host.rs#L70) | `pub` | not a function |
| [endpoint::host::MethodClass](../../src/host.rs#L76) | `pub` | not a function |
| [endpoint::host::StreamChannel](../../src/host.rs#L83) | `pub` | not a function |
| [endpoint::host::StreamChannel::method](../../src/host.rs#L90) | `pub` | no resolved direct caller |
| [endpoint::host::StreamChannel::frame_types](../../src/host.rs#L98) | `pub(crate)` | no resolved direct caller |
| [endpoint::host::StreamErrorCode](../../src/host.rs#L130) | `pub` | not a function |
| [endpoint::host::StreamErrorCode::code](../../src/host.rs#L140) | `pub` | no resolved direct caller |
| [endpoint::host::StreamErrorCode::close_code](../../src/host.rs#L151) | `pub` | no resolved direct caller |
| [endpoint::host::CallContext](../../src/host.rs#L162) | `pub` | not a function |
| [endpoint::host::DrainSignal](../../src/host.rs#L171) | `pub` | not a function |
| [endpoint::host::DrainSignal::new](../../src/host.rs#L175) | `pub` | [conformance::tests::slice9_gates::slice9_gate_69_endpoint_drain_backpressure_security](../../../conformance/tests/slice9_gates.rs#L183); [conformance::tests::slice9_gates::call_context](../../../conformance/tests/slice9_gates.rs#L714); [tekes-supervisor::tests::s4_lazy_boot_streams::call_context](../../../supervisor/tests/s4_lazy_boot_streams.rs#L212); [tekes-supervisor::tests::slice9_endpoint_carrier::call_context](../../../supervisor/tests/slice9_endpoint_carrier.rs#L1201); [transport::server::call_context](../../../transport/src/server.rs#L1187) |
| [endpoint::host::DrainSignal::is_draining](../../src/host.rs#L180) | `pub` | no resolved direct caller |
| [endpoint::host::DurableHandoffProof](../../src/host.rs#L186) | `pub` | not a function |
| [endpoint::host::DurableHandoffSignal](../../src/host.rs#L198) | `pub` | not a function |
| [endpoint::host::DurableHandoffSignal::new](../../src/host.rs#L202) | `pub` | [conformance::tests::slice11_gates::slice11_gate_80_production_input_seam_and_mux_authority](../../../conformance/tests/slice11_gates.rs#L530); [conformance::tests::slice9_gates::slice9_gate_69_endpoint_drain_backpressure_security](../../../conformance/tests/slice9_gates.rs#L183); [conformance::tests::slice9_gates::call_context](../../../conformance/tests/slice9_gates.rs#L714); [endpoint::host::EndpointDispatcher::dispatch](../../src/host.rs#L458); [endpoint::tests::slice9_lower_seam::durable_handoff_signal_survives_carrier_timeout_cancellation](../../tests/slice9_lower_seam.rs#L149); [tekes-supervisor::tests::s4_lazy_boot_streams::call_context](../../../supervisor/tests/s4_lazy_boot_streams.rs#L212); [tekes-supervisor::tests::slice14f_common_routes::mutation_forwards_rpc_identity_and_records_authority_handoff](../../../supervisor/tests/slice14f_common_routes.rs#L105); [tekes-supervisor::tests::slice9_endpoint_carrier::call_context](../../../supervisor/tests/slice9_endpoint_carrier.rs#L1201); [transport::server::unary_inner](../../../transport/src/server.rs#L701) |
| [endpoint::host::DurableHandoffSignal::mark_durable](../../src/host.rs#L210) | `pub` | [endpoint::host::DurableHandoffSignal::mark_handed_off](../../src/host.rs#L216) |
| [endpoint::host::DurableHandoffSignal::mark_handed_off](../../src/host.rs#L216) | `pub` | no resolved direct caller |
| [endpoint::host::DurableHandoffSignal::is_durable](../../src/host.rs#L254) | `pub` | no resolved direct caller |
| [endpoint::host::DurableHandoffError](../../src/host.rs#L260) | `pub` | not a function |
| [endpoint::host::EndpointStream](../../src/host.rs#L273) | `pub` | not a function |
| [endpoint::host::EndpointStreamReceiver](../../src/host.rs#L277) | `pub` | not a function |
| [endpoint::host::SessionStream](../../src/host.rs#L279) | `pub` | not a function |
| [endpoint::host::SessionStreamReceiver](../../src/host.rs#L283) | `pub` | not a function |
| [endpoint::host::StreamFailure](../../src/host.rs#L289) | `pub` | not a function |
| [endpoint::host::StreamFailure::new](../../src/host.rs#L296) | `pub` | [conformance::tests::slice9_gates::slice9_gate_69_endpoint_drain_backpressure_security](../../../conformance/tests/slice9_gates.rs#L183); [endpoint::all_session::AllSessionMuxStream::recv](../../src/all_session.rs#L259); [endpoint::stream_queue::EndpointFrameQueue::push_item](../../src/stream_queue.rs#L58); [transport::tests::slice9_transport::MockHost::open_stream](../../../transport/tests/slice9_transport.rs#L372) |
| [endpoint::host::StreamFailure::internal](../../src/host.rs#L304) | `pub` | [endpoint::all_session::AllSessionMuxStream::recv](../../src/all_session.rs#L259); [endpoint::stream_queue::FrameQueueReceiver::recv](../../src/stream_queue.rs#L115); [tekes-supervisor::endpoint_carrier::ProductionStream::recv](../../../supervisor/src/endpoint_carrier.rs#L2656) |
| [endpoint::host::StreamFailure::code](../../src/host.rs#L312) | `pub` | no resolved direct caller |
| [endpoint::host::StreamFailure::diagnostic](../../src/host.rs#L317) | `pub` | no resolved direct caller |
| [endpoint::host::EndpointCarrierHost](../../src/host.rs#L325) | `pub` | not a function |
| [endpoint::host::HostFailure](../../src/host.rs#L406) | `pub` | not a function |
| [endpoint::host::SessionHostDescription](../../src/host.rs#L428) | `pub` | not a function |
| [endpoint::host::EndpointDispatcher](../../src/host.rs#L444) | `pub` | not a function |
| [endpoint::host::EndpointDispatcher::new](../../src/host.rs#L451) | `pub` | [conformance::tests::slice14f_gates::dispatch](../../../conformance/tests/slice14f_gates.rs#L64); [endpoint::carrier_adapter::ComposedEndpointCarrierHost::new](../../src/carrier_adapter.rs#L430); [endpoint::tests::slice9_lower_seam::durable_pending_claim_is_recovered_with_the_same_rpc_id](../../tests/slice9_lower_seam.rs#L106); [endpoint::tests::slice9_lower_seam::dispatcher_persists_typed_handoff_before_completion](../../tests/slice9_lower_seam.rs#L186); [endpoint::tests::slice9_lower_seam::endpoint_wide_rpc_registry_returns_original_result_and_rejects_conflict](../../tests/slice9_lower_seam.rs#L77); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open](../../../supervisor/src/endpoint_host.rs#L1482); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock](../../../supervisor/src/endpoint_host.rs#L1494); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes](../../../supervisor/src/endpoint_host.rs#L1507); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority](../../../supervisor/src/endpoint_host.rs#L1521); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../../supervisor/src/endpoint_host.rs#L1561) |
| [endpoint::host::EndpointDispatcher::dispatch](../../src/host.rs#L458) | `pub` | no resolved direct caller |
| [endpoint::host::EndpointDispatcher::dispatch_with_handoff](../../src/host.rs#L467) | `pub` | [endpoint::host::EndpointDispatcher::dispatch](../../src/host.rs#L458) |
| [endpoint::host::EndpointDispatcher::registry](../../src/host.rs#L590) | `pub` | no resolved direct caller |
| [endpoint::host::EndpointDispatchError](../../src/host.rs#L637) | `pub` | not a function |
| [endpoint::hub::DEFAULT_SUBSCRIPTION_MAX_FRAMES](../../src/hub.rs#L14) | `pub` | not a function |
| [endpoint::hub::DEFAULT_SUBSCRIPTION_MAX_BYTES](../../src/hub.rs#L15) | `pub` | not a function |
| [endpoint::hub::MAX_MUX_SESSIONS](../../src/hub.rs#L16) | `pub` | not a function |
| [endpoint::hub::MuxRegistration](../../src/hub.rs#L18) | `pub` | not a function |
| [endpoint::hub::MuxReplayRegistration](../../src/hub.rs#L25) | `pub` | not a function |
| [endpoint::hub::MuxFrame](../../src/hub.rs#L32) | `pub` | not a function |
| [endpoint::hub::HostFrameKind](../../src/hub.rs#L117) | `pub` | not a function |
| [endpoint::hub::HostFrameKind::as_str](../../src/hub.rs#L132) | `pub` | no resolved direct caller |
| [endpoint::hub::HostFrame](../../src/hub.rs#L152) | `pub` | not a function |
| [endpoint::hub::HostFrame::new](../../src/hub.rs#L158) | `pub` | [conformance::tests::slice9_gates::slice9_gate_69_endpoint_drain_backpressure_security](../../../conformance/tests/slice9_gates.rs#L183); [endpoint::tests::slice9_lower_seam::mux_dto_preserves_the_authority_envelope_shape](../../tests/slice9_lower_seam.rs#L473); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::publish_session_status](../../../supervisor/src/endpoint_carrier.rs#L1097); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::publish_workspace_changed](../../../supervisor/src/endpoint_carrier.rs#L1112); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::attach_session](../../../supervisor/src/endpoint_carrier.rs#L1124); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::detach_for_archive](../../../supervisor/src/endpoint_carrier.rs#L1316) |
| [endpoint::hub::HostFrame::kind](../../src/hub.rs#L175) | `pub` | no resolved direct caller |
| [endpoint::hub::HostFrame::payload](../../src/hub.rs#L180) | `pub` | no resolved direct caller |
| [endpoint::hub::HostFrame::into_envelope](../../src/hub.rs#L184) | `pub` | no resolved direct caller |
| [endpoint::hub::MuxFrame::kind](../../src/hub.rs#L198) | `pub` | [endpoint::hub::MuxFrame::into_envelope](../../src/hub.rs#L239) |
| [endpoint::hub::MuxFrame::session_id](../../src/hub.rs#L215) | `pub` | no resolved direct caller |
| [endpoint::hub::MuxFrame::event_seq](../../src/hub.rs#L232) | `pub` | no resolved direct caller |
| [endpoint::hub::MuxFrame::into_envelope](../../src/hub.rs#L239) | `pub` | no resolved direct caller |
| [endpoint::hub::SubscriptionBaseline](../../src/hub.rs#L257) | `pub` | not a function |
| [endpoint::hub::SubscriptionPoll](../../src/hub.rs#L263) | `pub` | not a function |
| [endpoint::hub::EndpointSubscription](../../src/hub.rs#L390) | `pub` | not a function |
| [endpoint::hub::EndpointSubscription::baseline](../../src/hub.rs#L397) | `pub` | no resolved direct caller |
| [endpoint::hub::EndpointSubscription::poll](../../src/hub.rs#L401) | `pub` | no resolved direct caller |
| [endpoint::hub::EndpointSubscription::close](../../src/hub.rs#L405) | `pub` | no resolved direct caller |
| [endpoint::hub::EndpointSubscription::wake](../../src/hub.rs#L412) | `pub(crate)` | no resolved direct caller |
| [endpoint::hub::EndpointSubscription::poll_with_context](../../src/hub.rs#L417) | `pub(crate)` | no resolved direct caller |
| [endpoint::hub::EndpointSubscriptionHub](../../src/hub.rs#L430) | `pub` | not a function |
| [endpoint::hub::EndpointSubscriptionHub::subscribe](../../src/hub.rs#L435) | `pub` | no resolved direct caller |
| [endpoint::hub::EndpointSubscriptionHub::subscribe_with_bounds](../../src/hub.rs#L450) | `pub` | [endpoint::hub::EndpointSubscriptionHub::subscribe](../../src/hub.rs#L435) |
| [endpoint::hub::EndpointSubscriptionHub::subscribe_all](../../src/hub.rs#L515) | `pub` | no resolved direct caller |
| [endpoint::hub::EndpointSubscriptionHub::subscribe_all_with_replay](../../src/hub.rs#L537) | `pub` | [endpoint::hub::EndpointSubscriptionHub::subscribe_all](../../src/hub.rs#L515) |
| [endpoint::hub::EndpointSubscriptionHub::publish_event](../../src/hub.rs#L632) | `pub` | no resolved direct caller |
| [endpoint::hub::EndpointSubscriptionHub::publish_frame](../../src/hub.rs#L679) | `pub` | no resolved direct caller |
| [endpoint::hub::HubError](../../src/hub.rs#L731) | `pub` | not a function |
| [endpoint::idempotency::RpcClaim](../../src/idempotency.rs#L19) | `pub` | not a function |
| [endpoint::idempotency::RpcClaim::rpc_id](../../src/idempotency.rs#L29) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcClaim::method](../../src/idempotency.rs#L34) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcBegin](../../src/idempotency.rs#L40) | `pub` | not a function |
| [endpoint::idempotency::RpcLookup](../../src/idempotency.rs#L46) | `pub` | not a function |
| [endpoint::idempotency::RpcDurableIdentity](../../src/idempotency.rs#L69) | `pub` | not a function |
| [endpoint::idempotency::RpcRegistry](../../src/idempotency.rs#L116) | `pub` | not a function |
| [endpoint::idempotency::RpcRegistry::open](../../src/idempotency.rs#L123) | `pub` | [conformance::tests::slice14f_gates::dispatch](../../../conformance/tests/slice14f_gates.rs#L64); [conformance::tests::slice9_gates::slice9_respond_exact_retry_recovers_from_both_commit_windows](../../../conformance/tests/slice9_gates.rs#L413); [endpoint::tests::slice9_lower_seam::durable_pending_claim_is_recovered_with_the_same_rpc_id](../../tests/slice9_lower_seam.rs#L106); [endpoint::tests::slice9_lower_seam::dispatcher_persists_typed_handoff_before_completion](../../tests/slice9_lower_seam.rs#L186); [endpoint::tests::slice9_lower_seam::handed_off_claim_is_idempotent_across_restart_and_conflicts_fail_closed](../../tests/slice9_lower_seam.rs#L216); [endpoint::tests::slice9_lower_seam::endpoint_wide_rpc_registry_returns_original_result_and_rejects_conflict](../../tests/slice9_lower_seam.rs#L77); [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../../supervisor/src/endpoint_carrier.rs#L2875); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open](../../../supervisor/src/endpoint_host.rs#L1482); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock](../../../supervisor/src/endpoint_host.rs#L1494); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes](../../../supervisor/src/endpoint_host.rs#L1507); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority](../../../supervisor/src/endpoint_host.rs#L1521); [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../../supervisor/src/endpoint_host.rs#L1561) |
| [endpoint::idempotency::RpcRegistry::path](../../src/idempotency.rs#L146) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcRegistry::begin](../../src/idempotency.rs#L150) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcRegistry::lookup](../../src/idempotency.rs#L179) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcRegistry::handoff](../../src/idempotency.rs#L199) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcRegistry::complete](../../src/idempotency.rs#L219) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcRegistry::mark_handed_off](../../src/idempotency.rs#L315) | `pub` | no resolved direct caller |
| [endpoint::idempotency::RpcRegistryError](../../src/idempotency.rs#L770) | `pub` | not a function |
| [endpoint::journal::JOURNAL_RECORD_VERSION](../../src/journal.rs#L17) | `pub` | not a function |
| [endpoint::journal::JournalRecord](../../src/journal.rs#L21) | `pub` | not a function |
| [endpoint::journal::JournalRecord::validate](../../src/journal.rs#L29) | `pub` | no resolved direct caller |
| [endpoint::journal::JournalRecord::canonical_bytes](../../src/journal.rs#L52) | `pub` | no resolved direct caller |
| [endpoint::journal::JOURNAL_FILE](../../src/journal.rs#L60) | `pub` | not a function |
| [endpoint::journal::LOCK_FILE](../../src/journal.rs#L61) | `pub` | not a function |
| [endpoint::journal::JournalTail](../../src/journal.rs#L123) | `pub` | not a function |
| [endpoint::journal::EndpointJournal](../../src/journal.rs#L132) | `pub` | not a function |
| [endpoint::journal::EndpointJournal::open](../../src/journal.rs#L165) | `pub` | [conformance::tests::slice9_gates::slice9_gate_68_endpoint_raw_history_reconnect](../../../conformance/tests/slice9_gates.rs#L150); [conformance::tests::slice9_gates::slice9_gate_69_endpoint_drain_backpressure_security](../../../conformance/tests/slice9_gates.rs#L183); [conformance::tests::slice9_gates::slice9_gate_70_endpoint_archive_client_rematerialization](../../../conformance/tests/slice9_gates.rs#L277); [conformance::tests::slice9_gates::slice9_gate_67_endpoint_subscribe_atomicity](../../../conformance/tests/slice9_gates.rs#L57); [endpoint::attachment::AttachmentAuthority::read_authorized_file](../../src/attachment.rs#L433); [endpoint::attachment::AttachmentAuthority::read_authorized](../../src/attachment.rs#L515); [endpoint::service::NativeEndpoint::reconciled_journal](../../src/service.rs#L957); [endpoint::session_operations::SessionOperations::fork_session](../../src/session_operations.rs#L760); [endpoint::tests::automatic_title::automatic_title_is_a_durable_one_shot_and_projects_each_visible_write](../../tests/automatic_title.rs#L33); [endpoint::tests::legacy_replay::legacy_corpus_export_imports_and_projects_through_kernel](../../tests/legacy_replay.rs#L26); [endpoint::tests::session_endpoint::journal_follow_freezes_a_cut_before_live_events_and_pages_do_not_drift](../../tests/session_endpoint.rs#L137); [endpoint::tests::slice6_gates::slice6_gate_49_chunks_are_transient_and_never_journaled](../../tests/slice6_gates.rs#L105); [endpoint::tests::slice6_gates::tool_argument_completion_keeps_attempt_identity_in_live_and_replay](../../tests/slice6_gates.rs#L454); [endpoint::tests::slice6_gates::read_only_projection_matches_durable_journal_without_creating_state](../../tests/slice6_gates.rs#L497); [endpoint::tests::slice6_gates::slice6_gate_48_stable_projection_journal](../../tests/slice6_gates.rs#L62); [endpoint::tests::slice9_lower_seam::journal_baseline_and_publish_are_an_atomic_handoff](../../tests/slice9_lower_seam.rs#L274); [endpoint::tests::slice9_lower_seam::subscriber_overflow_preserves_prefix_then_reports_live_gap](../../tests/slice9_lower_seam.rs#L368); [endpoint::tests::slice9_lower_seam::atomic_unresolved_replay_is_ordered_and_private_to_the_new_mux_generation](../../tests/slice9_lower_seam.rs#L418); [endpoint::tests::slice9_lower_seam::publishing_with_no_subscriber_delivers_nothing_and_a_later_subscriber_still_sees_it](../../tests/slice9_lower_seam.rs#L504); [endpoint::tests::slice9_lower_seam::publishing_without_subscribers_still_refuses_an_event_beyond_the_journal_tail](../../tests/slice9_lower_seam.rs#L530); [tekes-supervisor::examples::slice9_production_client_harness_server::publish_one_durable_live_event](../../../supervisor/examples/slice9_production_client_harness_server.rs#L123); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::publish_durable_session_event](../../../supervisor/src/endpoint_carrier.rs#L1539); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::refresh_context_projection](../../../supervisor/src/endpoint_carrier.rs#L1665); [tekes-supervisor::endpoint_carrier::load_mux_journal](../../../supervisor/src/endpoint_carrier.rs#L2974); [tekes-supervisor::process_host::ProductionProcessHost::publish_appended_with](../../../supervisor/src/process_host.rs#L3398); [tekes-supervisor::process_host::ProductionProcessHost::publish_frame](../../../supervisor/src/process_host.rs#L3618); [tekes-supervisor::tests::slice9_endpoint_carrier::production_create_history_then_stream_append_is_durable_and_live](../../../supervisor/tests/slice9_endpoint_carrier.rs#L483); [tekes-worker::live_tests::live_kernel_turn](../../../worker/src/live_tests.rs#L38) |
| [endpoint::journal::EndpointJournal::path](../../src/journal.rs#L199) | `pub` | no resolved direct caller |
| [endpoint::journal::EndpointJournal::thread_folder](../../src/journal.rs#L204) | `pub` | no resolved direct caller |
| [endpoint::journal::EndpointJournal::records](../../src/journal.rs#L210) | `pub` | [endpoint::journal::EndpointJournal::last_settled_endpoint_seq](../../src/journal.rs#L275); [endpoint::journal::EndpointJournal::kernel_anchor_for_endpoint_seq](../../src/journal.rs#L308) |
| [endpoint::journal::EndpointJournal::event](../../src/journal.rs#L223) | `pub` | no resolved direct caller |
| [endpoint::journal::EndpointJournal::last_seq](../../src/journal.rs#L241) | `pub` | no resolved direct caller |
| [endpoint::journal::EndpointJournal::tail_summary](../../src/journal.rs#L247) | `pub` | [endpoint::journal::EndpointJournal::last_seq](../../src/journal.rs#L241) |
| [endpoint::journal::EndpointJournal::last_settled_endpoint_seq](../../src/journal.rs#L275) | `pub` | no resolved direct caller |
| [endpoint::journal::EndpointJournal::kernel_anchor_for_endpoint_seq](../../src/journal.rs#L308) | `pub` | no resolved direct caller |
| [endpoint::journal::EndpointJournal::append_kernel](../../src/journal.rs#L327) | `pub` | no resolved direct caller |
| [endpoint::journal::EndpointJournal::append_kernel_batch](../../src/journal.rs#L340) | `pub` | [endpoint::journal::EndpointJournal::append_kernel](../../src/journal.rs#L327) |
| [endpoint::journal::JournalError](../../src/journal.rs#L507) | `pub` | not a function |
| [endpoint::mux::SESSION_ENDPOINT_PROTOCOL_VERSION](../../src/mux.rs#L15) | `pub` | not a function |
| [endpoint::mux::DEFAULT_JOURNAL_WINDOW_MESSAGES](../../src/mux.rs#L16) | `pub` | not a function |
| [endpoint::mux::MAX_JOURNAL_WINDOW_MESSAGES](../../src/mux.rs#L17) | `pub` | not a function |
| [endpoint::mux::SessionEndpointCapability](../../src/mux.rs#L21) | `pub` | not a function |
| [endpoint::mux::SessionEndpointCapability::required](../../src/mux.rs#L34) | `pub` | [endpoint::mux::MuxHostDescription::validate](../../src/mux.rs#L73); [endpoint::tests::session_endpoint::description_requires_protocol_and_the_recovery_capabilities](../../tests/session_endpoint.rs#L39); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::new](../../../supervisor/src/endpoint_carrier.rs#L984); [tekes-supervisor::endpoint_host::ProductionEndpointHost::mux_description](../../../supervisor/src/endpoint_host.rs#L536); [transport::tests::slice9_transport::MockHost::mux_description](../../../transport/tests/slice9_transport.rs#L441) |
| [endpoint::mux::MuxHostProduct](../../src/mux.rs#L48) | `pub` | not a function |
| [endpoint::mux::MuxHostDescription](../../src/mux.rs#L55) | `pub` | not a function |
| [endpoint::mux::MuxHostDescription::validate](../../src/mux.rs#L73) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionErrorCategory](../../src/mux.rs#L97) | `pub` | not a function |
| [endpoint::mux::Retryability](../../src/mux.rs#L110) | `pub` | not a function |
| [endpoint::mux::SessionRemoteError](../../src/mux.rs#L119) | `pub` | not a function |
| [endpoint::mux::SessionAddress](../../src/mux.rs#L130) | `pub` | not a function |
| [endpoint::mux::SessionAddress::validate](../../src/mux.rs#L136) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionStreamTarget](../../src/mux.rs#L143) | `pub` | not a function |
| [endpoint::mux::SessionMuxClientFrame](../../src/mux.rs#L157) | `pub` | not a function |
| [endpoint::mux::SessionMuxClientFrame::validate](../../src/mux.rs#L190) | `pub` | no resolved direct caller |
| [endpoint::mux::WorkspaceSummary](../../src/mux.rs#L238) | `pub` | not a function |
| [endpoint::mux::WorkspaceBaseline](../../src/mux.rs#L252) | `pub` | not a function |
| [endpoint::mux::SessionSummary](../../src/mux.rs#L260) | `pub` | not a function |
| [endpoint::mux::SessionJournalSnapshot](../../src/mux.rs#L308) | `pub` | not a function |
| [endpoint::mux::SessionJournalPage](../../src/mux.rs#L322) | `pub` | not a function |
| [endpoint::mux::SessionControlItem](../../src/mux.rs#L333) | `pub` | not a function |
| [endpoint::mux::SessionActionableKind](../../src/mux.rs#L343) | `pub` | not a function |
| [endpoint::mux::SessionActionable](../../src/mux.rs#L350) | `pub` | not a function |
| [endpoint::mux::SessionActionable::validate](../../src/mux.rs#L360) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionSyncFrame](../../src/mux.rs#L369) | `pub` | not a function |
| [endpoint::mux::SessionSyncFrame::generation](../../src/mux.rs#L454) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionSyncFrame::is_baseline](../../src/mux.rs#L477) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionMuxServerFrame](../../src/mux.rs#L491) | `pub` | not a function |
| [endpoint::mux::SessionMuxGeneration](../../src/mux.rs#L525) | `pub` | not a function |
| [endpoint::mux::SessionMuxGeneration::new](../../src/mux.rs#L547) | `pub` | [endpoint::tests::session_endpoint::journal_validator_rejects_a_gap_after_the_snapshot](../../tests/session_endpoint.rs#L181); [endpoint::tests::session_endpoint::each_logical_stream_is_baseline_first_and_generation_scoped](../../tests/session_endpoint.rs#L85); [transport::server::remote_mux_inner](../../../transport/src/server.rs#L806) |
| [endpoint::mux::SessionMuxGeneration::generation](../../src/mux.rs#L558) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionMuxGeneration::open](../../src/mux.rs#L562) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionMuxGeneration::close](../../src/mux.rs#L597) | `pub` | no resolved direct caller |
| [endpoint::mux::SessionMuxGeneration::accept](../../src/mux.rs#L604) | `pub` | no resolved direct caller |
| [endpoint::mux::JournalFollow](../../src/mux.rs#L677) | `pub` | not a function |
| [endpoint::mux::open_journal_follow](../../src/mux.rs#L682) | `pub` | [endpoint::tests::session_endpoint::journal_follow_freezes_a_cut_before_live_events_and_pages_do_not_drift](../../tests/session_endpoint.rs#L137) |
| [endpoint::mux::frozen_history_page](../../src/mux.rs#L713) | `pub` | [endpoint::mux::open_journal_follow](../../src/mux.rs#L682); [endpoint::tests::session_endpoint::journal_follow_freezes_a_cut_before_live_events_and_pages_do_not_drift](../../tests/session_endpoint.rs#L137); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::open_mux_journal](../../../supervisor/src/endpoint_carrier.rs#L2161); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::mux_journal_page](../../../supervisor/src/endpoint_carrier.rs#L2386) |
| [endpoint::mux::ActionableRegistry](../../src/mux.rs#L746) | `pub` | not a function |
| [endpoint::mux::ActionableRegistry::upsert](../../src/mux.rs#L751) | `pub` | no resolved direct caller |
| [endpoint::mux::ActionableRegistry::baseline](../../src/mux.rs#L777) | `pub` | no resolved direct caller |
| [endpoint::mux::ActionableRegistry::resolve](../../src/mux.rs#L787) | `pub` | no resolved direct caller |
| [endpoint::mux::MuxProtocolError](../../src/mux.rs#L874) | `pub` | not a function |
| [endpoint::projection::Projector](../../src/projection.rs#L54) | `pub` | not a function |
| [endpoint::projection::AcceptedStreamFrame](../../src/projection.rs#L77) | `pub` | not a function |
| [endpoint::projection::Projector::processed_through](../../src/projection.rs#L94) | `pub` | no resolved direct caller |
| [endpoint::projection::Projector::reconcile](../../src/projection.rs#L98) | `pub` | no resolved direct caller |
| [endpoint::projection::Projector::read_only_history](../../src/projection.rs#L136) | `pub` | [endpoint::projection::Projector::read_only_legacy_history](../../src/projection.rs#L186); [endpoint::tests::slice6_gates::read_only_projection_matches_durable_journal_without_creating_state](../../tests/slice6_gates.rs#L497); [tekes-supervisor::bounded_child_history::read](../../../supervisor/src/bounded_child_history.rs#L80) |
| [endpoint::projection::Projector::read_only_legacy_history](../../src/projection.rs#L186) | `pub` | [tekes-supervisor::endpoint_carrier::legacy_history::ProductionCarrierStreams::open_legacy_journal](../../../supervisor/src/endpoint_carrier/legacy_history.rs#L76) |
| [endpoint::projection::Projector::stream_event](../../src/projection.rs#L196) | `pub` | no resolved direct caller |
| [endpoint::projection::timestamp_millis](../../src/projection.rs#L1052) | `pub(crate)` | [endpoint::projection::Projector::project](../../src/projection.rs#L257); [endpoint::service::card_updated_ms](../../src/service.rs#L172); [endpoint::service::last_record_updated_ms](../../src/service.rs#L187) |
| [endpoint::projection::ProjectionError](../../src/projection.rs#L1099) | `pub` | not a function |
| [endpoint::requests::RequestFrameType](../../src/requests.rs#L14) | `pub` | not a function |
| [endpoint::requests::RequestFrameType::as_str](../../src/requests.rs#L23) | `pub` | no resolved direct caller |
| [endpoint::requests::ResolutionOutcome](../../src/requests.rs#L33) | `pub` | not a function |
| [endpoint::requests::PendingRequest](../../src/requests.rs#L41) | `pub` | not a function |
| [endpoint::requests::RequestState](../../src/requests.rs#L53) | `pub` | not a function |
| [endpoint::requests::PendingRequest::derive](../../src/requests.rs#L62) | `pub` | [conformance::tests::slice9_gates::slice9_respond_request_is_derived_from_the_hold_and_resolved_by_the_ledger](../../../conformance/tests/slice9_gates.rs#L346); [conformance::tests::slice9_gates::approval_request](../../../conformance/tests/slice9_gates.rs#L673); [tekes-supervisor::endpoint_carrier::line_requests](../../../supervisor/src/endpoint_carrier.rs#L3131); [tekes-supervisor::tests::slice9_endpoint_carrier::production_mux_journal_is_follow_first_and_actionables_replay_by_stable_revision](../../../supervisor/tests/slice9_endpoint_carrier.rs#L179); [tekes-supervisor::tests::slice9_endpoint_carrier::production_respond_uses_shared_admission_and_durable_locked_append_handoff](../../../supervisor/tests/slice9_endpoint_carrier.rs#L641); [tekes-supervisor::tests::slice9_endpoint_carrier::child_respond_binds_parent_spawn_and_writes_only_child_ledger](../../../supervisor/tests/slice9_endpoint_carrier.rs#L803) |
| [endpoint::requests::RequestState::resolved](../../src/requests.rs#L97) | `pub` | [conformance::tests::slice9_gates::slice9_respond_request_is_derived_from_the_hold_and_resolved_by_the_ledger](../../../conformance/tests/slice9_gates.rs#L346); [conformance::tests::slice9_gates::MockRespondAuthority::locate](../../../conformance/tests/slice9_gates.rs#L734); [tekes-supervisor::endpoint_carrier::line_requests](../../../supervisor/src/endpoint_carrier.rs#L3131) |
| [endpoint::requests::derive_request_rpc_id](../../src/requests.rs#L111) | `pub` | [endpoint::requests::derive_line_request_rpc_id](../../src/requests.rs#L128) |
| [endpoint::requests::derive_line_request_rpc_id](../../src/requests.rs#L128) | `pub` | [endpoint::requests::PendingRequest::derive](../../src/requests.rs#L62); [tekes-supervisor::process_host::tests::child_rpc_id](../../../supervisor/src/process_host.rs#L11506); [tekes-supervisor::request_index::RequestIndex::insert](../../../supervisor/src/request_index.rs#L60); [tekes-supervisor::tests::s4_lazy_boot_streams::child_rpc](../../../supervisor/tests/s4_lazy_boot_streams.rs#L1267); [tekes-supervisor::tests::s4_lazy_boot_streams::approval_rpc](../../../supervisor/tests/s4_lazy_boot_streams.rs#L237) |
| [endpoint::requests::RequestError](../../src/requests.rs#L210) | `pub` | not a function |
| [endpoint::respond::RespondRejectionReason](../../src/respond.rs#L12) | `pub` | not a function |
| [endpoint::respond::RespondRejectionReason::as_str](../../src/respond.rs#L23) | `pub` | no resolved direct caller |
| [endpoint::respond::RespondPrepareContext](../../src/respond.rs#L36) | `pub` | not a function |
| [endpoint::respond::RespondAuthorization](../../src/respond.rs#L45) | `pub` | not a function |
| [endpoint::respond::RespondDecision](../../src/respond.rs#L57) | `pub` | not a function |
| [endpoint::respond::RespondLifecycle](../../src/respond.rs#L63) | `pub` | not a function |
| [endpoint::respond::RespondLifecycle::prepare](../../src/respond.rs#L68) | `pub` | [conformance::tests::slice9_gates::slice9_respond_request_is_derived_from_the_hold_and_resolved_by_the_ledger](../../../conformance/tests/slice9_gates.rs#L346); [endpoint::carrier_adapter::JournalRespondHandler::respond](../../src/carrier_adapter.rs#L114) |
| [endpoint::respond::RespondLifecycleError](../../src/respond.rs#L233) | `pub` | not a function |
| [endpoint::rpc::MAX_RPC_ID_BYTES](../../src/rpc.rs#L7) | `pub` | not a function |
| [endpoint::rpc::ClientRequest](../../src/rpc.rs#L11) | `pub` | not a function |
| [endpoint::rpc::RpcError](../../src/rpc.rs#L22) | `pub` | not a function |
| [endpoint::rpc::RpcResult](../../src/rpc.rs#L30) | `pub` | not a function |
| [endpoint::rpc::RpcResult::validate](../../src/rpc.rs#L39) | `pub` | no resolved direct caller |
| [endpoint::rpc::ServerResponse](../../src/rpc.rs#L49) | `pub` | not a function |
| [endpoint::rpc::ServerRequest](../../src/rpc.rs#L62) | `pub` | not a function |
| [endpoint::rpc::ServerRequest::validate](../../src/rpc.rs#L72) | `pub` | [endpoint::rpc::ServerRequest::canonical_bytes](../../src/rpc.rs#L83); [endpoint::rpc::ServerRequest::validate_stream](../../src/rpc.rs#L89) |
| [endpoint::rpc::ServerRequest::canonical_bytes](../../src/rpc.rs#L83) | `pub` | no resolved direct caller |
| [endpoint::rpc::ServerRequest::validate_stream](../../src/rpc.rs#L89) | `pub` | no resolved direct caller |
| [endpoint::rpc::ClientResponse](../../src/rpc.rs#L114) | `pub` | not a function |
| [endpoint::rpc::ClientResponse::validate](../../src/rpc.rs#L123) | `pub` | no resolved direct caller |
| [endpoint::rpc::ClientResponseResult](../../src/rpc.rs#L134) | `pub` | not a function |
| [endpoint::rpc::ClientResponseResult::validate](../../src/rpc.rs#L143) | `pub` | no resolved direct caller |
| [endpoint::rpc::RespondReceipt](../../src/rpc.rs#L153) | `pub` | not a function |
| [endpoint::rpc::validate_response](../../src/rpc.rs#L159) | `pub` | [endpoint::idempotency::parse_cached_result](../../src/idempotency.rs#L702); [transport::server::unary_inner](../../../transport/src/server.rs#L701) |
| [endpoint::rpc::validate_request](../../src/rpc.rs#L172) | `pub` | no resolved direct caller |
| [endpoint::rpc::validate_rpc_id](../../src/rpc.rs#L186) | `pub` | [endpoint::idempotency::make_claim](../../src/idempotency.rs#L491); [endpoint::idempotency::parse](../../src/idempotency.rs#L535); [endpoint::rpc::ClientResponse::validate](../../src/rpc.rs#L123); [endpoint::rpc::validate_response](../../src/rpc.rs#L159); [endpoint::rpc::validate_request](../../src/rpc.rs#L172); [endpoint::rpc::ServerRequest::validate](../../src/rpc.rs#L72); [transport::server::unary_inner](../../../transport/src/server.rs#L701) |
| [endpoint::rpc::RequestError](../../src/rpc.rs#L194) | `pub` | not a function |
| [endpoint::rpc::MuxBuffer](../../src/rpc.rs#L224) | `pub` | not a function |
| [endpoint::rpc::MuxBuffer::new](../../src/rpc.rs#L239) | `pub` | [endpoint::tests::slice6_gates::slice6_gate_52_stitch_gap_overlap_and_unknown_fail_closed](../../tests/slice6_gates.rs#L373) |
| [endpoint::rpc::MuxBuffer::push](../../src/rpc.rs#L248) | `pub` | no resolved direct caller |
| [endpoint::rpc::MuxBuffer::pop](../../src/rpc.rs#L262) | `pub` | no resolved direct caller |
| [endpoint::rpc::MuxBufferError](../../src/rpc.rs#L270) | `pub` | not a function |
| [endpoint::service::AUTOMATIC_TITLE_SEED_OPERATION](../../src/service.rs#L13) | `pub` | not a function |
| [endpoint::service::AUTOMATIC_TITLE_REFINE_OPERATION](../../src/service.rs#L14) | `pub` | not a function |
| [endpoint::service::SESSION_NOTICE_OPERATION](../../src/service.rs#L15) | `pub` | not a function |
| [endpoint::service::SessionNotice](../../src/service.rs#L22) | `pub` | not a function |
| [endpoint::service::SessionNoticeSeverity](../../src/service.rs#L30) | `pub` | not a function |
| [endpoint::service::SessionNoticeSeverity::as_str](../../src/service.rs#L37) | `pub` | no resolved direct caller |
| [endpoint::service::MutationReceipt](../../src/service.rs#L46) | `pub` | not a function |
| [endpoint::service::ArchiveResult](../../src/service.rs#L61) | `pub` | not a function |
| [endpoint::service::UnarchiveResult](../../src/service.rs#L66) | `pub` | not a function |
| [endpoint::service::SessionInventoryItem](../../src/service.rs#L71) | `pub` | not a function |
| [endpoint::service::genesis_cwd](../../src/service.rs#L101) | `pub` | [endpoint::service::card_item](../../src/service.rs#L133); [endpoint::service::NativeEndpoint::list_sessions_with](../../src/service.rs#L306); [tekes-supervisor::process_host::ProductionProcessHost::catalog_item](../../../supervisor/src/process_host.rs#L5241); [tekes-supervisor::thread_card::prepare](../../../supervisor/src/thread_card.rs#L155) |
| [endpoint::service::genesis_preset_summary](../../src/service.rs#L110) | `pub` | [endpoint::service::card_summary](../../src/service.rs#L210); [tekes-supervisor::process_host::ProductionProcessHost::catalog_item](../../../supervisor/src/process_host.rs#L5241) |
| [endpoint::service::permission_mode_of](../../src/service.rs#L116) | `pub` | [endpoint::service::card_item](../../src/service.rs#L133); [endpoint::service::NativeEndpoint::list_sessions_with](../../src/service.rs#L306); [tekes-supervisor::process_host::ProductionProcessHost::catalog_item](../../../supervisor/src/process_host.rs#L5241) |
| [endpoint::service::card_updated_ms](../../src/service.rs#L172) | `pub` | [endpoint::service::card_summary](../../src/service.rs#L210); [tekes-supervisor::catalog::Catalog::refresh_light](../../../supervisor/src/catalog.rs#L310) |
| [endpoint::service::last_record_updated_ms](../../src/service.rs#L187) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::catalog_item](../../../supervisor/src/process_host.rs#L5241) |
| [endpoint::service::card_summary](../../src/service.rs#L210) | `pub` | [endpoint::service::NativeEndpoint::list_sessions_with](../../src/service.rs#L306); [tekes-supervisor::thread_card::main_line](../../../supervisor/src/thread_card.rs#L129) |
| [endpoint::service::NativeEndpoint](../../src/service.rs#L258) | `pub` | not a function |
| [endpoint::service::NativeEndpoint::open](../../src/service.rs#L263) | `pub` | [conformance::tests::slice11_gates::bootstrap_input_session](../../../conformance/tests/slice11_gates.rs#L57); [conformance::tests::slice9_gates::slice9_gate_70_endpoint_archive_client_rematerialization](../../../conformance/tests/slice9_gates.rs#L277); [endpoint::session_operations::SessionOperations::fork_session](../../src/session_operations.rs#L760); [endpoint::tests::automatic_title::a_manual_rename_wins_the_refinement_race](../../tests/automatic_title.rs#L116); [endpoint::tests::automatic_title::automatic_title_is_a_durable_one_shot_and_projects_each_visible_write](../../tests/automatic_title.rs#L33); [endpoint::tests::session_notice::identical_notice_folds_until_the_next_input](../../tests/session_notice.rs#L29); [endpoint::tests::slice6_gates::slice6_gate_50_endpoint_mutation_and_archive_contract](../../tests/slice6_gates.rs#L183); [endpoint::tests::slice6_gates::slice6_gate_51_inventory_and_model_readiness_are_independent](../../tests/slice6_gates.rs#L276); [endpoint::tests::submission_intent::submission_is_durable_before_run_and_retry_keeps_the_original_selection](../../tests/submission_intent.rs#L8); [endpoint::tests::submission_intent::existing_history_uses_its_recorded_config_without_mutating_the_journal](../../tests/submission_intent.rs#L95); [tekes-supervisor::examples::slice9_production_client_harness_server::main](../../../supervisor/examples/slice9_production_client_harness_server.rs#L176); [tekes-supervisor::client_extensions::tests::tools_list_is_closed_and_includes_every_fixed_tool](../../../supervisor/src/client_extensions.rs#L2109); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::refresh_context_projection_locked](../../../supervisor/src/endpoint_carrier.rs#L1701); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::session_items](../../../supervisor/src/endpoint_carrier.rs#L1759); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::open_mux_journal](../../../supervisor/src/endpoint_carrier.rs#L2161); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::mux_journal_page](../../../supervisor/src/endpoint_carrier.rs#L2386); [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../../supervisor/src/endpoint_host.rs#L684); [tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_store](../../../supervisor/src/process_host.rs#L802); [tekes-supervisor::tests::s4_lazy_boot_streams::journal_page_does_not_rescan_unused_actionable_requests](../../../supervisor/tests/s4_lazy_boot_streams.rs#L1515); [tekes-supervisor::tests::s4_lazy_boot_streams::journal_open_does_not_rescan_unused_actionable_requests](../../../supervisor/tests/s4_lazy_boot_streams.rs#L1648); [tekes-supervisor::tests::session_execution_endpoint::create_replay_reopen_and_inventory_ignore_retired_registries](../../../supervisor/tests/session_execution_endpoint.rs#L47); [tekes-supervisor::tests::slice9_endpoint_carrier::context_control_reconnect_restores_valid_pair_and_replaces_it_after_compaction](../../../supervisor/tests/slice9_endpoint_carrier.rs#L350); [tekes-supervisor::tests::slice9_endpoint_carrier::production_create_history_then_stream_append_is_durable_and_live](../../../supervisor/tests/slice9_endpoint_carrier.rs#L483); [tekes-supervisor::tests::slice9_endpoint_host::ephemeral_fork_is_a_scratch_ledger_that_discard_and_startup_remove](../../../supervisor/tests/slice9_endpoint_host.rs#L1419); [tekes-supervisor::tests::slice9_endpoint_host::bootstrap](../../../supervisor/tests/slice9_endpoint_host.rs#L233); [tekes-supervisor::tests::slice9_endpoint_host::session_create_stages_snapshots_and_recovers_from_pre_carrier_record](../../../supervisor/tests/slice9_endpoint_host.rs#L301); [tekes-supervisor::tests::slice9_endpoint_host::session_create_freezes_a_preset_and_refuses_invalid_ones](../../../supervisor/tests/slice9_endpoint_host.rs#L404); [tekes-supervisor::tests::slice9_endpoint_host::inventory_reports_the_durable_permission_mode](../../../supervisor/tests/slice9_endpoint_host.rs#L518); [tekes-supervisor::tests::slice9_endpoint_host::cancel_and_rename_use_exact_endpoint_origins_and_durable_receipts](../../../supervisor/tests/slice9_endpoint_host.rs#L877) |
| [endpoint::service::NativeEndpoint::capabilities](../../src/service.rs#L270) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::list_sessions](../../src/service.rs#L289) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::list_sessions_cached](../../src/service.rs#L299) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::models](../../src/service.rs#L370) | `pub` | [endpoint::tests::slice6_gates::slice6_gate_51_inventory_and_model_readiness_are_independent](../../tests/slice6_gates.rs#L276) |
| [endpoint::service::NativeEndpoint::session_config_snapshot](../../src/service.rs#L434) | `pub` | [endpoint::service::NativeEndpoint::submission_snapshot](../../src/service.rs#L446) |
| [endpoint::service::NativeEndpoint::submission_snapshot](../../src/service.rs#L446) | `pub` | [endpoint::service::NativeEndpoint::prompt_with_operation](../../src/service.rs#L638) |
| [endpoint::service::NativeEndpoint::submission_metadata](../../src/service.rs#L453) | `pub` | [endpoint::service::NativeEndpoint::submission_snapshot](../../src/service.rs#L446); [endpoint::service::NativeEndpoint::hydrate_historical_submissions](../../src/service.rs#L476) |
| [endpoint::service::NativeEndpoint::hydrate_historical_submissions](../../src/service.rs#L476) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::create_session](../../src/service.rs#L584) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::prompt](../../src/service.rs#L609) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::prompt_for_endpoint](../../src/service.rs#L620) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::update_queue](../../src/service.rs#L661) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::rename_session](../../src/service.rs#L676) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::rename_session_for_endpoint](../../src/service.rs#L686) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::seed_automatic_title_if_missing](../../src/service.rs#L715) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::record_session_notice](../../src/service.rs#L748) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::refine_automatic_title](../../src/service.rs#L802) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::author_keyed_with_ledger](../../src/service.rs#L843) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::cancel](../../src/service.rs#L866) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::cancel_for_endpoint](../../src/service.rs#L875) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::fork_session](../../src/service.rs#L914) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::archive_session](../../src/service.rs#L930) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::unarchive_session](../../src/service.rs#L938) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::reconcile_projection](../../src/service.rs#L949) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpoint::history](../../src/service.rs#L971) | `pub` | no resolved direct caller |
| [endpoint::service::NativeEndpointError](../../src/service.rs#L1043) | `pub` | not a function |
| [endpoint::session_event_registry_generated::SessionEventProjectionKind](../../src/session_event_registry_generated.rs#L5) | `pub(crate)` | not a function |
| [endpoint::session_event_registry_generated::SessionEventUiAuthority](../../src/session_event_registry_generated.rs#L13) | `pub(crate)` | not a function |
| [endpoint::session_event_registry_generated::SessionEventProducer](../../src/session_event_registry_generated.rs#L20) | `pub(crate)` | not a function |
| [endpoint::session_event_registry_generated::SessionEventRegistryEntry](../../src/session_event_registry_generated.rs#L27) | `pub(crate)` | not a function |
| [endpoint::session_event_registry_generated::SESSION_EVENT_REGISTRY](../../src/session_event_registry_generated.rs#L35) | `pub(crate)` | not a function |
| [endpoint::session_event_registry_generated::session_event_registry_entry](../../src/session_event_registry_generated.rs#L422) | `pub(crate)` | [endpoint::session_event_registry_generated::is_surface_event_type](../../src/session_event_registry_generated.rs#L430); [endpoint::session_event_registry_generated::kernel_may_emit_event_type](../../src/session_event_registry_generated.rs#L435); [endpoint::types::SessionEvent::validate](../../src/types.rs#L46) |
| [endpoint::session_event_registry_generated::is_surface_event_type](../../src/session_event_registry_generated.rs#L430) | `pub(crate)` | [endpoint::types::SessionEvent::validate](../../src/types.rs#L46) |
| [endpoint::session_event_registry_generated::kernel_may_emit_event_type](../../src/session_event_registry_generated.rs#L435) | `pub(crate)` | [endpoint::projection::session_event](../../src/projection.rs#L767) |
| [endpoint::session_operations::ORIGIN_CLIENT](../../src/session_operations.rs#L125) | `pub` | not a function |
| [endpoint::session_operations::is_endpoint_origin_client](../../src/session_operations.rs#L128) | `pub` | [endpoint::session_operations::verify_session_create_genesis](../../src/session_operations.rs#L1876) |
| [endpoint::session_operations::PresetModelSource](../../src/session_operations.rs#L148) | `pub` | not a function |
| [endpoint::session_operations::SessionPresetEnvironment](../../src/session_operations.rs#L163) | `pub` | not a function |
| [endpoint::session_operations::SessionPresetEnvironment::new](../../src/session_operations.rs#L174) | `pub` | [endpoint::session_operations::SessionOperations::create_execution_session](../../src/session_operations.rs#L546); [endpoint::session_operations::SessionOperations::resolve_fork_preset](../../src/session_operations.rs#L885); [tekes-supervisor::client_extensions::ProductionClientExtensions::execute](../../../supervisor/src/client_extensions.rs#L897) |
| [endpoint::session_operations::SessionPresetEnvironment::with_models](../../src/session_operations.rs#L193) | `pub` | no resolved direct caller |
| [endpoint::session_operations::ExecutionCreateOperation](../../src/session_operations.rs#L234) | `pub` | not a function |
| [endpoint::session_operations::SelectModelOperation](../../src/session_operations.rs#L246) | `pub` | not a function |
| [endpoint::session_operations::ForkSessionOperation](../../src/session_operations.rs#L256) | `pub` | not a function |
| [endpoint::session_operations::ForkLineage](../../src/session_operations.rs#L274) | `pub` | not a function |
| [endpoint::session_operations::QueueTransactionOperation](../../src/session_operations.rs#L279) | `pub` | not a function |
| [endpoint::session_operations::PendingQueueTransaction](../../src/session_operations.rs#L292) | `pub` | not a function |
| [endpoint::session_operations::QueueTransactionDecision](../../src/session_operations.rs#L303) | `pub` | not a function |
| [endpoint::session_operations::QueueTransactionCompletion](../../src/session_operations.rs#L312) | `pub` | not a function |
| [endpoint::session_operations::QueueTransactionState](../../src/session_operations.rs#L323) | `pub` | not a function |
| [endpoint::session_operations::QueueRecoveryDriver](../../src/session_operations.rs#L328) | `pub` | not a function |
| [endpoint::session_operations::SelectedModel](../../src/session_operations.rs#L336) | `pub` | not a function |
| [endpoint::session_operations::CompleteOperations](../../src/session_operations.rs#L345) | `pub` | not a function |
| [endpoint::session_operations::SessionOperations](../../src/session_operations.rs#L355) | `pub` | not a function |
| [endpoint::session_operations::SessionOperations::open_sessions](../../src/session_operations.rs#L364) | `pub` | [conformance::tests::slice14f_gates::execution_create_replay_archive_restore_and_discard_survive_recovery](../../../conformance/tests/slice14f_gates.rs#L152); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::archive_session](../../../supervisor/src/endpoint_carrier.rs#L1045); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::inventory_baseline](../../../supervisor/src/endpoint_carrier.rs#L1776); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::inventory_row](../../../supervisor/src/endpoint_carrier.rs#L1802); [tekes-supervisor::endpoint_host::recover_pending_management](../../../supervisor/src/endpoint_host.rs#L190); [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../../supervisor/src/endpoint_host.rs#L684); [tekes-supervisor::process_host::tests::a_client_unarchive_admits_the_thread_and_lists_it_again](../../../supervisor/src/process_host.rs#L10849); [tekes-supervisor::tests::slice9_endpoint_host::ephemeral_fork_is_a_scratch_ledger_that_discard_and_startup_remove](../../../supervisor/tests/slice9_endpoint_host.rs#L1419); [tekes-supervisor::tests::slice9_endpoint_host::opening_a_root_with_more_than_256_session_folders_is_not_an_error](../../../supervisor/tests/slice9_endpoint_host.rs#L39); [tekes-supervisor::tests::slice9_endpoint_host::queue_management_gate_recovers_before_readiness_and_exact_retry_does_not_redeliver](../../../supervisor/tests/slice9_endpoint_host.rs#L973) |
| [endpoint::session_operations::SessionOperations::recover_sessions](../../src/session_operations.rs#L380) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::root](../../src/session_operations.rs#L389) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::recover](../../src/session_operations.rs#L394) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::recover_with_queue_driver](../../src/session_operations.rs#L398) | `pub` | [endpoint::session_operations::SessionOperations::recover](../../src/session_operations.rs#L394) |
| [endpoint::session_operations::SessionOperations::create_execution_session](../../src/session_operations.rs#L546) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::select_model](../../src/session_operations.rs#L678) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::fork_session](../../src/session_operations.rs#L760) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::prepare_queue_transaction](../../src/session_operations.rs#L907) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::complete_queue_transaction](../../src/session_operations.rs#L1010) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::has_incomplete_session_operation](../../src/session_operations.rs#L1026) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::completed_fork_lineage](../../src/session_operations.rs#L1043) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::archive_session](../../src/session_operations.rs#L1066) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::unarchive_session](../../src/session_operations.rs#L1096) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperations::discard_session](../../src/session_operations.rs#L1119) | `pub` | no resolved direct caller |
| [endpoint::session_operations::SessionOperationError](../../src/session_operations.rs#L2464) | `pub` | not a function |
| [endpoint::stitch::StitchDecision](../../src/stitch.rs#L8) | `pub` | not a function |
| [endpoint::stitch::stitch_window](../../src/stitch.rs#L13) | `pub` | [endpoint::tests::slice6_gates::slice6_gate_52_stitch_gap_overlap_and_unknown_fail_closed](../../tests/slice6_gates.rs#L373) |
| [endpoint::stitch::StitchError](../../src/stitch.rs#L68) | `pub` | not a function |
| [endpoint::stream_queue::EndpointFrameQueue](../../src/stream_queue.rs#L16) | `pub` | not a function |
| [endpoint::stream_queue::EndpointFrameQueue::new](../../src/stream_queue.rs#L30) | `pub` | [conformance::tests::slice9_gates::slice9_gate_69_endpoint_drain_backpressure_security](../../../conformance/tests/slice9_gates.rs#L183); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::open_host](../../../supervisor/src/endpoint_carrier.rs#L1598) |
| [endpoint::stream_queue::EndpointFrameQueue::push](../../src/stream_queue.rs#L46) | `pub` | no resolved direct caller |
| [endpoint::stream_queue::EndpointFrameQueue::fail](../../src/stream_queue.rs#L54) | `pub` | no resolved direct caller |
| [endpoint::stream_queue::EndpointFrameQueue::close](../../src/stream_queue.rs#L93) | `pub` | no resolved direct caller |
| [endpoint::stream_queue::EndpointFrameQueue::receiver](../../src/stream_queue.rs#L103) | `pub` | no resolved direct caller |
| [endpoint::stream_queue::FrameQueueError](../../src/stream_queue.rs#L146) | `pub` | not a function |
| [endpoint::types::MAX_SAFE_SEQUENCE](../../src/types.rs#L9) | `pub` | not a function |
| [endpoint::types::SurfaceOperation](../../src/types.rs#L13) | `pub` | not a function |
| [endpoint::types::SurfaceOperation::validate](../../src/types.rs#L19) | `pub` | no resolved direct caller |
| [endpoint::types::SessionEvent](../../src/types.rs#L31) | `pub` | not a function |
| [endpoint::types::SessionEvent::validate](../../src/types.rs#L46) | `pub` | no resolved direct caller |
| [endpoint::types::SessionEvent::canonical_bytes](../../src/types.rs#L84) | `pub` | no resolved direct caller |
| [endpoint::types::SessionToolEventView](../../src/types.rs#L92) | `pub` | not a function |
| [endpoint::types::SessionHistoryEntry](../../src/types.rs#L100) | `pub` | not a function |
| [endpoint::types::EndpointTypeError](../../src/types.rs#L107) | `pub` | not a function |
| [endpoint::types::validate_session_id](../../src/types.rs#L130) | `pub` | [endpoint::attachment::AttachmentAuthority::materialize_prompt_parts](../../src/attachment.rs#L251); [endpoint::attachment::AttachmentAuthority::upload_file](../../src/attachment.rs#L354); [endpoint::attachment::AttachmentAuthority::read_authorized_file](../../src/attachment.rs#L433); [endpoint::attachment::AttachmentAuthority::read_authorized](../../src/attachment.rs#L515); [endpoint::hub::EndpointSubscriptionHub::subscribe_with_bounds](../../src/hub.rs#L450); [endpoint::hub::EndpointSubscriptionHub::subscribe_all_with_replay](../../src/hub.rs#L537); [endpoint::hub::EndpointSubscriptionHub::publish](../../src/hub.rs#L697); [endpoint::mux::SessionAddress::validate](../../src/mux.rs#L136); [endpoint::mux::SessionActionable::validate](../../src/mux.rs#L360); [endpoint::requests::validate_source_line](../../src/requests.rs#L151); [endpoint::requests::PendingRequest::derive](../../src/requests.rs#L62); [endpoint::service::NativeEndpoint::active_folder](../../src/service.rs#L1012); [endpoint::service::NativeEndpoint::list_sessions_with](../../src/service.rs#L306); [endpoint::service::NativeEndpoint::session_config_snapshot](../../src/service.rs#L434); [endpoint::service::NativeEndpoint::create_session](../../src/service.rs#L584); [endpoint::service::NativeEndpoint::author_keyed_with_ledger](../../src/service.rs#L843); [endpoint::service::NativeEndpoint::cancel_with_operation](../../src/service.rs#L884); [endpoint::service::NativeEndpoint::fork_session](../../src/service.rs#L914); [endpoint::service::NativeEndpoint::archive_session](../../src/service.rs#L930); [endpoint::service::NativeEndpoint::unarchive_session](../../src/service.rs#L938); [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L999); [endpoint::session_operations::SessionOperations::has_incomplete_session_operation](../../src/session_operations.rs#L1026); [endpoint::session_operations::SessionOperations::discard_session](../../src/session_operations.rs#L1119); [endpoint::session_operations::SessionOperations::folder_move](../../src/session_operations.rs#L1241); [endpoint::session_operations::discard_intent](../../src/session_operations.rs#L2039); [endpoint::session_operations::session_create_intent](../../src/session_operations.rs#L2068); [endpoint::session_operations::select_model_intent](../../src/session_operations.rs#L2095); [endpoint::session_operations::fork_intent](../../src/session_operations.rs#L2117); [endpoint::session_operations::queue_transaction_intent](../../src/session_operations.rs#L2134); [endpoint::session_operations::SessionOperations::create_execution_session](../../src/session_operations.rs#L546); [endpoint::session_operations::SessionOperations::select_model](../../src/session_operations.rs#L678); [endpoint::session_operations::SessionOperations::fork_session](../../src/session_operations.rs#L760); [endpoint::session_operations::SessionOperations::prepare_queue_transaction](../../src/session_operations.rs#L907); [tekes-supervisor::bounded_child_history::request](../../../supervisor/src/bounded_child_history.rs#L19); [tekes-supervisor::catalog::Catalog::ensure_loaded](../../../supervisor/src/catalog.rs#L187); [tekes-supervisor::client_extensions::validate_payload](../../../supervisor/src/client_extensions.rs#L1714); [tekes-supervisor::client_extensions::ProductionClientExtensions::session_folder](../../../supervisor/src/client_extensions.rs#L845); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::refresh_context_projection](../../../supervisor/src/endpoint_carrier.rs#L1665); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::refresh_context_projection_shared](../../../supervisor/src/endpoint_carrier.rs#L1685); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::index_unverified_and_archived](../../../supervisor/src/endpoint_carrier.rs#L2042); [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::locate_by_scan](../../../supervisor/src/endpoint_carrier.rs#L290); [tekes-supervisor::endpoint_carrier::active_session_ids](../../../supervisor/src/endpoint_carrier.rs#L2961); [tekes-supervisor::endpoint_carrier::load_mux_journal](../../../supervisor/src/endpoint_carrier.rs#L2974); [tekes-supervisor::endpoint_carrier::semantic_line_projection_mode](../../../supervisor/src/endpoint_carrier.rs#L3098); [tekes-supervisor::endpoint_carrier::session_requests_mode](../../../supervisor/src/endpoint_carrier.rs#L3234); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::recover_actionables](../../../supervisor/src/endpoint_carrier.rs#L780); [tekes-supervisor::endpoint_carrier::legacy_history::ProductionCarrierStreams::legacy_mutation_failure](../../../supervisor/src/endpoint_carrier/legacy_history.rs#L40); [tekes-supervisor::endpoint_carrier::legacy_history::ProductionCarrierStreams::open_legacy_journal](../../../supervisor/src/endpoint_carrier/legacy_history.rs#L76); [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../../supervisor/src/endpoint_host.rs#L1378); [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_session_gate](../../../supervisor/src/endpoint_host.rs#L315); [tekes-supervisor::file_leases::SessionFileTransfer::prepare](../../../supervisor/src/file_leases.rs#L39); [tekes-supervisor::file_observation::SessionFileObservations::open](../../../supervisor/src/file_observation.rs#L26); [tekes-supervisor::process_host::ProductionProcessHost::build_inventory](../../../supervisor/src/process_host.rs#L5395); [tekes-supervisor::process_host::ProcessToolRuntime::archive_session](../../../supervisor/src/process_host.rs#L5803); [tekes-supervisor::process_host::ProductionProcessHost::session_scope](../../../supervisor/src/process_host.rs#L984); [tekes-supervisor::production_tool_control::validate_session_folder](../../../supervisor/src/production_tool_control.rs#L1169); [tekes-supervisor::production_tool_control::session_folder](../../../supervisor/src/production_tool_control.rs#L1189); [user-documents::feedback::validate_session](../../../user-documents/src/feedback.rs#L40) |
