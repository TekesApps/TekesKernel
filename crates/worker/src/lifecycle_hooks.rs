//! Frozen host extensions. No model tool call is fabricated for a hook invocation.
use super::*;
use sha2::{Digest, Sha256};
use store::AtomicPublisher;
use tools::{LifecycleEvent as Point, LifecycleHookBinding, LifecycleHookRequest};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(super) struct LifecycleHooks {
    bindings: Vec<LifecycleHookBinding>,
    root: PathBuf,
    workspace: String,
    thread: String,
    observed_through: std::cell::Cell<u64>,
    ledger_path: PathBuf,
    pub(super) cancelled: Arc<AtomicBool>,
}

impl LifecycleHooks {
    pub(super) fn load(profile: &RuntimeProfile, ledger: &LockedLedger) -> Result<Option<Self>> {
        let mut bindings = Vec::new();
        for (id, index) in &profile.instruction.effective.hooks {
            let source = &profile.instruction.sources[*index as usize];
            let value: Value = serde_json::from_str(&source.content)?;
            if value["format"] == 2 {
                let binding = tools::decode_lifecycle_hook_binding(source.content.as_bytes(), id)?;
                if binding.enabled {
                    bindings.push(binding);
                }
            }
        }
        if bindings.is_empty() {
            return Ok(None);
        }
        let digest = hex(&serde_json_canonicalizer::to_vec(&bindings)?);
        let root = ledger
            .path()
            .parent()
            .ok_or("missing thread folder")?
            .join("lifecycle-hooks")
            .join(digest)
            .join(hex(ledger
                .path()
                .file_name()
                .ok_or("missing ledger name")?
                .as_encoded_bytes()));
        let baseline_file = root.join("baseline.json");
        let baseline = if baseline_file.exists() {
            serde_json::from_slice(&fs::read(&baseline_file)?)?
        } else {
            let baseline = ledger.next_seq() - 1;
            AtomicPublisher::replace(&baseline_file, &serde_json::to_vec(&baseline)?)?;
            baseline
        };
        let thread = ledger
            .projection()
            .and_then(|p| p.events.first())
            .and_then(|e| e.string_field("thread"))
            .ok_or("missing thread identity")?
            .to_owned();
        Ok(Some(Self {
            bindings,
            root,
            workspace: profile.config.workspace.id.clone(),
            thread,
            observed_through: std::cell::Cell::new(baseline),
            ledger_path: ledger.path().to_path_buf(),
            cancelled: Arc::new(AtomicBool::new(false)),
        }))
    }

    /// Success receipts suppress duplicate delivery. A crash after the external
    /// effect but before its receipt can redeliver; consumers deduplicate event_id.
    fn invoke(&self, point: Point, identity: &str, turn: u64, data: Value) -> Vec<String> {
        let mut context = Vec::new();
        for binding in self.bindings.iter().filter(|b| b.event == point) {
            match self.invoke_one(binding, identity, turn, &data) {
                Ok(additions) => {
                    // Aggregate budget, in addition to the per-hook response limit.
                    for addition in additions {
                        if context.iter().map(String::len).sum::<usize>() + addition.len() <= 32_768
                        {
                            context.push(addition);
                        }
                    }
                }
                Err(_) => {
                    // Do not echo process output, environment or arbitrary error data.
                    eprintln!("lifecycle-hook failed: event={point:?} hook={}", binding.id);
                }
            }
        }
        context
    }

    fn invoke_one(
        &self,
        binding: &LifecycleHookBinding,
        identity: &str,
        turn: u64,
        data: &Value,
    ) -> Result<Vec<String>> {
        let event_id = hex(&serde_json_canonicalizer::to_vec(&json!({
            "thread": self.thread, "workspace": self.workspace, "binding": binding,
            "line": self.ledger_path.file_name().and_then(|name| name.to_str()),
            "identity": identity, "event": binding.event
        }))?);
        let receipt = self.root.join(format!("{event_id}.json"));
        if receipt.exists() {
            let response: tools::LifecycleHookResponse =
                serde_json::from_slice(&fs::read(receipt)?)?;
            if response.format != 2
                || response.event_id != event_id
                || response.hook_id != binding.id
                || response.context.len() > 32
                || response.context.iter().map(String::len).sum::<usize>() > 32_768
                || (binding.event != Point::ContextPrepare && !response.context.is_empty())
            {
                return Err("invalid hook receipt".into());
            }
            return Ok(response.context);
        }
        let data = scan(&json!({"source": {"ledger_path": self.ledger_path}, "payload": data}))?;
        let request = LifecycleHookRequest {
            format: 2,
            hook_id: binding.id.clone(),
            event_id,
            event: binding.event,
            workspace_id: self.workspace.clone(),
            thread_id: self.thread.clone(),
            turn_id: turn,
            data,
        };
        if tools::encode_hook_line(&request)?.len() > 2 * 1024 * 1024 {
            return Err("hook input exceeds limit".into());
        }
        let response = tools::run_lifecycle_hook_with_cancel(binding, &request, &|| {
            matches!(
                binding.event,
                Point::TurnBefore | Point::ContextPrepare | Point::ContextBeforeCompact
            ) && self.cancelled.load(Ordering::Acquire)
        })?;
        let safe_response = scan(&serde_json::to_value(response)?)?;
        let response: tools::LifecycleHookResponse =
            serde_json::from_value(serde_json::to_value(safe_response)?)?;
        AtomicPublisher::replace(receipt, &tools::encode_hook_line(&response)?)?;
        Ok(response.context)
    }

    pub(super) fn before_turn(&self, ledger: &LockedLedger, turn: u64) {
        let projection = match ledger.projection() {
            Some(p) => p,
            None => return,
        };
        if let Some(open) = projection
            .events
            .iter()
            .rev()
            .find(|e| *e.kind() == EventKind::TurnOpen && e.turn() == Some(turn))
        {
            self.invoke(
                Point::TurnBefore,
                &format!("turn-{}", open.seq()),
                turn,
                json!({"source_seq": open.seq(), "turn": open.raw()}),
            );
        }
    }

    pub(super) fn prepare(
        &self,
        ledger: &LockedLedger,
        turn: u64,
        items: &[IJsonValue],
    ) -> Vec<String> {
        self.invoke(
            Point::ContextPrepare,
            &format!("context-{}", ledger.next_seq()),
            turn,
            json!({"through_seq": ledger.next_seq() - 1, "items": items}),
        )
    }

    pub(super) fn before_compact(
        &self,
        ledger: &LockedLedger,
        turn: u64,
        covers: &[u64],
        manual: bool,
    ) {
        self.invoke(
            Point::ContextBeforeCompact,
            &format!("compact-{}", ledger.next_seq()),
            turn,
            json!({"through_seq": ledger.next_seq() - 1, "covers": covers, "manual": manual}),
        );
    }

    /// The source ledger is the durable outbox. Only records after first
    /// registration are observed; unacknowledged records retry on worker entry/exit.
    pub(super) fn observe(&self, ledger: &mut LockedLedger) {
        if ledger.next_seq() - 1 <= self.observed_through.get() {
            return;
        }
        let has_pending = ledger.projection().is_some_and(|projection| {
            projection.events.iter().any(|event| {
                event.seq() > self.observed_through.get()
                    && self.bindings.iter().any(|binding| {
                        matches!(
                            (event.kind(), binding.event),
                            (EventKind::ToolResult, Point::ToolCompleted)
                                | (EventKind::Settle, Point::TurnSettled)
                        )
                    })
            })
        });
        if !has_pending {
            self.observed_through.set(ledger.next_seq() - 1);
            return;
        }
        if ledger.sync_prefix().is_err() {
            eprintln!("lifecycle-hook observation withheld: source sync failed");
            return;
        }
        let Some(projection) = ledger.projection() else {
            return;
        };
        for event in projection
            .events
            .iter()
            .filter(|e| e.seq() > self.observed_through.get())
        {
            let point = match event.kind() {
                EventKind::ToolResult => Point::ToolCompleted,
                EventKind::Settle => Point::TurnSettled,
                _ => continue,
            };
            if !self.bindings.iter().any(|b| b.event == point) {
                continue;
            }
            self.invoke(
                point,
                &format!("source-{}", event.seq()),
                event.turn().unwrap_or(0),
                json!({"source_seq": event.seq(), "record": event.raw()}),
            );
        }
        self.observed_through.set(ledger.next_seq() - 1);
    }
}

fn scan(value: &Value) -> Result<IJsonValue> {
    let value = IJsonValue::parse(&serde_json::to_vec(value)?)?;
    match SecretScanner::default().scan(&value) {
        tools::SecretScan::Clean(value) | tools::SecretScan::Redacted(value) => Ok(value),
        tools::SecretScan::Withheld(_) => Err("hook content withheld".into()),
    }
}
fn hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
