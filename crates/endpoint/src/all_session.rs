use std::collections::BTreeMap;
use std::future::poll_fn;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};

use thiserror::Error;

use crate::{
    CarrierHostFuture, EndpointJournal, EndpointStream, EndpointStreamReceiver,
    EndpointSubscription, EndpointSubscriptionHub, HubError, MuxRegistration,
    MuxReplayRegistration, ServerRequest, StreamErrorCode, StreamFailure, SubscriptionBaseline,
    SubscriptionPoll,
};

/// One all-session mux generation. Construction is all-or-nothing; archive
/// detaches a session from this generation, and explicit unarchive attaches a
/// fresh subscription whose baseline is read from the unchanged durable
/// endpoint journal.
pub struct AllSessionMux {
    subscriptions: BTreeMap<String, EndpointSubscription>,
    retired: Vec<EndpointSubscription>,
    stream_waker: Option<Waker>,
    closed: bool,
}

impl AllSessionMux {
    pub fn open(
        hub: &EndpointSubscriptionHub,
        registrations: &[MuxRegistration<'_>],
    ) -> Result<Self, AllSessionMuxError> {
        Ok(Self {
            subscriptions: hub.subscribe_all(registrations)?,
            retired: Vec::new(),
            stream_waker: None,
            closed: false,
        })
    }

    pub fn open_with_replay(
        hub: &EndpointSubscriptionHub,
        registrations: &[MuxReplayRegistration<'_>],
    ) -> Result<Self, AllSessionMuxError> {
        Ok(Self {
            subscriptions: hub.subscribe_all_with_replay(registrations)?,
            retired: Vec::new(),
            stream_waker: None,
            closed: false,
        })
    }

    pub fn attach(
        &mut self,
        hub: &EndpointSubscriptionHub,
        session_id: &str,
        journal: &EndpointJournal,
        subscribed_rpc_id: &str,
    ) -> Result<SubscriptionBaseline, AllSessionMuxError> {
        if self.subscriptions.contains_key(session_id) {
            return Err(AllSessionMuxError::AlreadyAttached(session_id.to_owned()));
        }
        if self.closed {
            return Err(AllSessionMuxError::GenerationClosed);
        }
        let mut registered = hub.subscribe_all(&[MuxRegistration {
            session_id,
            journal,
            subscribed_rpc_id,
        }])?;
        let subscription = registered
            .remove(session_id)
            .ok_or_else(|| AllSessionMuxError::MissingRegistration(session_id.to_owned()))?;
        let baseline = subscription.baseline().clone();
        self.subscriptions
            .insert(session_id.to_owned(), subscription);
        // A receiver may already be pending on the previous membership set.
        // Wake that shared task so it observes the new subscription's queued
        // baseline instead of waiting for an unrelated old-session event.
        for subscription in self.subscriptions.values() {
            subscription.wake()?;
        }
        self.wake_stream();
        Ok(baseline)
    }

    pub fn attach_with_replay(
        &mut self,
        hub: &EndpointSubscriptionHub,
        session_id: &str,
        journal: &EndpointJournal,
        subscribed_rpc_id: &str,
        unresolved: &[ServerRequest],
    ) -> Result<SubscriptionBaseline, AllSessionMuxError> {
        if self.subscriptions.contains_key(session_id) {
            return Err(AllSessionMuxError::AlreadyAttached(session_id.to_owned()));
        }
        if self.closed {
            return Err(AllSessionMuxError::GenerationClosed);
        }
        let mut registered = hub.subscribe_all_with_replay(&[MuxReplayRegistration {
            registration: MuxRegistration {
                session_id,
                journal,
                subscribed_rpc_id,
            },
            unresolved,
        }])?;
        let subscription = registered
            .remove(session_id)
            .ok_or_else(|| AllSessionMuxError::MissingRegistration(session_id.to_owned()))?;
        let baseline = subscription.baseline().clone();
        self.subscriptions
            .insert(session_id.to_owned(), subscription);
        for subscription in self.subscriptions.values() {
            subscription.wake()?;
        }
        self.wake_stream();
        Ok(baseline)
    }

    pub fn detach_for_archive(&mut self, session_id: &str) -> Result<bool, AllSessionMuxError> {
        let Some(subscription) = self.subscriptions.remove(session_id) else {
            return Ok(false);
        };
        subscription.close()?;
        // Keep the closed subscription reachable until its already-buffered
        // prefix drains. A later fresh attachment is active immediately, but
        // the stream polls retired prefixes first so archive ordering cannot
        // discard or overtake durable pre-archive frames.
        self.retired.push(subscription);
        self.wake_stream();
        Ok(true)
    }

    pub fn poll(&self, session_id: &str) -> Result<SubscriptionPoll, AllSessionMuxError> {
        self.subscriptions
            .get(session_id)
            .ok_or_else(|| AllSessionMuxError::NotAttached(session_id.to_owned()))?
            .poll()
            .map_err(Into::into)
    }

    #[must_use]
    pub fn baselines(&self) -> Vec<SubscriptionBaseline> {
        self.subscriptions
            .values()
            .map(|subscription| subscription.baseline().clone())
            .collect()
    }

    #[must_use]
    pub fn contains(&self, session_id: &str) -> bool {
        self.subscriptions.contains_key(session_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.subscriptions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.subscriptions.is_empty()
    }

    /// Converts the registered generation into the runtime-neutral stream
    /// seam consumed by a physical WebSocket carrier. The returned handle is
    /// retained by archive/unarchive coordination so membership changes and
    /// socket reads share one generation.
    #[must_use]
    pub fn into_stream(self) -> (AllSessionMuxHandle, EndpointStreamReceiver) {
        let mux = Arc::new(Mutex::new(self));
        (
            AllSessionMuxHandle(Arc::clone(&mux)),
            Box::new(AllSessionMuxStream { mux, cursor: 0 }),
        )
    }

    fn wake_stream(&mut self) {
        if let Some(waker) = self.stream_waker.take() {
            waker.wake();
        }
    }
}

#[derive(Clone)]
pub struct AllSessionMuxHandle(Arc<Mutex<AllSessionMux>>);

impl AllSessionMuxHandle {
    pub fn contains(&self, session_id: &str) -> Result<bool, AllSessionMuxError> {
        Ok(self
            .0
            .lock()
            .map_err(|_| AllSessionMuxError::Poisoned)?
            .contains(session_id))
    }

    pub fn attach(
        &self,
        hub: &EndpointSubscriptionHub,
        session_id: &str,
        journal: &EndpointJournal,
        subscribed_rpc_id: &str,
    ) -> Result<SubscriptionBaseline, AllSessionMuxError> {
        self.0
            .lock()
            .map_err(|_| AllSessionMuxError::Poisoned)?
            .attach(hub, session_id, journal, subscribed_rpc_id)
    }

    pub fn attach_with_replay(
        &self,
        hub: &EndpointSubscriptionHub,
        session_id: &str,
        journal: &EndpointJournal,
        subscribed_rpc_id: &str,
        unresolved: &[ServerRequest],
    ) -> Result<SubscriptionBaseline, AllSessionMuxError> {
        self.0
            .lock()
            .map_err(|_| AllSessionMuxError::Poisoned)?
            .attach_with_replay(hub, session_id, journal, subscribed_rpc_id, unresolved)
    }

    pub fn detach_for_archive(&self, session_id: &str) -> Result<bool, AllSessionMuxError> {
        self.0
            .lock()
            .map_err(|_| AllSessionMuxError::Poisoned)?
            .detach_for_archive(session_id)
    }

    pub fn baselines(&self) -> Result<Vec<SubscriptionBaseline>, AllSessionMuxError> {
        Ok(self
            .0
            .lock()
            .map_err(|_| AllSessionMuxError::Poisoned)?
            .baselines())
    }

    pub fn close(&self) -> Result<(), AllSessionMuxError> {
        let mut mux = self.0.lock().map_err(|_| AllSessionMuxError::Poisoned)?;
        mux.closed = true;
        for subscription in mux.subscriptions.values() {
            subscription.close()?;
        }
        for subscription in &mux.retired {
            subscription.close()?;
        }
        mux.wake_stream();
        Ok(())
    }
}

struct AllSessionMuxStream {
    mux: Arc<Mutex<AllSessionMux>>,
    cursor: usize,
}

impl EndpointStream for AllSessionMuxStream {
    fn recv(
        &mut self,
    ) -> CarrierHostFuture<'_, Option<Result<crate::ServerRequest, StreamFailure>>> {
        Box::pin(poll_fn(move |context| {
            let mut mux = match self.mux.lock() {
                Ok(mux) => mux,
                Err(_) => {
                    return Poll::Ready(Some(Err(StreamFailure::internal(
                        "all-session mux mutex was poisoned".to_owned(),
                    ))));
                }
            };
            while let Some(subscription) = mux.retired.first() {
                match subscription.poll_with_context(context) {
                    Ok(Poll::Ready(SubscriptionPoll::Frame(frame))) => {
                        return Poll::Ready(Some(Ok(frame)));
                    }
                    Ok(Poll::Ready(SubscriptionPoll::LiveGap)) => {
                        return Poll::Ready(Some(Err(StreamFailure::new(
                            StreamErrorCode::LiveGap,
                        ))));
                    }
                    Ok(Poll::Ready(SubscriptionPoll::Closed)) => {
                        mux.retired.remove(0);
                    }
                    Ok(Poll::Ready(SubscriptionPoll::Pending) | Poll::Pending) => {
                        return Poll::Pending;
                    }
                    Err(error) => {
                        return Poll::Ready(Some(Err(StreamFailure::internal(error.to_string()))));
                    }
                }
            }
            if mux.subscriptions.is_empty() {
                if mux.closed {
                    return Poll::Ready(None);
                }
                if mux
                    .stream_waker
                    .as_ref()
                    .is_none_or(|waker| !waker.will_wake(context.waker()))
                {
                    mux.stream_waker = Some(context.waker().clone());
                }
                return Poll::Pending;
            }

            let count = mux.subscriptions.len();
            let mut closed = 0;
            for offset in 0..count {
                let index = (self.cursor + offset) % count;
                let Some(subscription) = mux.subscriptions.values().nth(index) else {
                    continue;
                };
                match subscription.poll_with_context(context) {
                    Ok(Poll::Ready(SubscriptionPoll::Frame(frame))) => {
                        self.cursor = (index + 1) % count;
                        return Poll::Ready(Some(Ok(frame)));
                    }
                    Ok(Poll::Ready(SubscriptionPoll::LiveGap)) => {
                        self.cursor = (index + 1) % count;
                        return Poll::Ready(Some(Err(StreamFailure::new(
                            StreamErrorCode::LiveGap,
                        ))));
                    }
                    Ok(Poll::Ready(SubscriptionPoll::Closed)) => closed += 1,
                    Ok(Poll::Ready(SubscriptionPoll::Pending) | Poll::Pending) => {}
                    Err(error) => {
                        return Poll::Ready(Some(Err(StreamFailure::internal(error.to_string()))));
                    }
                }
            }
            if closed == count {
                Poll::Ready(None)
            } else {
                if mux
                    .stream_waker
                    .as_ref()
                    .is_none_or(|waker| !waker.will_wake(context.waker()))
                {
                    mux.stream_waker = Some(context.waker().clone());
                }
                Poll::Pending
            }
        }))
    }
}

#[derive(Debug, Error)]
pub enum AllSessionMuxError {
    #[error("endpoint subscription failed: {0}")]
    Hub(#[from] HubError),
    #[error("session {0} is already attached")]
    AlreadyAttached(String),
    #[error("session {0} was not returned by atomic registration")]
    MissingRegistration(String),
    #[error("session {0} is not attached")]
    NotAttached(String),
    #[error("all-session mux mutex was poisoned")]
    Poisoned,
    #[error("all-session mux generation is closed")]
    GenerationClosed,
}
