use std::cmp::Ordering;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::PluginError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PluginVersion {
    major: String,
    minor: String,
    patch: String,
    prerelease: Vec<String>,
    build: Vec<String>,
}

impl FromStr for PluginVersion {
    type Err = PluginError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (core_pre, build) = value
            .split_once('+')
            .map_or((value, None), |(core, build)| (core, Some(build)));
        if build.is_some_and(|build| build.contains('+')) {
            return Err(PluginError::InvalidVersion(value.to_owned()));
        }
        let (core, prerelease) = core_pre
            .split_once('-')
            .map_or((core_pre, None), |(core, prerelease)| {
                (core, Some(prerelease))
            });
        let numbers = core.split('.').collect::<Vec<_>>();
        if numbers.len() != 3 {
            return Err(PluginError::InvalidVersion(value.to_owned()));
        }
        let parse_number = |number: &str| {
            if number.is_empty()
                || !number.bytes().all(|byte| byte.is_ascii_digit())
                || (number.len() > 1 && number.starts_with('0'))
            {
                return None;
            }
            Some(number.to_owned())
        };
        let parse_identifiers = |part: Option<&str>, leading_zero_forbidden: bool| {
            let Some(part) = part else {
                return Ok(Vec::new());
            };
            let values = part.split('.').map(str::to_owned).collect::<Vec<_>>();
            if values.is_empty()
                || values.iter().any(|item| {
                    item.is_empty()
                        || !item
                            .bytes()
                            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                        || (leading_zero_forbidden
                            && item.bytes().all(|byte| byte.is_ascii_digit())
                            && item.len() > 1
                            && item.starts_with('0'))
                })
            {
                Err(PluginError::InvalidVersion(value.to_owned()))
            } else {
                Ok(values)
            }
        };
        Ok(Self {
            major: parse_number(numbers[0])
                .ok_or_else(|| PluginError::InvalidVersion(value.to_owned()))?,
            minor: parse_number(numbers[1])
                .ok_or_else(|| PluginError::InvalidVersion(value.to_owned()))?,
            patch: parse_number(numbers[2])
                .ok_or_else(|| PluginError::InvalidVersion(value.to_owned()))?,
            prerelease: parse_identifiers(prerelease, true)?,
            build: parse_identifiers(build, false)?,
        })
    }
}

impl fmt::Display for PluginVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if !self.prerelease.is_empty() {
            write!(formatter, "-{}", self.prerelease.join("."))?;
        }
        if !self.build.is_empty() {
            write!(formatter, "+{}", self.build.join("."))?;
        }
        Ok(())
    }
}

impl PluginVersion {
    /// Compares SemVer precedence. Build metadata is identity-bearing but has
    /// no effect on precedence as required by SemVer 2.0.0 section 10.
    #[must_use]
    pub fn precedence_cmp(&self, other: &Self) -> Ordering {
        for (left, right) in [
            (&self.major, &other.major),
            (&self.minor, &other.minor),
            (&self.patch, &other.patch),
        ] {
            let order = numeric_identifier_cmp(left, right);
            if order != Ordering::Equal {
                return order;
            }
        }
        match (self.prerelease.is_empty(), other.prerelease.is_empty()) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            _ => {}
        }
        for (left, right) in self.prerelease.iter().zip(&other.prerelease) {
            if left == right {
                continue;
            }
            let left_numeric = left.bytes().all(|byte| byte.is_ascii_digit());
            let right_numeric = right.bytes().all(|byte| byte.is_ascii_digit());
            return match (left_numeric, right_numeric) {
                (true, true) => numeric_identifier_cmp(left, right),
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                (false, false) => left.cmp(right),
            };
        }
        self.prerelease.len().cmp(&other.prerelease.len())
    }
}

impl Ord for PluginVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.precedence_cmp(other)
            .then_with(|| self.build.cmp(&other.build))
    }
}

impl PartialOrd for PluginVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn numeric_identifier_cmp(left: &str, right: &str) -> Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

impl Serialize for PluginVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PluginVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub manifest_version: u32,
    pub id: String,
    pub version: PluginVersion,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_host_version: Option<PluginVersion>,
    pub platforms: Vec<PlatformRequirement>,
    #[serde(default)]
    pub capabilities: Vec<CapabilityRequest>,
    pub components: Vec<Component>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: ComponentKind,
    pub path: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComponentKind {
    McpServer,
    ExternalTool,
    Skill,
    Command,
    Hook,
    NativeHelper,
    Assets,
}

impl ComponentKind {
    pub(crate) fn expects_directory(self) -> bool {
        matches!(self, Self::Skill | Self::Assets)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRequirement {
    pub os: OperatingSystem,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_version: Option<String>,
    #[serde(default)]
    pub architectures: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    #[serde(rename = "macos")]
    MacOs,
    Linux,
    Windows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostEnvironment {
    pub host_version: PluginVersion,
    pub operating_system: OperatingSystem,
    pub operating_system_version: String,
    pub architecture: String,
}

impl Manifest {
    pub fn validate(&self, host: &HostEnvironment) -> Result<(), PluginError> {
        if self.manifest_version != 1 {
            return Err(PluginError::InvalidManifest(
                "manifestVersion must be 1".to_owned(),
            ));
        }
        if !qualified_id(&self.id, true) {
            return Err(PluginError::InvalidManifest("invalid plugin id".to_owned()));
        }
        if !bounded(&self.display_name, 128) {
            return Err(PluginError::InvalidManifest(
                "invalid displayName".to_owned(),
            ));
        }
        if self
            .description
            .as_ref()
            .is_some_and(|value| !bounded(value, 2048))
        {
            return Err(PluginError::InvalidManifest(
                "invalid description".to_owned(),
            ));
        }
        if self.platforms.is_empty() || self.components.is_empty() {
            return Err(PluginError::InvalidManifest(
                "platforms and components must not be empty".to_owned(),
            ));
        }
        let platform_count = self
            .platforms
            .iter()
            .map(|platform| platform.os)
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        if platform_count != self.platforms.len() {
            return Err(PluginError::InvalidManifest(
                "duplicate platform".to_owned(),
            ));
        }
        for platform in &self.platforms {
            let architectures = platform
                .architectures
                .iter()
                .collect::<std::collections::BTreeSet<_>>();
            if architectures.len() != platform.architectures.len()
                || platform.architectures.iter().any(|architecture| {
                    architecture.is_empty()
                        || architecture.len() > 64
                        || !architecture
                            .bytes()
                            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
                })
                || platform
                    .minimum_version
                    .as_ref()
                    .is_some_and(|version| parse_numeric_version(version).is_none())
            {
                return Err(PluginError::InvalidManifest(
                    "invalid platform requirement".to_owned(),
                ));
            }
        }
        if self
            .minimum_host_version
            .as_ref()
            .is_some_and(|version| host.host_version.precedence_cmp(version).is_lt())
        {
            return Err(PluginError::Incompatible("host-version".to_owned()));
        }
        let platform = self
            .platforms
            .iter()
            .find(|platform| platform.os == host.operating_system)
            .ok_or_else(|| PluginError::Incompatible("operating-system".to_owned()))?;
        if platform
            .minimum_version
            .as_ref()
            .is_some_and(|required| numeric_version_lt(&host.operating_system_version, required))
        {
            return Err(PluginError::Incompatible(
                "operating-system-version".to_owned(),
            ));
        }
        if !platform.architectures.is_empty()
            && !platform.architectures.contains(&host.architecture)
        {
            return Err(PluginError::Incompatible("architecture".to_owned()));
        }
        let requested = self
            .capabilities
            .iter()
            .map(|capability| capability.id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        if requested.len() != self.capabilities.len()
            || self.capabilities.iter().any(|capability| {
                !qualified_id(&capability.id, true)
                    || capability
                        .reason
                        .as_ref()
                        .is_some_and(|value| !bounded(value, 512))
            })
        {
            return Err(PluginError::InvalidManifest(
                "invalid or duplicate capability".to_owned(),
            ));
        }
        let mut ids = std::collections::BTreeSet::new();
        let mut paths = std::collections::BTreeSet::new();
        for component in &self.components {
            let capabilities = component
                .capabilities
                .iter()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();
            if !qualified_id(&component.id, false)
                || !ids.insert(component.id.clone())
                || !safe_relative(&component.path)
                || !paths.insert(component.path.clone())
                || capabilities.len() != component.capabilities.len()
                || !capabilities.is_subset(&requested)
                || (component.kind == ComponentKind::NativeHelper
                    && !capabilities.contains("native-helper.execute"))
            {
                return Err(PluginError::InvalidManifest(format!(
                    "invalid component {}",
                    component.id
                )));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentProjection {
    pub owner_plugin_id: String,
    pub owner_version: PluginVersion,
    pub component_id: String,
    pub component_type: ComponentKind,
    pub package_path: PathBuf,
    pub component_path: PathBuf,
    pub data_path: PathBuf,
    pub granted_capabilities: Vec<String>,
    /// Always false in Slice 12. Runtime activation belongs to Slice 13/14.
    pub launchable: bool,
}

/// Stable cross-slice identity for one component owned by one plugin.
///
/// Runtime configuration stores this tuple rather than a path. The plugin
/// authority resolves it again for every generation so package updates cannot
/// retain an executable from an older receipt.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginComponentReference {
    pub plugin_id: String,
    pub component_id: String,
}

/// Immutable executable resolution for a plugin-owned component.
///
/// This is a generic lifecycle carrier. It neither identifies a specific
/// product plugin nor launches the executable; an MCP/runtime consumer owns
/// process policy and keys its peer generation with `plugin_generation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedPluginExecutable {
    pub reference: PluginComponentReference,
    pub component_type: ComponentKind,
    pub executable_path: PathBuf,
    pub data_path: PathBuf,
    pub plugin_generation: String,
    pub granted_capabilities: Vec<String>,
}

pub(crate) fn safe_relative(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 1024
        && !value.starts_with('/')
        && !value.starts_with('~')
        && !value.contains('\\')
        && value
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

pub(crate) fn qualified_id(value: &str, namespace: bool) -> bool {
    value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-' | b'_')
        })
        && !value.starts_with('.')
        && !value.ends_with('.')
        && !value.contains("..")
        && (!namespace || value.contains('.'))
        && value.split('.').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .next()
                    .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
                && part
                    .bytes()
                    .last()
                    .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        })
}

fn bounded(value: &str, maximum: usize) -> bool {
    !value.trim().is_empty() && value.chars().count() <= maximum
}

fn numeric_version_lt(current: &str, required: &str) -> bool {
    let (Some(mut current), Some(mut required)) = (
        parse_numeric_version(current),
        parse_numeric_version(required),
    ) else {
        return true;
    };
    let width = current.len().max(required.len());
    current.resize(width, 0);
    required.resize(width, 0);
    current < required
}

fn parse_numeric_version(value: &str) -> Option<Vec<u64>> {
    let values = value
        .split('.')
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    (1..=4).contains(&values.len()).then_some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_accepts_hyphens_inside_identifiers_and_orders_prereleases() {
        let version = "1.2.3-alpha-beta.1+darwin-arm64"
            .parse::<PluginVersion>()
            .expect("valid SemVer");
        assert_eq!(version.to_string(), "1.2.3-alpha-beta.1+darwin-arm64");
        assert!(
            "1.2.3-alpha.9"
                .parse::<PluginVersion>()
                .expect("left")
                .precedence_cmp(&"1.2.3-alpha.10".parse().expect("right"))
                .is_lt()
        );
        assert!(
            "1.2.3-rc.1"
                .parse::<PluginVersion>()
                .expect("candidate")
                .precedence_cmp(&"1.2.3".parse().expect("release"))
                .is_lt()
        );
        let first = "1.2.3+darwin-arm64"
            .parse::<PluginVersion>()
            .expect("first build");
        let second = "1.2.3+darwin-x86-64"
            .parse::<PluginVersion>()
            .expect("second build");
        assert_ne!(first, second);
        assert_eq!(first.precedence_cmp(&second), Ordering::Equal);
        assert_ne!(first.cmp(&second), Ordering::Equal);

        let beyond_u64 = "18446744073709551616.18446744073709551617.18446744073709551618"
            .parse::<PluginVersion>()
            .expect("SemVer core identifiers have no integer-size limit");
        assert_eq!(
            beyond_u64.to_string(),
            "18446744073709551616.18446744073709551617.18446744073709551618"
        );
        assert!(
            beyond_u64
                .precedence_cmp(
                    &"18446744073709551615.999999999999999999999999.999999999999999999999999"
                        .parse()
                        .expect("smaller core version"),
                )
                .is_gt()
        );
        let encoded = serde_json::to_string(&beyond_u64).expect("serialize large SemVer");
        assert_eq!(
            serde_json::from_str::<PluginVersion>(&encoded).expect("deserialize large SemVer"),
            beyond_u64
        );

        let manifest = Manifest {
            manifest_version: 1,
            id: "com.example.large-version".to_owned(),
            version: "1.0.0".parse().expect("package version"),
            display_name: "Large version".to_owned(),
            description: None,
            minimum_host_version: Some(
                "18446744073709551616.0.0"
                    .parse()
                    .expect("large minimum host version"),
            ),
            platforms: vec![PlatformRequirement {
                os: OperatingSystem::MacOs,
                minimum_version: None,
                architectures: vec![],
            }],
            capabilities: vec![],
            components: vec![Component {
                id: "peer".to_owned(),
                kind: ComponentKind::McpServer,
                path: "bin/peer".to_owned(),
                capabilities: vec![],
            }],
        };
        assert!(
            manifest
                .validate(&HostEnvironment {
                    host_version: "18446744073709551617.0.0"
                        .parse()
                        .expect("large host version"),
                    operating_system: OperatingSystem::MacOs,
                    operating_system_version: "15.0".to_owned(),
                    architecture: "arm64".to_owned(),
                })
                .is_ok()
        );
        assert_eq!(
            manifest.validate(&HostEnvironment {
                host_version: "18446744073709551615.999999999999999999999999.0"
                    .parse()
                    .expect("lower large host version"),
                operating_system: OperatingSystem::MacOs,
                operating_system_version: "15.0".to_owned(),
                architecture: "arm64".to_owned(),
            }),
            Err(PluginError::Incompatible("host-version".to_owned()))
        );
    }
}
