//! Runtime-owned opening of the root system instructions.
//!
//! The harness identity, the session profile text, and the working directory
//! precede the instruction snapshot's `AGENTS.md` scopes. Each profile's text
//! lives in one file under `crates/tools/prompts/` (`coding.md`, `general.md`)
//! and is pinned byte-for-byte by `fixtures/tools/builtin-tool-guidance.canonical.json`
//! (spec builtin-tools §Model-facing tool guidance). A sentence ablation on
//! the weighted-ttl corpus found the identity lines carry the
//! trajectory-length effect for deepseek-v4-flash; per-tool usage paragraphs
//! carried none of it and were removed. The coding behavior rules and their
//! measured effect are described in docs/benchmarks/deepseek-cost-2026-10.md.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Fixed opener, before the model line.
pub const HARNESS_IDENTITY: &str = "You are an AI agent powered by Tekes Kernel.";

/// Coding profile text: model identity and coding behavior rules. `{model}` is
/// the wire model id of the request.
pub const CODING_PROFILE: &str = include_str!("../prompts/coding.md");

/// Neutral profile text for sessions that are not primarily coding work.
pub const GENERAL_PROFILE: &str = include_str!("../prompts/general.md");

/// Resolved session identity. Legacy genesis without an identity stays coding;
/// new sessions may defer selection until the first user request.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityProfile {
    #[default]
    Coding,
    General,
}

impl IdentityProfile {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Coding => "coding",
            Self::General => "general",
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "coding" => Some(Self::Coding),
            "general" => Some(Self::General),
            _ => None,
        }
    }
}

/// Working-directory line template; `{cwd}` is the execution cwd.
pub const WORKING_DIRECTORY: &str = "Your working directory is {cwd}.";

/// The root system instructions: harness identity, the profile text, the
/// working directory when one is bound, then the instruction snapshot text when
/// it is nonempty.
#[must_use]
pub fn root_system_instructions(
    identity: IdentityProfile,
    model: &str,
    cwd: Option<&str>,
    instruction_text: &str,
) -> String {
    let profile = match identity {
        IdentityProfile::Coding => CODING_PROFILE,
        IdentityProfile::General => GENERAL_PROFILE,
    };
    let mut sections = vec![
        HARNESS_IDENTITY.to_owned(),
        profile.trim().replace("{model}", model),
    ];
    if let Some(cwd) = cwd {
        sections.push(WORKING_DIRECTORY.replace("{cwd}", cwd));
    }
    let instruction_text = instruction_text.trim();
    if !instruction_text.is_empty() {
        sections.push(instruction_text.to_owned());
    }
    sections.join("\n\n")
}

/// The guidance oracle as a JSON value (fixture `builtin-tool-guidance`).
#[must_use]
pub fn guidance_oracle_value() -> Value {
    let preimage = json!({
        "harness_identity": HARNESS_IDENTITY,
        "coding_profile": CODING_PROFILE,
        "general_profile": GENERAL_PROFILE,
        "working_directory": WORKING_DIRECTORY,
    });
    let canonical = serde_json_canonicalizer::to_vec(&preimage).expect("guidance oracle is JSON");
    json!({
        "format": 4,
        "digest": format!("sha256-{:x}", Sha256::digest(canonical)),
        "harness_identity": HARNESS_IDENTITY,
        "coding_profile": CODING_PROFILE,
        "general_profile": GENERAL_PROFILE,
        "working_directory": WORKING_DIRECTORY,
    })
}

/// Canonical bytes of the guidance oracle, one final LF.
#[must_use]
pub fn canonical_guidance_oracle_bytes() -> Vec<u8> {
    let mut bytes = serde_json_canonicalizer::to_vec(&guidance_oracle_value())
        .expect("guidance oracle is JSON");
    bytes.push(b'\n');
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_instructions_are_identity_profile_cwd_then_agents_text() {
        let coding = CODING_PROFILE.trim().replace("{model}", "deepseek-flash");
        assert!(
            coding.starts_with("You are a coding agent powered by the deepseek-flash model.\n\n")
        );
        assert!(coding.contains("stop and report"));
        let text = root_system_instructions(
            IdentityProfile::Coding,
            "deepseek-flash",
            Some("/work"),
            "  Project rule.\n",
        );
        assert_eq!(
            text,
            format!(
                "{HARNESS_IDENTITY}\n\n{coding}\n\nYour working directory is /work.\n\nProject rule."
            )
        );
        let bare = root_system_instructions(IdentityProfile::Coding, "m", None, "");
        assert_eq!(
            bare,
            format!(
                "{HARNESS_IDENTITY}\n\n{}",
                CODING_PROFILE.trim().replace("{model}", "m")
            )
        );
        let general = root_system_instructions(IdentityProfile::General, "m", Some("/work"), "");
        assert_eq!(
            general,
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a general-purpose agent powered by the m model.\n\nYour working directory is /work."
            )
        );
    }
}
