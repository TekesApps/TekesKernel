use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde_json::{Map, Value};

use crate::event::ranges;
use crate::{Event, EventKind, SchemaError, Visibility};
use crate::{OriginTuple, ResumePolicy};

#[derive(Clone, Debug)]
pub struct LedgerProjection {
    pub events: Vec<Event>,
    pub last_seq: u64,
    pub latest_turn: Option<u64>,
    pub terminal_tail: bool,
    pub required_reader: u64,
    pub read_only: bool,
    pub origin_keys: BTreeMap<String, u64>,
    pub origin_tuples: BTreeMap<OriginTuple, u64>,
    pub lifecycle: LifecycleFacts,
    pub run_ranges: Vec<RunRange>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunRange {
    pub run: String,
    pub start_seq: u64,
    pub end_seq: u64,
}

/// A remote tool continuation the worker parked on: the call is unpaired and
/// its latest durable step names when the next poll is due.
#[derive(Clone, Debug, Default)]
struct ContinuationFold {
    poll_after: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleFacts {
    pub latest_turn: Option<u64>,
    pub terminal_tail: bool,
    pub stop_active: bool,
    pub stop_closure_point: Option<u64>,
    pub open_hold: bool,
    pub answered_hold: bool,
    pub runnable_inputs: Vec<u64>,
    pub turn_open_inputs: Vec<u64>,
    pub live_inputs: Vec<u64>,
    pub unstarted: bool,
    pub unresolved_work: bool,
    pub resume_policy: ResumePolicy,
    pub recovery_ordinal: u64,
    /// Latest `poll_after` among unpaired calls whose remote continuation is
    /// parked (no result, no hold). Recovery for such a line is due only after
    /// this RFC 3339 instant; `None` when no parked continuation exists.
    pub continuation_wait_until: Option<String>,
    /// `next_attempt_at` of the latest durable provider-admission wait
    /// (`state{subkind: "provider_admission"}`) whose retry has not been
    /// dispatched yet: the open turn owes that retry, and recovery for the
    /// line is due only after this RFC 3339 instant.
    pub admission_wait_until: Option<String>,
}

impl LifecycleFacts {
    /// Latest instant before which no recovery run is due: a parked remote
    /// continuation or a durable provider-admission wait, whichever is later.
    #[must_use]
    pub fn durable_wait_until(&self) -> Option<&str> {
        [
            self.continuation_wait_until.as_deref(),
            self.admission_wait_until.as_deref(),
        ]
        .into_iter()
        .flatten()
        .max()
    }
}

#[derive(Clone, Debug)]
struct InputState {
    steer: bool,
    target_turn: Option<u64>,
    queued_behind_hold: bool,
    consumed: bool,
    superseded: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AttemptStage {
    Open,
    Dispatched,
    Recovered,
    Terminal,
}

#[derive(Clone, Debug)]
struct AttemptState {
    turn: u64,
    stage: AttemptStage,
}

#[derive(Clone, Debug)]
struct StopState {
    closure_point: Option<u64>,
}

#[derive(Clone, Debug)]
struct ApprovalState {
    seq: u64,
    answered: bool,
}

#[derive(Clone, Debug)]
pub struct LedgerValidator {
    reader_version: u64,
    events: Vec<Event>,
    inputs: BTreeMap<u64, InputState>,
    attempts: HashMap<String, AttemptState>,
    tool_calls: HashMap<String, u64>,
    tool_results: HashMap<String, u64>,
    spawns: HashMap<String, u64>,
    child_results: HashSet<String>,
    approvals: HashMap<String, ApprovalState>,
    continuations: HashMap<String, ContinuationFold>,
    admission_wait: Option<String>,
    epochs: HashMap<String, u64>,
    latest_turn: Option<u64>,
    settled_turns: HashSet<u64>,
    latest_stop: Option<StopState>,
    genesis_has_parent: bool,
    required_reader: u64,
    read_only: bool,
    origin_keys: BTreeMap<String, u64>,
    origin_tuples: BTreeMap<OriginTuple, u64>,
    reconcile_since_settle: u64,
    resume_policy: ResumePolicy,
}

impl Default for LedgerValidator {
    fn default() -> Self {
        Self::new(1)
    }
}

impl LedgerValidator {
    #[must_use]
    pub fn new(reader_version: u64) -> Self {
        Self {
            reader_version,
            events: Vec::new(),
            inputs: BTreeMap::new(),
            attempts: HashMap::new(),
            tool_calls: HashMap::new(),
            tool_results: HashMap::new(),
            spawns: HashMap::new(),
            child_results: HashSet::new(),
            approvals: HashMap::new(),
            continuations: HashMap::new(),
            admission_wait: None,
            epochs: HashMap::new(),
            latest_turn: None,
            settled_turns: HashSet::new(),
            latest_stop: None,
            genesis_has_parent: false,
            required_reader: 1,
            read_only: false,
            origin_keys: BTreeMap::new(),
            origin_tuples: BTreeMap::new(),
            reconcile_since_settle: 0,
            resume_policy: ResumePolicy::Never,
        }
    }

    pub fn push(&mut self, event: Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let expected = self.events.len() as u64 + 1;
        if seq != expected {
            return Err(SchemaError::event(
                Some(seq),
                format!("seq must be {expected}, found {seq}"),
            ));
        }
        if seq == 1 && !matches!(event.kind(), EventKind::Genesis) {
            return Err(SchemaError::event(Some(seq), "seq 1 must be genesis"));
        }
        if seq > 1 && matches!(event.kind(), EventKind::Genesis) {
            return Err(SchemaError::event(
                Some(seq),
                "genesis appears more than once",
            ));
        }

        self.apply_reader_gate(&event)?;
        self.validate_backward_references(&event)?;
        self.validate_turn_allocation(&event)?;
        self.validate_kind_relations(&event)?;
        self.apply_supersedes(&event)?;
        self.record_origin(&event)?;
        self.events.push(event);
        Ok(())
    }

    pub fn finish(self) -> Result<LedgerProjection, SchemaError> {
        let lifecycle = self.lifecycle_facts();
        let mut run_ranges = Vec::<RunRange>::new();
        for event in &self.events {
            if matches!(event.kind(), EventKind::RunStart) {
                if let Some(previous) = run_ranges.last_mut() {
                    previous.end_seq = event.seq() - 1;
                }
                run_ranges.push(RunRange {
                    run: event
                        .string_field("run")
                        .expect("validated run_start has run")
                        .to_owned(),
                    start_seq: event.seq(),
                    end_seq: self.events.last().map_or(event.seq(), Event::seq),
                });
            }
        }
        Ok(LedgerProjection {
            last_seq: self.events.last().map_or(0, Event::seq),
            latest_turn: self.latest_turn,
            terminal_tail: self
                .latest_turn
                .is_some_and(|turn| self.settled_turns.contains(&turn)),
            events: self.events,
            required_reader: self.required_reader,
            read_only: self.read_only,
            origin_keys: self.origin_keys,
            origin_tuples: self.origin_tuples,
            lifecycle,
            run_ranges,
        })
    }

    fn lifecycle_facts(&self) -> LifecycleFacts {
        let terminal_tail = self
            .latest_turn
            .is_some_and(|turn| self.settled_turns.contains(&turn));
        let latest_hold = self
            .approvals
            .iter()
            .filter(|(call, _)| !self.tool_results.contains_key(*call))
            .max_by_key(|(_, state)| state.seq)
            .map(|(_, state)| state);
        let open_hold = latest_hold.is_some_and(|state| !state.answered);
        let answered_hold = latest_hold.is_some_and(|state| state.answered);
        let stop_active = self
            .latest_stop
            .as_ref()
            .is_some_and(|stop| stop.closure_point.is_none());
        let closure_point = self
            .latest_stop
            .as_ref()
            .and_then(|stop| stop.closure_point);
        let runnable_inputs = if open_hold || stop_active {
            Vec::new()
        } else {
            self.inputs
                .iter()
                .filter_map(|(seq, input)| {
                    (!input.steer
                        && !input.consumed
                        && !input.superseded
                        && (!input.queued_behind_hold || terminal_tail)
                        && closure_point.is_none_or(|closure| *seq > closure))
                    .then_some(*seq)
                })
                .collect()
        };
        let turn_open_inputs = runnable_inputs.last().map_or_else(Vec::new, |maximum| {
            self.inputs
                .iter()
                .filter_map(|(seq, input)| {
                    (*seq <= *maximum && !input.steer && !input.consumed && !input.superseded)
                        .then_some(*seq)
                })
                .collect()
        });
        let live_inputs = self
            .inputs
            .iter()
            .filter_map(|(seq, input)| (!input.consumed && !input.superseded).then_some(*seq))
            .collect();
        let unresolved_attempt = self
            .attempts
            .values()
            .any(|attempt| attempt.stage != AttemptStage::Terminal);
        // A serial provider batch can park at one call while later calls have
        // no approval yet. Tracked, never-started siblings behind that hold are
        // waiting, not crash residue. Otherwise recovery races the real answer
        // and aborts the held call. An unfinished response remains unresolved
        // above; legacy/untracked or already-started calls still need recovery.
        let waiting_siblings = if open_hold {
            let calls: HashMap<_, _> = self
                .events
                .iter()
                .filter(|e| e.kind() == &EventKind::ToolCall)
                .filter_map(|e| e.string_field("call").map(|id| (id, e)))
                .collect();
            let started: HashSet<_> = self
                .events
                .iter()
                .filter(|e| e.kind() == &EventKind::ToolExecutionStarted)
                .filter_map(|e| e.string_field("call"))
                .collect();
            calls
                .iter()
                .filter_map(|(id, event)| {
                    let tracked = event
                        .object()
                        .get("execution_tracked")
                        .and_then(Value::as_bool)
                        == Some(true);
                    let behind_hold = self.approvals.iter().any(|(held, approval)| {
                        !approval.answered
                            && !self.tool_results.contains_key(held)
                            && calls.get(held.as_str()).is_some_and(|head| {
                                head.seq() < event.seq()
                                    && head.string_field("attempt") == event.string_field("attempt")
                            })
                    });
                    (tracked && !started.contains(id) && behind_hold).then_some(*id)
                })
                .collect::<HashSet<_>>()
        } else {
            HashSet::new()
        };
        let unresolved_tool = self.tool_calls.keys().any(|call| {
            !self.tool_results.contains_key(call)
                && !self.approvals.contains_key(call)
                && !waiting_siblings.contains(call.as_str())
        });
        let unresolved_spawn = self
            .spawns
            .keys()
            .any(|call| !self.child_results.contains(call));
        let continuation_wait_until = self
            .continuations
            .iter()
            .filter(|(call, _)| {
                self.tool_calls.contains_key(*call)
                    && !self.tool_results.contains_key(*call)
                    && !self.approvals.contains_key(*call)
            })
            .filter_map(|(_, fold)| fold.poll_after.clone())
            .max();
        LifecycleFacts {
            latest_turn: self.latest_turn,
            terminal_tail,
            stop_active,
            stop_closure_point: closure_point,
            open_hold,
            answered_hold,
            runnable_inputs,
            turn_open_inputs,
            live_inputs,
            unstarted: self.genesis_has_parent && self.latest_turn.is_none(),
            unresolved_work: unresolved_attempt || unresolved_tool || unresolved_spawn,
            resume_policy: self.resume_policy,
            recovery_ordinal: self.reconcile_since_settle,
            continuation_wait_until,
            admission_wait_until: self.admission_wait.clone(),
        }
    }

    fn apply_reader_gate(&mut self, event: &Event) -> Result<(), SchemaError> {
        let object = event.object();
        if matches!(event.kind(), EventKind::Genesis) {
            self.required_reader = value_u64(object, "min_reader", event.seq())?;
            self.genesis_has_parent = object.contains_key("parent");
            self.resume_policy = parse_resume(object.get("resume"), event.seq())?;
        }
        if let Some(version) = event.min_reader() {
            self.required_reader = self.required_reader.max(version);
        }
        if matches!(event.kind(), EventKind::Meta) {
            if let Some(upgrade) = object.get("upgrade").and_then(Value::as_object) {
                self.required_reader =
                    self.required_reader
                        .max(value_u64(upgrade, "min_reader", event.seq())?);
            }
        }
        if self.required_reader > self.reader_version {
            self.read_only = true;
        }
        Ok(())
    }

    fn validate_backward_references(&self, event: &Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let object = event.object();
        for field in ["supersedes"] {
            if object.contains_key(field) {
                for (from, to) in ranges(object, field, seq)? {
                    if from >= seq || to >= seq {
                        return Err(SchemaError::constraint(
                            7,
                            seq,
                            format!("{field} must point backward"),
                        ));
                    }
                }
            }
        }
        match event.kind() {
            EventKind::Compact => {
                for (from, to) in ranges(object, "covers", seq)? {
                    if from >= seq || to >= seq {
                        return Err(SchemaError::constraint(
                            7,
                            seq,
                            "compact.covers must point backward",
                        ));
                    }
                }
            }
            EventKind::Checkpoint => {
                let covers = value_u64(object, "covers", seq)?;
                if covers >= seq {
                    return Err(SchemaError::constraint(
                        7,
                        seq,
                        "checkpoint.covers must point backward",
                    ));
                }
            }
            EventKind::Attempt => {
                let epoch = value_str(object, "epoch", seq)?;
                if self
                    .epochs
                    .get(epoch)
                    .is_none_or(|epoch_seq| *epoch_seq >= seq)
                {
                    return Err(SchemaError::constraint(
                        7,
                        seq,
                        "attempt.epoch must name an earlier epoch",
                    ));
                }
                for (from, to) in ranges(object, "admits", seq)? {
                    if from >= seq || to >= seq {
                        return Err(SchemaError::constraint(
                            7,
                            seq,
                            "attempt.admits must point backward",
                        ));
                    }
                }
            }
            EventKind::Epoch => {
                if let Some(pending) = object.get("pending").and_then(Value::as_object) {
                    for field in ["eligible", "withheld"] {
                        for reference in value_array(pending, field, seq)? {
                            if reference.as_u64().is_none_or(|value| value >= seq) {
                                return Err(SchemaError::constraint(
                                    7,
                                    seq,
                                    "epoch.pending must point backward",
                                ));
                            }
                        }
                    }
                }
            }
            EventKind::TurnOpen => {
                if let Some(inputs) = trigger_inputs(object, seq)? {
                    if inputs.iter().any(|value| *value >= seq) {
                        return Err(SchemaError::constraint(
                            7,
                            seq,
                            "turn_open inputs must point backward",
                        ));
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_turn_allocation(&mut self, event: &Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let Some(turn) = event.turn() else {
            return Ok(());
        };
        if matches!(event.kind(), EventKind::TurnOpen) {
            let expected = self.latest_turn.map_or(1, |value| value + 1);
            if turn != expected {
                return Err(SchemaError::constraint(
                    8,
                    seq,
                    format!("turn_open must open turn {expected}"),
                ));
            }
            if let Some(previous) = self.latest_turn {
                if !self.settled_turns.contains(&previous) {
                    return Err(SchemaError::constraint(
                        8,
                        seq,
                        "next turn opened before prior settle",
                    ));
                }
            }
            self.validate_turn_trigger(event)?;
            self.latest_turn = Some(turn);
            return Ok(());
        }
        if self.latest_turn != Some(turn) {
            return Err(SchemaError::constraint(
                8,
                seq,
                "turn-bound event does not match latest opened turn",
            ));
        }
        if self.settled_turns.contains(&turn) && !matches!(event.kind(), EventKind::Settle) {
            return Err(SchemaError::constraint(
                8,
                seq,
                "turn-bound event appended after settle",
            ));
        }
        Ok(())
    }

    fn validate_turn_trigger(&mut self, event: &Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let turn = event.turn().expect("turn_open is turn-bound");
        let object = event.object();
        let trigger = object.get("trigger").expect("event payload validated");
        if trigger.as_str() == Some("genesis") {
            if turn != 1 || !self.genesis_has_parent {
                return Err(SchemaError::constraint(
                    8,
                    seq,
                    "genesis trigger requires spawned child turn 1",
                ));
            }
            return Ok(());
        }
        if trigger.get("goal").is_some() {
            if self.genesis_has_parent
                || turn <= 1
                || !self.settled_turns.contains(&(turn - 1))
                || self
                    .latest_stop
                    .as_ref()
                    .is_some_and(|stop| stop.closure_point.is_none())
                || self.has_open_hold()
                || self
                    .inputs
                    .values()
                    .any(|input| !input.steer && !input.consumed && !input.superseded)
            {
                return Err(SchemaError::constraint(
                    8,
                    seq,
                    "goal continuation requires a settled root turn without pending input or stop",
                ));
            }
            return Ok(());
        }
        let inputs = trigger_inputs(object, seq)?.expect("non-genesis trigger has inputs");
        let maximum = *inputs
            .last()
            .expect("payload validation rejects empty inputs");
        if self
            .latest_stop
            .as_ref()
            .is_some_and(|stop| stop.closure_point.is_none())
        {
            return Err(SchemaError::constraint(
                8,
                seq,
                "turn cannot open while stop generation is active",
            ));
        }
        if self
            .latest_stop
            .as_ref()
            .and_then(|stop| stop.closure_point)
            .is_some_and(|closure| maximum <= closure)
        {
            return Err(SchemaError::constraint(
                8,
                seq,
                "maximum consumed input is not runnable after stop closure",
            ));
        }
        if self.has_open_hold() {
            return Err(SchemaError::constraint(
                8,
                seq,
                "turn cannot open while approval hold is open",
            ));
        }
        let named: BTreeSet<u64> = inputs.iter().copied().collect();
        for input_seq in &inputs {
            let input = self.inputs.get(input_seq).ok_or_else(|| {
                SchemaError::constraint(
                    8,
                    seq,
                    format!("turn_open names non-input seq {input_seq}"),
                )
            })?;
            if input.steer || input.consumed || input.superseded {
                return Err(SchemaError::constraint(
                    8,
                    seq,
                    format!("input {input_seq} is not live and consumable"),
                ));
            }
        }
        for (input_seq, input) in self.inputs.range(..maximum) {
            if !input.steer && !input.consumed && !input.superseded && !named.contains(input_seq) {
                return Err(SchemaError::constraint(
                    8,
                    seq,
                    format!("prefix-completeness skipped input {input_seq}"),
                ));
            }
        }
        for input_seq in inputs {
            self.inputs
                .get_mut(&input_seq)
                .expect("checked above")
                .consumed = true;
        }
        Ok(())
    }

    fn validate_kind_relations(&mut self, event: &Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let object = event.object();
        match event.kind() {
            EventKind::Input => {
                let steer = object
                    .get("steer")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let queued_behind_hold = self.has_open_hold();
                self.inputs.insert(
                    seq,
                    InputState {
                        steer,
                        target_turn: steer.then_some(self.latest_turn).flatten(),
                        queued_behind_hold,
                        consumed: false,
                        superseded: false,
                    },
                );
            }
            EventKind::Epoch => {
                let id = value_str(object, "id", seq)?.to_owned();
                if self.epochs.insert(id, seq).is_some() {
                    return Err(SchemaError::constraint(2, seq, "epoch id reused"));
                }
            }
            EventKind::Attempt => {
                self.admission_wait = None;
                self.open_attempt(event)?;
            }
            EventKind::AttemptDispatched => {
                let id = value_str(object, "attempt", seq)?;
                let attempt = self
                    .attempts
                    .get_mut(id)
                    .ok_or_else(|| SchemaError::constraint(2, seq, "dispatch without attempt"))?;
                if attempt.stage != AttemptStage::Open {
                    return Err(SchemaError::constraint(
                        2,
                        seq,
                        "attempt_dispatched out of order",
                    ));
                }
                attempt.stage = AttemptStage::Dispatched;
            }
            EventKind::AttemptRecovery => {
                let id = value_str(object, "attempt", seq)?;
                let attempt = self
                    .attempts
                    .get_mut(id)
                    .ok_or_else(|| SchemaError::constraint(2, seq, "recovery without attempt"))?;
                if !matches!(attempt.stage, AttemptStage::Open | AttemptStage::Dispatched) {
                    return Err(SchemaError::constraint(
                        2,
                        seq,
                        "attempt_recovery duplicated or out of order",
                    ));
                }
                attempt.stage = AttemptStage::Recovered;
            }
            EventKind::Reasoning => {
                let id = value_str(object, "attempt", seq)?;
                let attempt = self
                    .attempts
                    .get(id)
                    .ok_or_else(|| SchemaError::constraint(2, seq, "reasoning without attempt"))?;
                if attempt.stage == AttemptStage::Terminal {
                    return Err(SchemaError::constraint(
                        2,
                        seq,
                        "reasoning after attempt outcome",
                    ));
                }
            }
            EventKind::Output => self.finish_attempt(event)?,
            EventKind::Error if object.contains_key("attempt") => self.finish_attempt(event)?,
            EventKind::ToolCall => {
                let call = value_str(object, "call", seq)?.to_owned();
                if self.tool_calls.insert(call, seq).is_some() {
                    return Err(SchemaError::constraint(3, seq, "tool call id reused"));
                }
            }
            EventKind::ToolExecutionStarted => {
                let call = value_str(object, "call", seq)?;
                if !self.tool_calls.contains_key(call) || self.tool_results.contains_key(call) {
                    return Err(SchemaError::constraint(
                        3,
                        seq,
                        "execution start requires an unresolved tool_call",
                    ));
                }
                if self
                    .approvals
                    .get(call)
                    .is_some_and(|approval| !approval.answered)
                {
                    return Err(SchemaError::constraint(
                        3,
                        seq,
                        "execution start precedes approval response",
                    ));
                }
                if self
                    .events
                    .iter()
                    .rev()
                    .find(|event| {
                        event.kind() == &EventKind::ApprovalResponse
                            && event.string_field("call") == Some(call)
                    })
                    .is_some_and(|event| {
                        event.object().get("grant").and_then(Value::as_bool) != Some(true)
                    })
                {
                    return Err(SchemaError::constraint(
                        3,
                        seq,
                        "execution start follows approval denial",
                    ));
                }
            }
            EventKind::ToolResult => {
                let call = value_str(object, "call", seq)?.to_owned();
                if !self.tool_calls.contains_key(&call) {
                    return Err(SchemaError::constraint(3, seq, "tool_result is unpaired"));
                }
                // A call still pairs with exactly one live result. A second
                // result is legal only as an explicit replacement of the one
                // it retracts (a compaction-family trim), so the pairing stays
                // 1:1 in every projection and the original stays on disk.
                if let Some(previous) = self.tool_results.insert(call, seq) {
                    let retracts = object
                        .contains_key("supersedes")
                        .then(|| ranges(object, "supersedes", seq))
                        .transpose()?
                        .unwrap_or_default()
                        .into_iter()
                        .any(|(from, to)| from <= previous && previous <= to);
                    if !retracts {
                        return Err(SchemaError::constraint(
                            3,
                            seq,
                            "a second tool_result must supersede the one it replaces",
                        ));
                    }
                }
            }
            EventKind::Spawn => {
                let call = value_str(object, "call", seq)?.to_owned();
                if self.spawns.insert(call, seq).is_some() {
                    return Err(SchemaError::constraint(3, seq, "spawn call id reused"));
                }
            }
            EventKind::ChildResult => {
                let call = value_str(object, "call", seq)?.to_owned();
                if !self.spawns.contains_key(&call) || !self.child_results.insert(call) {
                    return Err(SchemaError::constraint(
                        3,
                        seq,
                        "child_result is unpaired or duplicated",
                    ));
                }
            }
            EventKind::State
                if object.get("subkind").and_then(serde_json::Value::as_str)
                    == Some("provider_admission") =>
            {
                self.admission_wait = object
                    .get("payload")
                    .and_then(serde_json::Value::as_object)
                    .and_then(|payload| payload.get("next_attempt_at"))
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned);
            }
            EventKind::State
                if object.get("subkind").and_then(serde_json::Value::as_str)
                    == Some("tool_continuation") =>
            {
                if let Some(payload) = object.get("payload").and_then(serde_json::Value::as_object)
                {
                    if let Some(call) = payload.get("call").and_then(serde_json::Value::as_str) {
                        let poll_after = payload
                            .get("poll_after")
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_owned);
                        self.continuations
                            .insert(call.to_owned(), ContinuationFold { poll_after });
                    }
                }
            }
            EventKind::ApprovalRequest => {
                let call = value_str(object, "call", seq)?.to_owned();
                if self
                    .approvals
                    .insert(
                        call,
                        ApprovalState {
                            seq,
                            answered: false,
                        },
                    )
                    .is_some()
                {
                    return Err(SchemaError::constraint(
                        3,
                        seq,
                        "approval_request call id reused",
                    ));
                }
            }
            EventKind::ApprovalResponse => {
                let call = value_str(object, "call", seq)?;
                let state = self.approvals.get_mut(call).ok_or_else(|| {
                    SchemaError::constraint(3, seq, "approval_response without request")
                })?;
                if state.answered {
                    return Err(SchemaError::constraint(
                        3,
                        seq,
                        "approval_response duplicated",
                    ));
                }
                state.answered = true;
            }
            EventKind::Settle => {
                self.admission_wait = None;
                let turn = event.turn().expect("settle is turn-bound");
                if !self.settled_turns.insert(turn) {
                    return Err(SchemaError::constraint(
                        1,
                        seq,
                        "turn has multiple final settles",
                    ));
                }
                if let Some(stop) = self.latest_stop.as_mut() {
                    if stop.closure_point.is_none() {
                        stop.closure_point = Some(seq);
                    }
                }
                self.reconcile_since_settle = 0;
            }
            EventKind::StopRequested => {
                let open_turn = self
                    .latest_turn
                    .is_some_and(|turn| !self.settled_turns.contains(&turn));
                let unstarted_child = self.genesis_has_parent && self.latest_turn.is_none();
                let closure_point = if open_turn || unstarted_child {
                    None
                } else {
                    Some(seq)
                };
                self.latest_stop = Some(StopState { closure_point });
            }
            EventKind::RunStart => {
                let mode = value_str(object, "mode", seq)?;
                let ordinal = value_u64(object, "recovery_ordinal", seq)?;
                if ordinal != self.reconcile_since_settle {
                    return Err(SchemaError::constraint(
                        9,
                        seq,
                        format!(
                            "recovery_ordinal must be {}, found {ordinal}",
                            self.reconcile_since_settle
                        ),
                    ));
                }
                if mode == "reconcile" {
                    self.reconcile_since_settle += 1;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn open_attempt(&mut self, event: &Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let object = event.object();
        let id = value_str(object, "attempt", seq)?.to_owned();
        if self.attempts.contains_key(&id) {
            return Err(SchemaError::constraint(2, seq, "attempt id reused"));
        }
        let turn = event.turn().expect("attempt is turn-bound");
        for (from, to) in ranges(object, "admits", seq)? {
            for admitted_seq in from..=to {
                let admitted_index = usize::try_from(admitted_seq - 1).map_err(|_| {
                    SchemaError::constraint(7, seq, "admitted seq does not fit this platform")
                })?;
                let admitted = self.events.get(admitted_index).ok_or_else(|| {
                    SchemaError::constraint(
                        7,
                        seq,
                        format!("attempt admits future seq {admitted_seq}"),
                    )
                })?;
                if admitted.effective_visibility() != Visibility::Model {
                    return Err(SchemaError::constraint(
                        4,
                        seq,
                        format!("attempt admits non-model seq {admitted_seq}"),
                    ));
                }
                if matches!(admitted.kind(), EventKind::Input) {
                    let input = self
                        .inputs
                        .get(&admitted_seq)
                        .expect("input state recorded with event");
                    let eligible = if input.steer {
                        input.target_turn == Some(turn)
                    } else {
                        input.consumed && self.turn_consumed_input(turn, admitted_seq)
                    };
                    if !eligible {
                        return Err(SchemaError::constraint(
                            4,
                            seq,
                            format!("input {admitted_seq} is not eligible for turn {turn}"),
                        ));
                    }
                }
            }
        }
        self.attempts.insert(
            id,
            AttemptState {
                turn,
                stage: AttemptStage::Open,
            },
        );
        Ok(())
    }

    fn turn_consumed_input(&self, turn: u64, input_seq: u64) -> bool {
        self.events.iter().any(|event| {
            event.turn() == Some(turn)
                && matches!(event.kind(), EventKind::TurnOpen)
                && trigger_inputs(event.object(), event.seq())
                    .ok()
                    .flatten()
                    .is_some_and(|inputs| inputs.contains(&input_seq))
        })
    }

    fn finish_attempt(&mut self, event: &Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let id = value_str(event.object(), "attempt", seq)?;
        let attempt = self
            .attempts
            .get_mut(id)
            .ok_or_else(|| SchemaError::constraint(2, seq, "attempt outcome without attempt"))?;
        if attempt.turn != event.turn().expect("attempt outcome is turn-bound")
            || attempt.stage == AttemptStage::Terminal
        {
            return Err(SchemaError::constraint(
                2,
                seq,
                "exactly one outcome settles an attempt",
            ));
        }
        attempt.stage = AttemptStage::Terminal;
        Ok(())
    }

    fn apply_supersedes(&mut self, event: &Event) -> Result<(), SchemaError> {
        let seq = event.seq();
        let object = event.object();
        let Some(_) = object.get("supersedes") else {
            return Ok(());
        };
        for (from, to) in ranges(object, "supersedes", seq)? {
            for target in from..=to {
                if matches!(event.kind(), EventKind::QueueEdit) {
                    let input = self.inputs.get_mut(&target).ok_or_else(|| {
                        SchemaError::constraint(
                            8,
                            seq,
                            format!("queue_edit target {target} is not an input"),
                        )
                    })?;
                    if input.consumed {
                        return Err(SchemaError::constraint(
                            8,
                            seq,
                            format!("queue_edit target {target} already consumed"),
                        ));
                    }
                    input.superseded = true;
                } else if let Some(input) = self.inputs.get_mut(&target) {
                    input.superseded = true;
                }
            }
        }
        Ok(())
    }

    fn record_origin(&mut self, event: &Event) -> Result<(), SchemaError> {
        let Some(origin_key) = event.origin_key() else {
            return Ok(());
        };
        if let Some(tuple) = event.origin_tuple()? {
            if let Some(previous) = self.origin_tuples.get(&tuple) {
                return Err(SchemaError::event(
                    Some(event.seq()),
                    format!("origin tuple already mapped to seq {previous}"),
                ));
            }
            self.origin_tuples.insert(tuple, event.seq());
        }
        self.origin_keys.insert(origin_key.to_owned(), event.seq());
        Ok(())
    }

    fn has_open_hold(&self) -> bool {
        self.approvals
            .iter()
            .filter(|(call, _)| !self.tool_results.contains_key(*call))
            .max_by_key(|(_, state)| state.seq)
            .is_some_and(|(_, state)| !state.answered)
    }
}

pub fn validate_ledger(bytes: &[u8], reader_version: u64) -> Result<LedgerProjection, SchemaError> {
    if bytes.is_empty() {
        return Err(SchemaError::event(None, "ledger is empty"));
    }
    if !bytes.ends_with(b"\n") {
        return Err(SchemaError::event(
            None,
            "ledger must end each event with LF",
        ));
    }
    let mut validator = LedgerValidator::new(reader_version);
    for line in bytes
        .split(|byte| *byte == b'\n')
        .take_while(|line| !line.is_empty())
    {
        validator.push(Event::decode_canonical(line)?)?;
    }
    validator.finish()
}

fn trigger_inputs(object: &Map<String, Value>, seq: u64) -> Result<Option<Vec<u64>>, SchemaError> {
    let trigger = object
        .get("trigger")
        .ok_or_else(|| SchemaError::event(Some(seq), "turn_open missing trigger"))?;
    if trigger.as_str() == Some("genesis") || trigger.get("goal").is_some() {
        return Ok(None);
    }
    let trigger = trigger
        .as_object()
        .ok_or_else(|| SchemaError::event(Some(seq), "turn_open trigger must be object"))?;
    value_array(trigger, "inputs", seq)?
        .iter()
        .map(|value| {
            value
                .as_u64()
                .ok_or_else(|| SchemaError::event(Some(seq), "turn_open input must be integer"))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn value_str<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<&'a str, SchemaError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be string")))
}

fn value_u64(object: &Map<String, Value>, field: &str, seq: u64) -> Result<u64, SchemaError> {
    object
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be integer")))
}

fn value_array<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<&'a [Value], SchemaError> {
    object
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be array")))
}

fn parse_resume(value: Option<&Value>, seq: u64) -> Result<ResumePolicy, SchemaError> {
    let value = value.ok_or_else(|| SchemaError::event(Some(seq), "genesis missing resume"))?;
    if value.as_str() == Some("never") {
        return Ok(ResumePolicy::Never);
    }
    let bounded = value
        .as_object()
        .and_then(|object| object.get("bounded"))
        .and_then(Value::as_u64)
        .ok_or_else(|| SchemaError::event(Some(seq), "invalid resume policy"))?;
    Ok(ResumePolicy::Bounded(bounded))
}
