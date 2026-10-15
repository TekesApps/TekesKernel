use profile::{FolderAuthorityStore, InstructionResolver, ResourceCatalog};
use std::fs;

#[test]
fn project_execution_is_opt_in_and_old_snapshots_and_resources_lose_authority() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("state");
    let project = root.path().join("project");
    fs::create_dir_all(project.join(".agents/hooks")).unwrap();
    fs::create_dir_all(project.join(".agents/skills/demo")).unwrap();
    fs::write(project.join("AGENTS.md"), "always readable").unwrap();
    fs::write(project.join(".agents/hooks/pre.txt"), "invalid until trust").unwrap();
    fs::write(
        project.join(".agents/skills/demo/SKILL.md"),
        "---\nname: demo\ndescription: demo\n---\nbody",
    )
    .unwrap();
    let resolver = InstructionResolver::new(root.path().join("user"), [&project])
        .with_folder_authority(&state);
    let snapshot = resolver.capture().unwrap();
    assert_eq!(snapshot.sources.len(), 1);
    assert_eq!(snapshot.sources[0].content, "always readable");
    let authority = FolderAuthorityStore::new(&state);
    let request = authority.request("request", 0, &project).unwrap();
    assert!(resolver.capture().unwrap().effective.skills.is_empty());
    authority.grant_from_host("grant", 1, "request").unwrap();
    assert!(
        resolver.capture().is_err(),
        "trusted malformed hook must be diagnosed"
    );
    fs::write(
        project.join(".agents/hooks/pre.txt"),
        include_bytes!("../../../fixtures/instruction-tree/project0/.agent/hooks/pre.txt"),
    )
    .unwrap();
    let snapshot = resolver.capture().unwrap();
    let bytes = snapshot.canonical_bytes().unwrap();
    let frozen = profile::InstructionSnapshot::decode(&bytes).unwrap();
    let catalog = ResourceCatalog::from_snapshot(&frozen).unwrap();
    assert_eq!(catalog.skill("demo").unwrap().body, "body");
    let source = frozen
        .sources
        .iter()
        .find(|source| source.path == "hooks/pre.txt")
        .unwrap();
    let active = frozen.admit_source(source).unwrap();
    authority.revoke("revoke", 2, &request.folder.id).unwrap();
    assert!(frozen.admit_source(source).is_err());
    assert!(catalog.skill("demo").is_err());
    assert!(catalog.skill_summaries().is_empty());
    assert!(catalog.skill_packages().next().is_none());
    assert!(!authority.effects_drained(&request.folder.id).unwrap());
    drop(active);
    assert!(authority.effects_drained(&request.folder.id).unwrap());
    assert!(resolver.capture().unwrap().effective.hooks.is_empty());
}
