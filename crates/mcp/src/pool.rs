use std::collections::BTreeMap;
use std::future::Future;
use std::sync::Arc;

use tokio::sync::{Mutex, Notify, RwLock};

use crate::{McpError, McpPeer};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct McpPoolKey {
    pub workspace: String,
    pub scope: String,
    pub server: String,
    pub config_digest: String,
    pub authorization_identity: String,
    pub protocol_mode: String,
    pub plugin_generation: Option<String>,
}

pub type PooledPeer = Arc<Mutex<Box<dyn McpPeer>>>;

struct PoolEntry {
    peer: PooledPeer,
    clients: usize,
    always_on: bool,
}

#[derive(Default)]
pub struct McpPool {
    peers: RwLock<BTreeMap<McpPoolKey, PoolEntry>>,
    creations: Mutex<BTreeMap<McpPoolKey, Arc<CreationSlot>>>,
}

struct CreationSlot {
    outcome: Mutex<Option<Result<PooledPeer, McpError>>>,
    ready: Notify,
}

pub struct McpPoolLease {
    pool: Arc<McpPool>,
    key: McpPoolKey,
    peer: PooledPeer,
    released: bool,
}

impl McpPoolLease {
    #[must_use]
    pub fn peer(&self) -> PooledPeer {
        Arc::clone(&self.peer)
    }

    pub async fn release(mut self) -> Result<bool, McpError> {
        self.released = true;
        self.pool.release(&self.key).await
    }
}

impl Drop for McpPoolLease {
    fn drop(&mut self) {
        if self.released {
            return;
        }
        let pool = Arc::clone(&self.pool);
        let key = self.key.clone();
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(async move {
                let _ = pool.release(&key).await;
            });
        }
    }
}

impl McpPool {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get(&self, key: &McpPoolKey) -> Option<PooledPeer> {
        self.peers
            .read()
            .await
            .get(key)
            .map(|entry| Arc::clone(&entry.peer))
    }

    async fn retain(&self, key: &McpPoolKey) -> Option<PooledPeer> {
        let mut peers = self.peers.write().await;
        let entry = peers.get_mut(key)?;
        entry.clients = entry.clients.saturating_add(1);
        Some(Arc::clone(&entry.peer))
    }

    pub async fn acquire_or_create<F, Fut>(
        self: &Arc<Self>,
        key: McpPoolKey,
        always_on: bool,
        factory: F,
    ) -> Result<McpPoolLease, McpError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<Box<dyn McpPeer>, McpError>> + Send,
    {
        if let Some(peer) = self.retain(&key).await {
            return Ok(McpPoolLease {
                pool: Arc::clone(self),
                key,
                peer,
                released: false,
            });
        }
        let (slot, creator) = {
            let mut creations = self.creations.lock().await;
            if let Some(slot) = creations.get(&key) {
                (Arc::clone(slot), false)
            } else {
                let slot = Arc::new(CreationSlot {
                    outcome: Mutex::new(None),
                    ready: Notify::new(),
                });
                creations.insert(key.clone(), Arc::clone(&slot));
                (slot, true)
            }
        };
        if creator {
            let outcome = match factory().await {
                Ok(peer) => self
                    .replace_generation(key.clone(), peer, always_on)
                    .await
                    .map_err(|error| McpError::Transport(error.to_string())),
                Err(error) => Err(error),
            };
            *slot.outcome.lock().await = Some(match &outcome {
                Ok(peer) => Ok(Arc::clone(peer)),
                Err(error) => Err(error.clone()),
            });
            slot.ready.notify_waiters();
            self.creations.lock().await.remove(&key);
            let peer = outcome?;
            return Ok(McpPoolLease {
                pool: Arc::clone(self),
                key,
                peer,
                released: false,
            });
        }
        loop {
            let notified = slot.ready.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if let Some(outcome) = slot.outcome.lock().await.as_ref() {
                match outcome {
                    Ok(_) => {
                        let peer = self.retain(&key).await.ok_or_else(|| {
                            McpError::Transport(
                                "MCP peer disappeared after single-flight creation".to_owned(),
                            )
                        })?;
                        return Ok(McpPoolLease {
                            pool: Arc::clone(self),
                            key,
                            peer,
                            released: false,
                        });
                    }
                    Err(error) => return Err(error.clone()),
                }
            }
            notified.await;
        }
    }

    pub async fn insert(
        &self,
        key: McpPoolKey,
        mut peer: Box<dyn McpPeer>,
    ) -> Result<PooledPeer, McpPoolError> {
        let mut peers = self.peers.write().await;
        if let Some(existing) = peers.get_mut(&key) {
            existing.clients = existing.clients.saturating_add(1);
            let existing = Arc::clone(&existing.peer);
            drop(peers);
            peer.close()
                .await
                .map_err(|error| McpPoolError::Close(error.to_string()))?;
            return Ok(existing);
        }
        let peer = Arc::new(Mutex::new(peer));
        peers.insert(
            key,
            PoolEntry {
                peer: Arc::clone(&peer),
                clients: 1,
                always_on: false,
            },
        );
        Ok(peer)
    }

    /// Installs one generation and closes every stale generation for the same
    /// workspace/scope/server identity before returning. Broker serialization
    /// makes this the single authority-change cutover.
    pub async fn replace_generation(
        &self,
        key: McpPoolKey,
        mut peer: Box<dyn McpPeer>,
        always_on: bool,
    ) -> Result<PooledPeer, McpPoolError> {
        let mut peers = self.peers.write().await;
        let stale_keys = peers
            .keys()
            .filter(|existing| {
                existing.workspace == key.workspace
                    && existing.scope == key.scope
                    && existing.server == key.server
                    && *existing != &key
            })
            .cloned()
            .collect::<Vec<_>>();
        let stale = stale_keys
            .iter()
            .filter_map(|stale| peers.remove(stale))
            .collect::<Vec<_>>();
        if let Some(existing) = peers.get_mut(&key) {
            existing.clients = existing.clients.saturating_add(1);
            existing.always_on |= always_on;
            let existing = Arc::clone(&existing.peer);
            drop(peers);
            for old in stale {
                old.peer
                    .lock()
                    .await
                    .close()
                    .await
                    .map_err(|error| McpPoolError::Close(error.to_string()))?;
            }
            peer.close()
                .await
                .map_err(|error| McpPoolError::Close(error.to_string()))?;
            return Ok(existing);
        }
        let peer = Arc::new(Mutex::new(peer));
        peers.insert(
            key,
            PoolEntry {
                peer: Arc::clone(&peer),
                clients: 1,
                always_on,
            },
        );
        drop(peers);
        for old in stale {
            old.peer
                .lock()
                .await
                .close()
                .await
                .map_err(|error| McpPoolError::Close(error.to_string()))?;
        }
        Ok(peer)
    }

    pub async fn remove(&self, key: &McpPoolKey) -> Result<bool, McpError> {
        let entry = self.peers.write().await.remove(key);
        let Some(entry) = entry else {
            return Ok(false);
        };
        entry.peer.lock().await.close().await?;
        Ok(true)
    }

    /// Releases one worker binding. v1 uses an immediate zero-client idle
    /// deadline; `always_on` entries remain until explicit removal/drain.
    pub async fn release(&self, key: &McpPoolKey) -> Result<bool, McpError> {
        let peer = {
            let mut peers = self.peers.write().await;
            let Some(entry) = peers.get_mut(key) else {
                return Ok(false);
            };
            entry.clients = entry.clients.saturating_sub(1);
            if entry.clients > 0 || entry.always_on {
                return Ok(false);
            }
            peers.remove(key).map(|entry| entry.peer)
        };
        if let Some(peer) = peer {
            peer.lock().await.close().await?;
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn drain(&self) -> Result<usize, McpError> {
        let entries = {
            let mut peers = self.peers.write().await;
            std::mem::take(&mut *peers)
                .into_values()
                .collect::<Vec<_>>()
        };
        let count = entries.len();
        for entry in entries {
            entry.peer.lock().await.close().await?;
        }
        Ok(count)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum McpPoolError {
    #[error("losing MCP peer could not be closed: {0}")]
    Close(String),
}
