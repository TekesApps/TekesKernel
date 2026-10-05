use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use schema::IJsonValue;

use crate::{McpCancellationToken, McpError, McpPeer, McpPool, McpPoolKey, McpToolCallContext};

const QUEUE_DEPTH: usize = 64;

pub struct McpBroker {
    handle: McpBrokerHandle,
    thread: Option<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct McpBrokerHandle {
    sender: SyncSender<Command>,
}

enum Command {
    Task {
        key: McpPoolKey,
        method: String,
        params: IJsonValue,
        cancellation: McpCancellationToken,
        reply: SyncSender<Result<IJsonValue, McpError>>,
    },
    Register {
        key: McpPoolKey,
        peer: Box<dyn McpPeer>,
        always_on: bool,
        reply: SyncSender<Result<(), McpError>>,
    },
    Call {
        key: McpPoolKey,
        name: String,
        arguments: IJsonValue,
        context: Option<McpToolCallContext>,
        task_ttl_ms: Option<u64>,
        cancellation: McpCancellationToken,
        reply: SyncSender<Result<IJsonValue, McpError>>,
    },
    Remove {
        key: McpPoolKey,
        reply: SyncSender<Result<bool, McpError>>,
    },
    Release {
        key: McpPoolKey,
        reply: SyncSender<Result<bool, McpError>>,
    },
    CatalogGeneration {
        key: McpPoolKey,
        reply: SyncSender<Result<Option<u64>, McpError>>,
    },
}

impl McpBroker {
    pub fn start() -> Result<Self, McpError> {
        let (sender, receiver) = mpsc::sync_channel(QUEUE_DEPTH);
        let shutdown = Arc::new(AtomicBool::new(false));
        let thread_shutdown = Arc::clone(&shutdown);
        let thread = thread::Builder::new()
            .name("tekes-mcp-pool".to_owned())
            .spawn(move || run(receiver, &thread_shutdown))
            .map_err(|error| McpError::Transport(error.to_string()))?;
        Ok(Self {
            handle: McpBrokerHandle { sender },
            thread: Some(thread),
            shutdown,
        })
    }

    #[must_use]
    pub fn handle(&self) -> McpBrokerHandle {
        self.handle.clone()
    }
}

impl Drop for McpBroker {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl McpBrokerHandle {
    pub fn register(
        &self,
        key: McpPoolKey,
        peer: Box<dyn McpPeer>,
        always_on: bool,
    ) -> Result<(), McpError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Register {
                key,
                peer,
                always_on,
                reply: sender,
            })
            .map_err(map_send)?;
        receive(receiver, Duration::from_secs(30))
    }

    pub fn call_tool(
        &self,
        key: McpPoolKey,
        name: impl Into<String>,
        arguments: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> Result<IJsonValue, McpError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Call {
                key,
                name: name.into(),
                arguments,
                context: None,
                task_ttl_ms: None,
                cancellation,
                reply: sender,
            })
            .map_err(map_send)?;
        receive_mutation(receiver, Duration::from_secs(601))
    }

    /// Task-augmented call: `task: {ttl}` travels with the request; an
    /// external-effect context still binds the idempotency key.
    pub fn call_tool_augmented(
        &self,
        key: McpPoolKey,
        name: impl Into<String>,
        arguments: IJsonValue,
        context: Option<McpToolCallContext>,
        task_ttl_ms: u64,
        cancellation: McpCancellationToken,
    ) -> Result<IJsonValue, McpError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Call {
                key,
                name: name.into(),
                arguments,
                context,
                task_ttl_ms: Some(task_ttl_ms),
                cancellation,
                reply: sender,
            })
            .map_err(map_send)?;
        receive_mutation(receiver, Duration::from_secs(601))
    }

    pub fn call_tool_with_context(
        &self,
        key: McpPoolKey,
        name: impl Into<String>,
        arguments: IJsonValue,
        context: McpToolCallContext,
        cancellation: McpCancellationToken,
    ) -> Result<IJsonValue, McpError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Call {
                key,
                name: name.into(),
                arguments,
                context: Some(context),
                task_ttl_ms: None,
                cancellation,
                reply: sender,
            })
            .map_err(map_send)?;
        receive_mutation(receiver, Duration::from_secs(601))
    }

    pub fn task_operation(
        &self,
        key: McpPoolKey,
        method: &str,
        params: IJsonValue,
    ) -> Result<IJsonValue, McpError> {
        self.task_operation_cancellable(key, method, params, McpCancellationToken::default())
    }

    pub fn task_operation_cancellable(
        &self,
        key: McpPoolKey,
        method: &str,
        params: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> Result<IJsonValue, McpError> {
        if !matches!(method, "tasks/get" | "tasks/update" | "tasks/cancel") {
            return Err(McpError::Unsupported(method.into()));
        }
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Task {
                key,
                method: method.into(),
                params,
                cancellation,
                reply: sender,
            })
            .map_err(map_send)?;
        if method == "tasks/get" {
            receive(receiver, Duration::from_secs(601))
        } else {
            receive_mutation(receiver, Duration::from_secs(601))
        }
    }

    pub fn remove(&self, key: McpPoolKey) -> Result<bool, McpError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Remove { key, reply: sender })
            .map_err(map_send)?;
        receive(receiver, Duration::from_secs(30))
    }

    pub fn release(&self, key: McpPoolKey) -> Result<bool, McpError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Release { key, reply: sender })
            .map_err(map_send)?;
        receive(receiver, Duration::from_secs(30))
    }

    pub fn catalog_generation(&self, key: McpPoolKey) -> Result<Option<u64>, McpError> {
        let (sender, receiver) = mpsc::sync_channel(1);
        self.sender
            .try_send(Command::CatalogGeneration { key, reply: sender })
            .map_err(map_send)?;
        receive(receiver, Duration::from_secs(30))
    }
}

fn receive<T>(receiver: Receiver<Result<T, McpError>>, timeout: Duration) -> Result<T, McpError> {
    receiver
        .recv_timeout(timeout)
        .map_err(|_| McpError::Timeout("broker response".to_owned()))?
}

fn receive_mutation<T>(
    receiver: Receiver<Result<T, McpError>>,
    timeout: Duration,
) -> Result<T, McpError> {
    match receiver.recv_timeout(timeout) {
        Ok(result) => result,
        Err(mpsc::RecvTimeoutError::Timeout) => Err(McpError::UnknownEffect),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(McpError::Transport("MCP broker is closed".to_owned()))
        }
    }
}

fn map_send<T>(error: mpsc::TrySendError<T>) -> McpError {
    match error {
        mpsc::TrySendError::Full(_) => McpError::Transport("MCP broker backpressure".to_owned()),
        mpsc::TrySendError::Disconnected(_) => {
            McpError::Transport("MCP broker is closed".to_owned())
        }
    }
}

fn run(receiver: Receiver<Command>, shutdown: &AtomicBool) {
    let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
    else {
        return;
    };
    let pool = Arc::new(McpPool::new());
    while !shutdown.load(Ordering::Acquire) {
        let command = match receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(command) => command,
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        };
        match command {
            Command::Task {
                key,
                method,
                params,
                cancellation,
                reply,
            } => {
                let pool = Arc::clone(&pool);
                runtime.spawn(async move {
                    let result = async {
                        let peer = pool
                            .get(&key)
                            .await
                            .ok_or_else(|| McpError::Transport("MCP peer is absent".into()))?;
                        let mut guard = peer.lock().await;
                        let result = guard
                            .task_operation_cancellable(&method, params, cancellation)
                            .await;
                        drop(guard);
                        if matches!(result, Err(McpError::Cancelled))
                            || is_fatal_peer_error(&result)
                        {
                            let _ = pool.remove(&key).await;
                        }
                        result
                    }
                    .await;
                    let _ = reply.send(result);
                });
            }

            Command::Register {
                key,
                peer,
                always_on,
                reply,
            } => {
                let pool = Arc::clone(&pool);
                runtime.spawn(async move {
                    let result = pool
                        .replace_generation(key, peer, always_on)
                        .await
                        .map_err(|error| McpError::Transport(error.to_string()));
                    let result = match result {
                        Ok(peer) => {
                            if let Some(subscription) =
                                peer.lock().await.start_catalog_subscription()
                            {
                                tokio::spawn(async move {
                                    let _ = subscription.await;
                                });
                            }
                            Ok(())
                        }
                        Err(error) => Err(error),
                    };
                    let _ = reply.send(result);
                });
            }
            Command::Call {
                key,
                name,
                arguments,
                context,
                task_ttl_ms,
                cancellation,
                reply,
            } => {
                let pool = Arc::clone(&pool);
                runtime.spawn(async move {
                    let result = async {
                        let peer = pool
                            .get(&key)
                            .await
                            .ok_or_else(|| McpError::Transport("MCP peer is absent".to_owned()))?;
                        let mut guard = peer.lock().await;
                        if let Some(ttl) = task_ttl_ms {
                            let result = guard
                                .call_tool_augmented(&name, arguments, context, ttl, cancellation)
                                .await;
                            drop(guard);
                            if is_fatal_peer_error(&result) {
                                let _ = pool.remove(&key).await;
                            }
                            return result;
                        }
                        if let Some(request) = guard.start_scoped_tool(
                            &name,
                            &arguments,
                            context.as_ref(),
                            cancellation.clone(),
                        ) {
                            drop(guard);
                            // Modern HTTP failure belongs to this request. Its
                            // parent generation remains usable by sibling calls.
                            return request.await;
                        }
                        let result = {
                            let peer = &mut guard;
                            match context {
                                Some(context) => {
                                    peer.call_tool_with_context(
                                        &name,
                                        arguments,
                                        context,
                                        cancellation,
                                    )
                                    .await
                                }
                                None => peer.call_tool(&name, arguments, cancellation).await,
                            }
                        };
                        drop(guard);
                        if is_fatal_peer_error(&result) {
                            let _ = pool.remove(&key).await;
                        }
                        result
                    }
                    .await;
                    let _ = reply.send(result);
                });
            }
            Command::Remove { key, reply } => {
                let pool = Arc::clone(&pool);
                runtime.spawn(async move {
                    let result = pool.remove(&key).await;
                    let _ = reply.send(result);
                });
            }
            Command::Release { key, reply } => {
                let pool = Arc::clone(&pool);
                runtime.spawn(async move {
                    let result = pool.release(&key).await;
                    let _ = reply.send(result);
                });
            }
            Command::CatalogGeneration { key, reply } => {
                let pool = Arc::clone(&pool);
                runtime.spawn(async move {
                    let generation = match pool.get(&key).await {
                        Some(peer) => Some(peer.lock().await.catalog_generation()),
                        None => None,
                    };
                    let _ = reply.send(Ok(generation));
                });
            }
        }
    }
    let _ = runtime.block_on(pool.drain());
}

fn is_fatal_peer_error<T>(result: &Result<T, McpError>) -> bool {
    matches!(
        result,
        Err(McpError::Transport(_)
            | McpError::Timeout(_)
            | McpError::Protocol(_)
            | McpError::UnknownEffect)
    )
}
