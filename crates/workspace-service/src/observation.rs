//! Scoped file observations for paths explicitly accessed by a session.
use crate::{Failure, fail, file_byte_page};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct FileObservations {
    paths: BTreeMap<(PathBuf, String), (String, Option<String>)>,
}

impl FileObservations {
    pub fn register(&mut self, root: &Path, relative: &str) -> Result<(), Failure> {
        let page = file_byte_page(root, relative, 0, 1)?;
        let key = (root.canonicalize()?, relative.to_owned());
        if !self.paths.contains_key(&key) && self.paths.len() >= 1024 {
            return Err(fail("observation-limit", "Too many observed files"));
        }
        // Existing observers must still see a change even if another reader
        // requests the file between its modification and the next observation.
        self.paths.entry(key).or_insert_with(|| {
            (
                page["absolutePath"].as_str().unwrap().to_owned(),
                Some(page["version"].as_str().unwrap().to_owned()),
            )
        });
        Ok(())
    }

    pub fn poll(&mut self) -> Result<Vec<Value>, Failure> {
        let mut next = self.paths.clone();
        let mut changes = Vec::new();
        for ((root, relative), (absolute, previous)) in &mut next {
            let version = match file_byte_page(root, relative, 0, 1) {
                Ok(page) => {
                    let current_path = page["absolutePath"].as_str().unwrap();
                    if current_path != absolute {
                        changes.push(json!({"kind":"change","change":{"absolutePath":absolute,"absent":true}}));
                        *absolute = current_path.to_owned();
                        *previous = None;
                    }
                    Some(page["version"].as_str().unwrap().to_owned())
                }
                Err(error) if error.code == "path-missing" => None,
                Err(error) => return Err(error),
            };
            if version != *previous {
                changes.push(match &version {
                    Some(version) => json!({"kind":"change","change":{"absolutePath":absolute,"version":version}}),
                    None => json!({"kind":"change","change":{"absolutePath":absolute,"absent":true}}),
                });
                *previous = version;
            }
        }
        self.paths = next;
        Ok(changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_delete_and_recreate_publish_once_and_reads_do_not_hide_changes() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("file");
        std::fs::write(&path, "first").unwrap();
        let mut observations = FileObservations::default();
        observations.register(root.path(), "file").unwrap();
        assert!(observations.poll().unwrap().is_empty());
        std::fs::write(&path, "changed").unwrap();
        observations.register(root.path(), "file").unwrap();
        let changes = observations.poll().unwrap();
        assert_eq!(changes.len(), 1);
        assert!(changes[0]["change"]["version"].is_string());
        assert!(observations.poll().unwrap().is_empty());
        std::fs::remove_file(&path).unwrap();
        assert_eq!(observations.poll().unwrap()[0]["change"]["absent"], true);
        assert!(observations.poll().unwrap().is_empty());
        std::fs::write(&path, "restored").unwrap();
        assert!(observations.poll().unwrap()[0]["change"]["version"].is_string());
    }
}
