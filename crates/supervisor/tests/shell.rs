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
    schema::validate_ledger(&bytes, schema::READER_VERSION).expect("valid bound ledger");
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

const FOREIGN: &str = "018f0000-0000-7000-8000-000000000099";
const FOREIGN_TWO: &str = "018f0000-0000-7000-8000-000000000098";

fn foreign_session(root: &Path, id: &str) {
    let folder = root.join("threads").join(id);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let events = [
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":id,"workspace":"other-ws","environment":"local","folders":["other-folder"],"config":{"digest":"cfg"},"resume":"never","min_reader":3,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":id,"op":"create","key":"create"}}),
        json!({"v":1,"seq":2,"kind":"meta","ts":TS,"title":"foreign first"}),
        json!({"v":1,"seq":3,"kind":"meta","ts":TS,"title":"foreign second"}),
    ];
    let bytes = events
        .iter()
        .flat_map(|event| {
            let mut row = serde_json_canonicalizer::to_vec(event).unwrap();
            row.push(b'\n');
            row
        })
        .collect::<Vec<_>>();
    schema::validate_ledger(&bytes, schema::READER_VERSION).unwrap();
    fs::write(folder.join("main.jsonl"), bytes).unwrap();
}

fn visibility_call(root: &Path, id: &str, name: &str, arguments: Value) -> ToolControlResult {
    let request = write_bound_request(root, id, name, arguments);
    let response = production_session(root)
        .handle_line(&encode_tool_control(&request).unwrap())
        .unwrap();
    decode_tool_control_result(&response).unwrap()
}

fn visibility_value(result: ToolControlResult) -> Value {
    assert!(result.error.is_none(), "{:?}", result.error);
    serde_json::to_value(result.value.unwrap()).unwrap()
}

#[test]
fn context_reads_messages_and_controls_are_visible_across_workspace_and_folder_ids() {
    let root = tempfile::tempdir().unwrap();
    foreign_session(root.path(), FOREIGN);
    foreign_session(root.path(), FOREIGN_TWO);
    let folder = root.path().join("threads").join(FOREIGN);
    engine::write_permission_mode(&folder, engine::PermissionMode::ReadOnly).unwrap();
    let original_ledger = fs::read(folder.join("main.jsonl")).unwrap();
    let original_mode = fs::read(engine::permission_mode_path(&folder)).unwrap();
    let threads = visibility_value(visibility_call(
        root.path(),
        "threads-global",
        "context",
        json!({"operation":"threads","reason":"discover"}),
    ));
    assert_eq!(threads["threads"].as_array().unwrap().len(), 3);
    let read = visibility_value(visibility_call(
        root.path(),
        "read-global",
        "context",
        json!({"operation":"read","thread_id":FOREIGN,"reason":"inspect"}),
    ));
    assert_eq!(
        read["workspace"], "other-ws",
        "metadata belongs to the target, not the caller"
    );
    let records = visibility_value(visibility_call(
        root.path(),
        "get-global",
        "context_get",
        json!({"thread_id":FOREIGN,"record_ids":[2],"reason":"inspect"}),
    ));
    assert_eq!(records["records"][0]["event"]["title"], "foreign first");
    let project = visibility_value(visibility_call(
        root.path(),
        "project-filter",
        "context",
        json!({"operation":"search","query":"foreign","scope":"project","reason":"inspect"}),
    ));
    assert!(
        project["hits"]
            .as_array()
            .unwrap()
            .iter()
            .all(|hit| hit["session_id"] == SESSION),
        "legacy project is a query filter"
    );
    for (id, selector) in [
        ("default-filter", None),
        ("current-filter", Some("current_thread")),
    ] {
        let mut query = json!({"operation":"search","query":"foreign","reason":"inspect"});
        if let Some(selector) = selector {
            query["scope"] = json!(selector);
        }
        let current = visibility_value(visibility_call(root.path(), id, "context", query));
        assert!(
            current["hits"]
                .as_array()
                .unwrap()
                .iter()
                .all(|hit| hit["session_id"] == SESSION)
        );
    }
    let global = visibility_value(visibility_call(
        root.path(),
        "all-filter",
        "context",
        json!({"operation":"search","query":"foreign","scope":"all","reason":"inspect"}),
    ));
    assert_eq!(
        global["hits"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|hit| hit["session_id"] != SESSION)
            .count(),
        4
    );
    let calls = Arc::new(std::sync::Mutex::new(Vec::new()));
    for operation in ["send", "resume", "interrupt"] {
        let request = write_bound_request(
            root.path(),
            operation,
            "context",
            json!({"operation":operation,"thread_id":FOREIGN,"message":"please inspect","reason":"coordinate"}),
        );
        let mut session = ToolControlSession::new(
            root.path(),
            Selected { version: 2 },
            ProductionToolControlHandler::new(
                root.path(),
                VisibilityRuntime(Arc::clone(&calls)),
                NoJobAuthority,
            ),
            ProductionToolControlPolicy::default(),
        );
        let line = encode_tool_control(&request).unwrap();
        let result = session.handle_line(&line).unwrap();
        assert!(decode_tool_control_result(&result).unwrap().error.is_none());
        let mut restarted = ToolControlSession::new(
            root.path(),
            Selected { version: 2 },
            ProductionToolControlHandler::new(
                root.path(),
                VisibilityRuntime(Arc::clone(&calls)),
                NoJobAuthority,
            ),
            ProductionToolControlPolicy::default(),
        );
        assert_eq!(
            restarted.handle_line(&line).unwrap(),
            result,
            "restart replays the receipt without invoking runtime twice"
        );
    }
    assert_eq!(*calls.lock().unwrap(), ["send", "resume", "interrupt"]);
    assert_eq!(
        fs::read(folder.join("main.jsonl")).unwrap(),
        original_ledger,
        "handler never authors target semantic events"
    );
    assert_eq!(
        fs::read(engine::permission_mode_path(&folder)).unwrap(),
        original_mode,
        "sending or control does not grant the sender's permission mode"
    );
    visibility_value(visibility_call(
        root.path(),
        "archive-global",
        "context",
        json!({"operation":"archive","thread_id":FOREIGN,"reason":"archive"}),
    ));
    let archived = visibility_value(visibility_call(
        root.path(),
        "read-archived",
        "context",
        json!({"operation":"read","thread_id":FOREIGN,"reason":"inspect"}),
    ));
    assert_eq!(archived["archived"], true);
    assert!(
        visibility_call(
            root.path(),
            "archive-again",
            "context",
            json!({"operation":"archive","thread_id":FOREIGN,"reason":"archive"})
        )
        .error
        .is_some()
    );
    assert_eq!(
        visibility_call(
            root.path(),
            "archive-self",
            "context",
            json!({"operation":"archive","thread_id":SESSION,"reason":"archive"})
        )
        .error
        .unwrap()
        .code,
        ToolControlErrorCode::Denied
    );
}

#[test]
fn context_search_cursors_bind_targets_filters_and_reject_legacy_ambiguous_identity() {
    use sha2::{Digest, Sha256};
    let root = tempfile::tempdir().unwrap();
    foreign_session(root.path(), FOREIGN);
    foreign_session(root.path(), FOREIGN_TWO);
    let query = json!({"operation":"search","query":"foreign","scope":"thread","thread_id":FOREIGN,"limit":1,"reason":"inspect"});
    let first = visibility_value(visibility_call(
        root.path(),
        "first",
        "context",
        query.clone(),
    ));
    let cursor = first["cursor"].as_str().unwrap();
    let mut second = query.clone();
    second["cursor"] = json!(cursor);
    let page = visibility_value(visibility_call(
        root.path(),
        "second",
        "context",
        second.clone(),
    ));
    assert_eq!(page["hits"][0]["record_id"], 3);
    for (id, key, value) in [
        ("target", "thread_id", FOREIGN_TWO),
        ("scope", "scope", "all"),
        ("query", "query", "different"),
    ] {
        let mut changed = second.clone();
        changed[key] = json!(value);
        assert_eq!(
            visibility_call(root.path(), id, "context", changed)
                .error
                .unwrap()
                .code,
            ToolControlErrorCode::Conflict
        );
    }
    let legacy = format!(
        "tc1-{}-1",
        &format!("{:x}", Sha256::digest(b"search:ws:thread:foreign"))[..16]
    );
    second["cursor"] = json!(legacy);
    assert_eq!(
        visibility_call(root.path(), "legacy", "context", second)
            .error
            .unwrap()
            .code,
        ToolControlErrorCode::Conflict
    );
    for (id, target) in [
        ("traversal", "../outside"),
        ("missing", "018f0000-0000-7000-8000-000000000097"),
    ] {
        assert!(
            visibility_call(
                root.path(),
                id,
                "context",
                json!({"operation":"read","thread_id":target,"reason":"inspect"})
            )
            .error
            .is_some()
        );
    }
    #[cfg(unix)]
    {
        let link = root
            .path()
            .join("threads")
            .join("018f0000-0000-7000-8000-000000000096");
        std::os::unix::fs::symlink(root.path().join("threads").join(FOREIGN), &link).unwrap();
        assert!(visibility_call(root.path(), "symlink", "context", json!({"operation":"read","thread_id":"018f0000-0000-7000-8000-000000000096","reason":"inspect"})).error.is_some());
    }
}

struct VisibilityRuntime(Arc<std::sync::Mutex<Vec<&'static str>>>);
impl SupervisorRuntimeAuthority for VisibilityRuntime {
    fn ensure_running(
        &mut self,
        session: &str,
        _request: &str,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        assert_eq!(session, FOREIGN);
        self.0.lock().unwrap().push("resume");
        Ok(ijson(json!({"running":true})))
    }
    fn deliver_input(
        &mut self,
        request: &DeliveryRequest,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        assert_eq!(request.target_session, FOREIGN);
        assert_eq!(request.message, "please inspect");
        self.0.lock().unwrap().push("send");
        Ok(ijson(json!({"seq":4})))
    }
    fn interrupt(
        &mut self,
        request: &InterruptRequest,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        assert_eq!(request.target_session, FOREIGN);
        self.0.lock().unwrap().push("interrupt");
        Ok(ijson(json!({"seq":5})))
    }
    fn ensure_child(
        &mut self,
        _proof: &ChildLaunchProof,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("not child spawn")
    }
    fn deliver_report(
        &mut self,
        _proof: &ParentReportProof,
        _result: &IJsonValue,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        panic!("not parent report")
    }
}
