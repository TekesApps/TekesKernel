use thiserror::Error;

const MAX_FRAME: usize = 8 * 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SseEvent {
    pub event: Option<String>,
    pub data: String,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SseError {
    #[error("SSE frame exceeds 8 MiB")]
    FrameTooLarge,
    #[error("SSE stream contains invalid UTF-8")]
    InvalidUtf8,
    #[error("SSE bytes follow the terminal event")]
    DataAfterTerminal,
    #[error("SSE stream ended mid-frame")]
    Truncated,
}

#[derive(Default)]
pub struct SseDecoder {
    bytes: Vec<u8>,
    terminal: bool,
}

impl SseDecoder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseEvent>, SseError> {
        if self.terminal && !chunk.is_empty() {
            return Err(SseError::DataAfterTerminal);
        }
        self.bytes.extend_from_slice(chunk);
        let mut events = Vec::new();
        let mut consumed = 0;
        while let Some((end, delimiter)) = frame_end(&self.bytes[consumed..]) {
            if end > MAX_FRAME {
                return Err(SseError::FrameTooLarge);
            }
            let frame_start = consumed;
            let frame_end = frame_start + end;
            consumed = frame_end + delimiter;
            if self.terminal {
                return Err(SseError::DataAfterTerminal);
            }
            if let Some(event) = parse_frame(&self.bytes[frame_start..frame_end])? {
                if event.data == "[DONE]" || event.event.as_deref() == Some("done") {
                    self.terminal = true;
                }
                events.push(event);
            }
        }
        if self.terminal && consumed < self.bytes.len() {
            return Err(SseError::DataAfterTerminal);
        }
        if consumed != 0 {
            self.bytes.drain(..consumed);
        }
        // The remaining bytes are the only incomplete frame. A transport
        // chunk may contain arbitrarily many complete, individually bounded
        // frames; the v1 limit applies to one frame, not to that chunk.
        if self.bytes.len() > MAX_FRAME {
            return Err(SseError::FrameTooLarge);
        }
        Ok(events)
    }

    pub fn finish(self) -> Result<(), SseError> {
        if self.bytes.iter().all(|byte| matches!(byte, b'\r' | b'\n')) {
            Ok(())
        } else {
            Err(SseError::Truncated)
        }
    }

    pub fn finish_events(mut self) -> Result<Vec<SseEvent>, SseError> {
        if self.bytes.is_empty() || self.bytes.iter().all(|byte| matches!(byte, b'\r' | b'\n')) {
            return Ok(Vec::new());
        }
        if self.terminal {
            return Err(SseError::DataAfterTerminal);
        }
        while self
            .bytes
            .last()
            .is_some_and(|byte| matches!(byte, b'\r' | b'\n'))
        {
            self.bytes.pop();
        }
        if self.bytes.len() > MAX_FRAME {
            return Err(SseError::FrameTooLarge);
        }
        Ok(parse_frame(&self.bytes)?.into_iter().collect())
    }
}

fn frame_end(bytes: &[u8]) -> Option<(usize, usize)> {
    let mut index = 0;
    while index + 1 < bytes.len() {
        if bytes[index] == b'\n' && bytes[index + 1] == b'\n' {
            return Some((index, 2));
        }
        if index + 3 < bytes.len() && &bytes[index..index + 4] == b"\r\n\r\n" {
            return Some((index, 4));
        }
        index += 1;
    }
    None
}

fn parse_frame(bytes: &[u8]) -> Result<Option<SseEvent>, SseError> {
    let text = std::str::from_utf8(bytes).map_err(|_| SseError::InvalidUtf8)?;
    let mut event = None;
    let mut data = Vec::new();
    for line in text.lines() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.starts_with(':') {
            continue;
        }
        if let Some(value) = line.strip_prefix("event:") {
            event = Some(value.strip_prefix(' ').unwrap_or(value).to_owned());
        } else if let Some(value) = line.strip_prefix("data:") {
            data.push(value.strip_prefix(' ').unwrap_or(value));
        }
    }
    if data.is_empty() {
        return Ok(None);
    }
    Ok(Some(SseEvent {
        event,
        data: data.join("\n"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_utf8_and_multiline_data_are_stable() {
        let bytes = "data: hé\ndata: two\n\n".as_bytes();
        for split in 0..=bytes.len() {
            let mut decoder = SseDecoder::new();
            let mut events = decoder.push(&bytes[..split]).expect("first");
            events.extend(decoder.push(&bytes[split..]).expect("second"));
            assert_eq!(events[0].data, "hé\ntwo");
            decoder.finish().expect("complete");
        }
    }

    #[test]
    fn one_large_transport_chunk_may_contain_many_small_frames() {
        let mut frame = b"data: ".to_vec();
        frame.resize(8 * 1024, b'x');
        frame.extend_from_slice(b"\n\n");
        let count = MAX_FRAME / frame.len() + 1;
        let mut chunk = Vec::with_capacity(count * frame.len());
        for _ in 0..count {
            chunk.extend_from_slice(&frame);
        }

        let mut decoder = SseDecoder::new();
        let events = decoder.push(&chunk).expect("individually bounded frames");
        assert_eq!(events.len(), count);
        assert!(events.iter().all(|event| event.data.len() == 8 * 1024 - 6));
        decoder.finish().expect("complete stream");
    }

    #[test]
    fn a_single_oversized_frame_is_rejected_even_with_a_delimiter() {
        let mut chunk = b"data: ".to_vec();
        chunk.resize(MAX_FRAME + 1, b'x');
        chunk.extend_from_slice(b"\n\n");

        let mut decoder = SseDecoder::new();
        assert_eq!(decoder.push(&chunk), Err(SseError::FrameTooLarge));
    }

    #[test]
    fn the_first_mixed_style_delimiter_wins() {
        let mut decoder = SseDecoder::new();
        let events = decoder
            .push(b"data: first\n\ndata: second\r\n\r\n")
            .expect("mixed delimiters");
        assert_eq!(
            events,
            vec![
                SseEvent {
                    event: None,
                    data: "first".to_owned(),
                },
                SseEvent {
                    event: None,
                    data: "second".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn data_after_terminal_in_the_same_chunk_is_rejected() {
        let mut decoder = SseDecoder::new();
        assert_eq!(
            decoder.push(b"data: [DONE]\n\ndata: late\n\n"),
            Err(SseError::DataAfterTerminal)
        );
    }
}
