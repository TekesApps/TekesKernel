# tools

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `lifecycle_hook::*` | `pub` |
| `Availability` | `builtin::Availability` | `pub` |
| `Backend` | `builtin::Backend` | `pub` |
| `BuiltinError` | `builtin::BuiltinError` | `pub` |
| `BuiltinManifest` | `builtin::BuiltinManifest` | `pub` |
| `BuiltinTool` | `builtin::BuiltinTool` | `pub` |
| `CatalogContext` | `builtin::CatalogContext` | `pub` |
| `CatalogRole` | `builtin::CatalogRole` | `pub` |
| `Effect` | `builtin::Effect` | `pub` |
| `index_by_name` | `builtin::index_by_name` | `pub` |
| `CODING_PROFILE` | `guidance::CODING_PROFILE` | `pub` |
| `GENERAL_PROFILE` | `guidance::GENERAL_PROFILE` | `pub` |
| `HARNESS_IDENTITY` | `guidance::HARNESS_IDENTITY` | `pub` |
| `IdentityProfile` | `guidance::IdentityProfile` | `pub` |
| `WORKING_DIRECTORY` | `guidance::WORKING_DIRECTORY` | `pub` |
| `canonical_guidance_oracle_bytes` | `guidance::canonical_guidance_oracle_bytes` | `pub` |
| `guidance_oracle_value` | `guidance::guidance_oracle_value` | `pub` |
| `root_system_instructions` | `guidance::root_system_instructions` | `pub` |
| `ByteString` | `helper::ByteString` | `pub` |
| `CreateMode` | `helper::CreateMode` | `pub` |
| `EX_PROTOCOL` | `helper::EX_PROTOCOL` | `pub` |
| `ExecRequest` | `helper::ExecRequest` | `pub` |
| `ExecValue` | `helper::ExecValue` | `pub` |
| `HelperClient` | `helper::HelperClient` | `pub` |
| `HelperClientError` | `helper::HelperClientError` | `pub` |
| `HelperError` | `helper::HelperError` | `pub` |
| `HelperErrorClass` | `helper::HelperErrorClass` | `pub` |
| `HelperOperation` | `helper::HelperOperation` | `pub` |
| `HelperPath` | `helper::HelperPath` | `pub` |
| `HelperRequest` | `helper::HelperRequest` | `pub` |
| `HelperResponse` | `helper::HelperResponse` | `pub` |
| `HelperServer` | `helper::HelperServer` | `pub` |
| `HelperValue` | `helper::HelperValue` | `pub` |
| `ReadValue` | `helper::ReadValue` | `pub` |
| `RootBinding` | `helper::RootBinding` | `pub` |
| `WriteValue` | `helper::WriteValue` | `pub` |
| `decode_helper_line` | `helper::decode_helper_line` | `pub` |
| `encode_helper_line` | `helper::encode_helper_line` | `pub` |
| `serve_stdio` | `helper::serve_stdio` | `pub` |
| `HookBinding` | `hook::HookBinding` | `pub` |
| `HookFailureMode` | `hook::HookFailureMode` | `pub` |
| `HookPhase` | `hook::HookPhase` | `pub` |
| `HookRequest` | `hook::HookRequest` | `pub` |
| `HookResponse` | `hook::HookResponse` | `pub` |
| `HookResultView` | `hook::HookResultView` | `pub` |
| `PostVerdict` | `hook::PostVerdict` | `pub` |
| `PreVerdict` | `hook::PreVerdict` | `pub` |
| `ProcessHook` | `hook::ProcessHook` | `pub` |
| `decode_hook_binding` | `hook::decode_hook_binding` | `pub` |
| `decode_hook_request` | `hook::decode_hook_request` | `pub` |
| `decode_hook_response` | `hook::decode_hook_response` | `pub` |
| `encode_hook_line` | `hook::encode_hook_line` | `pub` |
| `ApprovalGate` | `pipeline::ApprovalGate` | `pub` |
| `BackendTerminal` | `pipeline::BackendTerminal` | `pub` |
| `DurableApprovalResponse` | `pipeline::DurableApprovalResponse` | `pub` |
| `PipelineDecision` | `pipeline::PipelineDecision` | `pub` |
| `SecretScan` | `pipeline::SecretScan` | `pub` |
| `SecretScanner` | `pipeline::SecretScanner` | `pub` |
| `TOOL_CONTINUATION_SUBKIND` | `pipeline::TOOL_CONTINUATION_SUBKIND` | `pub` |
| `ToolExecution` | `pipeline::ToolExecution` | `pub` |
| `ToolPipeline` | `pipeline::ToolPipeline` | `pub` |
| `ToolPipelineError` | `pipeline::ToolPipelineError` | `pub` |
| `*` | `runtime_backends::*` | `pub` |
| `NetworkPolicy` | `sandbox::NetworkPolicy` | `pub` |
| `ProbeFailure` | `sandbox::ProbeFailure` | `pub` |
| `ProbeStatus` | `sandbox::ProbeStatus` | `pub` |
| `SandboxApproval` | `sandbox::SandboxApproval` | `pub` |
| `SandboxBackend` | `sandbox::SandboxBackend` | `pub` |
| `SandboxError` | `sandbox::SandboxError` | `pub` |
| `SandboxPolicy` | `sandbox::SandboxPolicy` | `pub` |
| `compile_darwin_profile` | `sandbox::compile_darwin_profile` | `pub` |
| `compile_linux_plan` | `sandbox::compile_linux_plan` | `pub` |
| `policy_digest` | `sandbox::policy_digest` | `pub` |
| `probe_backend` | `sandbox::probe_backend` | `pub` |
| `validate_unsandboxed_approval` | `sandbox::validate_unsandboxed_approval` | `pub` |
| `FIXED_SCHEMA_REVISION` | `schema_registry::FIXED_SCHEMA_REVISION` | `pub` |
| `FixedToolSchema` | `schema_registry::FixedToolSchema` | `pub` |
| `ObjectRule` | `schema_registry::ObjectRule` | `pub` |
| `ObjectSchema` | `schema_registry::ObjectSchema` | `pub` |
| `PropertySchema` | `schema_registry::PropertySchema` | `pub` |
| `SchemaValidationError` | `schema_registry::SchemaValidationError` | `pub` |
| `ValueSchema` | `schema_registry::ValueSchema` | `pub` |
| `canonical_schema_oracle_bytes` | `schema_registry::canonical_schema_oracle_bytes` | `pub` |
| `fixed_schema` | `schema_registry::fixed_schema` | `pub` |
| `fixed_schema_registry` | `schema_registry::fixed_schema_registry` | `pub` |
| `schema_oracle_value` | `schema_registry::schema_oracle_value` | `pub` |
| `validate_fixed_arguments` | `schema_registry::validate_fixed_arguments` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tools::builtin` | `private` |  |
| `tools::guidance` | `private` |  |
| `tools::helper` | `private` |  |
| `tools::hook` | `private` |  |
| `tools::lifecycle_hook` | `private` |  |
| `tools::pipeline` | `private` |  |
| `tools::runtime_backends` | `private` |  |
| `tools::sandbox` | `private` |  |
| `tools::schema_registry` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
