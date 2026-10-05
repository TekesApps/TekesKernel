# tools::helper::native_grep

[Package atlas](index.md) · [Source](../../src/helper/native_grep.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tools::helper::native_grep::search](../../src/helper/native_grep.rs#L8) | function_item | `pub(super)` |  |
| [tools::helper::native_grep::tests::native_regex_modes_bounds_and_root_confinement](../../src/helper/native_grep.rs#L207) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `WalkBuilder` | `ignore::WalkBuilder` | `private` |
| `OverrideBuilder` | `ignore::overrides::OverrideBuilder` | `private` |
| `RegexBuilder` | `regex::bytes::RegexBuilder` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tools::helper::native_grep::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–1: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::helper::native_grep::search"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `search` | `Err` | [20](../../src/helper/native_grep.rs#L20), [26](../../src/helper/native_grep.rs#L26), [51](../../src/helper/native_grep.rs#L51), [82](../../src/helper/native_grep.rs#L82), [110](../../src/helper/native_grep.rs#L110), [125](../../src/helper/native_grep.rs#L125), [132](../../src/helper/native_grep.rs#L132), [146](../../src/helper/native_grep.rs#L146) | external-constructor-callback-or-unresolved |
| `search` | `HelperError::new` | [20](../../src/helper/native_grep.rs#L20), [26](../../src/helper/native_grep.rs#L26), [37](../../src/helper/native_grep.rs#L37), [60](../../src/helper/native_grep.rs#L60), [64](../../src/helper/native_grep.rs#L64), [82](../../src/helper/native_grep.rs#L82), [87](../../src/helper/native_grep.rs#L87), [94](../../src/helper/native_grep.rs#L94), [110](../../src/helper/native_grep.rs#L110), [125](../../src/helper/native_grep.rs#L125), [132](../../src/helper/native_grep.rs#L132), [146](../../src/helper/native_grep.rs#L146) | external-constructor-callback-or-unresolved |
| `search` | `checked_cap` | [31](../../src/helper/native_grep.rs#L31) | external-constructor-callback-or-unresolved |
| `search` | `RegexBuilder::new(pattern)         .case_insensitive(insensitive)         .size_limit(4 * 1024 * 1024)         .dfa_size_limit(4 * 1024 * 1024)         .build()         .map_err` | [32](../../src/helper/native_grep.rs#L32) | receiver-type-required |
| `search` | `RegexBuilder::new(pattern)         .case_insensitive(insensitive)         .size_limit(4 * 1024 * 1024)         .dfa_size_limit(4 * 1024 * 1024)         .build` | [32](../../src/helper/native_grep.rs#L32) | receiver-type-required |
| `search` | `RegexBuilder::new(pattern)         .case_insensitive(insensitive)         .size_limit(4 * 1024 * 1024)         .dfa_size_limit` | [32](../../src/helper/native_grep.rs#L32) | receiver-type-required |
| `search` | `RegexBuilder::new(pattern)         .case_insensitive(insensitive)         .size_limit` | [32](../../src/helper/native_grep.rs#L32) | receiver-type-required |
| `search` | `RegexBuilder::new(pattern)         .case_insensitive` | [32](../../src/helper/native_grep.rs#L32) | receiver-type-required |
| `search` | `RegexBuilder::new` | [32](../../src/helper/native_grep.rs#L32) | external-constructor-callback-or-unresolved |
| `search` | `e.to_string` | [37](../../src/helper/native_grep.rs#L37), [60](../../src/helper/native_grep.rs#L60), [64](../../src/helper/native_grep.rs#L64), [87](../../src/helper/native_grep.rs#L87) | receiver-type-required |
| `search` | `path.is_empty` | [38](../../src/helper/native_grep.rs#L38) | receiver-type-required |
| `search` | `PathBuf::new` | [39](../../src/helper/native_grep.rs#L39) | external-constructor-callback-or-unresolved |
| `search` | `normalized_relative` | [41](../../src/helper/native_grep.rs#L41) | external-constructor-callback-or-unresolved |
| `search` | `relative.as_os_str().is_empty` | [43](../../src/helper/native_grep.rs#L43) | receiver-type-required |
| `search` | `relative.as_os_str` | [43](../../src/helper/native_grep.rs#L43) | receiver-type-required |
| `search` | `split_parent` | [44](../../src/helper/native_grep.rs#L44) | external-constructor-callback-or-unresolved |
| `search` | `open_parent` | [45](../../src/helper/native_grep.rs#L45) | external-constructor-callback-or-unresolved |
| `search` | `root.directory.as_raw_fd` | [45](../../src/helper/native_grep.rs#L45), [104](../../src/helper/native_grep.rs#L104) | receiver-type-required |
| `search` | `open_directory_at` | [46](../../src/helper/native_grep.rs#L46) | external-constructor-callback-or-unresolved |
| `search` | `parent.as_raw_fd` | [46](../../src/helper/native_grep.rs#L46), [49](../../src/helper/native_grep.rs#L49) | receiver-type-required |
| `search` | `open_regular_at` | [49](../../src/helper/native_grep.rs#L49) | external-constructor-callback-or-unresolved |
| `search` | `root.path.join` | [54](../../src/helper/native_grep.rs#L54) | receiver-type-required |
| `search` | `WalkBuilder::new` | [55](../../src/helper/native_grep.rs#L55) | external-constructor-callback-or-unresolved |
| `search` | `OverrideBuilder::new` | [57](../../src/helper/native_grep.rs#L57) | external-constructor-callback-or-unresolved |
| `search` | `overrides             .add(glob)             .map_err` | [58](../../src/helper/native_grep.rs#L58) | receiver-type-required |
| `search` | `overrides             .add` | [58](../../src/helper/native_grep.rs#L58) | receiver-type-required |
| `search` | `walker.overrides` | [61](../../src/helper/native_grep.rs#L61) | receiver-type-required |
| `search` | `overrides                 .build()                 .map_err` | [62](../../src/helper/native_grep.rs#L62) | receiver-type-required |
| `search` | `overrides                 .build` | [62](../../src/helper/native_grep.rs#L62) | receiver-type-required |
| `search` | `walker         .follow_links(false)         .max_filesize(Some(HARD_BYTES as u64))         .git_global(false)         .git_exclude(false)         .parents(false)         .sort_by_file_path` | [67](../../src/helper/native_grep.rs#L67) | receiver-type-required |
| `search` | `walker         .follow_links(false)         .max_filesize(Some(HARD_BYTES as u64))         .git_global(false)         .git_exclude(false)         .parents` | [67](../../src/helper/native_grep.rs#L67) | receiver-type-required |
| `search` | `walker         .follow_links(false)         .max_filesize(Some(HARD_BYTES as u64))         .git_global(false)         .git_exclude` | [67](../../src/helper/native_grep.rs#L67) | receiver-type-required |
| `search` | `walker         .follow_links(false)         .max_filesize(Some(HARD_BYTES as u64))         .git_global` | [67](../../src/helper/native_grep.rs#L67) | receiver-type-required |
| `search` | `walker         .follow_links(false)         .max_filesize` | [67](../../src/helper/native_grep.rs#L67) | receiver-type-required |
| `search` | `walker         .follow_links` | [67](../../src/helper/native_grep.rs#L67) | receiver-type-required |
| `search` | `Some` | [69](../../src/helper/native_grep.rs#L69), [173](../../src/helper/native_grep.rs#L173) | external-constructor-callback-or-unresolved |
| `search` | `a.cmp` | [73](../../src/helper/native_grep.rs#L73) | receiver-type-required |
| `search` | `Instant::now` | [74](../../src/helper/native_grep.rs#L74) | external-constructor-callback-or-unresolved |
| `search` | `Vec::new` | [75](../../src/helper/native_grep.rs#L75), [122](../../src/helper/native_grep.rs#L122) | external-constructor-callback-or-unresolved |
| `search` | `walker.build().enumerate` | [80](../../src/helper/native_grep.rs#L80) | receiver-type-required |
| `search` | `walker.build` | [80](../../src/helper/native_grep.rs#L80) | receiver-type-required |
| `search` | `started.elapsed` | [81](../../src/helper/native_grep.rs#L81), [124](../../src/helper/native_grep.rs#L124) | receiver-type-required |
| `search` | `Duration::from_millis` | [81](../../src/helper/native_grep.rs#L81), [124](../../src/helper/native_grep.rs#L124) | external-constructor-callback-or-unresolved |
| `search` | `entry.map_err` | [87](../../src/helper/native_grep.rs#L87) | receiver-type-required |
| `search` | `entry.file_type().is_some_and` | [88](../../src/helper/native_grep.rs#L88) | receiver-type-required |
| `search` | `entry.file_type` | [88](../../src/helper/native_grep.rs#L88) | receiver-type-required |
| `search` | `t.is_file` | [88](../../src/helper/native_grep.rs#L88) | receiver-type-required |
| `search` | `entry             .path()             .strip_prefix(&root.path)             .map_err` | [91](../../src/helper/native_grep.rs#L91) | receiver-type-required |
| `search` | `entry             .path()             .strip_prefix` | [91](../../src/helper/native_grep.rs#L91), [95](../../src/helper/native_grep.rs#L95) | receiver-type-required |
| `search` | `entry             .path` | [91](../../src/helper/native_grep.rs#L91), [95](../../src/helper/native_grep.rs#L95) | receiver-type-required |
| `search` | `entry             .path()             .strip_prefix(&base)             .ok()             .filter(&#124;p&#124; !p.as_os_str().is_empty())             .unwrap_or` | [95](../../src/helper/native_grep.rs#L95) | receiver-type-required |
| `search` | `entry             .path()             .strip_prefix(&base)             .ok()             .filter` | [95](../../src/helper/native_grep.rs#L95) | receiver-type-required |
| `search` | `entry             .path()             .strip_prefix(&base)             .ok` | [95](../../src/helper/native_grep.rs#L95) | receiver-type-required |
| `search` | `p.as_os_str().is_empty` | [99](../../src/helper/native_grep.rs#L99) | receiver-type-required |
| `search` | `p.as_os_str` | [99](../../src/helper/native_grep.rs#L99) | receiver-type-required |
| `search` | `read_regular` | [103](../../src/helper/native_grep.rs#L103) | external-constructor-callback-or-unresolved |
| `search` | `relative.to_string_lossy` | [105](../../src/helper/native_grep.rs#L105) | receiver-type-required |
| `search` | `total.saturating_add` | [108](../../src/helper/native_grep.rs#L108) | receiver-type-required |
| `search` | `bytes.len` | [108](../../src/helper/native_grep.rs#L108) | receiver-type-required |
| `search` | `bytes.contains` | [115](../../src/helper/native_grep.rs#L115) | receiver-type-required |
| `search` | `bytes.split(&#124;b&#124; *b == b'\n').collect` | [118](../../src/helper/native_grep.rs#L118) | receiver-type-required |
| `search` | `bytes.split` | [118](../../src/helper/native_grep.rs#L118) | receiver-type-required |
| `search` | `lines.last().is_some_and` | [119](../../src/helper/native_grep.rs#L119) | receiver-type-required |
| `search` | `lines.last` | [119](../../src/helper/native_grep.rs#L119) | receiver-type-required |
| `search` | `line.is_empty` | [119](../../src/helper/native_grep.rs#L119) | receiver-type-required |
| `search` | `lines.pop` | [120](../../src/helper/native_grep.rs#L120) | receiver-type-required |
| `search` | `lines.iter().enumerate` | [123](../../src/helper/native_grep.rs#L123) | receiver-type-required |
| `search` | `lines.iter` | [123](../../src/helper/native_grep.rs#L123) | receiver-type-required |
| `search` | `regex.is_match` | [130](../../src/helper/native_grep.rs#L130) | receiver-type-required |
| `search` | `matches.saturating_add` | [131](../../src/helper/native_grep.rs#L131), [144](../../src/helper/native_grep.rs#L144) | receiver-type-required |
| `search` | `hits.len` | [131](../../src/helper/native_grep.rs#L131), [144](../../src/helper/native_grep.rs#L144) | receiver-type-required |
| `search` | `hits.push` | [137](../../src/helper/native_grep.rs#L137) | receiver-type-required |
| `search` | `hits.is_empty` | [140](../../src/helper/native_grep.rs#L140) | receiver-type-required |
| `search` | `display.to_string_lossy` | [151](../../src/helper/native_grep.rs#L151) | receiver-type-required |
| `search` | `cap.saturating_sub` | [153](../../src/helper/native_grep.rs#L153) | receiver-type-required |
| `search` | `stdout.len` | [153](../../src/helper/native_grep.rs#L153) | receiver-type-required |
| `search` | `stdout.extend_from_slice` | [154](../../src/helper/native_grep.rs#L154) | receiver-type-required |
| `search` | `value.len().min` | [154](../../src/helper/native_grep.rs#L154) | receiver-type-required |
| `search` | `value.len` | [154](../../src/helper/native_grep.rs#L154), [155](../../src/helper/native_grep.rs#L155) | receiver-type-required |
| `search` | `emit` | [158](../../src/helper/native_grep.rs#L158), [159](../../src/helper/native_grep.rs#L159), [171](../../src/helper/native_grep.rs#L171), [179](../../src/helper/native_grep.rs#L179), [182](../../src/helper/native_grep.rs#L182), [184](../../src/helper/native_grep.rs#L184), [186](../../src/helper/native_grep.rs#L186) | external-constructor-callback-or-unresolved |
| `search` | `format!("{name}\n").as_bytes` | [158](../../src/helper/native_grep.rs#L158) | receiver-type-required |
| `search` | `format!("{name}:{}\n", hits.len()).as_bytes` | [159](../../src/helper/native_grep.rs#L159) | receiver-type-required |
| `search` | `std::collections::BTreeSet::new` | [161](../../src/helper/native_grep.rs#L161) | external-constructor-callback-or-unresolved |
| `search` | `selected.extend` | [163](../../src/helper/native_grep.rs#L163) | receiver-type-required |
| `search` | `hit.saturating_sub` | [164](../../src/helper/native_grep.rs#L164) | receiver-type-required |
| `search` | `hit.saturating_add(context as usize).min` | [165](../../src/helper/native_grep.rs#L165) | receiver-type-required |
| `search` | `hit.saturating_add` | [165](../../src/helper/native_grep.rs#L165) | receiver-type-required |
| `search` | `lines.len` | [165](../../src/helper/native_grep.rs#L165) | receiver-type-required |
| `search` | `previous.is_some_and` | [170](../../src/helper/native_grep.rs#L170) | receiver-type-required |
| `search` | `hits.binary_search(&i).is_ok` | [174](../../src/helper/native_grep.rs#L174) | receiver-type-required |
| `search` | `hits.binary_search` | [174](../../src/helper/native_grep.rs#L174) | receiver-type-required |
| `search` | `format!("{name}{separator}{}{separator}", i + 1).as_bytes` | [179](../../src/helper/native_grep.rs#L179) | receiver-type-required |
| `search` | `line.len` | [181](../../src/helper/native_grep.rs#L181) | receiver-type-required |
| `search` | `Ok` | [194](../../src/helper/native_grep.rs#L194) | external-constructor-callback-or-unresolved |
| `search` | `ByteString::from_bytes` | [196](../../src/helper/native_grep.rs#L196), [197](../../src/helper/native_grep.rs#L197) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `tempfile::tempdir().unwrap` | [208](../../src/helper/native_grep.rs#L208), [218](../../src/helper/native_grep.rs#L218) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `tempfile::tempdir` | [208](../../src/helper/native_grep.rs#L208), [218](../../src/helper/native_grep.rs#L218) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::create_dir(dir.path().join("docs")).unwrap` | [209](../../src/helper/native_grep.rs#L209) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::create_dir` | [209](../../src/helper/native_grep.rs#L209), [210](../../src/helper/native_grep.rs#L210) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `dir.path().join` | [209](../../src/helper/native_grep.rs#L209), [210](../../src/helper/native_grep.rs#L210), [212](../../src/helper/native_grep.rs#L212), [216](../../src/helper/native_grep.rs#L216), [217](../../src/helper/native_grep.rs#L217), [220](../../src/helper/native_grep.rs#L220) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `dir.path` | [209](../../src/helper/native_grep.rs#L209), [210](../../src/helper/native_grep.rs#L210), [212](../../src/helper/native_grep.rs#L212), [216](../../src/helper/native_grep.rs#L216), [217](../../src/helper/native_grep.rs#L217), [220](../../src/helper/native_grep.rs#L220), [221](../../src/helper/native_grep.rs#L221) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::create_dir(dir.path().join(".git")).unwrap` | [210](../../src/helper/native_grep.rs#L210) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::write(             dir.path().join("docs/a.md"),             b"before\nNeedle 42\nafter\nNeedle 7\n",         )         .unwrap` | [211](../../src/helper/native_grep.rs#L211) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::write` | [211](../../src/helper/native_grep.rs#L211), [216](../../src/helper/native_grep.rs#L216), [217](../../src/helper/native_grep.rs#L217), [219](../../src/helper/native_grep.rs#L219) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::write(dir.path().join("ignored.md"), b"Needle 9").unwrap` | [216](../../src/helper/native_grep.rs#L216) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::write(dir.path().join(".gitignore"), b"ignored.md\n").unwrap` | [217](../../src/helper/native_grep.rs#L217) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::fs::write(outside.path().join("secret"), b"Needle SECRET").unwrap` | [219](../../src/helper/native_grep.rs#L219) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `outside.path().join` | [219](../../src/helper/native_grep.rs#L219) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `outside.path` | [219](../../src/helper/native_grep.rs#L219), [220](../../src/helper/native_grep.rs#L220) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::os::unix::fs::symlink(outside.path(), dir.path().join("escape")).unwrap` | [220](../../src/helper/native_grep.rs#L220) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `std::os::unix::fs::symlink` | [220](../../src/helper/native_grep.rs#L220) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `RootBinding::open("workspace", dir.path()).unwrap` | [221](../../src/helper/native_grep.rs#L221) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `RootBinding::open` | [221](../../src/helper/native_grep.rs#L221) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `search(                 &root,                 "",                 r"needle \d+",                 None,                 mode,                 true,                 context,                 cap,                 1000,             )             .unwrap` | [223](../../src/helper/native_grep.rs#L223) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `search` | [223](../../src/helper/native_grep.rs#L223), [236](../../src/helper/native_grep.rs#L236) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `search(             &root,             "",             "Needle",             Some("ignored.md"),             "files_with_matches",             false,             0,             4096,             1000,         )         .unwrap` | [236](../../src/helper/native_grep.rs#L236) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `Some` | [240](../../src/helper/native_grep.rs#L240) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `run` | [253](../../src/helper/native_grep.rs#L253), [259](../../src/helper/native_grep.rs#L259) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `String::from_utf8(run("content", 1, 4096).stdout.decode().unwrap()).unwrap` | [259](../../src/helper/native_grep.rs#L259) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `String::from_utf8` | [259](../../src/helper/native_grep.rs#L259) | external-constructor-callback-or-unresolved |
| `native_regex_modes_bounds_and_root_confinement` | `run("content", 1, 4096).stdout.decode().unwrap` | [259](../../src/helper/native_grep.rs#L259) | receiver-type-required |
| `native_regex_modes_bounds_and_root_confinement` | `run("content", 1, 4096).stdout.decode` | [259](../../src/helper/native_grep.rs#L259) | receiver-type-required |
