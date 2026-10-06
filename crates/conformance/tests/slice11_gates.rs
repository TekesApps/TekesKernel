use std::collections::BTreeMap;
use std::fs;
use std::future::Future;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

use endpoint::{
    ClientRequest, DurableHandoffSignal, EndpointHost, EndpointHostCall, MaterializedPrompt,
    MethodClass, MutationReceipt, NativeEndpoint, SessionHostDescription,
};
use profile::{
    ConfigRepository, EffectiveInstructions, InstructionKind, InstructionOrigin,
    InstructionResolver, InstructionSnapshot, InstructionSource, ResourceCatalog, WorkspaceConfig,
};
use schema::{IJsonValue, OriginTuple, ResumePolicy};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tekes_supervisor::endpoint_host::{
    CompositeProductionEndpointRoutes, ProductionEndpointAssembly, ProductionEndpointHost,
    ProductionEndpointRoutes, ProductionRouteFailure, SessionDeliveryAuthority,
    SessionInputAdmissionAuthority, TEKES_UNARY_ROUTES,
};
use tekes_supervisor::host_runtime::assemble_application_endpoint_host;
use tekes_supervisor::process_host::ProductionProcessHost;
use tekes_supervisor::resource_capability::{
    COMMANDS_LIST, COMMANDS_RUN, ClientResourceService, CommandInputAuthority, CommandRunRequest,
    EndpointCommandInputAuthority, KeyedCommandInput, ResourceCapabilityError, SKILLS_LIST,
};
use test_support::{FixtureRoot, read};

const INPUT_SESSION_ID: &str = "018f0000-0000-7000-8000-000000000011";
const CONFIG_DIGEST: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn fixture_catalog() -> ResourceCatalog {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let root = fixtures.join("resources/migration");
    let resolver = InstructionResolver::new(root.join("user"), [root.join("project")]);
    let snapshot = resolver.capture().expect("resource instruction snapshot");
    ResourceCatalog::from_snapshot(&snapshot).expect("resource catalog")
}

fn fixture_value(name: &str) -> Value {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    serde_json::from_slice(
        &read(&fixtures.join("resources/migration").join(name)).expect("resource fixture"),
    )
    .expect("resource fixture JSON")
}

fn bootstrap_input_session(root: &std::path::Path) {
    NativeEndpoint::open(root)
        .expect("native endpoint")
        .create_session(
            INPUT_SESSION_ID,
            "workspace-1",
            CONFIG_DIGEST,
            ResumePolicy::Never,
            "2026-08-29T00:00:00.000Z",
            &OriginTuple {
                principal: "uid:501".to_owned(),
                client: "slice11-gate".to_owned(),
                target: INPUT_SESSION_ID.to_owned(),
                op: "create".to_owned(),
                key: "create-input-session".to_owned(),
            },
        )
        .expect("create input session");
}

fn block_on_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("production host future unexpectedly pending"),
    }
}

#[test]
fn slice11_gate_77_skill_package_migration_and_explicit_collisions() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    fixtures
        .verify_manifest()
        .expect("complete fixture manifest");
    let catalog = fixture_catalog();
    let expected = fixture_value("expected.canonical.json");
    assert_eq!(
        serde_json::to_value(catalog.skill_summaries()).expect("skill summaries"),
        expected["skills"]
    );
    let package = catalog.skill("review").expect("review package");
    assert_eq!(package.summary.source, "project:0");
    assert_eq!(
        package
            .resources
            .iter()
            .map(|resource| resource.path.as_str())
            .collect::<Vec<_>>(),
        ["SKILL.md", "references/rules.md"]
    );
    assert_eq!(
        catalog
            .skill_resource("review", "references/rules.md")
            .expect("companion resource"),
        "# Project review rules\n\nPrefer durable evidence.\n"
    );
    assert!(catalog.skill_resource("review", "../SKILL.md").is_err());
    assert!(
        catalog
            .skill_resource("review", "scripts/check.sh")
            .is_err()
    );

    let invalid = tempfile::tempdir().expect("invalid package root");
    let user_package = invalid.path().join("user/skills/review");
    let project_package = invalid.path().join("project/.agent/skills/review");
    fs::create_dir_all(&user_package).expect("user package");
    fs::create_dir_all(&project_package).expect("project package");
    fs::write(
        user_package.join("SKILL.md"),
        "---\nname: review\ndescription: User\n---\nUser\n",
    )
    .expect("user manifest");
    fs::write(
        project_package.join("SKILL.md"),
        "---\nname: review\ndescription: Project\n---\nProject\n",
    )
    .expect("project manifest");
    let snapshot = InstructionResolver::new(
        invalid.path().join("user"),
        [invalid.path().join("project")],
    )
    .capture()
    .expect("colliding skill snapshot");
    let error = ResourceCatalog::from_snapshot(&snapshot).expect_err("skill collision");
    assert!(error.to_string().contains("defined by both"), "{error}");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let linked = tempfile::tempdir().expect("symlink package root");
        let package = linked.path().join("user/skills/linked");
        fs::create_dir_all(&package).expect("linked package");
        let outside = linked.path().join("outside.md");
        fs::write(
            &outside,
            "---\nname: linked\ndescription: Linked\n---\nLinked\n",
        )
        .expect("outside manifest");
        symlink(&outside, package.join("SKILL.md")).expect("manifest symlink");
        assert!(
            InstructionResolver::new(
                linked.path().join("user"),
                std::iter::empty::<&std::path::Path>(),
            )
            .capture()
            .is_err(),
            "skill package symlinks must fail before catalog construction"
        );
    }
}

#[test]
fn slice11_gate_78_command_catalog_expansion_and_negative_corpus() {
    let catalog = fixture_catalog();
    let expected = fixture_value("expected.canonical.json");
    assert_eq!(
        serde_json::to_value(catalog.command_summaries()).expect("command summaries"),
        expected["commands"]
    );
    let expansion = catalog
        .expand_command("review", "'src/lib.rs' strict")
        .expect("command expansion");
    assert_eq!(
        json!({
            "name": expansion.name,
            "text": expansion.text,
            "content_digest": expansion.content_digest,
            "arguments": "'src/lib.rs' strict"
        }),
        expected["command_expansion"]
    );
    let invalid = fixture_value("invalid-command.canonical.json");
    assert!(
        catalog
            .expand_command(
                invalid["name"].as_str().expect("name"),
                invalid["arguments"].as_str().expect("arguments"),
            )
            .is_err()
    );
    assert!(catalog.expand_command("missing", "").is_err());

    let positional = "Use $100\n";
    let expanding = "$ARGUMENTS".repeat(20);
    let sources = [
        ("commands/position.md", positional),
        ("commands/expand.md", expanding.as_str()),
    ]
    .into_iter()
    .map(|(path, content)| InstructionSource {
        origin: InstructionOrigin::User,
        path: path.to_owned(),
        kind: InstructionKind::Command,
        content: content.to_owned(),
        content_sha256: format!("{:x}", Sha256::digest(content.as_bytes())),
    })
    .collect::<Vec<_>>();
    let mut effective = EffectiveInstructions::default();
    effective.commands.insert("position.md".to_owned(), 0);
    effective.commands.insert("expand.md".to_owned(), 1);
    let limits = ResourceCatalog::from_snapshot(&InstructionSnapshot {
        format: 1,
        sources,
        effective,
    })
    .expect("limit catalog");
    assert!(limits.expand_command("position", "one").is_err());
    assert!(
        limits
            .expand_command("expand", &"x".repeat(64 * 1024))
            .is_err()
    );

    let alias = fixture_value("invalid-alias-command.canonical.json");
    let content = alias["content"].as_str().expect("alias command content");
    let snapshot = InstructionSnapshot {
        format: 1,
        sources: vec![InstructionSource {
            origin: InstructionOrigin::User,
            path: format!(
                "commands/{}.md",
                alias["name"].as_str().expect("alias name")
            ),
            kind: InstructionKind::Command,
            content: content.to_owned(),
            content_sha256: format!("{:x}", Sha256::digest(content.as_bytes())),
        }],
        effective: EffectiveInstructions {
            commands: [("ambiguous.md".to_owned(), 0)].into_iter().collect(),
            ..EffectiveInstructions::default()
        },
    };
    assert!(ResourceCatalog::from_snapshot(&snapshot).is_err());
}

#[derive(Default)]
struct DeduplicatingInput {
    accepted: Mutex<BTreeMap<(String, String), (String, u64)>>,
    calls: Mutex<Vec<KeyedCommandInput>>,
}

impl CommandInputAuthority for DeduplicatingInput {
    fn submit(
        &self,
        input: &KeyedCommandInput,
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        let identity = (input.session_id.clone(), input.key.clone());
        let mut accepted = self.accepted.lock().expect("accepted");
        if let Some((text, seq)) = accepted.get(&identity) {
            if text != &input.text {
                return Err(ResourceCapabilityError::InvalidRequest(
                    "idempotency conflict".to_owned(),
                ));
            }
            return Ok(MutationReceipt {
                seq: *seq,
                deduplicated: true,
            });
        }
        let seq = 9;
        accepted.insert(identity, (input.text.clone(), seq));
        self.calls.lock().expect("calls").push(input.clone());
        Ok(MutationReceipt {
            seq,
            deduplicated: false,
        })
    }
}

#[test]
fn slice11_gate_79_resource_capabilities_and_keyed_command_submission() {
    let input = Arc::new(DeduplicatingInput::default());
    let service = ClientResourceService::new(fixture_catalog(), ArcInput(Arc::clone(&input)));
    assert_eq!(
        ClientResourceService::<ArcInput>::capabilities(),
        [COMMANDS_LIST, COMMANDS_RUN, SKILLS_LIST]
            .into_iter()
            .map(str::to_owned)
            .collect()
    );
    assert_eq!(service.skills_list().format, 1);
    assert_eq!(service.commands_list().format, 1);
    let request: CommandRunRequest =
        serde_json::from_value(fixture_value("keyed-run.canonical.json")["request"].clone())
            .expect("run request");
    let mut missing_arguments = fixture_value("keyed-run.canonical.json")["request"].clone();
    missing_arguments
        .as_object_mut()
        .expect("request object")
        .remove("arguments");
    assert!(serde_json::from_value::<CommandRunRequest>(missing_arguments).is_err());
    let first = service
        .commands_run(&request, "uid:501")
        .expect("first run");
    let second = service
        .commands_run(&request, "uid:501")
        .expect("deduplicated run");
    assert_eq!(
        serde_json::to_value(first).expect("first result"),
        fixture_value("keyed-run.canonical.json")["result"]
    );
    assert!(second.deduplicated);
    assert_eq!(second.seq, 9);
    assert_eq!(input.calls.lock().expect("calls").len(), 1);
    assert_eq!(input.calls.lock().expect("calls")[0].principal, "uid:501");
    let mut conflict = request;
    conflict.arguments = "different".to_owned();
    assert!(service.commands_run(&conflict, "uid:501").is_err());
}

struct ArcInput(Arc<DeduplicatingInput>);

impl CommandInputAuthority for ArcInput {
    fn submit(
        &self,
        input: &KeyedCommandInput,
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        self.0.submit(input)
    }
}

#[derive(Default)]
struct RecordingDelivery {
    prompt: Mutex<Option<(String, OriginTuple, MaterializedPrompt, bool)>>,
    order: Mutex<Vec<&'static str>>,
}

struct BlockingDelivery {
    entered: SyncSender<()>,
    release: Mutex<Receiver<()>>,
}

impl SessionDeliveryAuthority for BlockingDelivery {
    fn prompt(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _prompt: &MaterializedPrompt,
        _steer: bool,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.entered.send(()).expect("publish command admission");
        self.release
            .lock()
            .expect("release receiver")
            .recv_timeout(Duration::from_secs(2))
            .expect("release blocked command");
        Ok(MutationReceipt {
            seq: 12,
            deduplicated: false,
        })
    }

    fn cancel(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        unreachable!("command capability never cancels")
    }

    fn rename(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _title: &str,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        unreachable!("command capability never renames")
    }
}

struct LeakyDelivery;

impl SessionDeliveryAuthority for LeakyDelivery {
    fn prompt(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _prompt: &MaterializedPrompt,
        _steer: bool,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        Err(ProductionRouteFailure::new(
            "internal",
            "spawn failed at /private/var/secret/runtime.sock",
            IJsonValue::parse_str("{}").expect("empty details"),
        ))
    }

    fn cancel(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        unreachable!("command capability never cancels")
    }

    fn rename(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _title: &str,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        unreachable!("command capability never renames")
    }
}

struct FixtureExtensionRoute;

struct FrozenOverlapRoute;

impl ProductionEndpointRoutes for FrozenOverlapRoute {
    fn capabilities(&self) -> std::collections::BTreeSet<String> {
        ["session.fork".to_owned()].into_iter().collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        (method == "session.fork").then_some(MethodClass::Mutation)
    }

    fn execute(
        &self,
        _request: &EndpointHostCall,
        _payload: &Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        unreachable!("frozen route overlap must fail before dispatch")
    }
}

impl ProductionEndpointRoutes for FixtureExtensionRoute {
    fn capabilities(&self) -> std::collections::BTreeSet<String> {
        ["fixture/status".to_owned()].into_iter().collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        (method == "fixture/status").then_some(MethodClass::ReadOnly)
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        assert_eq!(operation, "fixture/status");
        assert_eq!(payload, &json!({}));
        Ok(())
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        _payload: &Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        assert_eq!(request.operation, "fixture/status");
        Ok(IJsonValue::parse_str(r#"{"ok":true}"#).expect("fixture result"))
    }
}

impl SessionDeliveryAuthority for RecordingDelivery {
    fn prompt(
        &self,
        session_id: &str,
        _timestamp: &str,
        origin: &OriginTuple,
        prompt: &MaterializedPrompt,
        steer: bool,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.order.lock().expect("order").push("prompt");
        *self.prompt.lock().expect("prompt") =
            Some((session_id.to_owned(), origin.clone(), prompt.clone(), steer));
        Ok(MutationReceipt {
            seq: 11,
            deduplicated: false,
        })
    }

    fn cancel(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        unreachable!("command capability never cancels")
    }

    fn rename(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _title: &str,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        unreachable!("command capability never renames")
    }
}

#[test]
fn slice11_gate_80_production_input_seam_and_mux_authority() {
    assert_eq!(TEKES_UNARY_ROUTES.len(), 16);
    assert!(
        TEKES_UNARY_ROUTES
            .iter()
            .all(|route| ![SKILLS_LIST, COMMANDS_LIST, COMMANDS_RUN].contains(&route.name))
    );

    let delivery = Arc::new(RecordingDelivery::default());
    let delivery_root = tempfile::tempdir().expect("delivery root");
    fs::create_dir_all(delivery_root.path().join("threads").join(INPUT_SESSION_ID))
        .expect("active delivery session");
    let authority = EndpointCommandInputAuthority::new(
        Arc::new(ArcDelivery(Arc::clone(&delivery))),
        Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
        Arc::new(SessionInputAdmissionAuthority::new(
            delivery_root.path().to_path_buf(),
        )),
    );
    let input = KeyedCommandInput {
        principal: "uid:501".to_owned(),
        session_id: INPUT_SESSION_ID.to_owned(),
        key: "command-fixture-1".to_owned(),
        command: "review".to_owned(),
        content_digest: "a".repeat(64),
        text: "Review src/lib.rs".to_owned(),
    };
    authority.submit(&input).expect("production submit");
    let recorded = delivery.prompt.lock().expect("prompt");
    let (session, origin, prompt, steer) = recorded.as_ref().expect("recorded prompt");
    assert_eq!(session, &input.session_id);
    assert_eq!(origin.client, "tekes-client-resource");
    assert_eq!(origin.op, "session.prompt");
    assert_eq!(origin.key, input.key);
    assert!(!steer);
    assert_eq!(
        prompt.blocks,
        [schema::Block::Text {
            text: input.text.clone()
        }]
    );
    assert_eq!(delivery.order.lock().expect("order").as_slice(), ["prompt"]);

    let carrier_delivery = Arc::new(RecordingDelivery::default());
    let root = tempfile::tempdir().expect("carrier root");
    let carrier_input_root = tempfile::tempdir().expect("carrier input root");
    fs::create_dir_all(
        carrier_input_root
            .path()
            .join("threads")
            .join(INPUT_SESSION_ID),
    )
    .expect("active carrier session");
    let resource_routes: Arc<dyn ProductionEndpointRoutes> = Arc::new(ClientResourceService::new(
        fixture_catalog(),
        EndpointCommandInputAuthority::new(
            Arc::new(ArcDelivery(Arc::clone(&carrier_delivery))),
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Arc::new(SessionInputAdmissionAuthority::new(
                carrier_input_root.path().to_path_buf(),
            )),
        ),
    ));
    let fixture_routes: Arc<dyn ProductionEndpointRoutes> = Arc::new(FixtureExtensionRoute);
    let routes = CompositeProductionEndpointRoutes::compose([
        Arc::clone(&resource_routes),
        Arc::clone(&fixture_routes),
    ])
    .expect("composite routes");
    assert!(CompositeProductionEndpointRoutes::compose([resource_routes, fixture_routes]).is_ok());
    let duplicate: Arc<dyn ProductionEndpointRoutes> = Arc::new(FixtureExtensionRoute);
    assert!(
        CompositeProductionEndpointRoutes::compose([
            duplicate,
            Arc::new(FixtureExtensionRoute) as Arc<dyn ProductionEndpointRoutes>,
        ])
        .is_err()
    );
    assert!(
        CompositeProductionEndpointRoutes::compose([
            Arc::new(FrozenOverlapRoute) as Arc<dyn ProductionEndpointRoutes>
        ])
        .is_err(),
        "an extension may never claim a Session Endpoint V3 route"
    );
    let description = SessionHostDescription {
        version: "0.1.0".to_owned(),
        cwd: "/workspace".to_owned(),
        provider: None,
        model: None,
        attached_sessions: 0,
        home: "/Users/example".to_owned(),
        can_open_path: false,
    };
    assert!(
        ProductionEndpointAssembly::open_with_routes(
            root.path(),
            description.clone(),
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Arc::new(FrozenOverlapRoute),
        )
        .is_err(),
        "ordinary extension assembly must reject a Session Endpoint V3 route"
    );
    let assembly = ProductionEndpointAssembly::open_with_routes(
        root.path(),
        description,
        Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
        Arc::new(routes),
    )
    .expect("resource carrier assembly");
    for method in [SKILLS_LIST, COMMANDS_LIST, COMMANDS_RUN] {
        assert!(assembly.host().capabilities().contains(method));
    }
    assert!(assembly.host().capabilities().contains("fixture/status"));
    for (rpc_id, method) in [
        ("rpc-skills-list", SKILLS_LIST),
        ("rpc-commands-list", COMMANDS_LIST),
        ("rpc-fixture-status", "fixture/status"),
    ] {
        let response = block_on_ready(assembly.dispatch(ClientRequest {
            envelope_type: "client-request".to_owned(),
            rpc_id: rpc_id.to_owned(),
            method: method.to_owned(),
            payload: IJsonValue::parse_str("{}").expect("empty payload"),
        }))
        .expect("read-only extension dispatch");
        assert!(response.result.ok, "{method} must be executable");
    }
    let run = block_on_ready(
        assembly.dispatch(ClientRequest {
            envelope_type: "client-request".to_owned(),
            rpc_id: "rpc-command-1".to_owned(),
            method: COMMANDS_RUN.to_owned(),
            payload: IJsonValue::parse(
                &serde_json::to_vec(&fixture_value("keyed-run.canonical.json")["request"])
                    .expect("run JSON"),
            )
            .expect("run I-JSON"),
        }),
    )
    .expect("resource carrier dispatch");
    assert!(run.result.ok);
    let missing = block_on_ready(
        assembly.dispatch(ClientRequest {
            envelope_type: "client-request".to_owned(),
            rpc_id: "rpc-command-missing".to_owned(),
            method: COMMANDS_RUN.to_owned(),
            payload: IJsonValue::parse_str(&format!(
                r#"{{"arguments":"","key":"missing-1","name":"missing","session_id":"{}"}}"#,
                input.session_id
            ))
            .expect("missing command payload"),
        }),
    )
    .expect("typed missing command response");
    assert_eq!(
        missing.result.error.expect("missing command error").code,
        "command-not-found"
    );
    let missing_arguments = block_on_ready(
        assembly.dispatch(ClientRequest {
            envelope_type: "client-request".to_owned(),
            rpc_id: "rpc-command-missing-arguments".to_owned(),
            method: COMMANDS_RUN.to_owned(),
            payload: IJsonValue::parse_str(&format!(
                r#"{{"key":"missing-arguments-1","name":"review","session_id":"{}"}}"#,
                input.session_id
            ))
            .expect("missing arguments payload"),
        }),
    )
    .expect("typed missing arguments response");
    assert_eq!(
        missing_arguments
            .result
            .error
            .expect("missing arguments error")
            .code,
        "bad-request"
    );
    assert_eq!(
        carrier_delivery.order.lock().expect("order").as_slice(),
        ["prompt"]
    );

    let race_root = tempfile::tempdir().expect("archive race root");
    bootstrap_input_session(race_root.path());
    let race_admission = Arc::new(SessionInputAdmissionAuthority::new(
        race_root.path().to_path_buf(),
    ));
    let (entered_tx, entered_rx) = mpsc::sync_channel(1);
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let blocking_delivery: Arc<dyn SessionDeliveryAuthority> = Arc::new(BlockingDelivery {
        entered: entered_tx,
        release: Mutex::new(release_rx),
    });
    let race_routes: Arc<dyn ProductionEndpointRoutes> = Arc::new(ClientResourceService::new(
        fixture_catalog(),
        EndpointCommandInputAuthority::new(
            blocking_delivery,
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Arc::clone(&race_admission),
        ),
    ));
    let race_host = Arc::new(
        ProductionEndpointHost::open_with_full_authorities_and_session_admission(
            race_root.path(),
            SessionHostDescription {
                version: "0.1.0".to_owned(),
                cwd: "/workspace".to_owned(),
                provider: None,
                model: None,
                attached_sessions: 0,
                home: "/Users/example".to_owned(),
                can_open_path: false,
            },
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Some(race_routes),
            None,
            None,
            None,
            race_admission,
        )
        .expect("shared command/archive host"),
    );
    let command_host = Arc::clone(&race_host);
    let command = thread::spawn(move || {
        let result = block_on_ready(
            command_host.call(EndpointHostCall {
                rpc_id: "rpc-command-archive-race".to_owned(),
                operation: COMMANDS_RUN.to_owned(),
                payload: IJsonValue::parse(
                    &serde_json::to_vec(&fixture_value("keyed-run.canonical.json")["request"])
                        .expect("run JSON"),
                )
                .expect("run I-JSON"),
                recovering: false,
                handoff: DurableHandoffSignal::new(),
            }),
        );
        assert!(
            result.error.is_none(),
            "command must complete before archive"
        );
    });
    entered_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("command entered the shared admission gate");
    let archive_host = Arc::clone(&race_host);
    let (archive_done_tx, archive_done_rx) = mpsc::sync_channel(1);
    let archive = thread::spawn(move || {
        let result = block_on_ready(
            archive_host.call(EndpointHostCall {
                rpc_id: "rpc-archive-command-race".to_owned(),
                operation: "workspace.archiveSession".to_owned(),
                payload: IJsonValue::parse_str(&format!(r#"{{"sessionId":"{INPUT_SESSION_ID}"}}"#))
                    .expect("archive payload"),
                recovering: false,
                handoff: DurableHandoffSignal::new(),
            }),
        );
        archive_done_tx
            .send(result.error.map(|error| error.code))
            .expect("publish archive result");
    });
    assert!(
        matches!(
            archive_done_rx.recv_timeout(Duration::from_millis(100)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ),
        "archive must wait while commands/run owns the per-session admission gate"
    );
    release_tx.send(()).expect("release command delivery");
    command.join().expect("command thread");
    assert_eq!(
        archive_done_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("archive completes after command"),
        None
    );
    archive.join().expect("archive thread");
    assert!(
        race_root
            .path()
            .join("archive")
            .join(INPUT_SESSION_ID)
            .is_dir(),
        "archive must move the session only after command delivery completes"
    );

    let leak_root = tempfile::tempdir().expect("leak redaction root");
    bootstrap_input_session(leak_root.path());
    let leak_routes: Arc<dyn ProductionEndpointRoutes> = Arc::new(ClientResourceService::new(
        fixture_catalog(),
        EndpointCommandInputAuthority::new(
            Arc::new(LeakyDelivery),
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Arc::new(SessionInputAdmissionAuthority::new(
                leak_root.path().to_path_buf(),
            )),
        ),
    ));
    let leak_assembly = ProductionEndpointAssembly::open_with_routes(
        leak_root.path(),
        SessionHostDescription {
            version: "0.1.0".to_owned(),
            cwd: "/workspace".to_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: "/Users/example".to_owned(),
            can_open_path: false,
        },
        Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
        leak_routes,
    )
    .expect("leak redaction assembly");
    let leak = block_on_ready(
        leak_assembly.dispatch(ClientRequest {
            envelope_type: "client-request".to_owned(),
            rpc_id: "rpc-command-leak".to_owned(),
            method: COMMANDS_RUN.to_owned(),
            payload: IJsonValue::parse(
                &serde_json::to_vec(&fixture_value("keyed-run.canonical.json")["request"])
                    .expect("run JSON"),
            )
            .expect("run I-JSON"),
        }),
    )
    .expect("typed leak response");
    let leak = leak.result.error.expect("redacted command failure");
    assert_eq!(leak.code, "internal");
    assert_eq!(leak.message, "Endpoint operation failed");
    assert_eq!(
        leak.details,
        IJsonValue::parse_str("{}").expect("empty details")
    );

    let production_root = tempfile::tempdir().expect("production daemon root");
    let migration = FixtureRoot::discover()
        .expect("fixtures")
        .join("resources/migration");
    let project = migration
        .join("project")
        .canonicalize()
        .expect("canonical project root");
    ConfigRepository::open(production_root.path())
        .expect("production config repository")
        .publish_workspace(
            0,
            &WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "resource-workspace".to_owned(),
                name: "Resource workspace".to_owned(),
                cwd: vec![project.to_string_lossy().into_owned()],
                folders: Vec::new(),
                policy: None,
            },
        )
        .expect("publish resource workspace");
    let process_host = ProductionProcessHost::open(
        production_root.path(),
        std::env::current_exe().expect("test binary"),
        "slice11-test",
        migration.join("user"),
    )
    .expect("production process host");
    let production = assemble_application_endpoint_host(
        production_root.path(),
        SessionHostDescription {
            version: "0.1.0".to_owned(),
            cwd: project.to_string_lossy().into_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: production_root.path().to_string_lossy().into_owned(),
            can_open_path: false,
        },
        Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
        &migration.join("user"),
        Arc::clone(&process_host),
    )
    .expect("production daemon endpoint host");
    for method in [SKILLS_LIST, COMMANDS_LIST, COMMANDS_RUN] {
        assert!(production.capabilities().contains(method));
    }
    for method in [
        "resources/list",
        "resources/read",
        "tools/list",
        "tools/resolve",
        "schedule.list",
        "schedule.save",
        "schedule.delete",
        "schedule.runNow",
        "thread.search",
        "usage.summary",
        "usage.cacheAttribution",
        "mcp.list",
        "mcp.get",
        "mcp.save",
        "mcp.remove",
        "mcp.probe",
        "mcp.oauth.start",
        "mcp.oauth.remove",
    ] {
        assert!(
            production.capabilities().contains(method),
            "missing production route {method}"
        );
    }
    assert!(
        !production
            .capabilities()
            .iter()
            .any(|method| method.starts_with("plugin/")),
        "an opaque development build must not advertise the unavailable plugin group"
    );
    for (rpc_id, operation) in [
        ("production-skills", SKILLS_LIST),
        ("production-commands", COMMANDS_LIST),
    ] {
        let result = block_on_ready(production.call(EndpointHostCall {
            rpc_id: rpc_id.to_owned(),
            operation: operation.to_owned(),
            payload: IJsonValue::parse_str("{}").expect("empty payload"),
            recovering: false,
            handoff: DurableHandoffSignal::new(),
        }));
        assert!(result.ok, "{operation} must execute in production assembly");
    }
    let result = block_on_ready(production.call(EndpointHostCall {
        rpc_id: "production-command-run".to_owned(),
        operation: COMMANDS_RUN.to_owned(),
        payload: IJsonValue::parse_str(
            r#"{"arguments":"","key":"production-command-1","name":"missing","session_id":"018f0000-0000-7000-8000-000000000011"}"#,
        )
        .expect("production command payload"),
        recovering: false,
        handoff: DurableHandoffSignal::new(),
    }));
    assert_eq!(
        result.error.expect("production command failure").code,
        "session-not-found",
        "commands/run must validate session admission before resolving its resource catalog"
    );
    process_host.shutdown();

    let sources = fixture_value("sources.canonical.json");
    assert_eq!(sources["format"], 1);
    assert_eq!(sources["sources"].as_array().expect("sources").len(), 2);
    for source in sources["sources"].as_array().expect("sources") {
        assert_eq!(source["revision"].as_str().expect("revision").len(), 40);
        assert!(!source["evidence"].as_array().expect("evidence").is_empty());
    }

    let transport = fs::read_to_string(
        FixtureRoot::discover()
            .expect("fixtures")
            .path()
            .parent()
            .expect("root")
            .join("spec/session-endpoint.md"),
    )
    .expect("transport spec");
    assert!(transport.contains("exactly the closed registration"));
}

struct ArcDelivery(Arc<RecordingDelivery>);

impl SessionDeliveryAuthority for ArcDelivery {
    fn prompt(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        prompt: &MaterializedPrompt,
        steer: bool,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.0.prompt(session_id, timestamp, origin, prompt, steer)
    }

    fn cancel(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.0.cancel(session_id, timestamp, origin)
    }

    fn rename(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.0.rename(session_id, timestamp, origin, title)
    }
}
