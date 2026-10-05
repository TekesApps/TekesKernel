use crate::{fs, *};
use std::collections::BTreeSet;
#[derive(Debug)]
pub(crate) struct Artifact {
    pub root: PathBuf,
    pub release: PathBuf,
    pub version: String,
    // These fields belong to the macOS trust adapter; other platforms reject installation.
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub team: String,
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub group: String,
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub client_requirement: String,
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub selector_requirement: String,
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub supervisor_requirement: String,
    pub identity: Vec<u8>,
    pub identity_sha: String,
    pub product_sha: String,
}
impl Artifact {
    pub fn selector(&self) -> PathBuf {
        self.release.join("selector/bin/tekes-selector")
    }
    pub fn selector_manifest(&self) -> PathBuf {
        self.release.join("selector/manifest.canonical.json")
    }
    pub fn bundle(&self) -> PathBuf {
        self.release.join("bundle")
    }
    pub fn load(root: &Path) -> Result<Self> {
        let team = option_env!("TEKES_INSTALLER_TEAM_ID").ok_or(Failure("platform-unavailable"))?;
        let artifact = Self::manifest(root, team)?;
        platform::verify_artifact(&artifact)?;
        Ok(artifact)
    }

    fn manifest(root: &Path, expected_team: &str) -> Result<Self> {
        fs::no_symlink_tree(root)?;
        let names = std::fs::read_dir(root)?
            .map(|e| e.map(|e| e.file_name().to_string_lossy().into_owned()))
            .collect::<std::io::Result<BTreeSet<_>>>()?;
        require(
            names
                == ["installer", "product-manifest.canonical.json", "release"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            "invalid-artifact-layout",
        )?;
        let (p, pbytes) = fs::object(&root.join("product-manifest.canonical.json"))?;
        let e = "invalid-artifact-manifest";
        require(
            keys(
                &p,
                &[
                    "format",
                    "installer",
                    "protocol",
                    "release",
                    "team_id",
                    "version",
                ],
            ) && p["format"].as_u64() == Some(1)
                && p["protocol"] == "tekes-kernel-product-artifact-v1",
            e,
        )?;
        let version = string(&p, "version", e)?;
        require(identifier(version), e)?;
        let team = string(&p, "team_id", e)?;
        require(team == expected_team, e)?;
        let i = &p["installer"];
        let r = &p["release"];
        let installer_requirement = format!("anchor apple generic and identifier {IDENTIFIER}");
        require(
            keys(i, &["contract_sha256", "path", "requirement"])
                && i["path"] == "installer/TekesKernelInstaller.app"
                && i["requirement"] == installer_requirement
                && keys(
                    r,
                    &[
                        "bundle_manifest_sha256",
                        "install_identity_sha256",
                        "path",
                        "selector_manifest_sha256",
                    ],
                )
                && r["path"] == "release",
            e,
        )?;
        fs::digest(
            &root.join("installer/contract.canonical.json"),
            string(i, "contract_sha256", e)?,
        )?;
        require(
            fs::regular(&root.join("installer/contract.canonical.json"))? == CONTRACT,
            "invalid-artifact-contract",
        )?;
        let release = root.join("release");
        let identity_path = release.join("install-identity.canonical.json");
        let bundle_path = release.join("bundle/manifest.canonical.json");
        let selector_path = release.join("selector/manifest.canonical.json");
        let identity_sha = string(r, "install_identity_sha256", e)?;
        fs::digest(&identity_path, identity_sha)?;
        fs::digest(&bundle_path, string(r, "bundle_manifest_sha256", e)?)?;
        fs::digest(&selector_path, string(r, "selector_manifest_sha256", e)?)?;
        let (id, identity) = fs::object(&identity_path)?;
        let e = "invalid-artifact-identity";
        require(
            keys(
                &id,
                &[
                    "access_group",
                    "client_requirement",
                    "format",
                    "installer_requirement",
                    "selector_requirement",
                    "supervisor_requirement",
                    "team_id",
                ],
            ) && id["format"].as_u64() == Some(1)
                && id["team_id"] == team
                && id["access_group"] == format!("{team}.com.tekes.shared.endpoint")
                && id["installer_requirement"] == installer_requirement
                && id["client_requirement"]
                    == "anchor apple generic and identifier com.tekesapps.TekesUI",
            e,
        )?;
        let group = string(&id, "access_group", e)?;
        let client_requirement = string(&id, "client_requirement", e)?;
        let selector_requirement = string(&id, "selector_requirement", e)?;
        let supervisor_requirement = string(&id, "supervisor_requirement", e)?;
        let (bundle, _) = fs::object(&bundle_path)?;
        let expected = [
            "apps/TekesKernelSupervisor.app/Contents/Info.plist",
            "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
            "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json",
            "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources",
            "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile",
            "bin/tekes-helper",
            "bin/tekes-worker",
            "bin/tekes-workspace-service",
        ];
        let rows = bundle["files"]
            .as_array()
            .ok_or(Failure("invalid-artifact-bundle"))?;
        require(
            bundle["version"] == version
                && rows.len() == expected.len()
                && rows.iter().zip(expected).all(|(row, p)| row["path"] == p),
            "invalid-artifact-bundle",
        )?;
        for row in rows {
            let e = "invalid-artifact-bundle-file";
            require(keys(row, &["bytes", "mode", "path", "sha256"]), e)?;
            let path = release.join("bundle").join(string(row, "path", e)?);
            let bytes = fs::regular(&path)?;
            let mode = string(row, "mode", e)?;
            require(
                row["bytes"].as_u64() == Some(bytes.len() as u64)
                    && string(row, "sha256", e)? == fs::sha(&bytes)
                    && fs::mode(&path)? == if mode == "0755" { 0o755 } else { 0o644 },
                "invalid-artifact",
            )?;
        }
        let (selector, _) = fs::object(&selector_path)?;
        let f = &selector["file"];
        let e = "invalid-artifact-selector-manifest";
        require(
            selector["version"] == version
                && f["path"] == "selector/bin/tekes-selector"
                && f["mode"] == "0755",
            e,
        )?;
        let bytes = fs::regular(&release.join("selector/bin/tekes-selector"))?;
        require(
            f["bytes"].as_u64() == Some(bytes.len() as u64)
                && string(f, "sha256", e)? == fs::sha(&bytes),
            "invalid-artifact-selector-file",
        )?;
        let artifact = Self {
            root: root.into(),
            release,
            version: version.into(),
            team: team.into(),
            group: group.into(),
            client_requirement: client_requirement.into(),
            selector_requirement: selector_requirement.into(),
            supervisor_requirement: supervisor_requirement.into(),
            identity,
            identity_sha: identity_sha.into(),
            product_sha: fs::sha(&pbytes),
        };
        Ok(artifact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    fn write(p: &Path, v: &Value) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, canonical(v).unwrap()).unwrap();
    }
    fn fixture() -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let root = std::fs::canonicalize(dir.path()).unwrap();
        let release = root.join("release");
        std::fs::create_dir_all(root.join("installer")).unwrap();
        std::fs::write(root.join("installer/contract.canonical.json"), CONTRACT).unwrap();
        let identity = json!({"access_group":"TEKESAPP01.com.tekes.shared.endpoint","client_requirement":"anchor apple generic and identifier com.tekesapps.TekesUI","format":1,"installer_requirement":"anchor apple generic and identifier com.tekes.kernel.installer","selector_requirement":"selector","supervisor_requirement":"supervisor","team_id":"TEKESAPP01"});
        write(&release.join("install-identity.canonical.json"), &identity);
        let mut rows = vec![];
        for name in [
            "apps/TekesKernelSupervisor.app/Contents/Info.plist",
            "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
            "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json",
            "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources",
            "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile",
            "bin/tekes-helper",
            "bin/tekes-worker",
            "bin/tekes-workspace-service",
        ] {
            let p = release.join("bundle").join(name);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, b"fixture").unwrap();
            std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).unwrap();
            rows.push(json!({"bytes":7,"mode":"0644","path":name,"sha256":fs::sha(b"fixture")}));
        }
        write(
            &release.join("bundle/manifest.canonical.json"),
            &json!({"version":"test-v1","files":rows}),
        );
        let selector = release.join("selector/bin/tekes-selector");
        std::fs::create_dir_all(selector.parent().unwrap()).unwrap();
        std::fs::write(selector, b"selector").unwrap();
        write(
            &release.join("selector/manifest.canonical.json"),
            &json!({"version":"test-v1","file":{"path":"selector/bin/tekes-selector","mode":"0755","bytes":8,"sha256":fs::sha(b"selector")}}),
        );
        write(
            &root.join("product-manifest.canonical.json"),
            &json!({"format":1,"installer":{"contract_sha256":fs::sha(CONTRACT),"path":"installer/TekesKernelInstaller.app","requirement":"anchor apple generic and identifier com.tekes.kernel.installer"},"protocol":"tekes-kernel-product-artifact-v1","release":{"bundle_manifest_sha256":fs::sha(&std::fs::read(release.join("bundle/manifest.canonical.json")).unwrap()),"install_identity_sha256":fs::sha(&canonical(&identity).unwrap()),"path":"release","selector_manifest_sha256":fs::sha(&std::fs::read(release.join("selector/manifest.canonical.json")).unwrap())},"team_id":"TEKESAPP01","version":"test-v1"}),
        );
        (dir, root)
    }
    #[test]
    fn artifact_integrity_rejects_wrong_team_and_changed_payload() {
        let (_d, root) = fixture();
        assert!(Artifact::manifest(&root, "TEKESAPP01").is_ok());
        assert_eq!(
            Artifact::manifest(&root, "OTHERAPP01").unwrap_err(),
            Failure("invalid-artifact-manifest")
        );
        std::fs::write(root.join("release/bundle/bin/tekes-worker"), b"modified").unwrap();
        assert_eq!(
            Artifact::manifest(&root, "TEKESAPP01").unwrap_err(),
            Failure("invalid-artifact")
        );
    }
    #[test]
    fn artifact_rejects_noncanonical_manifest_and_symlink() {
        let (_d, root) = fixture();
        let path = root.join("product-manifest.canonical.json");
        let mut bytes = std::fs::read(&path).unwrap();
        bytes.push(b'\n');
        std::fs::write(&path, bytes).unwrap();
        assert_eq!(
            Artifact::manifest(&root, "TEKESAPP01").unwrap_err(),
            Failure("invalid-artifact")
        );
        let (_d, root) = fixture();
        std::os::unix::fs::symlink("tekes-worker", root.join("release/bundle/bin/link")).unwrap();
        assert!(Artifact::manifest(&root, "TEKESAPP01").is_err());
    }
}
