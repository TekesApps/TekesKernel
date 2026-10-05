use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FixtureError {
    #[error("TEKES_KERNEL_FIXTURES must be an absolute existing directory: {0}")]
    InvalidOverride(String),
    #[error("fixture root not found; checked: {0:?}")]
    NotFound(Vec<PathBuf>),
    #[error("fixture IO failed at {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("fixture manifest JSON is invalid: {0}")]
    ManifestJson(#[from] serde_json::Error),
    #[error("fixture manifest mismatch in {corpus}: missing={missing:?}, extra={extra:?}")]
    ManifestMismatch {
        corpus: String,
        missing: BTreeSet<String>,
        extra: BTreeSet<String>,
    },
    #[error("asset {name} has digest {actual}")]
    AssetDigest { name: String, actual: String },
}

#[derive(Clone, Debug)]
pub struct FixtureRoot(PathBuf);

impl FixtureRoot {
    pub fn discover() -> Result<Self, FixtureError> {
        if let Some(value) = std::env::var_os("TEKES_KERNEL_FIXTURES") {
            let path = PathBuf::from(value);
            if path.is_absolute() && path.is_dir() {
                return Ok(Self(path));
            }
            return Err(FixtureError::InvalidOverride(path.display().to_string()));
        }
        Self::from_manifest_dir(Path::new(env!("CARGO_MANIFEST_DIR")))
    }

    pub fn from_manifest_dir(start: &Path) -> Result<Self, FixtureError> {
        let mut checked = Vec::new();
        for ancestor in start.ancestors() {
            checked.push(ancestor.to_path_buf());
            if ancestor.join("Cargo.toml").is_file()
                && ancestor.join("fixtures/manifest.json").is_file()
            {
                return Ok(Self(ancestor.join("fixtures")));
            }
        }
        Err(FixtureError::NotFound(checked))
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.0
    }

    #[must_use]
    pub fn join(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }

    pub fn verify_manifest(&self) -> Result<FixtureManifest, FixtureError> {
        let manifest_path = self.join("manifest.json");
        let bytes = read(&manifest_path)?;
        let manifest: FixtureManifest = serde_json::from_slice(&bytes)?;
        let mut disk_corpora = BTreeSet::new();
        for entry in fs::read_dir(&self.0).map_err(|source| FixtureError::Io {
            path: self.0.clone(),
            source,
        })? {
            let entry = entry.map_err(|source| FixtureError::Io {
                path: self.0.clone(),
                source,
            })?;
            if entry
                .file_type()
                .map_err(|source| FixtureError::Io {
                    path: entry.path(),
                    source,
                })?
                .is_dir()
            {
                disk_corpora.insert(entry.file_name().to_string_lossy().into_owned());
            }
        }
        let declared_corpora = manifest.corpora.keys().cloned().collect::<BTreeSet<_>>();
        let missing = declared_corpora
            .difference(&disk_corpora)
            .cloned()
            .collect::<BTreeSet<_>>();
        let extra = disk_corpora
            .difference(&declared_corpora)
            .cloned()
            .collect::<BTreeSet<_>>();
        if !missing.is_empty() || !extra.is_empty() {
            return Err(FixtureError::ManifestMismatch {
                corpus: "<root>".to_owned(),
                missing,
                extra,
            });
        }
        for (corpus, declared) in &manifest.corpora {
            let corpus_path = self.join(corpus);
            let mut disk = BTreeSet::new();
            collect_relative_files(&corpus_path, &corpus_path, &mut disk)?;
            let declared: BTreeSet<String> = declared.iter().cloned().collect();
            let missing = declared.difference(&disk).cloned().collect::<BTreeSet<_>>();
            let extra = disk.difference(&declared).cloned().collect::<BTreeSet<_>>();
            if !missing.is_empty() || !extra.is_empty() {
                return Err(FixtureError::ManifestMismatch {
                    corpus: corpus.clone(),
                    missing,
                    extra,
                });
            }
        }
        for name in manifest.corpora.get("assets").into_iter().flatten() {
            let bytes = read(&self.join("assets").join(name))?;
            let actual = format!("sha256-{:x}", Sha256::digest(bytes));
            if actual != *name {
                return Err(FixtureError::AssetDigest {
                    name: name.clone(),
                    actual,
                });
            }
        }
        Ok(manifest)
    }
}

fn collect_relative_files(
    root: &Path,
    current: &Path,
    output: &mut BTreeSet<String>,
) -> Result<(), FixtureError> {
    for entry in fs::read_dir(current).map_err(|source| FixtureError::Io {
        path: current.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|source| FixtureError::Io {
            path: current.to_path_buf(),
            source,
        })?;
        let file_type = entry.file_type().map_err(|source| FixtureError::Io {
            path: entry.path(),
            source,
        })?;
        if file_type.is_dir() {
            collect_relative_files(root, &entry.path(), output)?;
        } else if file_type.is_file() {
            let relative = entry
                .path()
                .strip_prefix(root)
                .expect("fixture descendant")
                .components()
                .map(|part| part.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            output.insert(relative);
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
pub struct FixtureManifest {
    pub version: u64,
    pub corpora: BTreeMap<String, Vec<String>>,
}

pub fn read(path: &Path) -> Result<Vec<u8>, FixtureError> {
    fs::read(path).map_err(|source| FixtureError::Io {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_rejects_undeclared_top_level_corpus() {
        let root = tempfile::tempdir().expect("fixture root");
        fs::write(
            root.path().join("manifest.json"),
            br#"{"corpora":{"known":["case.txt"]},"version":1}"#,
        )
        .expect("manifest");
        fs::create_dir(root.path().join("known")).expect("known corpus");
        fs::write(root.path().join("known/case.txt"), b"case").expect("case");
        fs::create_dir(root.path().join("undeclared")).expect("extra corpus");

        let error = FixtureRoot(root.path().to_path_buf())
            .verify_manifest()
            .expect_err("undeclared corpus must fail");
        match error {
            FixtureError::ManifestMismatch {
                corpus,
                missing,
                extra,
            } => {
                assert_eq!(corpus, "<root>");
                assert!(missing.is_empty());
                assert_eq!(extra, BTreeSet::from(["undeclared".to_owned()]));
            }
            other => panic!("unexpected error: {other}"),
        }
    }
}
