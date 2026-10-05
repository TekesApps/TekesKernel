use std::collections::BTreeSet;
use std::fs;

use engine::{
    AllowAllPolicy, ApprovalClass, CatalogEntry, FakeToolBackend, PolicyDecision,
    ProductionToolPolicy, ToolBackend, ToolDispatcher, ToolInvocation, ToolPolicy, WorkflowBackend,
};
use schema::IJsonValue;
use serde_json::{Value, json};
use store::{AssetStore, LockedLedger};
use tools::{
    BackendTerminal, BuiltinManifest, BuiltinTool, CatalogContext, PipelineDecision, SecretScanner,
    ToolExecution, ToolPipeline,
};

const THREAD: &str = "018f0000-0000-7000-8000-000000000003";
const TS: &str = "2026-08-26T09:00:00.000Z";

#[test]
fn provider_catalog_omits_visible_tools_without_an_executable_backend() {
    struct ReadOnlyBackend;
    impl ToolBackend for ReadOnlyBackend {
        fn supports(&self, name: &str) -> bool {
            name == "read"
        }

        fn execute(
            &mut self,
            _execution: &ToolExecution,
            _invocation: &IJsonValue,
        ) -> BackendTerminal {
            unreachable!("catalog construction never executes a backend")
        }
    }

    let manifest = BuiltinManifest::compiled();
    let context = CatalogContext::default();
    let catalog = ToolDispatcher::new(&manifest, &context)
        .provider_catalog(&ReadOnlyBackend)
        .expect("provider catalog");
    let values = catalog
        .iter()
        .map(|value| serde_json::to_value(value).expect("JSON"))
        .collect::<Vec<_>>();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0]["name"], "read");
    assert!(values[0]["parameters"].is_object());
}

#[test]
fn dispatcher_derives_catalog_schema_and_backend_from_fixed_entry() {
    let directory = ledger_with_call("think", json!({"thought":"inspect"}));
    let path = directory.path().join("main.jsonl");
    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let manifest = BuiltinManifest::compiled();
    manifest.validate().expect("compiled manifest");
    let context = CatalogContext {
        selected_tools: BTreeSet::from(["think".to_owned()]),
        ..CatalogContext::default()
    };
    let dispatcher = ToolDispatcher::new(&manifest, &context);
    let invocation = invocation("think", json!({"thought":"inspect"}));
    let mut backend = FakeToolBackend::default();
    backend.set_terminal(
        "c1",
        BackendTerminal::Completed(IJsonValue::from("checked")),
    );
    let mut policy = AllowAllPolicy;
    let decision = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation,
            &[],
            &mut policy,
            &mut backend,
        )
        .expect("dispatch");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 10 });
    assert_eq!(backend.execution_count("c1"), 1);
}

#[test]
fn dispatcher_rejects_unadvertised_invalid_and_non_durable_calls() {
    let directory = ledger_with_call("think", json!({"thought":"inspect"}));
    let path = directory.path().join("main.jsonl");
    let manifest = BuiltinManifest::compiled();
    let context = CatalogContext::default();
    let dispatcher = ToolDispatcher::new(&manifest, &context);
    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let mut backend = FakeToolBackend::default();
    let error = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("think", json!({"thought":"inspect"})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("think is selection-controlled");
    assert!(error.to_string().contains("not available"));
    assert_eq!(backend.execution_count("c1"), 0);
    drop(ledger);

    let context = CatalogContext {
        selected_tools: BTreeSet::from(["think".to_owned()]),
        ..CatalogContext::default()
    };
    let dispatcher = ToolDispatcher::new(&manifest, &context);
    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let error = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("think", json!({"thought":"different"})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("durable argument mismatch");
    assert!(error.to_string().contains("durable tool binding"));
}

#[test]
fn generic_policy_runs_after_effective_argument_validation() {
    struct DenyPolicy;
    impl ToolPolicy for DenyPolicy {
        fn decide(
            &mut self,
            class: ApprovalClass,
            _tool: &BuiltinTool,
            _execution: &ToolExecution,
            _effective_arguments: &IJsonValue,
        ) -> PolicyDecision {
            assert_eq!(class, ApprovalClass::ReadOnly);
            PolicyDecision::Deny("profile policy".to_owned())
        }
    }

    let directory = ledger_with_call("think", json!({"thought":"inspect"}));
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let manifest = BuiltinManifest::compiled();
    let context = CatalogContext {
        selected_tools: BTreeSet::from(["think".to_owned()]),
        ..CatalogContext::default()
    };
    let mut backend = FakeToolBackend::default();
    let decision = ToolDispatcher::new(&manifest, &context)
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("think", json!({"thought":"inspect"})),
            &[],
            &mut DenyPolicy,
            &mut backend,
        )
        .expect("policy denial is a terminal result");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 9 });
    assert_eq!(backend.execution_count("c1"), 0);
}

#[test]
fn production_policy_allows_reads_delegates_workflow_and_holds_effects() {
    let manifest = BuiltinManifest::compiled();
    let execution = ToolExecution {
        thread: THREAD.to_owned(),
        call: "c1".to_owned(),
        name: "read".to_owned(),
        attempt: "a1".to_owned(),
        invocation: IJsonValue::from(true),
        side_effectful: false,
        turn: 1,
        timestamp: TS.to_owned(),
    };
    let mut policy = ProductionToolPolicy;
    for (name, class, expected_hold) in [
        ("read", ApprovalClass::ReadOnly, false),
        ("ask_user_questions", ApprovalClass::WorkflowHold, false),
        ("apply_patch", ApprovalClass::Edit, true),
        ("shell", ApprovalClass::Execute, true),
        ("context", ApprovalClass::Destructive, true),
    ] {
        let tool = manifest
            .tools
            .iter()
            .find(|tool| tool.name == name)
            .expect("fixed tool");
        let decision = policy.decide(class, tool, &execution, &execution.invocation);
        assert_eq!(
            matches!(decision, PolicyDecision::Hold { .. }),
            expected_hold
        );
        if !expected_hold {
            assert_eq!(decision, PolicyDecision::Allow);
        }
    }
}

#[test]
fn production_policy_durably_parks_workspace_edits_before_backend_execution() {
    let arguments = json!({
        "operation":"update_file",
        "path":"a.txt",
        "diff":"@@\n-old\n+new",
        "expected_artifact_version":0,
        "summary":"update"
    });
    let directory = ledger_with_call("apply_patch", arguments.clone());
    let path = directory.path().join("main.jsonl");
    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let manifest = BuiltinManifest::compiled();
    let mut backend = FakeToolBackend::default();
    let decision = ToolDispatcher::new(&manifest, &CatalogContext::default())
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("apply_patch", arguments),
            &[],
            &mut ProductionToolPolicy,
            &mut backend,
        )
        .expect("production edit hold");
    assert_eq!(decision, PipelineDecision::Parked { request_seq: 9 });
    assert_eq!(backend.execution_count("c1"), 0);
    drop(ledger);
    let text = fs::read_to_string(path).expect("ledger text");
    assert!(text.contains(r#""kind":"approval_request""#));
    assert!(text.contains(r#""scope":"edit""#));
}

#[test]
fn granted_approval_resumes_durable_effective_invocation_exactly_once() {
    #[derive(Default)]
    struct RecordingBackend {
        calls: usize,
        invocation: Option<IJsonValue>,
    }
    impl ToolBackend for RecordingBackend {
        fn execute(
            &mut self,
            _execution: &ToolExecution,
            invocation: &IJsonValue,
        ) -> BackendTerminal {
            self.calls += 1;
            self.invocation = Some(invocation.clone());
            BackendTerminal::Completed(IJsonValue::from("executed"))
        }
    }
    struct MustNotRecheckPolicy;
    impl ToolPolicy for MustNotRecheckPolicy {
        fn decide(
            &mut self,
            _class: ApprovalClass,
            _tool: &BuiltinTool,
            _execution: &ToolExecution,
            _effective_arguments: &IJsonValue,
        ) -> PolicyDecision {
            panic!("a durable grant must not re-run policy")
        }
    }

    let original = json!({"thought":"original"});
    let effective = json!({"thought":"effective"});
    let directory = ledger_with_approval(
        "think",
        original.clone(),
        Some(effective.clone()),
        Some(true),
        "execute",
    );
    let path = directory.path().join("main.jsonl");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let manifest = BuiltinManifest::compiled();
    let context = CatalogContext {
        selected_tools: BTreeSet::from(["think".to_owned()]),
        ..CatalogContext::default()
    };
    let dispatcher = ToolDispatcher::new(&manifest, &context);
    let call = invocation("think", original);
    let mut backend = RecordingBackend::default();
    let first = {
        let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
        dispatcher
            .dispatch(
                &mut ToolPipeline::new(&mut ledger, assets.clone(), SecretScanner::default()),
                &call,
                &[],
                &mut MustNotRecheckPolicy,
                &mut backend,
            )
            .expect("resume approved call")
    };
    assert_eq!(first, PipelineDecision::Completed { result_seq: 13 });
    assert_eq!(backend.calls, 1);
    assert_eq!(
        serde_json::to_value(backend.invocation.as_ref().expect("invocation")).unwrap(),
        effective
    );

    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let second = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &call,
            &[],
            &mut MustNotRecheckPolicy,
            &mut backend,
        )
        .expect("completed call is idempotent");
    assert_eq!(second, first);
    assert_eq!(backend.calls, 1);
}

#[test]
fn pending_and_denied_approvals_never_execute_the_backend() {
    let manifest = BuiltinManifest::compiled();
    let context = CatalogContext {
        selected_tools: BTreeSet::from(["think".to_owned()]),
        ..CatalogContext::default()
    };
    let dispatcher = ToolDispatcher::new(&manifest, &context);
    let original = json!({"thought":"inspect"});

    let pending = ledger_with_approval("think", original.clone(), None, None, "execute");
    let mut ledger = LockedLedger::open(pending.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(pending.path().join("assets")).expect("assets");
    let mut backend = FakeToolBackend::default();
    let decision = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("think", original.clone()),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect("pending approval remains parked");
    assert_eq!(decision, PipelineDecision::Parked { request_seq: 9 });
    assert_eq!(backend.execution_count("c1"), 0);

    let denied = ledger_with_approval("think", original.clone(), None, Some(false), "execute");
    let denied_path = denied.path().join("main.jsonl");
    let mut ledger = LockedLedger::open(&denied_path, 1).expect("ledger");
    let assets = AssetStore::new(denied.path().join("assets")).expect("assets");
    let decision = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("think", original),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect("denial is a terminal result");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 11 });
    assert_eq!(backend.execution_count("c1"), 0);
    drop(ledger);
    let text = fs::read_to_string(denied_path).expect("ledger text");
    assert!(text.contains(r#""outcome":{"denied":"approval denied"}"#));

    let mismatched = ledger_with_approval(
        "think",
        json!({"thought":"inspect"}),
        None,
        Some(true),
        "edit",
    );
    let mut ledger = LockedLedger::open(mismatched.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(mismatched.path().join("assets")).expect("assets");
    let error = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("think", json!({"thought":"inspect"})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("a response for another scope must not release the hold");
    assert!(error.to_string().contains("scope does not match"));
    assert_eq!(backend.execution_count("c1"), 0);
}

#[test]
fn skill_consumption_requires_a_prior_durable_offer_in_the_same_turn() {
    let directory = ledger_with_skill_offer(true);
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let manifest = BuiltinManifest::compiled();
    let context = CatalogContext {
        selected_tools: BTreeSet::from(["skill".to_owned()]),
        ..CatalogContext::default()
    };
    let skill = CatalogEntry {
        name: "rust-review".to_owned(),
        summary: "Review Rust".to_owned(),
        aliases: vec![],
        schema_digest: "sha256-skill".to_owned(),
        content: IJsonValue::from("skill body"),
    };
    let state = tempfile::tempdir().expect("workflow state");
    let mut backend =
        WorkflowBackend::new(state.path(), "ws", None, vec![skill], vec![]).expect("backend");
    let invocation = invocation_for("c2", "skill", json!({"skill":"rust-review"}));
    let decision = ToolDispatcher::new(&manifest, &context)
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation,
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect("durable offer permits skill load");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 12 });

    let directory = ledger_with_skill_offer(false);
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let error = ToolDispatcher::new(&manifest, &context)
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation,
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("a different offered name is not causal proof");
    assert!(error.to_string().contains("not causally offered"));
}

fn invocation(name: &str, arguments: Value) -> ToolInvocation {
    invocation_for("c1", name, arguments)
}

fn invocation_for(call: &str, name: &str, arguments: Value) -> ToolInvocation {
    let bytes = serde_json_canonicalizer::to_vec(&arguments).expect("canonical arguments");
    ToolInvocation {
        thread: THREAD.to_owned(),
        turn: 1,
        attempt: "a1".to_owned(),
        call: call.to_owned(),
        name: name.to_owned(),
        arguments: IJsonValue::parse(&bytes).expect("I-JSON arguments"),
        timestamp: TS.to_owned(),
    }
}

fn ledger_with_skill_offer(matches: bool) -> tempfile::TempDir {
    let directory = tempfile::tempdir().expect("thread");
    fs::create_dir(directory.path().join("assets")).expect("assets");
    let offered = if matches {
        "rust-review"
    } else {
        "swift-review"
    };
    let offer = serde_json_canonicalizer::to_string(&json!({
        "offer":"c1",
        "items":[{"name":offered,"summary":"Review","aliases":[],"schema_digest":"sha256-skill"}]
    }))
    .expect("offer JSON");
    let events = vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":THREAD,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"create","key":"create"}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":[{"type":"text","text":"start"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
        json!({"v":1,"seq":3,"kind":"run_start","ts":TS,"run":"r1","mode":"ordinary","recovery_ordinal":0,"binary":"worker","config_digest":"cfg","instruction_digest":"ins","policy":"default"}),
        json!({"v":1,"seq":4,"turn":1,"kind":"turn_open","ts":TS,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":5,"kind":"epoch","ts":TS,"id":"e1","reason":"initial","adapter":"fake","model":"fake","system":{"asset":"sha256-aa","digest":"d"},"tools":{"asset":"sha256-td","digest":"td"},"renderer":1}),
        json!({"v":1,"seq":6,"turn":1,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":TS,"attempt":"a1","epoch":"e1","wire_digest":"wd","admits":[{"from":2,"to":2}]}),
        json!({"v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"skill_explorer","args":{"query":"review","limit":5},"source":"provider"}),
        json!({"v":1,"seq":8,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":offer}]}),
        json!({"v":1,"seq":9,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c2","name":"skill","args":{"skill":"rust-review"},"source":"provider"}),
        json!({"v":1,"seq":10,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[],"sealed":{"version":1,"adapter":"fake","fragments":"calls"},"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1"}}),
    ];
    let mut bytes = Vec::new();
    for event in events {
        bytes.extend(serde_json_canonicalizer::to_vec(&event).expect("canonical event"));
        bytes.push(b'\n');
    }
    fs::write(directory.path().join("main.jsonl"), bytes).expect("ledger");
    directory
}

fn ledger_with_call(name: &str, arguments: Value) -> tempfile::TempDir {
    let directory = tempfile::tempdir().expect("thread");
    fs::create_dir(directory.path().join("assets")).expect("assets");
    let events = vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":THREAD,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"create","key":"create"}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":[{"type":"text","text":"start"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
        json!({"v":1,"seq":3,"kind":"run_start","ts":TS,"run":"r1","mode":"ordinary","recovery_ordinal":0,"binary":"worker","config_digest":"cfg","instruction_digest":"ins","policy":"default"}),
        json!({"v":1,"seq":4,"turn":1,"kind":"turn_open","ts":TS,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":5,"kind":"epoch","ts":TS,"id":"e1","reason":"initial","adapter":"fake","model":"fake","system":{"asset":"sha256-aa","digest":"d"},"tools":{"asset":"sha256-td","digest":"td"},"renderer":1}),
        json!({"v":1,"seq":6,"turn":1,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":TS,"attempt":"a1","epoch":"e1","wire_digest":"wd","admits":[{"from":2,"to":2}]}),
        json!({"v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":name,"args":arguments,"source":"provider"}),
        json!({"v":1,"seq":8,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[],"sealed":{"version":1,"adapter":"fake","fragments":"calls"},"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1"}}),
    ];
    let mut bytes = Vec::new();
    for event in events {
        bytes.extend(serde_json_canonicalizer::to_vec(&event).expect("canonical event"));
        bytes.push(b'\n');
    }
    fs::write(directory.path().join("main.jsonl"), bytes).expect("ledger");
    directory
}

fn ledger_with_approval(
    name: &str,
    arguments: Value,
    effective: Option<Value>,
    grant: Option<bool>,
    response_scope: &str,
) -> tempfile::TempDir {
    let directory = ledger_with_call(name, arguments);
    let path = directory.path().join("main.jsonl");
    let mut bytes = fs::read(&path).expect("ledger prefix");
    let mut seq = 9;
    if let Some(effective) = effective {
        bytes.extend(
            serde_json_canonicalizer::to_vec(&json!({
                "v":1,"seq":seq,"turn":1,"kind":"effective_execution","ts":TS,
                "call":"c1","invocation":effective
            }))
            .expect("effective execution"),
        );
        bytes.push(b'\n');
        seq += 1;
    }
    bytes.extend(
        serde_json_canonicalizer::to_vec(&json!({
            "v":1,"seq":seq,"turn":1,"kind":"approval_request","ts":TS,
            "call":"c1","scope":"execute","question":{"tool":name}
        }))
        .expect("approval request"),
    );
    bytes.push(b'\n');
    seq += 1;
    if let Some(grant) = grant {
        bytes.extend(
            serde_json_canonicalizer::to_vec(&json!({
                "v":1,"seq":seq,"turn":1,"kind":"approval_response","ts":TS,
                "call":"c1","scope":response_scope,"grant":grant,
                "origin_key":"approval-1",
                "origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"approve","key":"approval-1"}
            }))
            .expect("approval response"),
        );
        bytes.push(b'\n');
    }
    fs::write(path, bytes).expect("approval ledger");
    directory
}
