# endpoint

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `AllSessionMux` | `all_session::AllSessionMux` | `pub` |
| `AllSessionMuxError` | `all_session::AllSessionMuxError` | `pub` |
| `AllSessionMuxHandle` | `all_session::AllSessionMuxHandle` | `pub` |
| `ApprovalOption` | `approval_policy::ApprovalOption` | `pub` |
| `ApprovalPolicy` | `approval_policy::ApprovalPolicy` | `pub` |
| `PERMISSION_MODE_DANGER_FULL_ACCESS` | `approval_policy::PERMISSION_MODE_DANGER_FULL_ACCESS` | `pub` |
| `PERMISSION_MODE_IDS` | `approval_policy::PERMISSION_MODE_IDS` | `pub` |
| `PERMISSION_MODE_READ_ONLY` | `approval_policy::PERMISSION_MODE_READ_ONLY` | `pub` |
| `PERMISSION_MODE_WORKSPACE_WRITE` | `approval_policy::PERMISSION_MODE_WORKSPACE_WRITE` | `pub` |
| `ServerApprovalState` | `approval_policy::ServerApprovalState` | `pub` |
| `AttachmentAuthority` | `attachment::AttachmentAuthority` | `pub` |
| `AttachmentErrorReason` | `attachment::AttachmentErrorReason` | `pub` |
| `AttachmentPolicy` | `attachment::AttachmentPolicy` | `pub` |
| `AttachmentReadError` | `attachment::AttachmentReadError` | `pub` |
| `DEFAULT_FILE_MEDIA_TYPE` | `attachment::DEFAULT_FILE_MEDIA_TYPE` | `pub` |
| `FileAttachment` | `attachment::FileAttachment` | `pub` |
| `FileAttachmentRef` | `attachment::FileAttachmentRef` | `pub` |
| `ImageAttachment` | `attachment::ImageAttachment` | `pub` |
| `ImageAttachmentRef` | `attachment::ImageAttachmentRef` | `pub` |
| `ImageMediaType` | `attachment::ImageMediaType` | `pub` |
| `MaterializedPrompt` | `attachment::MaterializedPrompt` | `pub` |
| `PromptMaterializeError` | `attachment::PromptMaterializeError` | `pub` |
| `PromptPart` | `attachment::PromptPart` | `pub` |
| `UPLOADS_DIR` | `attachment::UPLOADS_DIR` | `pub` |
| `UploadReceipt` | `attachment::UploadReceipt` | `pub` |
| `UploadedFile` | `attachment::UploadedFile` | `pub` |
| `project_content_blocks` | `attachment::project_content_blocks` | `pub` |
| `CarrierRespondHandler` | `carrier_adapter::CarrierRespondHandler` | `pub` |
| `CarrierStreamHandler` | `carrier_adapter::CarrierStreamHandler` | `pub` |
| `ComposedEndpointCarrierHost` | `carrier_adapter::ComposedEndpointCarrierHost` | `pub` |
| `JournalRespondHandler` | `carrier_adapter::JournalRespondHandler` | `pub` |
| `LocatedRespond` | `carrier_adapter::LocatedRespond` | `pub` |
| `RespondAuthorReceipt` | `carrier_adapter::RespondAuthorReceipt` | `pub` |
| `RespondAuthority` | `carrier_adapter::RespondAuthority` | `pub` |
| `RespondDelivery` | `carrier_adapter::RespondDelivery` | `pub` |
| `SESSION_ENDPOINT_PUBLIC_METHODS` | `carrier_adapter::SESSION_ENDPOINT_PUBLIC_METHODS` | `pub` |
| `HistoryError` | `history::HistoryError` | `pub` |
| `HistoryPage` | `history::HistoryPage` | `pub` |
| `history_page` | `history::history_page` | `pub` |
| `CallContext` | `host::CallContext` | `pub` |
| `CarrierHostFuture` | `host::CarrierHostFuture` | `pub` |
| `DrainSignal` | `host::DrainSignal` | `pub` |
| `DurableHandoffError` | `host::DurableHandoffError` | `pub` |
| `DurableHandoffProof` | `host::DurableHandoffProof` | `pub` |
| `DurableHandoffSignal` | `host::DurableHandoffSignal` | `pub` |
| `EndpointCarrierHost` | `host::EndpointCarrierHost` | `pub` |
| `EndpointDispatchError` | `host::EndpointDispatchError` | `pub` |
| `EndpointDispatcher` | `host::EndpointDispatcher` | `pub` |
| `EndpointHost` | `host::EndpointHost` | `pub` |
| `EndpointHostCall` | `host::EndpointHostCall` | `pub` |
| `EndpointHostFuture` | `host::EndpointHostFuture` | `pub` |
| `EndpointStream` | `host::EndpointStream` | `pub` |
| `EndpointStreamReceiver` | `host::EndpointStreamReceiver` | `pub` |
| `HostFailure` | `host::HostFailure` | `pub` |
| `HostReadiness` | `host::HostReadiness` | `pub` |
| `MethodClass` | `host::MethodClass` | `pub` |
| `SessionHostDescription` | `host::SessionHostDescription` | `pub` |
| `SessionStream` | `host::SessionStream` | `pub` |
| `SessionStreamReceiver` | `host::SessionStreamReceiver` | `pub` |
| `StreamChannel` | `host::StreamChannel` | `pub` |
| `StreamErrorCode` | `host::StreamErrorCode` | `pub` |
| `StreamFailure` | `host::StreamFailure` | `pub` |
| `DEFAULT_SUBSCRIPTION_MAX_BYTES` | `hub::DEFAULT_SUBSCRIPTION_MAX_BYTES` | `pub` |
| `DEFAULT_SUBSCRIPTION_MAX_FRAMES` | `hub::DEFAULT_SUBSCRIPTION_MAX_FRAMES` | `pub` |
| `EndpointSubscription` | `hub::EndpointSubscription` | `pub` |
| `EndpointSubscriptionHub` | `hub::EndpointSubscriptionHub` | `pub` |
| `HostFrame` | `hub::HostFrame` | `pub` |
| `HostFrameKind` | `hub::HostFrameKind` | `pub` |
| `HubError` | `hub::HubError` | `pub` |
| `MAX_MUX_SESSIONS` | `hub::MAX_MUX_SESSIONS` | `pub` |
| `MuxFrame` | `hub::MuxFrame` | `pub` |
| `MuxRegistration` | `hub::MuxRegistration` | `pub` |
| `MuxReplayRegistration` | `hub::MuxReplayRegistration` | `pub` |
| `SubscriptionBaseline` | `hub::SubscriptionBaseline` | `pub` |
| `SubscriptionPoll` | `hub::SubscriptionPoll` | `pub` |
| `RpcBegin` | `idempotency::RpcBegin` | `pub` |
| `RpcClaim` | `idempotency::RpcClaim` | `pub` |
| `RpcDurableIdentity` | `idempotency::RpcDurableIdentity` | `pub` |
| `RpcLookup` | `idempotency::RpcLookup` | `pub` |
| `RpcRegistry` | `idempotency::RpcRegistry` | `pub` |
| `RpcRegistryError` | `idempotency::RpcRegistryError` | `pub` |
| `EndpointJournal` | `journal::EndpointJournal` | `pub` |
| `JOURNAL_FILE` | `journal::JOURNAL_FILE` | `pub` |
| `JournalError` | `journal::JournalError` | `pub` |
| `JournalRecord` | `journal::JournalRecord` | `pub` |
| `LOCK_FILE` | `journal::LOCK_FILE` | `pub` |
| `ForkSessionOperation` | `management::ForkSessionOperation` | `pub` |
| `LEGACY_ORIGIN_CLIENT` | `management::LEGACY_ORIGIN_CLIENT` | `pub` |
| `ManagementError` | `management::ManagementError` | `pub` |
| `ManagementStore` | `management::ManagementStore` | `pub` |
| `ORIGIN_CLIENT` | `management::ORIGIN_CLIENT` | `pub` |
| `PendingQueueTransaction` | `management::PendingQueueTransaction` | `pub` |
| `QueueRecoveryDriver` | `management::QueueRecoveryDriver` | `pub` |
| `QueueTransactionCompletion` | `management::QueueTransactionCompletion` | `pub` |
| `QueueTransactionDecision` | `management::QueueTransactionDecision` | `pub` |
| `QueueTransactionOperation` | `management::QueueTransactionOperation` | `pub` |
| `QueueTransactionState` | `management::QueueTransactionState` | `pub` |
| `SelectModelOperation` | `management::SelectModelOperation` | `pub` |
| `SelectedModel` | `management::SelectedModel` | `pub` |
| `SessionCreateOperation` | `management::SessionCreateOperation` | `pub` |
| `WorkspaceList` | `management::WorkspaceList` | `pub` |
| `WorkspaceView` | `management::WorkspaceView` | `pub` |
| `is_endpoint_origin_client` | `management::is_endpoint_origin_client` | `pub` |
| `ActionableRegistry` | `mux::ActionableRegistry` | `pub` |
| `DEFAULT_JOURNAL_WINDOW_MESSAGES` | `mux::DEFAULT_JOURNAL_WINDOW_MESSAGES` | `pub` |
| `JournalFollow` | `mux::JournalFollow` | `pub` |
| `MAX_JOURNAL_WINDOW_MESSAGES` | `mux::MAX_JOURNAL_WINDOW_MESSAGES` | `pub` |
| `MuxHostDescription` | `mux::MuxHostDescription` | `pub` |
| `MuxHostProduct` | `mux::MuxHostProduct` | `pub` |
| `MuxProtocolError` | `mux::MuxProtocolError` | `pub` |
| `Retryability` | `mux::Retryability` | `pub` |
| `SESSION_ENDPOINT_PROTOCOL_VERSION` | `mux::SESSION_ENDPOINT_PROTOCOL_VERSION` | `pub` |
| `SessionActionable` | `mux::SessionActionable` | `pub` |
| `SessionActionableKind` | `mux::SessionActionableKind` | `pub` |
| `SessionAddress` | `mux::SessionAddress` | `pub` |
| `SessionControlItem` | `mux::SessionControlItem` | `pub` |
| `SessionEndpointCapability` | `mux::SessionEndpointCapability` | `pub` |
| `SessionErrorCategory` | `mux::SessionErrorCategory` | `pub` |
| `SessionJournalPage` | `mux::SessionJournalPage` | `pub` |
| `SessionJournalSnapshot` | `mux::SessionJournalSnapshot` | `pub` |
| `SessionMuxClientFrame` | `mux::SessionMuxClientFrame` | `pub` |
| `SessionMuxGeneration` | `mux::SessionMuxGeneration` | `pub` |
| `SessionMuxServerFrame` | `mux::SessionMuxServerFrame` | `pub` |
| `SessionRemoteError` | `mux::SessionRemoteError` | `pub` |
| `SessionStreamTarget` | `mux::SessionStreamTarget` | `pub` |
| `SessionSummary` | `mux::SessionSummary` | `pub` |
| `SessionSyncFrame` | `mux::SessionSyncFrame` | `pub` |
| `WorkspaceBaseline` | `mux::WorkspaceBaseline` | `pub` |
| `WorkspaceSummary` | `mux::WorkspaceSummary` | `pub` |
| `frozen_history_page` | `mux::frozen_history_page` | `pub` |
| `open_journal_follow` | `mux::open_journal_follow` | `pub` |
| `AcceptedStreamFrame` | `projection::AcceptedStreamFrame` | `pub` |
| `ProjectionError` | `projection::ProjectionError` | `pub` |
| `Projector` | `projection::Projector` | `pub` |
| `PendingRequest` | `requests::PendingRequest` | `pub` |
| `RequestError` | `requests::RequestError` | `pub` |
| `RequestFrameType` | `requests::RequestFrameType` | `pub` |
| `RequestState` | `requests::RequestState` | `pub` |
| `ResolutionOutcome` | `requests::ResolutionOutcome` | `pub` |
| `derive_line_request_rpc_id` | `requests::derive_line_request_rpc_id` | `pub` |
| `derive_request_rpc_id` | `requests::derive_request_rpc_id` | `pub` |
| `RespondAuthorization` | `respond::RespondAuthorization` | `pub` |
| `RespondDecision` | `respond::RespondDecision` | `pub` |
| `RespondLifecycle` | `respond::RespondLifecycle` | `pub` |
| `RespondLifecycleError` | `respond::RespondLifecycleError` | `pub` |
| `RespondPrepareContext` | `respond::RespondPrepareContext` | `pub` |
| `RespondRejectionReason` | `respond::RespondRejectionReason` | `pub` |
| `ClientRequest` | `rpc::ClientRequest` | `pub` |
| `ClientResponse` | `rpc::ClientResponse` | `pub` |
| `ClientResponseResult` | `rpc::ClientResponseResult` | `pub` |
| `MuxBuffer` | `rpc::MuxBuffer` | `pub` |
| `MuxBufferError` | `rpc::MuxBufferError` | `pub` |
| `RespondReceipt` | `rpc::RespondReceipt` | `pub` |
| `RpcError` | `rpc::RpcError` | `pub` |
| `RpcResult` | `rpc::RpcResult` | `pub` |
| `ServerRequest` | `rpc::ServerRequest` | `pub` |
| `ServerResponse` | `rpc::ServerResponse` | `pub` |
| `validate_request` | `rpc::validate_request` | `pub` |
| `validate_response` | `rpc::validate_response` | `pub` |
| `validate_rpc_id` | `rpc::validate_rpc_id` | `pub` |
| `AUTOMATIC_TITLE_REFINE_OPERATION` | `service::AUTOMATIC_TITLE_REFINE_OPERATION` | `pub` |
| `AUTOMATIC_TITLE_SEED_OPERATION` | `service::AUTOMATIC_TITLE_SEED_OPERATION` | `pub` |
| `ArchiveResult` | `service::ArchiveResult` | `pub` |
| `MutationReceipt` | `service::MutationReceipt` | `pub` |
| `NativeEndpoint` | `service::NativeEndpoint` | `pub` |
| `NativeEndpointError` | `service::NativeEndpointError` | `pub` |
| `SESSION_NOTICE_OPERATION` | `service::SESSION_NOTICE_OPERATION` | `pub` |
| `SessionInventoryItem` | `service::SessionInventoryItem` | `pub` |
| `SessionNotice` | `service::SessionNotice` | `pub` |
| `SessionNoticeSeverity` | `service::SessionNoticeSeverity` | `pub` |
| `UnarchiveResult` | `service::UnarchiveResult` | `pub` |
| `StitchDecision` | `stitch::StitchDecision` | `pub` |
| `StitchError` | `stitch::StitchError` | `pub` |
| `stitch_window` | `stitch::stitch_window` | `pub` |
| `EndpointFrameQueue` | `stream_queue::EndpointFrameQueue` | `pub` |
| `FrameQueueError` | `stream_queue::FrameQueueError` | `pub` |
| `EndpointTypeError` | `types::EndpointTypeError` | `pub` |
| `SessionEvent` | `types::SessionEvent` | `pub` |
| `SessionHistoryEntry` | `types::SessionHistoryEntry` | `pub` |
| `SessionToolEventView` | `types::SessionToolEventView` | `pub` |
| `SurfaceOperation` | `types::SurfaceOperation` | `pub` |
| `validate_session_id` | `types::validate_session_id` | `pub` |
| `client_mux_frames` | `client_wire::client_mux_frames` | `pub` |
| `client_sync_frames` | `client_wire::client_sync_frames` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `endpoint::all_session` | `private` |  |
| `endpoint::approval_policy` | `private` |  |
| `endpoint::attachment` | `private` |  |
| `endpoint::carrier_adapter` | `private` |  |
| `endpoint::history` | `private` |  |
| `endpoint::host` | `private` |  |
| `endpoint::hub` | `private` |  |
| `endpoint::idempotency` | `private` |  |
| `endpoint::journal` | `private` |  |
| `endpoint::management` | `private` |  |
| `endpoint::mux` | `private` |  |
| `endpoint::projection` | `private` |  |
| `endpoint::requests` | `private` |  |
| `endpoint::respond` | `private` |  |
| `endpoint::rpc` | `private` |  |
| `endpoint::service` | `private` |  |
| `endpoint::session_event_registry_generated` | `private` |  |
| `endpoint::stitch` | `private` |  |
| `endpoint::stream_queue` | `private` |  |
| `endpoint::types` | `private` |  |
| `endpoint::client_wire` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
