use super::*;

const CANCEL_NONE: u8 = 0;
pub(crate) const CANCEL_STOP: u8 = 1;
pub(crate) const CANCEL_SUPERVISOR_LOSS: u8 = 2;

#[derive(Clone, Default)]
pub(crate) struct RuntimeCancellation {
    pub(crate) tools: CancellationToken,
    pub(crate) provider: Arc<AtomicBool>,
    cause: Arc<AtomicU8>,
    protocol_failed: Arc<AtomicBool>,
    deferred: Arc<Mutex<VecDeque<String>>>,
    /// Deliveries (`input`, `queue_edit`, `meta`, `approval_response`,
    /// `queue_transaction`) the reader thread took off the pipe. They never
    /// enter the control channel, so a wait for `lease`/`tool_control_result`/
    /// `launch_result` cannot mistake them for a protocol error; the main
    /// thread appends and receipts them at its next yield point.
    parked_deliveries: Arc<Mutex<VecDeque<String>>>,
}

impl RuntimeCancellation {
    pub(crate) fn cancel(&self, cause: u8) {
        let _ =
            self.cause
                .compare_exchange(CANCEL_NONE, cause, Ordering::AcqRel, Ordering::Acquire);
        self.tools.cancel();
        self.provider.store(true, Ordering::Release);
    }

    pub(crate) fn stop_requested(&self) -> bool {
        self.cause.load(Ordering::Acquire) == CANCEL_STOP
    }

    pub(crate) fn supervisor_lost(&self) -> bool {
        self.cause.load(Ordering::Acquire) == CANCEL_SUPERVISOR_LOSS
    }

    pub(crate) fn mark_protocol_failed(&self) {
        self.protocol_failed.store(true, Ordering::Release);
    }

    pub(crate) fn protocol_failed(&self) -> bool {
        self.protocol_failed.load(Ordering::Acquire)
    }

    pub(crate) fn defer(&self, line: String) {
        self.deferred
            .lock()
            .expect("control defer lock")
            .push_back(line);
    }

    pub(crate) fn park_delivery(&self, line: String) {
        self.parked_deliveries
            .lock()
            .expect("control park lock")
            .push_back(line);
    }

    pub(crate) fn take_parked_deliveries(&self) -> Vec<String> {
        std::mem::take(&mut *self.parked_deliveries.lock().expect("control park lock")).into()
    }
}

pub(crate) struct ControlLines {
    pub(crate) receiver: Receiver<io::Result<String>>,
    pub(crate) cancellation: RuntimeCancellation,
}

impl ControlLines {
    /// Every control line that has already reached this process: deferred
    /// lines first, then whatever the reader thread has buffered. Never waits
    /// for stdin; a worker whose turn is over has nothing to wait for.
    pub(crate) fn drain_ready(&mut self) -> Vec<String> {
        let mut ready = self.cancellation.take_parked_deliveries();
        ready.extend(std::mem::take(
            &mut *self
                .cancellation
                .deferred
                .lock()
                .expect("control defer lock"),
        ));
        while let Ok(Ok(line)) = self.receiver.try_recv() {
            ready.push(line);
        }
        ready
    }
}

impl Iterator for ControlLines {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self
            .cancellation
            .deferred
            .lock()
            .expect("control defer lock")
            .pop_front()
        {
            return Some(Ok(line));
        }
        self.receiver.recv().ok()
    }
}

#[derive(Debug)]
pub(crate) struct ProtocolFailure(pub(crate) String);

impl std::fmt::Display for ProtocolFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ProtocolFailure {}

pub(crate) fn spawn_control_reader(cancellation: RuntimeCancellation) -> ControlLines {
    // One buffered frame lets the sole reader observe a following stop/EOF
    // while a backend is blocked. The receiver remains FIFO and stdout is
    // still written only by the main thread, so protocol replies cannot race.
    let (sender, receiver) = sync_channel(1);
    let reader_cancellation = cancellation.clone();
    std::thread::spawn(move || read_control_lines(sender, reader_cancellation));
    ControlLines {
        receiver,
        cancellation,
    }
}

fn read_control_lines(sender: SyncSender<io::Result<String>>, cancellation: RuntimeCancellation) {
    let stdin = io::stdin();
    forward_control_lines(stdin.lock().lines(), sender, cancellation);
}

pub(crate) fn forward_control_lines(
    lines: impl Iterator<Item = io::Result<String>>,
    sender: SyncSender<io::Result<String>>,
    cancellation: RuntimeCancellation,
) {
    for line in lines {
        match &line {
            Ok(line) if is_stop_control_line(line) => cancellation.cancel(CANCEL_STOP),
            Ok(line) if is_delivery_control_line(line) => {
                // Read continuously in every phase (R2-8): a delivery must not sit in
                // the channel behind a blocked backend, and must never reach a wait
                // that expects only `lease`/`tool_control_result`/`launch_result`.
                cancellation.park_delivery(line.clone());
                continue;
            }
            Err(_) => cancellation.cancel(CANCEL_SUPERVISOR_LOSS),
            _ => {}
        }
        if sender.send(line).is_err() {
            return;
        }
    }
    cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
}

pub(crate) fn control_line_key(line: &str) -> Option<String> {
    let value = serde_json::from_str::<Value>(line).ok()?;
    let object = value.as_object()?;
    (object.len() == 1)
        .then(|| object.keys().next().cloned())
        .flatten()
}

fn is_stop_control_line(line: &str) -> bool {
    control_line_key(line).as_deref() == Some("stop")
}

/// `queue_transaction` is not parked: at startup it is the protocol line the run
/// consumes right after `selected`; mid-run the waits park it themselves.
pub(crate) fn is_delivery_control_line(line: &str) -> bool {
    matches!(
        control_line_key(line).as_deref(),
        Some("input" | "queue_edit" | "meta" | "approval_response" | "compact")
    )
}
