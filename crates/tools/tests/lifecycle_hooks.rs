use serde_json::json;
use std::collections::BTreeMap;
use tools::*;

fn binding(event: LifecycleEvent, script: &str) -> LifecycleHookBinding {
    LifecycleHookBinding {
        format: 2,
        id: "memory.json".into(),
        event,
        enabled: true,
        argv: vec!["/usr/bin/python3".into(), "-c".into(), script.into()],
        timeout_ms: 1000,
        stdout_bytes: 65536,
        stderr_bytes: 4096,
        env: BTreeMap::new(),
    }
}
fn request(event: LifecycleEvent) -> LifecycleHookRequest {
    LifecycleHookRequest {
        format: 2,
        hook_id: "memory.json".into(),
        event_id: "event-1".into(),
        event,
        workspace_id: "ws".into(),
        thread_id: "thread".into(),
        turn_id: 1,
        data: schema::IJsonValue::from("context"),
    }
}
const REPLY: &str = "import sys,json; r=json.load(sys.stdin); print(json.dumps(dict(format=2,hook_id=r['hook_id'],event_id=r['event_id'],context=['project fact']),sort_keys=True,separators=(',',':')))";

#[test]
fn context_hook_returns_material_but_observer_cannot_mutate() {
    let point = LifecycleEvent::ContextPrepare;
    let result = run_lifecycle_hook(&binding(point, REPLY), &request(point)).unwrap();
    assert_eq!(result.context, vec!["project fact"]);
    for point in [
        LifecycleEvent::TurnBefore,
        LifecycleEvent::ToolCompleted,
        LifecycleEvent::ContextBeforeCompact,
        LifecycleEvent::TurnSettled,
    ] {
        assert!(run_lifecycle_hook(&binding(point, REPLY), &request(point)).is_err());
    }
}

#[test]
fn correlation_environment_and_closed_schemas() {
    let point = LifecycleEvent::ContextPrepare;
    let script = "import sys,json,os; r=json.load(sys.stdin); assert 'HOME' not in os.environ; print(json.dumps(dict(format=2,hook_id=r['hook_id'],event_id='wrong'),sort_keys=True,separators=(',',':')))";
    assert!(run_lifecycle_hook(&binding(point, script), &request(point)).is_err());
    let good = binding(point, REPLY);
    let bytes = encode_hook_line(&good).unwrap();
    validate_instruction_hook(&bytes, "memory.json").unwrap();
    assert!(validate_instruction_hook(&bytes, "other.json").is_err());
    let mut raw = serde_json::to_value(good).unwrap();
    raw["event"] = json!("memory.special");
    assert!(validate_instruction_hook(&encode_hook_line(&raw).unwrap(), "memory.json").is_err());
    raw["event"] = json!("context.prepare");
    raw["failure"] = json!("closed");
    assert!(validate_instruction_hook(&encode_hook_line(&raw).unwrap(), "memory.json").is_err());
}

#[test]
fn deadline_covers_blocked_stdin_and_inherited_pipes() {
    let point = LifecycleEvent::ContextPrepare;
    let mut hook = binding(point, "import time; time.sleep(10)");
    hook.timeout_ms = 100;
    let mut req = request(point);
    req.data = schema::IJsonValue::from("x".repeat(1024 * 1024));
    let start = std::time::Instant::now();
    assert!(run_lifecycle_hook(&hook, &req).is_err());
    assert!(start.elapsed() < std::time::Duration::from_secs(3));
    hook.argv = vec!["/bin/sh".into(), "-c".into(), "sleep 10 & exit 0".into()];
    let start = std::time::Instant::now();
    assert!(run_lifecycle_hook(&hook, &request(point)).is_err());
    assert!(start.elapsed() < std::time::Duration::from_secs(3));
}

#[test]
fn cancelled_hook_exits_promptly_and_disabled_binding_never_executes() {
    let point = LifecycleEvent::ContextPrepare;
    let hook = binding(point, "import time; time.sleep(10)");
    let start = std::time::Instant::now();
    assert!(
        run_lifecycle_hook_with_cancel(&hook, &request(point), &|| start.elapsed().as_millis()
            > 50)
        .is_err()
    );
    assert!(start.elapsed() < std::time::Duration::from_secs(2));
    let mut disabled = hook;
    disabled.enabled = false;
    assert!(run_lifecycle_hook(&disabled, &request(point)).is_err());
}
