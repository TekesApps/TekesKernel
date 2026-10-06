# tekes-supervisor::main

[Package atlas](index.md) · [Source](../../src/main.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::main::WEB_CLIENT_URL](../../src/main.rs#L5) | const_item | `private` |  |
| [tekes-supervisor::main::main](../../src/main.rs#L7) | function_item | `private` |  |
| [tekes-supervisor::main::run_web_launch_url](../../src/main.rs#L88) | function_item | `private` |  |
| [tekes-supervisor::main::run_support_bundle](../../src/main.rs#L116) | function_item | `private` |  |
| [tekes-supervisor::main::run_production](../../src/main.rs#L180) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `env` | `std::env` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `ExitCode` | `std::process::ExitCode` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 9 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::dialect::advertised_dialect_proofs"]
  n1["tekes-supervisor::builtin::run"]
  n2["tekes-supervisor::daemon::DaemonArgs::parse"]
  n3["tekes-supervisor::daemon::system_timestamp"]
  n4["tekes-supervisor::daemon::run_daemon"]
  n5["tekes-supervisor::main::run_support_bundle"]
  n6["tekes-supervisor::main::run_production"]
  n7["tekes-supervisor::main::main"]
  n8["tekes-supervisor::main::run_web_launch_url"]
  n9["tekes-supervisor::observability::publish_support_bundle_from_files"]
  n5 --> n3
  n5 --> n9
  n6 --> n2
  n6 --> n4
  n7 --> n0
  n7 --> n1
  n7 --> n5
  n7 --> n6
  n7 --> n8
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `main` | `env::args_os().len` | [8](../../src/main.rs#L8), [51](../../src/main.rs#L51), [60](../../src/main.rs#L60) | receiver-type-required |
| `main` | `env::args_os` | [8](../../src/main.rs#L8), [9](../../src/main.rs#L9), [25](../../src/main.rs#L25), [26](../../src/main.rs#L26), [48](../../src/main.rs#L48), [51](../../src/main.rs#L51), [52](../../src/main.rs#L52), [60](../../src/main.rs#L60), [61](../../src/main.rs#L61), [76](../../src/main.rs#L76), [79](../../src/main.rs#L79) | external-constructor-callback-or-unresolved |
| `main` | `env::args_os().nth(1).as_deref` | [9](../../src/main.rs#L9), [25](../../src/main.rs#L25), [48](../../src/main.rs#L48), [52](../../src/main.rs#L52), [61](../../src/main.rs#L61), [76](../../src/main.rs#L76), [79](../../src/main.rs#L79) | receiver-type-required |
| `main` | `env::args_os().nth` | [9](../../src/main.rs#L9), [25](../../src/main.rs#L25), [48](../../src/main.rs#L48), [52](../../src/main.rs#L52), [61](../../src/main.rs#L61), [76](../../src/main.rs#L76), [79](../../src/main.rs#L79) | receiver-type-required |
| `main` | `Some` | [9](../../src/main.rs#L9), [25](../../src/main.rs#L25), [48](../../src/main.rs#L48), [52](../../src/main.rs#L52), [61](../../src/main.rs#L61), [76](../../src/main.rs#L76), [79](../../src/main.rs#L79) | external-constructor-callback-or-unresolved |
| `main` | `std::ffi::OsStr::new` | [9](../../src/main.rs#L9), [25](../../src/main.rs#L25), [48](../../src/main.rs#L48), [52](../../src/main.rs#L52), [61](../../src/main.rs#L61), [76](../../src/main.rs#L76), [79](../../src/main.rs#L79) | external-constructor-callback-or-unresolved |
| `main` | `provider::advertised_dialect_proofs().and_then` | [11](../../src/main.rs#L11) | receiver-type-required |
| `main` | `provider::advertised_dialect_proofs` | [11](../../src/main.rs#L11) | [provider::dialect::advertised_dialect_proofs](../../../provider/src/dialect.rs#L880) |
| `main` | `serde_json::to_string(&proofs)                 .map_err` | [12](../../src/main.rs#L12) | receiver-type-required |
| `main` | `serde_json::to_string` | [12](../../src/main.rs#L12) | external-constructor-callback-or-unresolved |
| `main` | `provider::DialectError::UnprovedProfile` | [13](../../src/main.rs#L13) | external-constructor-callback-or-unresolved |
| `main` | `"catalog".into` | [13](../../src/main.rs#L13) | receiver-type-required |
| `main` | `ExitCode::from` | [21](../../src/main.rs#L21), [28](../../src/main.rs#L28), [43](../../src/main.rs#L43), [85](../../src/main.rs#L85) | external-constructor-callback-or-unresolved |
| `main` | `env::args_os().collect::<Vec<_>>` | [26](../../src/main.rs#L26) | receiver-type-required |
| `main` | `args.len` | [27](../../src/main.rs#L27) | receiver-type-required |
| `main` | `tokio::runtime::Builder::new_multi_thread()             .enable_all()             .build` | [30](../../src/main.rs#L30) | receiver-type-required |
| `main` | `tokio::runtime::Builder::new_multi_thread()             .enable_all` | [30](../../src/main.rs#L30) | receiver-type-required |
| `main` | `tokio::runtime::Builder::new_multi_thread` | [30](../../src/main.rs#L30) | external-constructor-callback-or-unresolved |
| `main` | `runtime.block_on` | [35](../../src/main.rs#L35) | receiver-type-required |
| `main` | `tekes_supervisor::builtin::run` | [35](../../src/main.rs#L35) | [tekes-supervisor::builtin::run](../../src/builtin.rs#L29) |
| `main` | `PathBuf::from` | [35](../../src/main.rs#L35) | external-constructor-callback-or-unresolved |
| `main` | `Err` | [37](../../src/main.rs#L37) | external-constructor-callback-or-unresolved |
| `main` | `Box::new` | [37](../../src/main.rs#L37) | external-constructor-callback-or-unresolved |
| `main` | `run_web_launch_url` | [49](../../src/main.rs#L49) | [tekes-supervisor::main::run_web_launch_url](../../src/main.rs#L88) |
| `main` | `option_env!("TEKES_SELECTED_BUILD").unwrap_or` | [64](../../src/main.rs#L64) | receiver-type-required |
| `main` | `tekes_supervisor::daemon::SOURCE_REVISION             .map(&#124;revision&#124; format!("\"source_revision\":\"{revision}\","))             .unwrap_or_default` | [65](../../src/main.rs#L65) | receiver-type-required |
| `main` | `tekes_supervisor::daemon::SOURCE_REVISION             .map` | [65](../../src/main.rs#L65) | receiver-type-required |
| `main` | `run_support_bundle` | [77](../../src/main.rs#L77) | [tekes-supervisor::main::run_support_bundle](../../src/main.rs#L116) |
| `main` | `run_production` | [80](../../src/main.rs#L80) | [tekes-supervisor::main::run_production](../../src/main.rs#L180) |
| `run_web_launch_url` | `env::args_os().skip(1).collect::<Vec<_>>` | [89](../../src/main.rs#L89) | receiver-type-required |
| `run_web_launch_url` | `env::args_os().skip` | [89](../../src/main.rs#L89) | receiver-type-required |
| `run_web_launch_url` | `env::args_os` | [89](../../src/main.rs#L89) | external-constructor-callback-or-unresolved |
| `run_web_launch_url` | `ExitCode::from` | [94](../../src/main.rs#L94) | external-constructor-callback-or-unresolved |
| `run_web_launch_url` | `arguments.len` | [96](../../src/main.rs#L96) | receiver-type-required |
| `run_web_launch_url` | `usage` | [97](../../src/main.rs#L97), [100](../../src/main.rs#L100), [103](../../src/main.rs#L103), [110](../../src/main.rs#L110) | external-constructor-callback-or-unresolved |
| `run_web_launch_url` | `arguments[2].to_str` | [99](../../src/main.rs#L99) | receiver-type-required |
| `run_web_launch_url` | `access_group.strip_suffix` | [102](../../src/main.rs#L102) | receiver-type-required |
| `run_web_launch_url` | `team.len` | [105](../../src/main.rs#L105) | receiver-type-required |
| `run_web_launch_url` | `team             .bytes()             .all` | [106](../../src/main.rs#L106) | receiver-type-required |
| `run_web_launch_url` | `team             .bytes` | [106](../../src/main.rs#L106) | receiver-type-required |
| `run_web_launch_url` | `byte.is_ascii_uppercase` | [108](../../src/main.rs#L108) | receiver-type-required |
| `run_web_launch_url` | `byte.is_ascii_digit` | [108](../../src/main.rs#L108) | receiver-type-required |
| `run_support_bundle` | `env::args_os().skip(1).collect::<Vec<_>>` | [117](../../src/main.rs#L117) | receiver-type-required |
| `run_support_bundle` | `env::args_os().skip` | [117](../../src/main.rs#L117) | receiver-type-required |
| `run_support_bundle` | `env::args_os` | [117](../../src/main.rs#L117) | external-constructor-callback-or-unresolved |
| `run_support_bundle` | `ExitCode::from` | [122](../../src/main.rs#L122), [161](../../src/main.rs#L161), [175](../../src/main.rs#L175) | external-constructor-callback-or-unresolved |
| `run_support_bundle` | `arguments.len` | [124](../../src/main.rs#L124) | receiver-type-required |
| `run_support_bundle` | `usage` | [132](../../src/main.rs#L132), [137](../../src/main.rs#L137), [140](../../src/main.rs#L140), [143](../../src/main.rs#L143), [151](../../src/main.rs#L151) | external-constructor-callback-or-unresolved |
| `run_support_bundle` | `PathBuf::from` | [134](../../src/main.rs#L134), [135](../../src/main.rs#L135) | external-constructor-callback-or-unresolved |
| `run_support_bundle` | `arguments[6].to_str` | [136](../../src/main.rs#L136) | receiver-type-required |
| `run_support_bundle` | `arguments[8].to_str` | [139](../../src/main.rs#L139) | receiver-type-required |
| `run_support_bundle` | `arguments[10].to_str` | [142](../../src/main.rs#L142) | receiver-type-required |
| `run_support_bundle` | `destination.is_absolute` | [145](../../src/main.rs#L145) | receiver-type-required |
| `run_support_bundle` | `log_root.is_absolute` | [146](../../src/main.rs#L146) | receiver-type-required |
| `run_support_bundle` | `destination.exists` | [147](../../src/main.rs#L147) | receiver-type-required |
| `run_support_bundle` | `build.is_empty` | [148](../../src/main.rs#L148) | receiver-type-required |
| `run_support_bundle` | `config_digests.is_empty` | [149](../../src/main.rs#L149) | receiver-type-required |
| `run_support_bundle` | `config_digests         .split(',')         .map(str::to_owned)         .collect::<Vec<_>>` | [153](../../src/main.rs#L153) | receiver-type-required |
| `run_support_bundle` | `config_digests         .split(',')         .map` | [153](../../src/main.rs#L153) | receiver-type-required |
| `run_support_bundle` | `config_digests         .split` | [153](../../src/main.rs#L153) | receiver-type-required |
| `run_support_bundle` | `tekes_supervisor::daemon::system_timestamp` | [157](../../src/main.rs#L157) | [tekes-supervisor::daemon::system_timestamp](../../src/daemon.rs#L1412) |
| `run_support_bundle` | `tekes_supervisor::observability::publish_support_bundle_from_files` | [164](../../src/main.rs#L164) | [tekes-supervisor::observability::publish_support_bundle_from_files](../../src/observability.rs#L987) |
| `run_production` | `env::args_os().skip(1).collect::<Vec<_>>` | [181](../../src/main.rs#L181) | receiver-type-required |
| `run_production` | `env::args_os().skip` | [181](../../src/main.rs#L181) | receiver-type-required |
| `run_production` | `env::args_os` | [181](../../src/main.rs#L181) | external-constructor-callback-or-unresolved |
| `run_production` | `tekes_supervisor::daemon::DaemonArgs::parse` | [182](../../src/main.rs#L182) | [tekes-supervisor::daemon::DaemonArgs::parse](../../src/daemon.rs#L100) |
| `run_production` | `ExitCode::from` | [186](../../src/main.rs#L186), [196](../../src/main.rs#L196), [203](../../src/main.rs#L203) | external-constructor-callback-or-unresolved |
| `run_production` | `error.exit_code` | [186](../../src/main.rs#L186), [203](../../src/main.rs#L203) | receiver-type-required |
| `run_production` | `tokio::runtime::Builder::new_multi_thread()         .enable_all()         .build` | [189](../../src/main.rs#L189) | receiver-type-required |
| `run_production` | `tokio::runtime::Builder::new_multi_thread()         .enable_all` | [189](../../src/main.rs#L189) | receiver-type-required |
| `run_production` | `tokio::runtime::Builder::new_multi_thread` | [189](../../src/main.rs#L189) | external-constructor-callback-or-unresolved |
| `run_production` | `runtime.block_on` | [199](../../src/main.rs#L199) | receiver-type-required |
| `run_production` | `tekes_supervisor::daemon::run_daemon` | [199](../../src/main.rs#L199) | [tekes-supervisor::daemon::run_daemon](../../src/daemon.rs#L742) |
