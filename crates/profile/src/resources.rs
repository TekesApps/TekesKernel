//! Immutable user/project skill packages and slash-command catalog.
//!
//! The resource catalog is derived exclusively from one already-validated
//! `InstructionSnapshot`. It never reopens mutable instruction directories.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{InstructionKind, InstructionOrigin, InstructionSnapshot, InstructionSource};

const MAX_NAME_BYTES: usize = 64;
const MAX_ARGUMENT_BYTES: usize = 64 * 1024;
const MAX_EXPANDED_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillResource {
    pub path: String,
    pub content: String,
    pub content_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: String,
    pub content_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillPackage {
    pub summary: SkillSummary,
    pub body: String,
    pub resources: Vec<SkillResource>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandSummary {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub source: String,
    pub content_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandCatalogEntry {
    pub summary: CommandSummary,
    pub body: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandExpansion {
    pub name: String,
    pub text: String,
    pub content_digest: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceCatalog {
    skills: BTreeMap<String, SkillPackage>,
    commands: BTreeMap<String, CommandCatalogEntry>,
}

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum ResourceError {
    #[error("resource path is invalid: {0}")]
    InvalidPath(String),
    #[error("resource name is invalid: {0}")]
    InvalidName(String),
    #[error("skill package is invalid: {0}")]
    InvalidSkill(String),
    #[error("command is invalid: {0}")]
    InvalidCommand(String),
    #[error("command arguments are invalid: {0}")]
    InvalidArguments(String),
    #[error("skill was not found: {0}")]
    SkillNotFound(String),
    #[error("skill resource was not found: {skill}/{path}")]
    SkillResourceNotFound { skill: String, path: String },
    #[error("command was not found: {0}")]
    CommandNotFound(String),
}

impl ResourceCatalog {
    pub fn from_snapshot(snapshot: &InstructionSnapshot) -> Result<Self, ResourceError> {
        let skills = build_skills(snapshot)?;
        let commands = build_commands(snapshot)?;
        Ok(Self { skills, commands })
    }

    #[must_use]
    pub fn skill_summaries(&self) -> Vec<SkillSummary> {
        self.skills
            .values()
            .map(|package| package.summary.clone())
            .collect()
    }

    pub fn skill(&self, name: &str) -> Result<&SkillPackage, ResourceError> {
        self.skills
            .get(name)
            .ok_or_else(|| ResourceError::SkillNotFound(name.to_owned()))
    }

    /// Stable package-order projection used by the versioned Client resource
    /// extension. The catalog is already one immutable instruction snapshot;
    /// callers must not reopen instruction files while paginating this view.
    pub fn skill_packages(&self) -> impl Iterator<Item = &SkillPackage> {
        self.skills.values()
    }

    pub fn skill_resource(&self, skill: &str, path: &str) -> Result<&str, ResourceError> {
        validate_relative(path)?;
        self.skill(skill)?
            .resources
            .iter()
            .find(|resource| resource.path == path)
            .map(|resource| resource.content.as_str())
            .ok_or_else(|| ResourceError::SkillResourceNotFound {
                skill: skill.to_owned(),
                path: path.to_owned(),
            })
    }

    #[must_use]
    pub fn command_summaries(&self) -> Vec<CommandSummary> {
        self.commands
            .values()
            .map(|command| command.summary.clone())
            .collect()
    }

    pub fn command(&self, name: &str) -> Result<&CommandCatalogEntry, ResourceError> {
        self.commands
            .get(name)
            .ok_or_else(|| ResourceError::CommandNotFound(name.to_owned()))
    }

    pub fn expand_command(
        &self,
        name: &str,
        arguments: &str,
    ) -> Result<CommandExpansion, ResourceError> {
        let command = self.command(name)?;
        let tokens = parse_arguments(arguments)?;
        let text = substitute_arguments(&command.body, arguments, &tokens)?;
        if text.trim().is_empty() {
            return Err(ResourceError::InvalidCommand(format!(
                "command {name:?} expands to empty input"
            )));
        }
        if text.len() > MAX_EXPANDED_BYTES {
            return Err(ResourceError::InvalidArguments(
                "expanded command exceeds 1 MiB".to_owned(),
            ));
        }
        Ok(CommandExpansion {
            name: name.to_owned(),
            text,
            content_digest: command.summary.content_digest.clone(),
        })
    }
}

#[derive(Clone)]
struct PackageCandidate<'a> {
    origin: InstructionOrigin,
    sources: Vec<(&'a str, &'a InstructionSource)>,
}

fn build_skills(
    snapshot: &InstructionSnapshot,
) -> Result<BTreeMap<String, SkillPackage>, ResourceError> {
    let mut candidates: BTreeMap<String, PackageCandidate<'_>> = BTreeMap::new();
    for source in snapshot
        .sources
        .iter()
        .filter(|source| source.kind == InstructionKind::Skill)
    {
        let relative = source
            .path
            .strip_prefix("skills/")
            .ok_or_else(|| ResourceError::InvalidPath(source.path.clone()))?;
        let Some((name, path)) = relative.split_once('/') else {
            continue;
        };
        validate_name(name)?;
        validate_relative(path)?;
        match candidates.get_mut(name) {
            Some(candidate) if candidate.origin == source.origin => {
                candidate.sources.push((path, source));
            }
            Some(candidate) => {
                return Err(ResourceError::InvalidSkill(format!(
                    "skill {name:?} is defined by both {} and {}; rename one source explicitly",
                    source_label(&candidate.origin),
                    source_label(&source.origin),
                )));
            }
            _ => {
                candidates.insert(
                    name.to_owned(),
                    PackageCandidate {
                        origin: source.origin.clone(),
                        sources: vec![(path, source)],
                    },
                );
            }
        }
    }

    let mut packages = BTreeMap::new();
    for (directory_name, mut candidate) in candidates {
        candidate
            .sources
            .sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        let Some((_, manifest)) = candidate
            .sources
            .iter()
            .find(|(path, _)| *path == "SKILL.md")
        else {
            return Err(ResourceError::InvalidSkill(format!(
                "winning package {directory_name:?} lacks SKILL.md"
            )));
        };
        let parsed = parse_skill_manifest(&manifest.content).map_err(|error| {
            ResourceError::InvalidSkill(format!(
                "{} ({}): {error}",
                manifest.path,
                source_label(&candidate.origin)
            ))
        })?;
        if parsed.name != directory_name {
            return Err(ResourceError::InvalidSkill(format!(
                "manifest name {:?} does not match directory {directory_name:?}",
                parsed.name
            )));
        }
        let resources = candidate
            .sources
            .iter()
            .map(|(path, source)| SkillResource {
                path: (*path).to_owned(),
                content: source.content.clone(),
                content_sha256: source.content_sha256.clone(),
            })
            .collect::<Vec<_>>();
        let digest_value = json!({
            "format": 1,
            "name": directory_name,
            "resources": resources.iter().map(|resource| json!({
                "path": resource.path,
                "content_sha256": resource.content_sha256,
            })).collect::<Vec<_>>()
        });
        let digest_bytes = serde_json_canonicalizer::to_vec(&digest_value)
            .map_err(|error| ResourceError::InvalidSkill(error.to_string()))?;
        let content_digest = hex_digest(&digest_bytes);
        let summary = SkillSummary {
            id: directory_name.clone(),
            name: directory_name.clone(),
            description: parsed.description,
            source: source_label(&candidate.origin),
            content_digest,
        };
        packages.insert(
            directory_name,
            SkillPackage {
                summary,
                body: parsed.body,
                resources,
            },
        );
    }
    Ok(packages)
}

fn build_commands(
    snapshot: &InstructionSnapshot,
) -> Result<BTreeMap<String, CommandCatalogEntry>, ResourceError> {
    let mut commands = BTreeMap::new();
    for (logical, index) in &snapshot.effective.commands {
        if logical.contains('/') || !logical.ends_with(".md") {
            continue;
        }
        let name = logical
            .strip_suffix(".md")
            .ok_or_else(|| ResourceError::InvalidPath(logical.clone()))?;
        validate_name(name)?;
        let source = snapshot.sources.get(*index as usize).ok_or_else(|| {
            ResourceError::InvalidCommand(format!("effective command {logical:?} has no source"))
        })?;
        let parsed = parse_command(&source.content)?;
        commands.insert(
            name.to_owned(),
            CommandCatalogEntry {
                summary: CommandSummary {
                    name: name.to_owned(),
                    description: parsed
                        .fields
                        .get("description")
                        .cloned()
                        .unwrap_or_default(),
                    argument_hint: parsed
                        .fields
                        .get("argument-hint")
                        .or_else(|| parsed.fields.get("argument_hint"))
                        .cloned(),
                    model: parsed.fields.get("model").cloned(),
                    source: source_label(&source.origin),
                    content_digest: source.content_sha256.clone(),
                },
                body: parsed.body,
            },
        );
    }
    Ok(commands)
}

struct ParsedDocument {
    fields: BTreeMap<String, String>,
    body: String,
}

struct ParsedSkill {
    name: String,
    description: String,
    body: String,
}

fn parse_skill_manifest(content: &str) -> Result<ParsedSkill, ResourceError> {
    let parsed = parse_front_matter(content, true)
        .map_err(|error| ResourceError::InvalidSkill(error.to_string()))?;
    if let Some(version) = parsed.fields.get("schema_version") {
        if version != "1" {
            return Err(ResourceError::InvalidSkill(
                "schema_version must be 1".to_owned(),
            ));
        }
    }
    let name = parsed
        .fields
        .get("name")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| ResourceError::InvalidSkill("front matter lacks name".to_owned()))?;
    validate_name(&name)?;
    let description = parsed
        .fields
        .get("description")
        .filter(|value| !value.is_empty())
        .cloned()
        .ok_or_else(|| ResourceError::InvalidSkill("front matter lacks description".to_owned()))?;
    Ok(ParsedSkill {
        name,
        description,
        body: parsed.body,
    })
}

fn parse_command(content: &str) -> Result<ParsedDocument, ResourceError> {
    let parsed = parse_front_matter(content, false)
        .map_err(|error| ResourceError::InvalidCommand(error.to_string()))?;
    if parsed.fields.contains_key("argument-hint") && parsed.fields.contains_key("argument_hint") {
        return Err(ResourceError::InvalidCommand(
            "argument-hint and argument_hint cannot both be present".to_owned(),
        ));
    }
    if parsed.body.trim().is_empty() {
        return Err(ResourceError::InvalidCommand(
            "command body must not be empty".to_owned(),
        ));
    }
    Ok(parsed)
}

fn parse_front_matter(content: &str, required: bool) -> Result<ParsedDocument, &'static str> {
    let normalized = content.replace("\r\n", "\n");
    if !normalized.starts_with("---\n") {
        return if required {
            Err("front matter is required")
        } else {
            Ok(ParsedDocument {
                fields: BTreeMap::new(),
                body: normalized.trim().to_owned(),
            })
        };
    }
    let lines = normalized.split('\n').collect::<Vec<_>>();
    let Some(closing) = lines.iter().skip(1).position(|line| *line == "---") else {
        return Err("front matter closing fence is missing");
    };
    let closing = closing + 1;
    // Shared skill catalogs also contain plain, single-line descriptions with
    // embedded `: `. Treat only that human-readable field as literal text;
    // keep structured YAML and all other syntax subject to normal validation.
    let header = lines[1..closing].join("\n");
    let yaml: serde_yaml::Mapping = serde_yaml::from_str(&header)
        .or_else(|original| {
            let repaired = lines[1..closing]
                .iter()
                .map(|line| {
                    if let Some(value) = line.strip_prefix("description:") {
                        let value = value.trim();
                        if value.contains(": ")
                            && !value.starts_with(['\'', '"', '|', '>', '{', '['])
                        {
                            return format!(
                                "description: {}",
                                serde_json::to_string(value).expect("string serialization")
                            );
                        }
                    }
                    (*line).to_owned()
                })
                .collect::<Vec<_>>()
                .join("\n");
            if repaired == header {
                Err(original)
            } else {
                serde_yaml::from_str(&repaired)
            }
        })
        .map_err(|_| "invalid YAML front matter")?;
    let mut fields = BTreeMap::new();
    for (key, value) in yaml {
        let key = key
            .as_str()
            .filter(|key| !key.is_empty())
            .ok_or("front matter key must be a nonempty string")?;
        let scalar = match value {
            serde_yaml::Value::String(value) => Some(value),
            serde_yaml::Value::Bool(value) => Some(value.to_string()),
            serde_yaml::Value::Number(value) => Some(value.to_string()),
            // Structured metadata is allowed, but not interpreted as a command
            // argument or skill identity by this catalog.
            _ => None,
        };
        if let Some(value) = scalar {
            fields.insert(key.to_owned(), value);
        }
    }
    Ok(ParsedDocument {
        fields,
        body: lines[closing + 1..].join("\n").trim().to_owned(),
    })
}

fn parse_arguments(arguments: &str) -> Result<Vec<String>, ResourceError> {
    if arguments.len() > MAX_ARGUMENT_BYTES {
        return Err(ResourceError::InvalidArguments(
            "arguments exceed 64 KiB".to_owned(),
        ));
    }
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut quote = None;
    let mut escaped = false;
    let mut active = false;
    for character in arguments.chars() {
        if escaped {
            token.push(character);
            escaped = false;
            active = true;
            continue;
        }
        match quote {
            Some('\'') if character == '\'' => quote = None,
            Some('"') if character == '"' => quote = None,
            Some('\'') => token.push(character),
            Some('"') if character == '\\' => escaped = true,
            Some('"') => token.push(character),
            None if character == '\'' || character == '"' => {
                quote = Some(character);
                active = true;
            }
            None if character == '\\' => {
                escaped = true;
                active = true;
            }
            None if character.is_whitespace() => {
                if active {
                    tokens.push(std::mem::take(&mut token));
                    active = false;
                }
            }
            None => {
                token.push(character);
                active = true;
            }
            Some(_) => unreachable!(),
        }
    }
    if quote.is_some() || escaped {
        return Err(ResourceError::InvalidArguments(
            "arguments contain an unterminated quote or escape".to_owned(),
        ));
    }
    if active {
        tokens.push(token);
    }
    Ok(tokens)
}

fn substitute_arguments(body: &str, raw: &str, tokens: &[String]) -> Result<String, ResourceError> {
    let chars = body.chars().collect::<Vec<_>>();
    let mut output = String::new();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '\\' && chars.get(index + 1) == Some(&'$') {
            push_expansion_char(&mut output, '$')?;
            index += 2;
            continue;
        }
        if chars[index] != '$' {
            push_expansion_char(&mut output, chars[index])?;
            index += 1;
            continue;
        }
        if chars.get(index + 1) == Some(&'$') {
            push_expansion_char(&mut output, '$')?;
            index += 2;
            continue;
        }
        let suffix = chars[index..].iter().collect::<String>();
        if suffix.starts_with("$ARGUMENTS") {
            push_expansion(&mut output, raw)?;
            index += "$ARGUMENTS".chars().count();
            continue;
        }
        let mut end = index + 1;
        while end < chars.len() && chars[end].is_ascii_digit() && end - index <= 2 {
            end += 1;
        }
        if end > index + 1 {
            if chars.get(end).is_some_and(char::is_ascii_digit) {
                return Err(ResourceError::InvalidArguments(
                    "positionals are limited to $1 through $99".to_owned(),
                ));
            }
            let position = chars[index + 1..end]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .map_err(|_| ResourceError::InvalidArguments("invalid positional".to_owned()))?;
            if position == 0 {
                return Err(ResourceError::InvalidArguments(
                    "$0 is not a valid positional".to_owned(),
                ));
            }
            if let Some(value) = tokens.get(position - 1) {
                push_expansion(&mut output, value)?;
            }
            index = end;
            continue;
        }
        push_expansion_char(&mut output, '$')?;
        index += 1;
    }
    Ok(output)
}

fn push_expansion(output: &mut String, value: &str) -> Result<(), ResourceError> {
    if value.len() > MAX_EXPANDED_BYTES.saturating_sub(output.len()) {
        return Err(ResourceError::InvalidArguments(
            "expanded command exceeds 1 MiB".to_owned(),
        ));
    }
    output.push_str(value);
    Ok(())
}

fn push_expansion_char(output: &mut String, value: char) -> Result<(), ResourceError> {
    let mut encoded = [0; 4];
    push_expansion(output, value.encode_utf8(&mut encoded))
}

fn source_label(origin: &InstructionOrigin) -> String {
    match origin {
        InstructionOrigin::User => "user".to_owned(),
        InstructionOrigin::Workspace => "workspace".to_owned(),
        InstructionOrigin::Project { workspace_index } => {
            format!("project:{workspace_index}")
        }
    }
}

fn validate_name(name: &str) -> Result<(), ResourceError> {
    if name.is_empty()
        || name.len() > MAX_NAME_BYTES
        || !name.bytes().enumerate().all(|(index, byte)| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || (index > 0 && matches!(byte, b'-' | b'_'))
        })
    {
        return Err(ResourceError::InvalidName(name.to_owned()));
    }
    Ok(())
}

fn validate_relative(path: &str) -> Result<(), ResourceError> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\0')
        || path
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(ResourceError::InvalidPath(path.to_owned()));
    }
    Ok(())
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    #[test]
    fn shared_skill_plain_description_accepts_embedded_colon() {
        let parsed = super::parse_front_matter("---\nname: example\ndescription: Set up a project: scan and validate.\nreferences:\n  - guide\n---\nBody", true).unwrap();
        assert_eq!(
            parsed.fields["description"],
            "Set up a project: scan and validate."
        );
        assert!(!parsed.fields.contains_key("references"));
        assert!(
            super::parse_front_matter(
                "---\nname: [broken\ndescription: Text: detail\n---\nBody",
                true
            )
            .is_err()
        );
    }

    use super::*;
    use crate::{EffectiveInstructions, InstructionSource};

    fn source(
        origin: InstructionOrigin,
        path: &str,
        kind: InstructionKind,
        content: &str,
    ) -> InstructionSource {
        InstructionSource {
            origin,
            path: path.to_owned(),
            kind,
            content: content.to_owned(),
            content_sha256: hex_digest(content.as_bytes()),
        }
    }

    #[test]
    fn cross_scope_skill_collision_is_explicit_instead_of_silent_precedence() {
        let user = InstructionOrigin::User;
        let project = InstructionOrigin::Project { workspace_index: 0 };
        let sources = vec![
            source(
                user.clone(),
                "skills/review/SKILL.md",
                InstructionKind::Skill,
                "---\nname: review\ndescription: User review\n---\nUser body\n",
            ),
            source(
                user,
                "skills/review/scripts/check.sh",
                InstructionKind::Skill,
                "user\n",
            ),
            source(
                project.clone(),
                "skills/review/SKILL.md",
                InstructionKind::Skill,
                "---\nname: review\ndescription: Project review\n---\nProject body\n",
            ),
            source(
                project,
                "skills/review/references/rules.md",
                InstructionKind::Skill,
                "rules\n",
            ),
        ];
        let snapshot = InstructionSnapshot {
            format: 1,
            sources,
            effective: EffectiveInstructions::default(),
        };
        let error = ResourceCatalog::from_snapshot(&snapshot).expect_err("scope collision");
        assert!(
            error
                .to_string()
                .contains("defined by both user and project:0")
        );
    }

    #[test]
    fn command_expansion_has_closed_quoting_and_escape_rules() {
        let content = "---\ndescription: Review files\nargument-hint: <path> <mode>\n---\nReview $1 in $2; raw=$ARGUMENTS; dollar=$$; literal=\\$1\n";
        let source = source(
            InstructionOrigin::User,
            "commands/review.md",
            InstructionKind::Command,
            content,
        );
        let mut effective = EffectiveInstructions::default();
        effective.commands.insert("review.md".to_owned(), 0);
        let snapshot = InstructionSnapshot {
            format: 1,
            sources: vec![source],
            effective,
        };
        let catalog = ResourceCatalog::from_snapshot(&snapshot).expect("catalog");
        let expanded = catalog
            .expand_command("review", "'a b' strict")
            .expect("expand");
        assert_eq!(
            expanded.text,
            "Review a b in strict; raw='a b' strict; dollar=$; literal=$1"
        );
        assert!(catalog.expand_command("review", "\"unterminated").is_err());
    }

    #[test]
    fn invalid_winner_and_out_of_range_positionals_fail_closed() {
        let invalid_package = InstructionSnapshot {
            format: 1,
            sources: vec![source(
                InstructionOrigin::Project { workspace_index: 0 },
                "skills/review/companion.md",
                InstructionKind::Skill,
                "companion\n",
            )],
            effective: EffectiveInstructions::default(),
        };
        assert!(matches!(
            ResourceCatalog::from_snapshot(&invalid_package),
            Err(ResourceError::InvalidSkill(_))
        ));

        let command = source(
            InstructionOrigin::User,
            "commands/position.md",
            InstructionKind::Command,
            "Use $100\n",
        );
        let mut effective = EffectiveInstructions::default();
        effective.commands.insert("position.md".to_owned(), 0);
        let catalog = ResourceCatalog::from_snapshot(&InstructionSnapshot {
            format: 1,
            sources: vec![command],
            effective,
        })
        .expect("catalog");
        assert!(matches!(
            catalog.expand_command("position", "one"),
            Err(ResourceError::InvalidArguments(_))
        ));

        let repeated = "$ARGUMENTS".repeat(20);
        let expanding = source(
            InstructionOrigin::User,
            "commands/expand.md",
            InstructionKind::Command,
            &repeated,
        );
        let mut effective = EffectiveInstructions::default();
        effective.commands.insert("expand.md".to_owned(), 0);
        let catalog = ResourceCatalog::from_snapshot(&InstructionSnapshot {
            format: 1,
            sources: vec![expanding],
            effective,
        })
        .expect("expanding catalog");
        assert!(matches!(
            catalog.expand_command("expand", &"x".repeat(MAX_ARGUMENT_BYTES)),
            Err(ResourceError::InvalidArguments(_))
        ));
    }
}
