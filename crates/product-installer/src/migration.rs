//! Crash-resumable, rename-only legacy data migration. Journals contain identities, never data.
use crate::{fs, *};
use std::{collections::BTreeSet, os::unix::fs::MetadataExt};
#[derive(Clone)]
struct Unit {
    name: String,
    source: PathBuf,
    dest: PathBuf,
    device: u64,
    inode: u64,
    mode: u32,
}
impl Unit {
    fn value(&self) -> Value {
        json!({"name":self.name,"device":self.device,"inode":self.inode,"mode":self.mode})
    }
}
fn candidates(l: &Layout) -> Vec<(String, PathBuf, PathBuf)> {
    let names = [
        "threads",
        "archive",
        "staging",
        ".create-staging",
        ".rewrite-trash",
        "workspaces",
        "memory",
        "goals",
        "jobs",
        "tool-state",
        "credential-state",
        "config",
        "config-admin",
        "config-admin-v1",
        "cache",
        "endpoint-management",
        "endpoint-management-v2",
        "plugins",
    ];
    let mut v: Vec<_> = names
        .into_iter()
        .map(|n| (n.into(), l.base.join(n), l.data.join(n)))
        .collect();
    v.push((
        "kernel-logs".into(),
        l.home.join("Library/Logs/Tekes/Kernel"),
        l.logs.clone(),
    ));
    v
}
fn validate_tree(root: &Path, uid: u32) -> Result<()> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(p) = pending.pop() {
        let m = std::fs::symlink_metadata(&p).map_err(|_| Failure("unsafe-migration-source"))?;
        require(
            m.uid() == uid && m.mode() & 0o022 == 0 && (m.is_dir() || m.is_file()),
            "unsafe-migration-source",
        )?;
        if m.is_dir() {
            for e in std::fs::read_dir(p).map_err(|_| Failure("migration-io"))? {
                pending.push(e.map_err(|_| Failure("migration-io"))?.path());
            }
        }
    }
    Ok(())
}
fn validate_identity(u: &Unit, p: &Path, uid: u32, device: bool) -> Result<()> {
    fs::no_symlink_ancestors(p)?;
    let m = std::fs::symlink_metadata(p).map_err(|_| Failure("migration-verification-failed"))?;
    require(
        m.is_dir()
            && m.uid() == uid
            && (!device || m.dev() == u.device)
            && m.ino() == u.inode
            && m.mode() & 0o777 == u.mode,
        "migration-verification-failed",
    )?;
    validate_tree(p, uid)
}
fn validate_state(
    l: &Layout,
    phase: &str,
    units: &[Unit],
    completed: &BTreeSet<String>,
    uid: u32,
) -> Result<()> {
    let names: BTreeSet<_> = units.iter().map(|u| u.name.clone()).collect();
    require(
        names.len() == units.len()
            && completed.is_subset(&names)
            && (phase != "prepared" || completed.is_empty())
            && (phase != "completed" || *completed == names),
        "invalid-migration-journal",
    )?;
    for (name, src, dst) in candidates(l) {
        let source = fs::exists(&src)?;
        let dest = fs::exists(&dst)?;
        require(
            !source || names.contains(&name),
            "invalid-migration-journal",
        )?;
        let Some(u) = units.iter().find(|u| u.name == name) else {
            continue;
        };
        require(!(source && dest), "migration-conflict")?;
        if completed.contains(&name) {
            require(!source && dest, "migration-conflict")?;
            validate_identity(u, &dst, uid, phase != "completed")?;
        } else if source {
            validate_identity(u, &src, uid, true)?;
        } else if dest {
            validate_identity(u, &dst, uid, true)?;
        } else {
            return Err(Failure("migration-verification-failed"));
        }
    }
    Ok(())
}
fn write(p: &Path, phase: &str, units: &[Unit], completed: &BTreeSet<String>) -> Result<()> {
    fs::durable(
        p,
        &canonical(
            &json!({"format":1,"phase":phase,"units":units.iter().map(Unit::value).collect::<Vec<_>>(),"completed":completed}),
        )?,
        0o600,
    )
}
fn read(p: &Path, l: &Layout) -> Result<(String, Vec<Unit>, BTreeSet<String>)> {
    let e = "invalid-migration-journal";
    let (v, _) = fs::object(p).map_err(|_| Failure(e))?;
    require(
        keys(&v, &["completed", "format", "phase", "units"]) && v["format"].as_u64() == Some(1),
        e,
    )?;
    let phase = string(&v, "phase", e)?;
    require(["prepared", "moving", "completed"].contains(&phase), e)?;
    let mut units = vec![];
    let choices = candidates(l);
    for r in v["units"].as_array().ok_or(Failure(e))? {
        require(keys(r, &["device", "inode", "mode", "name"]), e)?;
        let name = string(r, "name", e)?;
        let (_, src, dst) = choices.iter().find(|c| c.0 == name).ok_or(Failure(e))?;
        let device = r["device"].as_u64().ok_or(Failure(e))?;
        let inode = r["inode"].as_u64().ok_or(Failure(e))?;
        let mode = r["mode"].as_u64().ok_or(Failure(e))?;
        require(mode <= 0o777, e)?;
        units.push(Unit {
            name: name.into(),
            source: src.clone(),
            dest: dst.clone(),
            device,
            inode,
            mode: mode as u32,
        });
    }
    let raw = v["completed"].as_array().ok_or(Failure(e))?;
    let completed = raw
        .iter()
        .map(|s| s.as_str().map(str::to_owned).ok_or(Failure(e)))
        .collect::<Result<BTreeSet<_>>>()?;
    require(completed.len() == raw.len(), e)?;
    Ok((phase.into(), units, completed))
}
pub fn migrate(l: &Layout) -> Result<()> {
    migrate_with(l, fs::uid(), |a, b| std::fs::rename(a, b), |_| Ok(()))
}
fn migrate_with(
    l: &Layout,
    uid: u32,
    mut rename: impl FnMut(&Path, &Path) -> std::io::Result<()>,
    mut checkpoint: impl FnMut(&str) -> Result<()>,
) -> Result<()> {
    let root = l.data.join("runtime/migrations/kernel-data-v1");
    fs::directory(&l.data)?;
    fs::directory(&l.data.join("runtime"))?;
    fs::directory(&l.data.join("runtime/migrations"))?;
    fs::directory(&root)?;
    let journal = root.join("operation.json");
    let (mut phase, units, mut completed) = if fs::exists(&journal)? {
        read(&journal, l)?
    } else {
        let mut units = vec![];
        for (name, source, dest) in candidates(l) {
            if !fs::exists(&source)? {
                continue;
            }
            let m = std::fs::symlink_metadata(&source)?;
            require(
                m.is_dir() && m.uid() == uid && m.mode() & 0o022 == 0,
                "unsafe-migration-source",
            )?;
            fs::no_symlink_ancestors(&source)?;
            validate_tree(&source, uid)?;
            require(!fs::exists(&dest)?, "migration-conflict")?;
            units.push(Unit {
                name,
                source,
                dest,
                device: m.dev(),
                inode: m.ino(),
                mode: m.mode() & 0o777,
            });
        }
        if units.is_empty() {
            return Ok(());
        }
        let completed = BTreeSet::new();
        write(&journal, "prepared", &units, &completed)?;
        checkpoint("prepared")?;
        ("prepared".into(), units, completed)
    };
    validate_state(l, &phase, &units, &completed, uid)?;
    if phase == "completed" {
        return Ok(());
    }
    phase = "moving".into();
    write(&journal, &phase, &units, &completed)?;
    checkpoint("moving")?;
    for u in &units {
        if completed.contains(&u.name) {
            continue;
        }
        if fs::exists(&u.source)? {
            validate_identity(u, &u.source, uid, true)?;
            require(!fs::exists(&u.dest)?, "migration-conflict")?;
            let parent = u.dest.parent().ok_or(Failure("migration-io"))?;
            fs::directory(parent)?;
            rename(&u.source, &u.dest).map_err(|e| {
                Failure(if e.raw_os_error() == Some(libc::EXDEV) {
                    "migration-cross-device"
                } else {
                    "migration-io"
                })
            })?;
            fs::sync_dir(parent)?;
            fs::sync_dir(u.source.parent().ok_or(Failure("migration-io"))?)?;
            checkpoint(&format!("moved:{}", u.name))?;
        }
        validate_identity(u, &u.dest, uid, true)?;
        completed.insert(u.name.clone());
        write(&journal, &phase, &units, &completed)?;
        checkpoint(&format!("unit-completed:{}", u.name))?;
    }
    validate_state(l, "completed", &units, &completed, uid)?;
    checkpoint("before-completed")?;
    write(&journal, "completed", &units, &completed)?;
    checkpoint("completed")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() -> (tempfile::TempDir, Layout) {
        let d = tempfile::tempdir().unwrap();
        let home = std::fs::canonicalize(d.path()).unwrap();
        let l = Layout::macos(&home);
        fs::directory(&l.base.join("threads")).unwrap();
        std::fs::write(l.base.join("threads/data"), b"preserve").unwrap();
        (d, l)
    }
    #[test]
    fn interruption_after_rename_resumes_without_copying() {
        let (_d, l) = setup();
        let inode = std::fs::metadata(l.base.join("threads")).unwrap().ino();
        assert_eq!(
            migrate_with(
                &l,
                fs::uid(),
                |a, b| std::fs::rename(a, b),
                |s| if s == "moved:threads" {
                    Err(Failure("interrupted"))
                } else {
                    Ok(())
                }
            ),
            Err(Failure("interrupted"))
        );
        migrate(&l).unwrap();
        migrate(&l).unwrap();
        assert_eq!(std::fs::metadata(&l.threads).unwrap().ino(), inode);
        assert_eq!(std::fs::read(l.threads.join("data")).unwrap(), b"preserve");
    }
    #[test]
    fn conflicting_destination_never_overwrites() {
        let (_d, l) = setup();
        fs::directory(&l.threads).unwrap();
        assert_eq!(migrate(&l), Err(Failure("migration-conflict")));
        assert!(l.base.join("threads/data").exists());
    }
    #[test]
    fn cross_device_failure_preserves_source() {
        let (_d, l) = setup();
        assert_eq!(
            migrate_with(
                &l,
                fs::uid(),
                |_, _| Err(std::io::Error::from_raw_os_error(libc::EXDEV)),
                |_| Ok(())
            ),
            Err(Failure("migration-cross-device"))
        );
        assert!(l.base.join("threads/data").exists());
    }
    #[test]
    fn symlink_source_is_rejected() {
        let (_d, l) = setup();
        std::os::unix::fs::symlink("data", l.base.join("threads/link")).unwrap();
        assert_eq!(migrate(&l), Err(Failure("unsafe-migration-source")));
    }
    #[test]
    fn completed_journal_does_not_hide_new_legacy_data() {
        let (_d, l) = setup();
        migrate(&l).unwrap();
        fs::directory(&l.base.join("config")).unwrap();
        assert_eq!(migrate(&l), Err(Failure("invalid-migration-journal")));
    }
    fn journal(l: &Layout) -> PathBuf {
        l.data
            .join("runtime/migrations/kernel-data-v1/operation.json")
    }
    fn interrupt(l: &Layout, point: &str) {
        assert_eq!(
            migrate_with(
                l,
                fs::uid(),
                |a, b| std::fs::rename(a, b),
                |s| if s == point {
                    Err(Failure("interrupted"))
                } else {
                    Ok(())
                }
            ),
            Err(Failure("interrupted"))
        );
    }
    fn edit(l: &Layout, f: impl FnOnce(&mut Value)) {
        let (mut v, _) = fs::object(&journal(l)).unwrap();
        f(&mut v);
        fs::durable(&journal(l), &canonical(&v).unwrap(), 0o600).unwrap();
    }
    #[test]
    fn every_crash_checkpoint_recovers() {
        for point in [
            "prepared",
            "moving",
            "moved:threads",
            "unit-completed:threads",
            "before-completed",
            "completed",
        ] {
            let (_d, l) = setup();
            interrupt(&l, point);
            migrate(&l).unwrap();
            assert_eq!(std::fs::read(l.threads.join("data")).unwrap(), b"preserve");
        }
    }
    #[test]
    fn completed_migration_survives_mount_renumbering() {
        let (_d, l) = setup();
        migrate(&l).unwrap();
        edit(&l, |v| {
            v["units"][0]["device"] = json!(v["units"][0]["device"].as_u64().unwrap() + 1)
        });
        migrate(&l).unwrap();
    }
    #[test]
    fn forged_or_omitted_units_fail_closed() {
        for omit_all in [true, false] {
            let (_d, l) = setup();
            fs::directory(&l.base.join("archive")).unwrap();
            interrupt(&l, "prepared");
            edit(&l, |v| {
                v["phase"] = json!("moving");
                if omit_all {
                    v["units"] = json!([])
                } else {
                    v["units"]
                        .as_array_mut()
                        .unwrap()
                        .retain(|u| u["name"] != "archive");
                }
            });
            assert_eq!(migrate(&l), Err(Failure("invalid-migration-journal")));
        }
    }
    #[test]
    fn forged_completion_and_inode_are_rejected() {
        let (_d, l) = setup();
        interrupt(&l, "prepared");
        edit(&l, |v| {
            v["phase"] = json!("moving");
            v["completed"] = json!(["threads"]);
        });
        assert_eq!(migrate(&l), Err(Failure("migration-conflict")));
        let (_d, l) = setup();
        interrupt(&l, "moved:threads");
        edit(&l, |v| {
            v["completed"] = json!(["threads"]);
            v["units"][0]["inode"] = json!(v["units"][0]["inode"].as_u64().unwrap() + 1);
        });
        assert_eq!(migrate(&l), Err(Failure("migration-verification-failed")));
    }
    #[test]
    fn duplicate_entries_are_rejected() {
        for completed in [true, false] {
            let (_d, l) = setup();
            interrupt(&l, "prepared");
            edit(&l, |v| {
                if completed {
                    v["phase"] = json!("moving");
                    v["completed"] = json!(["threads", "threads"])
                } else {
                    let unit = v["units"][0].clone();
                    v["units"].as_array_mut().unwrap().push(unit);
                }
            });
            assert_eq!(migrate(&l), Err(Failure("invalid-migration-journal")));
        }
    }
    #[test]
    fn malformed_and_unknown_journals_are_rejected() {
        for malformed in [true, false] {
            let (_d, l) = setup();
            interrupt(&l, "prepared");
            if malformed {
                std::fs::write(journal(&l), b"{\"format\":1").unwrap();
            } else {
                edit(&l, |v| v["unexpected"] = json!(true));
            }
            assert_eq!(migrate(&l), Err(Failure("invalid-migration-journal")));
        }
    }
    #[test]
    fn wrong_owner_and_writable_source_are_rejected() {
        let (_d, l) = setup();
        assert_eq!(
            migrate_with(&l, fs::uid() + 1, |a, b| std::fs::rename(a, b), |_| Ok(())),
            Err(Failure("unsafe-migration-source"))
        );
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            l.base.join("threads"),
            std::fs::Permissions::from_mode(0o777),
        )
        .unwrap();
        assert_eq!(migrate(&l), Err(Failure("unsafe-migration-source")));
    }
}
