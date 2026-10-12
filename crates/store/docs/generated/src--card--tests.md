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
| [store::card::tests::thread_with](../../src/card/tests.rs#L49) | function_item | `private` | test;  |
| [store::card::tests::write_for](../../src/card/tests.rs#L62) | function_item | `private` | test;  |
| [store::card::tests::input_for](../../src/card/tests.rs#L92) | function_item | `private` | test;  |
| [store::card::tests::verdict](../../src/card/tests.rs#L111) | function_item | `private` | test;  |
| [store::card::tests::card_roundtrips_and_rejects_every_kind_of_damage](../../src/card/tests.rs#L117) | function_item | `private` | test;  |
| [store::card::tests::freshness_follows_content_not_inode_or_mtime](../../src/card/tests.rs#L152) | function_item | `private` | test;  |
| [store::card::tests::any_edit_of_a_verified_byte_makes_the_card_cold](../../src/card/tests.rs#L194) | function_item | `private` | test;  |
| [store::card::tests::writer_skips_instead_of_claiming_bytes_it_cannot_pin](../../src/card/tests.rs#L249) | function_item | `private` | test;  |
| [store::card::tests::journal_identity_is_classified_without_parsing_rows](../../src/card/tests.rs#L292) | function_item | `private` | test;  |
| [store::card::tests::the_card_module_takes_no_locks](../../src/card/tests.rs#L345) | function_item | `private` | test;  |
| [store::card::tests::card_writes_never_disturb_a_worker_appending_under_the_flock](../../src/card/tests.rs#L368) | function_item | `private` | test;  |
| [store::card::tests::classify_every_card_of_a_real_root](../../src/card/tests.rs#L426) | function_item | `private` | test;  |
| [store::card::tests::a_file_that_shrinks_during_classification_is_stale_not_an_io_error](../../src/card/tests.rs#L450) | function_item | `private` | test;  |
| [store::card::tests::card_line_names_are_plain_jsonl_names](../../src/card/tests.rs#L483) | function_item | `private` | test;  |

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
| `thread_with` | `tempfile::tempdir().unwrap` | [50](../../src/card/tests.rs#L50) | receiver-type-required |
| `thread_with` | `tempfile::tempdir` | [50](../../src/card/tests.rs#L50) | external-constructor-callback-or-unresolved |
| `thread_with` | `root.path().join` | [51](../../src/card/tests.rs#L51) | receiver-type-required |
| `thread_with` | `root.path` | [51](../../src/card/tests.rs#L51) | receiver-type-required |
| `thread_with` | `std::fs::create_dir(&folder).unwrap` | [52](../../src/card/tests.rs#L52) | receiver-type-required |
| `thread_with` | `std::fs::create_dir` | [52](../../src/card/tests.rs#L52) | external-constructor-callback-or-unresolved |
| `thread_with` | `line` | [53](../../src/card/tests.rs#L53), [55](../../src/card/tests.rs#L55) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `thread_with` | `genesis` | [53](../../src/card/tests.rs#L53) | [store::card::tests::genesis](../../src/card/tests.rs#L15) |
| `thread_with` | `titles.iter().enumerate` | [54](../../src/card/tests.rs#L54) | receiver-type-required |
| `thread_with` | `titles.iter` | [54](../../src/card/tests.rs#L54) | receiver-type-required |
| `thread_with` | `bytes.extend` | [55](../../src/card/tests.rs#L55) | receiver-type-required |
| `thread_with` | `title` | [55](../../src/card/tests.rs#L55) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `thread_with` | `std::fs::write(folder.join("main.jsonl"), bytes).unwrap` | [57](../../src/card/tests.rs#L57) | receiver-type-required |
| `thread_with` | `std::fs::write` | [57](../../src/card/tests.rs#L57) | external-constructor-callback-or-unresolved |
| `thread_with` | `folder.join` | [57](../../src/card/tests.rs#L57) | receiver-type-required |
| `write_for` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [63](../../src/card/tests.rs#L63) | receiver-type-required |
| `write_for` | `std::fs::read` | [63](../../src/card/tests.rs#L63) | external-constructor-callback-or-unresolved |
| `write_for` | `folder.join` | [63](../../src/card/tests.rs#L63) | receiver-type-required |
| `write_for` | `scan_valid_prefix` | [64](../../src/card/tests.rs#L64) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `write_for` | `scan.projection.clone().unwrap` | [65](../../src/card/tests.rs#L65) | receiver-type-required |
| `write_for` | `scan.projection.clone` | [65](../../src/card/tests.rs#L65) | receiver-type-required |
| `write_for` | `"2026-09-04T00:00:00.000Z".into` | [69](../../src/card/tests.rs#L69) | receiver-type-required |
| `write_for` | `summary` | [86](../../src/card/tests.rs#L86) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `write_for` | `write_card(folder, &input).unwrap` | [89](../../src/card/tests.rs#L89) | receiver-type-required |
| `write_for` | `write_card` | [89](../../src/card/tests.rs#L89) | external-constructor-callback-or-unresolved |
| `input_for` | `"t".into` | [96](../../src/card/tests.rs#L96) | receiver-type-required |
| `input_for` | `summary` | [106](../../src/card/tests.rs#L106) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `verdict` | `read_card(folder).expect` | [112](../../src/card/tests.rs#L112) | receiver-type-required |
| `verdict` | `read_card` | [112](../../src/card/tests.rs#L112) | external-constructor-callback-or-unresolved |
| `verdict` | `classify(folder, &card).unwrap().freshness` | [113](../../src/card/tests.rs#L113) | receiver-type-required |
| `verdict` | `classify(folder, &card).unwrap` | [113](../../src/card/tests.rs#L113) | receiver-type-required |
| `verdict` | `classify` | [113](../../src/card/tests.rs#L113) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `thread_with` | [118](../../src/card/tests.rs#L118) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `read_card(&folder).expect` | [120](../../src/card/tests.rs#L120) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `read_card` | [120](../../src/card/tests.rs#L120) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::read(folder.join(CARD_FILE)).unwrap` | [124](../../src/card/tests.rs#L124) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::read` | [124](../../src/card/tests.rs#L124) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `folder.join` | [124](../../src/card/tests.rs#L124), [145](../../src/card/tests.rs#L145), [147](../../src/card/tests.rs#L147) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `bytes.clone` | [130](../../src/card/tests.rs#L130), [135](../../src/card/tests.rs#L135) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `bytes.len` | [131](../../src/card/tests.rs#L131) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `String::from_utf8(bytes.clone()).unwrap` | [135](../../src/card/tests.rs#L135) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `String::from_utf8` | [135](../../src/card/tests.rs#L135) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `text.replacen` | [136](../../src/card/tests.rs#L136), [138](../../src/card/tests.rs#L138) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `card.clone` | [140](../../src/card/tests.rs#L140) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write(folder.join(CARD_FILE), &flipped).unwrap` | [145](../../src/card/tests.rs#L145) | receiver-type-required |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write` | [145](../../src/card/tests.rs#L145), [147](../../src/card/tests.rs#L147) | external-constructor-callback-or-unresolved |
| `card_roundtrips_and_rejects_every_kind_of_damage` | `std::fs::write(folder.join(CARD_FILE), b"").unwrap` | [147](../../src/card/tests.rs#L147) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `thread_with` | [153](../../src/card/tests.rs#L153) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `freshness_follows_content_not_inode_or_mtime` | `write_for` | [154](../../src/card/tests.rs#L154) | [store::card::tests::write_for](../../src/card/tests.rs#L62) |
| `freshness_follows_content_not_inode_or_mtime` | `folder.join` | [158](../../src/card/tests.rs#L158) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read(&ledger).unwrap` | [159](../../src/card/tests.rs#L159) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read` | [159](../../src/card/tests.rs#L159) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append(true)             .open(&ledger)             .unwrap` | [162](../../src/card/tests.rs#L162) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append(true)             .open` | [162](../../src/card/tests.rs#L162) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new()             .append` | [162](../../src/card/tests.rs#L162) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::OpenOptions::new` | [162](../../src/card/tests.rs#L162) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `file.write_all(&line(&title("d", 5))).unwrap` | [166](../../src/card/tests.rs#L166) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `file.write_all` | [166](../../src/card/tests.rs#L166) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `line` | [166](../../src/card/tests.rs#L166) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `freshness_follows_content_not_inode_or_mtime` | `title` | [166](../../src/card/tests.rs#L166) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::write(&ledger, &before).unwrap` | [172](../../src/card/tests.rs#L172) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::write` | [172](../../src/card/tests.rs#L172) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `tempfile::tempdir().unwrap` | [173](../../src/card/tests.rs#L173) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `tempfile::tempdir` | [173](../../src/card/tests.rs#L173) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `clone_parent.path().join` | [174](../../src/card/tests.rs#L174), [188](../../src/card/tests.rs#L188) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `clone_parent.path` | [174](../../src/card/tests.rs#L174), [188](../../src/card/tests.rs#L188) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir(&clone).unwrap` | [175](../../src/card/tests.rs#L175) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir` | [175](../../src/card/tests.rs#L175), [183](../../src/card/tests.rs#L183) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read_dir(&folder).unwrap` | [176](../../src/card/tests.rs#L176) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::read_dir` | [176](../../src/card/tests.rs#L176) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `entry.unwrap` | [177](../../src/card/tests.rs#L177) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::copy(entry.path(), clone.join(entry.file_name())).unwrap` | [178](../../src/card/tests.rs#L178) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::copy` | [178](../../src/card/tests.rs#L178) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `entry.path` | [178](../../src/card/tests.rs#L178) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `clone.join` | [178](../../src/card/tests.rs#L178) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `entry.file_name` | [178](../../src/card/tests.rs#L178) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `root.path().join` | [182](../../src/card/tests.rs#L182) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `root.path` | [182](../../src/card/tests.rs#L182) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::create_dir(&archive).unwrap` | [183](../../src/card/tests.rs#L183) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename(&folder, archive.join(THREAD)).unwrap` | [184](../../src/card/tests.rs#L184) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename` | [184](../../src/card/tests.rs#L184), [189](../../src/card/tests.rs#L189) | external-constructor-callback-or-unresolved |
| `freshness_follows_content_not_inode_or_mtime` | `archive.join` | [184](../../src/card/tests.rs#L184) | receiver-type-required |
| `freshness_follows_content_not_inode_or_mtime` | `std::fs::rename(&clone, &renamed).unwrap` | [189](../../src/card/tests.rs#L189) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `thread_with` | [195](../../src/card/tests.rs#L195) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `write_for` | [196](../../src/card/tests.rs#L196) | [store::card::tests::write_for](../../src/card/tests.rs#L62) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `folder.join` | [197](../../src/card/tests.rs#L197), [241](../../src/card/tests.rs#L241) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::read(&ledger).unwrap` | [198](../../src/card/tests.rs#L198) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::read` | [198](../../src/card/tests.rs#L198) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `read_card(&folder).unwrap` | [199](../../src/card/tests.rs#L199) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `read_card` | [199](../../src/card/tests.rs#L199) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &original[..original.len() - 1]).unwrap` | [203](../../src/card/tests.rs#L203) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write` | [203](../../src/card/tests.rs#L203), [209](../../src/card/tests.rs#L209), [214](../../src/card/tests.rs#L214), [225](../../src/card/tests.rs#L225), [237](../../src/card/tests.rs#L237), [240](../../src/card/tests.rs#L240) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `original.len` | [203](../../src/card/tests.rs#L203), [207](../../src/card/tests.rs#L207) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `original.clone` | [206](../../src/card/tests.rs#L206), [212](../../src/card/tests.rs#L212), [223](../../src/card/tests.rs#L223) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &tampered).unwrap` | [209](../../src/card/tests.rs#L209), [214](../../src/card/tests.rs#L214), [225](../../src/card/tests.rs#L225) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::remove_file(&ledger).unwrap` | [234](../../src/card/tests.rs#L234) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::remove_file` | [234](../../src/card/tests.rs#L234) | external-constructor-callback-or-unresolved |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(&ledger, &original).unwrap` | [237](../../src/card/tests.rs#L237) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `std::fs::write(         folder.join("018f0000-0000-7000-8000-000000000004.jsonl"),         line(&genesis()),     )     .unwrap` | [240](../../src/card/tests.rs#L240) | receiver-type-required |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `line` | [242](../../src/card/tests.rs#L242) | [store::card::tests::line](../../src/card/tests.rs#L29) |
| `any_edit_of_a_verified_byte_makes_the_card_cold` | `genesis` | [242](../../src/card/tests.rs#L242) | [store::card::tests::genesis](../../src/card/tests.rs#L15) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `thread_with` | [250](../../src/card/tests.rs#L250) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `folder.join` | [251](../../src/card/tests.rs#L251) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read(&ledger).unwrap` | [252](../../src/card/tests.rs#L252) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read` | [252](../../src/card/tests.rs#L252) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan_valid_prefix` | [253](../../src/card/tests.rs#L253) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan.projection.clone().unwrap` | [254](../../src/card/tests.rs#L254) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `scan.projection.clone` | [254](../../src/card/tests.rs#L254) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `input_for` | [255](../../src/card/tests.rs#L255) | [store::card::tests::input_for](../../src/card/tests.rs#L92) |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `bytes.clone` | [257](../../src/card/tests.rs#L257) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `bytes.len` | [258](../../src/card/tests.rs#L258), [267](../../src/card/tests.rs#L267) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write(&ledger, &different).unwrap` | [260](../../src/card/tests.rs#L260) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write` | [260](../../src/card/tests.rs#L260), [267](../../src/card/tests.rs#L267) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::write(&ledger, &bytes[..bytes.len() - 5]).unwrap` | [267](../../src/card/tests.rs#L267) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::remove_file(&ledger).unwrap` | [273](../../src/card/tests.rs#L273) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::remove_file` | [273](../../src/card/tests.rs#L273) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map(Result::ok)         .filter(&#124;entry&#124; entry.file_name().to_string_lossy().ends_with(".tmp"))         .count` | [283](../../src/card/tests.rs#L283) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map(Result::ok)         .filter` | [283](../../src/card/tests.rs#L283) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap()         .filter_map` | [283](../../src/card/tests.rs#L283) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir(&folder)         .unwrap` | [283](../../src/card/tests.rs#L283) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `std::fs::read_dir` | [283](../../src/card/tests.rs#L283) | external-constructor-callback-or-unresolved |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name().to_string_lossy().ends_with` | [286](../../src/card/tests.rs#L286) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name().to_string_lossy` | [286](../../src/card/tests.rs#L286) | receiver-type-required |
| `writer_skips_instead_of_claiming_bytes_it_cannot_pin` | `entry.file_name` | [286](../../src/card/tests.rs#L286) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `thread_with` | [293](../../src/card/tests.rs#L293) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `journal_identity_is_classified_without_parsing_rows` | `folder.join` | [294](../../src/card/tests.rs#L294), [297](../../src/card/tests.rs#L297) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write(&journal, rows).unwrap` | [296](../../src/card/tests.rs#L296) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write` | [296](../../src/card/tests.rs#L296), [337](../../src/card/tests.rs#L337) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [297](../../src/card/tests.rs#L297) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::read` | [297](../../src/card/tests.rs#L297) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `scan_valid_prefix` | [298](../../src/card/tests.rs#L298) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `journal_identity_is_classified_without_parsing_rows` | `scan.projection.clone().unwrap` | [299](../../src/card/tests.rs#L299) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `scan.projection.clone` | [299](../../src/card/tests.rs#L299) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `"t".into` | [303](../../src/card/tests.rs#L303) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `summary` | [313](../../src/card/tests.rs#L313) | [store::card::tests::summary](../../src/card/tests.rs#L35) |
| `journal_identity_is_classified_without_parsing_rows` | `Some` | [314](../../src/card/tests.rs#L314) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `rows.len` | [315](../../src/card/tests.rs#L315) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `read_card(&folder).unwrap` | [322](../../src/card/tests.rs#L322) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `read_card` | [322](../../src/card/tests.rs#L322) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `classify(&folder, &card).unwrap` | [323](../../src/card/tests.rs#L323) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `classify` | [323](../../src/card/tests.rs#L323) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append(true)             .open(&journal)             .unwrap` | [327](../../src/card/tests.rs#L327) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append(true)             .open` | [327](../../src/card/tests.rs#L327) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new()             .append` | [327](../../src/card/tests.rs#L327) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::OpenOptions::new` | [327](../../src/card/tests.rs#L327) | external-constructor-callback-or-unresolved |
| `journal_identity_is_classified_without_parsing_rows` | `file.write_all(b"{\"row\":2}\n").unwrap` | [331](../../src/card/tests.rs#L331) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `file.write_all` | [331](../../src/card/tests.rs#L331) | receiver-type-required |
| `journal_identity_is_classified_without_parsing_rows` | `std::fs::write(&journal, b"{\"row\":0,\"pad\":\"changed\"}\n").unwrap` | [337](../../src/card/tests.rs#L337) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `thread_with` | [372](../../src/card/tests.rs#L372) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `folder.join` | [373](../../src/card/tests.rs#L373) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `Arc::new` | [374](../../src/card/tests.rs#L374) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `AtomicBool::new` | [374](../../src/card/tests.rs#L374) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `ledger.clone` | [376](../../src/card/tests.rs#L376) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `Arc::clone` | [377](../../src/card/tests.rs#L377) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::thread::spawn` | [378](../../src/card/tests.rs#L378) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `LockedLedger::open(&ledger, 1).expect` | [382](../../src/card/tests.rs#L382) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `LockedLedger::open` | [382](../../src/card/tests.rs#L382) | [store::tail::LockedLedger::open](../../src/tail.rs#L186) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer                     .append(title(&format!("event-{seq}"), seq), seq % 7 == 0)                     .expect` | [384](../../src/card/tests.rs#L384) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer                     .append` | [384](../../src/card/tests.rs#L384) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `title` | [385](../../src/card/tests.rs#L385) | [store::card::tests::title](../../src/card/tests.rs#L25) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `done.store` | [388](../../src/card/tests.rs#L388) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `writer.next_seq` | [389](../../src/card/tests.rs#L389) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `done.load` | [393](../../src/card/tests.rs#L393) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `write_for` | [394](../../src/card/tests.rs#L394), [413](../../src/card/tests.rs#L413) | [store::card::tests::write_for](../../src/card/tests.rs#L62) |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `read_card` | [398](../../src/card/tests.rs#L398), [415](../../src/card/tests.rs#L415) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `classify(&folder, &card).unwrap` | [399](../../src/card/tests.rs#L399) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `classify` | [399](../../src/card/tests.rs#L399) | external-constructor-callback-or-unresolved |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `worker.join().unwrap` | [407](../../src/card/tests.rs#L407) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `worker.join` | [407](../../src/card/tests.rs#L407) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `read_card(&folder).unwrap` | [415](../../src/card/tests.rs#L415) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::fs::read(&ledger).unwrap` | [417](../../src/card/tests.rs#L417) | receiver-type-required |
| `card_writes_never_disturb_a_worker_appending_under_the_flock` | `std::fs::read` | [417](../../src/card/tests.rs#L417) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `std::env::var("TEKES_CARD_CHECK_ROOT").expect` | [427](../../src/card/tests.rs#L427) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::env::var` | [427](../../src/card/tests.rs#L427) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `std::fs::read_dir(Path::new(&root).join("threads")).unwrap` | [429](../../src/card/tests.rs#L429) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::fs::read_dir` | [429](../../src/card/tests.rs#L429) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `Path::new(&root).join` | [429](../../src/card/tests.rs#L429) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `Path::new` | [429](../../src/card/tests.rs#L429) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `entry.unwrap().path` | [430](../../src/card/tests.rs#L430) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `entry.unwrap` | [430](../../src/card/tests.rs#L430) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `folder.is_dir` | [431](../../src/card/tests.rs#L431) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `std::time::Instant::now` | [434](../../src/card/tests.rs#L434) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `read_card` | [435](../../src/card/tests.rs#L435) | external-constructor-callback-or-unresolved |
| `classify_every_card_of_a_real_root` | `classify(&folder, &card).unwrap().freshness` | [437](../../src/card/tests.rs#L437) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `classify(&folder, &card).unwrap` | [437](../../src/card/tests.rs#L437) | receiver-type-required |
| `classify_every_card_of_a_real_root` | `classify` | [437](../../src/card/tests.rs#L437) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `thread_with` | [454](../../src/card/tests.rs#L454) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `write_for` | [455](../../src/card/tests.rs#L455) | [store::card::tests::write_for](../../src/card/tests.rs#L62) |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `read_card(&folder).unwrap` | [456](../../src/card/tests.rs#L456) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `read_card` | [456](../../src/card/tests.rs#L456) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `folder.join` | [457](../../src/card/tests.rs#L457) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::read(&ledger).unwrap` | [458](../../src/card/tests.rs#L458) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::read` | [458](../../src/card/tests.rs#L458) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `open_nofollow(&ledger).unwrap` | [459](../../src/card/tests.rs#L459) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `open_nofollow` | [459](../../src/card/tests.rs#L459) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::write(&ledger, &original[..40]).unwrap` | [460](../../src/card/tests.rs#L460) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::write` | [460](../../src/card/tests.rs#L460) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `verify_prefix(         &file,         first.valid_bytes,         first.genesis_len,         &first.genesis_sha256,         first.tail_len,         &first.tail_sha256,         &first.samples,     )     .expect` | [463](../../src/card/tests.rs#L463) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `verify_prefix` | [463](../../src/card/tests.rs#L463) | external-constructor-callback-or-unresolved |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::File::open(&ledger).unwrap` | [476](../../src/card/tests.rs#L476) | receiver-type-required |
| `a_file_that_shrinks_during_classification_is_stale_not_an_io_error` | `std::fs::File::open` | [476](../../src/card/tests.rs#L476) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `thread_with` | [503](../../src/card/tests.rs#L503) | [store::card::tests::thread_with](../../src/card/tests.rs#L49) |
| `card_line_names_are_plain_jsonl_names` | `write_for` | [504](../../src/card/tests.rs#L504) | [store::card::tests::write_for](../../src/card/tests.rs#L62) |
| `card_line_names_are_plain_jsonl_names` | `read_card(&folder).unwrap` | [505](../../src/card/tests.rs#L505) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `read_card` | [505](../../src/card/tests.rs#L505) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `"../escape.jsonl".into` | [506](../../src/card/tests.rs#L506) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [508](../../src/card/tests.rs#L508) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `std::fs::read` | [508](../../src/card/tests.rs#L508) | external-constructor-callback-or-unresolved |
| `card_line_names_are_plain_jsonl_names` | `folder.join` | [508](../../src/card/tests.rs#L508) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `scan_valid_prefix` | [509](../../src/card/tests.rs#L509) | [store::tail::scan_valid_prefix](../../src/tail.rs#L37) |
| `card_line_names_are_plain_jsonl_names` | `scan.projection.unwrap` | [510](../../src/card/tests.rs#L510) | receiver-type-required |
| `card_line_names_are_plain_jsonl_names` | `input_for` | [511](../../src/card/tests.rs#L511) | [store::card::tests::input_for](../../src/card/tests.rs#L92) |
