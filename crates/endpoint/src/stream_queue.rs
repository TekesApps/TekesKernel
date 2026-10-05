use std::collections::VecDeque;
use std::future::poll_fn;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};

use thiserror::Error;

use crate::{
    CarrierHostFuture, EndpointStream, EndpointStreamReceiver, ServerRequest, StreamFailure,
};

/// Runtime-neutral bounded queue for host-stream frames and terminal errors.
/// The producer owns observation and drain ordering; the physical carrier only
/// awaits the receiver and applies socket backpressure.
#[derive(Clone)]
pub struct EndpointFrameQueue {
    state: Arc<Mutex<FrameQueueState>>,
}

struct FrameQueueState {
    frames: VecDeque<(Result<ServerRequest, StreamFailure>, usize)>,
    bytes: usize,
    max_frames: usize,
    max_bytes: usize,
    closed: bool,
    waker: Option<Waker>,
}

impl EndpointFrameQueue {
    pub fn new(max_frames: usize, max_bytes: usize) -> Result<Self, FrameQueueError> {
        if max_frames == 0 || max_bytes == 0 {
            return Err(FrameQueueError::InvalidBounds);
        }
        Ok(Self {
            state: Arc::new(Mutex::new(FrameQueueState {
                frames: VecDeque::new(),
                bytes: 0,
                max_frames,
                max_bytes,
                closed: false,
                waker: None,
            })),
        })
    }

    pub fn push(&self, frame: ServerRequest) -> Result<(), FrameQueueError> {
        let size = frame
            .canonical_bytes()
            .map_err(|error| FrameQueueError::Canonical(error.to_string()))?
            .len();
        self.push_item(Ok(frame), size)
    }

    pub fn fail(&self, error: StreamFailure) -> Result<(), FrameQueueError> {
        self.push_item(Err(error), 0)
    }

    fn push_item(
        &self,
        item: Result<ServerRequest, StreamFailure>,
        size: usize,
    ) -> Result<(), FrameQueueError> {
        let mut state = self.state.lock().map_err(|_| FrameQueueError::Poisoned)?;
        if state.closed {
            return Err(FrameQueueError::Closed);
        }
        if state.frames.len() >= state.max_frames
            || state
                .bytes
                .checked_add(size)
                .is_none_or(|bytes| bytes > state.max_bytes)
        {
            // Preserve the complete queued prefix, then surface one typed
            // terminal failure. The receiver must not mistake host-stream
            // backpressure for a normal detach/close.
            state
                .frames
                .push_back((Err(StreamFailure::new(crate::StreamErrorCode::LiveGap)), 0));
            state.closed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
            return Err(FrameQueueError::Backpressure);
        }
        state.bytes += size;
        state.frames.push_back((item, size));
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
        Ok(())
    }

    pub fn close(&self) -> Result<(), FrameQueueError> {
        let mut state = self.state.lock().map_err(|_| FrameQueueError::Poisoned)?;
        state.closed = true;
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
        Ok(())
    }

    #[must_use]
    pub fn receiver(&self) -> EndpointStreamReceiver {
        Box::new(FrameQueueReceiver {
            state: Arc::clone(&self.state),
        })
    }
}

struct FrameQueueReceiver {
    state: Arc<Mutex<FrameQueueState>>,
}

impl EndpointStream for FrameQueueReceiver {
    fn recv(&mut self) -> CarrierHostFuture<'_, Option<Result<ServerRequest, StreamFailure>>> {
        let state = Arc::clone(&self.state);
        Box::pin(poll_fn(move |context| {
            let mut state = match state.lock() {
                Ok(state) => state,
                Err(_) => {
                    return Poll::Ready(Some(Err(StreamFailure::internal(
                        "endpoint frame queue mutex was poisoned".to_owned(),
                    ))));
                }
            };
            if let Some((frame, size)) = state.frames.pop_front() {
                state.bytes -= size;
                return Poll::Ready(Some(frame));
            }
            if state.closed {
                return Poll::Ready(None);
            }
            if state
                .waker
                .as_ref()
                .is_none_or(|waker| !waker.will_wake(context.waker()))
            {
                state.waker = Some(context.waker().clone());
            }
            Poll::Pending
        }))
    }
}

#[derive(Debug, Error)]
pub enum FrameQueueError {
    #[error("endpoint frame queue bounds must be positive")]
    InvalidBounds,
    #[error("endpoint frame queue is closed")]
    Closed,
    #[error("endpoint frame queue exceeded its bounded capacity")]
    Backpressure,
    #[error("endpoint frame queue mutex was poisoned")]
    Poisoned,
    #[error("endpoint frame canonicalization failed: {0}")]
    Canonical(String),
}
