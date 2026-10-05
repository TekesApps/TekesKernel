use schema::Event;

const RUN_START: &[u8] =
    include_bytes!("../../../fixtures/launch-bindings/run-start.canonical.json");

#[test]
fn run_start_durably_attributes_launch_binding_bytes() {
    let body = RUN_START.strip_suffix(b"\n").expect("one final LF");
    let event = Event::decode(body).expect("run_start fixture");
    assert_eq!(
        event.string_field("launch_bindings_digest"),
        Some("d5420074f9b113c806c8ef51514bb2b10fbdd1808c11de971fbef99a03baa2df")
    );
    assert_eq!(event.canonical_bytes().expect("canonical event"), body);

    let invalid = String::from_utf8(body.to_vec()).expect("UTF-8").replace(
        "d5420074f9b113c806c8ef51514bb2b10fbdd1808c11de971fbef99a03baa2df",
        "UPPER",
    );
    assert!(Event::decode(invalid.as_bytes()).is_err());
}
