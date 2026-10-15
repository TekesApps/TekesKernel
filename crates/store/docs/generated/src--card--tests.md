# store::card::tests

[Package atlas](index.md) · [Source](../../src/card/tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [store::card::tests::THREAD](../../src/card/tests.rs#L6) | const_item | `private` | test;  |
| [store::card::tests::event](../../src/card/tests.rs#L8) | function_item | `private` | test;  |
| [store::card::tests::genesis](../../src/card/tests.rs#L15) | function_item | `private` | test;  |
| [store::card::tests::title](../../src/card/tests.rs#L25) | function_item | `private` | test;  |
| [store::card::tests::line](../../src/card/tests.rs#L29) | function_item | `private` | test;  |
| [store::card::tests::summary](../../src/card/tests.rs#L35) | function_item | `private` | test;  |
| [store::card::tests::thread_with](../../src/card/tests.rs#L51) | function_item | `private` | test;  |
| [store::card::tests::write_for](../../src/card/tests.rs#L64) | function_item | `private` | test;  |
| [store::card::tests::input_for](../../src/card/tests.rs#L94) | function_item | `private` | test;  |
| [store::card::tests::verdict](../../src/card/tests.rs#L113) | function_item | `private` | test;  |
| [store::card::tests::card_roundtrips_and_rejects_every_kind_of_damage](../../src/card/tests.rs#L119) | function_item | `private` | test;  |
| [store::card::tests::freshness_follows_content_not_inode_or_mtime](../../src/card/tests.rs#L154) | function_item | `private` | test;  |
| [store::card::tests::any_edit_of_a_verified_byte_makes_the_card_cold](../../src/card/tests.rs#L196) | function_item | `private` | test;  |
| [store::card::tests::writer_skips_instead_of_claiming_bytes_it_cannot_pin](../../src/card/tests.rs#L251) | function_item | `private` | test;  |
| [store::card::tests::journal_identity_is_classified_without_parsing_rows](../../src/card/tests.rs#L294) | function_item | `private` | test;  |
| [store::card::tests::the_card_module_takes_no_locks](../../src/card/tests.rs#L347) | function_item | `private` | test;  |
| [store::card::tests::card_writes_never_disturb_a_worker_appending_under_the_flock](../../src/card/tests.rs#L370) | function_item | `private` | test;  |
| [store::card::tests::classify_every_card_of_a_real_root](../../src/card/tests.rs#L429) | function_item | `private` | test;  |
| [store::card::tests::a_file_that_shrinks_during_classification_is_stale_not_an_io_error](../../src/card/tests.rs#L453) | function_item | `private` | test;  |
| [store::card::tests::card_line_names_are_plain_jsonl_names](../../src/card/tests.rs#L486) | function_item | `private` | test;  |
| [store::card::tests::stale_card_temporaries_of_dead_processes_are_removed](../../src/card/tests.rs#L525) | function_item | `private` | test;  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `LockedLedger` | `crate::LockedLedger` | `private` |
| `scan_valid_prefix` | `crate::scan_valid_prefix` | `private` |
| `Event` | `schema::Event` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `event` | `Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap` | [12](../../src/card/tests.rs#L12) | receiver-type-required |
| `event` | `Event::decode_canonical` | [12](../../src/card/tests.rs#L12) | [schema::event::Event::decode_canonical](../../../schema/src/event.rs#L168) |
| `event` | `serde_json_canonicalizer::to_vec(&value).unwrap` | [12](../../src/card/tests.rs#L12) | receiver-type-required |
| `event` | `serde_json_canonicalizer::to_vec` | [12](../../src/card/tests.rs#L12) | external-constructor-callback-or-unresolved |
| `genesis` | `event` | [16](../../src/card/tests.rs#L16) | [store::card::tests::event](../../src/card/tests.rs#L8) |
| `title` | `event` | [26](../../src/card/tests.rs#L26) | [store::card::tests::event](../../src/card/tests.rs#L8) |
| `line` | `event.canonical_bytes().unwrap` | [30](../../src/card/tests.rs#L30) | receiver-type-required |
| `line` | `event.canonical_bytes` | [30](../../src/card/tests.rs#L30) | receiver-type-required |
| `line` | `bytes.push` | [31](../../src/card/tests.rs#L31) | receiver-type-required |
| `summary` | `"ws".into` | [37](../../src/card/tests.rs#L37) | receiver-type-required |
| `summary` | `"2026-09-04T00:00:00.000Z".into` | [40](../../src/card/tests.rs#L40) | receiver-type-required |
| `summary` | `Some` | [43](../../src/card/tests.rs#L43) | external-constructor-callback-or-unresolved |
| `summary` | `"coding".into` | [43](../../src/card/tests.rs#L43) | receiver-type-required |
| `thread_with` | `tempfile::tempdir().unwrap` | [52](../../src/card/tests.rs#L52) | receiver-type-required |
| `thread_with` | `tempfile::tempdir` | [52](../../src/card/tests.rs#L52) | external-constructor-callback-or-unresolved |
| `thread_with` | `root.path().join` | [53](../../src/card/tests.rs#L53) | receiver-type-required |
| `thread_with` | `root.path` | [53](../../src/card/tests.rs#L53) | receiver-type-required |
| `thread_with` | `std::fs::create_dir(&folder).unwrap` | [54](../../src/card/tests.rs#L54) | receiver-type-required |
| `thread_with` | `std::fs::create_dir` | [54](../../src/card/tests.rs#L54) | external-constructor-callback-or-unresolved |
| `thread_with` | `line` | [55](../../src/card/tests.rs#L55), [57](../../src/card/tests.rs#L57) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `thread_with` | `genesis` | [55](../../src/card/tests.rs#L55) | [store::card::tests::genesis](../../src/card/tests.rs#L15) |
| `thread_with` | `titles.iter().enumerate` | [56](../../src/card/tests.rs#L56) | receiver-type-required |
| `thread_with` | `titles.iter` | [56](../../src/card/tests.rs#L56) | receiver-type-required |
| `thread_with` | `bytes.extend` | [57](../../src/card/tests.rs#L57) | receiver-type-required |
| `thread_with` | `title` | [57](../../src/card/tests.rs#L57) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `thread_with` | `std::fs::write(folder.join("main.jsonl"), bytes).unwrap` | [59](../../src/card/tests.rs#L59) | receiver-type-required |
| `thread_with` | `std::fs::write` | [59](../../src/card/tests.rs#L59) | external-constructor-callback-or-unresolved |
| `thread_with` | `folder.join` | [59](../../src/card/tests.rs#L59) | receiver-type-required |
| `write_for` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [65](../../src/card/tests.rs#L65) | receiver-type-required |
| `write_for` | `std::fs::read` | [65](../../src/card/tests.rs#L65) | external-constructor-callback-or-unresolved |
| `write_for` | `folder.join` | [65](../../src/card/tests.rs#L65) | receiver-type-required |
| `write_for` | `scan_valid_prefix` | [66](../../src/card/tests.rs#L66) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `write_for` | `scan.projection.clone().unwrap` | [67](../../src/card/tests.rs#L67) | receiver-type-required |
| `write_for` | `scan.projection.clone` | [67](../../src/card/tests.rs#L67) | receiver-type-required |
| `write_for` | `"2026-09-04T00:00:00.000Z".into` | [71](../../src/card/tests.rs#L71) | receiver-type-required |
| `write_for` | `summary` | [88](../../src/card/tests.rs#L88) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `write_for` | `write_card(folder, &input).unwrap` | [91](../../src/card/tests.rs#L91) | receiver-type-required |
| `write_for` | `write_card` | [91](../../src/card/tests.rs#L91) | external-constructor-callback-or-unresolved |
| `input_for` | `"t".into` | [98](../../src/card/tests.rs#L98) | receiver-type-required |
| `input_for` | `summary` | [108](../../src/card/tests.rs#L108) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `verdict` | `read_card(folder).expect` | [114](../../src/card/tests.rs#L114) | receiver-type-required |
| `verdict` | `read_card` | [114](../../src/card/tests.rs#L114) | external-constructor-callback-or-unresolved |
| `verdict` | `classify(folder, &card).unwrap().freshness` | [115](../../src/card/tests.rs#L115) | receiver-type-required |
| `verdict` | `classify(folder, &card).unwrap` | [115](../../src/card/tests.rs#L115) | receiver-type-required |
| `verdict` | `classify` | [115](../../src/card/tests.rs#L115) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `thread_with` | [120](../../src/card/tests.rs#L120) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `read_card(&folder).expect` | [122](../../src/card/tests.rs#L122) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `read_card` | [122](../../src/card/tests.rs#L122) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::read(folder.join(CARD_FILE)).unwrap` | [126](../../src/card/tests.rs#L126) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::read` | [126](../../src/card/tests.rs#L126) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `folder.join` | [126](../../src/card/tests.rs#L126), [147](../../src/card/tests.rs#L147), [149](../../src/card/tests.rs#L149) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `bytes.clone` | [132](../../src/card/tests.rs#L132), [137](../../src/card/tests.rs#L137) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `bytes.len` | [133](../../src/card/tests.rs#L133) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `String::from_utf8(bytes.clone()).unwrap` | [137](../../src/card/tests.rs#L137) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `String::from_utf8` | [137](../../src/card/tests.rs#L137) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `text.replacen` | [138](../../src/card/tests.rs#L138), [140](../../src/card/tests.rs#L140) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `card.clone` | [142](../../src/card/tests.rs#L142) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write(folder.join(CARD_FILE), &flipped).unwrap` | [147](../../src/card/tests.rs#L147) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write` | [147](../../src/card/tests.rs#L147), [149](../../src/card/tests.rs#L149) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write(folder.join(CARD_FILE), b"").unwrap` | [149](../../src/card/tests.rs#L149) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `thread_with` | [155](../../src/card/tests.rs#L155) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `freshness_follows_content_not_inode_or_mtime` | `write_for` | [156](../../src/card/tests.rs#L156) | [store::card::tests::write_for](../../src/card/tests.rs#L64) |
| `freshness_follows_content_not_inode_or_mtime` | `folder.join` | [160](../../src/card/tests.rs#L160) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read(&ledger).unwrap` | [161](../../src/card/tests.rs#L161) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read` | [161](../../src/card/tests.rs#L161) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append(true)             .open(&ledger)             .unwrap` | [164](../../src/card/tests.rs#L164) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append(true)             .open` | [164](../../src/card/tests.rs#L164) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append` | [164](../../src/card/tests.rs#L164) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new` | [164](../../src/card/tests.rs#L164) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `file.write_all(&line(&title("d", 5))).unwrap` | [168](../../src/card/tests.rs#L168) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `file.write_all` | [168](../../src/card/tests.rs#L168) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `line` | [168](../../src/card/tests.rs#L168) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `freshness_follows_content_not_inode_or_mtime` | `title` | [168](../../src/card/tests.rs#L168) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::write(&ledger, &before).unwrap` | [174](../../src/card/tests.rs#L174) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::write` | [174](../../src/card/tests.rs#L174) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `tempfile::tempdir().unwrap` | [175](../../src/card/tests.rs#L175) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `tempfile::tempdir` | [175](../../src/card/tests.rs#L175) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `clone_parent.path().join` | [176](../../src/card/tests.rs#L176), [190](../../src/card/tests.rs#L190) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `clone_parent.path` | [176](../../src/card/tests.rs#L176), [190](../../src/card/tests.rs#L190) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir(&clone).unwrap` | [177](../../src/card/tests.rs#L177) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir` | [177](../../src/card/tests.rs#L177), [185](../../src/card/tests.rs#L185) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read_dir(&folder).unwrap` | [178](../../src/card/tests.rs#L178) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read_dir` | [178](../../src/card/tests.rs#L178) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `entry.unwrap` | [179](../../src/card/tests.rs#L179) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::copy(entry.path(), clone.join(entry.file_name())).unwrap` | [180](../../src/card/tests.rs#L180) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::copy` | [180](../../src/card/tests.rs#L180) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `entry.path` | [180](../../src/card/tests.rs#L180) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `clone.join` | [180](../../src/card/tests.rs#L180) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `entry.file_name` | [180](../../src/card/tests.rs#L180) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `root.path().join` | [184](../../src/card/tests.rs#L184) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `root.path` | [184](../../src/card/tests.rs#L184) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir(&archive).unwrap` | [185](../../src/card/tests.rs#L185) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename(&folder, archive.join(THREAD)).unwrap` | [186](../../src/card/tests.rs#L186) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename` | [186](../../src/card/tests.rs#L186), [191](../../src/card/tests.rs#L191) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `archive.join` | [186](../../src/card/tests.rs#L186) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename(&clone, &renamed).unwrap` | [191](../../src/card/tests.rs#L191) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `thread_with` | [197](../../src/card/tests.rs#L197) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `write_for` | [198](../../src/card/tests.rs#L198) | [store::card::tests::write_for](../../src/card/tests.rs#L64) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `folder.join` | [199](../../src/card/tests.rs#L199), [243](../../src/card/tests.rs#L243) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::read(&ledger).unwrap` | [200](../../src/card/tests.rs#L200) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::read` | [200](../../src/card/tests.rs#L200) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `read_card(&folder).unwrap` | [201](../../src/card/tests.rs#L201) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `read_card` | [201](../../src/card/tests.rs#L201) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &original[..original.len() - 1]).unwrap` | [205](../../src/card/tests.rs#L205) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write` | [205](../../src/card/tests.rs#L205), [211](../../src/card/tests.rs#L211), [216](../../src/card/tests.rs#L216), [227](../../src/card/tests.rs#L227), [239](../../src/card/tests.rs#L239), [242](../../src/card/tests.rs#L242) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `original.len` | [205](../../src/card/tests.rs#L205), [209](../../src/card/tests.rs#L209) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `original.clone` | [208](../../src/card/tests.rs#L208), [214](../../src/card/tests.rs#L214), [225](../../src/card/tests.rs#L225) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &tampered).unwrap` | [211](../../src/card/tests.rs#L211), [216](../../src/card/tests.rs#L216), [227](../../src/card/tests.rs#L227) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::remove_file(&ledger).unwrap` | [236](../../src/card/tests.rs#L236) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::remove_file` | [236](../../src/card/tests.rs#L236) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &original).unwrap` | [239](../../src/card/tests.rs#L239) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(         folder.join("018f0000-0000-7000-8000-000000000004.jsonl"),         line(&genesis()),     )     .unwrap` | [242](../../src/card/tests.rs#L242) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `line` | [244](../../src/card/tests.rs#L244) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `genesis` | [244](../../src/card/tests.rs#L244) | [store::card::tests::genesis](../../src/card/tests.rs#L15) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `thread_with` | [252](../../src/card/tests.rs#L252) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `folder.join` | [253](../../src/card/tests.rs#L253) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read(&ledger).unwrap` | [254](../../src/card/tests.rs#L254) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read` | [254](../../src/card/tests.rs#L254) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan_valid_prefix` | [255](../../src/card/tests.rs#L255) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan.projection.clone().unwrap` | [256](../../src/card/tests.rs#L256) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan.projection.clone` | [256](../../src/card/tests.rs#L256) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `input_for` | [257](../../src/card/tests.rs#L257) | [store::card::tests::input_for](../../src/card/tests.rs#L94) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `bytes.clone` | [259](../../src/card/tests.rs#L259) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `bytes.len` | [260](../../src/card/tests.rs#L260), [269](../../src/card/tests.rs#L269) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write(&ledger, &different).unwrap` | [262](../../src/card/tests.rs#L262) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write` | [262](../../src/card/tests.rs#L262), [269](../../src/card/tests.rs#L269) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write(&ledger, &bytes[..bytes.len() - 5]).unwrap` | [269](../../src/card/tests.rs#L269) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::remove_file(&ledger).unwrap` | [275](../../src/card/tests.rs#L275) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::remove_file` | [275](../../src/card/tests.rs#L275) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map(Result::ok)         .filter(&#124;entry&#124; entry.file_name().to_string_lossy().ends_with(".tmp"))         .count` | [285](../../src/card/tests.rs#L285) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map(Result::ok)         .filter` | [285](../../src/card/tests.rs#L285) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map` | [285](../../src/card/tests.rs#L285) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap` | [285](../../src/card/tests.rs#L285) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir` | [285](../../src/card/tests.rs#L285) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name().to_string_lossy().ends_with` | [288](../../src/card/tests.rs#L288) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name().to_string_lossy` | [288](../../src/card/tests.rs#L288) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name` | [288](../../src/card/tests.rs#L288) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `thread_with` | [295](../../src/card/tests.rs#L295) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `journal_identity_is_classified_without_parsing_rows` | `folder.join` | [296](../../src/card/tests.rs#L296), [299](../../src/card/tests.rs#L299) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write(&journal, rows).unwrap` | [298](../../src/card/tests.rs#L298) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write` | [298](../../src/card/tests.rs#L298), [339](../../src/card/tests.rs#L339) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [299](../../src/card/tests.rs#L299) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::read` | [299](../../src/card/tests.rs#L299) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `scan_valid_prefix` | [300](../../src/card/tests.rs#L300) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `journal_identity_is_classified_without_parsing_rows` | `scan.projection.clone().unwrap` | [301](../../src/card/tests.rs#L301) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `scan.projection.clone` | [301](../../src/card/tests.rs#L301) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `"t".into` | [305](../../src/card/tests.rs#L305) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `summary` | [315](../../src/card/tests.rs#L315) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `journal_identity_is_classified_without_parsing_rows` | `Some` | [316](../../src/card/tests.rs#L316) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `rows.len` | [317](../../src/card/tests.rs#L317) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `read_card(&folder).unwrap` | [324](../../src/card/tests.rs#L324) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `read_card` | [324](../../src/card/tests.rs#L324) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `classify(&folder, &card).unwrap` | [325](../../src/card/tests.rs#L325) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `classify` | [325](../../src/card/tests.rs#L325) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append(true)             .open(&journal)             .unwrap` | [329](../../src/card/tests.rs#L329) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append(true)             .open` | [329](../../src/card/tests.rs#L329) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append` | [329](../../src/card/tests.rs#L329) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new` | [329](../../src/card/tests.rs#L329) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `file.write_all(b"{\"row\":2}\n").unwrap` | [333](../../src/card/tests.rs#L333) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `file.write_all` | [333](../../src/card/tests.rs#L333) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write(&journal, b"{\"row\":0,\"pad\":\"changed\"}\n").unwrap` | [339](../../src/card/tests.rs#L339) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `thread_with` | [374](../../src/card/tests.rs#L374) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `folder.join` | [375](../../src/card/tests.rs#L375) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `Arc::new` | [376](../../src/card/tests.rs#L376) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `AtomicBool::new` | [376](../../src/card/tests.rs#L376) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `ledger.clone` | [378](../../src/card/tests.rs#L378) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `Arc::clone` | [379](../../src/card/tests.rs#L379) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::thread::spawn` | [380](../../src/card/tests.rs#L380) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `LockedLedger::open(&ledger, schema::READER_VERSION).expect` | [385](../../src/card/tests.rs#L385) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `LockedLedger::open` | [385](../../src/card/tests.rs#L385) | [store::tail::LockedLedger::open](../../src/tail.rs#L186) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer                     .append(title(&format!("event-{seq}"), seq), seq % 7 == 0)                     .expect` | [387](../../src/card/tests.rs#L387) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer                     .append` | [387](../../src/card/tests.rs#L387) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `title` | [388](../../src/card/tests.rs#L388) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `done.store` | [391](../../src/card/tests.rs#L391) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer.next_seq` | [392](../../src/card/tests.rs#L392) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `done.load` | [396](../../src/card/tests.rs#L396) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `write_for` | [397](../../src/card/tests.rs#L397), [416](../../src/card/tests.rs#L416) | [store::card::tests::write_for](../../src/card/tests.rs#L64) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `read_card` | [401](../../src/card/tests.rs#L401), [418](../../src/card/tests.rs#L418) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `classify(&folder, &card).unwrap` | [402](../../src/card/tests.rs#L402) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `classify` | [402](../../src/card/tests.rs#L402) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `worker.join().unwrap` | [410](../../src/card/tests.rs#L410) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `worker.join` | [410](../../src/card/tests.rs#L410) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `read_card(&folder).unwrap` | [418](../../src/card/tests.rs#L418) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::fs::read(&ledger).unwrap` | [420](../../src/card/tests.rs#L420) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::fs::read` | [420](../../src/card/tests.rs#L420) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `std::env::var("TEKES_CARD_CHECK_ROOT").expect` | [430](../../src/card/tests.rs#L430) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::env::var` | [430](../../src/card/tests.rs#L430) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `std::fs::read_dir(Path::new(&root).join("threads")).unwrap` | [432](../../src/card/tests.rs#L432) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::fs::read_dir` | [432](../../src/card/tests.rs#L432) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `Path::new(&root).join` | [432](../../src/card/tests.rs#L432) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `Path::new` | [432](../../src/card/tests.rs#L432) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `entry.unwrap().path` | [433](../../src/card/tests.rs#L433) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `entry.unwrap` | [433](../../src/card/tests.rs#L433) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `folder.is_dir` | [434](../../src/card/tests.rs#L434) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::time::Instant::now` | [437](../../src/card/tests.rs#L437) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `read_card` | [438](../../src/card/tests.rs#L438) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `classify(&folder, &card).unwrap().freshness` | [440](../../src/card/tests.rs#L440) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `classify(&folder, &card).unwrap` | [440](../../src/card/tests.rs#L440) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `classify` | [440](../../src/card/tests.rs#L440) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `thread_with` | [457](../../src/card/tests.rs#L457) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `write_for` | [458](../../src/card/tests.rs#L458) | [store::card::tests::write_for](../../src/card/tests.rs#L64) |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `read_card(&folder).unwrap` | [459](../../src/card/tests.rs#L459) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `read_card` | [459](../../src/card/tests.rs#L459) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `folder.join` | [460](../../src/card/tests.rs#L460) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::read(&ledger).unwrap` | [461](../../src/card/tests.rs#L461) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::read` | [461](../../src/card/tests.rs#L461) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `open_nofollow(&ledger).unwrap` | [462](../../src/card/tests.rs#L462) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `open_nofollow` | [462](../../src/card/tests.rs#L462) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::write(&ledger, &original[..40]).unwrap` | [463](../../src/card/tests.rs#L463) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::write` | [463](../../src/card/tests.rs#L463) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `verify_prefix(         &file,         first.valid_bytes,         first.genesis_len,         &first.genesis_sha256,         first.tail_len,         &first.tail_sha256,         &first.samples,     )     .expect` | [466](../../src/card/tests.rs#L466) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `verify_prefix` | [466](../../src/card/tests.rs#L466) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::File::open(&ledger).unwrap` | [479](../../src/card/tests.rs#L479) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::File::open` | [479](../../src/card/tests.rs#L479) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `thread_with` | [506](../../src/card/tests.rs#L506) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `card_line_names_are_plain_jsonl_names` | `write_for` | [507](../../src/card/tests.rs#L507) | [store::card::tests::write_for](../../src/card/tests.rs#L64) |
| `card_line_names_are_plain_jsonl_names` | `read_card(&folder).unwrap` | [508](../../src/card/tests.rs#L508) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `read_card` | [508](../../src/card/tests.rs#L508) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `"../escape.jsonl".into` | [509](../../src/card/tests.rs#L509) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [511](../../src/card/tests.rs#L511) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `std::fs::read` | [511](../../src/card/tests.rs#L511) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `folder.join` | [511](../../src/card/tests.rs#L511) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `scan_valid_prefix` | [512](../../src/card/tests.rs#L512) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `card_line_names_are_plain_jsonl_names` | `scan.projection.unwrap` | [513](../../src/card/tests.rs#L513) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `input_for` | [514](../../src/card/tests.rs#L514) | [store::card::tests::input_for](../../src/card/tests.rs#L94) |
| `stale_card_temporaries_of_dead_processes_are_removed` | `thread_with` | [526](../../src/card/tests.rs#L526) | [store::card::tests::thread_with](../../src/card/tests.rs#L51) |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::Command::new("true").spawn().unwrap` | [527](../../src/card/tests.rs#L527) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::Command::new("true").spawn` | [527](../../src/card/tests.rs#L527) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::Command::new` | [527](../../src/card/tests.rs#L527) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `child.id` | [528](../../src/card/tests.rs#L528) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `child.wait().unwrap` | [529](../../src/card/tests.rs#L529) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `child.wait` | [529](../../src/card/tests.rs#L529) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::os::unix::process::parent_id` | [530](../../src/card/tests.rs#L530) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::id` | [531](../../src/card/tests.rs#L531) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `"main.jsonl.tmp".to_owned` | [536](../../src/card/tests.rs#L536) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::fs::write(folder.join(name), b"x").unwrap` | [538](../../src/card/tests.rs#L538) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::fs::write` | [538](../../src/card/tests.rs#L538) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `folder.join` | [538](../../src/card/tests.rs#L538) | receiver-type-required |
