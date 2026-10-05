use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrainAction {
    Sweep,
    ReapOwned,
    WaitBusyUnknown,
    Quarantine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryStage {
    SpawnEdges,
    Tools,
    Attempts,
    Holds,
}

pub const RECOVERY_ORDER: [RecoveryStage; 4] = [
    RecoveryStage::SpawnEdges,
    RecoveryStage::Tools,
    RecoveryStage::Attempts,
    RecoveryStage::Holds,
];

#[derive(Clone, Debug)]
pub struct DrainTracker {
    deadline_ticks: u64,
    busy_since: BTreeMap<String, u64>,
}

impl DrainTracker {
    #[must_use]
    pub fn new(deadline_ticks: u64) -> Self {
        Self {
            deadline_ticks,
            busy_since: BTreeMap::new(),
        }
    }

    pub fn observe(
        &mut self,
        line: &str,
        lock_busy: bool,
        owned_process: bool,
        now_tick: u64,
    ) -> DrainAction {
        if !lock_busy {
            self.busy_since.remove(line);
            return DrainAction::Sweep;
        }
        if owned_process {
            self.busy_since.remove(line);
            return DrainAction::ReapOwned;
        }
        let first_seen = *self.busy_since.entry(line.to_owned()).or_insert(now_tick);
        if now_tick.saturating_sub(first_seen) >= self.deadline_ticks {
            DrainAction::Quarantine
        } else {
            DrainAction::WaitBusyUnknown
        }
    }
}
