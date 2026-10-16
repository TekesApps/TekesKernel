//! The fixture acts as the CLI host: it owns children and hands only streams
//! to Kernel. No registry, workspace, Folder trust or Tekes is involved.
use serde_json::{Value, json};
use std::fs::File;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use tekes_supervisor::mcp_runtime::{McpRuntime, StdioConnector, StdioOpen};
use tekes_supervisor::production_tool_control::DynamicSupervisorAuthority;

const SESSION: &str = "018f0000-0000-7000-8000-000000000091";
const OTHER: &str = "018f0000-0000-7000-8000-000000000092";
const SERVER: &str = r#"
import json,sys,os
for line in sys.stdin:
 r=json.loads(line)
 if 'id' not in r:continue
 method=r['method']
 if method=='initialize':
  result={'protocolVersion':'2025-11-25','capabilities':{'tools':{}},'serverInfo':{'name':'host-fixture','version':'1'}}
 elif method=='tools/list':
  result={'tools':[{'name':'echo','description':'Echo','inputSchema':{'type':'object','properties':{'value':{'type':'string'}},'required':['value'],'additionalProperties':False},'annotations':{'readOnlyHint':True}},{'name':'unsupported','description':'Unsupported sibling','inputSchema':{'type':'object','patternProperties':{'x':{'type':'string'}}}}]}
 elif method=='tools/call':
  result={'content':[{'type':'text','text':r['params']['arguments']['value']}],'structuredContent':{'pid':os.getpid()},'isError':False}
 else:result={}
 print(json.dumps({'jsonrpc':'2.0','id':r['id'],'result':result}),flush=True)
 if method=='tools/call' and r['params']['arguments']['value']=='exit':break
"#;

#[derive(Default)]
struct FixtureHost {
    children: Mutex<Vec<Child>>,
}
impl StdioConnector for FixtureHost {
    fn open<'a>(&'a self, _session: &'a str, channel: &'a str) -> StdioOpen<'a> {
        Box::pin(async move {
            assert_eq!(channel, "fixture-channel");
            let mut child = Command::new("/usr/bin/python3")
                .args(["-u", "-c", SERVER])
                .env_clear()
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?;
            let output = File::from(std::os::fd::OwnedFd::from(child.stdin.take().unwrap()));
            let input = File::from(std::os::fd::OwnedFd::from(child.stdout.take().unwrap()));
            self.children.lock().unwrap().push(child);
            Ok((input, output))
        })
    }
}
impl Drop for FixtureHost {
    fn drop(&mut self) {
        for child in self.children.get_mut().unwrap() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
fn connection() -> schema::McpConnection {
    serde_json::from_value(
        json!({"name":"fixture","transport":{"type":"stdio","channel":"fixture-channel"}}),
    )
    .unwrap()
}
fn call(
    binding: &tekes_supervisor::mcp_runtime::McpSessionBinding,
    session: &str,
    value: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let tool = binding
        .catalog
        .tools
        .iter()
        .find(|tool| tool.name == "mcp__fixture__echo")
        .unwrap();
    let request = worker_control::ToolControl::new(
        session,
        OTHER,
        1,
        "call-1",
        &tool.name,
        schema::IJsonValue::parse(&serde_json::to_vec(&json!({"value":value}))?)?,
    )?;
    let result = binding.authority.execute(tool, &request)?;
    Ok(serde_json::from_slice(&result.canonical_bytes()?)?)
}
#[test]
fn explicit_stdio_connections_are_isolated_and_refused_siblings_do_not_block_calls() {
    let host = Arc::new(FixtureHost::default());
    let runtime = McpRuntime::new(Arc::new(provider::MemorySecretStore::new())).unwrap();
    runtime.set_stdio_connector(host.clone());
    let first = runtime.prepare_session(SESSION, &[connection()]).unwrap();
    let second = runtime.prepare_session(OTHER, &[connection()]).unwrap();
    assert_eq!(first.catalog.tools.len(), 1);
    let root = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let repo = profile::ConfigRepository::open(root.path()).unwrap();
    let execution=repo.normalize_execution(&serde_json::from_value(json!({"cwd":project.path(),"allowed_tools":["mcp__fixture__echo","mcp__fixture__unsupported"]})).unwrap()).unwrap();
    let config = repo.resolve_execution(SESSION, &execution, None).unwrap();
    let instruction = profile::InstructionResolver::new(
        root.path().join("agents"),
        std::iter::empty::<&std::path::Path>(),
    )
    .capture()
    .unwrap();
    tekes_supervisor::dynamic_bindings::resolve_worker_launch_bindings(
        &config,
        &instruction,
        None,
        first.catalog.tools.clone(),
        first.unavailable_tools(&config.execution.policy.allowed_tools),
    )
    .expect("one refused allowed tool must not block its healthy sibling");

    assert!(
        first
            .unavailable_tools(&["mcp__fixture__echo".into()])
            .is_empty()
    );
    assert_eq!(
        first.unavailable_tools(&config.execution.policy.allowed_tools),
        ["mcp__fixture__unsupported"]
    );
    let mut unknown = config.clone();
    unknown
        .execution
        .policy
        .allowed_tools
        .push("mcp__fixture__unknown".into());
    unknown.execution.policy.allowed_tools.sort();
    assert!(
        tekes_supervisor::dynamic_bindings::resolve_worker_launch_bindings(
            &unknown,
            &instruction,
            None,
            first.catalog.tools.clone(),
            first.unavailable_tools(&unknown.execution.policy.allowed_tools)
        )
        .is_err()
    );
    let a = call(&first, SESSION, "one").unwrap();
    let b = call(&second, OTHER, "two").unwrap();
    assert_ne!(a["structuredContent"]["pid"], b["structuredContent"]["pid"]);
    assert_eq!(a["content"][0]["text"], "one");
    drop(first);
    assert_eq!(
        call(&second, OTHER, "still open").unwrap()["content"][0]["text"],
        "still open"
    );
}
#[test]
fn lost_stdio_peer_reopens_through_host_and_preserves_its_contract() {
    let host = Arc::new(FixtureHost::default());
    let runtime = McpRuntime::new(Arc::new(provider::MemorySecretStore::new())).unwrap();
    runtime.set_stdio_connector(host.clone());
    let binding = runtime.prepare_session(SESSION, &[connection()]).unwrap();
    let first = call(&binding, SESSION, "exit").unwrap();
    host.children.lock().unwrap()[0].wait().unwrap();
    assert!(call(&binding, SESSION, "lost").is_err());
    let resumed = call(&binding, SESSION, "reconnected").unwrap();
    assert_ne!(
        first["structuredContent"]["pid"],
        resumed["structuredContent"]["pid"]
    );
    assert_eq!(host.children.lock().unwrap().len(), 2);
}
#[test]
fn unbound_stdio_channels_fail_without_starting_a_process() {
    let runtime = McpRuntime::new(Arc::new(provider::MemorySecretStore::new())).unwrap();
    assert!(runtime.prepare_session(SESSION, &[connection()]).is_err());
}

struct HttpFixture {
    child: Child,
    directory: tempfile::TempDir,
    url: String,
}
impl HttpFixture {
    fn start() -> Self {
        use std::io::{BufRead, BufReader};
        let directory = tempfile::tempdir().unwrap();
        let script = r#"
import json,os,http.server
class Server(http.server.BaseHTTPRequestHandler):
 def do_POST(self):
  r=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
  with open(os.environ['REQUEST_LOG'],'a') as f:f.write(json.dumps({'method':r['method'],'authorization':self.headers.get('Authorization')})+'\n')
  if r['method']=='server/discover':result={'resultType':'complete','supportedVersions':['2026-07-28'],'capabilities':{'tools':{}},'_meta':{'io.modelcontextprotocol/serverInfo':{'name':'http-fixture','version':'1'}}}
  elif r['method']=='tools/list':result={'tools':[{'name':'echo','description':'Echo','inputSchema':{'type':'object','properties':{'value':{'type':'string'}},'required':['value'],'additionalProperties':False},'annotations':{'readOnlyHint':True}}]}
  elif r['method']=='tools/call':result={'content':[{'type':'text','text':r['params']['arguments']['value']}],'isError':False}
  else:result={}
  data=json.dumps({'jsonrpc':'2.0','id':r['id'],'result':result}).encode()
  self.send_response(200);self.send_header('Content-Type','application/json');self.send_header('Content-Length',str(len(data)));self.send_header('Connection','close');self.end_headers();self.wfile.write(data)
 def log_message(self,*args):pass
server=http.server.HTTPServer(('127.0.0.1',0),Server)
print(server.server_port,flush=True)
server.serve_forever()
"#;
        let mut child = Command::new("/usr/bin/python3")
            .args(["-u", "-c", script])
            .env_clear()
            .env("REQUEST_LOG", directory.path().join("requests.jsonl"))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        let port = line.trim().parse::<u16>().unwrap();
        Self {
            child,
            directory,
            url: format!("http://127.0.0.1:{port}/mcp"),
        }
    }
    fn connection(&self, credential: Option<&str>) -> schema::McpConnection {
        serde_json::from_value(json!({"name":"fixture","transport":{"type":"http","url":self.url,"credential_id":credential}})).unwrap()
    }
    fn requests(&self) -> Vec<Value> {
        std::fs::read_to_string(self.directory.path().join("requests.jsonl"))
            .unwrap_or_default()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }
}
impl Drop for HttpFixture {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
#[test]
fn http_connections_execute_without_any_registry_or_application() {
    let server = HttpFixture::start();
    let runtime = McpRuntime::new(Arc::new(provider::MemorySecretStore::new())).unwrap();
    let binding = runtime
        .prepare_session(SESSION, &[server.connection(None)])
        .unwrap();
    assert_eq!(
        call(&binding, SESSION, "http").unwrap()["content"][0]["text"],
        "http"
    );
    assert!(
        server
            .requests()
            .iter()
            .all(|request| request["authorization"].is_null())
    );
}
#[test]
fn host_http_grants_are_destination_bound_and_revocation_stops_new_calls() {
    let server = HttpFixture::start();
    let secrets = Arc::new(provider::MemorySecretStore::new());
    let publish = |id: &str, generation, material| {
        secrets
            .publish(
                id,
                provider::SecretRecord::Active {
                    generation,
                    material,
                },
            )
            .unwrap()
    };
    publish("provider-key", 1, "provider-secret-must-not-leak".into());
    publish(
        "mcp-key",
        1,
        json!({"kind":"mcp","url":server.url,"token":"mcp-secret-a"}).to_string(),
    );
    let runtime = McpRuntime::new(secrets.clone()).unwrap();
    assert!(
        runtime
            .prepare_session(SESSION, &[server.connection(Some("provider-key"))])
            .is_err()
    );
    let mut wrong = server.connection(Some("mcp-key"));
    if let schema::McpConnectionTransport::Http { url, .. } = &mut wrong.transport {
        url.push_str("/other");
    }
    assert!(runtime.prepare_session(SESSION, &[wrong]).is_err());
    assert!(
        server.requests().is_empty(),
        "no credentials may reach an ungranted target"
    );
    let binding = runtime
        .prepare_session(SESSION, &[server.connection(Some("mcp-key"))])
        .unwrap();
    call(&binding, SESSION, "authorized").unwrap();
    assert!(
        server
            .requests()
            .iter()
            .all(|request| request["authorization"] == "Bearer mcp-secret-a")
    );
    let count = server.requests().len();
    publish(
        "mcp-key",
        2,
        json!({"kind":"mcp","url":server.url,"token":"mcp-secret-b"}).to_string(),
    );
    assert!(call(&binding, SESSION, "rotated").is_err());
    assert_eq!(
        server.requests().len(),
        count,
        "stale authorization must not send another request"
    );
    let renewed = runtime
        .prepare_session(OTHER, &[server.connection(Some("mcp-key"))])
        .unwrap();
    call(&renewed, OTHER, "inherited connection").unwrap();
    assert!(
        server.requests()[count..]
            .iter()
            .all(|request| request["authorization"] == "Bearer mcp-secret-b")
    );
    let count = server.requests().len();
    secrets
        .publish("mcp-key", provider::SecretRecord::Revoked { generation: 3 })
        .unwrap();
    assert!(call(&renewed, OTHER, "revoked").is_err());
    assert_eq!(server.requests().len(), count);
}
