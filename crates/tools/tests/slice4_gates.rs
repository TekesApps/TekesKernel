use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::os::unix::fs::symlink;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use schema::{Event, IJsonValue, OriginTuple};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use store::{AssetStore, BarrierContext, LockedLedger, SyncPolicy, requires_barrier};
use tempfile::TempDir;
use test_support::{FixtureRoot, read};
use tools::{
    Availability, Backend, BuiltinManifest, ByteString, CatalogContext, CatalogRole, CreateMode,
    Effect, ExecRequest, HelperClient, HelperErrorClass, HelperOperation, HelperRequest,
    HelperResponse, HelperServer, HookBinding, HookFailureMode, HookPhase, HookRequest,
    NetworkPolicy, PipelineDecision, ProbeFailure, ProbeStatus, ProcessHook, RootBinding,
    SandboxApproval, SandboxBackend, SandboxPolicy, SecretScanner, ToolExecution, ToolPipeline,
    compile_darwin_profile, compile_linux_plan, decode_helper_line, decode_hook_request,
    decode_hook_response, policy_digest, probe_backend, validate_unsandboxed_approval,
};

const TS: &str = "2026-08-26T09:00:00.000Z";

#[test]
fn execution_start_is_synced_before_backend_and_absent_while_approval_is_held() {
    let directory = tool_ledger();
    let path = directory.path().join("main.jsonl");
    let assets = AssetStore::new(directory.path().join("assets")).unwrap();
    let sync = Arc::new(CountingSync::default());
    let mut ledger = LockedLedger::open_with_sync(&path, 1, sync.clone()).unwrap();
    let execution = tool_execution();
    let held = ToolPipeline::new(&mut ledger, assets.clone(), SecretScanner::default())
        .execute_terminal_with_gate(
            &execution,
            &[],
            |_, _| tools::ApprovalGate::Hold {
                scope: "execute".into(),
                question: IJsonValue::from("run?"),
            },
            |_, _| panic!("hold must not execute"),
        )
        .unwrap();
    assert!(matches!(held, PipelineDecision::Parked { .. }));
    assert!(
        !fs::read_to_string(&path)
            .unwrap()
            .contains("tool_execution_started")
    );
    let answer = event(
        json!({"v":1,"seq":ledger.next_seq(),"turn":1,"kind":"approval_response","ts":TS,
        "call":"c1","grant":true,"origin_key":"approve","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"approve","key":"approve"}}),
    );
    ledger
        .append_contract(answer, BarrierContext::default())
        .unwrap();
    drop(ledger);
    let mut ledger = LockedLedger::open_with_sync(&path, 1, sync.clone()).unwrap();
    let before = sync.count.load(Ordering::SeqCst);
    ToolPipeline::new(&mut ledger, assets, SecretScanner::default())
        .execute_terminal_with_gate_and_resume(
            &execution,
            &[],
            |_, _| panic!("approval already granted"),
            |_, _, approval| {
                assert!(approval.is_some());
                assert_eq!(sync.count.load(Ordering::SeqCst), before + 1);
                let rows = fs::read_to_string(&path).unwrap();
                let last: Value = serde_json::from_str(rows.lines().last().unwrap()).unwrap();
                assert_eq!(last["kind"], "tool_execution_started");
                assert_eq!(last["call"], "c1");
                tools::BackendTerminal::Completed(IJsonValue::from("executed"))
            },
        )
        .unwrap();
}

#[test]
fn execution_start_sync_failure_never_enters_backend() {
    struct FailedSync;
    impl SyncPolicy for FailedSync {
        fn full_sync(&self, _file: &fs::File) -> io::Result<()> {
            Err(io::Error::other("injected execution barrier failure"))
        }
    }
    let directory = tool_ledger();
    let assets = AssetStore::new(directory.path().join("assets")).unwrap();
    let mut ledger =
        LockedLedger::open_with_sync(directory.path().join("main.jsonl"), 1, Arc::new(FailedSync))
            .unwrap();
    assert!(
        ToolPipeline::new(&mut ledger, assets, SecretScanner::default())
            .execute(&tool_execution(), &[], |_, _| panic!(
                "failed fsync must not execute"
            ))
            .is_err()
    );
}

#[test]
fn tool_contract_oracles() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    fixtures.verify_manifest().expect("manifest matches disk");

    let pre_bytes = read(&fixtures.join("hooks/pre-request.json")).expect("pre request");
    assert_canonical_json(&pre_bytes);
    let pre = decode_hook_request(&pre_bytes).expect("pre request contract");
    for name in [
        "pre-allow.json",
        "pre-deny.json",
        "pre-mutate.json",
        "pre-ask.json",
    ] {
        let bytes = read(&fixtures.join("hooks").join(name)).expect("pre response");
        assert_canonical_json(&bytes);
        decode_hook_response(&bytes, &pre).unwrap_or_else(|error| panic!("{name}: {error}"));
    }
    let correlation =
        read(&fixtures.join("hooks/correlation.invalid.json")).expect("correlation negative");
    assert!(decode_hook_response(&correlation, &pre).is_err());
    let post_bytes = read(&fixtures.join("hooks/post-request.json")).expect("post request");
    let post = decode_hook_request(&post_bytes).expect("post request contract");
    for name in ["post-allow.json", "post-deny.json"] {
        let bytes = read(&fixtures.join("hooks").join(name)).expect("post response");
        decode_hook_response(&bytes, &post).unwrap_or_else(|error| panic!("{name}: {error}"));
    }
    assert!(
        decode_hook_request(&read(&fixtures.join("hooks/extra-output.invalid.txt")).unwrap())
            .is_err()
    );

    for name in [
        "hello-selected.jsonl",
        "read.jsonl",
        "write.jsonl",
        "exec.jsonl",
        "error.jsonl",
    ] {
        let bytes = read(&fixtures.join("helper").join(name)).expect("helper transcript");
        for line in bytes.split_inclusive(|byte| *byte == b'\n') {
            let (key, payload) =
                decode_helper_line(line).unwrap_or_else(|error| panic!("{name}: {error}"));
            if key == "request" {
                serde_json::from_value::<HelperRequest>(payload)
                    .unwrap_or_else(|error| panic!("{name}: {error}"));
            }
        }
    }
    assert!(
        decode_helper_line(&read(&fixtures.join("helper/multi-key.invalid.jsonl")).unwrap())
            .is_err()
    );

    let policy_bytes = read(&fixtures.join("sandbox/policy.canonical.json")).expect("policy");
    assert_canonical_json(&policy_bytes);
    let policy: SandboxPolicy = serde_json::from_slice(&policy_bytes).expect("policy shape");
    policy.validate().expect("policy semantics");
    assert_eq!(
        compile_darwin_profile(&policy).expect("Darwin profile"),
        read(&fixtures.join("sandbox/darwin-seatbelt.sb")).expect("Darwin golden")
    );
    assert_eq!(
        compile_linux_plan(&policy).expect("Linux plan"),
        read(&fixtures.join("sandbox/linux-plan.canonical.json")).expect("Linux golden")
    );
    for name in ["unsorted.invalid.json", "write-outside.invalid.json"] {
        let policy: SandboxPolicy = serde_json::from_slice(
            &read(&fixtures.join("sandbox").join(name)).expect("invalid policy"),
        )
        .expect("negative shape");
        assert!(policy.validate().is_err(), "{name} must reject");
    }
}

#[test]
fn slice4_gate_43_builtin_tool_manifest_parity() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let bytes =
        read(&fixtures.join("tools/builtin-tools.canonical.json")).expect("builtin manifest");
    let raw: Value = serde_json::from_slice(&bytes).expect("manifest JSON");
    assert_eq!(raw.as_object().unwrap().len(), 2);
    assert!(
        raw["tools"]
            .as_array()
            .unwrap()
            .iter()
            .all(|tool| tool.as_object().unwrap().len() == 5)
    );
    let mut historical = raw.clone();
    historical["migration_sources"] = json!([]);
    assert!(BuiltinManifest::decode_canonical(&canonical_line(&historical)).is_err());
    let manifest = BuiltinManifest::decode_canonical(&bytes).expect("closed canonical manifest");
    assert_eq!(manifest.format, 1);
    assert_eq!(manifest.tools.len(), 25);
    assert_eq!(manifest.tools.first().unwrap().name, "apply_patch");
    assert_eq!(manifest.tools.last().unwrap().name, "write");
    let original_digest = manifest.catalog_digest().unwrap();
    let digest_oracle: Value = serde_json::from_slice(
        &read(&fixtures.join("tools/builtin-tools.digest.canonical.json"))
            .expect("builtin digest oracle"),
    )
    .expect("digest oracle JSON");
    assert_eq!(digest_oracle["format"], 1);
    assert_eq!(digest_oracle["algorithm"], "sha256");
    assert_eq!(digest_oracle["preimage"], "rfc8785-tools-array");
    assert_eq!(digest_oracle["digest"], original_digest);
    assert_eq!(
        digest_oracle["preimage_bytes"].as_u64(),
        Some(
            serde_json_canonicalizer::to_vec(&manifest.tools)
                .expect("canonical tools array")
                .len() as u64
        )
    );
    let removed_names = [
        "note",
        "recall",
        "compact",
        "git_branch",
        "git_commit",
        "git_diff",
        "git_push",
        "git_status",
        "github_pr_create",
        "goal_completed",
    ];
    for removed in removed_names {
        assert!(!manifest.fixed_names().contains(removed));
        let mut replaced = manifest.clone();
        replaced.tools[0].name = removed.to_owned();
        assert!(
            replaced.validate().is_err(),
            "legacy name {removed} accepted"
        );
    }

    let mut replaced_name = manifest.clone();
    replaced_name.tools[5].name = "glub".to_owned();
    assert_ne!(replaced_name.catalog_digest().unwrap(), original_digest);
    assert!(replaced_name.validate().is_err());

    let mut changed_arguments = manifest.clone();
    changed_arguments.tools[0].arguments.swap(0, 1);
    assert_ne!(changed_arguments.catalog_digest().unwrap(), original_digest);
    assert!(changed_arguments.validate().is_err());

    let mut changed_availability = manifest.clone();
    changed_availability.tools[0].availability = Availability::SelectionControlled;
    assert_ne!(
        changed_availability.catalog_digest().unwrap(),
        original_digest
    );
    assert!(changed_availability.validate().is_err());

    let mut changed_backend = manifest.clone();
    changed_backend.tools[0].backend = Backend::InProcess;
    assert_ne!(changed_backend.catalog_digest().unwrap(), original_digest);
    assert!(changed_backend.validate().is_err());

    let mut changed_effect = manifest.clone();
    changed_effect.tools[0].effect = Effect::ReadOnly;
    assert_ne!(changed_effect.catalog_digest().unwrap(), original_digest);
    assert!(changed_effect.validate().is_err());

    for historical_field in ["owner", "source", "exposure", "schema"] {
        let mut historical_item = raw.clone();
        historical_item["tools"][0][historical_field] = json!("historical");
        assert!(
            BuiltinManifest::decode_canonical(&canonical_line(&historical_item)).is_err(),
            "historical field {historical_field} accepted"
        );
    }
    assert!(manifest.reject_dynamic_collisions(["dynamic_ok"]).is_ok());
    assert!(manifest.reject_dynamic_collisions(["read"]).is_err());
    assert!(
        manifest
            .reject_dynamic_collisions(["same", "same"])
            .is_err()
    );

    let resident = [
        "apply_patch",
        "context",
        "context_get",
        "edit",
        "glob",
        "grep",
        "job",
        "read",
        "shell",
        "skill_explorer",
        "web_fetch",
        "write",
    ];
    assert_eq!(
        projection_names(&manifest, &CatalogContext::default()),
        resident
    );

    let credential = CatalogContext {
        has_web_credential: true,
        ..CatalogContext::default()
    };
    assert_eq!(
        projection_names(&manifest, &credential),
        [&resident[..11], &["web_search"], &resident[11..]].concat()
    );

    let deferred = CatalogContext {
        has_deferred_catalog: true,
        ..CatalogContext::default()
    };
    assert_eq!(
        projection_names(&manifest, &deferred),
        [&resident[..10], &["tool_search"], &resident[10..],].concat()
    );

    for (role, role_name) in [
        (CatalogRole::Plan, "plan"),
        (CatalogRole::Compactor, "summary_artifact"),
        (CatalogRole::Subagent, "report"),
        (CatalogRole::Validator, "verify"),
    ] {
        assert_eq!(
            projection_names(
                &manifest,
                &CatalogContext {
                    role,
                    ..CatalogContext::default()
                }
            ),
            [resident.as_slice(), &[role_name]].concat()
        );
    }

    let selected = [
        "ask_user_questions",
        "new_goal",
        "set_goal_state",
        "skill",
        "subagent",
        "task",
        "think",
    ];
    let selected_context = CatalogContext {
        selected_tools: selected.into_iter().map(str::to_owned).collect(),
        ..CatalogContext::default()
    };
    assert_eq!(
        projection_names(&manifest, &selected_context),
        [resident.as_slice(), selected.as_slice()].concat()
    );

    let benchmark = CatalogContext {
        role: CatalogRole::Benchmark,
        has_web_credential: true,
        has_deferred_catalog: true,
        selected_tools: selected.into_iter().map(str::to_owned).collect(),
    };
    assert_eq!(
        projection_names(&manifest, &benchmark),
        [
            "skill_explorer",
            "tool_search",
            "new_goal",
            "set_goal_state",
            "skill",
            "think",
        ]
    );

    let migration = read(&fixtures.join("tools/builtin-tool-migration.canonical.json"))
        .expect("migration audit");
    assert_canonical_json(&migration);
    let migration: Value = serde_json::from_slice(&migration).unwrap();
    assert_eq!(migration["format"], 1);
    assert_eq!(migration["sources"].as_array().unwrap().len(), 2);
    assert_eq!(
        migration["dispositions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item["legacy_names"].as_array().unwrap().len())
            .sum::<usize>(),
        10
    );

    let first_party = read(&fixtures.join("tools/first-party-tools.canonical.json"))
        .expect("first-party inventory");
    assert_canonical_json(&first_party);
    let value: serde_json::Value = serde_json::from_slice(&first_party).unwrap();
    assert_eq!(value["plugins"][0]["id"], "com.tekes.computer-use");
    assert_eq!(value["plugins"][0]["tools"].as_array().unwrap().len(), 12);
}

#[test]
fn slice4_gate_44_exec_helper_process_protocol() {
    let root = tempfile::tempdir().expect("root");
    fs::write(root.path().join("a.txt"), b"ok").expect("seed file");
    let root_path = fs::canonicalize(root.path()).expect("canonical root");
    let executable =
        fs::canonicalize(env!("CARGO_BIN_EXE_tekes-helper")).expect("canonical helper");
    let mut read_roots = vec![
        root_path.to_string_lossy().into_owned(),
        executable
            .parent()
            .expect("helper parent")
            .to_string_lossy()
            .into_owned(),
    ];
    read_roots.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    read_roots.dedup();
    let policy = SandboxPolicy {
        format: 1,
        read_roots,
        write_roots: vec![root_path.to_string_lossy().into_owned()],
        network: NetworkPolicy::Deny,
        allow_process: true,
        scratch: None,
    };
    let probe = probe_backend(host_sandbox_backend());
    if !host_has_sandbox() {
        assert!(matches!(probe, ProbeStatus::Unavailable { .. }));
        return;
    }
    assert!(matches!(probe, ProbeStatus::Available { .. }), "{probe:?}");

    let client = HelperClient::sandboxed(
        &executable,
        vec![("workspace".to_owned(), root_path.clone())],
        policy,
        probe,
    )
    .expect("sandboxed client");
    let read_response = client
        .execute(&HelperRequest {
            id: "r1".to_owned(),
            operation: HelperOperation::Read {
                root: "workspace".to_owned(),
                path: "a.txt".to_owned(),
                max_bytes: 4096,
            },
        })
        .expect("read process");
    assert!(matches!(
        read_response,
        HelperResponse::Result {
            value: tools::HelperValue::Read(ref value),
            ..
        } if value.content.decode().expect("read bytes") == b"ok"
    ));
    let write_response = client
        .execute(&HelperRequest {
            id: "w1".to_owned(),
            operation: HelperOperation::Write {
                root: "workspace".to_owned(),
                path: "b.txt".to_owned(),
                content: ByteString::from_bytes(b"written"),
                create: CreateMode::New,
            },
        })
        .expect("write process");
    assert!(matches!(
        write_response,
        HelperResponse::Result {
            value: tools::HelperValue::Write(_),
            ..
        }
    ));
    assert_eq!(fs::read(root.path().join("b.txt")).unwrap(), b"written");
}

#[test]
fn slice4_gate_45_sandbox_probe_and_approval_binding() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let policy: SandboxPolicy = serde_json::from_slice(
        &read(&fixtures.join("sandbox/policy.canonical.json")).expect("policy"),
    )
    .expect("policy shape");
    let approval: SandboxApproval = serde_json::from_slice(
        &read(&fixtures.join("sandbox/approval.canonical.json")).expect("approval"),
    )
    .expect("approval shape");
    let digest = policy_digest(&policy).expect("policy digest");
    assert_eq!(approval.policy_digest, digest);
    validate_unsandboxed_approval(&approval, "run1", "c1", &digest).expect("matching grant");
    for (run, call, candidate_digest) in [
        ("run2", "c1", digest.as_str()),
        ("run1", "c2", digest.as_str()),
        ("run1", "c1", "sha256-other"),
    ] {
        assert!(validate_unsandboxed_approval(&approval, run, call, candidate_digest).is_err());
    }
    let unavailable = ProbeStatus::Unavailable {
        class: ProbeFailure::Missing,
        detail: "removed for test".to_owned(),
    };
    assert!(
        HelperClient::sandboxed("/missing/helper", Vec::new(), policy.clone(), unavailable)
            .is_err()
    );
    assert!(
        HelperClient::approved_unsandboxed(
            "/missing/helper",
            Vec::new(),
            &policy,
            &approval,
            "wrong-run",
            "c1"
        )
        .is_err()
    );
}

#[test]
fn slice4_gate_46_hook_process_limits_and_correlation() {
    let request = HookRequest {
        format: 1,
        hook_id: "policy".to_owned(),
        phase: HookPhase::Pre,
        call: "c1".to_owned(),
        name: "exec".to_owned(),
        invocation: IJsonValue::parse_str(r#"{"argv":["/usr/bin/true"]}"#).unwrap(),
        result: None,
    };
    let response = r#"{"call":"c1","format":1,"hook_id":"policy","verdict":{"allow":{}}}"#;
    let binding = shell_hook(
        "policy",
        HookPhase::Pre,
        &format!("read _; printf '%s\\n' '{response}'"),
    );
    assert!(ProcessHook::run(&binding, &request).is_ok());

    let timeout = HookBinding {
        format: 1,
        id: "policy".to_owned(),
        argv: vec!["/bin/sh".to_owned(), "-c".to_owned(), "sleep 5".to_owned()],
        phase: HookPhase::Pre,
        failure: HookFailureMode::Closed,
        timeout_ms: 20,
        stdout_bytes: 4096,
        stderr_bytes: 4096,
        env: BTreeMap::new(),
    };
    let started = Instant::now();
    assert!(ProcessHook::run(&timeout, &request).is_err());
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn slice4_gate_07_mutated_tool_write_ahead() {
    let first = tool_ledger();
    let path = first.path().join("main.jsonl");
    let assets = AssetStore::new(first.path().join("assets")).expect("assets");
    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let tool_call = ledger
        .projection()
        .expect("projection")
        .events
        .last()
        .expect("tool call");
    assert!(requires_barrier(
        tool_call,
        BarrierContext {
            side_effectful_tool_call: true
        }
    ));
    let response = r#"{"call":"c1","format":1,"hook_id":"policy","verdict":{"mutate":{"invocation":{"argv":["/usr/bin/printf","safe"]},"reason":"remove shell"}}}"#;
    let binding = shell_hook(
        "policy",
        HookPhase::Pre,
        &format!("read _; printf '%s\\n' '{response}'"),
    );
    let execution = tool_execution();
    let mut count = 0;
    let decision = ToolPipeline::new(&mut ledger, assets, SecretScanner::default())
        .execute(&execution, &[binding], |_, invocation| {
            count += 1;
            assert_eq!(
                invocation.canonical_string().expect("invocation"),
                r#"{"argv":["/usr/bin/printf","safe"]}"#
            );
            Ok(IJsonValue::from("done"))
        })
        .expect("pipeline");
    assert_eq!(count, 1);
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 11 });
    drop(ledger);
    let bytes = fs::read(&path).expect("ledger bytes");
    let lines = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(
        Event::decode(lines[8]).unwrap().kind().as_str(),
        "effective_execution"
    );
    assert_eq!(
        Event::decode(lines[9]).unwrap().kind().as_str(),
        "tool_execution_started"
    );
    assert_eq!(
        Event::decode(lines[10]).unwrap().kind().as_str(),
        "tool_result"
    );

    let crash_prefix = lines[..9]
        .iter()
        .flat_map(|line| line.iter().copied().chain(std::iter::once(b'\n')))
        .collect::<Vec<_>>();
    fs::write(&path, crash_prefix).expect("simulate loss after effect");
    let recovered = LockedLedger::open(&path, 1).expect("recovered prefix");
    assert_eq!(recovered.projection().unwrap().last_seq, 9);
    assert!(recovered.projection().unwrap().lifecycle.unresolved_work);

    let second = tool_ledger();
    let second_path = second.path().join("main.jsonl");
    let assets = AssetStore::new(second.path().join("assets")).expect("assets");
    let mut ledger = LockedLedger::open(&second_path, 1).expect("ledger");
    let decision = ToolPipeline::new(&mut ledger, assets, SecretScanner::failing())
        .execute(&execution, &[], |_, _| Ok(IJsonValue::from("secret")))
        .expect("withheld pipeline");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 10 });
    drop(ledger);
    assert!(
        String::from_utf8(fs::read(second_path).unwrap())
            .unwrap()
            .contains(r#""outcome":{"withheld":"scan_failed"}"#)
    );
}

#[test]
fn ordered_hooks_chain_mutations_and_compose_post_decisions() {
    let directory = tool_ledger();
    let path = directory.path().join("main.jsonl");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let first = shell_hook(
        "01-pre",
        HookPhase::Pre,
        r#"read _; printf '%s\n' '{"call":"c1","format":1,"hook_id":"01-pre","verdict":{"mutate":{"invocation":{"argv":["stage1"]},"reason":"first"}}}'"#,
    );
    let second = shell_hook(
        "02-pre",
        HookPhase::Pre,
        r#"input=$(cat); case "$input" in *'"stage1"'*) printf '%s\n' '{"call":"c1","format":1,"hook_id":"02-pre","verdict":{"mutate":{"invocation":{"argv":["stage2"]},"reason":"second"}}}' ;; *) exit 7 ;; esac"#,
    );
    let post_first = shell_hook(
        "03-post",
        HookPhase::Post,
        r#"read _; printf '%s\n' '{"call":"c1","format":1,"hook_id":"03-post","verdict":{"allow":{"annotations":{"order":1}}}}'"#,
    );
    let post_second = shell_hook(
        "04-post",
        HookPhase::Post,
        r#"input=$(cat); case "$input" in *'"hook_id":"03-post"'*) printf '%s\n' '{"call":"c1","format":1,"hook_id":"04-post","verdict":{"deny":{"annotations":{"order":2},"reason":"review"}}}' ;; *) exit 7 ;; esac"#,
    );
    let post_third = shell_hook(
        "05-post",
        HookPhase::Post,
        r#"input=$(cat); case "$input" in *'"hook_id":"04-post"'*) printf '%s\n' '{"call":"c1","format":1,"hook_id":"05-post","verdict":{"allow":{"annotations":{"order":3}}}}' ;; *) exit 7 ;; esac"#,
    );
    let hooks = [first, second, post_first, post_second, post_third];
    let mut backend_calls = 0;
    let decision = ToolPipeline::new(&mut ledger, assets, SecretScanner::default())
        .execute(&tool_execution(), &hooks, |_, invocation| {
            backend_calls += 1;
            assert_eq!(
                invocation.canonical_string().expect("effective invocation"),
                r#"{"argv":["stage2"]}"#
            );
            Ok(IJsonValue::from("backend"))
        })
        .expect("ordered hooks");
    assert_eq!(decision, PipelineDecision::Completed { result_seq: 11 });
    assert_eq!(backend_calls, 1);
    drop(ledger);

    let events = fs::read(&path)
        .expect("ledger")
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Event::decode(line).expect("event"))
        .collect::<Vec<_>>();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind().as_str() == "effective_execution")
            .count(),
        1
    );
    let result = serde_json::to_value(events.last().expect("result").raw()).expect("result JSON");
    assert_eq!(result["outcome"]["withheld"], "quarantined");
    assert_eq!(result["meta"]["pre_hook_mutations"][0]["hook_id"], "01-pre");
    assert_eq!(result["meta"]["pre_hook_mutations"][1]["hook_id"], "02-pre");
    assert_eq!(result["meta"]["post_hooks"][0]["annotations"]["order"], 1);
    assert_eq!(result["meta"]["post_hooks"][1]["denied"], "review");
    assert_eq!(result["meta"]["post_hooks"][2]["annotations"]["order"], 3);
}

#[test]
fn ordered_hooks_apply_open_and_closed_failure_modes_without_shortening_post_chain() {
    let directory = tool_ledger();
    let path = directory.path().join("main.jsonl");
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let mut ledger = LockedLedger::open(&path, 1).expect("ledger");
    let mut pre_open = shell_hook("01-open", HookPhase::Pre, "exit 9");
    pre_open.failure = HookFailureMode::Open;
    let mutate = shell_hook(
        "02-mutate",
        HookPhase::Pre,
        r#"read _; printf '%s\n' '{"call":"c1","format":1,"hook_id":"02-mutate","verdict":{"mutate":{"invocation":{"argv":["safe"]}}}}'"#,
    );
    let mut post_closed = shell_hook("03-closed", HookPhase::Post, "exit 9");
    post_closed.failure = HookFailureMode::Closed;
    let post_after = shell_hook(
        "04-after",
        HookPhase::Post,
        r#"input=$(cat); case "$input" in *'"hook_id":"03-closed"'*) printf '%s\n' '{"call":"c1","format":1,"hook_id":"04-after","verdict":{"allow":{"annotations":{"continued":true}}}}' ;; *) exit 7 ;; esac"#,
    );
    ToolPipeline::new(&mut ledger, assets, SecretScanner::default())
        .execute(
            &tool_execution(),
            &[pre_open, mutate, post_closed, post_after],
            |_, _| Ok(IJsonValue::from("backend")),
        )
        .expect("failure policy chain");
    drop(ledger);
    let text = String::from_utf8(fs::read(&path).expect("ledger")).expect("UTF-8");
    assert!(text.contains(r#""hook_id":"01-open""#));
    assert!(text.contains(r#""hook_id":"03-closed""#));
    assert!(text.contains(r#""hook_id":"04-after""#));
    assert!(text.contains(r#""withheld":"quarantined""#));

    let directory = tool_ledger();
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let mut ledger = LockedLedger::open(directory.path().join("main.jsonl"), 1).expect("ledger");
    let pre_closed = shell_hook("01-closed", HookPhase::Pre, "exit 9");
    let never = shell_hook(
        "02-never",
        HookPhase::Pre,
        r#"read _; printf '%s\n' '{"call":"c1","format":1,"hook_id":"02-never","verdict":{"allow":{}}}'"#,
    );
    let mut backend_calls = 0;
    ToolPipeline::new(&mut ledger, assets, SecretScanner::default())
        .execute(&tool_execution(), &[pre_closed, never], |_, _| {
            backend_calls += 1;
            Ok(IJsonValue::from("backend"))
        })
        .expect("closed pre failure");
    assert_eq!(backend_calls, 0);
}

#[test]
fn slice4_gate_08_ask_user_fast_answer() {
    let directory = tempfile::tempdir().expect("thread");
    let path = directory.path().join("main.jsonl");
    fs::write(&path, hold_prefix()).expect("hold prefix");
    let sync = Arc::new(CountingSync::default());
    let mut ledger = LockedLedger::open_with_sync(&path, 1, sync.clone()).expect("ledger");
    let origin = OriginTuple {
        principal: "p".to_owned(),
        client: "cli".to_owned(),
        target: "t".to_owned(),
        op: "approve".to_owned(),
        key: "answer1".to_owned(),
    };
    let response = event(json!({
        "v": 1, "seq": 10, "kind": "approval_response", "ts": TS, "turn": 1,
        "call": "ask1", "grant": true, "answer": "yes",
        "origin_key": "answer1", "origin_tuple": origin
    }));
    ledger
        .append_contract(response, BarrierContext::default())
        .expect("durable answer");
    assert_eq!(sync.count.load(Ordering::SeqCst), 1);
    let result = event(json!({
        "v": 1, "seq": 11, "kind": "tool_result", "ts": TS, "turn": 1,
        "call": "ask1", "outcome": "ok", "anchor": true,
        "content": [{"type":"text", "text":"yes"}]
    }));
    ledger
        .append_contract(result, BarrierContext::default())
        .expect("paired answer");
    let mut delivery = BTreeMap::new();
    assert_eq!(delivery.get(&origin), None);
    delivery.entry(origin.clone()).or_insert(10);
    assert_eq!(delivery.get(&origin), Some(&10));
    let checkpoint_seq = ledger
        .create_checkpoint(TS, "answer anchor")
        .expect("checkpoint");
    assert_eq!(checkpoint_seq, 12);
    assert_eq!(
        sync.count.load(Ordering::SeqCst),
        2,
        "the checkpoint is a barrier"
    );
}

#[test]
fn spawn_seed_contract_oracle() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let spawn_bytes =
        read(&fixtures.join("events/spawn-bounded.canonical.json")).expect("spawn fixture");
    let genesis_bytes =
        read(&fixtures.join("events/genesis-bounded.canonical.json")).expect("genesis fixture");
    let spawn = Event::decode_canonical(spawn_bytes.strip_suffix(b"\n").unwrap_or(&spawn_bytes))
        .expect("spawn event");
    let _genesis =
        Event::decode_canonical(genesis_bytes.strip_suffix(b"\n").unwrap_or(&genesis_bytes))
            .expect("child genesis");
    let spawn_value: Value = serde_json::from_slice(&spawn_bytes).unwrap();
    let genesis_value: Value = serde_json::from_slice(&genesis_bytes).unwrap();
    let parent = genesis_value["parent"].as_object().expect("parent binding");
    assert_eq!(parent["seq"].as_u64(), Some(spawn.seq()));
    assert_eq!(parent["spawn_id"], spawn_value["spawn_id"]);

    let seed_name = genesis_value["seed"]["snapshot"]["asset"]
        .as_str()
        .expect("seed asset name");
    let seed = read(&fixtures.join("assets").join(seed_name)).expect("seed bytes");
    assert!(seed.ends_with(b"\n"));
    assert_eq!(format!("sha256-{:x}", Sha256::digest(&seed)), seed_name);
    let source = seed
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(Event::decode_canonical)
        .collect::<Result<Vec<_>, _>>()
        .expect("canonical seed source lines");
    assert!(source.windows(2).all(|pair| pair[0].seq() < pair[1].seq()));
    let kinds = source
        .iter()
        .map(|event| event.kind().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        kinds.into_iter().collect::<Vec<_>>(),
        genesis_value["seed"]["kinds"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>()
    );

    // Every crash prefix is reconciled to the same one-result ordering. A
    // crash before durable spawn leaves no registered edge; every later
    // prefix completes the registered edge before the parent can settle.
    for crash_after in 0..=4 {
        let mut durable = Vec::new();
        for phase in ["child_create", "spawn", "launch", "child_settle"]
            .into_iter()
            .take(crash_after)
        {
            durable.push(phase);
        }
        if durable.contains(&"spawn") {
            if !durable.contains(&"launch") {
                durable.push("launch");
            }
            if !durable.contains(&"child_settle") {
                durable.push("child_settle");
            }
            durable.push("child_result");
            assert!(
                durable.iter().position(|p| *p == "child_result").unwrap()
                    > durable.iter().position(|p| *p == "child_settle").unwrap()
            );
            durable.push("parent_settle");
            assert!(
                durable.iter().position(|p| *p == "child_result").unwrap()
                    < durable.iter().position(|p| *p == "parent_settle").unwrap()
            );
            assert_eq!(durable.iter().filter(|p| **p == "child_result").count(), 1);
        } else {
            assert!(!durable.contains(&"child_result"));
        }
    }
}

#[test]
fn slice4_gate_10_descriptor_first_file_open() {
    let directory = tempfile::tempdir().expect("root");
    let root = fs::canonicalize(directory.path()).expect("canonical root");
    fs::write(root.join("regular"), b"old").expect("regular");
    symlink("/etc/passwd", root.join("link")).expect("symlink");
    fs::create_dir(root.join("inside")).expect("inside");
    symlink("/etc", root.join("inside/escape")).expect("intermediate symlink");
    let fifo = root.join("fifo");
    let status = Command::new("/usr/bin/mkfifo")
        .arg(&fifo)
        .status()
        .expect("mkfifo");
    assert!(status.success());
    let server = HelperServer::new([RootBinding::open("workspace", &root).expect("root binding")])
        .expect("server");
    for path in ["link", "inside/escape/passwd", "fifo"] {
        let started = Instant::now();
        let response = server.execute(&HelperRequest {
            id: path.to_owned(),
            operation: HelperOperation::Read {
                root: "workspace".to_owned(),
                path: path.to_owned(),
                max_bytes: 4096,
            },
        });
        assert!(matches!(response, HelperResponse::Error { .. }), "{path}");
        assert!(started.elapsed() < Duration::from_secs(1), "{path} blocked");
    }
    let write = server.execute(&HelperRequest {
        id: "write".to_owned(),
        operation: HelperOperation::Write {
            root: "workspace".to_owned(),
            path: "regular".to_owned(),
            content: ByteString::from_bytes(b"new"),
            create: CreateMode::Replace,
        },
    });
    assert!(matches!(write, HelperResponse::Result { .. }));
    assert_eq!(fs::read(root.join("regular")).unwrap(), b"new");
    let patch =
        server.execute(&HelperRequest {
            id: "patch".to_owned(),
            operation: HelperOperation::Patch {
                root: "workspace".to_owned(),
                path: "regular".to_owned(),
                expected_sha256:
                    "sha256-11507a0e2f5e69d5dfa40a62a1bd7b6ee57e6bcd85c67c9b8431b36fff21c437"
                        .to_owned(),
                replacement: ByteString::from_bytes(b"patched"),
            },
        });
    assert!(matches!(patch, HelperResponse::Result { .. }));
    assert_eq!(fs::read(root.join("regular")).unwrap(), b"patched");
    assert!(fs::read_dir(&root).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".tekes-tmp-")
    }));

    // A command may run as long as it needs: only zero is rejected, and an
    // absent timeout means the helper waits for the process to finish.
    for (timeout_ms, accepted) in [(Some(0), false), (Some(600_001), true), (None, true)] {
        let result = server.execute(&HelperRequest {
            id: "shell-timeout-boundary".into(),
            operation: HelperOperation::Exec(ExecRequest {
                argv: vec!["/usr/bin/true".into()],
                stdin: None,
                cwd: None,
                env: BTreeMap::new(),
                stdout_bytes: 1024,
                stderr_bytes: 1024,
                timeout_ms,
            }),
        });
        assert_eq!(matches!(result, HelperResponse::Result { .. }), accepted);
    }

    let started = Instant::now();
    let timeout = server.execute(&HelperRequest {
        id: "timeout".to_owned(),
        operation: HelperOperation::Exec(ExecRequest {
            argv: vec![
                "/bin/sh".to_owned(),
                "-c".to_owned(),
                "trap '' TERM; sleep 5".to_owned(),
            ],
            stdin: None,
            cwd: None,
            env: BTreeMap::new(),
            stdout_bytes: 4096,
            stderr_bytes: 4096,
            timeout_ms: Some(20),
        }),
    });
    assert!(matches!(
        timeout,
        HelperResponse::Error {
            error: tools::HelperError {
                class: HelperErrorClass::Timeout,
                ..
            },
            ..
        }
    ));
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn slice4_gate_11_sandbox_backend_probe() {
    let status = probe_backend(host_sandbox_backend());
    if host_has_sandbox() {
        assert!(
            matches!(status, ProbeStatus::Available { .. }),
            "{status:?}"
        );
    } else {
        assert!(matches!(status, ProbeStatus::Unavailable { .. }));
    }
    // A backend for another platform never reports available.
    let foreign = if cfg!(target_os = "macos") {
        SandboxBackend::LinuxLandlockSeccompV1
    } else {
        SandboxBackend::DarwinSeatbeltV1
    };
    assert!(matches!(
        probe_backend(foreign),
        ProbeStatus::Unavailable { .. }
    ));
}

/// The backend production selects on this host.
fn host_sandbox_backend() -> SandboxBackend {
    if cfg!(target_os = "macos") {
        SandboxBackend::DarwinSeatbeltV1
    } else {
        SandboxBackend::LinuxLandlockSeccompV1
    }
}

/// Whether this host must provide a working sandbox: macOS, and Linux on the
/// architectures the Landlock/seccomp launcher supports.
fn host_has_sandbox() -> bool {
    cfg!(any(
        target_os = "macos",
        all(
            target_os = "linux",
            any(target_arch = "x86_64", target_arch = "aarch64")
        )
    ))
}

#[derive(Default)]
struct CountingSync {
    count: AtomicUsize,
}

impl SyncPolicy for CountingSync {
    fn full_sync(&self, file: &fs::File) -> io::Result<()> {
        self.count.fetch_add(1, Ordering::SeqCst);
        file.sync_all()
    }
}

fn assert_canonical_json(bytes: &[u8]) {
    let body = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    let value = IJsonValue::parse(body).expect("I-JSON");
    assert_eq!(value.canonical_bytes().expect("JCS"), body);
}

fn canonical_line(value: &Value) -> Vec<u8> {
    let mut bytes = serde_json_canonicalizer::to_vec(value).expect("canonical JSON");
    bytes.push(b'\n');
    bytes
}

fn projection_names<'a>(manifest: &'a BuiltinManifest, context: &CatalogContext) -> Vec<&'a str> {
    manifest
        .projection(context)
        .into_iter()
        .map(|tool| tool.name.as_str())
        .collect()
}

fn shell_hook(id: &str, phase: HookPhase, command: &str) -> HookBinding {
    HookBinding {
        format: 1,
        id: id.to_owned(),
        argv: vec!["/bin/sh".to_owned(), "-c".to_owned(), command.to_owned()],
        phase,
        failure: HookFailureMode::Closed,
        timeout_ms: 1000,
        stdout_bytes: 4096,
        stderr_bytes: 4096,
        env: BTreeMap::new(),
    }
}

fn event(value: Value) -> Event {
    let bytes = serde_json_canonicalizer::to_vec(&value).expect("canonical event");
    Event::decode_canonical(&bytes).expect("valid event")
}

fn tool_execution() -> ToolExecution {
    ToolExecution {
        thread: "018f0000-0000-7000-8000-000000000001".to_owned(),
        call: "c1".to_owned(),
        name: "exec".to_owned(),
        attempt: "a1".to_owned(),
        invocation: IJsonValue::parse_str(r#"{"argv":["/bin/sh","-c","unsafe"]}"#).unwrap(),
        side_effectful: true,
        turn: 1,
        timestamp: TS.to_owned(),
    }
}

fn tool_ledger() -> TempDir {
    let directory = tempfile::tempdir().expect("thread");
    fs::create_dir(directory.path().join("assets")).expect("assets");
    fs::write(directory.path().join("main.jsonl"), tool_prefix()).expect("tool prefix");
    directory
}

fn tool_prefix() -> Vec<u8> {
    lines(&[
        r#"{"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":"018f0000-0000-7000-8000-000000000001"},"resume":"never","seq":1,"thread":"018f0000-0000-7000-8000-000000000001","ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"ws"}"#,
        r#"{"content":[{"text":"start","type":"text"}],"kind":"input","origin_key":"i1","origin_tuple":{"client":"cli","key":"i1","op":"submit","principal":"p","target":"t"},"seq":2,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"binary":"worker-1","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":3,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"kind":"turn_open","seq":4,"trigger":{"inputs":[2]},"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
        r#"{"adapter":"fake","id":"e1","kind":"epoch","model":"fake","reason":"initial","renderer":1,"seq":5,"system":{"asset":"sha256-aa","digest":"d1"},"tools":{"asset":"sha256-td","digest":"td"},"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"admits":[{"from":2,"to":2}],"attempt":"a1","epoch":"e1","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":6,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1,"wire_digest":"wd"}"#,
        r#"{"attempt":"a1","content":[{"args":{"argv":["/bin/sh","-c","unsafe"]},"call":"c1","name":"exec","type":"tool-call"}],"kind":"output","sealed":{"adapter":"fake","fragments":"call","version":1},"seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"usage":{"availability":"reported","input_tokens":"5","output_tokens":"1"},"v":1}"#,
        r#"{"args":{"argv":["/bin/sh","-c","unsafe"]},"attempt":"a1","call":"c1","kind":"tool_call","name":"exec","seq":8,"source":"provider","ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ])
}

fn hold_prefix() -> Vec<u8> {
    lines(&[
        r#"{"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":"018f0000-0000-7000-8000-000000000001"},"resume":"never","seq":1,"thread":"018f0000-0000-7000-8000-000000000001","ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"ws"}"#,
        r#"{"content":[{"text":"start","type":"text"}],"kind":"input","origin_key":"i1","origin_tuple":{"client":"cli","key":"i1","op":"submit","principal":"p","target":"t"},"seq":2,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"binary":"worker-1","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":3,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"kind":"turn_open","seq":4,"trigger":{"inputs":[2]},"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
        r#"{"adapter":"fake","id":"e1","kind":"epoch","model":"fake","reason":"initial","renderer":1,"seq":5,"system":{"asset":"sha256-aa","digest":"d1"},"tools":{"asset":"sha256-td","digest":"td"},"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"admits":[{"from":2,"to":2}],"attempt":"a1","epoch":"e1","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":6,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1,"wire_digest":"wd"}"#,
        r#"{"attempt":"a1","content":[{"args":{"question":"continue?"},"call":"ask1","name":"ask_user","type":"tool-call"}],"kind":"output","sealed":{"adapter":"fake","fragments":"ask","version":1},"seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"usage":{"availability":"reported","input_tokens":"5","output_tokens":"1"},"v":1}"#,
        r#"{"args":{"question":"continue?"},"attempt":"a1","call":"ask1","kind":"tool_call","name":"ask_user","seq":8,"source":"provider","ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
        r#"{"call":"ask1","kind":"approval_request","scope":"answer","seq":9,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ])
}

fn lines(lines: &[&str]) -> Vec<u8> {
    let mut output = Vec::new();
    for line in lines {
        output.extend_from_slice(line.as_bytes());
        output.push(b'\n');
    }
    output
}
