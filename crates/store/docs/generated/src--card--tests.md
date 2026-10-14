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
| [store::card::tests::thread_with](../../src/card/tests.rs#L50) | function_item | `private` | test;  |
| [store::card::tests::write_for](../../src/card/tests.rs#L63) | function_item | `private` | test;  |
| [store::card::tests::input_for](../../src/card/tests.rs#L93) | function_item | `private` | test;  |
| [store::card::tests::verdict](../../src/card/tests.rs#L112) | function_item | `private` | test;  |
| [store::card::tests::card_roundtrips_and_rejects_every_kind_of_damage](../../src/card/tests.rs#L118) | function_item | `private` | test;  |
| [store::card::tests::freshness_follows_content_not_inode_or_mtime](../../src/card/tests.rs#L153) | function_item | `private` | test;  |
| [store::card::tests::any_edit_of_a_verified_byte_makes_the_card_cold](../../src/card/tests.rs#L195) | function_item | `private` | test;  |
| [store::card::tests::writer_skips_instead_of_claiming_bytes_it_cannot_pin](../../src/card/tests.rs#L250) | function_item | `private` | test;  |
| [store::card::tests::journal_identity_is_classified_without_parsing_rows](../../src/card/tests.rs#L293) | function_item | `private` | test;  |
| [store::card::tests::the_card_module_takes_no_locks](../../src/card/tests.rs#L346) | function_item | `private` | test;  |
| [store::card::tests::card_writes_never_disturb_a_worker_appending_under_the_flock](../../src/card/tests.rs#L369) | function_item | `private` | test;  |
| [store::card::tests::classify_every_card_of_a_real_root](../../src/card/tests.rs#L428) | function_item | `private` | test;  |
| [store::card::tests::a_file_that_shrinks_during_classification_is_stale_not_an_io_error](../../src/card/tests.rs#L452) | function_item | `private` | test;  |
| [store::card::tests::card_line_names_are_plain_jsonl_names](../../src/card/tests.rs#L485) | function_item | `private` | test;  |
| [store::card::tests::stale_card_temporaries_of_dead_processes_are_removed](../../src/card/tests.rs#L524) | function_item | `private` | test;  |

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
| `summary` | `"2026-09-04T00:00:00.000Z".into` | [39](../../src/card/tests.rs#L39) | receiver-type-required |
| `summary` | `Some` | [42](../../src/card/tests.rs#L42) | external-constructor-callback-or-unresolved |
| `summary` | `"coding".into` | [42](../../src/card/tests.rs#L42) | receiver-type-required |
| `thread_with` | `tempfile::tempdir().unwrap` | [51](../../src/card/tests.rs#L51) | receiver-type-required |
| `thread_with` | `tempfile::tempdir` | [51](../../src/card/tests.rs#L51) | external-constructor-callback-or-unresolved |
| `thread_with` | `root.path().join` | [52](../../src/card/tests.rs#L52) | receiver-type-required |
| `thread_with` | `root.path` | [52](../../src/card/tests.rs#L52) | receiver-type-required |
| `thread_with` | `std::fs::create_dir(&folder).unwrap` | [53](../../src/card/tests.rs#L53) | receiver-type-required |
| `thread_with` | `std::fs::create_dir` | [53](../../src/card/tests.rs#L53) | external-constructor-callback-or-unresolved |
| `thread_with` | `line` | [54](../../src/card/tests.rs#L54), [56](../../src/card/tests.rs#L56) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `thread_with` | `genesis` | [54](../../src/card/tests.rs#L54) | [store::card::tests::genesis](../../src/card/tests.rs#L15) |
| `thread_with` | `titles.iter().enumerate` | [55](../../src/card/tests.rs#L55) | receiver-type-required |
| `thread_with` | `titles.iter` | [55](../../src/card/tests.rs#L55) | receiver-type-required |
| `thread_with` | `bytes.extend` | [56](../../src/card/tests.rs#L56) | receiver-type-required |
| `thread_with` | `title` | [56](../../src/card/tests.rs#L56) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `thread_with` | `std::fs::write(folder.join("main.jsonl"), bytes).unwrap` | [58](../../src/card/tests.rs#L58) | receiver-type-required |
| `thread_with` | `std::fs::write` | [58](../../src/card/tests.rs#L58) | external-constructor-callback-or-unresolved |
| `thread_with` | `folder.join` | [58](../../src/card/tests.rs#L58) | receiver-type-required |
| `write_for` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [64](../../src/card/tests.rs#L64) | receiver-type-required |
| `write_for` | `std::fs::read` | [64](../../src/card/tests.rs#L64) | external-constructor-callback-or-unresolved |
| `write_for` | `folder.join` | [64](../../src/card/tests.rs#L64) | receiver-type-required |
| `write_for` | `scan_valid_prefix` | [65](../../src/card/tests.rs#L65) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `write_for` | `scan.projection.clone().unwrap` | [66](../../src/card/tests.rs#L66) | receiver-type-required |
| `write_for` | `scan.projection.clone` | [66](../../src/card/tests.rs#L66) | receiver-type-required |
| `write_for` | `"2026-09-04T00:00:00.000Z".into` | [70](../../src/card/tests.rs#L70) | receiver-type-required |
| `write_for` | `summary` | [87](../../src/card/tests.rs#L87) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `write_for` | `write_card(folder, &input).unwrap` | [90](../../src/card/tests.rs#L90) | receiver-type-required |
| `write_for` | `write_card` | [90](../../src/card/tests.rs#L90) | external-constructor-callback-or-unresolved |
| `input_for` | `"t".into` | [97](../../src/card/tests.rs#L97) | receiver-type-required |
| `input_for` | `summary` | [107](../../src/card/tests.rs#L107) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `verdict` | `read_card(folder).expect` | [113](../../src/card/tests.rs#L113) | receiver-type-required |
| `verdict` | `read_card` | [113](../../src/card/tests.rs#L113) | external-constructor-callback-or-unresolved |
| `verdict` | `classify(folder, &card).unwrap().freshness` | [114](../../src/card/tests.rs#L114) | receiver-type-required |
| `verdict` | `classify(folder, &card).unwrap` | [114](../../src/card/tests.rs#L114) | receiver-type-required |
| `verdict` | `classify` | [114](../../src/card/tests.rs#L114) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `thread_with` | [119](../../src/card/tests.rs#L119) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `read_card(&folder).expect` | [121](../../src/card/tests.rs#L121) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `read_card` | [121](../../src/card/tests.rs#L121) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::read(folder.join(CARD_FILE)).unwrap` | [125](../../src/card/tests.rs#L125) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::read` | [125](../../src/card/tests.rs#L125) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `folder.join` | [125](../../src/card/tests.rs#L125), [146](../../src/card/tests.rs#L146), [148](../../src/card/tests.rs#L148) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `bytes.clone` | [131](../../src/card/tests.rs#L131), [136](../../src/card/tests.rs#L136) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `bytes.len` | [132](../../src/card/tests.rs#L132) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `String::from_utf8(bytes.clone()).unwrap` | [136](../../src/card/tests.rs#L136) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `String::from_utf8` | [136](../../src/card/tests.rs#L136) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `text.replacen` | [137](../../src/card/tests.rs#L137), [139](../../src/card/tests.rs#L139) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `card.clone` | [141](../../src/card/tests.rs#L141) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write(folder.join(CARD_FILE), &flipped).unwrap` | [146](../../src/card/tests.rs#L146) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write` | [146](../../src/card/tests.rs#L146), [148](../../src/card/tests.rs#L148) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write(folder.join(CARD_FILE), b"").unwrap` | [148](../../src/card/tests.rs#L148) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `thread_with` | [154](../../src/card/tests.rs#L154) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `freshness_follows_content_not_inode_or_mtime` | `write_for` | [155](../../src/card/tests.rs#L155) | [store::card::tests::write_for](../../src/card/tests.rs#L63) |
| `freshness_follows_content_not_inode_or_mtime` | `folder.join` | [159](../../src/card/tests.rs#L159) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read(&ledger).unwrap` | [160](../../src/card/tests.rs#L160) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read` | [160](../../src/card/tests.rs#L160) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append(true)             .open(&ledger)             .unwrap` | [163](../../src/card/tests.rs#L163) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append(true)             .open` | [163](../../src/card/tests.rs#L163) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append` | [163](../../src/card/tests.rs#L163) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new` | [163](../../src/card/tests.rs#L163) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `file.write_all(&line(&title("d", 5))).unwrap` | [167](../../src/card/tests.rs#L167) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `file.write_all` | [167](../../src/card/tests.rs#L167) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `line` | [167](../../src/card/tests.rs#L167) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `freshness_follows_content_not_inode_or_mtime` | `title` | [167](../../src/card/tests.rs#L167) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::write(&ledger, &before).unwrap` | [173](../../src/card/tests.rs#L173) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::write` | [173](../../src/card/tests.rs#L173) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `tempfile::tempdir().unwrap` | [174](../../src/card/tests.rs#L174) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `tempfile::tempdir` | [174](../../src/card/tests.rs#L174) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `clone_parent.path().join` | [175](../../src/card/tests.rs#L175), [189](../../src/card/tests.rs#L189) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `clone_parent.path` | [175](../../src/card/tests.rs#L175), [189](../../src/card/tests.rs#L189) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir(&clone).unwrap` | [176](../../src/card/tests.rs#L176) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir` | [176](../../src/card/tests.rs#L176), [184](../../src/card/tests.rs#L184) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read_dir(&folder).unwrap` | [177](../../src/card/tests.rs#L177) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read_dir` | [177](../../src/card/tests.rs#L177) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `entry.unwrap` | [178](../../src/card/tests.rs#L178) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::copy(entry.path(), clone.join(entry.file_name())).unwrap` | [179](../../src/card/tests.rs#L179) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::copy` | [179](../../src/card/tests.rs#L179) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `entry.path` | [179](../../src/card/tests.rs#L179) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `clone.join` | [179](../../src/card/tests.rs#L179) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `entry.file_name` | [179](../../src/card/tests.rs#L179) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `root.path().join` | [183](../../src/card/tests.rs#L183) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `root.path` | [183](../../src/card/tests.rs#L183) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir(&archive).unwrap` | [184](../../src/card/tests.rs#L184) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename(&folder, archive.join(THREAD)).unwrap` | [185](../../src/card/tests.rs#L185) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename` | [185](../../src/card/tests.rs#L185), [190](../../src/card/tests.rs#L190) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `archive.join` | [185](../../src/card/tests.rs#L185) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename(&clone, &renamed).unwrap` | [190](../../src/card/tests.rs#L190) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `thread_with` | [196](../../src/card/tests.rs#L196) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `write_for` | [197](../../src/card/tests.rs#L197) | [store::card::tests::write_for](../../src/card/tests.rs#L63) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `folder.join` | [198](../../src/card/tests.rs#L198), [242](../../src/card/tests.rs#L242) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::read(&ledger).unwrap` | [199](../../src/card/tests.rs#L199) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::read` | [199](../../src/card/tests.rs#L199) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `read_card(&folder).unwrap` | [200](../../src/card/tests.rs#L200) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `read_card` | [200](../../src/card/tests.rs#L200) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &original[..original.len() - 1]).unwrap` | [204](../../src/card/tests.rs#L204) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write` | [204](../../src/card/tests.rs#L204), [210](../../src/card/tests.rs#L210), [215](../../src/card/tests.rs#L215), [226](../../src/card/tests.rs#L226), [238](../../src/card/tests.rs#L238), [241](../../src/card/tests.rs#L241) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `original.len` | [204](../../src/card/tests.rs#L204), [208](../../src/card/tests.rs#L208) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `original.clone` | [207](../../src/card/tests.rs#L207), [213](../../src/card/tests.rs#L213), [224](../../src/card/tests.rs#L224) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &tampered).unwrap` | [210](../../src/card/tests.rs#L210), [215](../../src/card/tests.rs#L215), [226](../../src/card/tests.rs#L226) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::remove_file(&ledger).unwrap` | [235](../../src/card/tests.rs#L235) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::remove_file` | [235](../../src/card/tests.rs#L235) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &original).unwrap` | [238](../../src/card/tests.rs#L238) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(         folder.join("018f0000-0000-7000-8000-000000000004.jsonl"),         line(&genesis()),     )     .unwrap` | [241](../../src/card/tests.rs#L241) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `line` | [243](../../src/card/tests.rs#L243) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `genesis` | [243](../../src/card/tests.rs#L243) | [store::card::tests::genesis](../../src/card/tests.rs#L15) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `thread_with` | [251](../../src/card/tests.rs#L251) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `folder.join` | [252](../../src/card/tests.rs#L252) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read(&ledger).unwrap` | [253](../../src/card/tests.rs#L253) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read` | [253](../../src/card/tests.rs#L253) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan_valid_prefix` | [254](../../src/card/tests.rs#L254) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan.projection.clone().unwrap` | [255](../../src/card/tests.rs#L255) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan.projection.clone` | [255](../../src/card/tests.rs#L255) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `input_for` | [256](../../src/card/tests.rs#L256) | [store::card::tests::input_for](../../src/card/tests.rs#L93) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `bytes.clone` | [258](../../src/card/tests.rs#L258) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `bytes.len` | [259](../../src/card/tests.rs#L259), [268](../../src/card/tests.rs#L268) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write(&ledger, &different).unwrap` | [261](../../src/card/tests.rs#L261) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write` | [261](../../src/card/tests.rs#L261), [268](../../src/card/tests.rs#L268) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write(&ledger, &bytes[..bytes.len() - 5]).unwrap` | [268](../../src/card/tests.rs#L268) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::remove_file(&ledger).unwrap` | [274](../../src/card/tests.rs#L274) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::remove_file` | [274](../../src/card/tests.rs#L274) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map(Result::ok)         .filter(&#124;entry&#124; entry.file_name().to_string_lossy().ends_with(".tmp"))         .count` | [284](../../src/card/tests.rs#L284) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map(Result::ok)         .filter` | [284](../../src/card/tests.rs#L284) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map` | [284](../../src/card/tests.rs#L284) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap` | [284](../../src/card/tests.rs#L284) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir` | [284](../../src/card/tests.rs#L284) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name().to_string_lossy().ends_with` | [287](../../src/card/tests.rs#L287) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name().to_string_lossy` | [287](../../src/card/tests.rs#L287) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name` | [287](../../src/card/tests.rs#L287) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `thread_with` | [294](../../src/card/tests.rs#L294) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `journal_identity_is_classified_without_parsing_rows` | `folder.join` | [295](../../src/card/tests.rs#L295), [298](../../src/card/tests.rs#L298) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write(&journal, rows).unwrap` | [297](../../src/card/tests.rs#L297) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write` | [297](../../src/card/tests.rs#L297), [338](../../src/card/tests.rs#L338) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [298](../../src/card/tests.rs#L298) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::read` | [298](../../src/card/tests.rs#L298) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `scan_valid_prefix` | [299](../../src/card/tests.rs#L299) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `journal_identity_is_classified_without_parsing_rows` | `scan.projection.clone().unwrap` | [300](../../src/card/tests.rs#L300) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `scan.projection.clone` | [300](../../src/card/tests.rs#L300) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `"t".into` | [304](../../src/card/tests.rs#L304) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `summary` | [314](../../src/card/tests.rs#L314) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `journal_identity_is_classified_without_parsing_rows` | `Some` | [315](../../src/card/tests.rs#L315) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `rows.len` | [316](../../src/card/tests.rs#L316) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `read_card(&folder).unwrap` | [323](../../src/card/tests.rs#L323) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `read_card` | [323](../../src/card/tests.rs#L323) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `classify(&folder, &card).unwrap` | [324](../../src/card/tests.rs#L324) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `classify` | [324](../../src/card/tests.rs#L324) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append(true)             .open(&journal)             .unwrap` | [328](../../src/card/tests.rs#L328) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append(true)             .open` | [328](../../src/card/tests.rs#L328) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append` | [328](../../src/card/tests.rs#L328) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new` | [328](../../src/card/tests.rs#L328) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `file.write_all(b"{\"row\":2}\n").unwrap` | [332](../../src/card/tests.rs#L332) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `file.write_all` | [332](../../src/card/tests.rs#L332) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write(&journal, b"{\"row\":0,\"pad\":\"changed\"}\n").unwrap` | [338](../../src/card/tests.rs#L338) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `thread_with` | [373](../../src/card/tests.rs#L373) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `folder.join` | [374](../../src/card/tests.rs#L374) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `Arc::new` | [375](../../src/card/tests.rs#L375) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `AtomicBool::new` | [375](../../src/card/tests.rs#L375) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `ledger.clone` | [377](../../src/card/tests.rs#L377) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `Arc::clone` | [378](../../src/card/tests.rs#L378) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::thread::spawn` | [379](../../src/card/tests.rs#L379) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `LockedLedger::open(&ledger, schema::READER_VERSION).expect` | [384](../../src/card/tests.rs#L384) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `LockedLedger::open` | [384](../../src/card/tests.rs#L384) | [store::tail::LockedLedger::open](../../src/tail.rs#L186) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer                     .append(title(&format!("event-{seq}"), seq), seq % 7 == 0)                     .expect` | [386](../../src/card/tests.rs#L386) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer                     .append` | [386](../../src/card/tests.rs#L386) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `title` | [387](../../src/card/tests.rs#L387) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `done.store` | [390](../../src/card/tests.rs#L390) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer.next_seq` | [391](../../src/card/tests.rs#L391) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `done.load` | [395](../../src/card/tests.rs#L395) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `write_for` | [396](../../src/card/tests.rs#L396), [415](../../src/card/tests.rs#L415) | [store::card::tests::write_for](../../src/card/tests.rs#L63) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `read_card` | [400](../../src/card/tests.rs#L400), [417](../../src/card/tests.rs#L417) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `classify(&folder, &card).unwrap` | [401](../../src/card/tests.rs#L401) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `classify` | [401](../../src/card/tests.rs#L401) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `worker.join().unwrap` | [409](../../src/card/tests.rs#L409) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `worker.join` | [409](../../src/card/tests.rs#L409) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `read_card(&folder).unwrap` | [417](../../src/card/tests.rs#L417) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::fs::read(&ledger).unwrap` | [419](../../src/card/tests.rs#L419) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::fs::read` | [419](../../src/card/tests.rs#L419) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `std::env::var("TEKES_CARD_CHECK_ROOT").expect` | [429](../../src/card/tests.rs#L429) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::env::var` | [429](../../src/card/tests.rs#L429) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `std::fs::read_dir(Path::new(&root).join("threads")).unwrap` | [431](../../src/card/tests.rs#L431) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::fs::read_dir` | [431](../../src/card/tests.rs#L431) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `Path::new(&root).join` | [431](../../src/card/tests.rs#L431) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `Path::new` | [431](../../src/card/tests.rs#L431) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `entry.unwrap().path` | [432](../../src/card/tests.rs#L432) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `entry.unwrap` | [432](../../src/card/tests.rs#L432) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `folder.is_dir` | [433](../../src/card/tests.rs#L433) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::time::Instant::now` | [436](../../src/card/tests.rs#L436) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `read_card` | [437](../../src/card/tests.rs#L437) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `classify(&folder, &card).unwrap().freshness` | [439](../../src/card/tests.rs#L439) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `classify(&folder, &card).unwrap` | [439](../../src/card/tests.rs#L439) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `classify` | [439](../../src/card/tests.rs#L439) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `thread_with` | [456](../../src/card/tests.rs#L456) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `write_for` | [457](../../src/card/tests.rs#L457) | [store::card::tests::write_for](../../src/card/tests.rs#L63) |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `read_card(&folder).unwrap` | [458](../../src/card/tests.rs#L458) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `read_card` | [458](../../src/card/tests.rs#L458) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `folder.join` | [459](../../src/card/tests.rs#L459) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::read(&ledger).unwrap` | [460](../../src/card/tests.rs#L460) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::read` | [460](../../src/card/tests.rs#L460) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `open_nofollow(&ledger).unwrap` | [461](../../src/card/tests.rs#L461) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `open_nofollow` | [461](../../src/card/tests.rs#L461) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::write(&ledger, &original[..40]).unwrap` | [462](../../src/card/tests.rs#L462) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::write` | [462](../../src/card/tests.rs#L462) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `verify_prefix(         &file,         first.valid_bytes,         first.genesis_len,         &first.genesis_sha256,         first.tail_len,         &first.tail_sha256,         &first.samples,     )     .expect` | [465](../../src/card/tests.rs#L465) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `verify_prefix` | [465](../../src/card/tests.rs#L465) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::File::open(&ledger).unwrap` | [478](../../src/card/tests.rs#L478) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::File::open` | [478](../../src/card/tests.rs#L478) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `thread_with` | [505](../../src/card/tests.rs#L505) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `card_line_names_are_plain_jsonl_names` | `write_for` | [506](../../src/card/tests.rs#L506) | [store::card::tests::write_for](../../src/card/tests.rs#L63) |
| `card_line_names_are_plain_jsonl_names` | `read_card(&folder).unwrap` | [507](../../src/card/tests.rs#L507) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `read_card` | [507](../../src/card/tests.rs#L507) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `"../escape.jsonl".into` | [508](../../src/card/tests.rs#L508) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [510](../../src/card/tests.rs#L510) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `std::fs::read` | [510](../../src/card/tests.rs#L510) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `folder.join` | [510](../../src/card/tests.rs#L510) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `scan_valid_prefix` | [511](../../src/card/tests.rs#L511) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `card_line_names_are_plain_jsonl_names` | `scan.projection.unwrap` | [512](../../src/card/tests.rs#L512) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `input_for` | [513](../../src/card/tests.rs#L513) | [store::card::tests::input_for](../../src/card/tests.rs#L93) |
| `stale_card_temporaries_of_dead_processes_are_removed` | `thread_with` | [525](../../src/card/tests.rs#L525) | [store::card::tests::thread_with](../../src/card/tests.rs#L50) |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::Command::new("true").spawn().unwrap` | [526](../../src/card/tests.rs#L526) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::Command::new("true").spawn` | [526](../../src/card/tests.rs#L526) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::Command::new` | [526](../../src/card/tests.rs#L526) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `child.id` | [527](../../src/card/tests.rs#L527) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `child.wait().unwrap` | [528](../../src/card/tests.rs#L528) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `child.wait` | [528](../../src/card/tests.rs#L528) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::os::unix::process::parent_id` | [529](../../src/card/tests.rs#L529) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::process::id` | [530](../../src/card/tests.rs#L530) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `"main.jsonl.tmp".to_owned` | [535](../../src/card/tests.rs#L535) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::fs::write(folder.join(name), b"x").unwrap` | [537](../../src/card/tests.rs#L537) | receiver-type-required |
| `stale_card_temporaries_of_dead_processes_are_removed` | `std::fs::write` | [537](../../src/card/tests.rs#L537) | external-constructor-callback-or-unresolved |
| `stale_card_temporaries_of_dead_processes_are_removed` | `folder.join` | [537](../../src/card/tests.rs#L537) | receiver-type-required |
