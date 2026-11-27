# tools::sandbox

[Package atlas](index.md) · [Source](../../src/sandbox.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tools::sandbox::LINUX_LANDLOCK_ABI_MIN](../../src/sandbox.rs#L13) | const_item | `pub(crate)` |  |
| [tools::sandbox::ExecutionAccess](../../src/sandbox.rs#L18) | enum_item | `pub` |  |
| [tools::sandbox::ExecutionAccess::policy](../../src/sandbox.rs#L27) | function_item | `pub` |  |
| [tools::sandbox::NetworkPolicy](../../src/sandbox.rs#L45) | enum_item | `pub` |  |
| [tools::sandbox::SandboxPolicy](../../src/sandbox.rs#L53) | struct_item | `pub` |  |
| [tools::sandbox::SandboxPolicy::validate](../../src/sandbox.rs#L71) | function_item | `pub` |  |
| [tools::sandbox::SandboxPolicy::is_protected_exception](../../src/sandbox.rs#L98) | function_item | `pub` |  |
| [tools::sandbox::SandboxPolicy::exposed_protected_root](../../src/sandbox.rs#L107) | function_item | `pub` |  |
| [tools::sandbox::SandboxPolicy::canonical_bytes](../../src/sandbox.rs#L117) | function_item | `pub` |  |
| [tools::sandbox::SandboxBackend](../../src/sandbox.rs#L126) | enum_item | `pub` |  |
| [tools::sandbox::ProbeStatus](../../src/sandbox.rs#L136) | enum_item | `pub` |  |
| [tools::sandbox::ProbeFailure](../../src/sandbox.rs#L143) | enum_item | `pub` |  |
| [tools::sandbox::SandboxApproval](../../src/sandbox.rs#L151) | struct_item | `pub` |  |
| [tools::sandbox::SandboxError](../../src/sandbox.rs#L159) | enum_item | `pub` |  |
| [tools::sandbox::kernel_protected_roots](../../src/sandbox.rs#L179) | function_item | `pub` |  |
| [tools::sandbox::kernel_protected_roots_with_user_agent_dir](../../src/sandbox.rs#L200) | function_item | `pub` |  |
| [tools::sandbox::exposed_protected_roots](../../src/sandbox.rs#L215) | function_item | `pub` |  |
| [tools::sandbox::protected_root_containing](../../src/sandbox.rs#L241) | function_item | `pub` |  |
| [tools::sandbox::canonical_text](../../src/sandbox.rs#L272) | function_item | `private` |  |
| [tools::sandbox::service_user_home](../../src/sandbox.rs#L296) | function_item | `private` |  |
| [tools::sandbox::policy_digest](../../src/sandbox.rs#L323) | function_item | `pub` |  |
| [tools::sandbox::compile_darwin_profile](../../src/sandbox.rs#L330) | function_item | `pub` |  |
| [tools::sandbox::compile_linux_plan](../../src/sandbox.rs#L443) | function_item | `pub` |  |
| [tools::sandbox::compile_linux_bubblewrap_plan](../../src/sandbox.rs#L476) | function_item | `pub` |  |
| [tools::sandbox::validate_unsandboxed_approval](../../src/sandbox.rs#L518) | function_item | `pub` |  |
| [tools::sandbox::probe_backend](../../src/sandbox.rs#L535) | function_item | `pub` |  |
| [tools::sandbox::sandbox_command](../../src/sandbox.rs#L544) | function_item | `pub(crate)` |  |
| [tools::sandbox::probe_darwin](../../src/sandbox.rs#L599) | function_item | `private` |  |
| [tools::sandbox::select_linux_backend](../../src/sandbox.rs#L622) | function_item | `private` |  |
| [tools::sandbox::probe_bubblewrap](../../src/sandbox.rs#L642) | function_item | `private` |  |
| [tools::sandbox::probe_linux](../../src/sandbox.rs#L685) | function_item | `private` |  |
| [tools::sandbox::probe_confinement](../../src/sandbox.rs#L720) | function_item | `private` | #[cfg(any(     target_os = "macos",     all(         target_os = "linux",         any(target_arch = "x86_64", target_arch = "aarch64")     ) ))] |
| [tools::sandbox::probe_output](../../src/sandbox.rs#L798) | function_item | `private` | #[cfg(any(     target_os = "macos",     all(         target_os = "linux",         any(target_arch = "x86_64", target_arch = "aarch64")     ) ))] |
| [tools::sandbox::validate_roots](../../src/sandbox.rs#L832) | function_item | `private` |  |
| [tools::sandbox::validate_absolute](../../src/sandbox.rs#L849) | function_item | `private` |  |
| [tools::sandbox::escape_sbpl](../../src/sandbox.rs#L865) | function_item | `private` |  |
| [tools::sandbox::probe_nonce](../../src/sandbox.rs#L881) | function_item | `private` | #[cfg(any(     target_os = "macos",     all(         target_os = "linux",         any(target_arch = "x86_64", target_arch = "aarch64")     ) ))] |
| [tools::sandbox::probe_nonce::NONCE](../../src/sandbox.rs#L883) | static_item | `private` | #[cfg(any(     target_os = "macos",     all(         target_os = "linux",         any(target_arch = "x86_64", target_arch = "aarch64")     ) ))] |
| [tools::sandbox::tests::available](../../src/sandbox.rs#L890) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::missing](../../src/sandbox.rs#L896) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::probe_output_reaps_completed_child_and_preserves_both_streams](../../src/sandbox.rs#L910) | function_item | `private` | test; #[cfg(test)] #[cfg(any(         target_os = "macos",         all(             target_os = "linux",             any(target_arch = "x86_64", target_arch = "aarch64")         )     ))] |
| [tools::sandbox::tests::linux_auto_prefers_bubblewrap_without_probing_landlock](../../src/sandbox.rs#L920) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::linux_auto_falls_back_only_to_available_landlock](../../src/sandbox.rs#L929) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::bubblewrap_plan_matches_golden_bytes](../../src/sandbox.rs#L940) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::protected_policy](../../src/sandbox.rs#L951) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::protected_roots_are_omitted_from_serialization_when_empty](../../src/sandbox.rs#L964) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::darwin_profile_denies_protected_root_and_its_writable_ancestors](../../src/sandbox.rs#L979) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::protected_exceptions_must_be_strictly_beneath](../../src/sandbox.rs#L1012) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::unsorted_protected_roots_are_rejected](../../src/sandbox.rs#L1027) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::exposed_roots_keep_only_protected_roots_inside_write_roots](../../src/sandbox.rs#L1036) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::protected_root_lookup_resolves_symlinked_aliases](../../src/sandbox.rs#L1050) | function_item | `private` | test; #[cfg(test)] |
| [tools::sandbox::tests::user_instructions_stay_protected_through_absent_paths_and_aliases](../../src/sandbox.rs#L1076) | function_item | `private` | test; #[cfg(test)] #[cfg(target_os = "macos")] |
| [tools::sandbox::tests::seatbelt_keeps_protected_root_read_only_beneath_a_writable_home](../../src/sandbox.rs#L1136) | function_item | `private` | test; #[cfg(test)] #[cfg(target_os = "macos")] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `fs` | `std::fs` | `private` |
| `Path` | `std::path::Path` | `private` |
| `Command` | `std::process::Command` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tools::sandbox::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 17 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::bubblewrap_sandbox::command"]
  n1["tools::linux_sandbox::confine"]
  n2["tools::sandbox::SandboxPolicy::exposed_protected_root"]
  n3["tools::sandbox::SandboxPolicy::canonical_bytes"]
  n4["tools::sandbox::kernel_protected_roots"]
  n5["tools::sandbox::kernel_protected_roots_with_user_agent_dir"]
  n6["tools::sandbox::exposed_protected_roots"]
  n7["tools::sandbox::protected_root_containing"]
  n8["tools::sandbox::canonical_text"]
  n9["tools::sandbox::ExecutionAccess::policy"]
  n10["tools::sandbox::service_user_home"]
  n11["tools::sandbox::policy_digest"]
  n12["tools::sandbox::compile_darwin_profile"]
  n13["tools::sandbox::compile_linux_plan"]
  n14["tools::sandbox::compile_linux_bubblewrap_plan"]
  n15["tools::sandbox::validate_unsandboxed_approval"]
  n16["tools::sandbox::probe_backend"]
  n17["tools::sandbox::sandbox_command"]
  n18["tools::sandbox::probe_darwin"]
  n19["tools::sandbox::select_linux_backend"]
  n20["tools::sandbox::probe_bubblewrap"]
  n21["tools::sandbox::probe_linux"]
  n22["tools::sandbox::SandboxPolicy::validate"]
  n23["tools::sandbox::probe_confinement"]
  n24["tools::sandbox::validate_roots"]
  n25["tools::sandbox::validate_absolute"]
  n26["tools::sandbox::SandboxPolicy::is_protected_exception"]
  n2 --> n26
  n3 --> n22
  n4 --> n8
  n4 --> n10
  n5 --> n4
  n5 --> n8
  n6 --> n8
  n16 --> n18
  n16 --> n19
  n16 --> n20
  n16 --> n21
  n17 --> n0
  n17 --> n1
  n17 --> n12
  n18 --> n23
  n22 --> n24
  n22 --> n25
```

</details>

<details><summary>Functions 21–28: 8 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::bubblewrap_sandbox::binary"]
  n1["tools::linux_sandbox::supported_abi"]
  n2["tools::sandbox::sandbox_command"]
  n3["tools::sandbox::probe_bubblewrap"]
  n4["tools::sandbox::probe_linux"]
  n5["tools::sandbox::probe_confinement"]
  n6["tools::sandbox::probe_output"]
  n7["tools::sandbox::validate_roots"]
  n8["tools::sandbox::validate_absolute"]
  n9["tools::sandbox::escape_sbpl"]
  n10["tools::sandbox::probe_nonce"]
  n3 --> n0
  n3 --> n5
  n3 --> n6
  n4 --> n1
  n4 --> n5
  n5 --> n2
  n5 --> n6
  n7 --> n8
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `policy` | `Vec::new` | [37](../../src/sandbox.rs#L37) | external-constructor-callback-or-unresolved |
| `validate` | `Err` | [73](../../src/sandbox.rs#L73), [87](../../src/sandbox.rs#L87) | external-constructor-callback-or-unresolved |
| `validate` | `SandboxError::Invalid` | [73](../../src/sandbox.rs#L73), [87](../../src/sandbox.rs#L87) | external-constructor-callback-or-unresolved |
| `validate` | `validate_roots` | [75](../../src/sandbox.rs#L75), [76](../../src/sandbox.rs#L76), [77](../../src/sandbox.rs#L77) | [tools::sandbox::validate_roots](../../src/sandbox.rs#L832) |
| `validate` | `validate_absolute` | [79](../../src/sandbox.rs#L79) | [tools::sandbox::validate_absolute](../../src/sandbox.rs#L849) |
| `validate` | `self                 .read_roots                 .iter()                 .any` | [82](../../src/sandbox.rs#L82) | receiver-type-required |
| `validate` | `self                 .read_roots                 .iter` | [82](../../src/sandbox.rs#L82) | receiver-type-required |
| `validate` | `Path::new(root).starts_with` | [85](../../src/sandbox.rs#L85) | receiver-type-required |
| `validate` | `Path::new` | [85](../../src/sandbox.rs#L85) | external-constructor-callback-or-unresolved |
| `validate` | `Ok` | [92](../../src/sandbox.rs#L92) | external-constructor-callback-or-unresolved |
| `is_protected_exception` | `self.protected_roots             .iter()             .any` | [99](../../src/sandbox.rs#L99) | receiver-type-required |
| `is_protected_exception` | `self.protected_roots             .iter` | [99](../../src/sandbox.rs#L99) | receiver-type-required |
| `is_protected_exception` | `Path::new` | [101](../../src/sandbox.rs#L101) | external-constructor-callback-or-unresolved |
| `is_protected_exception` | `root.starts_with` | [101](../../src/sandbox.rs#L101) | receiver-type-required |
| `exposed_protected_root` | `self.is_protected_exception` | [108](../../src/sandbox.rs#L108) | [tools::sandbox::SandboxPolicy::is_protected_exception](../../src/sandbox.rs#L98) |
| `exposed_protected_root` | `self.protected_roots             .iter()             .find(&#124;protected&#124; Path::new(protected).starts_with(root))             .map` | [111](../../src/sandbox.rs#L111) | receiver-type-required |
| `exposed_protected_root` | `self.protected_roots             .iter()             .find` | [111](../../src/sandbox.rs#L111) | receiver-type-required |
| `exposed_protected_root` | `self.protected_roots             .iter` | [111](../../src/sandbox.rs#L111) | receiver-type-required |
| `exposed_protected_root` | `Path::new(protected).starts_with` | [113](../../src/sandbox.rs#L113) | receiver-type-required |
| `exposed_protected_root` | `Path::new` | [113](../../src/sandbox.rs#L113) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `self.validate` | [118](../../src/sandbox.rs#L118) | [tools::sandbox::SandboxPolicy::validate](../../src/sandbox.rs#L71) |
| `canonical_bytes` | `serde_json_canonicalizer::to_vec(self)             .map_err` | [119](../../src/sandbox.rs#L119) | receiver-type-required |
| `canonical_bytes` | `serde_json_canonicalizer::to_vec` | [119](../../src/sandbox.rs#L119) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `SandboxError::Encoding` | [120](../../src/sandbox.rs#L120) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `error.to_string` | [120](../../src/sandbox.rs#L120) | receiver-type-required |
| `kernel_protected_roots` | `Vec::new` | [180](../../src/sandbox.rs#L180) | external-constructor-callback-or-unresolved |
| `kernel_protected_roots` | `roots.push` | [181](../../src/sandbox.rs#L181), [183](../../src/sandbox.rs#L183), [189](../../src/sandbox.rs#L189) | receiver-type-required |
| `kernel_protected_roots` | `canonical_text` | [181](../../src/sandbox.rs#L181), [183](../../src/sandbox.rs#L183), [189](../../src/sandbox.rs#L189) | [tools::sandbox::canonical_text](../../src/sandbox.rs#L272) |
| `kernel_protected_roots` | `service_user_home` | [182](../../src/sandbox.rs#L182) | [tools::sandbox::service_user_home](../../src/sandbox.rs#L296) |
| `kernel_protected_roots` | `home.join` | [183](../../src/sandbox.rs#L183), [185](../../src/sandbox.rs#L185), [187](../../src/sandbox.rs#L187) | receiver-type-required |
| `kernel_protected_roots` | `roots.sort` | [192](../../src/sandbox.rs#L192) | receiver-type-required |
| `kernel_protected_roots` | `roots.dedup` | [193](../../src/sandbox.rs#L193) | receiver-type-required |
| `kernel_protected_roots_with_user_agent_dir` | `kernel_protected_roots` | [204](../../src/sandbox.rs#L204) | [tools::sandbox::kernel_protected_roots](../../src/sandbox.rs#L179) |
| `kernel_protected_roots_with_user_agent_dir` | `roots.push` | [205](../../src/sandbox.rs#L205) | receiver-type-required |
| `kernel_protected_roots_with_user_agent_dir` | `canonical_text` | [205](../../src/sandbox.rs#L205) | [tools::sandbox::canonical_text](../../src/sandbox.rs#L272) |
| `kernel_protected_roots_with_user_agent_dir` | `roots.sort` | [206](../../src/sandbox.rs#L206) | receiver-type-required |
| `kernel_protected_roots_with_user_agent_dir` | `roots.dedup` | [207](../../src/sandbox.rs#L207) | receiver-type-required |
| `exposed_protected_roots` | `write_roots         .iter()         .flat_map(&#124;root&#124; {             let canonical = canonical_text(Path::new(root));             [root.clone(), canonical]         })         .collect::<Vec<_>>` | [216](../../src/sandbox.rs#L216) | receiver-type-required |
| `exposed_protected_roots` | `write_roots         .iter()         .flat_map` | [216](../../src/sandbox.rs#L216) | receiver-type-required |
| `exposed_protected_roots` | `write_roots         .iter` | [216](../../src/sandbox.rs#L216) | receiver-type-required |
| `exposed_protected_roots` | `canonical_text` | [219](../../src/sandbox.rs#L219) | [tools::sandbox::canonical_text](../../src/sandbox.rs#L272) |
| `exposed_protected_roots` | `Path::new` | [219](../../src/sandbox.rs#L219), [228](../../src/sandbox.rs#L228) | external-constructor-callback-or-unresolved |
| `exposed_protected_roots` | `root.clone` | [220](../../src/sandbox.rs#L220) | receiver-type-required |
| `exposed_protected_roots` | `protected         .iter()         .filter(&#124;root&#124; {             writes                 .iter()                 .any(&#124;write&#124; Path::new(root).starts_with(write))         })         .cloned()         .collect::<Vec<_>>` | [223](../../src/sandbox.rs#L223) | receiver-type-required |
| `exposed_protected_roots` | `protected         .iter()         .filter(&#124;root&#124; {             writes                 .iter()                 .any(&#124;write&#124; Path::new(root).starts_with(write))         })         .cloned` | [223](../../src/sandbox.rs#L223) | receiver-type-required |
| `exposed_protected_roots` | `protected         .iter()         .filter` | [223](../../src/sandbox.rs#L223) | receiver-type-required |
| `exposed_protected_roots` | `protected         .iter` | [223](../../src/sandbox.rs#L223) | receiver-type-required |
| `exposed_protected_roots` | `writes                 .iter()                 .any` | [226](../../src/sandbox.rs#L226) | receiver-type-required |
| `exposed_protected_roots` | `writes                 .iter` | [226](../../src/sandbox.rs#L226) | receiver-type-required |
| `exposed_protected_roots` | `Path::new(root).starts_with` | [228](../../src/sandbox.rs#L228) | receiver-type-required |
| `exposed_protected_roots` | `exposed.sort` | [232](../../src/sandbox.rs#L232) | receiver-type-required |
| `exposed_protected_roots` | `exposed.dedup` | [233](../../src/sandbox.rs#L233) | receiver-type-required |
| `protected_root_containing` | `Vec::new` | [243](../../src/sandbox.rs#L243) | external-constructor-callback-or-unresolved |
| `protected_root_containing` | `existing.canonicalize` | [245](../../src/sandbox.rs#L245) | receiver-type-required |
| `protected_root_containing` | `Some` | [246](../../src/sandbox.rs#L246) | external-constructor-callback-or-unresolved |
| `protected_root_containing` | `suffix                     .iter()                     .rev()                     .fold` | [247](../../src/sandbox.rs#L247) | receiver-type-required |
| `protected_root_containing` | `suffix                     .iter()                     .rev` | [247](../../src/sandbox.rs#L247) | receiver-type-required |
| `protected_root_containing` | `suffix                     .iter` | [247](../../src/sandbox.rs#L247) | receiver-type-required |
| `protected_root_containing` | `base.join` | [250](../../src/sandbox.rs#L250) | receiver-type-required |
| `protected_root_containing` | `existing.parent` | [253](../../src/sandbox.rs#L253) | receiver-type-required |
| `protected_root_containing` | `existing.file_name` | [253](../../src/sandbox.rs#L253) | receiver-type-required |
| `protected_root_containing` | `suffix.push` | [255](../../src/sandbox.rs#L255) | receiver-type-required |
| `protected_root_containing` | `name.to_owned` | [255](../../src/sandbox.rs#L255) | receiver-type-required |
| `protected_root_containing` | `protected         .iter()         .find(&#124;root&#124; {             path.starts_with(root)                 &#124;&#124; resolved                     .as_deref()                     .is_some_and(&#124;resolved&#124; resolved.starts_with(root))         })         .map` | [261](../../src/sandbox.rs#L261) | receiver-type-required |
| `protected_root_containing` | `protected         .iter()         .find` | [261](../../src/sandbox.rs#L261) | receiver-type-required |
| `protected_root_containing` | `protected         .iter` | [261](../../src/sandbox.rs#L261) | receiver-type-required |
| `protected_root_containing` | `path.starts_with` | [264](../../src/sandbox.rs#L264) | receiver-type-required |
| `protected_root_containing` | `resolved                     .as_deref()                     .is_some_and` | [265](../../src/sandbox.rs#L265) | receiver-type-required |
| `protected_root_containing` | `resolved                     .as_deref` | [265](../../src/sandbox.rs#L265) | receiver-type-required |
| `protected_root_containing` | `resolved.starts_with` | [267](../../src/sandbox.rs#L267) | receiver-type-required |
| `canonical_text` | `Vec::new` | [274](../../src/sandbox.rs#L274) | external-constructor-callback-or-unresolved |
| `canonical_text` | `existing.canonicalize` | [276](../../src/sandbox.rs#L276) | receiver-type-required |
| `canonical_text` | `suffix                 .iter()                 .rev()                 .fold(canonical, &#124;base, part&#124; base.join(part))                 .to_string_lossy()                 .into_owned` | [277](../../src/sandbox.rs#L277) | receiver-type-required |
| `canonical_text` | `suffix                 .iter()                 .rev()                 .fold(canonical, &#124;base, part&#124; base.join(part))                 .to_string_lossy` | [277](../../src/sandbox.rs#L277) | receiver-type-required |
| `canonical_text` | `suffix                 .iter()                 .rev()                 .fold` | [277](../../src/sandbox.rs#L277) | receiver-type-required |
| `canonical_text` | `suffix                 .iter()                 .rev` | [277](../../src/sandbox.rs#L277) | receiver-type-required |
| `canonical_text` | `suffix                 .iter` | [277](../../src/sandbox.rs#L277) | receiver-type-required |
| `canonical_text` | `base.join` | [280](../../src/sandbox.rs#L280) | receiver-type-required |
| `canonical_text` | `existing.parent` | [284](../../src/sandbox.rs#L284) | receiver-type-required |
| `canonical_text` | `existing.file_name` | [284](../../src/sandbox.rs#L284) | receiver-type-required |
| `canonical_text` | `suffix.push` | [286](../../src/sandbox.rs#L286) | receiver-type-required |
| `canonical_text` | `name.to_owned` | [286](../../src/sandbox.rs#L286) | receiver-type-required |
| `canonical_text` | `path.to_string_lossy().into_owned` | [289](../../src/sandbox.rs#L289) | receiver-type-required |
| `canonical_text` | `path.to_string_lossy` | [289](../../src/sandbox.rs#L289) | receiver-type-required |
| `service_user_home` | `std::mem::zeroed` | [301](../../src/sandbox.rs#L301) | external-constructor-callback-or-unresolved |
| `service_user_home` | `std::ptr::null_mut` | [302](../../src/sandbox.rs#L302) | external-constructor-callback-or-unresolved |
| `service_user_home` | `libc::getpwuid_r` | [306](../../src/sandbox.rs#L306) | external-constructor-callback-or-unresolved |
| `service_user_home` | `libc::geteuid` | [307](../../src/sandbox.rs#L307) | external-constructor-callback-or-unresolved |
| `service_user_home` | `buffer.as_mut_ptr().cast` | [309](../../src/sandbox.rs#L309) | receiver-type-required |
| `service_user_home` | `buffer.as_mut_ptr` | [309](../../src/sandbox.rs#L309) | receiver-type-required |
| `service_user_home` | `buffer.len` | [310](../../src/sandbox.rs#L310) | receiver-type-required |
| `service_user_home` | `result.is_null` | [314](../../src/sandbox.rs#L314) | receiver-type-required |
| `service_user_home` | `entry.pw_dir.is_null` | [314](../../src/sandbox.rs#L314) | receiver-type-required |
| `service_user_home` | `CStr::from_ptr` | [318](../../src/sandbox.rs#L318) | external-constructor-callback-or-unresolved |
| `service_user_home` | `Path::new` | [319](../../src/sandbox.rs#L319) | external-constructor-callback-or-unresolved |
| `service_user_home` | `std::ffi::OsStr::from_bytes` | [319](../../src/sandbox.rs#L319) | external-constructor-callback-or-unresolved |
| `service_user_home` | `home.to_bytes` | [319](../../src/sandbox.rs#L319) | receiver-type-required |
| `service_user_home` | `home.is_absolute().then` | [320](../../src/sandbox.rs#L320) | receiver-type-required |
| `service_user_home` | `home.is_absolute` | [320](../../src/sandbox.rs#L320) | receiver-type-required |
| `service_user_home` | `home.to_path_buf` | [320](../../src/sandbox.rs#L320) | receiver-type-required |
| `policy_digest` | `Ok` | [324](../../src/sandbox.rs#L324) | external-constructor-callback-or-unresolved |
| `compile_darwin_profile` | `policy.validate` | [331](../../src/sandbox.rs#L331) | receiver-type-required |
| `compile_darwin_profile` | `policy         .read_roots         .iter()         .chain(&policy.write_roots)         .chain(policy.scratch.iter())         .flat_map(&#124;root&#124; Path::new(root).ancestors().skip(1))         .filter_map(Path::to_str)         .collect::<BTreeSet<_>>` | [340](../../src/sandbox.rs#L340) | receiver-type-required |
| `compile_darwin_profile` | `policy         .read_roots         .iter()         .chain(&policy.write_roots)         .chain(policy.scratch.iter())         .flat_map(&#124;root&#124; Path::new(root).ancestors().skip(1))         .filter_map` | [340](../../src/sandbox.rs#L340) | receiver-type-required |
| `compile_darwin_profile` | `policy         .read_roots         .iter()         .chain(&policy.write_roots)         .chain(policy.scratch.iter())         .flat_map` | [340](../../src/sandbox.rs#L340) | receiver-type-required |
| `compile_darwin_profile` | `policy         .read_roots         .iter()         .chain(&policy.write_roots)         .chain` | [340](../../src/sandbox.rs#L340) | receiver-type-required |
| `compile_darwin_profile` | `policy         .read_roots         .iter()         .chain` | [340](../../src/sandbox.rs#L340) | receiver-type-required |
| `compile_darwin_profile` | `policy         .read_roots         .iter` | [340](../../src/sandbox.rs#L340) | receiver-type-required |
| `compile_darwin_profile` | `policy.scratch.iter` | [344](../../src/sandbox.rs#L344), [397](../../src/sandbox.rs#L397) | receiver-type-required |
| `compile_darwin_profile` | `Path::new(root).ancestors().skip` | [345](../../src/sandbox.rs#L345) | receiver-type-required |
| `compile_darwin_profile` | `Path::new(root).ancestors` | [345](../../src/sandbox.rs#L345) | receiver-type-required |
| `compile_darwin_profile` | `Path::new` | [345](../../src/sandbox.rs#L345), [405](../../src/sandbox.rs#L405), [419](../../src/sandbox.rs#L419) | external-constructor-callback-or-unresolved |
| `compile_darwin_profile` | `lines.push` | [349](../../src/sandbox.rs#L349), [357](../../src/sandbox.rs#L357), [358](../../src/sandbox.rs#L358), [372](../../src/sandbox.rs#L372), [378](../../src/sandbox.rs#L378), [384](../../src/sandbox.rs#L384), [401](../../src/sandbox.rs#L401), [412](../../src/sandbox.rs#L412), [421](../../src/sandbox.rs#L421), [427](../../src/sandbox.rs#L427), [428](../../src/sandbox.rs#L428), [433](../../src/sandbox.rs#L433), [434](../../src/sandbox.rs#L434), [436](../../src/sandbox.rs#L436) | receiver-type-required |
| `compile_darwin_profile` | `"(allow file-read-metadata (literal \"/Applications\"))".to_owned` | [357](../../src/sandbox.rs#L357) | receiver-type-required |
| `compile_darwin_profile` | `"(allow file-read-metadata (literal \"/Library\"))".to_owned` | [358](../../src/sandbox.rs#L358) | receiver-type-required |
| `compile_darwin_profile` | `[         "/System",         "/usr",         "/bin",         "/sbin",         "/private/var/select",         "/Applications/Xcode.app",         "/Library/Developer/CommandLineTools",     ]     .into_iter()     .map(ToOwned::to_owned)     .chain` | [359](../../src/sandbox.rs#L359) | receiver-type-required |
| `compile_darwin_profile` | `[         "/System",         "/usr",         "/bin",         "/sbin",         "/private/var/select",         "/Applications/Xcode.app",         "/Library/Developer/CommandLineTools",     ]     .into_iter()     .map` | [359](../../src/sandbox.rs#L359) | receiver-type-required |
| `compile_darwin_profile` | `[         "/System",         "/usr",         "/bin",         "/sbin",         "/private/var/select",         "/Applications/Xcode.app",         "/Library/Developer/CommandLineTools",     ]     .into_iter` | [359](../../src/sandbox.rs#L359) | receiver-type-required |
| `compile_darwin_profile` | `policy.read_roots.iter().cloned` | [370](../../src/sandbox.rs#L370) | receiver-type-required |
| `compile_darwin_profile` | `policy.read_roots.iter` | [370](../../src/sandbox.rs#L370) | receiver-type-required |
| `compile_darwin_profile` | `policy         .write_roots         .iter()         .chain(policy.scratch.iter())         .collect::<Vec<_>>` | [394](../../src/sandbox.rs#L394) | receiver-type-required |
| `compile_darwin_profile` | `policy         .write_roots         .iter()         .chain` | [394](../../src/sandbox.rs#L394) | receiver-type-required |
| `compile_darwin_profile` | `policy         .write_roots         .iter` | [394](../../src/sandbox.rs#L394) | receiver-type-required |
| `compile_darwin_profile` | `BTreeSet::new` | [399](../../src/sandbox.rs#L399) | external-constructor-callback-or-unresolved |
| `compile_darwin_profile` | `Path::new(protected).ancestors().skip` | [405](../../src/sandbox.rs#L405) | receiver-type-required |
| `compile_darwin_profile` | `Path::new(protected).ancestors` | [405](../../src/sandbox.rs#L405) | receiver-type-required |
| `compile_darwin_profile` | `grants.iter().any` | [406](../../src/sandbox.rs#L406) | receiver-type-required |
| `compile_darwin_profile` | `grants.iter` | [406](../../src/sandbox.rs#L406) | receiver-type-required |
| `compile_darwin_profile` | `ancestor.starts_with` | [406](../../src/sandbox.rs#L406) | receiver-type-required |
| `compile_darwin_profile` | `denied_ancestors.extend` | [407](../../src/sandbox.rs#L407) | receiver-type-required |
| `compile_darwin_profile` | `ancestor.to_str` | [407](../../src/sandbox.rs#L407) | receiver-type-required |
| `compile_darwin_profile` | `grants         .iter()         .filter` | [417](../../src/sandbox.rs#L417) | receiver-type-required |
| `compile_darwin_profile` | `grants         .iter` | [417](../../src/sandbox.rs#L417) | receiver-type-required |
| `compile_darwin_profile` | `policy.is_protected_exception` | [419](../../src/sandbox.rs#L419) | receiver-type-required |
| `compile_darwin_profile` | `"(allow process-exec process-fork)".to_owned` | [427](../../src/sandbox.rs#L427) | receiver-type-required |
| `compile_darwin_profile` | `"(allow signal (target same-sandbox))".to_owned` | [428](../../src/sandbox.rs#L428) | receiver-type-required |
| `compile_darwin_profile` | `"(allow network* (local ip \"localhost:*\"))".to_owned` | [433](../../src/sandbox.rs#L433) | receiver-type-required |
| `compile_darwin_profile` | `"(allow network* (remote ip \"localhost:*\"))".to_owned` | [434](../../src/sandbox.rs#L434) | receiver-type-required |
| `compile_darwin_profile` | `"(allow network*)".to_owned` | [436](../../src/sandbox.rs#L436) | receiver-type-required |
| `compile_darwin_profile` | `lines.join("\n").into_bytes` | [438](../../src/sandbox.rs#L438) | receiver-type-required |
| `compile_darwin_profile` | `lines.join` | [438](../../src/sandbox.rs#L438) | receiver-type-required |
| `compile_darwin_profile` | `bytes.push` | [439](../../src/sandbox.rs#L439) | receiver-type-required |
| `compile_darwin_profile` | `Ok` | [440](../../src/sandbox.rs#L440) | external-constructor-callback-or-unresolved |
| `compile_linux_plan` | `policy.validate` | [444](../../src/sandbox.rs#L444) | receiver-type-required |
| `compile_linux_plan` | `plan.as_object_mut()             .expect("plan object")             .insert` | [459](../../src/sandbox.rs#L459), [464](../../src/sandbox.rs#L464) | receiver-type-required |
| `compile_linux_plan` | `plan.as_object_mut()             .expect` | [459](../../src/sandbox.rs#L459), [464](../../src/sandbox.rs#L464) | receiver-type-required |
| `compile_linux_plan` | `plan.as_object_mut` | [459](../../src/sandbox.rs#L459), [464](../../src/sandbox.rs#L464) | receiver-type-required |
| `compile_linux_plan` | `"scratch".to_owned` | [461](../../src/sandbox.rs#L461) | receiver-type-required |
| `compile_linux_plan` | `policy.protected_roots.is_empty` | [463](../../src/sandbox.rs#L463) | receiver-type-required |
| `compile_linux_plan` | `"protected_roots".to_owned` | [466](../../src/sandbox.rs#L466) | receiver-type-required |
| `compile_linux_plan` | `serde_json_canonicalizer::to_vec(&plan)         .map_err` | [468](../../src/sandbox.rs#L468) | receiver-type-required |
| `compile_linux_plan` | `serde_json_canonicalizer::to_vec` | [468](../../src/sandbox.rs#L468) | external-constructor-callback-or-unresolved |
| `compile_linux_plan` | `SandboxError::Encoding` | [469](../../src/sandbox.rs#L469) | external-constructor-callback-or-unresolved |
| `compile_linux_plan` | `error.to_string` | [469](../../src/sandbox.rs#L469) | receiver-type-required |
| `compile_linux_plan` | `bytes.push` | [470](../../src/sandbox.rs#L470) | receiver-type-required |
| `compile_linux_plan` | `Ok` | [471](../../src/sandbox.rs#L471) | external-constructor-callback-or-unresolved |
| `compile_linux_bubblewrap_plan` | `policy.validate` | [477](../../src/sandbox.rs#L477) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `Err` | [479](../../src/sandbox.rs#L479) | external-constructor-callback-or-unresolved |
| `compile_linux_bubblewrap_plan` | `SandboxError::Unavailable` | [479](../../src/sandbox.rs#L479) | external-constructor-callback-or-unresolved |
| `compile_linux_bubblewrap_plan` | `"bubblewrap requires allow_process=true and network=deny or all".to_owned` | [480](../../src/sandbox.rs#L480) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `namespaces.push` | [485](../../src/sandbox.rs#L485) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `namespaces.sort_unstable` | [486](../../src/sandbox.rs#L486) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `plan.as_object_mut()             .expect("plan object")             .insert` | [503](../../src/sandbox.rs#L503), [508](../../src/sandbox.rs#L508) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `plan.as_object_mut()             .expect` | [503](../../src/sandbox.rs#L503), [508](../../src/sandbox.rs#L508) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `plan.as_object_mut` | [503](../../src/sandbox.rs#L503), [508](../../src/sandbox.rs#L508) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `"scratch".to_owned` | [505](../../src/sandbox.rs#L505) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `policy.protected_roots.is_empty` | [507](../../src/sandbox.rs#L507) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `"protected_roots".to_owned` | [510](../../src/sandbox.rs#L510) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `serde_json_canonicalizer::to_vec(&plan)         .map_err` | [512](../../src/sandbox.rs#L512) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `serde_json_canonicalizer::to_vec` | [512](../../src/sandbox.rs#L512) | external-constructor-callback-or-unresolved |
| `compile_linux_bubblewrap_plan` | `SandboxError::Encoding` | [513](../../src/sandbox.rs#L513) | external-constructor-callback-or-unresolved |
| `compile_linux_bubblewrap_plan` | `error.to_string` | [513](../../src/sandbox.rs#L513) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `bytes.push` | [514](../../src/sandbox.rs#L514) | receiver-type-required |
| `compile_linux_bubblewrap_plan` | `Ok` | [515](../../src/sandbox.rs#L515) | external-constructor-callback-or-unresolved |
| `validate_unsandboxed_approval` | `Ok` | [529](../../src/sandbox.rs#L529) | external-constructor-callback-or-unresolved |
| `validate_unsandboxed_approval` | `Err` | [531](../../src/sandbox.rs#L531) | external-constructor-callback-or-unresolved |
| `probe_backend` | `probe_darwin` | [537](../../src/sandbox.rs#L537) | [tools::sandbox::probe_darwin](../../src/sandbox.rs#L599) |
| `probe_backend` | `select_linux_backend` | [538](../../src/sandbox.rs#L538) | [tools::sandbox::select_linux_backend](../../src/sandbox.rs#L622) |
| `probe_backend` | `probe_bubblewrap` | [538](../../src/sandbox.rs#L538), [539](../../src/sandbox.rs#L539) | [tools::sandbox::probe_bubblewrap](../../src/sandbox.rs#L642) |
| `probe_backend` | `probe_linux` | [540](../../src/sandbox.rs#L540) | [tools::sandbox::probe_linux](../../src/sandbox.rs#L685) |
| `sandbox_command` | `Err` | [550](../../src/sandbox.rs#L550), [557](../../src/sandbox.rs#L557), [576](../../src/sandbox.rs#L576), [593](../../src/sandbox.rs#L593) | external-constructor-callback-or-unresolved |
| `sandbox_command` | `SandboxError::Unavailable` | [550](../../src/sandbox.rs#L550), [557](../../src/sandbox.rs#L557), [576](../../src/sandbox.rs#L576), [593](../../src/sandbox.rs#L593) | external-constructor-callback-or-unresolved |
| `sandbox_command` | `"sandbox probe did not report available".to_owned` | [551](../../src/sandbox.rs#L551) | receiver-type-required |
| `sandbox_command` | `String::from_utf8(compile_darwin_profile(policy)?)             .map_err` | [561](../../src/sandbox.rs#L561) | receiver-type-required |
| `sandbox_command` | `String::from_utf8` | [561](../../src/sandbox.rs#L561) | external-constructor-callback-or-unresolved |
| `sandbox_command` | `compile_darwin_profile` | [561](../../src/sandbox.rs#L561) | [tools::sandbox::compile_darwin_profile](../../src/sandbox.rs#L330) |
| `sandbox_command` | `SandboxError::Encoding` | [562](../../src/sandbox.rs#L562) | external-constructor-callback-or-unresolved |
| `sandbox_command` | `"profile is not UTF-8".to_owned` | [562](../../src/sandbox.rs#L562) | receiver-type-required |
| `sandbox_command` | `Command::new` | [563](../../src/sandbox.rs#L563), [580](../../src/sandbox.rs#L580) | external-constructor-callback-or-unresolved |
| `sandbox_command` | `command.arg("-p").arg(profile).arg` | [564](../../src/sandbox.rs#L564) | receiver-type-required |
| `sandbox_command` | `command.arg("-p").arg` | [564](../../src/sandbox.rs#L564) | receiver-type-required |
| `sandbox_command` | `command.arg` | [564](../../src/sandbox.rs#L564) | receiver-type-required |
| `sandbox_command` | `Ok` | [565](../../src/sandbox.rs#L565), [582](../../src/sandbox.rs#L582) | external-constructor-callback-or-unresolved |
| `sandbox_command` | `crate::bubblewrap_sandbox::command` | [573](../../src/sandbox.rs#L573) | [tools::bubblewrap_sandbox::command](../../src/bubblewrap_sandbox.rs#L57) |
| `sandbox_command` | `crate::linux_sandbox::confine` | [581](../../src/sandbox.rs#L581) | [tools::linux_sandbox::confine](../../src/linux_sandbox.rs#L125) |
| `sandbox_command` | `"no applied sandbox launcher on this platform".to_owned` | [594](../../src/sandbox.rs#L594) | receiver-type-required |
| `probe_darwin` | `"Seatbelt requested on a non-macOS host".to_owned` | [604](../../src/sandbox.rs#L604) | receiver-type-required |
| `probe_darwin` | `Path::new("/usr/bin/sandbox-exec").is_file` | [609](../../src/sandbox.rs#L609) | receiver-type-required |
| `probe_darwin` | `Path::new` | [609](../../src/sandbox.rs#L609) | external-constructor-callback-or-unresolved |
| `probe_darwin` | `"sandbox-exec is missing".to_owned` | [612](../../src/sandbox.rs#L612) | receiver-type-required |
| `probe_darwin` | `probe_confinement` | [615](../../src/sandbox.rs#L615) | [tools::sandbox::probe_confinement](../../src/sandbox.rs#L720) |
| `probe_darwin` | `"darwin-seatbelt".to_owned` | [616](../../src/sandbox.rs#L616) | receiver-type-required |
| `probe_darwin` | `std::env::consts::OS.to_owned` | [617](../../src/sandbox.rs#L617) | receiver-type-required |
| `select_linux_backend` | `landlock` | [629](../../src/sandbox.rs#L629) | external-constructor-callback-or-unresolved |
| `probe_bubblewrap` | `"bubblewrap/seccomp requires Linux on x86_64 or aarch64".to_owned` | [650](../../src/sandbox.rs#L650) | receiver-type-required |
| `probe_bubblewrap` | `crate::bubblewrap_sandbox::binary` | [658](../../src/sandbox.rs#L658) | [tools::bubblewrap_sandbox::binary](../../src/bubblewrap_sandbox.rs#L26) |
| `probe_bubblewrap` | `error.to_string` | [663](../../src/sandbox.rs#L663) | receiver-type-required |
| `probe_bubblewrap` | `probe_output` | [667](../../src/sandbox.rs#L667) | [tools::sandbox::probe_output](../../src/sandbox.rs#L798) |
| `probe_bubblewrap` | `Command::new(binary).arg` | [667](../../src/sandbox.rs#L667) | receiver-type-required |
| `probe_bubblewrap` | `Command::new` | [667](../../src/sandbox.rs#L667) | external-constructor-callback-or-unresolved |
| `probe_bubblewrap` | `output.status.success` | [668](../../src/sandbox.rs#L668) | receiver-type-required |
| `probe_bubblewrap` | `String::from_utf8_lossy(&output.stdout).trim().to_owned` | [669](../../src/sandbox.rs#L669) | receiver-type-required |
| `probe_bubblewrap` | `String::from_utf8_lossy(&output.stdout).trim` | [669](../../src/sandbox.rs#L669) | receiver-type-required |
| `probe_bubblewrap` | `String::from_utf8_lossy` | [669](../../src/sandbox.rs#L669) | external-constructor-callback-or-unresolved |
| `probe_bubblewrap` | `probe_confinement` | [678](../../src/sandbox.rs#L678) | [tools::sandbox::probe_confinement](../../src/sandbox.rs#L720) |
| `probe_bubblewrap` | `crate::bubblewrap_sandbox::BACKEND.to_owned` | [679](../../src/sandbox.rs#L679) | receiver-type-required |
| `probe_linux` | `"Landlock/seccomp requires Linux on x86_64 or aarch64".to_owned` | [693](../../src/sandbox.rs#L693) | receiver-type-required |
| `probe_linux` | `crate::linux_sandbox::supported_abi` | [701](../../src/sandbox.rs#L701) | [tools::linux_sandbox::supported_abi](../../src/linux_sandbox.rs#L79) |
| `probe_linux` | `probe_confinement` | [702](../../src/sandbox.rs#L702) | [tools::sandbox::probe_confinement](../../src/sandbox.rs#L720) |
| `probe_linux` | `crate::linux_sandbox::BACKEND.to_owned` | [703](../../src/sandbox.rs#L703) | receiver-type-required |
| `probe_confinement` | `std::env::temp_dir().join` | [721](../../src/sandbox.rs#L721) | receiver-type-required |
| `probe_confinement` | `std::env::temp_dir` | [721](../../src/sandbox.rs#L721) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `unresolved_root.join` | [726](../../src/sandbox.rs#L726), [727](../../src/sandbox.rs#L727) | receiver-type-required |
| `probe_confinement` | `(&#124;&#124; -> Result<(), SandboxError> {         fs::create_dir_all(&allowed_unresolved)?;         fs::create_dir_all(&outside_unresolved)?;         let root = fs::canonicalize(&unresolved_root)?;         let allowed = root.join("allowed");         let outside = root.join("outside");         fs::write(allowed.join("read.txt"), b"ok")?;         let policy = SandboxPolicy {             format: 1,             read_roots: vec![allowed.to_string_lossy().into_owned()],             write_roots: vec![allowed.to_string_lossy().into_owned()],             network: NetworkPolicy::Deny,             allow_process: true,             scratch: None,             protected_roots: Vec::new(),         };         let read = probe_output(             sandbox_command(&policy, &candidate, Path::new("/bin/cat"))?                 .current_dir(&allowed)                 .arg(allowed.join("read.txt")),         )?;         let write = probe_output(             sandbox_command(&policy, &candidate, Path::new("/bin/sh"))?                 .current_dir(&allowed)                 .args(["-c", "printf ok > write.txt"]),         )?;         let denied = probe_output(             sandbox_command(&policy, &candidate, Path::new("/bin/sh"))?                 .current_dir(&allowed)                 .args(["-c", "printf denied > \"$1\"", "probe"])                 .arg(outside.join("denied.txt")),         )?;         if read.status.success()             && read.stdout == b"ok"             && write.status.success()             && fs::read(allowed.join("write.txt"))? == b"ok"             && !denied.status.success()             && !outside.join("denied.txt").exists()         {             Ok(())         } else {             Err(SandboxError::Probe(format!(                 "read_status={:?} read_stderr={} denied_status={:?} denied_stderr={} outside_exists={}",                 read.status.code(),                 String::from_utf8_lossy(&read.stderr),                 denied.status.code(),                 String::from_utf8_lossy(&denied.stderr),                 outside.join("denied.txt").exists()             )))         }     })` | [728](../../src/sandbox.rs#L728) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `fs::create_dir_all` | [729](../../src/sandbox.rs#L729), [730](../../src/sandbox.rs#L730) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `fs::canonicalize` | [731](../../src/sandbox.rs#L731) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `root.join` | [732](../../src/sandbox.rs#L732), [733](../../src/sandbox.rs#L733) | receiver-type-required |
| `probe_confinement` | `fs::write` | [734](../../src/sandbox.rs#L734) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `allowed.join` | [734](../../src/sandbox.rs#L734), [747](../../src/sandbox.rs#L747), [763](../../src/sandbox.rs#L763) | receiver-type-required |
| `probe_confinement` | `Vec::new` | [742](../../src/sandbox.rs#L742) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `probe_output` | [744](../../src/sandbox.rs#L744), [749](../../src/sandbox.rs#L749), [754](../../src/sandbox.rs#L754) | [tools::sandbox::probe_output](../../src/sandbox.rs#L798) |
| `probe_confinement` | `sandbox_command(&policy, &candidate, Path::new("/bin/cat"))?                 .current_dir(&allowed)                 .arg` | [745](../../src/sandbox.rs#L745) | receiver-type-required |
| `probe_confinement` | `sandbox_command(&policy, &candidate, Path::new("/bin/cat"))?                 .current_dir` | [745](../../src/sandbox.rs#L745) | receiver-type-required |
| `probe_confinement` | `sandbox_command` | [745](../../src/sandbox.rs#L745), [750](../../src/sandbox.rs#L750), [755](../../src/sandbox.rs#L755) | [tools::sandbox::sandbox_command](../../src/sandbox.rs#L544) |
| `probe_confinement` | `Path::new` | [745](../../src/sandbox.rs#L745), [750](../../src/sandbox.rs#L750), [755](../../src/sandbox.rs#L755) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `sandbox_command(&policy, &candidate, Path::new("/bin/sh"))?                 .current_dir(&allowed)                 .args` | [750](../../src/sandbox.rs#L750), [755](../../src/sandbox.rs#L755) | receiver-type-required |
| `probe_confinement` | `sandbox_command(&policy, &candidate, Path::new("/bin/sh"))?                 .current_dir` | [750](../../src/sandbox.rs#L750), [755](../../src/sandbox.rs#L755) | receiver-type-required |
| `probe_confinement` | `sandbox_command(&policy, &candidate, Path::new("/bin/sh"))?                 .current_dir(&allowed)                 .args(["-c", "printf denied > \"$1\"", "probe"])                 .arg` | [755](../../src/sandbox.rs#L755) | receiver-type-required |
| `probe_confinement` | `outside.join` | [758](../../src/sandbox.rs#L758), [765](../../src/sandbox.rs#L765) | receiver-type-required |
| `probe_confinement` | `read.status.success` | [760](../../src/sandbox.rs#L760) | receiver-type-required |
| `probe_confinement` | `write.status.success` | [762](../../src/sandbox.rs#L762) | receiver-type-required |
| `probe_confinement` | `fs::read` | [763](../../src/sandbox.rs#L763) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `denied.status.success` | [764](../../src/sandbox.rs#L764) | receiver-type-required |
| `probe_confinement` | `outside.join("denied.txt").exists` | [765](../../src/sandbox.rs#L765) | receiver-type-required |
| `probe_confinement` | `Ok` | [767](../../src/sandbox.rs#L767) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `Err` | [769](../../src/sandbox.rs#L769) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `SandboxError::Probe` | [769](../../src/sandbox.rs#L769) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `fs::remove_dir_all` | [779](../../src/sandbox.rs#L779) | external-constructor-callback-or-unresolved |
| `probe_confinement` | `error.to_string` | [784](../../src/sandbox.rs#L784) | receiver-type-required |
| `probe_output` | `command         .process_group(0)         .stdin(Stdio::null())         .stdout(Stdio::piped())         .stderr(Stdio::piped())         .spawn` | [802](../../src/sandbox.rs#L802) | receiver-type-required |
| `probe_output` | `command         .process_group(0)         .stdin(Stdio::null())         .stdout(Stdio::piped())         .stderr` | [802](../../src/sandbox.rs#L802) | receiver-type-required |
| `probe_output` | `command         .process_group(0)         .stdin(Stdio::null())         .stdout` | [802](../../src/sandbox.rs#L802) | receiver-type-required |
| `probe_output` | `command         .process_group(0)         .stdin` | [802](../../src/sandbox.rs#L802) | receiver-type-required |
| `probe_output` | `command         .process_group` | [802](../../src/sandbox.rs#L802) | receiver-type-required |
| `probe_output` | `Stdio::null` | [804](../../src/sandbox.rs#L804) | external-constructor-callback-or-unresolved |
| `probe_output` | `Stdio::piped` | [805](../../src/sandbox.rs#L805), [806](../../src/sandbox.rs#L806) | external-constructor-callback-or-unresolved |
| `probe_output` | `Instant::now` | [808](../../src/sandbox.rs#L808), [810](../../src/sandbox.rs#L810), [825](../../src/sandbox.rs#L825) | external-constructor-callback-or-unresolved |
| `probe_output` | `Duration::from_secs` | [808](../../src/sandbox.rs#L808) | external-constructor-callback-or-unresolved |
| `probe_output` | `child.try_wait()?.is_none` | [809](../../src/sandbox.rs#L809) | receiver-type-required |
| `probe_output` | `child.try_wait` | [809](../../src/sandbox.rs#L809) | receiver-type-required |
| `probe_output` | `libc::kill` | [813](../../src/sandbox.rs#L813) | external-constructor-callback-or-unresolved |
| `probe_output` | `child.id` | [813](../../src/sandbox.rs#L813) | receiver-type-required |
| `probe_output` | `child.kill` | [815](../../src/sandbox.rs#L815) | receiver-type-required |
| `probe_output` | `child.wait` | [816](../../src/sandbox.rs#L816) | receiver-type-required |
| `probe_output` | `Err` | [817](../../src/sandbox.rs#L817) | external-constructor-callback-or-unresolved |
| `probe_output` | `SandboxError::Probe` | [817](../../src/sandbox.rs#L817) | external-constructor-callback-or-unresolved |
| `probe_output` | `"sandbox probe timed out".to_owned` | [817](../../src/sandbox.rs#L817) | receiver-type-required |
| `probe_output` | `std::thread::sleep` | [823](../../src/sandbox.rs#L823) | external-constructor-callback-or-unresolved |
| `probe_output` | `deadline                 .saturating_duration_since(Instant::now())                 .min` | [824](../../src/sandbox.rs#L824) | receiver-type-required |
| `probe_output` | `deadline                 .saturating_duration_since` | [824](../../src/sandbox.rs#L824) | receiver-type-required |
| `probe_output` | `Duration::from_millis` | [826](../../src/sandbox.rs#L826) | external-constructor-callback-or-unresolved |
| `probe_output` | `Ok` | [829](../../src/sandbox.rs#L829) | external-constructor-callback-or-unresolved |
| `probe_output` | `child.wait_with_output` | [829](../../src/sandbox.rs#L829) | receiver-type-required |
| `validate_roots` | `BTreeSet::new` | [834](../../src/sandbox.rs#L834) | external-constructor-callback-or-unresolved |
| `validate_roots` | `validate_absolute` | [836](../../src/sandbox.rs#L836) | [tools::sandbox::validate_absolute](../../src/sandbox.rs#L849) |
| `validate_roots` | `previous.is_some_and` | [837](../../src/sandbox.rs#L837) | receiver-type-required |
| `validate_roots` | `last.as_bytes` | [837](../../src/sandbox.rs#L837) | receiver-type-required |
| `validate_roots` | `value.as_bytes` | [837](../../src/sandbox.rs#L837) | receiver-type-required |
| `validate_roots` | `unique.insert` | [838](../../src/sandbox.rs#L838) | receiver-type-required |
| `validate_roots` | `value.as_str` | [838](../../src/sandbox.rs#L838) | receiver-type-required |
| `validate_roots` | `Err` | [840](../../src/sandbox.rs#L840) | external-constructor-callback-or-unresolved |
| `validate_roots` | `SandboxError::Invalid` | [840](../../src/sandbox.rs#L840) | external-constructor-callback-or-unresolved |
| `validate_roots` | `Some` | [844](../../src/sandbox.rs#L844) | external-constructor-callback-or-unresolved |
| `validate_roots` | `Ok` | [846](../../src/sandbox.rs#L846) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `Path::new` | [850](../../src/sandbox.rs#L850) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `path.is_absolute` | [851](../../src/sandbox.rs#L851) | receiver-type-required |
| `validate_absolute` | `value.contains` | [852](../../src/sandbox.rs#L852) | receiver-type-required |
| `validate_absolute` | `value.chars().any` | [853](../../src/sandbox.rs#L853) | receiver-type-required |
| `validate_absolute` | `value.chars` | [853](../../src/sandbox.rs#L853) | receiver-type-required |
| `validate_absolute` | `path             .components()             .any` | [854](../../src/sandbox.rs#L854) | receiver-type-required |
| `validate_absolute` | `path             .components` | [854](../../src/sandbox.rs#L854) | receiver-type-required |
| `validate_absolute` | `Err` | [858](../../src/sandbox.rs#L858) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `SandboxError::Invalid` | [858](../../src/sandbox.rs#L858) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `Ok` | [862](../../src/sandbox.rs#L862) | external-constructor-callback-or-unresolved |
| `escape_sbpl` | `value.chars().any` | [866](../../src/sandbox.rs#L866) | receiver-type-required |
| `escape_sbpl` | `value.chars` | [866](../../src/sandbox.rs#L866) | receiver-type-required |
| `escape_sbpl` | `Err` | [867](../../src/sandbox.rs#L867) | external-constructor-callback-or-unresolved |
| `escape_sbpl` | `SandboxError::Invalid` | [867](../../src/sandbox.rs#L867) | external-constructor-callback-or-unresolved |
| `escape_sbpl` | `Ok` | [871](../../src/sandbox.rs#L871) | external-constructor-callback-or-unresolved |
| `escape_sbpl` | `value.replace('\\', "\\\\").replace` | [871](../../src/sandbox.rs#L871) | receiver-type-required |
| `escape_sbpl` | `value.replace` | [871](../../src/sandbox.rs#L871) | receiver-type-required |
| `probe_nonce` | `NONCE.fetch_add` | [884](../../src/sandbox.rs#L884) | receiver-type-required |
| `NONCE` | `AtomicU64::new` | [883](../../src/sandbox.rs#L883) | external-constructor-callback-or-unresolved |
| `available` | `backend.to_owned` | [892](../../src/sandbox.rs#L892) | receiver-type-required |
| `available` | `"test".to_owned` | [893](../../src/sandbox.rs#L893) | receiver-type-required |
| `missing` | `"missing".to_owned` | [899](../../src/sandbox.rs#L899) | receiver-type-required |
| `probe_output_reaps_completed_child_and_preserves_both_streams` | `probe_output(             Command::new("/bin/sh").args(["-c", "printf probe-out; printf probe-err >&2"]),         )         .unwrap` | [911](../../src/sandbox.rs#L911) | receiver-type-required |
| `probe_output_reaps_completed_child_and_preserves_both_streams` | `probe_output` | [911](../../src/sandbox.rs#L911) | external-constructor-callback-or-unresolved |
| `probe_output_reaps_completed_child_and_preserves_both_streams` | `Command::new("/bin/sh").args` | [912](../../src/sandbox.rs#L912) | receiver-type-required |
| `probe_output_reaps_completed_child_and_preserves_both_streams` | `Command::new` | [912](../../src/sandbox.rs#L912) | external-constructor-callback-or-unresolved |
| `bubblewrap_plan_matches_golden_bytes` | `serde_json::from_str(include_str!(             "../../../fixtures/sandbox/policy.canonical.json"         ))         .unwrap` | [941](../../src/sandbox.rs#L941) | receiver-type-required |
| `bubblewrap_plan_matches_golden_bytes` | `serde_json::from_str` | [941](../../src/sandbox.rs#L941) | external-constructor-callback-or-unresolved |
| `protected_policy` | `writes.iter().map(&#124;root&#124; (*root).to_owned()).collect` | [955](../../src/sandbox.rs#L955) | receiver-type-required |
| `protected_policy` | `writes.iter().map` | [955](../../src/sandbox.rs#L955) | receiver-type-required |
| `protected_policy` | `writes.iter` | [955](../../src/sandbox.rs#L955) | receiver-type-required |
| `protected_policy` | `(*root).to_owned` | [955](../../src/sandbox.rs#L955) | receiver-type-required |
| `protected_roots_are_omitted_from_serialization_when_empty` | `serde_json::from_str(include_str!(             "../../../fixtures/sandbox/policy.canonical.json"         ))         .unwrap` | [965](../../src/sandbox.rs#L965) | receiver-type-required |
| `protected_roots_are_omitted_from_serialization_when_empty` | `serde_json::from_str` | [965](../../src/sandbox.rs#L965) | external-constructor-callback-or-unresolved |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `protected_policy` | [980](../../src/sandbox.rs#L980) | [tools::sandbox::tests::protected_policy](../../src/sandbox.rs#L951) |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `String::from_utf8(compile_darwin_profile(&policy).unwrap()).unwrap` | [985](../../src/sandbox.rs#L985) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `String::from_utf8` | [985](../../src/sandbox.rs#L985) | external-constructor-callback-or-unresolved |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `compile_darwin_profile(&policy).unwrap` | [985](../../src/sandbox.rs#L985) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `compile_darwin_profile` | [985](../../src/sandbox.rs#L985) | external-constructor-callback-or-unresolved |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `profile.lines().collect::<Vec<_>>` | [986](../../src/sandbox.rs#L986) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `profile.lines` | [986](../../src/sandbox.rs#L986) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `lines                 .iter()                 .position(&#124;candidate&#124; *candidate == line)                 .unwrap_or_else` | [988](../../src/sandbox.rs#L988) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `lines                 .iter()                 .position` | [988](../../src/sandbox.rs#L988) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `lines                 .iter` | [988](../../src/sandbox.rs#L988) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `position` | [993](../../src/sandbox.rs#L993), [994](../../src/sandbox.rs#L994), [995](../../src/sandbox.rs#L995), [996](../../src/sandbox.rs#L996) | external-constructor-callback-or-unresolved |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `lines             .iter()             .rposition(&#124;line&#124; {                 *line == "(allow file-write* (subpath \"/Users/u/Library/Kernel/jobs/j1\"))"             })             .unwrap` | [997](../../src/sandbox.rs#L997) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `lines             .iter()             .rposition` | [997](../../src/sandbox.rs#L997) | receiver-type-required |
| `darwin_profile_denies_protected_root_and_its_writable_ancestors` | `lines             .iter` | [997](../../src/sandbox.rs#L997) | receiver-type-required |
| `protected_exceptions_must_be_strictly_beneath` | `protected_policy` | [1013](../../src/sandbox.rs#L1013) | [tools::sandbox::tests::protected_policy](../../src/sandbox.rs#L951) |
| `unsorted_protected_roots_are_rejected` | `protected_policy` | [1028](../../src/sandbox.rs#L1028) | [tools::sandbox::tests::protected_policy](../../src/sandbox.rs#L951) |
| `protected_root_lookup_resolves_symlinked_aliases` | `tempfile::tempdir().unwrap` | [1051](../../src/sandbox.rs#L1051) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `tempfile::tempdir` | [1051](../../src/sandbox.rs#L1051) | external-constructor-callback-or-unresolved |
| `protected_root_lookup_resolves_symlinked_aliases` | `root.path().canonicalize().unwrap` | [1052](../../src/sandbox.rs#L1052) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `root.path().canonicalize` | [1052](../../src/sandbox.rs#L1052) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `root.path` | [1052](../../src/sandbox.rs#L1052) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `base.join` | [1053](../../src/sandbox.rs#L1053), [1055](../../src/sandbox.rs#L1055), [1061](../../src/sandbox.rs#L1061), [1062](../../src/sandbox.rs#L1062) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `fs::create_dir_all(storage.join("threads")).unwrap` | [1054](../../src/sandbox.rs#L1054) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `fs::create_dir_all` | [1054](../../src/sandbox.rs#L1054) | external-constructor-callback-or-unresolved |
| `protected_root_lookup_resolves_symlinked_aliases` | `storage.join` | [1054](../../src/sandbox.rs#L1054), [1060](../../src/sandbox.rs#L1060) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `std::os::unix::fs::symlink(&storage, base.join("alias")).unwrap` | [1055](../../src/sandbox.rs#L1055) | receiver-type-required |
| `protected_root_lookup_resolves_symlinked_aliases` | `std::os::unix::fs::symlink` | [1055](../../src/sandbox.rs#L1055) | external-constructor-callback-or-unresolved |
| `protected_root_lookup_resolves_symlinked_aliases` | `kernel_protected_roots` | [1056](../../src/sandbox.rs#L1056) | external-constructor-callback-or-unresolved |
| `protected_root_lookup_resolves_symlinked_aliases` | `storage.clone` | [1059](../../src/sandbox.rs#L1059) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `tempfile::tempdir().unwrap` | [1077](../../src/sandbox.rs#L1077) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `tempfile::tempdir` | [1077](../../src/sandbox.rs#L1077) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `root.path().canonicalize().unwrap` | [1078](../../src/sandbox.rs#L1078) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `root.path().canonicalize` | [1078](../../src/sandbox.rs#L1078) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `root.path` | [1078](../../src/sandbox.rs#L1078) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `home.join` | [1079](../../src/sandbox.rs#L1079), [1080](../../src/sandbox.rs#L1080), [1083](../../src/sandbox.rs#L1083), [1084](../../src/sandbox.rs#L1084) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `fs::create_dir_all(&storage).unwrap` | [1081](../../src/sandbox.rs#L1081) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `fs::create_dir_all` | [1081](../../src/sandbox.rs#L1081), [1082](../../src/sandbox.rs#L1082), [1095](../../src/sandbox.rs#L1095) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `fs::create_dir_all(&authored).unwrap` | [1082](../../src/sandbox.rs#L1082) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `std::os::unix::fs::symlink(&authored, home.join("alias")).unwrap` | [1083](../../src/sandbox.rs#L1083) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `std::os::unix::fs::symlink` | [1083](../../src/sandbox.rs#L1083) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `kernel_protected_roots_with_user_agent_dir` | [1085](../../src/sandbox.rs#L1085) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `authored.join` | [1086](../../src/sandbox.rs#L1086) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `fs::create_dir_all(&canonical_user).unwrap` | [1095](../../src/sandbox.rs#L1095) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `fs::write(canonical_user.join("original"), b"original").unwrap` | [1096](../../src/sandbox.rs#L1096) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `fs::write` | [1096](../../src/sandbox.rs#L1096) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `canonical_user.join` | [1096](../../src/sandbox.rs#L1096) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `path.to_string_lossy().into_owned` | [1097](../../src/sandbox.rs#L1097) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `path.to_string_lossy` | [1097](../../src/sandbox.rs#L1097) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `exposed_protected_roots` | [1105](../../src/sandbox.rs#L1105) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `text` | [1105](../../src/sandbox.rs#L1105) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command(&policy, &available("darwin-seatbelt"), Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear()                 .output()                 .unwrap()                 .status                 .success` | [1108](../../src/sandbox.rs#L1108) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command(&policy, &available("darwin-seatbelt"), Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear()                 .output()                 .unwrap` | [1108](../../src/sandbox.rs#L1108) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command(&policy, &available("darwin-seatbelt"), Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear()                 .output` | [1108](../../src/sandbox.rs#L1108) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command(&policy, &available("darwin-seatbelt"), Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear` | [1108](../../src/sandbox.rs#L1108) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command(&policy, &available("darwin-seatbelt"), Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir` | [1108](../../src/sandbox.rs#L1108) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command(&policy, &available("darwin-seatbelt"), Path::new("/bin/sh"))                 .unwrap()                 .args` | [1108](../../src/sandbox.rs#L1108) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command(&policy, &available("darwin-seatbelt"), Path::new("/bin/sh"))                 .unwrap` | [1108](../../src/sandbox.rs#L1108) | receiver-type-required |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `sandbox_command` | [1108](../../src/sandbox.rs#L1108) | external-constructor-callback-or-unresolved |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `available` | [1108](../../src/sandbox.rs#L1108) | [tools::sandbox::tests::available](../../src/sandbox.rs#L890) |
| `user_instructions_stay_protected_through_absent_paths_and_aliases` | `Path::new` | [1108](../../src/sandbox.rs#L1108) | external-constructor-callback-or-unresolved |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `tempfile::tempdir().unwrap` | [1137](../../src/sandbox.rs#L1137) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `tempfile::tempdir` | [1137](../../src/sandbox.rs#L1137) | external-constructor-callback-or-unresolved |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `root.path().canonicalize().unwrap` | [1138](../../src/sandbox.rs#L1138) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `root.path().canonicalize` | [1138](../../src/sandbox.rs#L1138) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `root.path` | [1138](../../src/sandbox.rs#L1138) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `home.join` | [1139](../../src/sandbox.rs#L1139), [1142](../../src/sandbox.rs#L1142) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `storage.join` | [1140](../../src/sandbox.rs#L1140), [1143](../../src/sandbox.rs#L1143) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `fs::create_dir_all(&job).unwrap` | [1141](../../src/sandbox.rs#L1141) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `fs::create_dir_all` | [1141](../../src/sandbox.rs#L1141), [1142](../../src/sandbox.rs#L1142) | external-constructor-callback-or-unresolved |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `fs::create_dir_all(home.join("project")).unwrap` | [1142](../../src/sandbox.rs#L1142) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `fs::write(storage.join("ledger"), b"original").unwrap` | [1143](../../src/sandbox.rs#L1143) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `fs::write` | [1143](../../src/sandbox.rs#L1143) | external-constructor-callback-or-unresolved |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `path.to_string_lossy().into_owned` | [1144](../../src/sandbox.rs#L1144) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `path.to_string_lossy` | [1144](../../src/sandbox.rs#L1144) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `writes.sort` | [1146](../../src/sandbox.rs#L1146) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `available` | [1156](../../src/sandbox.rs#L1156) | [tools::sandbox::tests::available](../../src/sandbox.rs#L890) |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command(&policy, &status, Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear()                 .output()                 .unwrap()                 .status                 .success` | [1158](../../src/sandbox.rs#L1158) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command(&policy, &status, Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear()                 .output()                 .unwrap` | [1158](../../src/sandbox.rs#L1158) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command(&policy, &status, Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear()                 .output` | [1158](../../src/sandbox.rs#L1158) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command(&policy, &status, Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir(&home)                 .env_clear` | [1158](../../src/sandbox.rs#L1158) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command(&policy, &status, Path::new("/bin/sh"))                 .unwrap()                 .args(["-c", script])                 .current_dir` | [1158](../../src/sandbox.rs#L1158) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command(&policy, &status, Path::new("/bin/sh"))                 .unwrap()                 .args` | [1158](../../src/sandbox.rs#L1158) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command(&policy, &status, Path::new("/bin/sh"))                 .unwrap` | [1158](../../src/sandbox.rs#L1158) | receiver-type-required |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `sandbox_command` | [1158](../../src/sandbox.rs#L1158) | external-constructor-callback-or-unresolved |
| `seatbelt_keeps_protected_root_read_only_beneath_a_writable_home` | `Path::new` | [1158](../../src/sandbox.rs#L1158) | external-constructor-callback-or-unresolved |
