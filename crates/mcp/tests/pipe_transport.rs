use mcp::{JsonRpcRequest, McpError, McpTransport, PipeTransport};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;
use std::time::Duration;

fn pair() -> (PipeTransport, UnixStream) {
    let (client, server) = UnixStream::pair().unwrap();
    let input = File::from(OwnedFd::from(client.try_clone().unwrap()));
    let output = File::from(OwnedFd::from(client));
    server
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    (PipeTransport::new(input, output).unwrap(), server)
}
fn request() -> JsonRpcRequest {
    JsonRpcRequest::call(1, "ping", None)
}
#[tokio::test]
async fn coalesced_notifications_and_response_preserve_framing() {
    let (mut transport, mut server) = pair();
    let peer = std::thread::spawn(move || {
        let mut reader = BufReader::new(server.try_clone().unwrap());
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&line).unwrap()["method"],
            "ping"
        );
        server.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/tools/list_changed\"}\r\n{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}\n").unwrap();
    });
    transport
        .request(&request(), Duration::from_secs(2))
        .await
        .unwrap();
    assert_eq!(
        transport.take_notifications(),
        vec!["notifications/tools/list_changed"]
    );
    transport.close().await.unwrap();
    peer.join().unwrap();
}
#[tokio::test]
async fn malformed_response_closes_only_the_owned_streams() {
    let (mut transport, mut server) = pair();
    let peer = std::thread::spawn(move || {
        let mut reader = BufReader::new(server.try_clone().unwrap());
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        server
            .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":9,\"result\":{}}\n")
            .unwrap();
        let mut byte = [0];
        assert_eq!(server.read(&mut byte).unwrap(), 0);
    });
    assert!(matches!(
        transport.request(&request(), Duration::from_secs(2)).await,
        Err(McpError::Protocol(_))
    ));
    peer.join().unwrap();
}
#[tokio::test]
async fn partial_frame_timeout_cannot_be_reused_as_a_response() {
    let (mut transport, mut server) = pair();
    let peer = std::thread::spawn(move || {
        let mut reader = BufReader::new(server.try_clone().unwrap());
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        server.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":1,").unwrap();
        let mut byte = [0];
        assert_eq!(server.read(&mut byte).unwrap(), 0);
    });
    assert!(matches!(
        transport
            .request(&request(), Duration::from_millis(100))
            .await,
        Err(McpError::Timeout(_))
    ));
    assert!(
        transport
            .request(&request(), Duration::from_millis(100))
            .await
            .is_err()
    );
    peer.join().unwrap();
}
#[tokio::test]
async fn oversized_frame_is_bounded_and_the_channel_is_closed() {
    let (mut transport, mut server) = pair();
    let peer = std::thread::spawn(move || {
        let mut reader = BufReader::new(server.try_clone().unwrap());
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let _ = server.write_all(&vec![b'x'; mcp::MAX_FRAME_BYTES + 8192]);
    });
    assert!(matches!(
        transport.request(&request(), Duration::from_secs(3)).await,
        Err(McpError::Protocol(_))
    ));
    peer.join().unwrap();
}
