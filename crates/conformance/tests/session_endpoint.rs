use std::collections::BTreeSet;

use endpoint::{
    MuxHostDescription, SESSION_ENDPOINT_PUBLIC_METHODS, SessionEndpointCapability,
    SessionMuxClientFrame, SessionMuxServerFrame,
};

#[test]
fn public_route_authority_is_single_and_has_no_legacy_reads() {
    assert_eq!(transport::ROUTE_REGISTRY, SESSION_ENDPOINT_PUBLIC_METHODS);
    let public = transport::ROUTE_REGISTRY
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(public.len(), 17);
    assert!(public.contains("models.list"));
    assert!(public.contains("remote.mux"));
    for removed in [
        "host.describe",
        "workspace.list",
        "session.list",
        "session.history",
        "events.mux",
        "events.host",
        "respond",
    ] {
        assert!(
            !public.contains(removed),
            "legacy method remained: {removed}"
        );
    }
}

#[test]
fn canonical_mux_fixture_is_typed_on_both_wire_directions() {
    let bytes = include_bytes!("../../../fixtures/endpoint-mux/cases.canonical.json");
    let value: serde_json::Value = serde_json::from_slice(bytes).expect("fixture JSON");
    assert_eq!(
        serde_json_canonicalizer::to_vec(&value).expect("canonical JSON"),
        bytes.strip_suffix(b"\n").unwrap_or(bytes)
    );

    let description: MuxHostDescription =
        serde_json::from_value(value["description"].clone()).expect("description");
    description.validate().expect("V3 description");
    assert!(
        description
            .capabilities
            .contains(&SessionEndpointCapability::SessionControlSync)
    );
    assert_eq!(description.attached_sessions, 1);

    for client in [
        &value["workspace"]["open"],
        &value["journal"]["open"],
        &value["journal"]["page"],
        &value["actionable"]["client"],
    ] {
        let frame: SessionMuxClientFrame =
            serde_json::from_value(client.clone()).expect("client frame");
        frame.validate().expect("valid client frame");
    }
    for server in [
        &value["ready"],
        &value["workspace"]["baseline"],
        &value["journal"]["snapshot"],
        &value["actionable"]["server"],
    ] {
        let _: SessionMuxServerFrame =
            serde_json::from_value(server.clone()).expect("server frame");
    }
    assert_eq!(value["actionable"]["client"]["expectedRevision"], 0);
    assert_eq!(
        value["workspace"]["baseline"]["frame"]["baseline"]["items"][0]["createdAt"],
        "2026-09-01T00:00:00.000Z"
    );
}
