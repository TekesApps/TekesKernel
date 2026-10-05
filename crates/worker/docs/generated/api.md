# tekes-worker — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [tekes-worker::identity::selected](../../src/identity.rs#L7) | `pub(super)` | [tekes-worker::identity::resolve](../../src/identity.rs#L41) |
| [tekes-worker::identity::resolve](../../src/identity.rs#L41) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks](../../src/lifecycle_hooks.rs#L9) | `pub(super)` | not a function |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::load](../../src/lifecycle_hooks.rs#L20) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::before_turn](../../src/lifecycle_hooks.rs#L150) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::prepare](../../src/lifecycle_hooks.rs#L170) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::before_compact](../../src/lifecycle_hooks.rs#L184) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::observe](../../src/lifecycle_hooks.rs#L201) | `pub(super)` | no resolved direct caller |
| [tekes-worker::main::exchange_tool_control](../../src/main.rs#L1256) | `pub` | no resolved direct caller |
| [tekes-worker::result_presentation::present](../../src/result_presentation.rs#L13) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::validation_runtime::JUDGE_SYSTEM](../../src/validation_runtime.rs#L6) | `pub(super)` | not a function |
| [tekes-worker::validation_runtime::validator_workspace_paths](../../src/validation_runtime.rs#L8) | `pub(super)` | [tekes-worker::validation_runtime::ensure_validator](../../src/validation_runtime.rs#L1108); [tekes-worker::validation_runtime::activate_validator_profile](../../src/validation_runtime.rs#L135) |
| [tekes-worker::validation_runtime::has_pending_validation](../../src/validation_runtime.rs#L107) | `pub(super)` | no resolved direct caller |
| [tekes-worker::validation_runtime::activate_validator_profile](../../src/validation_runtime.rs#L135) | `pub(super)` | no resolved direct caller |
| [tekes-worker::validation_runtime::isolate_validator_profile](../../src/validation_runtime.rs#L204) | `pub(super)` | [tekes-worker::validation_runtime::activate_validator_profile](../../src/validation_runtime.rs#L135) |
| [tekes-worker::validation_runtime::result_value](../../src/validation_runtime.rs#L349) | `pub(super)` | [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719); [tekes-worker::validation_runtime::judge](../../src/validation_runtime.rs#L998) |
| [tekes-worker::validation_runtime::snapshot](../../src/validation_runtime.rs#L477) | `pub(super)` | [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719) |
| [tekes-worker::validation_runtime::artifact_bytes](../../src/validation_runtime.rs#L662) | `pub(super)` | [tekes-worker::validation_runtime::ensure_validator](../../src/validation_runtime.rs#L1108); [tekes-worker::validation_runtime::judge](../../src/validation_runtime.rs#L998) |
| [tekes-worker::validation_runtime::validator_mandate_exhausted](../../src/validation_runtime.rs#L705) | `pub(super)` | [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719) |
| [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719) | `pub(super)` | no resolved direct caller |
| [tekes-worker::workspace_edits::WorkspaceEditRecorder](../../src/workspace_edits.rs#L30) | `pub` | not a function |
