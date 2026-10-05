use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use endpoint::{
    EndpointJournal, MaterializedPrompt, MuxFrame, NativeEndpoint, SessionHostDescription,
};
use schema::{IJsonValue, OriginTuple};
use tekes_supervisor::endpoint_carrier::{
    LiveRespondAuthority, ProductionCarrierAssembly, ProductionCarrierStreams,
};
use tekes_supervisor::endpoint_host::{
    ProductionEndpointHost, ProductionRouteFailure, ProviderReadinessAuthority,
    QueueTransactionAuthority, RuntimeProviderReadiness, SessionDeliveryAuthority,
};
use tokio::net::TcpListener;
use transport::{BearerToken, TransportConfig};

const AUTHORIZATION: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
const SESSION_ID: &str = "018f0000-0000-7000-8000-000000000003";
const TIMESTAMP: &str = "2026-08-28T09:00:00.000Z";

struct HarnessDelivery {
    endpoint: NativeEndpoint,
}

impl SessionDeliveryAuthority for HarnessDelivery {
    fn prompt(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        prompt: &MaterializedPrompt,
        steer: bool,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        let content = IJsonValue::parse(
            &serde_json::to_vec(&prompt.blocks)
                .map_err(|error| route_failure("internal", &error.to_string()))?,
        )
        .map_err(|error| route_failure("internal", &error.to_string()))?;
        self.endpoint
            .prompt_for_endpoint(session_id, timestamp, origin, content, steer)
            .map_err(|error| route_failure("internal", &error.to_string()))
    }

    fn cancel(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        self.endpoint
            .cancel_for_endpoint(session_id, timestamp, origin)
            .map_err(|error| route_failure("internal", &error.to_string()))
    }

    fn rename(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        self.endpoint
            .rename_session_for_endpoint(session_id, timestamp, origin, title)
            .map_err(|error| route_failure("internal", &error.to_string()))
    }
}

struct HarnessProviders;

impl ProviderReadinessAuthority for HarnessProviders {
    fn readiness(
        &self,
        _session_id: &str,
        _config: &profile::ConfigSnapshot,
    ) -> Result<Vec<RuntimeProviderReadiness>, ProductionRouteFailure> {
        Ok(Vec::new())
    }
}

struct HarnessQueue;

impl QueueTransactionAuthority for HarnessQueue {
    fn execute(
        &self,
        _session_id: &str,
        _transaction: &worker_control::QueueTransaction,
    ) -> Result<worker_control::QueueTransactionResult, ProductionRouteFailure> {
        Err(route_failure(
            "internal",
            "the Client harness does not drive queue recovery",
        ))
    }
}

struct NoLiveWorker;

impl LiveRespondAuthority for NoLiveWorker {
    fn deliver_if_live(
        &self,
        _session_id: &str,
        _response: &worker_control::ApprovalResponse,
    ) -> Result<Option<worker_control::Receipt>, ProductionRouteFailure> {
        Ok(None)
    }
}

fn route_failure(code: &str, diagnostic: &str) -> ProductionRouteFailure {
    eprintln!("slice9 Client harness authority failure: {diagnostic}");
    ProductionRouteFailure::new(
        code,
        if code == "session-not-found" {
            "Session was not found"
        } else {
            "Endpoint operation failed"
        },
        IJsonValue::parse_str("{}").expect("empty details are I-JSON"),
    )
}

async fn publish_one_durable_live_event(
    root: &Path,
    streams: ProductionCarrierStreams,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const GATE_WAIT_ATTEMPTS: usize = 60_000;
    const GATE_WAIT_INTERVAL: Duration = Duration::from_millis(10);

    let folder = root.join("threads").join(SESSION_ID);
    for _ in 0..GATE_WAIT_ATTEMPTS {
        if folder.is_dir() {
            break;
        }
        tokio::time::sleep(GATE_WAIT_INTERVAL).await;
    }
    if !folder.is_dir() {
        return Err("Client did not create the production session".into());
    }

    // A harmless queue projection is the registration barrier. It is sent
    // through the real mux and lets the harness append its live event only
    // after the Client generation is known to exist.
    for _ in 0..GATE_WAIT_ATTEMPTS {
        let delivered = streams.publish_session_frame(
            SESSION_ID,
            MuxFrame::Queue {
                session_id: SESSION_ID.to_owned(),
                items: Vec::new(),
            },
        )?;
        if delivered != 0 {
            // The journal is a projection of the ledger: publish the session's
            // first projected row (projecting it now if the doorbell has not).
            let journal = EndpointJournal::open(&folder)?;
            let ledger = std::fs::read(folder.join("main.jsonl"))?;
            let projection = schema::validate_ledger(&ledger, 1)?;
            let mut projected =
                endpoint::Projector::default().reconcile(&projection.events, &journal)?;
            let event = match projected.is_empty() {
                true => journal.event(0)?.ok_or("session has no projected event")?,
                false => projected.remove(0),
            };
            drop(journal);
            if streams.publish_durable_session_event(SESSION_ID, event, None)? == 0 {
                return Err("Client mux disappeared before durable event publish".into());
            }
            return Ok(());
        }
        tokio::time::sleep(GATE_WAIT_INTERVAL).await;
    }
    Err("Client did not open the production mux".into())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ready_file = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: slice9_production_client_harness_server READY_FILE")?;
    let root = tempfile::tempdir()?;
    let project = root.path().join("project");
    let home = root.path().join("home");
    std::fs::create_dir(&project)?;
    std::fs::create_dir(&home)?;
    let workspace_routes: Option<
        Arc<dyn tekes_supervisor::endpoint_host::ProductionEndpointRoutes>,
    > = std::env::var_os("TEKES_WORKSPACE_SERVICE_BIN")
        .map(|executable| {
            tekes_supervisor::workspace_routes::WorkspaceRoutes::open(
                root.path(),
                PathBuf::from(executable),
            )
            .map(|routes| {
                Arc::new(routes)
                    as Arc<dyn tekes_supervisor::endpoint_host::ProductionEndpointRoutes>
            })
            .map_err(|error| std::io::Error::other(error.message))
        })
        .transpose()?;
    let workspace_test_enabled = workspace_routes.is_some();

    let unary = ProductionEndpointHost::open_with_full_authorities(
        root.path(),
        SessionHostDescription {
            version: "0.1.0-rc.5".to_owned(),
            cwd: project.to_string_lossy().into_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: home.to_string_lossy().into_owned(),
            can_open_path: false,
        },
        Arc::new(|| Ok(TIMESTAMP.to_owned())),
        workspace_routes,
        Some(Arc::new(HarnessProviders)),
        Some(Arc::new(HarnessDelivery {
            endpoint: NativeEndpoint::open(root.path())?,
        })),
        Some(Arc::new(HarnessQueue)),
    )?;
    let config = TransportConfig::loopback(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        BearerToken::new([0; 32]),
    );
    let assembly =
        ProductionCarrierAssembly::assemble(root.path(), unary, Arc::new(NoLiveWorker), config)?;
    assembly.finish_recovery()?;
    let streams = assembly.streams().clone();
    let publisher_root = root.path().to_path_buf();
    tokio::spawn(async move {
        if let Err(error) = publish_one_durable_live_event(&publisher_root, streams).await {
            eprintln!("slice9 production live publisher failed: {error}");
        }
    });

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await?;
    let address = listener.local_addr()?;
    std::fs::write(
        ready_file,
        format!(
            "http://{address}\n{AUTHORIZATION}\n{}\n{}",
            project.display(),
            if workspace_test_enabled {
                format!("{}\n", root.path().display())
            } else {
                String::new()
            }
        ),
    )?;
    assembly.into_server().serve(listener).await?;
    Ok(())
}
