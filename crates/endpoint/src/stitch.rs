use std::collections::BTreeMap;

use thiserror::Error;

use crate::SessionEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum StitchDecision {
    Installed(Vec<SessionEvent>),
    RefetchTail,
}

pub fn stitch_window(
    history: Vec<SessionEvent>,
    buffered: Vec<SessionEvent>,
    subscribed_last_seq: i64,
) -> Result<StitchDecision, StitchError> {
    if subscribed_last_seq < -1 {
        return Err(StitchError::Baseline(subscribed_last_seq));
    }
    validate_contiguous(&history)?;
    for event in history.iter().chain(&buffered) {
        event.validate().map_err(StitchError::Type)?;
    }
    let history_tail = history.last().map(|event| event.seq);
    if subscribed_last_seq >= 0 && history_tail.is_none_or(|tail| subscribed_last_seq as u64 > tail)
    {
        return Ok(StitchDecision::RefetchTail);
    }
    let mut merged = history
        .into_iter()
        .map(|event| (event.seq, event))
        .collect::<BTreeMap<_, _>>();
    for event in buffered {
        if let Some(existing) = merged.get(&event.seq) {
            if existing.canonical_bytes().map_err(StitchError::Type)?
                != event.canonical_bytes().map_err(StitchError::Type)?
            {
                return Err(StitchError::OverlapMismatch(event.seq));
            }
        } else {
            merged.insert(event.seq, event);
        }
    }
    let events = merged.into_values().collect::<Vec<_>>();
    if validate_contiguous(&events).is_err() {
        return Ok(StitchDecision::RefetchTail);
    }
    if history_tail.is_none() && events.first().is_some_and(|event| event.seq != 0) {
        return Ok(StitchDecision::RefetchTail);
    }
    Ok(StitchDecision::Installed(events))
}

fn validate_contiguous(events: &[SessionEvent]) -> Result<(), StitchError> {
    for pair in events.windows(2) {
        if pair[1].seq != pair[0].seq + 1 {
            return Err(StitchError::Gap {
                expected: pair[0].seq + 1,
                actual: pair[1].seq,
            });
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum StitchError {
    #[error("sequence gap: expected {expected}, found {actual}")]
    Gap { expected: u64, actual: u64 },
    #[error("overlap at seq {0} changed bytes")]
    OverlapMismatch(u64),
    #[error("invalid subscribed baseline {0}")]
    Baseline(i64),
    #[error("event validation failed: {0}")]
    Type(crate::types::EndpointTypeError),
}
