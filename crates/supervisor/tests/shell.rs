use std::fs;

use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use profile::{
    DynamicTool, DynamicToolCatalog, DynamicToolEffect, DynamicToolSource, DynamicToolSourceKind,
    ExternalEffectBinding, ExternalEffectProtocol,
};
use schema::IJsonValue;
use serde_json::{Value, json};
use tekes_supervisor::production_tool_control::{
    ChildLaunchProof, DeliveryRequest, DynamicSupervisorAuthority, InterruptRequest,
    ParentReportProof, ProductionToolControlHandler, ProductionToolControlPolicy,
    SupervisorJobAuthority, SupervisorOperationError, SupervisorRuntimeAuthority,
};
use tekes_supervisor::tool_control::{
    ExternalEffectResolution, ToolControlReceiptStore, ToolControlSession,
};
use worker_control::Selected;
use worker_control::{
    ToolControl, ToolControlErrorCode, ToolControlResult, decode_tool_control_result,
    encode_tool_control,
};

const SESSION: &str = "018f0000-0000-7000-8000-000000000001";
const TS: &str = "2026-08-26T09:00:00.000Z";

#[test]
fn supervisor_describe_build_binds_embedded_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_tekes-supervisor"))
        .arg("--describe-build")
        .output()
        .expect("describe supervisor build");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let embedded_build = option_env!("TEKES_SELECTED_BUILD").unwrap_or(env!("CARGO_PKG_VERSION"));
    let mut expected = json!({
        "format": 1,
        "identifier": "com.tekes.kernel.supervisor",
        "version": embedded_build,
    });
    if let Some(revision) = tekes_supervisor::host_runtime::SOURCE_REVISION {
        expected["source_revision"] = revision.into();
    }
    let mut expected = serde_json_canonicalizer::to_vec(&expected).expect("canonical build");
    expected.push(b'\n');
    assert_eq!(output.stdout, expected);
}

#[test]
fn slice8_gate_62_supervisor_tool_control_dedup() {
    let directory = tempfile::tempdir().expect("tempdir");
    let request = write_bound_request(
        directory.path(),
        "call-context",
        "context",
        json!({"operation":"threads","reason":"fixture audit"}),
    );
    let line = encode_tool_control(&request).expect("request line");
    let ledger = directory
        .path()
        .join("threads")
        .join(SESSION)
        .join("main.jsonl");
    let ledger_before = fs::read(&ledger).expect("ledger before control");

    let mut first = production_session(directory.path());
    let first_response = first.handle_line(&line).expect("first response");
    let first_result = decode_tool_control_result(&first_response).expect("first result");
    assert!(
        first_result.error.is_none(),
        "context request failed: {:?}",
        first_result.error
    );
    let value: Value = serde_json::from_value(
        serde_json::to_value(first_result.value.expect("context value")).expect("JSON value"),
    )
    .expect("plain JSON");
    assert_eq!(
        value["threads"].as_array().expect("thread inventory").len(),
        1
    );

    let mut restarted = production_session(directory.path());
    assert_eq!(
        restarted.handle_line(&line).expect("replayed response"),
        first_response,
        "restart must re-ack the exact policy-filtered receipt"
    );

    let mut conflicting = request.clone();
    conflicting.name = "report".to_owned();
    let conflict = restarted
        .handle_line(&encode_tool_control(&conflicting).expect("conflict line"))
        .expect("typed conflict");
    assert_eq!(
        decode_tool_control_result(&conflict)
            .expect("conflict result")
            .error
            .expect("conflict error")
            .code,
        ToolControlErrorCode::Conflict
    );
    assert_eq!(
        fs::read(&ledger).expect("ledger after control"),
        ledger_before,
        "the supervisor must not author tool_result or any semantic fact"
    );

    assert_proof_required(
        "task",
        json!({"task_name":"audit","goal":"find defects","instructions":"inspect inputs","input_sources":[],"output":{"mode":"inline","format":"text","name":"result","path":"","contract":"findings"}}),
        ToolControlErrorCode::Conflict,
    );
    assert_proof_required(
        "subagent",
        json!({"brief":"audit the files","report_back_session_id":""}),
        ToolControlErrorCode::Conflict,
    );
    assert_proof_required(
        "report",
        json!({"result":"ok"}),
        ToolControlErrorCode::Denied,
    );
}

#[test]
fn dynamic_supervisor_route_is_catalog_bound_receipted_and_omitted_without_authority() {
    let directory = tempfile::tempdir().expect("tempdir");
    let request = write_bound_request(
        directory.path(),
        "call-dynamic",
        "plugin.frame",
        json!({"quality":7}),
    );
    let line = encode_tool_control(&request).expect("request line");
    let calls = Arc::new(AtomicUsize::new(0));
    let route: Arc<dyn DynamicSupervisorAuthority> = Arc::new(FakeDynamicRoute {
        calls: Arc::clone(&calls),
        reconciliations: Arc::new(AtomicUsize::new(0)),
        reconcile_status: FakeReconcileStatus::Confirmed,
        execute_unknown: false,
        execute_conflicted: false,
    });
    let catalog = dynamic_catalog();
    let handler =
        ProductionToolControlHandler::new(directory.path(), NoRuntimeAuthority, NoJobAuthority)
            .with_dynamic_routes(catalog.clone(), Arc::clone(&route));
    let mut first = ToolControlSession::new(
        directory.path(),
        Selected { version: 2 },
        handler,
        ProductionToolControlPolicy::default(),
    );
    let first_bytes = first.handle_line(&line).expect("dynamic response");
    let first_result = decode_tool_control_result(&first_bytes).expect("dynamic result");
    assert_eq!(
        serde_json::to_value(first_result.value.expect("dynamic value")).unwrap(),
        json!({"source":"com.tekes.camera","value":"ready"})
    );
    assert_eq!(calls.load(Ordering::Acquire), 1);

    let handler =
        ProductionToolControlHandler::new(directory.path(), NoRuntimeAuthority, NoJobAuthority)
            .with_dynamic_routes(catalog, route);
    let mut restarted = ToolControlSession::new(
        directory.path(),
        Selected { version: 2 },
        handler,
        ProductionToolControlPolicy::default(),
    );
    assert_eq!(restarted.handle_line(&line).unwrap(), first_bytes);
    assert_eq!(
        calls.load(Ordering::Acquire),
        1,
        "durable receipt must suppress a repeated dynamic effect"
    );

    let missing = tempfile::tempdir().expect("missing route tempdir");
    let missing_request = write_bound_request(
        missing.path(),
        "call-dynamic",
        "plugin.frame",
        json!({"quality":7}),
    );
    let result = production_session(missing.path())
        .handle_line(&encode_tool_control(&missing_request).unwrap())
        .expect("typed unavailable dynamic result");
    assert_eq!(
        decode_tool_control_result(&result)
            .unwrap()
            .error
            .unwrap()
            .code,
        ToolControlErrorCode::Unsupported
    );
}

#[test]
fn external_effect_intent_reconciles_closed_states_without_blind_reexecution() {
    for (status, expected_code, expected_calls) in [
        (FakeReconcileStatus::Confirmed, None, 0),
        (FakeReconcileStatus::NotFound, None, 1),
        (
            FakeReconcileStatus::Unknown,
            Some(ToolControlErrorCode::EffectUnknown),
            0,
        ),
        (
            FakeReconcileStatus::Conflicted,
            Some(ToolControlErrorCode::EffectConflicted),
            0,
        ),
    ] {
        let directory = tempfile::tempdir().expect("tempdir");
        let request = write_bound_request(
            directory.path(),
            "call-external-recovery",
            "plugin.frame",
            json!({"quality":7}),
        );
        publish_interrupted_intent(directory.path(), &request);
        let calls = Arc::new(AtomicUsize::new(0));
        let reconciliations = Arc::new(AtomicUsize::new(0));
        let route: Arc<dyn DynamicSupervisorAuthority> = Arc::new(FakeDynamicRoute {
            calls: Arc::clone(&calls),
            reconciliations: Arc::clone(&reconciliations),
            reconcile_status: status,
            execute_unknown: false,
            execute_conflicted: false,
        });
        let handler =
            ProductionToolControlHandler::new(directory.path(), NoRuntimeAuthority, NoJobAuthority)
                .with_dynamic_routes(dynamic_catalog(), route);
        let mut session = ToolControlSession::new(
            directory.path(),
            Selected { version: 2 },
            handler,
            ProductionToolControlPolicy::default(),
        );
        let response = session
            .handle_line(&encode_tool_control(&request).expect("control line"))
            .expect("closed recovery result");
        let result = decode_tool_control_result(&response).expect("control result");
        assert_eq!(result.error.as_ref().map(|error| error.code), expected_code);
        assert_eq!(calls.load(Ordering::Acquire), expected_calls);
        assert_eq!(reconciliations.load(Ordering::Acquire), 1);
        let settled =
            ToolControlReceiptStore::new(directory.path().join("threads").join(SESSION), SESSION)
                .read_record(&request.request_id)
                .expect("control record")
                .is_some_and(|record| record.response.is_some());
        assert_eq!(
            settled,
            expected_code.is_none(),
            "only confirmed or authoritative not_found-then-executed outcomes settle a receipt"
        );
    }
}

#[test]
fn not_found_retry_authority_drift_stays_unresolved_without_a_receipt() {
    let directory = tempfile::tempdir().expect("tempdir");
    let request = write_bound_request(
        directory.path(),
        "call-external-drift",
        "plugin.frame",
        json!({"quality":7}),
    );
    publish_interrupted_intent(directory.path(), &request);
    let calls = Arc::new(AtomicUsize::new(0));
    let reconciliations = Arc::new(AtomicUsize::new(0));
    let route: Arc<dyn DynamicSupervisorAuthority> = Arc::new(FakeDynamicRoute {
        calls: Arc::clone(&calls),
        reconciliations: Arc::clone(&reconciliations),
        reconcile_status: FakeReconcileStatus::NotFound,
        execute_unknown: false,
        execute_conflicted: true,
    });
    let handler =
        ProductionToolControlHandler::new(directory.path(), NoRuntimeAuthority, NoJobAuthority)
            .with_dynamic_routes(dynamic_catalog(), route);
    let mut session = ToolControlSession::new(
        directory.path(),
        Selected { version: 2 },
        handler,
        ProductionToolControlPolicy::default(),
    );

    let response = session
        .handle_line(&encode_tool_control(&request).expect("control line"))
        .expect("unresolved recovery result");
    let result = decode_tool_control_result(&response).expect("control result");
    assert_eq!(
        result.error.as_ref().map(|error| error.code),
        Some(ToolControlErrorCode::EffectConflicted)
    );
    assert_eq!(calls.load(Ordering::Acquire), 1);
    assert_eq!(reconciliations.load(Ordering::Acquire), 1);
    let store =
        ToolControlReceiptStore::new(directory.path().join("threads").join(SESSION), SESSION);
    let record = store
        .read_record(&request.request_id)
        .expect("control record")
        .expect("the intent is retained");
    assert!(
        record.response.is_none(),
        "an unresolved effect never settles a receipt"
    );
}

#[test]
fn effectful_dynamic_tool_without_reconciliation_fails_before_authority() {
    let directory = tempfile::tempdir().expect("tempdir");
    let request = write_bound_request(
        directory.path(),
        "call-unqueryable",
        "plugin.frame",
        json!({"quality":7}),
    );
    let calls = Arc::new(AtomicUsize::new(0));
    let route: Arc<dyn DynamicSupervisorAuthority> = Arc::new(FakeDynamicRoute {
        calls: Arc::clone(&calls),
        reconciliations: Arc::new(AtomicUsize::new(0)),
        reconcile_status: FakeReconcileStatus::Confirmed,
        execute_unknown: false,
        execute_conflicted: false,
    });
    let mut catalog = dynamic_catalog();
    catalog.tools[0].external_effect = None;
    let handler =
        ProductionToolControlHandler::new(directory.path(), NoRuntimeAuthority, NoJobAuthority)
            .with_dynamic_routes(catalog, route);
    let mut session = ToolControlSession::new(
        directory.path(),
        Selected { version: 2 },
        handler,
        ProductionToolControlPolicy::default(),
    );
    let response = session
        .handle_line(&encode_tool_control(&request).expect("control line"))
        .expect("fail-closed result");
    assert_eq!(
        decode_tool_control_result(&response)
            .expect("control result")
            .error
            .expect("unqueryable effect error")
            .code,
        ToolControlErrorCode::EffectUnknown
    );
    assert_eq!(calls.load(Ordering::Acquire), 0);
}

#[test]
fn ambiguous_effect_is_reconciled_instead_of_treated_as_timeout_failure() {
    let directory = tempfile::tempdir().expect("tempdir");
    let request = write_bound_request(
        directory.path(),
        "call-ambiguous",
        "plugin.frame",
        json!({"quality":7}),
    );
    let calls = Arc::new(AtomicUsize::new(0));
    let reconciliations = Arc::new(AtomicUsize::new(0));
    let route: Arc<dyn DynamicSupervisorAuthority> = Arc::new(FakeDynamicRoute {
        calls: Arc::clone(&calls),
        reconciliations: Arc::clone(&reconciliations),
        reconcile_status: FakeReconcileStatus::Confirmed,
        execute_unknown: true,
        execute_conflicted: false,
    });
    let handler =
        ProductionToolControlHandler::new(directory.path(), NoRuntimeAuthority, NoJobAuthority)
            .with_dynamic_routes(dynamic_catalog(), route);
    let mut session = ToolControlSession::new(
        directory.path(),
        Selected { version: 2 },
        handler,
        ProductionToolControlPolicy::default(),
    );
    let response = session
        .handle_line(&encode_tool_control(&request).expect("control line"))
        .expect("reconciled result");
    assert!(
        decode_tool_control_result(&response)
            .expect("control result")
            .error
            .is_none()
    );
    assert_eq!(calls.load(Ordering::Acquire), 1);
    assert_eq!(reconciliations.load(Ordering::Acquire), 1);
}

fn production_session(
    root: &Path,
) -> ToolControlSession<
    ProductionToolControlHandler<NoRuntimeAuthority, NoJobAuthority>,
    ProductionToolControlPolicy,
> {
    ToolControlSession::new(
        root,
        Selected { version: 2 },
        ProductionToolControlHandler::new(root, NoRuntimeAuthority, NoJobAuthority),
        ProductionToolControlPolicy::default(),
    )
}

fn assert_proof_required(name: &str, arguments: Value, expected: ToolControlErrorCode) {
    let directory = tempfile::tempdir().expect("tempdir");
    let request = write_bound_request(directory.path(), "call-proof", name, arguments);
    let response = production_session(directory.path())
        .handle_line(&encode_tool_control(&request).expect("request line"))
        .expect("typed result");
    let error = decode_tool_control_result(&response)
        .expect("control result")
        .error
        .expect("missing durable proof must fail");
    assert_eq!(
        error.code, expected,
        "{name} failed before the durable-proof check: {error:?}"
    );
}

fn write_bound_request(root: &Path, call_id: &str, name: &str, arguments: Value) -> ToolControl {
    let thread_folder = root.join("threads").join(SESSION);
    fs::create_dir_all(thread_folder.join("assets")).expect("thread folder");
    fs::create_dir_all(root.join("archive")).expect("archive folder");
    let events = [
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":SESSION,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":SESSION,"op":"create","key":"create"}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":[{"type":"text","text":"start"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":SESSION,"op":"submit","key":"i1"}}),
        json!({"v":1,"seq":3,"kind":"run_start","ts":TS,"run":"r1","mode":"ordinary","recovery_ordinal":0,"binary":"worker","config_digest":"cfg","instruction_digest":"ins","policy":"default"}),
        json!({"v":1,"seq":4,"turn":1,"kind":"turn_open","ts":TS,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":5,"kind":"epoch","ts":TS,"id":"e1","reason":"initial","adapter":"fake","model":"fake","system":{"asset":"sha256-aa","digest":"d"},"tools":{"asset":"sha256-td","digest":"td"},"renderer":1}),
        json!({"v":1,"seq":6,"turn":1,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":TS,"attempt":"a1","epoch":"e1","wire_digest":"wd","admits":[{"from":2,"to":2}]}),
        json!({"v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":call_id,"name":name,"args":arguments,"source":"provider"}),
    ];
    let mut bytes = Vec::new();
    for event in events {
        bytes.extend(serde_json_canonicalizer::to_vec(&event).expect("canonical event"));
        bytes.push(b'\n');
    }
    schema::validate_ledger(&bytes, 1).expect("valid bound ledger");
    fs::write(thread_folder.join("main.jsonl"), bytes).expect("ledger");
    ToolControl::new(
        SESSION,
        SESSION,
        1,
        call_id,
        name,
        IJsonValue::parse(&serde_json::to_vec(&arguments).expect("arguments JSON"))
            .expect("I-JSON arguments"),
    )
    .expect("tool-control request")
}

struct NoRuntimeAuthority;

struct FakeDynamicRoute {
    calls: Arc<AtomicUsize>,
    reconciliations: Arc<AtomicUsize>,
    reconcile_status: FakeReconcileStatus,
    execute_unknown: bool,
    execute_conflicted: bool,
}

#[derive(Clone, Copy)]
enum FakeReconcileStatus {
    Confirmed,
    NotFound,
    Unknown,
    Conflicted,
}

impl DynamicSupervisorAuthority for FakeDynamicRoute {
    fn supports(&self, tool: &DynamicTool) -> bool {
        tool.source.id == "com.tekes.camera"
    }

    fn execute(
        &self,
        tool: &DynamicTool,
        _request: &ToolControl,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        self.calls.fetch_add(1, Ordering::AcqRel);
        if self.execute_unknown {
            return Err(SupervisorOperationError::AmbiguousEffect(
                "response was lost after dispatch".to_owned(),
            ));
        }
        if self.execute_conflicted {
            return Err(SupervisorOperationError::EffectConflicted(
                "authority contract changed before retry dispatch".to_owned(),
            ));
        }
        Ok(ijson(json!({"source":tool.source.id,"value":"ready"})))
    }

    fn reconcile(&self, tool: &DynamicTool, request: &ToolControl) -> ExternalEffectResolution {
        self.reconciliations.fetch_add(1, Ordering::AcqRel);
        match self.reconcile_status {
            FakeReconcileStatus::Confirmed => {
                ExternalEffectResolution::Confirmed(ToolControlResult::success(
                    request.request_id.clone(),
                    request.call_id.clone(),
                    ijson(json!({"source":tool.source.id,"value":"reconciled"})),
                ))
            }
            FakeReconcileStatus::NotFound => ExternalEffectResolution::NotFound,
            FakeReconcileStatus::Unknown => ExternalEffectResolution::Unknown {
                message: "external authority is temporarily unavailable".to_owned(),
            },
            FakeReconcileStatus::Conflicted => ExternalEffectResolution::Conflicted {
                message: "idempotency key maps to incompatible business inputs".to_owned(),
            },
        }
    }
}

fn dynamic_catalog() -> DynamicToolCatalog {
    let mut tool = DynamicTool::declared(
        "plugin.frame",
        DynamicToolSource {
            kind: DynamicToolSourceKind::Plugin,
            id: "com.tekes.camera".to_owned(),
        },
        DynamicToolEffect::ComputerControl,
        true,
        Vec::new(),
        ijson(json!({
            "name":"plugin.frame",
            "description":"Capture one frame",
            "parameters":{
                "type":"object",
                "properties":{"quality":{"type":"integer","minimum":1,"maximum":10}},
                "required":["quality"],
                "additionalProperties":false
            }
        })),
    )
    .expect("dynamic tool");
    tool.external_effect = Some(ExternalEffectBinding {
        protocol: ExternalEffectProtocol::IdempotencyReconcileV1,
        reconcile_tool: "plugin.frame.get_by_idempotency_key".to_owned(),
    });
    DynamicToolCatalog::resolve(vec![tool]).expect("dynamic catalog")
}

fn publish_interrupted_intent(root: &Path, request: &ToolControl) {
    let store = ToolControlReceiptStore::new(root.join("threads").join(SESSION), SESSION);
    let path = store.record_path(&request.request_id);
    fs::create_dir_all(path.parent().expect("intent parent")).expect("intent parent");
    let bytes = serde_json_canonicalizer::to_vec(&json!({
        "format":1,
        "request":request,
    }))
    .expect("canonical intent");
    fs::write(path, bytes).expect("interrupted durable intent");
}

impl SupervisorRuntimeAuthority for NoRuntimeAuthority {
    fn ensure_running(
        &mut self,
        _session: &str,
        _request_id: &str,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("context threads must not invoke runtime authority")
    }

    fn deliver_input(
        &mut self,
        _request: &DeliveryRequest,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("test request must not deliver input")
    }

    fn interrupt(
        &mut self,
        _request: &InterruptRequest,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("test request must not interrupt")
    }

    fn ensure_child(
        &mut self,
        _proof: &ChildLaunchProof,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("missing durable spawn proof must fail before runtime authority")
    }

    fn deliver_report(
        &mut self,
        _proof: &ParentReportProof,
        _result: &IJsonValue,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("missing durable parent proof must fail before runtime authority")
    }
}

struct NoJobAuthority;

impl SupervisorJobAuthority for NoJobAuthority {
    fn execute(&mut self, _request: &ToolControl) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("test request must not invoke job authority")
    }
}

fn ijson(value: Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(&value).expect("JSON bytes")).expect("I-JSON")
}
