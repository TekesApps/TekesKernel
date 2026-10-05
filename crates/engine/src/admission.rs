use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmissionLease {
    pub attempt: String,
    pub class: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeaseRelease {
    Released,
    StaleNoOp,
}

#[derive(Debug)]
pub struct AdmissionPool {
    capacity: usize,
    held: HashMap<String, AdmissionLease>,
    queued: VecDeque<AdmissionLease>,
}

impl AdmissionPool {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            held: HashMap::new(),
            queued: VecDeque::new(),
        }
    }

    pub fn request(&mut self, lease: AdmissionLease) -> bool {
        if self.held.contains_key(&lease.attempt)
            || self.queued.iter().any(|item| item.attempt == lease.attempt)
        {
            return self.held.contains_key(&lease.attempt);
        }
        if self.held.len() < self.capacity {
            self.held.insert(lease.attempt.clone(), lease);
            true
        } else {
            self.queued.push_back(lease);
            false
        }
    }

    pub fn settle(&mut self, attempt: &str) -> LeaseRelease {
        if self.held.remove(attempt).is_none() {
            return LeaseRelease::StaleNoOp;
        }
        self.promote();
        LeaseRelease::Released
    }

    pub fn reap(&mut self, attempts: impl IntoIterator<Item = String>) {
        for attempt in attempts {
            self.held.remove(&attempt);
        }
        self.promote();
    }

    #[must_use]
    pub fn is_held(&self, attempt: &str) -> bool {
        self.held.contains_key(attempt)
    }

    #[must_use]
    pub fn held_count(&self) -> usize {
        self.held.len()
    }

    fn promote(&mut self) {
        while self.held.len() < self.capacity {
            let Some(next) = self.queued.pop_front() else {
                break;
            };
            self.held.insert(next.attempt.clone(), next);
        }
    }
}
