//! Slice-4 tool hook, helper, sandbox, and write-ahead execution contracts.

mod builtin;
mod guidance;
mod helper;
mod hook;
mod lifecycle_hook;
pub use lifecycle_hook::*;
#[cfg(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
mod linux_sandbox;
mod pipeline;
mod runtime_backends;
mod sandbox;
mod schema_registry;

pub use builtin::{
    Availability, Backend, BuiltinError, BuiltinManifest, BuiltinTool, CatalogContext, CatalogRole,
    Effect, index_by_name,
};
pub use guidance::{
    CODING_PROFILE, GENERAL_PROFILE, HARNESS_IDENTITY, IdentityProfile, WORKING_DIRECTORY,
    canonical_guidance_oracle_bytes, guidance_oracle_value, root_system_instructions,
};
pub use helper::{
    ByteString, CreateMode, EX_PROTOCOL, ExecRequest, ExecValue, HelperClient, HelperClientError,
    HelperError, HelperErrorClass, HelperOperation, HelperPath, HelperRequest, HelperResponse,
    HelperServer, HelperValue, ReadValue, RootBinding, WriteValue, decode_helper_line,
    encode_helper_line, serve_stdio,
};
pub use hook::{
    HookBinding, HookFailureMode, HookPhase, HookRequest, HookResponse, HookResultView,
    PostVerdict, PreVerdict, ProcessHook, decode_hook_binding, decode_hook_request,
    decode_hook_response, encode_hook_line,
};
pub use pipeline::{
    ApprovalGate, BackendTerminal, DurableApprovalResponse, PipelineDecision, SecretScan,
    SecretScanner, TOOL_CONTINUATION_SUBKIND, ToolExecution, ToolPipeline, ToolPipelineError,
};
pub use runtime_backends::*;
pub use sandbox::{
    NetworkPolicy, ProbeFailure, ProbeStatus, SandboxApproval, SandboxBackend, SandboxError,
    SandboxPolicy, compile_darwin_profile, compile_linux_plan, policy_digest, probe_backend,
    validate_unsandboxed_approval,
};
pub use schema_registry::{
    FIXED_SCHEMA_REVISION, FixedToolSchema, ObjectRule, ObjectSchema, PropertySchema,
    SchemaValidationError, ValueSchema, canonical_schema_oracle_bytes, fixed_schema,
    fixed_schema_registry, schema_oracle_value, validate_fixed_arguments,
};
