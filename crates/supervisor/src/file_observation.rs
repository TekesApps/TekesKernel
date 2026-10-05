//! Session-scoped subscription ownership; dropping a feed releases its state.
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use workspace_service::observation::FileObservations;

#[derive(Default)]
struct State {
    next_id: u64,
    feeds: BTreeMap<u64, (String, FileObservations)>,
}

#[derive(Clone, Default)]
pub struct SessionFileObservations(Arc<Mutex<State>>);

pub struct FileSubscription {
    registry: SessionFileObservations,
    id: u64,
    ready: bool,
    authority_check: Option<Box<dyn Fn() -> Result<(), String> + Send + Sync>>,
}

impl SessionFileObservations {
    /// Session existence/authorization must be checked by the caller before open.
    pub fn open(&self, session_id: &str) -> Result<FileSubscription, String> {
        endpoint::validate_session_id(session_id).map_err(|_| "Invalid session identity")?;
        let mut state = self.0.lock().map_err(|_| "Observation lock poisoned")?;
        if state.feeds.len() >= 128 {
            return Err("Too many file subscriptions".into());
        }
        state.next_id = state
            .next_id
            .checked_add(1)
            .ok_or("Subscription identity exhausted")?;
        let id = state.next_id;
        state
            .feeds
            .insert(id, (session_id.to_owned(), FileObservations::default()));
        Ok(FileSubscription {
            registry: self.clone(),
            id,
            ready: false,
            authority_check: None,
        })
    }

    pub fn register(&self, session_id: &str, root: &Path, relative: &str) -> Result<(), String> {
        let mut state = self.0.lock().map_err(|_| "Observation lock poisoned")?;
        for (session, observations) in state.feeds.values_mut() {
            if session == session_id {
                observations
                    .register(root, relative)
                    .map_err(|error| error.code.to_owned())?;
            }
        }
        Ok(())
    }
}

impl FileSubscription {
    pub(crate) fn with_authority_check(
        mut self,
        check: impl Fn() -> Result<(), String> + Send + Sync + 'static,
    ) -> Self {
        self.authority_check = Some(Box::new(check));
        self
    }

    pub fn poll(&mut self) -> Result<Vec<Value>, String> {
        if let Some(check) = &self.authority_check {
            check()?;
        }
        if !self.ready {
            self.ready = true;
            return Ok(vec![json!({"kind":"ready"})]);
        }
        let mut state = self
            .registry
            .0
            .lock()
            .map_err(|_| "Observation lock poisoned")?;
        let (_, observations) = state.feeds.get_mut(&self.id).ok_or("Subscription closed")?;
        let frames = observations.poll().map_err(|error| error.code.to_owned())?;
        if let Some(check) = &self.authority_check {
            check()?;
        }
        Ok(frames)
    }
}

impl Drop for FileSubscription {
    fn drop(&mut self) {
        if let Ok(mut state) = self.registry.0.lock() {
            state.feeds.remove(&self.id);
        }
    }
}

impl transport::FileChangeFeed for FileSubscription {
    fn poll(&mut self) -> Result<Vec<Value>, String> {
        FileSubscription::poll(self)
    }
}

impl transport::FileChangeAuthority for crate::process_host::ProductionProcessHost {
    fn open(&self, session_id: &str) -> Result<Box<dyn transport::FileChangeFeed>, String> {
        self.client_file_changes(session_id)
            .map(|feed| Box::new(feed) as Box<dyn transport::FileChangeFeed>)
            .map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn revoked_authority_stops_observation_before_file_poll() {
        use std::sync::atomic::{AtomicBool, Ordering};
        let allowed = Arc::new(AtomicBool::new(true));
        let registry = SessionFileObservations::default();
        let check = allowed.clone();
        let mut feed = registry
            .open("018f0000-0000-7000-8000-000000000001")
            .unwrap()
            .with_authority_check(move || {
                if check.load(Ordering::SeqCst) {
                    Ok(())
                } else {
                    Err("revoked".into())
                }
            });
        assert_eq!(feed.poll().unwrap(), vec![json!({"kind":"ready"})]);
        allowed.store(false, Ordering::SeqCst);
        assert_eq!(feed.poll().unwrap_err(), "revoked");
    }
    #[test]
    fn feeds_are_independent_session_scoped_and_released_on_drop() {
        let registry = SessionFileObservations::default();
        let one = "018f0000-0000-7000-8000-000000000001";
        let two = "018f0000-0000-7000-8000-000000000002";
        let mut first = registry.open(one).unwrap();
        let mut second = registry.open(one).unwrap();
        let mut other = registry.open(two).unwrap();
        for feed in [&mut first, &mut second, &mut other] {
            assert_eq!(feed.poll().unwrap(), vec![json!({"kind":"ready"})]);
        }
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("file"), "before").unwrap();
        registry.register(one, root.path(), "file").unwrap();
        std::fs::write(root.path().join("file"), "after").unwrap();
        assert_eq!(first.poll().unwrap().len(), 1);
        assert_eq!(second.poll().unwrap().len(), 1);
        assert!(other.poll().unwrap().is_empty());
        assert!(first.poll().unwrap().is_empty());
        drop(first);
        drop(second);
        drop(other);
        assert!(registry.0.lock().unwrap().feeds.is_empty());
        let mut reopened = registry.open(one).unwrap();
        assert_eq!(reopened.poll().unwrap(), vec![json!({"kind":"ready"})]);
    }
}
