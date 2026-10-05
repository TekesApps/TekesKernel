use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::SelectorError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeSignature {
    pub architectures: Vec<String>,
    pub team_id: String,
}

pub trait CodeSignatureVerifier: Send + Sync {
    fn verify(&self, executable: &Path, requirement: &str) -> Result<CodeSignature, SelectorError>;

    fn verify_provisioned_app(
        &self,
        app_bundle: &Path,
        requirement: &str,
        team_id: &str,
        bundle_identifier: &str,
        required_access_groups: &[String],
    ) -> Result<(), SelectorError>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MacOsCodeSignatureVerifier;

impl CodeSignatureVerifier for MacOsCodeSignatureVerifier {
    fn verify(&self, executable: &Path, requirement: &str) -> Result<CodeSignature, SelectorError> {
        #[cfg(target_os = "macos")]
        {
            verify_requirement(executable, requirement)?;
            let metadata = Command::new("/usr/bin/codesign")
                .args(["--display", "--verbose=4"])
                .arg(executable)
                .output()
                .map_err(|error| SelectorError::io("codesign-display", error))?;
            if !metadata.status.success() {
                return Err(SelectorError::invalid_bundle(
                    executable.display().to_string(),
                ));
            }
            let text = String::from_utf8_lossy(&metadata.stderr);
            let team_id = field(&text, "TeamIdentifier=")?;
            let arch_output = Command::new("/usr/bin/lipo")
                .args(["-archs"])
                .arg(executable)
                .output()
                .map_err(|error| SelectorError::io("lipo-archs", error))?;
            if !arch_output.status.success() {
                return Err(SelectorError::invalid_bundle(
                    executable.display().to_string(),
                ));
            }
            let architectures = String::from_utf8_lossy(&arch_output.stdout)
                .split_whitespace()
                .map(|arch| if arch == "arm64" { "aarch64" } else { arch }.to_owned())
                .collect();
            Ok(CodeSignature {
                architectures,
                team_id,
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (executable, requirement);
            Err(SelectorError::invalid_bundle(
                "code signatures require macOS",
            ))
        }
    }

    fn verify_provisioned_app(
        &self,
        app_bundle: &Path,
        requirement: &str,
        team_id: &str,
        bundle_identifier: &str,
        required_access_groups: &[String],
    ) -> Result<(), SelectorError> {
        #[cfg(target_os = "macos")]
        {
            verify_requirement(app_bundle, requirement)?;
            let metadata = codesign_display(app_bundle, &["--verbose=4"], "codesign-app-display")?;
            if field(&String::from_utf8_lossy(&metadata), "TeamIdentifier=")? != team_id {
                return invalid(app_bundle);
            }

            let info = std::fs::read(app_bundle.join("Contents/Info.plist"))
                .map_err(|error| SelectorError::io("app-info-read", error))?;
            let info = plist_json(&info)?;
            if json_string(&info, "CFBundleIdentifier")? != bundle_identifier {
                return invalid(app_bundle);
            }

            let entitlements = codesign_display(
                app_bundle,
                &["--entitlements", ":-"],
                "codesign-entitlements",
            )?;
            let entitlements = plist_json(&entitlements)?;
            let application_identifier = format!("{team_id}.{bundle_identifier}");
            if json_string(&entitlements, "com.apple.application-identifier")?
                != application_identifier
                || json_string(&entitlements, "com.apple.developer.team-identifier")? != team_id
                || !groups_include(
                    &json_strings(&entitlements, "keychain-access-groups")?,
                    required_access_groups,
                    team_id,
                    false,
                )
            {
                return invalid(app_bundle);
            }

            let profile_path = app_bundle.join("Contents/embedded.provisionprofile");
            let decoded = Command::new("/usr/bin/security")
                .args(["cms", "-D", "-i"])
                .arg(&profile_path)
                .output()
                .map_err(|error| SelectorError::io("profile-cms-decode", error))?;
            if !decoded.status.success() {
                return invalid(app_bundle);
            }
            let profile_entitlements =
                plist_extract(&decoded.stdout, "Entitlements", "xml1", "dictionary")?;
            let profile_entitlements = plist_json(&profile_entitlements)?;
            if !plist_strings(&decoded.stdout, "TeamIdentifier")?
                .iter()
                .any(|value| value == team_id)
                || !profile_allows_application(
                    json_string(&profile_entitlements, "com.apple.application-identifier")?,
                    &application_identifier,
                    team_id,
                )
                || json_string(&profile_entitlements, "com.apple.developer.team-identifier")?
                    != team_id
                || !groups_include(
                    &json_strings(&profile_entitlements, "keychain-access-groups")?,
                    required_access_groups,
                    team_id,
                    true,
                )
                || profile_expired(&plist_raw(&decoded.stdout, "ExpirationDate", "date")?)?
            {
                return invalid(app_bundle);
            }
            Ok(())
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (
                app_bundle,
                requirement,
                team_id,
                bundle_identifier,
                required_access_groups,
            );
            Err(SelectorError::invalid_bundle(
                "provisioned app verification requires macOS",
            ))
        }
    }
}

#[cfg(target_os = "macos")]
fn verify_requirement(path: &Path, requirement: &str) -> Result<(), SelectorError> {
    let result = Command::new("/usr/bin/codesign")
        .args(["--verify", "--strict", "--verbose=4", "-R"])
        .arg(format!("={requirement}"))
        .arg(path)
        .output()
        .map_err(|error| SelectorError::io("codesign-verify", error))?;
    if result.status.success() {
        Ok(())
    } else {
        invalid(path)
    }
}

#[cfg(target_os = "macos")]
fn codesign_display(
    path: &Path,
    arguments: &[&str],
    operation: &'static str,
) -> Result<Vec<u8>, SelectorError> {
    let output = Command::new("/usr/bin/codesign")
        .arg("--display")
        .args(arguments)
        .arg(path)
        .output()
        .map_err(|error| SelectorError::io(operation, error))?;
    if !output.status.success() {
        return invalid(path);
    }
    if output.stdout.is_empty() {
        Ok(output.stderr)
    } else {
        Ok(output.stdout)
    }
}

#[cfg(target_os = "macos")]
fn plist_json(plist: &[u8]) -> Result<serde_json::Value, SelectorError> {
    let mut child = Command::new("/usr/bin/plutil")
        .args(["-convert", "json", "-o", "-", "--", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| SelectorError::io("plist-extract", error))?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| SelectorError::invalid_bundle("plist-stdin"))?;
    std::io::Write::write_all(&mut stdin, plist)
        .map_err(|error| SelectorError::io("plist-write", error))?;
    drop(stdin);
    let output = child
        .wait_with_output()
        .map_err(|error| SelectorError::io("plist-wait", error))?;
    if output.status.success() {
        serde_json::from_slice(&output.stdout)
            .map_err(|_| SelectorError::invalid_bundle("plist-json"))
    } else {
        Err(SelectorError::invalid_bundle("plist-convert"))
    }
}

#[cfg(target_os = "macos")]
fn plist_extract(
    plist: &[u8],
    key: &str,
    format: &str,
    expected_type: &str,
) -> Result<Vec<u8>, SelectorError> {
    // Callers use only top-level keys without dots. Entitlement dictionaries
    // are extracted first and then decoded as JSON so literal dotted keys are
    // never interpreted as plutil key-path segments.
    let mut child = Command::new("/usr/bin/plutil")
        .args([
            "-extract",
            key,
            format,
            "-expect",
            expected_type,
            "-o",
            "-",
            "--",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| SelectorError::io("plist-extract", error))?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| SelectorError::invalid_bundle("plist-stdin"))?;
    std::io::Write::write_all(&mut stdin, plist)
        .map_err(|error| SelectorError::io("plist-write", error))?;
    drop(stdin);
    let output = child
        .wait_with_output()
        .map_err(|error| SelectorError::io("plist-wait", error))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(SelectorError::invalid_bundle("plist-field"))
    }
}

#[cfg(target_os = "macos")]
fn plist_raw(plist: &[u8], key: &str, expected_type: &str) -> Result<String, SelectorError> {
    let bytes = plist_extract(plist, key, "raw", expected_type)?;
    String::from_utf8(bytes)
        .map(|value| value.trim_end_matches('\n').to_owned())
        .map_err(|_| SelectorError::invalid_bundle("plist-utf8"))
}

#[cfg(target_os = "macos")]
fn plist_strings(plist: &[u8], key: &str) -> Result<Vec<String>, SelectorError> {
    let bytes = plist_extract(plist, key, "json", "array")?;
    serde_json::from_slice(&bytes).map_err(|_| SelectorError::invalid_bundle("plist-array"))
}

#[cfg(target_os = "macos")]
fn json_string<'a>(value: &'a serde_json::Value, key: &str) -> Result<&'a str, SelectorError> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| SelectorError::invalid_bundle("plist-string"))
}

#[cfg(target_os = "macos")]
fn json_strings(value: &serde_json::Value, key: &str) -> Result<Vec<String>, SelectorError> {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| SelectorError::invalid_bundle("plist-array"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| SelectorError::invalid_bundle("plist-array-value"))
        })
        .collect()
}

#[cfg(target_os = "macos")]
fn groups_include(
    actual: &[String],
    required: &[String],
    team_id: &str,
    allow_wildcard: bool,
) -> bool {
    (allow_wildcard && actual.iter().any(|group| group == &format!("{team_id}.*")))
        || required.iter().all(|group| actual.contains(group))
}

#[cfg(target_os = "macos")]
fn profile_allows_application(grant: &str, application: &str, team: &str) -> bool {
    grant == application || grant == format!("{team}.*")
}

#[cfg(target_os = "macos")]
fn profile_expired(expiration: &str) -> Result<bool, SelectorError> {
    let output = Command::new("/bin/date")
        .args(["-j", "-u", "-f", "%Y-%m-%dT%H:%M:%SZ", expiration, "+%s"])
        .output()
        .map_err(|error| SelectorError::io("profile-expiration", error))?;
    let expiration_epoch = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .map_err(|_| SelectorError::invalid_bundle("profile-expiration"))?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SelectorError::invalid_bundle("system-time"))?
        .as_secs();
    Ok(!output.status.success() || expiration_epoch <= now)
}

#[cfg(target_os = "macos")]
fn invalid<T>(path: &Path) -> Result<T, SelectorError> {
    Err(SelectorError::invalid_bundle(path.display().to_string()))
}

#[cfg(target_os = "macos")]
fn field(text: &str, prefix: &str) -> Result<String, SelectorError> {
    text.lines()
        .find_map(|line| line.strip_prefix(prefix))
        .map(str::to_owned)
        .ok_or_else(|| SelectorError::invalid_bundle("codesign-metadata"))
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::{json_string, json_strings, plist_json, profile_allows_application};

    #[test]
    fn profile_grants_accept_only_exact_app_or_same_team_wildcard() {
        let app = "TEAM.com.tekes.kernel.supervisor";
        assert!(profile_allows_application(app, app, "TEAM"));
        assert!(profile_allows_application("TEAM.*", app, "TEAM"));
        for grant in ["OTHER.*", "*", "TEAM.com.tekes.*", "TEAM.com.other"] {
            assert!(!profile_allows_application(grant, app, "TEAM"));
        }
    }

    #[test]
    fn plist_conversion_preserves_literal_entitlement_keys_with_dots() {
        let value = plist_json(
            br#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0"><dict>
<key>com.apple.application-identifier</key><string>TEAM.bundle</string>
<key>keychain-access-groups</key><array><string>TEAM.group</string></array>
</dict></plist>"#,
        )
        .expect("plist conversion");
        assert_eq!(
            json_string(&value, "com.apple.application-identifier").unwrap(),
            "TEAM.bundle"
        );
        assert_eq!(
            json_strings(&value, "keychain-access-groups").unwrap(),
            ["TEAM.group"]
        );
    }
}
