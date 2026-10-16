# test-support

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [test-support::FixtureError](../../src/lib.rs#L10) | enum_item | `pub` |  |
| [test-support::FixtureRoot](../../src/lib.rs#L33) | struct_item | `pub` |  |
| [test-support::FixtureRoot::discover](../../src/lib.rs#L36) | function_item | `pub` |  |
| [test-support::FixtureRoot::from_manifest_dir](../../src/lib.rs#L47) | function_item | `pub` |  |
| [test-support::FixtureRoot::path](../../src/lib.rs#L61) | function_item | `pub` |  |
| [test-support::FixtureRoot::join](../../src/lib.rs#L66) | function_item | `pub` |  |
| [test-support::FixtureRoot::verify_manifest](../../src/lib.rs#L70) | function_item | `pub` |  |
| [test-support::collect_relative_files](../../src/lib.rs#L139) | function_item | `private` |  |
| [test-support::FixtureManifest](../../src/lib.rs#L174) | struct_item | `pub` |  |
| [test-support::read](../../src/lib.rs#L179) | function_item | `pub` |  |
| [test-support::execution_fixture](../../src/lib.rs#L188) | function_item | `pub` |  |
| [test-support::execution_fixture::PROJECT](../../src/lib.rs#L189) | static_item | `private` |  |
| [test-support::freeze_execution_fixture](../../src/lib.rs#L197) | function_item | `pub` |  |
| [test-support::resolve_execution_fixture](../../src/lib.rs#L227) | function_item | `pub` |  |
| [test-support::bind_execution_fixture](../../src/lib.rs#L240) | function_item | `pub` |  |
| [test-support::tests::manifest_rejects_undeclared_top_level_corpus](../../src/lib.rs#L283) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `fs` | `std::fs` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `test-support::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–11: 7 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::event::Event::from_value"]
  n1["schema::ijson::IJsonValue::parse"]
  n2["test-support::collect_relative_files"]
  n3["test-support::read"]
  n4["test-support::execution_fixture"]
  n5["test-support::freeze_execution_fixture"]
  n6["test-support::resolve_execution_fixture"]
  n7["test-support::bind_execution_fixture"]
  n8["test-support::FixtureRoot::discover"]
  n9["test-support::FixtureRoot::from_manifest_dir"]
  n10["test-support::FixtureRoot::path"]
  n11["test-support::FixtureRoot::join"]
  n12["test-support::FixtureRoot::verify_manifest"]
  n2 --> n2
  n7 --> n0
  n7 --> n1
  n8 --> n9
  n12 --> n2
  n12 --> n3
  n12 --> n11
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `discover` | `std::env::var_os` | [37](../../src/lib.rs#L37) | external-constructor-callback-or-unresolved |
| `discover` | `PathBuf::from` | [38](../../src/lib.rs#L38) | external-constructor-callback-or-unresolved |
| `discover` | `path.is_absolute` | [39](../../src/lib.rs#L39) | receiver-type-required |
| `discover` | `path.is_dir` | [39](../../src/lib.rs#L39) | receiver-type-required |
| `discover` | `Ok` | [40](../../src/lib.rs#L40) | external-constructor-callback-or-unresolved |
| `discover` | `Self` | [40](../../src/lib.rs#L40) | external-constructor-callback-or-unresolved |
| `discover` | `Err` | [42](../../src/lib.rs#L42) | external-constructor-callback-or-unresolved |
| `discover` | `FixtureError::InvalidOverride` | [42](../../src/lib.rs#L42) | external-constructor-callback-or-unresolved |
| `discover` | `path.display().to_string` | [42](../../src/lib.rs#L42) | receiver-type-required |
| `discover` | `path.display` | [42](../../src/lib.rs#L42) | receiver-type-required |
| `discover` | `Self::from_manifest_dir` | [44](../../src/lib.rs#L44) | [test-support::FixtureRoot::from_manifest_dir](../../src/lib.rs#L47) |
| `discover` | `Path::new` | [44](../../src/lib.rs#L44) | external-constructor-callback-or-unresolved |
| `from_manifest_dir` | `Vec::new` | [48](../../src/lib.rs#L48) | external-constructor-callback-or-unresolved |
| `from_manifest_dir` | `start.ancestors` | [49](../../src/lib.rs#L49) | receiver-type-required |
| `from_manifest_dir` | `checked.push` | [50](../../src/lib.rs#L50) | receiver-type-required |
| `from_manifest_dir` | `ancestor.to_path_buf` | [50](../../src/lib.rs#L50) | receiver-type-required |
| `from_manifest_dir` | `ancestor.join("Cargo.toml").is_file` | [51](../../src/lib.rs#L51) | receiver-type-required |
| `from_manifest_dir` | `ancestor.join` | [51](../../src/lib.rs#L51), [52](../../src/lib.rs#L52), [54](../../src/lib.rs#L54) | receiver-type-required |
| `from_manifest_dir` | `ancestor.join("fixtures/manifest.json").is_file` | [52](../../src/lib.rs#L52) | receiver-type-required |
| `from_manifest_dir` | `Ok` | [54](../../src/lib.rs#L54) | external-constructor-callback-or-unresolved |
| `from_manifest_dir` | `Self` | [54](../../src/lib.rs#L54) | external-constructor-callback-or-unresolved |
| `from_manifest_dir` | `Err` | [57](../../src/lib.rs#L57) | external-constructor-callback-or-unresolved |
| `from_manifest_dir` | `FixtureError::NotFound` | [57](../../src/lib.rs#L57) | external-constructor-callback-or-unresolved |
| `join` | `self.0.join` | [67](../../src/lib.rs#L67) | receiver-type-required |
| `verify_manifest` | `self.join` | [71](../../src/lib.rs#L71), [111](../../src/lib.rs#L111), [126](../../src/lib.rs#L126) | [test-support::FixtureRoot::join](../../src/lib.rs#L66) |
| `verify_manifest` | `read` | [72](../../src/lib.rs#L72), [126](../../src/lib.rs#L126) | [test-support::read](../../src/lib.rs#L179) |
| `verify_manifest` | `serde_json::from_slice` | [73](../../src/lib.rs#L73) | external-constructor-callback-or-unresolved |
| `verify_manifest` | `BTreeSet::new` | [74](../../src/lib.rs#L74), [112](../../src/lib.rs#L112) | external-constructor-callback-or-unresolved |
| `verify_manifest` | `fs::read_dir(&self.0).map_err` | [75](../../src/lib.rs#L75) | receiver-type-required |
| `verify_manifest` | `fs::read_dir` | [75](../../src/lib.rs#L75) | external-constructor-callback-or-unresolved |
| `verify_manifest` | `self.0.clone` | [76](../../src/lib.rs#L76), [80](../../src/lib.rs#L80) | receiver-type-required |
| `verify_manifest` | `entry.map_err` | [79](../../src/lib.rs#L79) | receiver-type-required |
| `verify_manifest` | `entry                 .file_type()                 .map_err(&#124;source&#124; FixtureError::Io {                     path: entry.path(),                     source,                 })?                 .is_dir` | [83](../../src/lib.rs#L83) | receiver-type-required |
| `verify_manifest` | `entry                 .file_type()                 .map_err` | [83](../../src/lib.rs#L83) | receiver-type-required |
| `verify_manifest` | `entry                 .file_type` | [83](../../src/lib.rs#L83) | receiver-type-required |
| `verify_manifest` | `entry.path` | [86](../../src/lib.rs#L86) | receiver-type-required |
| `verify_manifest` | `disk_corpora.insert` | [91](../../src/lib.rs#L91) | receiver-type-required |
| `verify_manifest` | `entry.file_name().to_string_lossy().into_owned` | [91](../../src/lib.rs#L91) | receiver-type-required |
| `verify_manifest` | `entry.file_name().to_string_lossy` | [91](../../src/lib.rs#L91) | receiver-type-required |
| `verify_manifest` | `entry.file_name` | [91](../../src/lib.rs#L91) | receiver-type-required |
| `verify_manifest` | `manifest.corpora.keys().cloned().collect::<BTreeSet<_>>` | [94](../../src/lib.rs#L94) | receiver-type-required |
| `verify_manifest` | `manifest.corpora.keys().cloned` | [94](../../src/lib.rs#L94) | receiver-type-required |
| `verify_manifest` | `manifest.corpora.keys` | [94](../../src/lib.rs#L94) | receiver-type-required |
| `verify_manifest` | `declared_corpora             .difference(&disk_corpora)             .cloned()             .collect::<BTreeSet<_>>` | [95](../../src/lib.rs#L95) | receiver-type-required |
| `verify_manifest` | `declared_corpora             .difference(&disk_corpora)             .cloned` | [95](../../src/lib.rs#L95) | receiver-type-required |
| `verify_manifest` | `declared_corpora             .difference` | [95](../../src/lib.rs#L95) | receiver-type-required |
| `verify_manifest` | `disk_corpora             .difference(&declared_corpora)             .cloned()             .collect::<BTreeSet<_>>` | [99](../../src/lib.rs#L99) | receiver-type-required |
| `verify_manifest` | `disk_corpora             .difference(&declared_corpora)             .cloned` | [99](../../src/lib.rs#L99) | receiver-type-required |
| `verify_manifest` | `disk_corpora             .difference` | [99](../../src/lib.rs#L99) | receiver-type-required |
| `verify_manifest` | `missing.is_empty` | [103](../../src/lib.rs#L103), [117](../../src/lib.rs#L117) | receiver-type-required |
| `verify_manifest` | `extra.is_empty` | [103](../../src/lib.rs#L103), [117](../../src/lib.rs#L117) | receiver-type-required |
| `verify_manifest` | `Err` | [104](../../src/lib.rs#L104), [118](../../src/lib.rs#L118), [129](../../src/lib.rs#L129) | external-constructor-callback-or-unresolved |
| `verify_manifest` | `"<root>".to_owned` | [105](../../src/lib.rs#L105) | receiver-type-required |
| `verify_manifest` | `collect_relative_files` | [113](../../src/lib.rs#L113) | [test-support::collect_relative_files](../../src/lib.rs#L139) |
| `verify_manifest` | `declared.iter().cloned().collect` | [114](../../src/lib.rs#L114) | receiver-type-required |
| `verify_manifest` | `declared.iter().cloned` | [114](../../src/lib.rs#L114) | receiver-type-required |
| `verify_manifest` | `declared.iter` | [114](../../src/lib.rs#L114) | receiver-type-required |
| `verify_manifest` | `declared.difference(&disk).cloned().collect::<BTreeSet<_>>` | [115](../../src/lib.rs#L115) | receiver-type-required |
| `verify_manifest` | `declared.difference(&disk).cloned` | [115](../../src/lib.rs#L115) | receiver-type-required |
| `verify_manifest` | `declared.difference` | [115](../../src/lib.rs#L115) | receiver-type-required |
| `verify_manifest` | `disk.difference(&declared).cloned().collect::<BTreeSet<_>>` | [116](../../src/lib.rs#L116) | receiver-type-required |
| `verify_manifest` | `disk.difference(&declared).cloned` | [116](../../src/lib.rs#L116) | receiver-type-required |
| `verify_manifest` | `disk.difference` | [116](../../src/lib.rs#L116) | receiver-type-required |
| `verify_manifest` | `corpus.clone` | [119](../../src/lib.rs#L119) | receiver-type-required |
| `verify_manifest` | `manifest.corpora.get("assets").into_iter().flatten` | [125](../../src/lib.rs#L125) | receiver-type-required |
| `verify_manifest` | `manifest.corpora.get("assets").into_iter` | [125](../../src/lib.rs#L125) | receiver-type-required |
| `verify_manifest` | `manifest.corpora.get` | [125](../../src/lib.rs#L125) | receiver-type-required |
| `verify_manifest` | `self.join("assets").join` | [126](../../src/lib.rs#L126) | receiver-type-required |
| `verify_manifest` | `name.clone` | [130](../../src/lib.rs#L130) | receiver-type-required |
| `verify_manifest` | `Ok` | [135](../../src/lib.rs#L135) | external-constructor-callback-or-unresolved |
| `collect_relative_files` | `fs::read_dir(current).map_err` | [144](../../src/lib.rs#L144) | receiver-type-required |
| `collect_relative_files` | `fs::read_dir` | [144](../../src/lib.rs#L144) | external-constructor-callback-or-unresolved |
| `collect_relative_files` | `current.to_path_buf` | [145](../../src/lib.rs#L145), [149](../../src/lib.rs#L149) | receiver-type-required |
| `collect_relative_files` | `entry.map_err` | [148](../../src/lib.rs#L148) | receiver-type-required |
| `collect_relative_files` | `entry.file_type().map_err` | [152](../../src/lib.rs#L152) | receiver-type-required |
| `collect_relative_files` | `entry.file_type` | [152](../../src/lib.rs#L152) | receiver-type-required |
| `collect_relative_files` | `entry.path` | [153](../../src/lib.rs#L153), [157](../../src/lib.rs#L157) | receiver-type-required |
| `collect_relative_files` | `file_type.is_dir` | [156](../../src/lib.rs#L156) | receiver-type-required |
| `collect_relative_files` | `collect_relative_files` | [157](../../src/lib.rs#L157) | [test-support::collect_relative_files](../../src/lib.rs#L139) |
| `collect_relative_files` | `file_type.is_file` | [158](../../src/lib.rs#L158) | receiver-type-required |
| `collect_relative_files` | `entry                 .path()                 .strip_prefix(root)                 .expect("fixture descendant")                 .components()                 .map(&#124;part&#124; part.as_os_str().to_string_lossy())                 .collect::<Vec<_>>()                 .join` | [159](../../src/lib.rs#L159) | receiver-type-required |
| `collect_relative_files` | `entry                 .path()                 .strip_prefix(root)                 .expect("fixture descendant")                 .components()                 .map(&#124;part&#124; part.as_os_str().to_string_lossy())                 .collect::<Vec<_>>` | [159](../../src/lib.rs#L159) | receiver-type-required |
| `collect_relative_files` | `entry                 .path()                 .strip_prefix(root)                 .expect("fixture descendant")                 .components()                 .map` | [159](../../src/lib.rs#L159) | receiver-type-required |
| `collect_relative_files` | `entry                 .path()                 .strip_prefix(root)                 .expect("fixture descendant")                 .components` | [159](../../src/lib.rs#L159) | receiver-type-required |
| `collect_relative_files` | `entry                 .path()                 .strip_prefix(root)                 .expect` | [159](../../src/lib.rs#L159) | receiver-type-required |
| `collect_relative_files` | `entry                 .path()                 .strip_prefix` | [159](../../src/lib.rs#L159) | receiver-type-required |
| `collect_relative_files` | `entry                 .path` | [159](../../src/lib.rs#L159) | receiver-type-required |
| `collect_relative_files` | `part.as_os_str().to_string_lossy` | [164](../../src/lib.rs#L164) | receiver-type-required |
| `collect_relative_files` | `part.as_os_str` | [164](../../src/lib.rs#L164) | receiver-type-required |
| `collect_relative_files` | `output.insert` | [167](../../src/lib.rs#L167) | receiver-type-required |
| `collect_relative_files` | `Ok` | [170](../../src/lib.rs#L170) | external-constructor-callback-or-unresolved |
| `read` | `fs::read(path).map_err` | [180](../../src/lib.rs#L180) | receiver-type-required |
| `read` | `fs::read` | [180](../../src/lib.rs#L180) | external-constructor-callback-or-unresolved |
| `read` | `path.to_path_buf` | [181](../../src/lib.rs#L181) | receiver-type-required |
| `execution_fixture` | `PROJECT.get_or_init` | [190](../../src/lib.rs#L190) | receiver-type-required |
| `execution_fixture` | `tempfile::tempdir().unwrap` | [190](../../src/lib.rs#L190) | receiver-type-required |
| `execution_fixture` | `tempfile::tempdir` | [190](../../src/lib.rs#L190) | external-constructor-callback-or-unresolved |
| `execution_fixture` | `serde_json::from_value(serde_json::json!({"cwd":project.path().canonicalize().unwrap()}))         .unwrap` | [191](../../src/lib.rs#L191) | receiver-type-required |
| `execution_fixture` | `serde_json::from_value` | [191](../../src/lib.rs#L191) | external-constructor-callback-or-unresolved |
| `PROJECT` | `std::sync::OnceLock::new` | [189](../../src/lib.rs#L189) | external-constructor-callback-or-unresolved |
| `freeze_execution_fixture` | `policy.unwrap_or_default` | [203](../../src/lib.rs#L203) | receiver-type-required |
| `freeze_execution_fixture` | `roots.split_first().ok_or` | [204](../../src/lib.rs#L204) | receiver-type-required |
| `freeze_execution_fixture` | `roots.split_first` | [204](../../src/lib.rs#L204) | receiver-type-required |
| `freeze_execution_fixture` | `serde_json::from_value` | [205](../../src/lib.rs#L205) | external-constructor-callback-or-unresolved |
| `freeze_execution_fixture` | `repository.normalize_execution` | [210](../../src/lib.rs#L210) | receiver-type-required |
| `freeze_execution_fixture` | `std::fs::write` | [211](../../src/lib.rs#L211) | external-constructor-callback-or-unresolved |
| `freeze_execution_fixture` | `repository.root().join` | [212](../../src/lib.rs#L212) | receiver-type-required |
| `freeze_execution_fixture` | `repository.root` | [212](../../src/lib.rs#L212) | receiver-type-required |
| `freeze_execution_fixture` | `serde_json::to_vec` | [213](../../src/lib.rs#L213) | external-constructor-callback-or-unresolved |
| `freeze_execution_fixture` | `provider.is_some` | [216](../../src/lib.rs#L216) | receiver-type-required |
| `freeze_execution_fixture` | `model.is_some` | [216](../../src/lib.rs#L216) | receiver-type-required |
| `freeze_execution_fixture` | `repository.settings` | [217](../../src/lib.rs#L217) | receiver-type-required |
| `freeze_execution_fixture` | `repository.publish_settings` | [222](../../src/lib.rs#L222) | receiver-type-required |
| `freeze_execution_fixture` | `Ok` | [224](../../src/lib.rs#L224) | external-constructor-callback-or-unresolved |
| `resolve_execution_fixture` | `serde_json::from_slice` | [232](../../src/lib.rs#L232) | external-constructor-callback-or-unresolved |
| `resolve_execution_fixture` | `std::fs::read` | [232](../../src/lib.rs#L232) | external-constructor-callback-or-unresolved |
| `resolve_execution_fixture` | `repository.root().join` | [233](../../src/lib.rs#L233) | receiver-type-required |
| `resolve_execution_fixture` | `repository.root` | [233](../../src/lib.rs#L233) | receiver-type-required |
| `resolve_execution_fixture` | `Ok` | [235](../../src/lib.rs#L235) | external-constructor-callback-or-unresolved |
| `resolve_execution_fixture` | `repository.resolve_execution` | [235](../../src/lib.rs#L235) | receiver-type-required |
| `bind_execution_fixture` | `std::fs::read(ledger).unwrap` | [241](../../src/lib.rs#L241) | receiver-type-required |
| `bind_execution_fixture` | `std::fs::read` | [241](../../src/lib.rs#L241), [248](../../src/lib.rs#L248) | external-constructor-callback-or-unresolved |
| `bind_execution_fixture` | `bytes.iter().position(&#124;byte&#124; *byte == b'\n').unwrap` | [242](../../src/lib.rs#L242) | receiver-type-required |
| `bind_execution_fixture` | `bytes.iter().position` | [242](../../src/lib.rs#L242) | receiver-type-required |
| `bind_execution_fixture` | `bytes.iter` | [242](../../src/lib.rs#L242) | receiver-type-required |
| `bind_execution_fixture` | `serde_json::from_slice(&bytes[..split]).unwrap` | [243](../../src/lib.rs#L243) | receiver-type-required |
| `bind_execution_fixture` | `serde_json::from_slice` | [243](../../src/lib.rs#L243), [247](../../src/lib.rs#L247) | external-constructor-callback-or-unresolved |
| `bind_execution_fixture` | `genesis.get("execution").is_some` | [244](../../src/lib.rs#L244) | receiver-type-required |
| `bind_execution_fixture` | `genesis.get` | [244](../../src/lib.rs#L244) | receiver-type-required |
| `bind_execution_fixture` | `serde_json::from_slice(         &std::fs::read(repository.root().join("execution-fixture.json")).unwrap(),     )     .unwrap` | [247](../../src/lib.rs#L247) | receiver-type-required |
| `bind_execution_fixture` | `std::fs::read(repository.root().join("execution-fixture.json")).unwrap` | [248](../../src/lib.rs#L248) | receiver-type-required |
| `bind_execution_fixture` | `repository.root().join` | [248](../../src/lib.rs#L248) | receiver-type-required |
| `bind_execution_fixture` | `repository.root` | [248](../../src/lib.rs#L248) | receiver-type-required |
| `bind_execution_fixture` | `genesis.as_object_mut().unwrap().remove` | [258](../../src/lib.rs#L258) | receiver-type-required |
| `bind_execution_fixture` | `genesis.as_object_mut().unwrap` | [258](../../src/lib.rs#L258) | receiver-type-required |
| `bind_execution_fixture` | `genesis.as_object_mut` | [258](../../src/lib.rs#L258) | receiver-type-required |
| `bind_execution_fixture` | `serde_json::to_value(execution).unwrap` | [261](../../src/lib.rs#L261) | receiver-type-required |
| `bind_execution_fixture` | `serde_json::to_value` | [261](../../src/lib.rs#L261) | external-constructor-callback-or-unresolved |
| `bind_execution_fixture` | `schema::Event::from_value(         schema::IJsonValue::parse(&serde_json::to_vec(&genesis).unwrap()).unwrap(),     )     .unwrap` | [268](../../src/lib.rs#L268) | receiver-type-required |
| `bind_execution_fixture` | `schema::Event::from_value` | [268](../../src/lib.rs#L268) | [schema::event::Event::from_value](../../../schema/src/event.rs#L179) |
| `bind_execution_fixture` | `schema::IJsonValue::parse(&serde_json::to_vec(&genesis).unwrap()).unwrap` | [269](../../src/lib.rs#L269) | receiver-type-required |
| `bind_execution_fixture` | `schema::IJsonValue::parse` | [269](../../src/lib.rs#L269) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `bind_execution_fixture` | `serde_json::to_vec(&genesis).unwrap` | [269](../../src/lib.rs#L269) | receiver-type-required |
| `bind_execution_fixture` | `serde_json::to_vec` | [269](../../src/lib.rs#L269) | external-constructor-callback-or-unresolved |
| `bind_execution_fixture` | `event.canonical_bytes().unwrap` | [272](../../src/lib.rs#L272) | receiver-type-required |
| `bind_execution_fixture` | `event.canonical_bytes` | [272](../../src/lib.rs#L272) | receiver-type-required |
| `bind_execution_fixture` | `output.push` | [273](../../src/lib.rs#L273) | receiver-type-required |
| `bind_execution_fixture` | `output.extend_from_slice` | [274](../../src/lib.rs#L274) | receiver-type-required |
| `bind_execution_fixture` | `std::fs::write(ledger, output).unwrap` | [275](../../src/lib.rs#L275) | receiver-type-required |
| `bind_execution_fixture` | `std::fs::write` | [275](../../src/lib.rs#L275) | external-constructor-callback-or-unresolved |
| `manifest_rejects_undeclared_top_level_corpus` | `tempfile::tempdir().expect` | [284](../../src/lib.rs#L284) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `tempfile::tempdir` | [284](../../src/lib.rs#L284) | external-constructor-callback-or-unresolved |
| `manifest_rejects_undeclared_top_level_corpus` | `fs::write(             root.path().join("manifest.json"),             br#"{"corpora":{"known":["case.txt"]},"version":1}"#,         )         .expect` | [285](../../src/lib.rs#L285) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `fs::write` | [285](../../src/lib.rs#L285), [291](../../src/lib.rs#L291) | external-constructor-callback-or-unresolved |
| `manifest_rejects_undeclared_top_level_corpus` | `root.path().join` | [286](../../src/lib.rs#L286), [290](../../src/lib.rs#L290), [291](../../src/lib.rs#L291), [292](../../src/lib.rs#L292) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `root.path` | [286](../../src/lib.rs#L286), [290](../../src/lib.rs#L290), [291](../../src/lib.rs#L291), [292](../../src/lib.rs#L292), [294](../../src/lib.rs#L294) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `fs::create_dir(root.path().join("known")).expect` | [290](../../src/lib.rs#L290) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `fs::create_dir` | [290](../../src/lib.rs#L290), [292](../../src/lib.rs#L292) | external-constructor-callback-or-unresolved |
| `manifest_rejects_undeclared_top_level_corpus` | `fs::write(root.path().join("known/case.txt"), b"case").expect` | [291](../../src/lib.rs#L291) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `fs::create_dir(root.path().join("undeclared")).expect` | [292](../../src/lib.rs#L292) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `FixtureRoot(root.path().to_path_buf())             .verify_manifest()             .expect_err` | [294](../../src/lib.rs#L294) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `FixtureRoot(root.path().to_path_buf())             .verify_manifest` | [294](../../src/lib.rs#L294) | receiver-type-required |
| `manifest_rejects_undeclared_top_level_corpus` | `FixtureRoot` | [294](../../src/lib.rs#L294) | external-constructor-callback-or-unresolved |
| `manifest_rejects_undeclared_top_level_corpus` | `root.path().to_path_buf` | [294](../../src/lib.rs#L294) | receiver-type-required |
