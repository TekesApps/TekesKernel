# store::platform

[Package atlas](index.md) · [Source](../../src/platform.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [store::platform::PROBE_ORDINAL](../../src/platform.rs#L10) | static_item | `private` |  |
| [store::platform::SyncPolicy](../../src/platform.rs#L14) | trait_item | `pub` |  |
| [store::platform::SyncPolicy::full_sync](../../src/platform.rs#L15) | function_signature_item | `private` |  |
| [store::platform::SystemSync](../../src/platform.rs#L19) | struct_item | `pub` |  |
| [store::platform::SystemSync::full_sync](../../src/platform.rs#L22) | function_item | `private` |  |
| [store::platform::FullSync](../../src/platform.rs#L27) | struct_item | `pub` |  |
| [store::platform::FullSync::full_sync](../../src/platform.rs#L30) | function_item | `pub` |  |
| [store::platform::DirectoryLock](../../src/platform.rs#L51) | struct_item | `pub` |  |
| [store::platform::DirectoryLock::shared](../../src/platform.rs#L57) | function_item | `pub` |  |
| [store::platform::DirectoryLock::exclusive](../../src/platform.rs#L61) | function_item | `pub` |  |
| [store::platform::DirectoryLock::try_exclusive](../../src/platform.rs#L65) | function_item | `pub` |  |
| [store::platform::DirectoryLock::acquire](../../src/platform.rs#L69) | function_item | `private` |  |
| [store::platform::DirectoryLock::path](../../src/platform.rs#L82) | function_item | `pub` |  |
| [store::platform::RootLock](../../src/platform.rs#L87) | struct_item | `pub` |  |
| [store::platform::NamedLock](../../src/platform.rs#L92) | struct_item | `pub` |  |
| [store::platform::NamedLock::shared](../../src/platform.rs#L98) | function_item | `pub` |  |
| [store::platform::NamedLock::exclusive](../../src/platform.rs#L102) | function_item | `pub` |  |
| [store::platform::NamedLock::try_exclusive](../../src/platform.rs#L106) | function_item | `pub` |  |
| [store::platform::NamedLock::acquire](../../src/platform.rs#L110) | function_item | `private` |  |
| [store::platform::NamedLock::path](../../src/platform.rs#L129) | function_item | `pub` |  |
| [store::platform::NamedLock::drop](../../src/platform.rs#L135) | function_item | `private` |  |
| [store::platform::RootLock::acquire](../../src/platform.rs#L141) | function_item | `pub` |  |
| [store::platform::RootLock::path](../../src/platform.rs#L148) | function_item | `pub` |  |
| [store::platform::RootLock::drop](../../src/platform.rs#L154) | function_item | `private` |  |
| [store::platform::DirectoryLock::drop](../../src/platform.rs#L160) | function_item | `private` |  |
| [store::platform::try_lock_exclusive](../../src/platform.rs#L165) | function_item | `pub(crate)` |  |
| [store::platform::open_exclusive_create](../../src/platform.rs#L169) | function_item | `pub(crate)` |  |
| [store::platform::lock_operation](../../src/platform.rs#L182) | function_item | `private` |  |
| [store::platform::revalidate_inode](../../src/platform.rs#L210) | function_item | `private` |  |
| [store::platform::probe_local_filesystem](../../src/platform.rs#L219) | function_item | `pub` |  |
| [store::platform::reject_known_remote_filesystem](../../src/platform.rs#L265) | function_item | `private` | #[cfg(target_os = "macos")] |
| [store::platform::reject_known_remote_filesystem](../../src/platform.rs#L291) | function_item | `private` | #[cfg(not(target_os = "macos"))] |
| [store::platform::unlock](../../src/platform.rs#L296) | function_item | `pub(crate)` |  |
| [store::platform::probe_tests::concurrent_open_probes_do_not_remove_each_others_files](../../src/platform.rs#L314) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `CString` | `std::ffi::CString` | `private` |
| `File` | `std::fs::File` | `private` |
| `OpenOptions` | `std::fs::OpenOptions` | `private` |
| `io` | `std::io` | `private` |
| `AsRawFd` | `std::os::fd::AsRawFd` | `private` |
| `OsStrExt` | `std::os::unix::ffi::OsStrExt` | `private` |
| `MetadataExt` | `std::os::unix::fs::MetadataExt` | `private` |
| `OpenOptionsExt` | `std::os::unix::fs::OpenOptionsExt` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `AtomicU64` | `std::sync::atomic::AtomicU64` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `StoreError` | `crate::StoreError` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `store::platform::probe_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 18 direct edges</summary>

```mermaid
flowchart TD
  n0["store::platform::NamedLock::exclusive"]
  n1["store::platform::NamedLock::try_exclusive"]
  n2["store::platform::NamedLock::acquire"]
  n3["store::platform::NamedLock::path"]
  n4["store::platform::NamedLock::drop"]
  n5["store::platform::RootLock::acquire"]
  n6["store::platform::RootLock::path"]
  n7["store::platform::RootLock::drop"]
  n8["store::platform::DirectoryLock::drop"]
  n9["store::platform::try_lock_exclusive"]
  n10["store::platform::open_exclusive_create"]
  n11["store::platform::lock_operation"]
  n12["store::platform::revalidate_inode"]
  n13["store::platform::SystemSync::full_sync"]
  n14["store::platform::unlock"]
  n15["store::platform::FullSync::full_sync"]
  n16["store::platform::DirectoryLock::shared"]
  n17["store::platform::DirectoryLock::exclusive"]
  n18["store::platform::DirectoryLock::try_exclusive"]
  n19["store::platform::DirectoryLock::acquire"]
  n20["store::platform::DirectoryLock::path"]
  n21["store::platform::NamedLock::shared"]
  n0 --> n2
  n1 --> n2
  n2 --> n11
  n2 --> n12
  n4 --> n14
  n5 --> n10
  n7 --> n14
  n8 --> n14
  n9 --> n11
  n10 --> n11
  n10 --> n12
  n13 --> n15
  n16 --> n19
  n17 --> n19
  n18 --> n19
  n19 --> n11
  n19 --> n12
  n21 --> n2
```

</details>

<details><summary>Functions 21–25: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["store::platform::try_lock_exclusive"]
  n1["store::platform::revalidate_inode"]
  n2["store::platform::probe_local_filesystem"]
  n3["store::platform::reject_known_remote_filesystem"]
  n4["store::platform::reject_known_remote_filesystem"]
  n5["store::platform::unlock"]
  n6["store::platform::FullSync::full_sync"]
  n2 --> n0
  n2 --> n6
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `PROBE_ORDINAL` | `AtomicU64::new` | [10](../../src/platform.rs#L10) | external-constructor-callback-or-unresolved |
| `full_sync` | `FullSync::full_sync` | [23](../../src/platform.rs#L23) | [store::platform::FullSync::full_sync](../../src/platform.rs#L30) |
| `full_sync` | `libc::fcntl` | [36](../../src/platform.rs#L36) | external-constructor-callback-or-unresolved |
| `full_sync` | `file.as_raw_fd` | [36](../../src/platform.rs#L36) | receiver-type-required |
| `full_sync` | `Ok` | [38](../../src/platform.rs#L38) | external-constructor-callback-or-unresolved |
| `full_sync` | `io::Error::last_os_error` | [40](../../src/platform.rs#L40) | external-constructor-callback-or-unresolved |
| `full_sync` | `error.kind` | [41](../../src/platform.rs#L41) | receiver-type-required |
| `full_sync` | `Err` | [42](../../src/platform.rs#L42) | external-constructor-callback-or-unresolved |
| `full_sync` | `file.sync_all` | [47](../../src/platform.rs#L47) | receiver-type-required |
| `shared` | `Self::acquire` | [58](../../src/platform.rs#L58) | [store::platform::DirectoryLock::acquire](../../src/platform.rs#L69) |
| `exclusive` | `Self::acquire` | [62](../../src/platform.rs#L62) | [store::platform::DirectoryLock::acquire](../../src/platform.rs#L69) |
| `try_exclusive` | `Self::acquire` | [66](../../src/platform.rs#L66) | [store::platform::DirectoryLock::acquire](../../src/platform.rs#L69) |
| `acquire` | `path.as_ref().to_path_buf` | [74](../../src/platform.rs#L74) | receiver-type-required |
| `acquire` | `path.as_ref` | [74](../../src/platform.rs#L74) | receiver-type-required |
| `acquire` | `File::open` | [75](../../src/platform.rs#L75) | external-constructor-callback-or-unresolved |
| `acquire` | `lock_operation` | [76](../../src/platform.rs#L76) | [store::platform::lock_operation](../../src/platform.rs#L182) |
| `acquire` | `revalidate_inode` | [77](../../src/platform.rs#L77) | [store::platform::revalidate_inode](../../src/platform.rs#L210) |
| `acquire` | `Ok` | [78](../../src/platform.rs#L78) | external-constructor-callback-or-unresolved |
| `shared` | `Self::acquire` | [99](../../src/platform.rs#L99) | [store::platform::NamedLock::acquire](../../src/platform.rs#L110) |
| `exclusive` | `Self::acquire` | [103](../../src/platform.rs#L103) | [store::platform::NamedLock::acquire](../../src/platform.rs#L110) |
| `try_exclusive` | `Self::acquire` | [107](../../src/platform.rs#L107) | [store::platform::NamedLock::acquire](../../src/platform.rs#L110) |
| `acquire` | `path.as_ref().to_path_buf` | [115](../../src/platform.rs#L115) | receiver-type-required |
| `acquire` | `path.as_ref` | [115](../../src/platform.rs#L115) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write(true)             .create(true)             .mode(0o600)             .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)             .open` | [116](../../src/platform.rs#L116) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write(true)             .create(true)             .mode(0o600)             .custom_flags` | [116](../../src/platform.rs#L116) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write(true)             .create(true)             .mode` | [116](../../src/platform.rs#L116) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write(true)             .create` | [116](../../src/platform.rs#L116) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write` | [116](../../src/platform.rs#L116) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read` | [116](../../src/platform.rs#L116) | receiver-type-required |
| `acquire` | `OpenOptions::new` | [116](../../src/platform.rs#L116) | external-constructor-callback-or-unresolved |
| `acquire` | `lock_operation` | [123](../../src/platform.rs#L123) | [store::platform::lock_operation](../../src/platform.rs#L182) |
| `acquire` | `revalidate_inode` | [124](../../src/platform.rs#L124) | [store::platform::revalidate_inode](../../src/platform.rs#L210) |
| `acquire` | `Ok` | [125](../../src/platform.rs#L125) | external-constructor-callback-or-unresolved |
| `drop` | `unlock` | [136](../../src/platform.rs#L136) | [store::platform::unlock](../../src/platform.rs#L296) |
| `acquire` | `root.as_ref().join` | [142](../../src/platform.rs#L142) | receiver-type-required |
| `acquire` | `root.as_ref` | [142](../../src/platform.rs#L142) | receiver-type-required |
| `acquire` | `open_exclusive_create` | [143](../../src/platform.rs#L143) | [store::platform::open_exclusive_create](../../src/platform.rs#L169) |
| `acquire` | `Ok` | [144](../../src/platform.rs#L144) | external-constructor-callback-or-unresolved |
| `drop` | `unlock` | [155](../../src/platform.rs#L155) | [store::platform::unlock](../../src/platform.rs#L296) |
| `drop` | `unlock` | [161](../../src/platform.rs#L161) | [store::platform::unlock](../../src/platform.rs#L296) |
| `try_lock_exclusive` | `lock_operation` | [166](../../src/platform.rs#L166) | [store::platform::lock_operation](../../src/platform.rs#L182) |
| `open_exclusive_create` | `OpenOptions::new()         .read(true)         .write(true)         .create(true)         .mode(0o600)         .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)         .open` | [170](../../src/platform.rs#L170) | receiver-type-required |
| `open_exclusive_create` | `OpenOptions::new()         .read(true)         .write(true)         .create(true)         .mode(0o600)         .custom_flags` | [170](../../src/platform.rs#L170) | receiver-type-required |
| `open_exclusive_create` | `OpenOptions::new()         .read(true)         .write(true)         .create(true)         .mode` | [170](../../src/platform.rs#L170) | receiver-type-required |
| `open_exclusive_create` | `OpenOptions::new()         .read(true)         .write(true)         .create` | [170](../../src/platform.rs#L170) | receiver-type-required |
| `open_exclusive_create` | `OpenOptions::new()         .read(true)         .write` | [170](../../src/platform.rs#L170) | receiver-type-required |
| `open_exclusive_create` | `OpenOptions::new()         .read` | [170](../../src/platform.rs#L170) | receiver-type-required |
| `open_exclusive_create` | `OpenOptions::new` | [170](../../src/platform.rs#L170) | external-constructor-callback-or-unresolved |
| `open_exclusive_create` | `lock_operation` | [177](../../src/platform.rs#L177) | [store::platform::lock_operation](../../src/platform.rs#L182) |
| `open_exclusive_create` | `revalidate_inode` | [178](../../src/platform.rs#L178) | [store::platform::revalidate_inode](../../src/platform.rs#L210) |
| `open_exclusive_create` | `Ok` | [179](../../src/platform.rs#L179) | external-constructor-callback-or-unresolved |
| `lock_operation` | `libc::flock` | [195](../../src/platform.rs#L195) | external-constructor-callback-or-unresolved |
| `lock_operation` | `file.as_raw_fd` | [195](../../src/platform.rs#L195) | receiver-type-required |
| `lock_operation` | `Ok` | [197](../../src/platform.rs#L197) | external-constructor-callback-or-unresolved |
| `lock_operation` | `io::Error::last_os_error` | [199](../../src/platform.rs#L199) | external-constructor-callback-or-unresolved |
| `lock_operation` | `error.raw_os_error` | [200](../../src/platform.rs#L200) | receiver-type-required |
| `lock_operation` | `Err` | [202](../../src/platform.rs#L202), [205](../../src/platform.rs#L205) | external-constructor-callback-or-unresolved |
| `lock_operation` | `error.kind` | [204](../../src/platform.rs#L204) | receiver-type-required |
| `lock_operation` | `StoreError::Io` | [205](../../src/platform.rs#L205) | external-constructor-callback-or-unresolved |
| `revalidate_inode` | `file.metadata` | [211](../../src/platform.rs#L211) | receiver-type-required |
| `revalidate_inode` | `std::fs::metadata` | [212](../../src/platform.rs#L212) | external-constructor-callback-or-unresolved |
| `revalidate_inode` | `descriptor.dev` | [213](../../src/platform.rs#L213) | receiver-type-required |
| `revalidate_inode` | `pathname.dev` | [213](../../src/platform.rs#L213) | receiver-type-required |
| `revalidate_inode` | `descriptor.ino` | [213](../../src/platform.rs#L213) | receiver-type-required |
| `revalidate_inode` | `pathname.ino` | [213](../../src/platform.rs#L213) | receiver-type-required |
| `revalidate_inode` | `Err` | [214](../../src/platform.rs#L214) | external-constructor-callback-or-unresolved |
| `revalidate_inode` | `Ok` | [216](../../src/platform.rs#L216) | external-constructor-callback-or-unresolved |
| `probe_local_filesystem` | `root.as_ref` | [220](../../src/platform.rs#L220) | receiver-type-required |
| `probe_local_filesystem` | `root         .to_string_lossy()         .contains` | [221](../../src/platform.rs#L221) | receiver-type-required |
| `probe_local_filesystem` | `root         .to_string_lossy` | [221](../../src/platform.rs#L221) | receiver-type-required |
| `probe_local_filesystem` | `Err` | [225](../../src/platform.rs#L225), [254](../../src/platform.rs#L254) | external-constructor-callback-or-unresolved |
| `probe_local_filesystem` | `StoreError::UnsupportedFilesystem` | [225](../../src/platform.rs#L225), [254](../../src/platform.rs#L254) | external-constructor-callback-or-unresolved |
| `probe_local_filesystem` | `"iCloud-synchronized storage is not supported".to_owned` | [226](../../src/platform.rs#L226) | receiver-type-required |
| `probe_local_filesystem` | `std::fs::create_dir_all` | [229](../../src/platform.rs#L229) | external-constructor-callback-or-unresolved |
| `probe_local_filesystem` | `reject_known_remote_filesystem` | [230](../../src/platform.rs#L230) | ambiguous-cfg-or-overload |
| `probe_local_filesystem` | `root.join` | [231](../../src/platform.rs#L231) | receiver-type-required |
| `probe_local_filesystem` | `(&#124;&#124; {         let mut first = OpenOptions::new()             .read(true)             .append(true)             .create_new(true)             .mode(0o600)             .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)             .open(&path)?;         use std::io::Write as _;         first.write_all(b"probe\n")?;         FullSync::full_sync(&first)?;         try_lock_exclusive(&first)?;         let second = OpenOptions::new()             .read(true)             .append(true)             .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)             .open(&path)?;         if !matches!(try_lock_exclusive(&second), Err(StoreError::Busy)) {             return Err(StoreError::UnsupportedFilesystem(                 "flock contention probe did not report busy".to_owned(),             ));         }         Ok(())     })` | [236](../../src/platform.rs#L236) | external-constructor-callback-or-unresolved |
| `probe_local_filesystem` | `OpenOptions::new()             .read(true)             .append(true)             .create_new(true)             .mode(0o600)             .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)             .open` | [237](../../src/platform.rs#L237) | receiver-type-required |
| `probe_local_filesystem` | `OpenOptions::new()             .read(true)             .append(true)             .create_new(true)             .mode(0o600)             .custom_flags` | [237](../../src/platform.rs#L237) | receiver-type-required |
| `probe_local_filesystem` | `OpenOptions::new()             .read(true)             .append(true)             .create_new(true)             .mode` | [237](../../src/platform.rs#L237) | receiver-type-required |
| `probe_local_filesystem` | `OpenOptions::new()             .read(true)             .append(true)             .create_new` | [237](../../src/platform.rs#L237) | receiver-type-required |
| `probe_local_filesystem` | `OpenOptions::new()             .read(true)             .append` | [237](../../src/platform.rs#L237), [248](../../src/platform.rs#L248) | receiver-type-required |
| `probe_local_filesystem` | `OpenOptions::new()             .read` | [237](../../src/platform.rs#L237), [248](../../src/platform.rs#L248) | receiver-type-required |
| `probe_local_filesystem` | `OpenOptions::new` | [237](../../src/platform.rs#L237), [248](../../src/platform.rs#L248) | external-constructor-callback-or-unresolved |
| `probe_local_filesystem` | `first.write_all` | [245](../../src/platform.rs#L245) | receiver-type-required |
| `probe_local_filesystem` | `FullSync::full_sync` | [246](../../src/platform.rs#L246) | [store::platform::FullSync::full_sync](../../src/platform.rs#L30) |
| `probe_local_filesystem` | `try_lock_exclusive` | [247](../../src/platform.rs#L247) | [store::platform::try_lock_exclusive](../../src/platform.rs#L165) |
| `probe_local_filesystem` | `OpenOptions::new()             .read(true)             .append(true)             .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)             .open` | [248](../../src/platform.rs#L248) | receiver-type-required |
| `probe_local_filesystem` | `OpenOptions::new()             .read(true)             .append(true)             .custom_flags` | [248](../../src/platform.rs#L248) | receiver-type-required |
| `probe_local_filesystem` | `"flock contention probe did not report busy".to_owned` | [255](../../src/platform.rs#L255) | receiver-type-required |
| `probe_local_filesystem` | `Ok` | [258](../../src/platform.rs#L258) | external-constructor-callback-or-unresolved |
| `probe_local_filesystem` | `std::fs::remove_file` | [260](../../src/platform.rs#L260) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `CString::new(root.as_os_str().as_bytes())         .map_err` | [266](../../src/platform.rs#L266) | receiver-type-required |
| `reject_known_remote_filesystem` | `CString::new` | [266](../../src/platform.rs#L266) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `root.as_os_str().as_bytes` | [266](../../src/platform.rs#L266) | receiver-type-required |
| `reject_known_remote_filesystem` | `root.as_os_str` | [266](../../src/platform.rs#L266) | receiver-type-required |
| `reject_known_remote_filesystem` | `StoreError::UnsupportedFilesystem` | [267](../../src/platform.rs#L267), [283](../../src/platform.rs#L283) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `"storage path contains NUL".to_owned` | [267](../../src/platform.rs#L267) | receiver-type-required |
| `reject_known_remote_filesystem` | `std::mem::zeroed::<libc::statfs>` | [270](../../src/platform.rs#L270) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `libc::statfs` | [272](../../src/platform.rs#L272) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `path.as_ptr` | [272](../../src/platform.rs#L272) | receiver-type-required |
| `reject_known_remote_filesystem` | `Err` | [273](../../src/platform.rs#L273), [283](../../src/platform.rs#L283) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `StoreError::Io` | [273](../../src/platform.rs#L273) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `io::Error::last_os_error` | [273](../../src/platform.rs#L273) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `stat         .f_fstypename         .iter()         .map(&#124;value&#124; *value as u8)         .take_while(&#124;value&#124; *value != 0)         .collect::<Vec<_>>` | [275](../../src/platform.rs#L275) | receiver-type-required |
| `reject_known_remote_filesystem` | `stat         .f_fstypename         .iter()         .map(&#124;value&#124; *value as u8)         .take_while` | [275](../../src/platform.rs#L275) | receiver-type-required |
| `reject_known_remote_filesystem` | `stat         .f_fstypename         .iter()         .map` | [275](../../src/platform.rs#L275) | receiver-type-required |
| `reject_known_remote_filesystem` | `stat         .f_fstypename         .iter` | [275](../../src/platform.rs#L275) | receiver-type-required |
| `reject_known_remote_filesystem` | `String::from_utf8_lossy(&bytes).to_ascii_lowercase` | [281](../../src/platform.rs#L281) | receiver-type-required |
| `reject_known_remote_filesystem` | `String::from_utf8_lossy` | [281](../../src/platform.rs#L281) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `Ok` | [287](../../src/platform.rs#L287) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `CString::new` | [292](../../src/platform.rs#L292) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `Vec::<u8>::new` | [292](../../src/platform.rs#L292) | external-constructor-callback-or-unresolved |
| `reject_known_remote_filesystem` | `Ok` | [293](../../src/platform.rs#L293) | external-constructor-callback-or-unresolved |
| `unlock` | `libc::flock` | [300](../../src/platform.rs#L300) | external-constructor-callback-or-unresolved |
| `unlock` | `file.as_raw_fd` | [300](../../src/platform.rs#L300) | receiver-type-required |
| `unlock` | `Ok` | [302](../../src/platform.rs#L302) | external-constructor-callback-or-unresolved |
| `unlock` | `io::Error::last_os_error` | [304](../../src/platform.rs#L304) | external-constructor-callback-or-unresolved |
| `unlock` | `error.kind` | [305](../../src/platform.rs#L305) | receiver-type-required |
| `unlock` | `Err` | [306](../../src/platform.rs#L306) | external-constructor-callback-or-unresolved |
| `concurrent_open_probes_do_not_remove_each_others_files` | `tempfile::tempdir().expect` | [315](../../src/platform.rs#L315) | receiver-type-required |
| `concurrent_open_probes_do_not_remove_each_others_files` | `tempfile::tempdir` | [315](../../src/platform.rs#L315) | external-constructor-callback-or-unresolved |
| `concurrent_open_probes_do_not_remove_each_others_files` | `std::sync::Barrier::new` | [316](../../src/platform.rs#L316) | external-constructor-callback-or-unresolved |
| `concurrent_open_probes_do_not_remove_each_others_files` | `std::thread::scope` | [317](../../src/platform.rs#L317) | external-constructor-callback-or-unresolved |
| `concurrent_open_probes_do_not_remove_each_others_files` | `root.path` | [319](../../src/platform.rs#L319) | receiver-type-required |
| `concurrent_open_probes_do_not_remove_each_others_files` | `scope.spawn` | [321](../../src/platform.rs#L321) | receiver-type-required |
| `concurrent_open_probes_do_not_remove_each_others_files` | `barrier.wait` | [322](../../src/platform.rs#L322) | receiver-type-required |
| `concurrent_open_probes_do_not_remove_each_others_files` | `super::probe_local_filesystem(root).expect` | [323](../../src/platform.rs#L323) | receiver-type-required |
| `concurrent_open_probes_do_not_remove_each_others_files` | `super::probe_local_filesystem` | [323](../../src/platform.rs#L323) | [store::platform::probe_local_filesystem](../../src/platform.rs#L219) |
