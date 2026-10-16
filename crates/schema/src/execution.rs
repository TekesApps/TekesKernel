//! Session-owned execution input, independent of named management registries.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionConfig {
    pub cwd: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp: Vec<McpConnection>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub read_roots: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub writable_roots: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub toolchain_roots: Vec<String>,
    #[serde(default)]
    pub network: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_tools: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_wall_seconds: Option<u64>,
}

impl ExecutionConfig {
    pub fn validate(&self) -> Result<(), &'static str> {
        for path in std::iter::once(&self.cwd)
            .chain(&self.read_roots)
            .chain(&self.writable_roots)
            .chain(&self.toolchain_roots)
        {
            if !std::path::Path::new(path).is_absolute() || path.contains('\0') {
                return Err("execution paths must be absolute and contain no NUL");
            }
        }
        if self
            .max_wall_seconds
            .is_some_and(|value| value == 0 || value > 9_007_199_254_740_991)
        {
            return Err("execution max_wall_seconds must be a positive safe integer");
        }
        let mut names = std::collections::BTreeSet::new();
        for connection in &self.mcp {
            connection.validate()?;
            if !names.insert(&connection.name) {
                return Err("duplicate MCP connection name");
            }
        }
        Ok(())
    }
}

/// A ready host connection, never a process or a management scope. Secrets
/// stay in the private credential channel; only a reference is durable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpConnection {
    pub name: String,
    pub transport: McpConnectionTransport,
}
impl McpConnection {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.name.is_empty()
            || self.name.len() > 128
            || !self
                .name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err("invalid MCP connection name");
        }
        match &self.transport {
            McpConnectionTransport::Http { url, credential_id } => {
                let url = url::Url::parse(url).map_err(|_| "invalid MCP URL")?;
                let loopback = url.host().is_some_and(|host| match host {
                    url::Host::Domain(name) => name == "localhost",
                    url::Host::Ipv4(address) => address.is_loopback(),
                    url::Host::Ipv6(address) => address.is_loopback(),
                });
                if (url.scheme() != "https" && !(url.scheme() == "http" && loopback))
                    || url.host().is_none()
                    || !url.username().is_empty()
                    || url.password().is_some()
                    || url.fragment().is_some()
                    || url.query().is_some()
                {
                    return Err(
                        "MCP URL requires HTTPS or loopback HTTP without embedded credentials, query or fragment",
                    );
                }
                if credential_id.as_ref().is_some_and(|id| {
                    id.is_empty() || id.len() > 512 || id.chars().any(char::is_control)
                }) {
                    return Err("invalid MCP credential reference");
                }
            }
            McpConnectionTransport::Stdio { channel } => {
                if channel.is_empty()
                    || channel.len() > 128
                    || !channel
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
                {
                    return Err("invalid stdio channel");
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum McpConnectionTransport {
    Http {
        url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        credential_id: Option<String>,
    },
    Stdio {
        channel: String,
    },
}
