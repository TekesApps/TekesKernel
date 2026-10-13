# tools::helper

[Package atlas](index.md) · [Source](../../src/helper.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tools::helper::HELPER_PROTOCOL](../../src/helper.rs#L32) | const_item | `pub` |  |
| [tools::helper::HELPER_VERSION](../../src/helper.rs#L33) | const_item | `pub` |  |
| [tools::helper::EX_PROTOCOL](../../src/helper.rs#L34) | const_item | `pub` |  |
| [tools::helper::HARD_BYTES](../../src/helper.rs#L35) | const_item | `private` |  |
| [tools::helper::HARD_READ_BYTES](../../src/helper.rs#L36) | const_item | `private` |  |
| [tools::helper::HARD_TIMEOUT_MS](../../src/helper.rs#L37) | const_item | `private` |  |
| [tools::helper::TERM_GRACE_MS](../../src/helper.rs#L38) | const_item | `private` |  |
| [tools::helper::HELPER_PROCESS_TIMEOUT_MS](../../src/helper.rs#L39) | const_item | `private` |  |
| [tools::helper::HELPER_PROTOCOL_STDOUT_BYTES](../../src/helper.rs#L42) | const_item | `private` |  |
| [tools::helper::HELPER_PROTOCOL_STDERR_BYTES](../../src/helper.rs#L43) | const_item | `private` |  |
| [tools::helper::ByteString](../../src/helper.rs#L47) | struct_item | `pub` |  |
| [tools::helper::ByteEncoding](../../src/helper.rs#L54) | enum_item | `pub` |  |
| [tools::helper::ByteString::from_bytes](../../src/helper.rs#L61) | function_item | `pub` |  |
| [tools::helper::ByteString::decode](../../src/helper.rs#L74) | function_item | `pub` |  |
| [tools::helper::CreateMode](../../src/helper.rs#L89) | enum_item | `pub` |  |
| [tools::helper::ExecRequest](../../src/helper.rs#L97) | struct_item | `pub` |  |
| [tools::helper::HelperPath](../../src/helper.rs#L115) | struct_item | `pub` |  |
| [tools::helper::HelperOperation](../../src/helper.rs#L122) | enum_item | `pub` |  |
| [tools::helper::HelperRequest](../../src/helper.rs#L161) | struct_item | `pub` |  |
| [tools::helper::ReadValue](../../src/helper.rs#L168) | struct_item | `pub` |  |
| [tools::helper::WriteValue](../../src/helper.rs#L176) | struct_item | `pub` |  |
| [tools::helper::ExecValue](../../src/helper.rs#L183) | struct_item | `pub` |  |
| [tools::helper::HelperValue](../../src/helper.rs#L193) | enum_item | `pub` |  |
| [tools::helper::HelperErrorClass](../../src/helper.rs#L203) | enum_item | `pub` |  |
| [tools::helper::HelperError](../../src/helper.rs#L223) | struct_item | `pub` |  |
| [tools::helper::HelperClientError](../../src/helper.rs#L230) | enum_item | `pub` |  |
| [tools::helper::HelperError::new](../../src/helper.rs#L239) | function_item | `pub` |  |
| [tools::helper::HelperError::io](../../src/helper.rs#L247) | function_item | `private` |  |
| [tools::helper::HelperError::from](../../src/helper.rs#L259) | function_item | `private` |  |
| [tools::helper::HelperResponse](../../src/helper.rs#L265) | enum_item | `pub` |  |
| [tools::helper::Hello](../../src/helper.rs#L272) | struct_item | `private` |  |
| [tools::helper::Selected](../../src/helper.rs#L280) | struct_item | `private` |  |
| [tools::helper::Reject](../../src/helper.rs#L286) | struct_item | `private` |  |
| [tools::helper::ResultPayload](../../src/helper.rs#L292) | struct_item | `private` |  |
| [tools::helper::ErrorPayload](../../src/helper.rs#L299) | struct_item | `private` |  |
| [tools::helper::encode_helper_line](../../src/helper.rs#L305) | function_item | `pub` |  |
| [tools::helper::decode_helper_line](../../src/helper.rs#L318) | function_item | `pub` |  |
| [tools::helper::RootBinding](../../src/helper.rs#L362) | struct_item | `pub` |  |
| [tools::helper::RootBinding::open](../../src/helper.rs#L369) | function_item | `pub` |  |
| [tools::helper::HelperServer](../../src/helper.rs#L409) | struct_item | `pub` |  |
| [tools::helper::HelperServer::new](../../src/helper.rs#L414) | function_item | `pub` |  |
| [tools::helper::HelperServer::execute](../../src/helper.rs#L427) | function_item | `pub` |  |
| [tools::helper::HelperServer::execute_inner](../../src/helper.rs#L441) | function_item | `private` |  |
| [tools::helper::HelperServer::root](../../src/helper.rs#L544) | function_item | `private` |  |
| [tools::helper::HelperServer::exec](../../src/helper.rs#L553) | function_item | `private` |  |
| [tools::helper::HelperClient](../../src/helper.rs#L672) | struct_item | `pub` |  |
| [tools::helper::HelperIsolation](../../src/helper.rs#L680) | enum_item | `private` |  |
| [tools::helper::HelperClient::sandboxed_with_scratch](../../src/helper.rs#L690) | function_item | `pub` |  |
| [tools::helper::HelperClient::scratch_path](../../src/helper.rs#L716) | function_item | `pub` |  |
| [tools::helper::HelperClient::sandboxed](../../src/helper.rs#L723) | function_item | `pub` |  |
| [tools::helper::HelperClient::approved_unsandboxed](../../src/helper.rs#L746) | function_item | `pub` |  |
| [tools::helper::HelperClient::execute](../../src/helper.rs#L767) | function_item | `pub` |  |
| [tools::helper::HelperClient::execute_cancellable](../../src/helper.rs#L784) | function_item | `pub` |  |
| [tools::helper::serve_stdio](../../src/helper.rs#L946) | function_item | `pub` |  |
| [tools::helper::decode_terminal](../../src/helper.rs#L1000) | function_item | `private` |  |
| [tools::helper::from_payload](../../src/helper.rs#L1024) | function_item | `private` |  |
| [tools::helper::read_line_limited](../../src/helper.rs#L1029) | function_item | `private` |  |
| [tools::helper::valid_root_name](../../src/helper.rs#L1048) | function_item | `private` |  |
| [tools::helper::normalized_relative](../../src/helper.rs#L1057) | function_item | `private` |  |
| [tools::helper::split_parent](../../src/helper.rs#L1080) | function_item | `private` |  |
| [tools::helper::open_parent](../../src/helper.rs#L1102) | function_item | `private` |  |
| [tools::helper::create_parent](../../src/helper.rs#L1110) | function_item | `private` |  |
| [tools::helper::duplicate_fd](../../src/helper.rs#L1137) | function_item | `private` |  |
| [tools::helper::open_directory_at](../../src/helper.rs#L1147) | function_item | `private` |  |
| [tools::helper::open_regular_at](../../src/helper.rs#L1169) | function_item | `private` |  |
| [tools::helper::classify_open](../../src/helper.rs#L1201) | function_item | `private` |  |
| [tools::helper::read_regular](../../src/helper.rs#L1209) | function_item | `private` |  |
| [tools::helper::atomic_replace](../../src/helper.rs#L1232) | function_item | `private` |  |
| [tools::helper::rename_at](../../src/helper.rs#L1284) | function_item | `private` |  |
| [tools::helper::unlink_at](../../src/helper.rs#L1326) | function_item | `private` |  |
| [tools::helper::glob_beneath](../../src/helper.rs#L1336) | function_item | `private` |  |
| [tools::helper::glob_match](../../src/helper.rs#L1387) | function_item | `private` |  |
| [tools::helper::glob_match::matches](../../src/helper.rs#L1388) | function_item | `private` |  |
| [tools::helper::read_capped](../../src/helper.rs#L1427) | function_item | `private` |  |
| [tools::helper::terminate_then_kill](../../src/helper.rs#L1443) | function_item | `private` |  |
| [tools::helper::checked_cap](../../src/helper.rs#L1463) | function_item | `private` |  |
| [tools::helper::checked_cap_with](../../src/helper.rs#L1467) | function_item | `private` |  |
| [tools::helper::check_content_cap](../../src/helper.rs#L1479) | function_item | `private` |  |
| [tools::helper::sha256](../../src/helper.rs#L1489) | function_item | `private` |  |
| [tools::helper::monotonic_nonce](../../src/helper.rs#L1493) | function_item | `private` |  |
| [tools::helper::monotonic_nonce::NONCE](../../src/helper.rs#L1495) | static_item | `private` |  |
| [tools::helper::nested_create_tests::create_builds_parents_and_preserves_no_replace](../../src/helper.rs#L1505) | function_item | `private` | test; #[cfg(test)] |
| [tools::helper::nested_create_tests::parent_creation_is_limited_to_new_and_validates_entire_path_first](../../src/helper.rs#L1535) | function_item | `private` | test; #[cfg(test)] |
| [tools::helper::nested_create_tests::parent_creation_cannot_follow_symlinks_or_escape_root](../../src/helper.rs#L1572) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `CString` | `std::ffi::CString` | `private` |
| `OsStr` | `std::ffi::OsStr` | `private` |
| `OsString` | `std::ffi::OsString` | `private` |
| `File` | `std::fs::File` | `private` |
| `Read` | `std::io::Read` | `private` |
| `Write` | `std::io::Write` | `private` |
| `AsRawFd` | `std::os::fd::AsRawFd` | `private` |
| `FromRawFd` | `std::os::fd::FromRawFd` | `private` |
| `RawFd` | `std::os::fd::RawFd` | `private` |
| `_` | `std::os::unix::ffi::OsStrExt` | `private` |
| `_` | `std::os::unix::fs::PermissionsExt` | `private` |
| `_` | `std::os::unix::process::CommandExt` | `private` |
| `Component` | `std::path::Component` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Command` | `std::process::Command` | `private` |
| `Stdio` | `std::process::Stdio` | `private` |
| `thread` | `std::thread` | `private` |
| `Duration` | `std::time::Duration` | `private` |
| `Instant` | `std::time::Instant` | `private` |
| `_` | `base64::Engine` | `private` |
| `BASE64` | `base64::engine::general_purpose::STANDARD` | `private` |
| `DeserializeOwned` | `serde::de::DeserializeOwned` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Map` | `serde_json::Map` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `FullSync` | `store::FullSync` | `private` |
| `ProbeStatus` | `crate::sandbox::ProbeStatus` | `private` |
| `SandboxApproval` | `crate::sandbox::SandboxApproval` | `private` |
| `SandboxPolicy` | `crate::sandbox::SandboxPolicy` | `private` |
| `policy_digest` | `crate::sandbox::policy_digest` | `private` |
| `sandbox_command` | `crate::sandbox::sandbox_command` | `private` |
| `validate_unsandboxed_approval` | `crate::sandbox::validate_unsandboxed_approval` | `private` |
| `*` | `super::*` | `private` |
| `symlink` | `std::os::unix::fs::symlink` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tools::helper::native_grep` | `private` | #[path = "helper/native_grep.rs"] |
| `tools::helper::nested_create_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 55 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["tools::helper::decode_terminal"]
  n2["tools::helper::from_payload"]
  n3["tools::helper::read_line_limited"]
  n4["tools::helper::valid_root_name"]
  n5["tools::helper::normalized_relative"]
  n6["tools::helper::open_parent"]
  n7["tools::helper::open_directory_at"]
  n8["tools::helper::read_regular"]
  n9["tools::helper::atomic_replace"]
  n10["tools::helper::glob_beneath"]
  n11["tools::helper::read_capped"]
  n12["tools::helper::terminate_then_kill"]
  n13["tools::helper::checked_cap"]
  n14["tools::helper::checked_cap_with"]
  n15["tools::helper::check_content_cap"]
  n16["tools::helper::sha256"]
  n17["tools::helper::HelperError::new"]
  n18["tools::helper::HelperError::io"]
  n19["tools::helper::HelperError::from"]
  n20["tools::helper::encode_helper_line"]
  n21["tools::helper::decode_helper_line"]
  n22["tools::helper::RootBinding::open"]
  n23["tools::helper::HelperServer::new"]
  n24["tools::helper::HelperServer::execute"]
  n25["tools::helper::HelperServer::execute_inner"]
  n26["tools::helper::HelperServer::root"]
  n27["tools::helper::HelperServer::exec"]
  n28["tools::helper::ByteString::from_bytes"]
  n29["tools::helper::HelperClient::sandboxed_with_scratch"]
  n30["tools::helper::HelperClient::scratch_path"]
  n31["tools::helper::HelperClient::sandboxed"]
  n32["tools::helper::HelperClient::approved_unsandboxed"]
  n33["tools::helper::ByteString::decode"]
  n34["tools::helper::HelperClient::execute"]
  n35["tools::helper::HelperClient::execute_cancellable"]
  n36["tools::helper::serve_stdio"]
  n37["tools::helper::native_grep::search"]
  n38["tools::sandbox::policy_digest"]
  n39["tools::sandbox::validate_unsandboxed_approval"]
  n40["tools::sandbox::sandbox_command"]
  n18 --> n17
  n20 --> n17
  n21 --> n0
  n21 --> n17
  n22 --> n4
  n22 --> n7
  n22 --> n17
  n23 --> n17
  n24 --> n25
  n25 --> n8
  n25 --> n9
  n25 --> n10
  n25 --> n13
  n25 --> n14
  n25 --> n15
  n25 --> n16
  n25 --> n17
  n25 --> n26
  n25 --> n27
  n25 --> n28
  n25 --> n37
  n26 --> n17
  n27 --> n5
  n27 --> n6
  n27 --> n11
  n27 --> n12
  n27 --> n13
  n27 --> n15
  n27 --> n17
  n27 --> n18
  n27 --> n26
  n27 --> n28
  n29 --> n17
  n29 --> n31
  n31 --> n17
  n32 --> n17
  n32 --> n38
  n32 --> n39
  n33 --> n17
  n34 --> n17
  n34 --> n35
  n35 --> n1
  n35 --> n2
  n35 --> n11
  n35 --> n12
  n35 --> n17
  n35 --> n18
  n35 --> n20
  n35 --> n21
  n35 --> n40
  n36 --> n2
  n36 --> n3
  n36 --> n17
  n36 --> n20
  n36 --> n21
```

</details>

<details><summary>Functions 21–40: 40 direct edges</summary>

```mermaid
flowchart TD
  n0["store::platform::FullSync::full_sync"]
  n1["tools::helper::decode_terminal"]
  n2["tools::helper::from_payload"]
  n3["tools::helper::read_line_limited"]
  n4["tools::helper::valid_root_name"]
  n5["tools::helper::normalized_relative"]
  n6["tools::helper::split_parent"]
  n7["tools::helper::open_parent"]
  n8["tools::helper::create_parent"]
  n9["tools::helper::duplicate_fd"]
  n10["tools::helper::open_directory_at"]
  n11["tools::helper::open_regular_at"]
  n12["tools::helper::classify_open"]
  n13["tools::helper::read_regular"]
  n14["tools::helper::atomic_replace"]
  n15["tools::helper::rename_at"]
  n16["tools::helper::unlink_at"]
  n17["tools::helper::glob_beneath"]
  n18["tools::helper::glob_match"]
  n19["tools::helper::glob_match::matches"]
  n20["tools::helper::read_capped"]
  n21["tools::helper::HelperError::new"]
  n22["tools::helper::HelperError::io"]
  n23["tools::helper::decode_helper_line"]
  n1 --> n2
  n1 --> n21
  n1 --> n23
  n2 --> n21
  n3 --> n21
  n5 --> n21
  n6 --> n5
  n6 --> n21
  n7 --> n9
  n7 --> n10
  n8 --> n9
  n8 --> n10
  n8 --> n12
  n8 --> n21
  n9 --> n22
  n10 --> n12
  n10 --> n21
  n11 --> n12
  n11 --> n21
  n12 --> n21
  n12 --> n22
  n13 --> n6
  n13 --> n7
  n13 --> n11
  n13 --> n21
  n14 --> n0
  n14 --> n6
  n14 --> n7
  n14 --> n8
  n14 --> n11
  n14 --> n15
  n14 --> n16
  n14 --> n21
  n15 --> n21
  n15 --> n22
  n16 --> n21
  n16 --> n22
  n17 --> n18
  n17 --> n21
  n19 --> n19
```

</details>

<details><summary>Functions 41–46: 3 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::helper::terminate_then_kill"]
  n1["tools::helper::checked_cap"]
  n2["tools::helper::checked_cap_with"]
  n3["tools::helper::check_content_cap"]
  n4["tools::helper::sha256"]
  n5["tools::helper::monotonic_nonce"]
  n6["tools::helper::HelperError::new"]
  n1 --> n2
  n2 --> n6
  n3 --> n6
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `from_bytes` | `std::str::from_utf8` | [62](../../src/helper.rs#L62) | external-constructor-callback-or-unresolved |
| `from_bytes` | `text.to_owned` | [65](../../src/helper.rs#L65) | receiver-type-required |
| `from_bytes` | `BASE64.encode` | [69](../../src/helper.rs#L69) | receiver-type-required |
| `decode` | `Ok` | [76](../../src/helper.rs#L76) | external-constructor-callback-or-unresolved |
| `decode` | `self.data.as_bytes().to_vec` | [76](../../src/helper.rs#L76) | receiver-type-required |
| `decode` | `self.data.as_bytes` | [76](../../src/helper.rs#L76) | receiver-type-required |
| `decode` | `BASE64.decode(&self.data).map_err` | [77](../../src/helper.rs#L77) | receiver-type-required |
| `decode` | `BASE64.decode` | [77](../../src/helper.rs#L77) | receiver-type-required |
| `decode` | `HelperError::new` | [78](../../src/helper.rs#L78) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `new` | `message.into` | [242](../../src/helper.rs#L242) | receiver-type-required |
| `io` | `error.kind` | [248](../../src/helper.rs#L248) | receiver-type-required |
| `io` | `Self::new` | [254](../../src/helper.rs#L254) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `io` | `error.to_string` | [254](../../src/helper.rs#L254) | receiver-type-required |
| `from` | `Self::io` | [260](../../src/helper.rs#L260) | external-constructor-callback-or-unresolved |
| `encode_helper_line` | `Map::new` | [306](../../src/helper.rs#L306) | external-constructor-callback-or-unresolved |
| `encode_helper_line` | `object.insert` | [307](../../src/helper.rs#L307) | receiver-type-required |
| `encode_helper_line` | `key.to_owned` | [308](../../src/helper.rs#L308) | receiver-type-required |
| `encode_helper_line` | `serde_json::to_value(payload)             .map_err` | [309](../../src/helper.rs#L309) | receiver-type-required |
| `encode_helper_line` | `serde_json::to_value` | [309](../../src/helper.rs#L309) | external-constructor-callback-or-unresolved |
| `encode_helper_line` | `HelperError::new` | [310](../../src/helper.rs#L310), [313](../../src/helper.rs#L313) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `encode_helper_line` | `error.to_string` | [310](../../src/helper.rs#L310), [313](../../src/helper.rs#L313) | receiver-type-required |
| `encode_helper_line` | `serde_json_canonicalizer::to_vec(&Value::Object(object))         .map_err` | [312](../../src/helper.rs#L312) | receiver-type-required |
| `encode_helper_line` | `serde_json_canonicalizer::to_vec` | [312](../../src/helper.rs#L312) | external-constructor-callback-or-unresolved |
| `encode_helper_line` | `Value::Object` | [312](../../src/helper.rs#L312) | external-constructor-callback-or-unresolved |
| `encode_helper_line` | `bytes.push` | [314](../../src/helper.rs#L314) | receiver-type-required |
| `encode_helper_line` | `Ok` | [315](../../src/helper.rs#L315) | external-constructor-callback-or-unresolved |
| `decode_helper_line` | `bytes.strip_suffix` | [319](../../src/helper.rs#L319) | receiver-type-required |
| `decode_helper_line` | `Err` | [320](../../src/helper.rs#L320), [326](../../src/helper.rs#L326), [338](../../src/helper.rs#L338), [349](../../src/helper.rs#L349) | external-constructor-callback-or-unresolved |
| `decode_helper_line` | `HelperError::new` | [320](../../src/helper.rs#L320), [326](../../src/helper.rs#L326), [332](../../src/helper.rs#L332), [335](../../src/helper.rs#L335), [338](../../src/helper.rs#L338), [344](../../src/helper.rs#L344), [347](../../src/helper.rs#L347), [349](../../src/helper.rs#L349) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `decode_helper_line` | `body.is_empty` | [325](../../src/helper.rs#L325) | receiver-type-required |
| `decode_helper_line` | `body.contains` | [325](../../src/helper.rs#L325) | receiver-type-required |
| `decode_helper_line` | `schema::IJsonValue::parse(body)         .map_err` | [331](../../src/helper.rs#L331) | receiver-type-required |
| `decode_helper_line` | `schema::IJsonValue::parse` | [331](../../src/helper.rs#L331) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `decode_helper_line` | `error.to_string` | [332](../../src/helper.rs#L332), [335](../../src/helper.rs#L335), [344](../../src/helper.rs#L344) | receiver-type-required |
| `decode_helper_line` | `parsed         .canonical_bytes()         .map_err` | [333](../../src/helper.rs#L333) | receiver-type-required |
| `decode_helper_line` | `parsed         .canonical_bytes` | [333](../../src/helper.rs#L333) | receiver-type-required |
| `decode_helper_line` | `serde_json::from_slice(body)         .map_err` | [343](../../src/helper.rs#L343) | receiver-type-required |
| `decode_helper_line` | `serde_json::from_slice` | [343](../../src/helper.rs#L343) | external-constructor-callback-or-unresolved |
| `decode_helper_line` | `value         .as_object()         .ok_or_else` | [345](../../src/helper.rs#L345) | receiver-type-required |
| `decode_helper_line` | `value         .as_object` | [345](../../src/helper.rs#L345) | receiver-type-required |
| `decode_helper_line` | `object.len` | [348](../../src/helper.rs#L348) | receiver-type-required |
| `decode_helper_line` | `Ok` | [354](../../src/helper.rs#L354) | external-constructor-callback-or-unresolved |
| `decode_helper_line` | `object         .iter()         .next()         .map(&#124;(key, value)&#124; (key.clone(), value.clone()))         .expect` | [354](../../src/helper.rs#L354) | receiver-type-required |
| `decode_helper_line` | `object         .iter()         .next()         .map` | [354](../../src/helper.rs#L354) | receiver-type-required |
| `decode_helper_line` | `object         .iter()         .next` | [354](../../src/helper.rs#L354) | receiver-type-required |
| `decode_helper_line` | `object         .iter` | [354](../../src/helper.rs#L354) | receiver-type-required |
| `decode_helper_line` | `key.clone` | [357](../../src/helper.rs#L357) | receiver-type-required |
| `decode_helper_line` | `value.clone` | [357](../../src/helper.rs#L357) | receiver-type-required |
| `open` | `name.into` | [370](../../src/helper.rs#L370) | receiver-type-required |
| `open` | `valid_root_name` | [371](../../src/helper.rs#L371) | [tools::helper::valid_root_name](../../src/helper.rs#L1048) |
| `open` | `Err` | [372](../../src/helper.rs#L372), [379](../../src/helper.rs#L379), [390](../../src/helper.rs#L390) | external-constructor-callback-or-unresolved |
| `open` | `HelperError::new` | [372](../../src/helper.rs#L372), [379](../../src/helper.rs#L379), [390](../../src/helper.rs#L390) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `open` | `path.as_ref` | [377](../../src/helper.rs#L377) | receiver-type-required |
| `open` | `path.is_absolute` | [378](../../src/helper.rs#L378) | receiver-type-required |
| `open` | `path.to_str().is_none` | [378](../../src/helper.rs#L378) | receiver-type-required |
| `open` | `path.to_str` | [378](../../src/helper.rs#L378) | receiver-type-required |
| `open` | `path.components().any` | [384](../../src/helper.rs#L384) | receiver-type-required |
| `open` | `path.components` | [384](../../src/helper.rs#L384) | receiver-type-required |
| `open` | `open_directory_at(libc::AT_FDCWD, path.as_os_str()).map_err` | [396](../../src/helper.rs#L396) | receiver-type-required |
| `open` | `open_directory_at` | [396](../../src/helper.rs#L396) | [tools::helper::open_directory_at](../../src/helper.rs#L1147) |
| `open` | `path.as_os_str` | [396](../../src/helper.rs#L396) | receiver-type-required |
| `open` | `Ok` | [400](../../src/helper.rs#L400) | external-constructor-callback-or-unresolved |
| `open` | `path.to_path_buf` | [402](../../src/helper.rs#L402) | receiver-type-required |
| `new` | `BTreeMap::new` | [415](../../src/helper.rs#L415) | external-constructor-callback-or-unresolved |
| `new` | `map.insert(root.name.clone(), root).is_some` | [417](../../src/helper.rs#L417) | receiver-type-required |
| `new` | `map.insert` | [417](../../src/helper.rs#L417) | receiver-type-required |
| `new` | `root.name.clone` | [417](../../src/helper.rs#L417) | receiver-type-required |
| `new` | `Err` | [418](../../src/helper.rs#L418) | external-constructor-callback-or-unresolved |
| `new` | `HelperError::new` | [418](../../src/helper.rs#L418) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `new` | `Ok` | [424](../../src/helper.rs#L424) | external-constructor-callback-or-unresolved |
| `execute` | `self.execute_inner` | [428](../../src/helper.rs#L428) | [tools::helper::HelperServer::execute_inner](../../src/helper.rs#L441) |
| `execute` | `request.id.clone` | [431](../../src/helper.rs#L431), [435](../../src/helper.rs#L435) | receiver-type-required |
| `execute_inner` | `request.id.is_empty` | [442](../../src/helper.rs#L442) | receiver-type-required |
| `execute_inner` | `Err` | [443](../../src/helper.rs#L443), [487](../../src/helper.rs#L487) | external-constructor-callback-or-unresolved |
| `execute_inner` | `HelperError::new` | [443](../../src/helper.rs#L443), [487](../../src/helper.rs#L487) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `execute_inner` | `checked_cap_with` | [454](../../src/helper.rs#L454) | [tools::helper::checked_cap_with](../../src/helper.rs#L1467) |
| `execute_inner` | `self.root` | [455](../../src/helper.rs#L455), [469](../../src/helper.rs#L469), [484](../../src/helper.rs#L484), [511](../../src/helper.rs#L511), [526](../../src/helper.rs#L526) | [tools::helper::HelperServer::root](../../src/helper.rs#L544) |
| `execute_inner` | `read_regular` | [456](../../src/helper.rs#L456), [485](../../src/helper.rs#L485) | [tools::helper::read_regular](../../src/helper.rs#L1209) |
| `execute_inner` | `root.directory.as_raw_fd` | [456](../../src/helper.rs#L456), [472](../../src/helper.rs#L472), [485](../../src/helper.rs#L485), [495](../../src/helper.rs#L495) | receiver-type-required |
| `execute_inner` | `Ok` | [457](../../src/helper.rs#L457), [473](../../src/helper.rs#L473), [500](../../src/helper.rs#L500), [513](../../src/helper.rs#L513), [540](../../src/helper.rs#L540) | external-constructor-callback-or-unresolved |
| `execute_inner` | `HelperValue::Read` | [457](../../src/helper.rs#L457) | external-constructor-callback-or-unresolved |
| `execute_inner` | `ByteString::from_bytes` | [458](../../src/helper.rs#L458) | [tools::helper::ByteString::from_bytes](../../src/helper.rs#L61) |
| `execute_inner` | `bytes.len` | [459](../../src/helper.rs#L459), [474](../../src/helper.rs#L474), [501](../../src/helper.rs#L501) | receiver-type-required |
| `execute_inner` | `sha256` | [460](../../src/helper.rs#L460), [475](../../src/helper.rs#L475), [486](../../src/helper.rs#L486), [502](../../src/helper.rs#L502) | [tools::helper::sha256](../../src/helper.rs#L1489) |
| `execute_inner` | `content.decode` | [470](../../src/helper.rs#L470) | receiver-type-required |
| `execute_inner` | `check_content_cap` | [471](../../src/helper.rs#L471), [493](../../src/helper.rs#L493) | [tools::helper::check_content_cap](../../src/helper.rs#L1479) |
| `execute_inner` | `atomic_replace` | [472](../../src/helper.rs#L472), [494](../../src/helper.rs#L494) | [tools::helper::atomic_replace](../../src/helper.rs#L1232) |
| `execute_inner` | `HelperValue::Write` | [473](../../src/helper.rs#L473) | external-constructor-callback-or-unresolved |
| `execute_inner` | `replacement.decode` | [492](../../src/helper.rs#L492) | receiver-type-required |
| `execute_inner` | `HelperValue::Patch` | [500](../../src/helper.rs#L500) | external-constructor-callback-or-unresolved |
| `execute_inner` | `checked_cap` | [510](../../src/helper.rs#L510) | [tools::helper::checked_cap](../../src/helper.rs#L1463) |
| `execute_inner` | `glob_beneath` | [512](../../src/helper.rs#L512) | [tools::helper::glob_beneath](../../src/helper.rs#L1336) |
| `execute_inner` | `native_grep::search(                     root,                     path,                     pattern,                     glob.as_deref(),                     output_mode,                     *case_insensitive,                     *context_lines,                     *stdout_bytes,                     *timeout_ms,                 )                 .map` | [527](../../src/helper.rs#L527) | receiver-type-required |
| `execute_inner` | `native_grep::search` | [527](../../src/helper.rs#L527) | [tools::helper::native_grep::search](../../src/helper/native_grep.rs#L8) |
| `execute_inner` | `glob.as_deref` | [531](../../src/helper.rs#L531) | receiver-type-required |
| `execute_inner` | `HelperValue::Exec` | [540](../../src/helper.rs#L540) | external-constructor-callback-or-unresolved |
| `execute_inner` | `self.exec` | [540](../../src/helper.rs#L540) | [tools::helper::HelperServer::exec](../../src/helper.rs#L553) |
| `root` | `self.roots.get(name).ok_or_else` | [545](../../src/helper.rs#L545) | receiver-type-required |
| `root` | `self.roots.get` | [545](../../src/helper.rs#L545) | receiver-type-required |
| `root` | `HelperError::new` | [546](../../src/helper.rs#L546) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `exec` | `request.argv.first().is_none_or` | [554](../../src/helper.rs#L554) | receiver-type-required |
| `exec` | `request.argv.first` | [554](../../src/helper.rs#L554) | receiver-type-required |
| `exec` | `Err` | [555](../../src/helper.rs#L555), [563](../../src/helper.rs#L563), [601](../../src/helper.rs#L601), [646](../../src/helper.rs#L646) | external-constructor-callback-or-unresolved |
| `exec` | `HelperError::new` | [555](../../src/helper.rs#L555), [563](../../src/helper.rs#L563), [619](../../src/helper.rs#L619), [628](../../src/helper.rs#L628), [632](../../src/helper.rs#L632), [646](../../src/helper.rs#L646), [655](../../src/helper.rs#L655), [659](../../src/helper.rs#L659) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `exec` | `checked_cap` | [560](../../src/helper.rs#L560), [561](../../src/helper.rs#L561) | [tools::helper::checked_cap](../../src/helper.rs#L1463) |
| `exec` | `Some` | [562](../../src/helper.rs#L562) | external-constructor-callback-or-unresolved |
| `exec` | `Command::new` | [568](../../src/helper.rs#L568) | external-constructor-callback-or-unresolved |
| `exec` | `command             .args(&request.argv[1..])             .env_clear()             .envs(&request.env)             .stdin(Stdio::piped())             .stdout(Stdio::piped())             .stderr` | [569](../../src/helper.rs#L569) | receiver-type-required |
| `exec` | `command             .args(&request.argv[1..])             .env_clear()             .envs(&request.env)             .stdin(Stdio::piped())             .stdout` | [569](../../src/helper.rs#L569) | receiver-type-required |
| `exec` | `command             .args(&request.argv[1..])             .env_clear()             .envs(&request.env)             .stdin` | [569](../../src/helper.rs#L569) | receiver-type-required |
| `exec` | `command             .args(&request.argv[1..])             .env_clear()             .envs` | [569](../../src/helper.rs#L569) | receiver-type-required |
| `exec` | `command             .args(&request.argv[1..])             .env_clear` | [569](../../src/helper.rs#L569) | receiver-type-required |
| `exec` | `command             .args` | [569](../../src/helper.rs#L569) | receiver-type-required |
| `exec` | `Stdio::piped` | [573](../../src/helper.rs#L573), [574](../../src/helper.rs#L574), [575](../../src/helper.rs#L575) | external-constructor-callback-or-unresolved |
| `exec` | `command.process_group` | [576](../../src/helper.rs#L576) | receiver-type-required |
| `exec` | `self.root` | [578](../../src/helper.rs#L578) | [tools::helper::HelperServer::root](../../src/helper.rs#L544) |
| `exec` | `cwd.path.is_empty` | [581](../../src/helper.rs#L581) | receiver-type-required |
| `exec` | `PathBuf::new` | [582](../../src/helper.rs#L582) | external-constructor-callback-or-unresolved |
| `exec` | `normalized_relative` | [584](../../src/helper.rs#L584) | [tools::helper::normalized_relative](../../src/helper.rs#L1057) |
| `exec` | `relative                 .components()                 .map(&#124;component&#124; match component {                     Component::Normal(name) => name.to_os_string(),                     _ => unreachable!("cwd was normalized"),                 })                 .collect::<Vec<_>>` | [586](../../src/helper.rs#L586) | receiver-type-required |
| `exec` | `relative                 .components()                 .map` | [586](../../src/helper.rs#L586) | receiver-type-required |
| `exec` | `relative                 .components` | [586](../../src/helper.rs#L586) | receiver-type-required |
| `exec` | `name.to_os_string` | [589](../../src/helper.rs#L589) | receiver-type-required |
| `exec` | `open_parent` | [593](../../src/helper.rs#L593) | [tools::helper::open_parent](../../src/helper.rs#L1102) |
| `exec` | `root.directory.as_raw_fd` | [593](../../src/helper.rs#L593) | receiver-type-required |
| `exec` | `command.pre_exec` | [599](../../src/helper.rs#L599) | receiver-type-required |
| `exec` | `libc::fchdir` | [600](../../src/helper.rs#L600) | external-constructor-callback-or-unresolved |
| `exec` | `directory.as_raw_fd` | [600](../../src/helper.rs#L600) | receiver-type-required |
| `exec` | `std::io::Error::last_os_error` | [601](../../src/helper.rs#L601) | external-constructor-callback-or-unresolved |
| `exec` | `Ok` | [603](../../src/helper.rs#L603), [661](../../src/helper.rs#L661) | external-constructor-callback-or-unresolved |
| `exec` | `command.spawn().map_err` | [608](../../src/helper.rs#L608) | receiver-type-required |
| `exec` | `command.spawn` | [608](../../src/helper.rs#L608) | receiver-type-required |
| `exec` | `HelperError::io` | [609](../../src/helper.rs#L609) | [tools::helper::HelperError::io](../../src/helper.rs#L247) |
| `exec` | `input.decode` | [614](../../src/helper.rs#L614) | receiver-type-required |
| `exec` | `check_content_cap` | [615](../../src/helper.rs#L615) | [tools::helper::check_content_cap](../../src/helper.rs#L1479) |
| `exec` | `child                 .stdin                 .take()                 .ok_or_else(&#124;&#124; HelperError::new(HelperErrorClass::Io, "child stdin missing"))?                 .write_all(&bytes)                 .map_err` | [616](../../src/helper.rs#L616) | receiver-type-required |
| `exec` | `child                 .stdin                 .take()                 .ok_or_else(&#124;&#124; HelperError::new(HelperErrorClass::Io, "child stdin missing"))?                 .write_all` | [616](../../src/helper.rs#L616) | receiver-type-required |
| `exec` | `child                 .stdin                 .take()                 .ok_or_else` | [616](../../src/helper.rs#L616) | receiver-type-required |
| `exec` | `child                 .stdin                 .take` | [616](../../src/helper.rs#L616) | receiver-type-required |
| `exec` | `drop` | [623](../../src/helper.rs#L623) | external-constructor-callback-or-unresolved |
| `exec` | `child.stdin.take` | [623](../../src/helper.rs#L623) | receiver-type-required |
| `exec` | `child             .stdout             .take()             .ok_or_else` | [625](../../src/helper.rs#L625) | receiver-type-required |
| `exec` | `child             .stdout             .take` | [625](../../src/helper.rs#L625) | receiver-type-required |
| `exec` | `child             .stderr             .take()             .ok_or_else` | [629](../../src/helper.rs#L629) | receiver-type-required |
| `exec` | `child             .stderr             .take` | [629](../../src/helper.rs#L629) | receiver-type-required |
| `exec` | `thread::spawn` | [633](../../src/helper.rs#L633), [634](../../src/helper.rs#L634) | external-constructor-callback-or-unresolved |
| `exec` | `read_capped` | [633](../../src/helper.rs#L633), [634](../../src/helper.rs#L634) | [tools::helper::read_capped](../../src/helper.rs#L1427) |
| `exec` | `request             .timeout_ms             .map` | [635](../../src/helper.rs#L635) | receiver-type-required |
| `exec` | `Instant::now` | [637](../../src/helper.rs#L637), [642](../../src/helper.rs#L642) | external-constructor-callback-or-unresolved |
| `exec` | `Duration::from_millis` | [637](../../src/helper.rs#L637), [651](../../src/helper.rs#L651) | external-constructor-callback-or-unresolved |
| `exec` | `child.try_wait().map_err` | [639](../../src/helper.rs#L639) | receiver-type-required |
| `exec` | `child.try_wait` | [639](../../src/helper.rs#L639) | receiver-type-required |
| `exec` | `deadline.is_some_and` | [642](../../src/helper.rs#L642) | receiver-type-required |
| `exec` | `terminate_then_kill` | [643](../../src/helper.rs#L643) | [tools::helper::terminate_then_kill](../../src/helper.rs#L1443) |
| `exec` | `stdout_thread.join` | [644](../../src/helper.rs#L644) | receiver-type-required |
| `exec` | `stderr_thread.join` | [645](../../src/helper.rs#L645) | receiver-type-required |
| `exec` | `thread::sleep` | [651](../../src/helper.rs#L651) | external-constructor-callback-or-unresolved |
| `exec` | `stdout_thread             .join()             .map_err(&#124;_&#124; HelperError::new(HelperErrorClass::Io, "stdout reader panicked"))?             .map_err` | [653](../../src/helper.rs#L653) | receiver-type-required |
| `exec` | `stdout_thread             .join()             .map_err` | [653](../../src/helper.rs#L653) | receiver-type-required |
| `exec` | `stdout_thread             .join` | [653](../../src/helper.rs#L653) | receiver-type-required |
| `exec` | `stderr_thread             .join()             .map_err(&#124;_&#124; HelperError::new(HelperErrorClass::Io, "stderr reader panicked"))?             .map_err` | [657](../../src/helper.rs#L657) | receiver-type-required |
| `exec` | `stderr_thread             .join()             .map_err` | [657](../../src/helper.rs#L657) | receiver-type-required |
| `exec` | `stderr_thread             .join` | [657](../../src/helper.rs#L657) | receiver-type-required |
| `exec` | `status.code().unwrap_or` | [662](../../src/helper.rs#L662) | receiver-type-required |
| `exec` | `status.code` | [662](../../src/helper.rs#L662) | receiver-type-required |
| `exec` | `ByteString::from_bytes` | [663](../../src/helper.rs#L663), [664](../../src/helper.rs#L664) | [tools::helper::ByteString::from_bytes](../../src/helper.rs#L61) |
| `sandboxed_with_scratch` | `policy.scratch.is_some` | [696](../../src/helper.rs#L696) | receiver-type-required |
| `sandboxed_with_scratch` | `Err` | [697](../../src/helper.rs#L697) | external-constructor-callback-or-unresolved |
| `sandboxed_with_scratch` | `HelperError::new` | [697](../../src/helper.rs#L697) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `sandboxed_with_scratch` | `tempfile::Builder::new()             .prefix("tekes-tool-")             .permissions(std::fs::Permissions::from_mode(0o700))             .tempdir` | [702](../../src/helper.rs#L702) | receiver-type-required |
| `sandboxed_with_scratch` | `tempfile::Builder::new()             .prefix("tekes-tool-")             .permissions` | [702](../../src/helper.rs#L702) | receiver-type-required |
| `sandboxed_with_scratch` | `tempfile::Builder::new()             .prefix` | [702](../../src/helper.rs#L702) | receiver-type-required |
| `sandboxed_with_scratch` | `tempfile::Builder::new` | [702](../../src/helper.rs#L702) | external-constructor-callback-or-unresolved |
| `sandboxed_with_scratch` | `std::fs::Permissions::from_mode` | [704](../../src/helper.rs#L704) | external-constructor-callback-or-unresolved |
| `sandboxed_with_scratch` | `Some` | [706](../../src/helper.rs#L706), [712](../../src/helper.rs#L712) | external-constructor-callback-or-unresolved |
| `sandboxed_with_scratch` | `std::fs::canonicalize(scratch.path())?                 .to_string_lossy()                 .into_owned` | [707](../../src/helper.rs#L707) | receiver-type-required |
| `sandboxed_with_scratch` | `std::fs::canonicalize(scratch.path())?                 .to_string_lossy` | [707](../../src/helper.rs#L707) | receiver-type-required |
| `sandboxed_with_scratch` | `std::fs::canonicalize` | [707](../../src/helper.rs#L707) | external-constructor-callback-or-unresolved |
| `sandboxed_with_scratch` | `scratch.path` | [707](../../src/helper.rs#L707) | receiver-type-required |
| `sandboxed_with_scratch` | `Self::sandboxed` | [711](../../src/helper.rs#L711) | [tools::helper::HelperClient::sandboxed](../../src/helper.rs#L723) |
| `sandboxed_with_scratch` | `std::sync::Arc::new` | [712](../../src/helper.rs#L712) | external-constructor-callback-or-unresolved |
| `sandboxed_with_scratch` | `Ok` | [713](../../src/helper.rs#L713) | external-constructor-callback-or-unresolved |
| `scratch_path` | `policy.scratch.as_deref` | [718](../../src/helper.rs#L718) | receiver-type-required |
| `sandboxed` | `policy.validate().map_err` | [729](../../src/helper.rs#L729) | receiver-type-required |
| `sandboxed` | `policy.validate` | [729](../../src/helper.rs#L729) | receiver-type-required |
| `sandboxed` | `HelperError::new` | [730](../../src/helper.rs#L730), [733](../../src/helper.rs#L733) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `sandboxed` | `error.to_string` | [730](../../src/helper.rs#L730) | receiver-type-required |
| `sandboxed` | `Err` | [733](../../src/helper.rs#L733) | external-constructor-callback-or-unresolved |
| `sandboxed` | `Ok` | [738](../../src/helper.rs#L738) | external-constructor-callback-or-unresolved |
| `sandboxed` | `executable.into` | [739](../../src/helper.rs#L739) | receiver-type-required |
| `approved_unsandboxed` | `policy_digest(policy).map_err` | [754](../../src/helper.rs#L754) | receiver-type-required |
| `approved_unsandboxed` | `policy_digest` | [754](../../src/helper.rs#L754) | [tools::sandbox::policy_digest](../../src/sandbox.rs#L116) |
| `approved_unsandboxed` | `HelperError::new` | [755](../../src/helper.rs#L755), [758](../../src/helper.rs#L758) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `approved_unsandboxed` | `error.to_string` | [755](../../src/helper.rs#L755), [758](../../src/helper.rs#L758) | receiver-type-required |
| `approved_unsandboxed` | `validate_unsandboxed_approval(approval, run, call, &digest)             .map_err` | [757](../../src/helper.rs#L757) | receiver-type-required |
| `approved_unsandboxed` | `validate_unsandboxed_approval` | [757](../../src/helper.rs#L757) | [tools::sandbox::validate_unsandboxed_approval](../../src/sandbox.rs#L263) |
| `approved_unsandboxed` | `Ok` | [759](../../src/helper.rs#L759) | external-constructor-callback-or-unresolved |
| `approved_unsandboxed` | `executable.into` | [760](../../src/helper.rs#L760) | receiver-type-required |
| `execute` | `self.execute_cancellable(             request,             Some(Duration::from_millis(HELPER_PROCESS_TIMEOUT_MS)),             &#124;&#124; false,         )         .map_err` | [768](../../src/helper.rs#L768) | receiver-type-required |
| `execute` | `self.execute_cancellable` | [768](../../src/helper.rs#L768) | [tools::helper::HelperClient::execute_cancellable](../../src/helper.rs#L784) |
| `execute` | `Some` | [770](../../src/helper.rs#L770) | external-constructor-callback-or-unresolved |
| `execute` | `Duration::from_millis` | [770](../../src/helper.rs#L770) | external-constructor-callback-or-unresolved |
| `execute` | `HelperError::new` | [776](../../src/helper.rs#L776) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `execute_cancellable` | `timeout.is_some_and` | [793](../../src/helper.rs#L793) | receiver-type-required |
| `execute_cancellable` | `timeout.is_zero` | [793](../../src/helper.rs#L793) | receiver-type-required |
| `execute_cancellable` | `Err` | [794](../../src/helper.rs#L794), [839](../../src/helper.rs#L839), [843](../../src/helper.rs#L843), [848](../../src/helper.rs#L848), [854](../../src/helper.rs#L854), [868](../../src/helper.rs#L868), [874](../../src/helper.rs#L874), [887](../../src/helper.rs#L887), [901](../../src/helper.rs#L901), [908](../../src/helper.rs#L908), [924](../../src/helper.rs#L924), [930](../../src/helper.rs#L930), [940](../../src/helper.rs#L940) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `HelperError::new(                 HelperErrorClass::InvalidRequest,                 "helper process timeout is outside hard limits",             )             .into` | [794](../../src/helper.rs#L794) | receiver-type-required |
| `execute_cancellable` | `HelperError::new` | [794](../../src/helper.rs#L794), [812](../../src/helper.rs#L812), [836](../../src/helper.rs#L836), [849](../../src/helper.rs#L849), [855](../../src/helper.rs#L855), [874](../../src/helper.rs#L874), [894](../../src/helper.rs#L894), [898](../../src/helper.rs#L898), [901](../../src/helper.rs#L901), [908](../../src/helper.rs#L908), [920](../../src/helper.rs#L920), [925](../../src/helper.rs#L925), [930](../../src/helper.rs#L930), [937](../../src/helper.rs#L937), [940](../../src/helper.rs#L940) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `execute_cancellable` | `encode_helper_line` | [800](../../src/helper.rs#L800), [808](../../src/helper.rs#L808) | [tools::helper::encode_helper_line](../../src/helper.rs#L305) |
| `execute_cancellable` | `HELPER_PROTOCOL.to_owned` | [803](../../src/helper.rs#L803) | receiver-type-required |
| `execute_cancellable` | `sandbox_command(policy, probe, &self.executable).map_err` | [811](../../src/helper.rs#L811) | receiver-type-required |
| `execute_cancellable` | `sandbox_command` | [811](../../src/helper.rs#L811) | [tools::sandbox::sandbox_command](../../src/sandbox.rs#L289) |
| `execute_cancellable` | `error.to_string` | [812](../../src/helper.rs#L812) | receiver-type-required |
| `execute_cancellable` | `Command::new` | [815](../../src/helper.rs#L815) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `command                 .arg("--root")                 .arg` | [818](../../src/helper.rs#L818) | receiver-type-required |
| `execute_cancellable` | `command                 .arg` | [818](../../src/helper.rs#L818) | receiver-type-required |
| `execute_cancellable` | `self.roots.first` | [823](../../src/helper.rs#L823) | receiver-type-required |
| `execute_cancellable` | `command.current_dir` | [824](../../src/helper.rs#L824) | receiver-type-required |
| `execute_cancellable` | `command.process_group` | [826](../../src/helper.rs#L826) | receiver-type-required |
| `execute_cancellable` | `command             .stdin(Stdio::piped())             .stdout(Stdio::piped())             .stderr(Stdio::piped())             .spawn()             .map_err` | [827](../../src/helper.rs#L827) | receiver-type-required |
| `execute_cancellable` | `command             .stdin(Stdio::piped())             .stdout(Stdio::piped())             .stderr(Stdio::piped())             .spawn` | [827](../../src/helper.rs#L827) | receiver-type-required |
| `execute_cancellable` | `command             .stdin(Stdio::piped())             .stdout(Stdio::piped())             .stderr` | [827](../../src/helper.rs#L827) | receiver-type-required |
| `execute_cancellable` | `command             .stdin(Stdio::piped())             .stdout` | [827](../../src/helper.rs#L827) | receiver-type-required |
| `execute_cancellable` | `command             .stdin` | [827](../../src/helper.rs#L827) | receiver-type-required |
| `execute_cancellable` | `Stdio::piped` | [828](../../src/helper.rs#L828), [829](../../src/helper.rs#L829), [830](../../src/helper.rs#L830) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `child             .stdin             .take()             .ok_or_else` | [833](../../src/helper.rs#L833) | receiver-type-required |
| `execute_cancellable` | `child             .stdin             .take` | [833](../../src/helper.rs#L833) | receiver-type-required |
| `execute_cancellable` | `stdin.write_all` | [837](../../src/helper.rs#L837), [841](../../src/helper.rs#L841) | receiver-type-required |
| `execute_cancellable` | `terminate_then_kill` | [838](../../src/helper.rs#L838), [842](../../src/helper.rs#L842), [847](../../src/helper.rs#L847), [853](../../src/helper.rs#L853), [865](../../src/helper.rs#L865), [871](../../src/helper.rs#L871), [884](../../src/helper.rs#L884) | [tools::helper::terminate_then_kill](../../src/helper.rs#L1443) |
| `execute_cancellable` | `HelperError::io(error).into` | [839](../../src/helper.rs#L839), [843](../../src/helper.rs#L843), [887](../../src/helper.rs#L887) | receiver-type-required |
| `execute_cancellable` | `HelperError::io` | [839](../../src/helper.rs#L839), [843](../../src/helper.rs#L843), [887](../../src/helper.rs#L887) | [tools::helper::HelperError::io](../../src/helper.rs#L247) |
| `execute_cancellable` | `drop` | [845](../../src/helper.rs#L845) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `child.stdout.take` | [846](../../src/helper.rs#L846) | receiver-type-required |
| `execute_cancellable` | `HelperError::new(HelperErrorClass::Protocol, "helper stdout missing").into` | [849](../../src/helper.rs#L849) | receiver-type-required |
| `execute_cancellable` | `child.stderr.take` | [852](../../src/helper.rs#L852) | receiver-type-required |
| `execute_cancellable` | `HelperError::new(HelperErrorClass::Protocol, "helper stderr missing").into` | [855](../../src/helper.rs#L855) | receiver-type-required |
| `execute_cancellable` | `thread::spawn` | [859](../../src/helper.rs#L859), [861](../../src/helper.rs#L861) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `read_capped` | [859](../../src/helper.rs#L859), [861](../../src/helper.rs#L861) | [tools::helper::read_capped](../../src/helper.rs#L1427) |
| `execute_cancellable` | `timeout.map` | [862](../../src/helper.rs#L862) | receiver-type-required |
| `execute_cancellable` | `Instant::now` | [862](../../src/helper.rs#L862), [870](../../src/helper.rs#L870) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `cancelled` | [864](../../src/helper.rs#L864) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `stdout_thread.join` | [866](../../src/helper.rs#L866), [872](../../src/helper.rs#L872), [885](../../src/helper.rs#L885) | receiver-type-required |
| `execute_cancellable` | `stderr_thread.join` | [867](../../src/helper.rs#L867), [873](../../src/helper.rs#L873), [886](../../src/helper.rs#L886) | receiver-type-required |
| `execute_cancellable` | `deadline.is_some_and` | [870](../../src/helper.rs#L870) | receiver-type-required |
| `execute_cancellable` | `HelperError::new(                     HelperErrorClass::Timeout,                     "helper process timed out",                 )                 .into` | [874](../../src/helper.rs#L874) | receiver-type-required |
| `execute_cancellable` | `child.try_wait` | [880](../../src/helper.rs#L880) | receiver-type-required |
| `execute_cancellable` | `thread::sleep` | [890](../../src/helper.rs#L890) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `Duration::from_millis` | [890](../../src/helper.rs#L890) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `stdout_thread             .join()             .map_err(&#124;_&#124; HelperError::new(HelperErrorClass::Io, "helper stdout reader panicked"))?             .map_err` | [892](../../src/helper.rs#L892) | receiver-type-required |
| `execute_cancellable` | `stdout_thread             .join()             .map_err` | [892](../../src/helper.rs#L892) | receiver-type-required |
| `execute_cancellable` | `stdout_thread             .join` | [892](../../src/helper.rs#L892) | receiver-type-required |
| `execute_cancellable` | `stderr_thread             .join()             .map_err(&#124;_&#124; HelperError::new(HelperErrorClass::Io, "helper stderr reader panicked"))?             .map_err` | [896](../../src/helper.rs#L896) | receiver-type-required |
| `execute_cancellable` | `stderr_thread             .join()             .map_err` | [896](../../src/helper.rs#L896) | receiver-type-required |
| `execute_cancellable` | `stderr_thread             .join` | [896](../../src/helper.rs#L896) | receiver-type-required |
| `execute_cancellable` | `HelperError::new(                 HelperErrorClass::Limit,                 "helper protocol output exceeded its cap",             )             .into` | [901](../../src/helper.rs#L901) | receiver-type-required |
| `execute_cancellable` | `status.success` | [907](../../src/helper.rs#L907) | receiver-type-required |
| `execute_cancellable` | `HelperError::new(                 HelperErrorClass::Protocol,                 format!(                     "helper exited {}: {}",                     status.code().unwrap_or(-1),                     String::from_utf8_lossy(&stderr)                 ),             )             .into` | [908](../../src/helper.rs#L908) | receiver-type-required |
| `execute_cancellable` | `stdout.split_inclusive` | [918](../../src/helper.rs#L918) | receiver-type-required |
| `execute_cancellable` | `lines.next().ok_or_else` | [919](../../src/helper.rs#L919), [936](../../src/helper.rs#L936) | receiver-type-required |
| `execute_cancellable` | `lines.next` | [919](../../src/helper.rs#L919), [936](../../src/helper.rs#L936), [939](../../src/helper.rs#L939) | receiver-type-required |
| `execute_cancellable` | `decode_helper_line` | [922](../../src/helper.rs#L922) | [tools::helper::decode_helper_line](../../src/helper.rs#L318) |
| `execute_cancellable` | `HelperError::new(HelperErrorClass::Protocol, "expected selected response").into` | [925](../../src/helper.rs#L925) | receiver-type-required |
| `execute_cancellable` | `from_payload` | [928](../../src/helper.rs#L928) | [tools::helper::from_payload](../../src/helper.rs#L1024) |
| `execute_cancellable` | `HelperError::new(                 HelperErrorClass::Protocol,                 "helper selected unsupported version",             )             .into` | [930](../../src/helper.rs#L930) | receiver-type-required |
| `execute_cancellable` | `lines.next().is_some` | [939](../../src/helper.rs#L939) | receiver-type-required |
| `execute_cancellable` | `HelperError::new(HelperErrorClass::Protocol, "extra helper output").into` | [940](../../src/helper.rs#L940) | receiver-type-required |
| `execute_cancellable` | `Ok` | [942](../../src/helper.rs#L942) | external-constructor-callback-or-unresolved |
| `execute_cancellable` | `decode_terminal` | [942](../../src/helper.rs#L942) | [tools::helper::decode_terminal](../../src/helper.rs#L1000) |
| `serve_stdio` | `std::io::stdin().lock` | [947](../../src/helper.rs#L947) | receiver-type-required |
| `serve_stdio` | `std::io::stdin` | [947](../../src/helper.rs#L947) | external-constructor-callback-or-unresolved |
| `serve_stdio` | `std::io::stdout().lock` | [948](../../src/helper.rs#L948) | receiver-type-required |
| `serve_stdio` | `std::io::stdout` | [948](../../src/helper.rs#L948) | external-constructor-callback-or-unresolved |
| `serve_stdio` | `read_line_limited` | [949](../../src/helper.rs#L949), [978](../../src/helper.rs#L978) | [tools::helper::read_line_limited](../../src/helper.rs#L1029) |
| `serve_stdio` | `decode_helper_line` | [950](../../src/helper.rs#L950), [979](../../src/helper.rs#L979) | [tools::helper::decode_helper_line](../../src/helper.rs#L318) |
| `serve_stdio` | `Err` | [952](../../src/helper.rs#L952), [966](../../src/helper.rs#L966), [981](../../src/helper.rs#L981) | external-constructor-callback-or-unresolved |
| `serve_stdio` | `HelperError::new` | [952](../../src/helper.rs#L952), [966](../../src/helper.rs#L966), [981](../../src/helper.rs#L981) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `serve_stdio` | `from_payload` | [957](../../src/helper.rs#L957), [986](../../src/helper.rs#L986) | [tools::helper::from_payload](../../src/helper.rs#L1024) |
| `serve_stdio` | `output.write_all` | [959](../../src/helper.rs#L959), [971](../../src/helper.rs#L971), [990](../../src/helper.rs#L990), [993](../../src/helper.rs#L993) | receiver-type-required |
| `serve_stdio` | `encode_helper_line` | [959](../../src/helper.rs#L959), [971](../../src/helper.rs#L971), [990](../../src/helper.rs#L990), [993](../../src/helper.rs#L993) | [tools::helper::encode_helper_line](../../src/helper.rs#L305) |
| `serve_stdio` | `"no mutual protocol version".to_owned` | [962](../../src/helper.rs#L962) | receiver-type-required |
| `serve_stdio` | `output.flush` | [965](../../src/helper.rs#L965), [977](../../src/helper.rs#L977), [996](../../src/helper.rs#L996) | receiver-type-required |
| `serve_stdio` | `server.execute` | [987](../../src/helper.rs#L987) | receiver-type-required |
| `serve_stdio` | `Ok` | [997](../../src/helper.rs#L997) | external-constructor-callback-or-unresolved |
| `decode_terminal` | `decode_helper_line` | [1001](../../src/helper.rs#L1001) | [tools::helper::decode_helper_line](../../src/helper.rs#L318) |
| `decode_terminal` | `key.as_str` | [1002](../../src/helper.rs#L1002) | receiver-type-required |
| `decode_terminal` | `from_payload` | [1004](../../src/helper.rs#L1004), [1011](../../src/helper.rs#L1011) | [tools::helper::from_payload](../../src/helper.rs#L1024) |
| `decode_terminal` | `Ok` | [1005](../../src/helper.rs#L1005), [1012](../../src/helper.rs#L1012) | external-constructor-callback-or-unresolved |
| `decode_terminal` | `Err` | [1017](../../src/helper.rs#L1017) | external-constructor-callback-or-unresolved |
| `decode_terminal` | `HelperError::new` | [1017](../../src/helper.rs#L1017) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `from_payload` | `serde_json::from_value(value)         .map_err` | [1025](../../src/helper.rs#L1025) | receiver-type-required |
| `from_payload` | `serde_json::from_value` | [1025](../../src/helper.rs#L1025) | external-constructor-callback-or-unresolved |
| `from_payload` | `HelperError::new` | [1026](../../src/helper.rs#L1026) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `from_payload` | `error.to_string` | [1026](../../src/helper.rs#L1026) | receiver-type-required |
| `read_line_limited` | `Vec::new` | [1030](../../src/helper.rs#L1030) | external-constructor-callback-or-unresolved |
| `read_line_limited` | `output.len` | [1032](../../src/helper.rs#L1032) | receiver-type-required |
| `read_line_limited` | `reader.read(&mut byte).map_err` | [1033](../../src/helper.rs#L1033) | receiver-type-required |
| `read_line_limited` | `reader.read` | [1033](../../src/helper.rs#L1033) | receiver-type-required |
| `read_line_limited` | `output.push` | [1037](../../src/helper.rs#L1037) | receiver-type-required |
| `read_line_limited` | `Ok` | [1039](../../src/helper.rs#L1039) | external-constructor-callback-or-unresolved |
| `read_line_limited` | `Err` | [1042](../../src/helper.rs#L1042) | external-constructor-callback-or-unresolved |
| `read_line_limited` | `HelperError::new` | [1042](../../src/helper.rs#L1042) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `valid_root_name` | `name.chars` | [1049](../../src/helper.rs#L1049) | receiver-type-required |
| `valid_root_name` | `chars         .next()         .is_some_and` | [1050](../../src/helper.rs#L1050) | receiver-type-required |
| `valid_root_name` | `chars         .next` | [1050](../../src/helper.rs#L1050) | receiver-type-required |
| `valid_root_name` | `value.is_ascii_alphabetic` | [1052](../../src/helper.rs#L1052) | receiver-type-required |
| `valid_root_name` | `name.len` | [1053](../../src/helper.rs#L1053) | receiver-type-required |
| `valid_root_name` | `chars.all` | [1054](../../src/helper.rs#L1054) | receiver-type-required |
| `valid_root_name` | `value.is_ascii_alphanumeric` | [1054](../../src/helper.rs#L1054) | receiver-type-required |
| `normalized_relative` | `Path::new` | [1058](../../src/helper.rs#L1058) | external-constructor-callback-or-unresolved |
| `normalized_relative` | `path.as_os_str().is_empty` | [1059](../../src/helper.rs#L1059) | receiver-type-required |
| `normalized_relative` | `path.as_os_str` | [1059](../../src/helper.rs#L1059) | receiver-type-required |
| `normalized_relative` | `path.is_absolute` | [1059](../../src/helper.rs#L1059) | receiver-type-required |
| `normalized_relative` | `Err` | [1060](../../src/helper.rs#L1060), [1070](../../src/helper.rs#L1070) | external-constructor-callback-or-unresolved |
| `normalized_relative` | `HelperError::new` | [1060](../../src/helper.rs#L1060), [1070](../../src/helper.rs#L1070) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `normalized_relative` | `PathBuf::new` | [1065](../../src/helper.rs#L1065) | external-constructor-callback-or-unresolved |
| `normalized_relative` | `path.components` | [1066](../../src/helper.rs#L1066) | receiver-type-required |
| `normalized_relative` | `value.as_bytes().contains` | [1068](../../src/helper.rs#L1068) | receiver-type-required |
| `normalized_relative` | `value.as_bytes` | [1068](../../src/helper.rs#L1068) | receiver-type-required |
| `normalized_relative` | `result.push` | [1068](../../src/helper.rs#L1068) | receiver-type-required |
| `normalized_relative` | `Ok` | [1077](../../src/helper.rs#L1077) | external-constructor-callback-or-unresolved |
| `split_parent` | `normalized_relative` | [1081](../../src/helper.rs#L1081) | [tools::helper::normalized_relative](../../src/helper.rs#L1057) |
| `split_parent` | `normalized.components().collect::<Vec<_>>` | [1082](../../src/helper.rs#L1082) | receiver-type-required |
| `split_parent` | `normalized.components` | [1082](../../src/helper.rs#L1082) | receiver-type-required |
| `split_parent` | `components.pop` | [1083](../../src/helper.rs#L1083) | receiver-type-required |
| `split_parent` | `value.to_os_string` | [1084](../../src/helper.rs#L1084), [1095](../../src/helper.rs#L1095) | receiver-type-required |
| `split_parent` | `Err` | [1086](../../src/helper.rs#L1086) | external-constructor-callback-or-unresolved |
| `split_parent` | `HelperError::new` | [1086](../../src/helper.rs#L1086) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `split_parent` | `components         .into_iter()         .map(&#124;component&#124; match component {             Component::Normal(value) => value.to_os_string(),             _ => unreachable!("normalized components"),         })         .collect` | [1092](../../src/helper.rs#L1092) | receiver-type-required |
| `split_parent` | `components         .into_iter()         .map` | [1092](../../src/helper.rs#L1092) | receiver-type-required |
| `split_parent` | `components         .into_iter` | [1092](../../src/helper.rs#L1092) | receiver-type-required |
| `split_parent` | `Ok` | [1099](../../src/helper.rs#L1099) | external-constructor-callback-or-unresolved |
| `open_parent` | `duplicate_fd` | [1103](../../src/helper.rs#L1103) | [tools::helper::duplicate_fd](../../src/helper.rs#L1137) |
| `open_parent` | `open_directory_at` | [1105](../../src/helper.rs#L1105) | [tools::helper::open_directory_at](../../src/helper.rs#L1147) |
| `open_parent` | `current.as_raw_fd` | [1105](../../src/helper.rs#L1105) | receiver-type-required |
| `open_parent` | `component.as_os_str` | [1105](../../src/helper.rs#L1105) | receiver-type-required |
| `open_parent` | `Ok` | [1107](../../src/helper.rs#L1107) | external-constructor-callback-or-unresolved |
| `create_parent` | `duplicate_fd` | [1111](../../src/helper.rs#L1111) | [tools::helper::duplicate_fd](../../src/helper.rs#L1137) |
| `create_parent` | `open_directory_at` | [1113](../../src/helper.rs#L1113), [1127](../../src/helper.rs#L1127) | [tools::helper::open_directory_at](../../src/helper.rs#L1147) |
| `create_parent` | `current.as_raw_fd` | [1113](../../src/helper.rs#L1113), [1120](../../src/helper.rs#L1120), [1127](../../src/helper.rs#L1127) | receiver-type-required |
| `create_parent` | `component.as_os_str` | [1113](../../src/helper.rs#L1113), [1127](../../src/helper.rs#L1127) | receiver-type-required |
| `create_parent` | `CString::new(component.as_bytes()).map_err` | [1116](../../src/helper.rs#L1116) | receiver-type-required |
| `create_parent` | `CString::new` | [1116](../../src/helper.rs#L1116) | external-constructor-callback-or-unresolved |
| `create_parent` | `component.as_bytes` | [1116](../../src/helper.rs#L1116) | receiver-type-required |
| `create_parent` | `HelperError::new` | [1117](../../src/helper.rs#L1117) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `create_parent` | `libc::mkdirat` | [1120](../../src/helper.rs#L1120) | external-constructor-callback-or-unresolved |
| `create_parent` | `name.as_ptr` | [1120](../../src/helper.rs#L1120) | receiver-type-required |
| `create_parent` | `std::io::Error::last_os_error` | [1121](../../src/helper.rs#L1121) | external-constructor-callback-or-unresolved |
| `create_parent` | `error.raw_os_error` | [1122](../../src/helper.rs#L1122) | receiver-type-required |
| `create_parent` | `Some` | [1122](../../src/helper.rs#L1122) | external-constructor-callback-or-unresolved |
| `create_parent` | `Err` | [1123](../../src/helper.rs#L1123), [1131](../../src/helper.rs#L1131) | external-constructor-callback-or-unresolved |
| `create_parent` | `classify_open` | [1123](../../src/helper.rs#L1123) | [tools::helper::classify_open](../../src/helper.rs#L1201) |
| `create_parent` | `current.sync_all().map_err` | [1128](../../src/helper.rs#L1128) | receiver-type-required |
| `create_parent` | `current.sync_all` | [1128](../../src/helper.rs#L1128) | receiver-type-required |
| `create_parent` | `Ok` | [1134](../../src/helper.rs#L1134) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `libc::fcntl` | [1139](../../src/helper.rs#L1139) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `Err` | [1141](../../src/helper.rs#L1141) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `HelperError::io` | [1141](../../src/helper.rs#L1141) | [tools::helper::HelperError::io](../../src/helper.rs#L247) |
| `duplicate_fd` | `std::io::Error::last_os_error` | [1141](../../src/helper.rs#L1141) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `Ok` | [1144](../../src/helper.rs#L1144) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `File::from_raw_fd` | [1144](../../src/helper.rs#L1144) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `CString::new(name.as_bytes())         .map_err` | [1148](../../src/helper.rs#L1148) | receiver-type-required |
| `open_directory_at` | `CString::new` | [1148](../../src/helper.rs#L1148) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `name.as_bytes` | [1148](../../src/helper.rs#L1148) | receiver-type-required |
| `open_directory_at` | `HelperError::new` | [1149](../../src/helper.rs#L1149) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `open_directory_at` | `libc::openat` | [1152](../../src/helper.rs#L1152) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `name.as_ptr` | [1154](../../src/helper.rs#L1154) | receiver-type-required |
| `open_directory_at` | `Err` | [1163](../../src/helper.rs#L1163) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `classify_open` | [1163](../../src/helper.rs#L1163) | [tools::helper::classify_open](../../src/helper.rs#L1201) |
| `open_directory_at` | `std::io::Error::last_os_error` | [1163](../../src/helper.rs#L1163) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `Ok` | [1166](../../src/helper.rs#L1166) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `File::from_raw_fd` | [1166](../../src/helper.rs#L1166) | external-constructor-callback-or-unresolved |
| `open_regular_at` | `CString::new(leaf.as_bytes())         .map_err` | [1175](../../src/helper.rs#L1175) | receiver-type-required |
| `open_regular_at` | `CString::new` | [1175](../../src/helper.rs#L1175) | external-constructor-callback-or-unresolved |
| `open_regular_at` | `leaf.as_bytes` | [1175](../../src/helper.rs#L1175) | receiver-type-required |
| `open_regular_at` | `HelperError::new` | [1176](../../src/helper.rs#L1176), [1193](../../src/helper.rs#L1193) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `open_regular_at` | `libc::openat` | [1179](../../src/helper.rs#L1179) | external-constructor-callback-or-unresolved |
| `open_regular_at` | `leaf.as_ptr` | [1181](../../src/helper.rs#L1181) | receiver-type-required |
| `open_regular_at` | `Err` | [1187](../../src/helper.rs#L1187), [1193](../../src/helper.rs#L1193) | external-constructor-callback-or-unresolved |
| `open_regular_at` | `classify_open` | [1187](../../src/helper.rs#L1187) | [tools::helper::classify_open](../../src/helper.rs#L1201) |
| `open_regular_at` | `std::io::Error::last_os_error` | [1187](../../src/helper.rs#L1187) | external-constructor-callback-or-unresolved |
| `open_regular_at` | `File::from_raw_fd` | [1190](../../src/helper.rs#L1190) | external-constructor-callback-or-unresolved |
| `open_regular_at` | `file.metadata().map_err` | [1191](../../src/helper.rs#L1191) | receiver-type-required |
| `open_regular_at` | `file.metadata` | [1191](../../src/helper.rs#L1191) | receiver-type-required |
| `open_regular_at` | `metadata.file_type().is_file` | [1192](../../src/helper.rs#L1192) | receiver-type-required |
| `open_regular_at` | `metadata.file_type` | [1192](../../src/helper.rs#L1192) | receiver-type-required |
| `open_regular_at` | `Ok` | [1198](../../src/helper.rs#L1198) | external-constructor-callback-or-unresolved |
| `classify_open` | `error.raw_os_error` | [1202](../../src/helper.rs#L1202) | receiver-type-required |
| `classify_open` | `HelperError::new` | [1203](../../src/helper.rs#L1203), [1204](../../src/helper.rs#L1204) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `classify_open` | `error.to_string` | [1203](../../src/helper.rs#L1203), [1204](../../src/helper.rs#L1204) | receiver-type-required |
| `classify_open` | `HelperError::io` | [1205](../../src/helper.rs#L1205) | [tools::helper::HelperError::io](../../src/helper.rs#L247) |
| `read_regular` | `split_parent` | [1210](../../src/helper.rs#L1210) | [tools::helper::split_parent](../../src/helper.rs#L1080) |
| `read_regular` | `open_parent` | [1211](../../src/helper.rs#L1211) | [tools::helper::open_parent](../../src/helper.rs#L1102) |
| `read_regular` | `open_regular_at` | [1212](../../src/helper.rs#L1212) | [tools::helper::open_regular_at](../../src/helper.rs#L1169) |
| `read_regular` | `parent.as_raw_fd` | [1212](../../src/helper.rs#L1212) | receiver-type-required |
| `read_regular` | `file.metadata().map_err(HelperError::io)?.len` | [1213](../../src/helper.rs#L1213) | receiver-type-required |
| `read_regular` | `file.metadata().map_err` | [1213](../../src/helper.rs#L1213) | receiver-type-required |
| `read_regular` | `file.metadata` | [1213](../../src/helper.rs#L1213) | receiver-type-required |
| `read_regular` | `Err` | [1214](../../src/helper.rs#L1214), [1224](../../src/helper.rs#L1224) | external-constructor-callback-or-unresolved |
| `read_regular` | `HelperError::new` | [1214](../../src/helper.rs#L1214), [1224](../../src/helper.rs#L1224) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `read_regular` | `Vec::new` | [1219](../../src/helper.rs#L1219) | external-constructor-callback-or-unresolved |
| `read_regular` | `file.take((cap as u64) + 1)         .read_to_end(&mut bytes)         .map_err` | [1220](../../src/helper.rs#L1220) | receiver-type-required |
| `read_regular` | `file.take((cap as u64) + 1)         .read_to_end` | [1220](../../src/helper.rs#L1220) | receiver-type-required |
| `read_regular` | `file.take` | [1220](../../src/helper.rs#L1220) | receiver-type-required |
| `read_regular` | `bytes.len` | [1223](../../src/helper.rs#L1223) | receiver-type-required |
| `read_regular` | `Ok` | [1229](../../src/helper.rs#L1229) | external-constructor-callback-or-unresolved |
| `atomic_replace` | `split_parent` | [1238](../../src/helper.rs#L1238) | [tools::helper::split_parent](../../src/helper.rs#L1080) |
| `atomic_replace` | `create_parent` | [1240](../../src/helper.rs#L1240) | [tools::helper::create_parent](../../src/helper.rs#L1110) |
| `atomic_replace` | `open_parent` | [1242](../../src/helper.rs#L1242) | [tools::helper::open_parent](../../src/helper.rs#L1102) |
| `atomic_replace` | `open_regular_at` | [1244](../../src/helper.rs#L1244), [1260](../../src/helper.rs#L1260) | [tools::helper::open_regular_at](../../src/helper.rs#L1169) |
| `atomic_replace` | `parent.as_raw_fd` | [1244](../../src/helper.rs#L1244), [1261](../../src/helper.rs#L1261), [1270](../../src/helper.rs#L1270), [1279](../../src/helper.rs#L1279) | receiver-type-required |
| `atomic_replace` | `Err` | [1247](../../src/helper.rs#L1247), [1253](../../src/helper.rs#L1253), [1255](../../src/helper.rs#L1255) | external-constructor-callback-or-unresolved |
| `atomic_replace` | `HelperError::new` | [1247](../../src/helper.rs#L1247) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `atomic_replace` | `error.clone` | [1253](../../src/helper.rs#L1253), [1255](../../src/helper.rs#L1255) | receiver-type-required |
| `atomic_replace` | `OsStr::new` | [1259](../../src/helper.rs#L1259) | external-constructor-callback-or-unresolved |
| `atomic_replace` | `(&#124;&#124; {         temp.write_all(bytes).map_err(HelperError::io)?;         FullSync::full_sync(&temp).map_err(HelperError::io)?;         rename_at(             parent.as_raw_fd(),             temp_os,             &leaf,             create == CreateMode::New,         )?;         FullSync::full_sync(&parent).map_err(HelperError::io)?;         Ok(())     })` | [1266](../../src/helper.rs#L1266) | external-constructor-callback-or-unresolved |
| `atomic_replace` | `temp.write_all(bytes).map_err` | [1267](../../src/helper.rs#L1267) | receiver-type-required |
| `atomic_replace` | `temp.write_all` | [1267](../../src/helper.rs#L1267) | receiver-type-required |
| `atomic_replace` | `FullSync::full_sync(&temp).map_err` | [1268](../../src/helper.rs#L1268) | receiver-type-required |
| `atomic_replace` | `FullSync::full_sync` | [1268](../../src/helper.rs#L1268), [1275](../../src/helper.rs#L1275) | [store::platform::FullSync::full_sync](../../../store/src/platform.rs#L31) |
| `atomic_replace` | `rename_at` | [1269](../../src/helper.rs#L1269) | [tools::helper::rename_at](../../src/helper.rs#L1284) |
| `atomic_replace` | `FullSync::full_sync(&parent).map_err` | [1275](../../src/helper.rs#L1275) | receiver-type-required |
| `atomic_replace` | `Ok` | [1276](../../src/helper.rs#L1276) | external-constructor-callback-or-unresolved |
| `atomic_replace` | `result.is_err` | [1278](../../src/helper.rs#L1278) | receiver-type-required |
| `atomic_replace` | `unlink_at` | [1279](../../src/helper.rs#L1279) | [tools::helper::unlink_at](../../src/helper.rs#L1326) |
| `rename_at` | `CString::new(source.as_bytes())         .map_err` | [1290](../../src/helper.rs#L1290) | receiver-type-required |
| `rename_at` | `CString::new` | [1290](../../src/helper.rs#L1290), [1292](../../src/helper.rs#L1292) | external-constructor-callback-or-unresolved |
| `rename_at` | `source.as_bytes` | [1290](../../src/helper.rs#L1290) | receiver-type-required |
| `rename_at` | `HelperError::new` | [1291](../../src/helper.rs#L1291), [1293](../../src/helper.rs#L1293) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `rename_at` | `CString::new(destination.as_bytes())         .map_err` | [1292](../../src/helper.rs#L1292) | receiver-type-required |
| `rename_at` | `destination.as_bytes` | [1292](../../src/helper.rs#L1292) | receiver-type-required |
| `rename_at` | `libc::renameatx_np` | [1297](../../src/helper.rs#L1297) | external-constructor-callback-or-unresolved |
| `rename_at` | `source.as_ptr` | [1299](../../src/helper.rs#L1299), [1309](../../src/helper.rs#L1309), [1312](../../src/helper.rs#L1312), [1318](../../src/helper.rs#L1318) | receiver-type-required |
| `rename_at` | `destination.as_ptr` | [1301](../../src/helper.rs#L1301), [1309](../../src/helper.rs#L1309), [1318](../../src/helper.rs#L1318) | receiver-type-required |
| `rename_at` | `libc::linkat` | [1309](../../src/helper.rs#L1309) | external-constructor-callback-or-unresolved |
| `rename_at` | `libc::unlinkat` | [1312](../../src/helper.rs#L1312) | external-constructor-callback-or-unresolved |
| `rename_at` | `libc::renameat` | [1318](../../src/helper.rs#L1318) | external-constructor-callback-or-unresolved |
| `rename_at` | `Err` | [1321](../../src/helper.rs#L1321) | external-constructor-callback-or-unresolved |
| `rename_at` | `HelperError::io` | [1321](../../src/helper.rs#L1321) | [tools::helper::HelperError::io](../../src/helper.rs#L247) |
| `rename_at` | `std::io::Error::last_os_error` | [1321](../../src/helper.rs#L1321) | external-constructor-callback-or-unresolved |
| `rename_at` | `Ok` | [1323](../../src/helper.rs#L1323) | external-constructor-callback-or-unresolved |
| `unlink_at` | `CString::new(name.as_bytes())         .map_err` | [1327](../../src/helper.rs#L1327) | receiver-type-required |
| `unlink_at` | `CString::new` | [1327](../../src/helper.rs#L1327) | external-constructor-callback-or-unresolved |
| `unlink_at` | `name.as_bytes` | [1327](../../src/helper.rs#L1327) | receiver-type-required |
| `unlink_at` | `HelperError::new` | [1328](../../src/helper.rs#L1328) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `unlink_at` | `libc::unlinkat` | [1330](../../src/helper.rs#L1330) | external-constructor-callback-or-unresolved |
| `unlink_at` | `name.as_ptr` | [1330](../../src/helper.rs#L1330) | receiver-type-required |
| `unlink_at` | `Err` | [1331](../../src/helper.rs#L1331) | external-constructor-callback-or-unresolved |
| `unlink_at` | `HelperError::io` | [1331](../../src/helper.rs#L1331) | [tools::helper::HelperError::io](../../src/helper.rs#L247) |
| `unlink_at` | `std::io::Error::last_os_error` | [1331](../../src/helper.rs#L1331) | external-constructor-callback-or-unresolved |
| `unlink_at` | `Ok` | [1333](../../src/helper.rs#L1333) | external-constructor-callback-or-unresolved |
| `glob_beneath` | `pattern.is_empty` | [1337](../../src/helper.rs#L1337) | receiver-type-required |
| `glob_beneath` | `Path::new(pattern).is_absolute` | [1337](../../src/helper.rs#L1337) | receiver-type-required |
| `glob_beneath` | `Path::new` | [1337](../../src/helper.rs#L1337) | external-constructor-callback-or-unresolved |
| `glob_beneath` | `pattern.contains` | [1337](../../src/helper.rs#L1337) | receiver-type-required |
| `glob_beneath` | `Err` | [1338](../../src/helper.rs#L1338), [1351](../../src/helper.rs#L1351) | external-constructor-callback-or-unresolved |
| `glob_beneath` | `HelperError::new` | [1338](../../src/helper.rs#L1338), [1351](../../src/helper.rs#L1351), [1363](../../src/helper.rs#L1363) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `glob_beneath` | `Vec::new` | [1344](../../src/helper.rs#L1344) | external-constructor-callback-or-unresolved |
| `glob_beneath` | `stack.pop` | [1346](../../src/helper.rs#L1346) | receiver-type-required |
| `glob_beneath` | `std::fs::read_dir(&directory).map_err` | [1347](../../src/helper.rs#L1347) | receiver-type-required |
| `glob_beneath` | `std::fs::read_dir` | [1347](../../src/helper.rs#L1347) | external-constructor-callback-or-unresolved |
| `glob_beneath` | `entry.map_err` | [1348](../../src/helper.rs#L1348) | receiver-type-required |
| `glob_beneath` | `scanned.saturating_add` | [1349](../../src/helper.rs#L1349) | receiver-type-required |
| `glob_beneath` | `entry.file_type().map_err` | [1356](../../src/helper.rs#L1356) | receiver-type-required |
| `glob_beneath` | `entry.file_type` | [1356](../../src/helper.rs#L1356) | receiver-type-required |
| `glob_beneath` | `file_type.is_symlink` | [1357](../../src/helper.rs#L1357) | receiver-type-required |
| `glob_beneath` | `entry                 .path()                 .strip_prefix(root)                 .map_err(&#124;_&#124; HelperError::new(HelperErrorClass::PathEscape, "glob escaped root"))?                 .to_string_lossy()                 .replace` | [1360](../../src/helper.rs#L1360) | receiver-type-required |
| `glob_beneath` | `entry                 .path()                 .strip_prefix(root)                 .map_err(&#124;_&#124; HelperError::new(HelperErrorClass::PathEscape, "glob escaped root"))?                 .to_string_lossy` | [1360](../../src/helper.rs#L1360) | receiver-type-required |
| `glob_beneath` | `entry                 .path()                 .strip_prefix(root)                 .map_err` | [1360](../../src/helper.rs#L1360) | receiver-type-required |
| `glob_beneath` | `entry                 .path()                 .strip_prefix` | [1360](../../src/helper.rs#L1360) | receiver-type-required |
| `glob_beneath` | `entry                 .path` | [1360](../../src/helper.rs#L1360) | receiver-type-required |
| `glob_beneath` | `glob_match` | [1366](../../src/helper.rs#L1366) | [tools::helper::glob_match](../../src/helper.rs#L1387) |
| `glob_beneath` | `pattern.as_bytes` | [1366](../../src/helper.rs#L1366) | receiver-type-required |
| `glob_beneath` | `relative.as_bytes` | [1366](../../src/helper.rs#L1366) | receiver-type-required |
| `glob_beneath` | `entry                     .metadata()                     .and_then(&#124;metadata&#124; metadata.modified())                     .unwrap_or` | [1367](../../src/helper.rs#L1367) | receiver-type-required |
| `glob_beneath` | `entry                     .metadata()                     .and_then` | [1367](../../src/helper.rs#L1367) | receiver-type-required |
| `glob_beneath` | `entry                     .metadata` | [1367](../../src/helper.rs#L1367) | receiver-type-required |
| `glob_beneath` | `metadata.modified` | [1369](../../src/helper.rs#L1369) | receiver-type-required |
| `glob_beneath` | `result.push` | [1371](../../src/helper.rs#L1371) | receiver-type-required |
| `glob_beneath` | `relative.clone` | [1371](../../src/helper.rs#L1371) | receiver-type-required |
| `glob_beneath` | `file_type.is_dir` | [1373](../../src/helper.rs#L1373) | receiver-type-required |
| `glob_beneath` | `stack.push` | [1374](../../src/helper.rs#L1374) | receiver-type-required |
| `glob_beneath` | `entry.path` | [1374](../../src/helper.rs#L1374) | receiver-type-required |
| `glob_beneath` | `result.sort_by` | [1378](../../src/helper.rs#L1378) | receiver-type-required |
| `glob_beneath` | `right_time             .cmp(left_time)             .then_with` | [1379](../../src/helper.rs#L1379) | receiver-type-required |
| `glob_beneath` | `right_time             .cmp` | [1379](../../src/helper.rs#L1379) | receiver-type-required |
| `glob_beneath` | `left_path.as_bytes().cmp` | [1381](../../src/helper.rs#L1381) | receiver-type-required |
| `glob_beneath` | `left_path.as_bytes` | [1381](../../src/helper.rs#L1381) | receiver-type-required |
| `glob_beneath` | `right_path.as_bytes` | [1381](../../src/helper.rs#L1381) | receiver-type-required |
| `glob_beneath` | `result.truncate` | [1383](../../src/helper.rs#L1383) | receiver-type-required |
| `glob_beneath` | `Ok` | [1384](../../src/helper.rs#L1384) | external-constructor-callback-or-unresolved |
| `glob_beneath` | `result.into_iter().map(&#124;(_, path)&#124; path).collect` | [1384](../../src/helper.rs#L1384) | receiver-type-required |
| `glob_beneath` | `result.into_iter().map` | [1384](../../src/helper.rs#L1384) | receiver-type-required |
| `glob_beneath` | `result.into_iter` | [1384](../../src/helper.rs#L1384) | receiver-type-required |
| `glob_match` | `matches` | [1424](../../src/helper.rs#L1424) | external-constructor-callback-or-unresolved |
| `glob_match` | `BTreeMap::new` | [1424](../../src/helper.rs#L1424) | external-constructor-callback-or-unresolved |
| `matches` | `memo.get` | [1395](../../src/helper.rs#L1395) | receiver-type-required |
| `matches` | `pattern.len` | [1398](../../src/helper.rs#L1398) | receiver-type-required |
| `matches` | `text.len` | [1399](../../src/helper.rs#L1399), [1402](../../src/helper.rs#L1402), [1406](../../src/helper.rs#L1406), [1410](../../src/helper.rs#L1410), [1413](../../src/helper.rs#L1413) | receiver-type-required |
| `matches` | `pattern[pattern_index..].starts_with` | [1400](../../src/helper.rs#L1400), [1404](../../src/helper.rs#L1404) | receiver-type-required |
| `matches` | `matches` | [1401](../../src/helper.rs#L1401), [1403](../../src/helper.rs#L1403), [1405](../../src/helper.rs#L1405), [1407](../../src/helper.rs#L1407), [1409](../../src/helper.rs#L1409), [1412](../../src/helper.rs#L1412), [1417](../../src/helper.rs#L1417) | [tools::helper::glob_match::matches](../../src/helper.rs#L1388) |
| `matches` | `memo.insert` | [1421](../../src/helper.rs#L1421) | receiver-type-required |
| `read_capped` | `Vec::with_capacity` | [1428](../../src/helper.rs#L1428) | external-constructor-callback-or-unresolved |
| `read_capped` | `cap.min` | [1428](../../src/helper.rs#L1428) | receiver-type-required |
| `read_capped` | `reader.read` | [1432](../../src/helper.rs#L1432) | receiver-type-required |
| `read_capped` | `cap.saturating_sub` | [1436](../../src/helper.rs#L1436) | receiver-type-required |
| `read_capped` | `output.len` | [1436](../../src/helper.rs#L1436) | receiver-type-required |
| `read_capped` | `output.extend_from_slice` | [1437](../../src/helper.rs#L1437) | receiver-type-required |
| `read_capped` | `count.min` | [1437](../../src/helper.rs#L1437) | receiver-type-required |
| `read_capped` | `Ok` | [1440](../../src/helper.rs#L1440) | external-constructor-callback-or-unresolved |
| `terminate_then_kill` | `child.id` | [1444](../../src/helper.rs#L1444) | receiver-type-required |
| `terminate_then_kill` | `libc::kill` | [1447](../../src/helper.rs#L1447), [1458](../../src/helper.rs#L1458) | external-constructor-callback-or-unresolved |
| `terminate_then_kill` | `Instant::now` | [1449](../../src/helper.rs#L1449), [1450](../../src/helper.rs#L1450) | external-constructor-callback-or-unresolved |
| `terminate_then_kill` | `Duration::from_millis` | [1449](../../src/helper.rs#L1449), [1454](../../src/helper.rs#L1454) | external-constructor-callback-or-unresolved |
| `terminate_then_kill` | `child.try_wait().ok().flatten().is_some` | [1451](../../src/helper.rs#L1451) | receiver-type-required |
| `terminate_then_kill` | `child.try_wait().ok().flatten` | [1451](../../src/helper.rs#L1451) | receiver-type-required |
| `terminate_then_kill` | `child.try_wait().ok` | [1451](../../src/helper.rs#L1451) | receiver-type-required |
| `terminate_then_kill` | `child.try_wait` | [1451](../../src/helper.rs#L1451) | receiver-type-required |
| `terminate_then_kill` | `thread::sleep` | [1454](../../src/helper.rs#L1454) | external-constructor-callback-or-unresolved |
| `terminate_then_kill` | `child.wait` | [1460](../../src/helper.rs#L1460) | receiver-type-required |
| `checked_cap` | `checked_cap_with` | [1464](../../src/helper.rs#L1464) | [tools::helper::checked_cap_with](../../src/helper.rs#L1467) |
| `checked_cap_with` | `usize::try_from(value)         .map_err` | [1468](../../src/helper.rs#L1468) | receiver-type-required |
| `checked_cap_with` | `usize::try_from` | [1468](../../src/helper.rs#L1468) | external-constructor-callback-or-unresolved |
| `checked_cap_with` | `HelperError::new` | [1469](../../src/helper.rs#L1469), [1471](../../src/helper.rs#L1471) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `checked_cap_with` | `Err` | [1471](../../src/helper.rs#L1471) | external-constructor-callback-or-unresolved |
| `checked_cap_with` | `Ok` | [1476](../../src/helper.rs#L1476) | external-constructor-callback-or-unresolved |
| `check_content_cap` | `bytes.len` | [1480](../../src/helper.rs#L1480) | receiver-type-required |
| `check_content_cap` | `Err` | [1481](../../src/helper.rs#L1481) | external-constructor-callback-or-unresolved |
| `check_content_cap` | `HelperError::new` | [1481](../../src/helper.rs#L1481) | [tools::helper::HelperError::new](../../src/helper.rs#L239) |
| `check_content_cap` | `Ok` | [1486](../../src/helper.rs#L1486) | external-constructor-callback-or-unresolved |
| `monotonic_nonce` | `NONCE.fetch_add` | [1496](../../src/helper.rs#L1496) | receiver-type-required |
| `NONCE` | `AtomicU64::new` | [1495](../../src/helper.rs#L1495) | external-constructor-callback-or-unresolved |
| `create_builds_parents_and_preserves_no_replace` | `tempfile::tempdir().unwrap` | [1506](../../src/helper.rs#L1506) | receiver-type-required |
| `create_builds_parents_and_preserves_no_replace` | `tempfile::tempdir` | [1506](../../src/helper.rs#L1506) | external-constructor-callback-or-unresolved |
| `create_builds_parents_and_preserves_no_replace` | `File::open(root.path()).unwrap` | [1507](../../src/helper.rs#L1507) | receiver-type-required |
| `create_builds_parents_and_preserves_no_replace` | `File::open` | [1507](../../src/helper.rs#L1507) | external-constructor-callback-or-unresolved |
| `create_builds_parents_and_preserves_no_replace` | `root.path` | [1507](../../src/helper.rs#L1507) | receiver-type-required |
| `create_builds_parents_and_preserves_no_replace` | `atomic_replace(             directory.as_raw_fd(),             "docs/nested/file.md",             b"LIVE_OK\n",             CreateMode::New,         )         .unwrap` | [1508](../../src/helper.rs#L1508) | receiver-type-required |
| `create_builds_parents_and_preserves_no_replace` | `atomic_replace` | [1508](../../src/helper.rs#L1508) | external-constructor-callback-or-unresolved |
| `create_builds_parents_and_preserves_no_replace` | `directory.as_raw_fd` | [1509](../../src/helper.rs#L1509) | receiver-type-required |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `tempfile::tempdir().unwrap` | [1536](../../src/helper.rs#L1536) | receiver-type-required |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `tempfile::tempdir` | [1536](../../src/helper.rs#L1536) | external-constructor-callback-or-unresolved |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `File::open(root.path()).unwrap` | [1537](../../src/helper.rs#L1537) | receiver-type-required |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `File::open` | [1537](../../src/helper.rs#L1537) | external-constructor-callback-or-unresolved |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `root.path` | [1537](../../src/helper.rs#L1537), [1555](../../src/helper.rs#L1555) | receiver-type-required |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `std::fs::write(root.path().join("regular"), b"unchanged").unwrap` | [1555](../../src/helper.rs#L1555) | receiver-type-required |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `std::fs::write` | [1555](../../src/helper.rs#L1555) | external-constructor-callback-or-unresolved |
| `parent_creation_is_limited_to_new_and_validates_entire_path_first` | `root.path().join` | [1555](../../src/helper.rs#L1555) | receiver-type-required |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `tempfile::tempdir().unwrap` | [1573](../../src/helper.rs#L1573), [1574](../../src/helper.rs#L1574) | receiver-type-required |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `tempfile::tempdir` | [1573](../../src/helper.rs#L1573), [1574](../../src/helper.rs#L1574) | external-constructor-callback-or-unresolved |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `File::open(root.path()).unwrap` | [1575](../../src/helper.rs#L1575) | receiver-type-required |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `File::open` | [1575](../../src/helper.rs#L1575) | external-constructor-callback-or-unresolved |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `root.path` | [1575](../../src/helper.rs#L1575), [1576](../../src/helper.rs#L1576) | receiver-type-required |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `symlink(outside.path(), root.path().join("escape")).unwrap` | [1576](../../src/helper.rs#L1576) | receiver-type-required |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `symlink` | [1576](../../src/helper.rs#L1576) | external-constructor-callback-or-unresolved |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `outside.path` | [1576](../../src/helper.rs#L1576) | receiver-type-required |
| `parent_creation_cannot_follow_symlinks_or_escape_root` | `root.path().join` | [1576](../../src/helper.rs#L1576) | receiver-type-required |
