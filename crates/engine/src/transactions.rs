use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryPhase {
    Received,
    Appended { seq: u64 },
    Durable { seq: u64 },
    Acknowledged { seq: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryCommit {
    pub delivery: String,
    pub phase: DeliveryPhase,
}

impl DeliveryCommit {
    #[must_use]
    pub fn new(delivery: impl Into<String>) -> Self {
        Self {
            delivery: delivery.into(),
            phase: DeliveryPhase::Received,
        }
    }

    pub fn appended(&mut self, seq: u64) -> Result<(), TransactionError> {
        if self.phase != DeliveryPhase::Received {
            return Err(TransactionError::Order("delivery append"));
        }
        self.phase = DeliveryPhase::Appended { seq };
        Ok(())
    }

    pub fn durable(&mut self) -> Result<(), TransactionError> {
        let DeliveryPhase::Appended { seq } = self.phase else {
            return Err(TransactionError::Order("delivery durability"));
        };
        self.phase = DeliveryPhase::Durable { seq };
        Ok(())
    }

    pub fn acknowledge(&mut self) -> Result<u64, TransactionError> {
        let DeliveryPhase::Durable { seq } = self.phase else {
            return Err(TransactionError::BeforeDurability);
        };
        self.phase = DeliveryPhase::Acknowledged { seq };
        Ok(seq)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutcomePhase {
    AttemptDurable,
    OutcomeAppended { outcome_seq: u64 },
    Durable { outcome_seq: u64 },
    LeaseReleased { outcome_seq: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptPhase {
    Identified,
    LeaseGranted,
    AttemptDurable { seq: u64 },
    DispatchDurable { attempt_seq: u64, dispatch_seq: u64 },
    HttpInFlight,
    TerminalReceived,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttemptFlow {
    pub attempt: String,
    pub marker_required: bool,
    pub phase: AttemptPhase,
}

impl AttemptFlow {
    #[must_use]
    pub fn new(attempt: impl Into<String>, marker_required: bool) -> Self {
        Self {
            attempt: attempt.into(),
            marker_required,
            phase: AttemptPhase::Identified,
        }
    }

    pub fn lease_granted(&mut self) -> Result<(), TransactionError> {
        if self.phase != AttemptPhase::Identified {
            return Err(TransactionError::Order("attempt lease"));
        }
        self.phase = AttemptPhase::LeaseGranted;
        Ok(())
    }

    pub fn attempt_durable(&mut self, seq: u64) -> Result<(), TransactionError> {
        if self.phase != AttemptPhase::LeaseGranted {
            return Err(TransactionError::Order("attempt barrier"));
        }
        self.phase = AttemptPhase::AttemptDurable { seq };
        Ok(())
    }

    pub fn dispatch_durable(&mut self, seq: u64) -> Result<(), TransactionError> {
        let AttemptPhase::AttemptDurable { seq: attempt_seq } = self.phase else {
            return Err(TransactionError::Order("dispatch barrier"));
        };
        self.phase = AttemptPhase::DispatchDurable {
            attempt_seq,
            dispatch_seq: seq,
        };
        Ok(())
    }

    pub fn begin_http(&mut self) -> Result<(), TransactionError> {
        let ready = if self.marker_required {
            matches!(self.phase, AttemptPhase::DispatchDurable { .. })
        } else {
            matches!(self.phase, AttemptPhase::AttemptDurable { .. })
        };
        if !ready {
            return Err(TransactionError::BeforeDurability);
        }
        self.phase = AttemptPhase::HttpInFlight;
        Ok(())
    }

    pub fn terminal_received(&mut self) -> Result<(), TransactionError> {
        if self.phase != AttemptPhase::HttpInFlight {
            return Err(TransactionError::Order("provider terminal"));
        }
        self.phase = AttemptPhase::TerminalReceived;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeCommit {
    pub attempt: String,
    pub phase: OutcomePhase,
}

impl OutcomeCommit {
    #[must_use]
    pub fn new(attempt: impl Into<String>) -> Self {
        Self {
            attempt: attempt.into(),
            phase: OutcomePhase::AttemptDurable,
        }
    }

    /// The settling `output`/`error` carries the attempt's usage; there is
    /// no separate usage record to order before it.
    pub fn outcome_appended(&mut self, seq: u64) -> Result<(), TransactionError> {
        if self.phase != OutcomePhase::AttemptDurable {
            return Err(TransactionError::Order("outcome append"));
        }
        self.phase = OutcomePhase::OutcomeAppended { outcome_seq: seq };
        Ok(())
    }

    pub fn durable(&mut self) -> Result<(), TransactionError> {
        let OutcomePhase::OutcomeAppended { outcome_seq } = self.phase else {
            return Err(TransactionError::Order("outcome durability"));
        };
        self.phase = OutcomePhase::Durable { outcome_seq };
        Ok(())
    }

    pub fn release_lease(&mut self) -> Result<u64, TransactionError> {
        let OutcomePhase::Durable { outcome_seq } = self.phase else {
            return Err(TransactionError::BeforeDurability);
        };
        self.phase = OutcomePhase::LeaseReleased { outcome_seq };
        Ok(outcome_seq)
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TransactionError {
    #[error("operation occurred before durability")]
    BeforeDurability,
    #[error("transaction step out of order: {0}")]
    Order(&'static str),
}
