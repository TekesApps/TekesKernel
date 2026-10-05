use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinManifest {
    pub format: u64,
    pub tools: Vec<BuiltinTool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinTool {
    pub arguments: Vec<String>,
    pub availability: Availability,
    pub backend: Backend,
    pub effect: Effect,
    pub name: String,
}

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
}

closed_enum!(Availability {
    Default,
    SelectionControlled,
    PlanMode,
    RoleCompactor,
    RoleSubagent,
    RoleValidator,
    ConditionalCredential,
    ConditionalDeferredCatalog,
});
closed_enum!(Backend {
    InProcess,
    WorkerHold,
    SupervisorControl,
    HelperFs,
    HelperExec,
    BackgroundExec,
    WorkerHttp
});
closed_enum!(Effect {
    None,
    ReadOnly,
    UserInteraction,
    GoalState,
    WorkspaceWrite,
    ExternalProcess,
    NetworkRead,
    ThreadControl,
    ChildSpawn
});

#[derive(Clone, Copy)]
struct BuiltinDescriptor {
    name: &'static str,
    arguments: &'static [&'static str],
    availability: Availability,
    backend: Backend,
    effect: Effect,
}

macro_rules! descriptor {
    ($name:literal, [$($argument:literal),* $(,)?], $availability:ident, $backend:ident, $effect:ident) => {
        BuiltinDescriptor {
            name: $name,
            arguments: &[$($argument),*],
            availability: Availability::$availability,
            backend: Backend::$backend,
            effect: Effect::$effect,
        }
    };
}

// The fixture is the byte authority, while this registry is the compiled
// implementation-side proof that every v1 descriptor has exactly one slot.
// A canonical, sorted manifest with 25 arbitrary replacements must not pass
// startup validation merely because its cardinality happens to match.
const BUILTIN_DESCRIPTORS: [BuiltinDescriptor; 25] = [
    descriptor!(
        "apply_patch",
        [
            "operation",
            "path",
            "diff",
            "expected_artifact_version",
            "summary"
        ],
        Default,
        HelperFs,
        WorkspaceWrite
    ),
    descriptor!(
        "ask_user_questions",
        ["question", "options"],
        SelectionControlled,
        WorkerHold,
        UserInteraction
    ),
    descriptor!(
        "context",
        [
            "operation",
            "thread_id",
            "message",
            "goal",
            "reason",
            "turn_id",
            "cursor",
            "limit",
            "query",
            "scope"
        ],
        Default,
        SupervisorControl,
        ThreadControl
    ),
    descriptor!(
        "context_get",
        ["record_ids", "thread_id", "reason", "cursor", "page_bytes"],
        Default,
        SupervisorControl,
        ReadOnly
    ),
    descriptor!(
        "edit",
        ["path", "old_text", "new_text"],
        Default,
        HelperFs,
        WorkspaceWrite
    ),
    descriptor!("glob", ["pattern", "path"], Default, HelperFs, ReadOnly),
    descriptor!(
        "grep",
        [
            "pattern",
            "path",
            "glob",
            "output_mode",
            "case_insensitive",
            "context_lines"
        ],
        Default,
        HelperExec,
        ReadOnly
    ),
    descriptor!(
        "job",
        [
            "action",
            "program",
            "args",
            "working_directory",
            "writable_paths",
            "job_id"
        ],
        Default,
        BackgroundExec,
        ExternalProcess
    ),
    descriptor!(
        "new_goal",
        ["goal", "completion_criteria", "reason"],
        SelectionControlled,
        InProcess,
        GoalState
    ),
    descriptor!("plan", ["plan"], PlanMode, WorkerHold, UserInteraction),
    descriptor!(
        "read",
        ["path", "offset", "limit"],
        Default,
        HelperFs,
        ReadOnly
    ),
    descriptor!(
        "report",
        ["result"],
        RoleSubagent,
        SupervisorControl,
        ThreadControl
    ),
    descriptor!(
        "set_goal_state",
        ["state", "progress", "reason", "user_action"],
        SelectionControlled,
        InProcess,
        GoalState
    ),
    descriptor!(
        "shell",
        [
            "working_directory",
            "max_duration_ms",
            "max_output_tokens",
            "command",
            "program",
            "args",
            "steps",
            "failure_policy",
            "writable_paths",
            "artifact_outputs"
        ],
        Default,
        HelperExec,
        ExternalProcess
    ),
    descriptor!("skill", ["skill"], SelectionControlled, InProcess, ReadOnly),
    descriptor!(
        "skill_explorer",
        ["query", "limit"],
        Default,
        InProcess,
        ReadOnly
    ),
    descriptor!(
        "subagent",
        ["brief", "report_back_session_id"],
        SelectionControlled,
        SupervisorControl,
        ChildSpawn
    ),
    descriptor!(
        "summary_artifact",
        ["continuation", "evidence_refs"],
        RoleCompactor,
        InProcess,
        None
    ),
    descriptor!(
        "task",
        [
            "task_name",
            "goal",
            "instructions",
            "input_sources",
            "output"
        ],
        SelectionControlled,
        SupervisorControl,
        ChildSpawn
    ),
    descriptor!("think", ["thought"], SelectionControlled, InProcess, None),
    descriptor!(
        "tool_search",
        ["query", "limit"],
        ConditionalDeferredCatalog,
        InProcess,
        ReadOnly
    ),
    descriptor!(
        "verify",
        ["covered_set", "verdict", "failures"],
        RoleValidator,
        InProcess,
        None
    ),
    descriptor!("web_fetch", ["url"], Default, WorkerHttp, NetworkRead),
    descriptor!(
        "web_search",
        ["query", "max_results", "topic"],
        ConditionalCredential,
        WorkerHttp,
        NetworkRead
    ),
    descriptor!(
        "write",
        ["path", "content"],
        Default,
        HelperFs,
        WorkspaceWrite
    ),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogRole {
    Ordinary,
    Plan,
    Compactor,
    Subagent,
    Validator,
    Benchmark,
}

#[derive(Clone, Debug)]
pub struct CatalogContext {
    pub role: CatalogRole,
    pub has_web_credential: bool,
    pub has_deferred_catalog: bool,
    pub selected_tools: BTreeSet<String>,
}

impl Default for CatalogContext {
    fn default() -> Self {
        Self {
            role: CatalogRole::Ordinary,
            has_web_credential: false,
            has_deferred_catalog: false,
            selected_tools: BTreeSet::new(),
        }
    }
}

impl BuiltinManifest {
    #[must_use]
    pub fn compiled() -> Self {
        Self {
            format: 1,
            tools: BUILTIN_DESCRIPTORS
                .iter()
                .map(|descriptor| BuiltinTool {
                    arguments: descriptor
                        .arguments
                        .iter()
                        .map(|value| (*value).to_owned())
                        .collect(),
                    availability: descriptor.availability,
                    backend: descriptor.backend,
                    effect: descriptor.effect,
                    name: descriptor.name.to_owned(),
                })
                .collect(),
        }
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self, BuiltinError> {
        let body = bytes
            .strip_suffix(b"\n")
            .ok_or(BuiltinError::Invalid("manifest requires one final LF"))?;
        if body.is_empty() || body.contains(&b'\n') || body.contains(&b'\r') {
            return Err(BuiltinError::Invalid(
                "manifest must be exactly one JSON line",
            ));
        }
        let value = schema::IJsonValue::parse(body)
            .map_err(|_| BuiltinError::Invalid("manifest is not I-JSON"))?;
        if value
            .canonical_bytes()
            .map_err(|_| BuiltinError::Invalid("manifest cannot be canonicalized"))?
            != body
        {
            return Err(BuiltinError::Invalid("manifest is not RFC-8785 canonical"));
        }
        let manifest: Self = serde_json::from_slice(body)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), BuiltinError> {
        if self.format != 1 {
            return Err(BuiltinError::Invalid("format must equal 1"));
        }
        if self.tools.len() != BUILTIN_DESCRIPTORS.len() {
            return Err(BuiltinError::Invalid(
                "fixed manifest must contain 25 tools",
            ));
        }
        for pair in self.tools.windows(2) {
            if pair[0].name.as_bytes() >= pair[1].name.as_bytes() {
                return Err(BuiltinError::Invalid(
                    "tool names must be unique UTF-8-byte sorted",
                ));
            }
        }
        for tool in &self.tools {
            validate_name(&tool.name)?;
            let mut arguments = BTreeSet::new();
            if tool.arguments.iter().any(|name| !arguments.insert(name)) {
                return Err(BuiltinError::Invalid("tool arguments must be unique"));
            }
        }
        for (tool, expected) in self.tools.iter().zip(BUILTIN_DESCRIPTORS) {
            if tool.name != expected.name
                || tool.arguments.len() != expected.arguments.len()
                || !tool
                    .arguments
                    .iter()
                    .map(String::as_str)
                    .eq(expected.arguments.iter().copied())
                || tool.availability != expected.availability
                || tool.backend != expected.backend
                || tool.effect != expected.effect
            {
                return Err(BuiltinError::Invalid(
                    "manifest does not match the compiled v1 descriptor registry",
                ));
            }
        }
        Ok(())
    }

    pub fn fixed_names(&self) -> BTreeSet<&str> {
        self.tools.iter().map(|tool| tool.name.as_str()).collect()
    }

    pub fn reject_dynamic_collisions<'a>(
        &self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), BuiltinError> {
        let fixed = self.fixed_names();
        let mut dynamic = BTreeSet::new();
        for name in names {
            validate_name(name)?;
            if fixed.contains(name) || !dynamic.insert(name) {
                return Err(BuiltinError::NameCollision(name.to_owned()));
            }
        }
        Ok(())
    }

    pub fn projection<'a>(&'a self, context: &CatalogContext) -> Vec<&'a BuiltinTool> {
        let visible = |tool: &&BuiltinTool| {
            availability_visible(tool, context)
                && (context.role != CatalogRole::Benchmark
                    || (tool.backend == Backend::InProcess && tool.effect != Effect::ChildSpawn))
        };
        self.tools
            .iter()
            .filter(visible)
            .filter(|tool| is_resident(tool.availability))
            .chain(
                self.tools
                    .iter()
                    .filter(visible)
                    .filter(|tool| !is_resident(tool.availability)),
            )
            .collect()
    }

    /// Names of every fixed tool an interactive session may select on its own:
    /// the manifest minus the role selectors (`plan`, `summary_artifact`,
    /// `verify`, `report`), which the worker grants by role and which conflict
    /// when more than one is selected. Sorted, so the result is a valid
    /// `allowed_tools` list as-is.
    #[must_use]
    pub fn interactive_names(&self) -> Vec<String> {
        let mut names = self
            .tools
            .iter()
            .filter(|tool| {
                !matches!(
                    tool.availability,
                    Availability::PlanMode
                        | Availability::RoleCompactor
                        | Availability::RoleSubagent
                        | Availability::RoleValidator
                )
            })
            .map(|tool| tool.name.clone())
            .collect::<Vec<_>>();
        names.sort();
        names.dedup();
        names
    }

    pub fn catalog_digest(&self) -> Result<String, BuiltinError> {
        let bytes = serde_json_canonicalizer::to_vec(&self.tools)?;
        Ok(format!("sha256-{:x}", Sha256::digest(bytes)))
    }
}

fn availability_visible(tool: &BuiltinTool, context: &CatalogContext) -> bool {
    match tool.availability {
        Availability::Default => true,
        Availability::SelectionControlled => context.selected_tools.contains(&tool.name),
        Availability::PlanMode => context.role == CatalogRole::Plan,
        Availability::RoleCompactor => context.role == CatalogRole::Compactor,
        Availability::RoleSubagent => context.role == CatalogRole::Subagent,
        Availability::RoleValidator => context.role == CatalogRole::Validator,
        Availability::ConditionalCredential => context.has_web_credential,
        Availability::ConditionalDeferredCatalog => context.has_deferred_catalog,
    }
}

fn is_resident(availability: Availability) -> bool {
    matches!(
        availability,
        Availability::Default
            | Availability::ConditionalCredential
            | Availability::ConditionalDeferredCatalog
    )
}

fn validate_name(name: &str) -> Result<(), BuiltinError> {
    if name.is_empty() || name.chars().any(|ch| ch.is_ascii_control()) {
        Err(BuiltinError::Invalid(
            "tool name is empty or contains an ASCII control",
        ))
    } else {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum BuiltinError {
    #[error("invalid builtin manifest: {0}")]
    Invalid(&'static str),
    #[error("tool name collision: {0}")]
    NameCollision(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub fn index_by_name(manifest: &BuiltinManifest) -> BTreeMap<&str, &BuiltinTool> {
    manifest
        .tools
        .iter()
        .map(|tool| (tool.name.as_str(), tool))
        .collect()
}
