# provider::sse

[Package atlas](index.md) · [Source](../../src/sse.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [provider::sse::MAX_FRAME](../../src/sse.rs#L3) | const_item | `private` |  |
| [provider::sse::SseEvent](../../src/sse.rs#L6) | struct_item | `pub` |  |
| [provider::sse::SseError](../../src/sse.rs#L12) | enum_item | `pub` |  |
| [provider::sse::SseDecoder](../../src/sse.rs#L24) | struct_item | `pub` |  |
| [provider::sse::SseDecoder::new](../../src/sse.rs#L31) | function_item | `pub` |  |
| [provider::sse::SseDecoder::push](../../src/sse.rs#L35) | function_item | `pub` |  |
| [provider::sse::SseDecoder::finish](../../src/sse.rs#L74) | function_item | `pub` |  |
| [provider::sse::SseDecoder::finish_events](../../src/sse.rs#L82) | function_item | `pub` |  |
| [provider::sse::frame_end](../../src/sse.rs#L103) | function_item | `private` |  |
| [provider::sse::parse_frame](../../src/sse.rs#L117) | function_item | `private` |  |
| [provider::sse::tests::split_utf8_and_multiline_data_are_stable](../../src/sse.rs#L146) | function_item | `private` | test; #[cfg(test)] |
| [provider::sse::tests::one_large_transport_chunk_may_contain_many_small_frames](../../src/sse.rs#L158) | function_item | `private` | test; #[cfg(test)] |
| [provider::sse::tests::a_single_oversized_frame_is_rejected_even_with_a_delimiter](../../src/sse.rs#L176) | function_item | `private` | test; #[cfg(test)] |
| [provider::sse::tests::the_first_mixed_style_delimiter_wins](../../src/sse.rs#L186) | function_item | `private` | test; #[cfg(test)] |
| [provider::sse::tests::data_after_terminal_in_the_same_chunk_is_rejected](../../src/sse.rs#L207) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Error` | `thiserror::Error` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `provider::sse::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–6: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::sse::frame_end"]
  n1["provider::sse::parse_frame"]
  n2["provider::sse::SseDecoder::new"]
  n3["provider::sse::SseDecoder::push"]
  n4["provider::sse::SseDecoder::finish"]
  n5["provider::sse::SseDecoder::finish_events"]
  n3 --> n1
  n5 --> n1
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `new` | `Self::default` | [32](../../src/sse.rs#L32) | external-constructor-callback-or-unresolved |
| `push` | `chunk.is_empty` | [36](../../src/sse.rs#L36) | receiver-type-required |
| `push` | `Err` | [37](../../src/sse.rs#L37), [44](../../src/sse.rs#L44), [50](../../src/sse.rs#L50), [60](../../src/sse.rs#L60), [69](../../src/sse.rs#L69) | external-constructor-callback-or-unresolved |
| `push` | `self.bytes.extend_from_slice` | [39](../../src/sse.rs#L39) | receiver-type-required |
| `push` | `Vec::new` | [40](../../src/sse.rs#L40) | external-constructor-callback-or-unresolved |
| `push` | `frame_end` | [42](../../src/sse.rs#L42) | external-constructor-callback-or-unresolved |
| `push` | `parse_frame` | [52](../../src/sse.rs#L52) | [provider::sse::parse_frame](../../src/sse.rs#L117) |
| `push` | `event.event.as_deref` | [53](../../src/sse.rs#L53) | receiver-type-required |
| `push` | `Some` | [53](../../src/sse.rs#L53) | external-constructor-callback-or-unresolved |
| `push` | `events.push` | [56](../../src/sse.rs#L56) | receiver-type-required |
| `push` | `self.bytes.len` | [59](../../src/sse.rs#L59), [68](../../src/sse.rs#L68) | receiver-type-required |
| `push` | `self.bytes.drain` | [63](../../src/sse.rs#L63) | receiver-type-required |
| `push` | `Ok` | [71](../../src/sse.rs#L71) | external-constructor-callback-or-unresolved |
| `finish` | `self.bytes.iter().all` | [75](../../src/sse.rs#L75) | receiver-type-required |
| `finish` | `self.bytes.iter` | [75](../../src/sse.rs#L75) | receiver-type-required |
| `finish` | `Ok` | [76](../../src/sse.rs#L76) | external-constructor-callback-or-unresolved |
| `finish` | `Err` | [78](../../src/sse.rs#L78) | external-constructor-callback-or-unresolved |
| `finish_events` | `self.bytes.is_empty` | [83](../../src/sse.rs#L83) | receiver-type-required |
| `finish_events` | `self.bytes.iter().all` | [83](../../src/sse.rs#L83) | receiver-type-required |
| `finish_events` | `self.bytes.iter` | [83](../../src/sse.rs#L83) | receiver-type-required |
| `finish_events` | `Ok` | [84](../../src/sse.rs#L84), [99](../../src/sse.rs#L99) | external-constructor-callback-or-unresolved |
| `finish_events` | `Vec::new` | [84](../../src/sse.rs#L84) | external-constructor-callback-or-unresolved |
| `finish_events` | `Err` | [87](../../src/sse.rs#L87), [97](../../src/sse.rs#L97) | external-constructor-callback-or-unresolved |
| `finish_events` | `self             .bytes             .last()             .is_some_and` | [89](../../src/sse.rs#L89) | receiver-type-required |
| `finish_events` | `self             .bytes             .last` | [89](../../src/sse.rs#L89) | receiver-type-required |
| `finish_events` | `self.bytes.pop` | [94](../../src/sse.rs#L94) | receiver-type-required |
| `finish_events` | `self.bytes.len` | [96](../../src/sse.rs#L96) | receiver-type-required |
| `finish_events` | `parse_frame(&self.bytes)?.into_iter().collect` | [99](../../src/sse.rs#L99) | receiver-type-required |
| `finish_events` | `parse_frame(&self.bytes)?.into_iter` | [99](../../src/sse.rs#L99) | receiver-type-required |
| `finish_events` | `parse_frame` | [99](../../src/sse.rs#L99) | [provider::sse::parse_frame](../../src/sse.rs#L117) |
| `frame_end` | `bytes.len` | [105](../../src/sse.rs#L105), [109](../../src/sse.rs#L109) | receiver-type-required |
| `frame_end` | `Some` | [107](../../src/sse.rs#L107), [110](../../src/sse.rs#L110) | external-constructor-callback-or-unresolved |
| `parse_frame` | `std::str::from_utf8(bytes).map_err` | [118](../../src/sse.rs#L118) | receiver-type-required |
| `parse_frame` | `std::str::from_utf8` | [118](../../src/sse.rs#L118) | external-constructor-callback-or-unresolved |
| `parse_frame` | `Vec::new` | [120](../../src/sse.rs#L120) | external-constructor-callback-or-unresolved |
| `parse_frame` | `text.lines` | [121](../../src/sse.rs#L121) | receiver-type-required |
| `parse_frame` | `line.strip_suffix('\r').unwrap_or` | [122](../../src/sse.rs#L122) | receiver-type-required |
| `parse_frame` | `line.strip_suffix` | [122](../../src/sse.rs#L122) | receiver-type-required |
| `parse_frame` | `line.starts_with` | [123](../../src/sse.rs#L123) | receiver-type-required |
| `parse_frame` | `line.strip_prefix` | [126](../../src/sse.rs#L126), [128](../../src/sse.rs#L128) | receiver-type-required |
| `parse_frame` | `Some` | [127](../../src/sse.rs#L127), [135](../../src/sse.rs#L135) | external-constructor-callback-or-unresolved |
| `parse_frame` | `value.strip_prefix(' ').unwrap_or(value).to_owned` | [127](../../src/sse.rs#L127) | receiver-type-required |
| `parse_frame` | `value.strip_prefix(' ').unwrap_or` | [127](../../src/sse.rs#L127), [129](../../src/sse.rs#L129) | receiver-type-required |
| `parse_frame` | `value.strip_prefix` | [127](../../src/sse.rs#L127), [129](../../src/sse.rs#L129) | receiver-type-required |
| `parse_frame` | `data.push` | [129](../../src/sse.rs#L129) | receiver-type-required |
| `parse_frame` | `data.is_empty` | [132](../../src/sse.rs#L132) | receiver-type-required |
| `parse_frame` | `Ok` | [133](../../src/sse.rs#L133), [135](../../src/sse.rs#L135) | external-constructor-callback-or-unresolved |
| `parse_frame` | `data.join` | [137](../../src/sse.rs#L137) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `"data: hé\ndata: two\n\n".as_bytes` | [147](../../src/sse.rs#L147) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `bytes.len` | [148](../../src/sse.rs#L148) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `SseDecoder::new` | [149](../../src/sse.rs#L149) | external-constructor-callback-or-unresolved |
| `split_utf8_and_multiline_data_are_stable` | `decoder.push(&bytes[..split]).expect` | [150](../../src/sse.rs#L150) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `decoder.push` | [150](../../src/sse.rs#L150), [151](../../src/sse.rs#L151) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `events.extend` | [151](../../src/sse.rs#L151) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `decoder.push(&bytes[split..]).expect` | [151](../../src/sse.rs#L151) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `decoder.finish().expect` | [153](../../src/sse.rs#L153) | receiver-type-required |
| `split_utf8_and_multiline_data_are_stable` | `decoder.finish` | [153](../../src/sse.rs#L153) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `b"data: ".to_vec` | [159](../../src/sse.rs#L159) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `frame.resize` | [160](../../src/sse.rs#L160) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `frame.extend_from_slice` | [161](../../src/sse.rs#L161) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `frame.len` | [162](../../src/sse.rs#L162), [163](../../src/sse.rs#L163) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `Vec::with_capacity` | [163](../../src/sse.rs#L163) | external-constructor-callback-or-unresolved |
| `one_large_transport_chunk_may_contain_many_small_frames` | `chunk.extend_from_slice` | [165](../../src/sse.rs#L165) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `SseDecoder::new` | [168](../../src/sse.rs#L168) | external-constructor-callback-or-unresolved |
| `one_large_transport_chunk_may_contain_many_small_frames` | `decoder.push(&chunk).expect` | [169](../../src/sse.rs#L169) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `decoder.push` | [169](../../src/sse.rs#L169) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `decoder.finish().expect` | [172](../../src/sse.rs#L172) | receiver-type-required |
| `one_large_transport_chunk_may_contain_many_small_frames` | `decoder.finish` | [172](../../src/sse.rs#L172) | receiver-type-required |
| `a_single_oversized_frame_is_rejected_even_with_a_delimiter` | `b"data: ".to_vec` | [177](../../src/sse.rs#L177) | receiver-type-required |
| `a_single_oversized_frame_is_rejected_even_with_a_delimiter` | `chunk.resize` | [178](../../src/sse.rs#L178) | receiver-type-required |
| `a_single_oversized_frame_is_rejected_even_with_a_delimiter` | `chunk.extend_from_slice` | [179](../../src/sse.rs#L179) | receiver-type-required |
| `a_single_oversized_frame_is_rejected_even_with_a_delimiter` | `SseDecoder::new` | [181](../../src/sse.rs#L181) | external-constructor-callback-or-unresolved |
| `the_first_mixed_style_delimiter_wins` | `SseDecoder::new` | [187](../../src/sse.rs#L187) | external-constructor-callback-or-unresolved |
| `the_first_mixed_style_delimiter_wins` | `decoder             .push(b"data: first\n\ndata: second\r\n\r\n")             .expect` | [188](../../src/sse.rs#L188) | receiver-type-required |
| `the_first_mixed_style_delimiter_wins` | `decoder             .push` | [188](../../src/sse.rs#L188) | receiver-type-required |
| `data_after_terminal_in_the_same_chunk_is_rejected` | `SseDecoder::new` | [208](../../src/sse.rs#L208) | external-constructor-callback-or-unresolved |
