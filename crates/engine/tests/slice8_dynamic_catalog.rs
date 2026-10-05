use std::fs;
use std::sync::{Arc, Mutex};

use engine::{
    AllowAllPolicy, ApprovalClass, DynamicBackend, DynamicSupervisorBackend, DynamicToolDispatcher,
    PolicyDecision, ToolInvocation, ToolPolicy,
};
use mcp::{McpTool, McpToolAnnotations, project_catalog};
use profile::{
    DynamicTool, DynamicToolCatalog, DynamicToolEffect, DynamicToolSource, DynamicToolSourceKind,
};
use schema::IJsonValue;
use serde_json::{Value, json};
use store::{AssetStore, LockedLedger};
use tools::{
    BackendTerminal, BuiltinTool, PipelineDecision, SecretScanner, ToolExecution, ToolPipeline,
};
use worker_control::ToolControlResult;

const THREAD: &str = "018f0000-0000-7000-8000-000000000003";
const TS: &str = "2026-08-27T09:00:00.000Z";

#[derive(Default)]
struct RecordingDynamicBackend {
    available: bool,
    calls: usize,
    seen: Option<DynamicTool>,
}

impl DynamicBackend for RecordingDynamicBackend {
    fn supports(&self, _tool: &DynamicTool) -> bool {
        self.available
    }

    fn execute(
        &mut self,
        tool: &DynamicTool,
        _execution: &ToolExecution,
        invocation: &IJsonValue,
    ) -> BackendTerminal {
        self.calls += 1;
        self.seen = Some(tool.clone());
        BackendTerminal::Completed(ijson(json!({
            "source":tool.source.id,
            "arguments":serde_json::to_value(invocation).expect("arguments")
        })))
    }
}

#[test]
fn resident_projection_is_exact_stable_and_dependency_gated() {
    let catalog = catalog(vec![
        tool("plugin.frame", false, DynamicToolEffect::ComputerControl),
        tool("plugin.status", true, DynamicToolEffect::ReadOnly),
    ]);
    let dispatcher = DynamicToolDispatcher::new(&catalog);
    let available = RecordingDynamicBackend {
        available: true,
        ..RecordingDynamicBackend::default()
    };
    let projection = dispatcher.provider_catalog(&available);
    assert_eq!(projection.len(), 1);
    assert_eq!(
        serde_json::to_value(&projection[0]).unwrap()["name"],
        "plugin.status"
    );
    assert_eq!(dispatcher.complete_catalog(&available).len(), 2);
    assert_eq!(
        dispatcher
            .deferred_search_entries(&available)
            .unwrap()
            .len(),
        1
    );

    let unavailable = RecordingDynamicBackend::default();
    assert!(dispatcher.provider_catalog(&unavailable).is_empty());
    assert!(dispatcher.complete_catalog(&unavailable).is_empty());
}

#[test]
fn dynamic_dispatch_uses_exact_metadata_and_the_common_policy_pipeline() {
    struct DestructivePolicy {
        calls: usize,
    }
    impl ToolPolicy for DestructivePolicy {
        fn decide(
            &mut self,
            class: ApprovalClass,
            tool: &BuiltinTool,
            _execution: &ToolExecution,
            _effective_arguments: &IJsonValue,
        ) -> PolicyDecision {
            self.calls += 1;
            assert_eq!(class, ApprovalClass::Destructive);
            assert_eq!(tool.name, "plugin.frame");
            assert_eq!(tool.backend, tools::Backend::SupervisorControl);
            PolicyDecision::Allow
        }
    }

    let dynamic = tool("plugin.frame", true, DynamicToolEffect::ComputerControl);
    let catalog = catalog(vec![dynamic.clone()]);
    let directory = ledger_with_call("plugin.frame", json!({"quality":7}));
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let mut backend = RecordingDynamicBackend {
        available: true,
        ..RecordingDynamicBackend::default()
    };
    let mut policy = DestructivePolicy { calls: 0 };
    let decision = DynamicToolDispatcher::new(&catalog)
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("c1", "plugin.frame", json!({"quality":7})),
            &[],
            &mut policy,
            &mut backend,
        )
        .expect("dynamic dispatch");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 10 });
    assert_eq!(policy.calls, 1);
    assert_eq!(backend.calls, 1);
    assert_eq!(backend.seen, Some(dynamic));
}

#[test]
fn mcp_destructive_hint_reaches_the_engine_destructive_approval_gate() {
    struct DestructivePolicy;
    impl ToolPolicy for DestructivePolicy {
        fn decide(
            &mut self,
            class: ApprovalClass,
            tool: &BuiltinTool,
            _execution: &ToolExecution,
            _effective_arguments: &IJsonValue,
        ) -> PolicyDecision {
            assert_eq!(class, ApprovalClass::Destructive);
            assert_eq!(tool.name, "mcp__computer_2duse__request_permissions");
            PolicyDecision::Allow
        }
    }

    let projected = project_catalog(
        "computer-use",
        &[McpTool {
            name: "request_permissions".to_owned(),
            description: "Request a system permission".to_owned(),
            input_schema: ijson(json!({
                "type":"object",
                "properties":{},
                "required":[],
                "additionalProperties":false
            })),
            annotations: McpToolAnnotations {
                read_only_hint: false,
                destructive_hint: true,
            },
            metadata: Some(ijson(json!({"example.invalid/approval":{}}))),
            execution: None,
        }],
        true,
    )
    .expect("project destructive MCP tool");
    assert_eq!(projected.tools[0].effect, DynamicToolEffect::Destructive);
    let directory = ledger_with_call("mcp__computer_2duse__request_permissions", json!({}));
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let mut backend = RecordingDynamicBackend {
        available: true,
        ..RecordingDynamicBackend::default()
    };
    let decision = DynamicToolDispatcher::new(&projected)
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("c1", "mcp__computer_2duse__request_permissions", json!({})),
            &[],
            &mut DestructivePolicy,
            &mut backend,
        )
        .expect("destructive MCP dispatch");
    assert!(matches!(decision, PipelineDecision::Completed { .. }));
    assert_eq!(backend.calls, 1);
}

#[test]
fn deferred_provider_projection_requires_a_visible_search_offer_and_backend() {
    let catalog = catalog(vec![
        tool("plugin.frame", false, DynamicToolEffect::ComputerControl),
        tool("plugin.status", true, DynamicToolEffect::ReadOnly),
    ]);
    let dispatcher = DynamicToolDispatcher::new(&catalog);
    for offered_name in ["plugin.frame", "different.tool"] {
        let directory = ledger_with_offer(offered_name);
        // Reopen durable history: no in-memory activation cache is involved.
        let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).unwrap();
        let assets = AssetStore::new(directory.path().join("assets")).unwrap();
        let pipeline = ToolPipeline::new(&mut ledger, assets, SecretScanner::default());
        let offered = if offered_name == "plugin.frame" {
            vec!["plugin.frame", "plugin.status"]
        } else {
            vec!["plugin.status"]
        };
        for (before_seq, available, expected) in [
            (8, true, vec!["plugin.status"]),
            (9, true, offered.clone()),
            // The offer outlives its turn: a later request still declares it.
            (99, true, offered),
            (99, false, vec![]),
        ] {
            let backend = RecordingDynamicBackend {
                available,
                ..Default::default()
            };
            let schemas = dispatcher
                .provider_catalog_visible(&backend, &pipeline, before_seq)
                .unwrap();
            let names: Vec<String> = schemas
                .iter()
                .map(|schema| {
                    serde_json::to_value(schema).unwrap()["name"]
                        .as_str()
                        .unwrap()
                        .to_owned()
                })
                .collect();
            assert_eq!(
                names, expected,
                "before={before_seq} offered={offered_name}"
            );
            assert_eq!(backend.calls, 0, "projection must not execute a tool");
        }
    }
}

/// A compact that covers the search result takes the offer with it: the
/// model can no longer see the offer, so the tool is neither declared nor
/// executable, while the same offer stays valid before the compact.
#[test]
fn compaction_over_the_search_result_retires_the_offer() {
    let catalog = catalog(vec![tool(
        "plugin.frame",
        false,
        DynamicToolEffect::ComputerControl,
    )]);
    let dispatcher = DynamicToolDispatcher::new(&catalog);
    let offer = serde_json_canonicalizer::to_string(&json!({
        "offer":"c1",
        "items":[{"name":"plugin.frame","summary":"Capture a frame","aliases":["camera"],"schema_digest":"sha256-offer"}]
    }))
    .expect("offer");
    let directory = ledger(vec![
        json!({"v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"tool_search","args":{"query":"camera","limit":5},"source":"provider"}),
        json!({"v":1,"seq":8,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":offer}]}),
        json!({"v":1,"seq":9,"kind":"checkpoint","ts":TS,"covers":8,"summary":"boundary"}),
        json!({"v":1,"seq":10,"kind":"compact","ts":TS,"covers":[{"from":7,"to":8}],"summary":"[compacted history]"}),
        json!({"v":1,"seq":11,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c2","name":"plugin.frame","args":{"quality":7},"source":"provider"}),
    ]);
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    assert_eq!(
        ledger.projection().unwrap().last_seq,
        12,
        "the checkpoint and compact are accepted"
    );
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let backend = RecordingDynamicBackend {
        available: true,
        ..Default::default()
    };
    let names = |before_seq: u64, pipeline: &ToolPipeline<'_>| -> Vec<String> {
        dispatcher
            .provider_catalog_visible(&backend, pipeline, before_seq)
            .unwrap()
            .iter()
            .map(|schema| {
                serde_json::to_value(schema).unwrap()["name"]
                    .as_str()
                    .unwrap()
                    .to_owned()
            })
            .collect()
    };
    {
        let pipeline = ToolPipeline::new(
            &mut ledger,
            AssetStore::new(directory.path().join("assets")).unwrap(),
            SecretScanner::default(),
        );
        assert_eq!(
            names(10, &pipeline),
            vec!["plugin.frame"],
            "visible before the compact"
        );
        assert_eq!(
            names(11, &pipeline),
            Vec::<String>::new(),
            "retired once the compact covers the result"
        );
    }
    let mut backend = RecordingDynamicBackend {
        available: true,
        ..Default::default()
    };
    let error = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("c2", "plugin.frame", json!({"quality":7})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("the compacted offer no longer authorizes execution");
    assert!(
        error.to_string().contains("not causally offered"),
        "{error}"
    );
    assert_eq!(backend.calls, 0);
}

#[test]
fn deferred_dispatch_requires_a_durable_visible_tool_search_offer() {
    let catalog = catalog(vec![tool(
        "plugin.frame",
        false,
        DynamicToolEffect::ComputerControl,
    )]);
    let dispatcher = DynamicToolDispatcher::new(&catalog);
    let mut backend = RecordingDynamicBackend {
        available: true,
        ..RecordingDynamicBackend::default()
    };

    let offered = ledger_with_offer("plugin.frame");
    let mut ledger = LockedLedger::open(offered.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(offered.path().join("assets")).expect("assets");
    let decision = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("c2", "plugin.frame", json!({"quality":7})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect("offered dispatch");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 12 });
    assert_eq!(backend.calls, 1);

    let not_offered = ledger_with_offer("different.tool");
    let mut ledger = LockedLedger::open(not_offered.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(not_offered.path().join("assets")).expect("assets");
    let error = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("c2", "plugin.frame", json!({"quality":7})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("missing causal offer");
    assert!(error.to_string().contains("not causally offered"));
    assert_eq!(backend.calls, 1);
}

#[test]
fn exact_dynamic_schema_and_dependency_are_checked_before_effect() {
    let catalog = catalog(vec![tool(
        "plugin.frame",
        true,
        DynamicToolEffect::ComputerControl,
    )]);
    let dispatcher = DynamicToolDispatcher::new(&catalog);
    let directory = ledger_with_call("plugin.frame", json!({"quality":0,"extra":true}));
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let mut backend = RecordingDynamicBackend {
        available: true,
        ..RecordingDynamicBackend::default()
    };
    let error = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("c1", "plugin.frame", json!({"quality":0,"extra":true})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("schema rejection");
    assert!(error.to_string().contains("invalid dynamic arguments"));
    assert_eq!(backend.calls, 0);

    let directory = ledger_with_call("plugin.frame", json!({"quality":7}));
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    backend.available = false;
    let error = dispatcher
        .dispatch(
            &mut ToolPipeline::new(&mut ledger, assets, SecretScanner::default()),
            &invocation("c1", "plugin.frame", json!({"quality":7})),
            &[],
            &mut AllowAllPolicy,
            &mut backend,
        )
        .expect_err("dependency rejection");
    assert!(error.to_string().contains("dependency is unavailable"));
    assert_eq!(backend.calls, 0);
}

#[test]
fn supervisor_dynamic_route_uses_the_same_correlated_control_tuple() {
    let dynamic = tool("plugin.frame", true, DynamicToolEffect::ComputerControl);
    let seen = Arc::new(Mutex::new(None));
    let captured = Arc::clone(&seen);
    let mut backend = DynamicSupervisorBackend::new(
        "018f0000-0000-7000-8000-000000000003",
        move |request: &worker_control::ToolControl| {
            *captured.lock().expect("request lock") = Some(request.clone());
            Ok(ToolControlResult::success(
                request.request_id.clone(),
                request.call_id.clone(),
                ijson(json!({"frame":"ready"})),
            ))
        },
    );
    assert!(backend.supports(&dynamic));
    let terminal = backend.execute(
        &dynamic,
        &ToolExecution {
            thread: THREAD.to_owned(),
            turn: 1,
            attempt: "a1".to_owned(),
            call: "control-call".to_owned(),
            name: dynamic.name.clone(),
            invocation: ijson(json!({"quality":7})),
            side_effectful: true,
            timestamp: TS.to_owned(),
        },
        &ijson(json!({"quality":7})),
    );
    assert_eq!(
        terminal,
        BackendTerminal::Completed(ijson(json!({"frame":"ready"})))
    );
    let request = seen.lock().unwrap().clone().expect("captured request");
    assert_eq!(request.name, "plugin.frame");
    assert_eq!(request.call_id, "control-call");
    assert_eq!(request.thread, THREAD);
    assert_eq!(request.turn, 1);
    assert_eq!(request.arguments, ijson(json!({"quality":7})));
}

#[test]
fn supervisor_dynamic_route_maps_control_eof_to_retryable_unavailable() {
    let dynamic = tool("plugin.frame", true, DynamicToolEffect::ComputerControl);
    let mut backend =
        DynamicSupervisorBackend::new(THREAD, |_request: &worker_control::ToolControl| {
            Err("supervisor control eof".to_owned())
        });
    let terminal = backend.execute(
        &dynamic,
        &ToolExecution {
            thread: THREAD.to_owned(),
            turn: 1,
            attempt: "a1".to_owned(),
            call: "control-eof".to_owned(),
            name: dynamic.name.clone(),
            invocation: ijson(json!({"quality":7})),
            side_effectful: true,
            timestamp: TS.to_owned(),
        },
        &ijson(json!({"quality":7})),
    );
    assert_eq!(
        terminal,
        BackendTerminal::Unavailable {
            code: "transport".to_owned(),
            message: "supervisor control eof".to_owned(),
            retryable: true,
        }
    );
}

fn catalog(tools: Vec<DynamicTool>) -> DynamicToolCatalog {
    DynamicToolCatalog::resolve(tools).expect("dynamic catalog")
}

fn tool(name: &str, always_on: bool, effect: DynamicToolEffect) -> DynamicTool {
    DynamicTool::declared(
        name,
        DynamicToolSource {
            kind: DynamicToolSourceKind::Plugin,
            id: "com.tekes.camera".to_owned(),
        },
        effect,
        always_on,
        vec!["camera".to_owned()],
        ijson(json!({
            "name":name,
            "description":"Capture a frame",
            "parameters":{
                "type":"object",
                "properties":{"quality":{"type":"integer","minimum":1,"maximum":10}},
                "required":["quality"],
                "additionalProperties":false
            }
        })),
    )
    .expect("dynamic tool")
}

fn invocation(call: &str, name: &str, arguments: Value) -> ToolInvocation {
    ToolInvocation {
        thread: THREAD.to_owned(),
        turn: 1,
        attempt: "a1".to_owned(),
        call: call.to_owned(),
        name: name.to_owned(),
        arguments: ijson(arguments),
        timestamp: TS.to_owned(),
    }
}

fn ledger_with_call(name: &str, arguments: Value) -> tempfile::TempDir {
    ledger(vec![json!({
        "v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1",
        "call":"c1","name":name,"args":arguments,"source":"provider"
    })])
}

fn ledger_with_offer(offered_name: &str) -> tempfile::TempDir {
    let offer = serde_json_canonicalizer::to_string(&json!({
        "offer":"c1",
        "items":[{
            "name":offered_name,"summary":"Capture a frame","aliases":["camera"],
            "schema_digest":"sha256-offer"
        }]
    }))
    .expect("offer");
    ledger(vec![
        json!({"v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"tool_search","args":{"query":"camera","limit":5},"source":"provider"}),
        json!({"v":1,"seq":8,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":offer}]}),
        json!({"v":1,"seq":9,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c2","name":"plugin.frame","args":{"quality":7},"source":"provider"}),
    ])
}

fn ledger(mut calls: Vec<Value>) -> tempfile::TempDir {
    let directory = tempfile::tempdir().expect("thread");
    fs::create_dir(directory.path().join("assets")).expect("assets");
    let mut events = vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":THREAD,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"create","key":"create"}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":[{"type":"text","text":"start"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
        json!({"v":1,"seq":3,"kind":"run_start","ts":TS,"run":"r1","mode":"ordinary","recovery_ordinal":0,"binary":"worker","config_digest":"cfg","instruction_digest":"ins","policy":"default"}),
        json!({"v":1,"seq":4,"turn":1,"kind":"turn_open","ts":TS,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":5,"kind":"epoch","ts":TS,"id":"e1","reason":"initial","adapter":"fake","model":"fake","system":{"asset":"sha256-aa","digest":"d"},"tools":{"asset":"sha256-td","digest":"td"},"renderer":1}),
        json!({"v":1,"seq":6,"turn":1,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":TS,"attempt":"a1","epoch":"e1","wire_digest":"wd","admits":[{"from":2,"to":2}]}),
    ];
    events.append(&mut calls);
    let next_seq = events
        .last()
        .and_then(|event| event["seq"].as_u64())
        .unwrap()
        + 1;
    events.push(json!({"v":1,"seq":next_seq,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[],"sealed":{"version":1,"adapter":"fake","fragments":"calls"},"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1"}}));
    let mut bytes = Vec::new();
    for event in events {
        bytes.extend(serde_json_canonicalizer::to_vec(&event).expect("canonical event"));
        bytes.push(b'\n');
    }
    fs::write(directory.path().join("main.jsonl"), bytes).expect("ledger");
    directory
}

fn ijson(value: Value) -> IJsonValue {
    IJsonValue::parse(&serde_json_canonicalizer::to_vec(&value).expect("canonical JSON"))
        .expect("I-JSON")
}
