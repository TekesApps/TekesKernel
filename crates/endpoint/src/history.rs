use std::collections::BTreeSet;

use thiserror::Error;

use crate::{JournalRecord, SessionHistoryEntry};

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryPage {
    pub events: Vec<SessionHistoryEntry>,
    pub has_more: bool,
}

pub fn history_page(
    records: &[JournalRecord],
    before_seq: Option<u64>,
    max_messages: Option<usize>,
) -> Result<HistoryPage, HistoryError> {
    let max_messages = max_messages.unwrap_or(50);
    if max_messages == 0 {
        return Err(HistoryError::InvalidMaxMessages);
    }
    let upper = before_seq.unwrap_or(records.len() as u64);
    let mut eligible_end = records.partition_point(|record| record.event.seq < upper);
    if eligible_end < records.len() && !records[eligible_end].kernel_seqs.is_empty() {
        let boundary = records[eligible_end]
            .kernel_seqs
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if let Some(group_start) = records[..eligible_end]
            .iter()
            .position(|record| record.kernel_seqs.iter().any(|seq| boundary.contains(seq)))
        {
            eligible_end = group_start;
        }
    }
    if eligible_end == 0 {
        return Ok(HistoryPage {
            events: Vec::new(),
            has_more: false,
        });
    }

    let mut lower = eligible_end;
    let mut messages = 0;
    while lower > 0 {
        lower -= 1;
        let event = &records[lower].event;
        if is_append_origin_message(event) {
            messages += 1;
            if messages == max_messages {
                break;
            }
        }
    }
    if messages < max_messages {
        lower = 0;
    }

    loop {
        let previous = lower;
        let mut group = records[lower..eligible_end]
            .iter()
            .flat_map(|record| record.kernel_seqs.iter().copied())
            .collect::<BTreeSet<_>>();
        loop {
            let Some(group_start) = records[..lower]
                .iter()
                .position(|record| record.kernel_seqs.iter().any(|seq| group.contains(seq)))
            else {
                break;
            };
            lower = group_start;
            group.extend(
                records[lower..eligible_end]
                    .iter()
                    .flat_map(|record| record.kernel_seqs.iter().copied()),
            );
        }
        let mut needed = BTreeSet::new();
        for record in &records[lower..eligible_end] {
            for source in record
                .event
                .source_event_seqs
                .as_deref()
                .unwrap_or_default()
            {
                if *source < records[lower].event.seq {
                    needed.insert(*source);
                }
            }
        }
        if let Some(first) = needed.first() {
            let first =
                usize::try_from(*first).map_err(|_| HistoryError::SourceSequence(*first))?;
            if first >= records.len() {
                return Err(HistoryError::SourceSequence(
                    *needed.first().expect("present"),
                ));
            }
            lower = lower.min(first);
        }
        if lower == previous {
            break;
        }
    }

    Ok(HistoryPage {
        events: records[lower..eligible_end]
            .iter()
            .map(|record| SessionHistoryEntry {
                event: record.event.clone(),
                view: None,
            })
            .collect(),
        has_more: lower > 0,
    })
}

fn is_append_origin_message(event: &crate::SessionEvent) -> bool {
    matches!(
        event.event_type.as_str(),
        "user/message" | "assistant/message"
    ) && matches!(
        event.surface_op,
        Some(crate::SurfaceOperation::Append(ref value)) if value == "append"
    )
}

#[derive(Debug, Error)]
pub enum HistoryError {
    #[error("maxMessages must be positive")]
    InvalidMaxMessages,
    #[error("sourceEventSeqs contains invalid endpoint seq {0}")]
    SourceSequence(u64),
}

#[cfg(test)]
mod tests {
    use schema::IJsonValue;

    use super::*;
    use crate::{SessionEvent, SurfaceOperation};

    fn record(seq: u64, event_type: &str, kernel: u64) -> JournalRecord {
        JournalRecord {
            v: 2,
            event: SessionEvent {
                event_type: event_type.to_owned(),
                seq,
                time: seq as f64,
                data: IJsonValue::parse_str("{}").expect("data"),
                ignorable: None,
                source_event_seqs: None,
                surface_op: matches!(event_type, "user/message" | "assistant/message")
                    .then(|| SurfaceOperation::Append("append".to_owned())),
            },
            kernel_seqs: vec![kernel],
            slot: format!("slot-{seq}"),
        }
    }

    #[test]
    fn page_is_message_aligned_and_keeps_projection_group() {
        let records = vec![
            record(0, "turn/start", 2),
            record(1, "user/message", 2),
            record(2, "step/start", 3),
            record(3, "assistant/chunk", 4),
            record(4, "assistant/message", 5),
            record(5, "step/end", 6),
        ];
        let page = history_page(&records, None, Some(1)).expect("page");
        assert_eq!(page.events.first().expect("first").event.seq, 4);
        assert!(page.has_more);
        let older = history_page(&records, Some(4), Some(1)).expect("older");
        assert_eq!(older.events.first().expect("first").event.seq, 0);
        assert_eq!(older.events.last().expect("last").event.seq, 3);
    }
}
