# tekes-selector::cli

[Package atlas](index.md) · [Source](../../src/cli.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-selector::cli::Command](../../src/cli.rs#L11) | enum_item | `pub` |  |
| [tekes-selector::cli::parse_args](../../src/cli.rs#L45) | function_item | `pub` |  |
| [tekes-selector::cli::run_command](../../src/cli.rs#L146) | function_item | `pub` |  |
| [tekes-selector::cli::reply](../../src/cli.rs#L199) | function_item | `private` |  |
| [tekes-selector::cli::rfc3339_now](../../src/cli.rs#L203) | function_item | `pub(crate)` |  |
| [tekes-selector::cli::rfc3339_after](../../src/cli.rs#L207) | function_item | `pub(crate)` |  |
| [tekes-selector::cli::rfc3339_from](../../src/cli.rs#L211) | function_item | `private` |  |
| [tekes-selector::cli::tests::parser_accepts_only_the_closed_ordered_grammar](../../src/cli.rs#L242) | function_item | `private` | test; #[cfg(test)] |
| [tekes-selector::cli::tests::parser_rejects_noncanonical_paths_and_non_loopback_serve](../../src/cli.rs#L301) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `SelectorError` | `crate::error::SelectorError` | `private` |
| `canonical_line` | `crate::fs::canonical_line` | `private` |
| `validate_absolute_lexical` | `crate::fs::validate_absolute_lexical` | `private` |
| `validate_id` | `crate::fs::validate_id` | `private` |
| `Selector` | `crate::selector::Selector` | `private` |
| `cli_command_sha256` | `crate::selector::cli_command_sha256` | `private` |
| `CodeSignatureVerifier` | `crate::signature::CodeSignatureVerifier` | `private` |
| `Command` | `super::Command` | `private` |
| `parse_args` | `super::parse_args` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-selector::cli::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–6: 11 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-selector::cli::run_command"]
  n1["tekes-selector::cli::reply"]
  n2["tekes-selector::cli::rfc3339_now"]
  n3["tekes-selector::cli::rfc3339_after"]
  n4["tekes-selector::cli::rfc3339_from"]
  n5["tekes-selector::cli::parse_args"]
  n6["tekes-selector::error::SelectorError::usage"]
  n7["tekes-selector::error::SelectorError::invalid_state"]
  n8["tekes-selector::fs::canonical_line"]
  n9["tekes-selector::fs::validate_absolute_lexical"]
  n10["tekes-selector::fs::validate_id"]
  n11["tekes-selector::selector::cli_command_sha256"]
  n0 --> n1
  n0 --> n2
  n0 --> n3
  n0 --> n11
  n1 --> n8
  n2 --> n4
  n3 --> n4
  n4 --> n7
  n5 --> n6
  n5 --> n9
  n5 --> n10
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `parse_args` | `args.len` | [46](../../src/cli.rs#L46) | receiver-type-required |
| `parse_args` | `Err` | [47](../../src/cli.rs#L47), [138](../../src/cli.rs#L138) | external-constructor-callback-or-unresolved |
| `parse_args` | `SelectorError::usage` | [47](../../src/cli.rs#L47), [138](../../src/cli.rs#L138) | [tekes-selector::error::SelectorError::usage](../../src/error.rs#L77) |
| `parse_args` | `args.first().cloned().unwrap_or_default` | [48](../../src/cli.rs#L48) | receiver-type-required |
| `parse_args` | `args.first().cloned` | [48](../../src/cli.rs#L48) | receiver-type-required |
| `parse_args` | `args.first` | [48](../../src/cli.rs#L48) | receiver-type-required |
| `parse_args` | `PathBuf::from` | [51](../../src/cli.rs#L51), [57](../../src/cli.rs#L57), [83](../../src/cli.rs#L83), [105](../../src/cli.rs#L105), [120](../../src/cli.rs#L120), [131](../../src/cli.rs#L131), [132](../../src/cli.rs#L132) | external-constructor-callback-or-unresolved |
| `parse_args` | `validate_absolute_lexical` | [52](../../src/cli.rs#L52), [58](../../src/cli.rs#L58), [84](../../src/cli.rs#L84), [106](../../src/cli.rs#L106), [121](../../src/cli.rs#L121), [133](../../src/cli.rs#L133), [134](../../src/cli.rs#L134) | [tekes-selector::fs::validate_absolute_lexical](../../src/fs.rs#L241) |
| `parse_args` | `args[2].as_str` | [53](../../src/cli.rs#L53) | receiver-type-required |
| `parse_args` | `validate_id` | [59](../../src/cli.rs#L59), [66](../../src/cli.rs#L66), [72](../../src/cli.rs#L72), [85](../../src/cli.rs#L85), [86](../../src/cli.rs#L86), [95](../../src/cli.rs#L95) | [tekes-selector::fs::validate_id](../../src/fs.rs#L267) |
| `parse_args` | `version.clone` | [62](../../src/cli.rs#L62), [68](../../src/cli.rs#L68), [88](../../src/cli.rs#L88), [97](../../src/cli.rs#L97) | receiver-type-required |
| `parse_args` | `reason.clone` | [74](../../src/cli.rs#L74) | receiver-type-required |
| `parse_args` | `session.clone` | [89](../../src/cli.rs#L89) | receiver-type-required |
| `parse_args` | `run.clone` | [90](../../src/cli.rs#L90) | receiver-type-required |
| `parse_args` | `listen.clone` | [109](../../src/cli.rs#L109), [124](../../src/cli.rs#L124) | receiver-type-required |
| `parse_args` | `Some` | [125](../../src/cli.rs#L125) | external-constructor-callback-or-unresolved |
| `parse_args` | `web_listen.clone` | [125](../../src/cli.rs#L125) | receiver-type-required |
| `parse_args` | `args.get(2).cloned().unwrap_or_default` | [139](../../src/cli.rs#L139) | receiver-type-required |
| `parse_args` | `args.get(2).cloned` | [139](../../src/cli.rs#L139) | receiver-type-required |
| `parse_args` | `args.get` | [139](../../src/cli.rs#L139) | receiver-type-required |
| `parse_args` | `Ok` | [143](../../src/cli.rs#L143) | external-constructor-callback-or-unresolved |
| `run_command` | `cli_command_sha256` | [151](../../src/cli.rs#L151) | [tekes-selector::selector::cli_command_sha256](../../src/selector.rs#L2682) |
| `run_command` | `reply(selector.stage(bundle, version, command_hash)?).map` | [154](../../src/cli.rs#L154) | receiver-type-required |
| `run_command` | `reply` | [154](../../src/cli.rs#L154), [156](../../src/cli.rs#L156), [157](../../src/cli.rs#L157), [158](../../src/cli.rs#L158), [159](../../src/cli.rs#L159), [160](../../src/cli.rs#L160), [169](../../src/cli.rs#L169), [182](../../src/cli.rs#L182), [186](../../src/cli.rs#L186) | [tekes-selector::cli::reply](../../src/cli.rs#L199) |
| `run_command` | `selector.stage` | [154](../../src/cli.rs#L154) | receiver-type-required |
| `run_command` | `reply(selector.activate(version, command_hash)?).map` | [156](../../src/cli.rs#L156) | receiver-type-required |
| `run_command` | `selector.activate` | [156](../../src/cli.rs#L156) | receiver-type-required |
| `run_command` | `reply(selector.rollback(reason, command_hash)?).map` | [157](../../src/cli.rs#L157) | receiver-type-required |
| `run_command` | `selector.rollback` | [157](../../src/cli.rs#L157) | receiver-type-required |
| `run_command` | `reply(selector.status()?).map` | [158](../../src/cli.rs#L158) | receiver-type-required |
| `run_command` | `selector.status` | [158](../../src/cli.rs#L158) | receiver-type-required |
| `run_command` | `reply(selector.recover()?).map` | [159](../../src/cli.rs#L159) | receiver-type-required |
| `run_command` | `selector.recover` | [159](../../src/cli.rs#L159) | receiver-type-required |
| `run_command` | `reply(selector.prune()?).map` | [160](../../src/cli.rs#L160) | receiver-type-required |
| `run_command` | `selector.prune` | [160](../../src/cli.rs#L160) | receiver-type-required |
| `run_command` | `rfc3339_now` | [167](../../src/cli.rs#L167), [180](../../src/cli.rs#L180) | [tekes-selector::cli::rfc3339_now](../../src/cli.rs#L203) |
| `run_command` | `rfc3339_after` | [168](../../src/cli.rs#L168), [181](../../src/cli.rs#L181) | [tekes-selector::cli::rfc3339_after](../../src/cli.rs#L207) |
| `run_command` | `reply(selector.attest_canary(                 version,                 session,                 run,                 storage_root,                 &accepted,                 &window,             )?)             .map` | [169](../../src/cli.rs#L169) | receiver-type-required |
| `run_command` | `selector.attest_canary` | [169](../../src/cli.rs#L169) | receiver-type-required |
| `run_command` | `reply(selector.attest_install_health(version, "127.0.0.1:7347", &accepted, &window)?)                 .map` | [182](../../src/cli.rs#L182) | receiver-type-required |
| `run_command` | `selector.attest_install_health` | [182](../../src/cli.rs#L182) | receiver-type-required |
| `run_command` | `reply(selector.update_selector(artifact, manifest)?).map` | [186](../../src/cli.rs#L186) | receiver-type-required |
| `run_command` | `selector.update_selector` | [186](../../src/cli.rs#L186) | receiver-type-required |
| `run_command` | `selector.serve_with_web` | [193](../../src/cli.rs#L193) | receiver-type-required |
| `run_command` | `web_listen.as_deref` | [193](../../src/cli.rs#L193) | receiver-type-required |
| `run_command` | `Ok` | [194](../../src/cli.rs#L194) | external-constructor-callback-or-unresolved |
| `reply` | `canonical_line` | [200](../../src/cli.rs#L200) | [tekes-selector::fs::canonical_line](../../src/fs.rs#L117) |
| `rfc3339_now` | `rfc3339_from` | [204](../../src/cli.rs#L204) | [tekes-selector::cli::rfc3339_from](../../src/cli.rs#L211) |
| `rfc3339_now` | `std::time::SystemTime::now` | [204](../../src/cli.rs#L204) | external-constructor-callback-or-unresolved |
| `rfc3339_after` | `rfc3339_from` | [208](../../src/cli.rs#L208) | [tekes-selector::cli::rfc3339_from](../../src/cli.rs#L211) |
| `rfc3339_after` | `std::time::SystemTime::now` | [208](../../src/cli.rs#L208) | external-constructor-callback-or-unresolved |
| `rfc3339_after` | `std::time::Duration::from_secs` | [208](../../src/cli.rs#L208) | external-constructor-callback-or-unresolved |
| `rfc3339_from` | `time         .duration_since(UNIX_EPOCH)         .map_err` | [213](../../src/cli.rs#L213) | receiver-type-required |
| `rfc3339_from` | `time         .duration_since` | [213](../../src/cli.rs#L213) | receiver-type-required |
| `rfc3339_from` | `SelectorError::invalid_state` | [215](../../src/cli.rs#L215), [221](../../src/cli.rs#L221) | [tekes-selector::error::SelectorError::invalid_state](../../src/error.rs#L87) |
| `rfc3339_from` | `duration.as_secs` | [216](../../src/cli.rs#L216) | receiver-type-required |
| `rfc3339_from` | `std::mem::MaybeUninit::<libc::tm>::uninit` | [217](../../src/cli.rs#L217) | external-constructor-callback-or-unresolved |
| `rfc3339_from` | `libc::gmtime_r` | [219](../../src/cli.rs#L219) | external-constructor-callback-or-unresolved |
| `rfc3339_from` | `broken_down.as_mut_ptr` | [219](../../src/cli.rs#L219) | receiver-type-required |
| `rfc3339_from` | `result.is_null` | [220](../../src/cli.rs#L220) | receiver-type-required |
| `rfc3339_from` | `Err` | [221](../../src/cli.rs#L221) | external-constructor-callback-or-unresolved |
| `rfc3339_from` | `broken_down.assume_init` | [224](../../src/cli.rs#L224) | receiver-type-required |
| `rfc3339_from` | `Ok` | [225](../../src/cli.rs#L225) | external-constructor-callback-or-unresolved |
| `parser_accepts_only_the_closed_ordered_grammar` | `[             "--install-root",             "/tmp/Kernel",             "stage",             "--bundle",             "/tmp/Bundle",             "--version",             "1.0.0",         ]         .map` | [243](../../src/cli.rs#L243) | receiver-type-required |
| `parser_accepts_only_the_closed_ordered_grammar` | `parse_args(&args).expect` | [253](../../src/cli.rs#L253) | receiver-type-required |
| `parser_accepts_only_the_closed_ordered_grammar` | `parse_args` | [253](../../src/cli.rs#L253) | [tekes-selector::cli::parse_args](../../src/cli.rs#L45) |
| `parser_accepts_only_the_closed_ordered_grammar` | `[             "--install-root",             "/tmp/Kernel",             "attest-install-health",             "--version",             "1.0.0",         ]         .map` | [256](../../src/cli.rs#L256) | receiver-type-required |
| `parser_accepts_only_the_closed_ordered_grammar` | `[             "--install-root",             "/tmp/Kernel",             "stage",             "--version",             "1.0.0",             "--bundle",             "/tmp/Bundle",         ]         .map` | [271](../../src/cli.rs#L271) | receiver-type-required |
| `parser_accepts_only_the_closed_ordered_grammar` | `["--install-root", "/tmp/Kernel", "prune"].map` | [283](../../src/cli.rs#L283) | receiver-type-required |
| `parser_accepts_only_the_closed_ordered_grammar` | `["--install-root", "/tmp/Kernel", "prune", "--all"].map` | [286](../../src/cli.rs#L286) | receiver-type-required |
| `parser_accepts_only_the_closed_ordered_grammar` | `[             "--install-root",             "/tmp/Kernel",             "status",             "--install-root",             "/tmp/Other",         ]         .map` | [289](../../src/cli.rs#L289) | receiver-type-required |
| `parser_rejects_noncanonical_paths_and_non_loopback_serve` | `["--install-root", "/tmp//Kernel", "status"].map` | [302](../../src/cli.rs#L302) | receiver-type-required |
| `parser_rejects_noncanonical_paths_and_non_loopback_serve` | `[             "--install-root",             "/tmp/Kernel",             "serve",             "--storage-root",             "/tmp/threads",             "--listen",             "0.0.0.0:7347",         ]         .map` | [305](../../src/cli.rs#L305) | receiver-type-required |
| `parser_rejects_noncanonical_paths_and_non_loopback_serve` | `[             "--install-root",             "/tmp/Kernel",             "serve",             "--storage-root",             "/tmp/threads",             "--listen",             "127.0.0.1:7347",             "--web-listen",             "127.0.0.1:7357",         ]         .map` | [317](../../src/cli.rs#L317) | receiver-type-required |
| `parser_rejects_noncanonical_paths_and_non_loopback_serve` | `[             "--install-root",             "/tmp/Kernel",             "serve",             "--storage-root",             "/tmp/threads",             "--listen",             "127.0.0.1:7347",             "--web-listen",             "0.0.0.0:7357",         ]         .map` | [339](../../src/cli.rs#L339) | receiver-type-required |
