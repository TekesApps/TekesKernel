//! Runtime-owned opening of the root system instructions.
//!
//! The harness identity, the session profile text, and the working directory
//! precede the instruction snapshot's `AGENTS.md` scopes. A profile is its
//! identity line, the general efficiency rules every profile shares, then the
//! profile's own rules. Each part lives in one file under
//! `crates/tools/prompts/` and is pinned byte-for-byte by
//! `fixtures/tools/builtin-tool-guidance.canonical.json` (spec builtin-tools
//! §Model-facing tool guidance). A sentence ablation on the weighted-ttl corpus
//! found the identity lines carry the trajectory-length effect for
//! deepseek-v4-flash, so the identity line stays first; per-tool usage
//! paragraphs carried none of it and were removed. The behavior rules and their
//! measured effect are described in docs/benchmarks/deepseek-cost-2026-10.md.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// Fixed opener, before the model line.
pub const HARNESS_IDENTITY: &str = "You are an AI agent powered by Tekes Kernel.";

/// General identity line. `{model}` is the wire model id of the request.
pub const GENERAL_IDENTITY: &str = include_str!("../prompts/general-identity.md");

/// Domain-neutral efficiency rules shared by every profile.
pub const GENERAL_RULES: &str = include_str!("../prompts/general-rules.md");

/// Coding identity line. `{model}` is the wire model id of the request.
pub const CODING_IDENTITY: &str = include_str!("../prompts/coding-identity.md");

/// Coding-only behavior rules, after the general rules.
pub const CODING_RULES: &str = include_str!("../prompts/coding-rules.md");

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

    /// The profile text with `{model}` unresolved: the identity line, the
    /// general rules, then the profile's own rules, joined by blank lines.
    #[must_use]
    pub fn profile_text(self) -> String {
        let parts: &[&str] = match self {
            Self::Coding => &[CODING_IDENTITY, GENERAL_RULES, CODING_RULES],
            Self::General => &[GENERAL_IDENTITY, GENERAL_RULES],
        };
        parts
            .iter()
            .map(|part| part.trim())
            .collect::<Vec<_>>()
            .join("\n\n")
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
    let mut sections = vec![
        HARNESS_IDENTITY.to_owned(),
        identity.profile_text().replace("{model}", model),
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
        "general_identity": GENERAL_IDENTITY,
        "general_rules": GENERAL_RULES,
        "coding_identity": CODING_IDENTITY,
        "coding_rules": CODING_RULES,
        "working_directory": WORKING_DIRECTORY,
    });
    let canonical = serde_json_canonicalizer::to_vec(&preimage).expect("guidance oracle is JSON");
    json!({
        "format": 5,
        "digest": format!("sha256-{:x}", Sha256::digest(canonical)),
        "harness_identity": HARNESS_IDENTITY,
        "general_identity": GENERAL_IDENTITY,
        "general_rules": GENERAL_RULES,
        "coding_identity": CODING_IDENTITY,
        "coding_rules": CODING_RULES,
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
        let general_rules = "When the requested work is done, stop and report. If you find no problem, say so instead of continuing to search for one.\n\nBe concise. Do not narrate before each tool call.";
        let coding = format!(
            "You are a coding agent powered by the deepseek-flash model.\n\n{general_rules}\n\nTreat a change as done when the requested tests pass.\n\nDo not write extra fuzz, randomized, or differential test scripts unless the user asks for them.\n\nCheck expected results by writing and running tests, not by working out program output at length in your reasoning."
        );
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
                IdentityProfile::Coding
                    .profile_text()
                    .replace("{model}", "m")
            )
        );
        let general = root_system_instructions(IdentityProfile::General, "m", Some("/work"), "");
        assert_eq!(
            general,
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a general-purpose agent powered by the m model.\n\n{general_rules}\n\nYour working directory is /work."
            )
        );
    }

    #[test]
    fn profile_text_keeps_model_placeholder_and_identity_first() {
        for profile in [IdentityProfile::Coding, IdentityProfile::General] {
            let text = profile.profile_text();
            assert!(text.contains("{model}"));
            assert!(text.starts_with("You are a "));
            assert_eq!(text.matches("You are a").count(), 1);
            assert!(text.contains(GENERAL_RULES.trim()));
        }
        assert!(
            !IdentityProfile::General
                .profile_text()
                .contains(CODING_RULES.trim())
        );
    }
}
