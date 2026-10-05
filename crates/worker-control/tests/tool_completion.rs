use serde_json::json;
use worker_control::Frame;

#[test]
fn completion_marker_is_optional_and_tool_only() {
    let mut value =
        json!({"attempt":"a1","channel":"tool","block":0,"delta":"","call_id":"c1","name":"read"});
    let legacy: Frame = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(legacy.arguments_complete, None);
    assert!(legacy.validate().is_ok());
    value["arguments_complete"] = json!(true);
    let ready: Frame = serde_json::from_value(value.clone()).unwrap();
    assert!(ready.validate().is_ok());
    assert_eq!(
        serde_json::to_value(ready).unwrap()["arguments_complete"],
        true
    );
    value["channel"] = json!("text");
    value.as_object_mut().unwrap().remove("call_id");
    value.as_object_mut().unwrap().remove("name");
    assert!(
        serde_json::from_value::<Frame>(value)
            .unwrap()
            .validate()
            .is_err()
    );
}
