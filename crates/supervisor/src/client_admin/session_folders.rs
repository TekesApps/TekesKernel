//! Session-owned folder mutations. The append is the durable CAS and receipt;
//! workspace documents are never edited and running writers are never replayed.
use super::*;
use crate::endpoint_carrier::SupervisorSessionAuthority;

pub const SESSION_FOLDER_METHODS: [(&str, MethodClass); 3] = [
    ("session.listFolders", MethodClass::ReadOnly),
    ("session.addFolder", MethodClass::Mutation),
    ("session.removeFolder", MethodClass::Mutation),
];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct SessionFoldersGet {
    session_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct SessionFolderChange {
    session_id: String,
    expected_revision: u64,
    path: String,
}

fn revision(events: &[schema::Event]) -> u64 {
    events
        .iter()
        .rev()
        .find(|event| {
            event.kind() == &schema::EventKind::Meta
                && serde_json::to_value(event.raw())
                    .ok()
                    .is_some_and(|raw| raw.get("scope_change").is_some())
        })
        .map_or(1, schema::Event::seq)
}

impl ClientAdminRoutes {
    pub(super) fn session_folders(
        &self,
        input: SessionFoldersGet,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        endpoint::validate_session_id(&input.session_id)
            .map_err(|error| internal(error.to_string()))?;
        let scope = profile::SessionScope::locate(&self.root, &input.session_id)
            .map_err(internal_profile)?;
        let bytes = fs::read(scope.session_folder().join("main.jsonl")).map_err(internal_io)?;
        let projection = store::scan_valid_prefix(&bytes, schema::READER_VERSION)
            .projection
            .ok_or_else(|| internal("session ledger has no genesis"))?;
        let raw = schema::effective_session_genesis(&projection.events)
            .map_err(|error| internal(error.to_string()))?
            .expect("validated genesis");
        let scope = profile::SessionScope::from_genesis_value(scope.session_folder(), &raw)
            .map_err(internal_profile)?;
        let folders = scope.folders(&self.repository).map_err(internal_profile)?;
        to_ijson(
            &json!({"sessionId":input.session_id,"revision":revision(&projection.events),
            "folders":folders.into_iter().map(|folder| {
                let id = scope.session_config().and_then(|config| config.folder_refs.get(&folder.id)).unwrap_or(&folder.id);
                json!({"id":id,"path":folder.path})
            }).collect::<Vec<_>>()}),
        )
    }

    pub(super) fn change_session_folder(
        &self,
        request: &EndpointHostCall,
        input: SessionFolderChange,
        add: bool,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        endpoint::validate_session_id(&input.session_id)
            .map_err(|error| internal(error.to_string()))?;
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        let hash = management_request_hash(request)?;
        let store = store::ThreadStore::open(&self.root).map_err(internal_store)?;
        let origin = schema::OriginTuple {
            principal: principal.to_owned(),
            client: "session-folders".to_owned(),
            target: input.session_id.clone(),
            op: request.operation.clone(),
            key: request.rpc_id.clone(),
        };
        // Replays are reads, including after the original successful edit's
        // next worker has started. Do not wait for or append behind that worker.
        let old_scope = profile::SessionScope::locate(&self.root, &input.session_id)
            .map_err(internal_profile)?;
        let prefix =
            fs::read(old_scope.session_folder().join("main.jsonl")).map_err(internal_io)?;
        let projection = store::scan_valid_prefix(&prefix, schema::READER_VERSION)
            .projection
            .ok_or_else(|| internal("session genesis missing"))?;
        if let Some(seq) = projection.origin_tuples.get(&origin) {
            let event = projection
                .events
                .iter()
                .find(|event| event.seq() == *seq)
                .expect("origin projection");
            if event.string_field("request_sha256") != Some(&hash) {
                return Err(failure(
                    "idempotency-conflict",
                    "RPC id was reused with different parameters",
                    json!({"rpcId":request.rpc_id}),
                ));
            }
            return self.complete_session_folder_change(request, &input.session_id, *seq);
        }
        let _launch =
            NamedLock::try_exclusive(crate::process_host::workspace_quiescence_lock_path(
                &self.root,
                &format!("session:{}", input.session_id),
            ))
            .map_err(|error| match error {
                store::StoreError::Busy => failure(
                    "workspace-busy",
                    "Session has a running worker",
                    json!({"sessionId":input.session_id}),
                ),
                other => internal_store(other),
            })?;
        let mut rejected = None;
        let mut capture = None;
        let appended = store.append_keyed_with_projection(&input.session_id, &origin, |seq, projection| {
            let mut build = || -> Result<schema::Event, ProductionRouteFailure> {
                let actual = revision(&projection.events);
                if actual != input.expected_revision { return Err(stale(input.expected_revision, actual)); }
                let raw = schema::effective_session_genesis(&projection.events).map_err(|error| internal(error.to_string()))?
                    .ok_or_else(|| internal("session genesis missing"))?;
                let scope = profile::SessionScope::from_genesis_value(self.root.join("threads").join(&input.session_id), &raw).map_err(internal_profile)?;
                let catalog = profile::FolderCatalog::new(&self.root);
                let catalog_revision = catalog.revision().map_err(internal_profile)?;
                let mut seed = match scope.session_config() {
                    Some(config) => catalog.resolve(config).map_err(internal_profile)?,
                    None => {
                        // An old workspace is read once as a compatibility adapter.
                        let config = scope.workspace_config(&self.repository).map_err(internal_profile)?;
                        profile::WorkspaceConfig { format:1,revision:1,id:scope.workspace_id().to_owned(),name:config.workspace.name,
                            cwd:Vec::new(),folders:scope.folders(&self.repository).map_err(internal_profile)?.into_iter().map(|folder| profile::WorkspaceFolder{id:folder.id,path:folder.path}).collect(),policy:Some(config.workspace.policy) }
                    }
                };
                if seed.folders.is_empty() {
                    seed.folders = seed.folder_ids().into_iter().zip(seed.folder_paths()).map(|(id,path)| profile::WorkspaceFolder {id,path:path.to_owned()}).collect();
                    seed.cwd.clear();
                }
                let path = if add {
                    let canonical = Path::new(&input.path).canonicalize().map_err(|_| failure("workspace-invalid-path", "Workspace path is invalid", json!({"path":input.path})))?;
                    if !canonical.is_dir() || canonical.parent().is_none() || tools::protected_root_containing(&tools::kernel_protected_roots(&self.root), &canonical).is_some() {
                        return Err(failure("workspace-invalid-path", "Workspace path is invalid", json!({"path":input.path})));
                    }
                    canonical.to_string_lossy().into_owned()
                } else { input.path.clone() };
                let contains = seed.folders.iter().any(|folder| folder.path == path);
                if add && contains || !add && !contains {
                    return Err(failure("bad-request", "Session folder change has no effect", json!({})));
                }
                let mut binding = scope.folder_binding().map(str::to_owned).unwrap_or_else(|| seed.folders[0].id.clone());
                if add {
                    let id = format!("folder-{:x}", Sha256::digest(path.as_bytes()));
                    seed.folders.push(profile::WorkspaceFolder { id, path:path.clone() });
                    seed.policy.get_or_insert_with(profile::WorkspacePolicy::default).writable_roots.push(path);
                } else {
                    if seed.folders.len() == 1 { return Err(failure("workspace-last-folder", "Cannot remove the last folder", json!({"sessionId":input.session_id}))); }
                    seed.folders.retain(|folder| folder.path != path);
                    if !seed.folders.iter().any(|folder| folder.id == binding) { binding = seed.folders[0].id.clone(); }
                    if let Some(policy) = &mut seed.policy { policy.writable_roots.retain(|root| seed.folders.iter().any(|folder| Path::new(root).starts_with(&folder.path))); }
                }
                // Preserve immutable limits; only this session's selected folder set changes.
                let folders = seed.folder_ids();
                let config = self.repository.resolve_session_seed(&seed, Some(scope.session_folder()), Some(&binding), profile::ScopeSelection { environment:scope.environment(),folders:Some(&folders) }).map_err(internal_profile)?;
                let staged = catalog.begin_capture(&seed, Some(catalog_revision)).map_err(|error| match error {
                    profile::ProfileError::StaleRevision {..} => failure("workspace-busy", "Folder catalog changed during edit; retry with the same RPC id", json!({"sessionId":input.session_id})),
                    other => internal_profile(other),
                })?;
                let mut next = staged.scope().clone();
                next.compatibility_instructions = scope.session_config().is_none_or(|config| config.compatibility_instructions);
                let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis,true);
                let value = json!({"v":1,"seq":seq,"ts":timestamp,"kind":"meta","min_reader":schema::SESSION_SCOPE_READER,
                    "scope_change":{"session_scope":next,"folders":folders,"folder_binding":binding,"cwd":config.execution_cwd(),"environment":scope.environment()},
                    "request_sha256":hash,"origin_key":origin.key,"origin_tuple":origin});
                let event = schema::Event::from_value(to_ijson(&value)?).map_err(|error| internal(error.to_string()))?;
                staged.prepare(&scope.session_folder().join("main.jsonl"), &event).map_err(internal_profile)?;
                capture = Some(staged);
                Ok(event)
            };
            build().map_err(|error| { rejected = Some(error); store::StoreError::Corruption("session folder change rejected".to_owned()) })
        });
        // Settle even an ambiguous append error from the exact durable prefix.
        // A crash leaves the private intent for the next catalog access.
        if let Some(capture) = capture {
            capture.finish().map_err(internal_profile)?;
        }
        if let Some(error) = rejected {
            return Err(error);
        }
        let outcome = appended.map_err(|error| match error {
            store::StoreError::Busy => failure(
                "workspace-busy",
                "Session has a running worker",
                json!({"sessionId":input.session_id}),
            ),
            other => internal_store(other),
        })?;
        let projection = store
            .validated_line_projection(&input.session_id, "main.jsonl")
            .map_err(internal_store)?;
        let event = projection
            .events
            .iter()
            .find(|event| event.seq() == outcome.seq)
            .ok_or_else(|| internal("folder receipt missing"))?;
        if event.string_field("request_sha256") != Some(&hash) {
            return Err(failure(
                "idempotency-conflict",
                "RPC id was reused with different parameters",
                json!({"rpcId":request.rpc_id}),
            ));
        }
        self.complete_session_folder_change(request, &input.session_id, outcome.seq)
    }

    fn complete_session_folder_change(
        &self,
        request: &EndpointHostCall,
        session: &str,
        seq: u64,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        request
            .handoff
            .mark_handed_off(DurableHandoffProof {
                delivery: request.rpc_id.clone(),
                durable_identity: Some(RpcDurableIdentity {
                    kind: "ledger".to_owned(),
                    id: session.to_owned(),
                    seq: Some(seq),
                }),
            })
            .map_err(|error| internal(error.to_string()))?;
        self.process_host
            .reconcile_projection(session)
            .map_err(internal)?;
        crate::endpoint_host::SessionDeliveryAuthority::session_metadata_changed(
            self.process_host.as_ref(),
            session,
        );
        to_ijson(&json!({"sessionId":session,"revision":seq,"applies":"next-launch"}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use endpoint::DurableHandoffSignal;

    fn request(id: &str, method: &str, payload: Value) -> EndpointHostCall {
        EndpointHostCall {
            rpc_id: id.into(),
            operation: method.into(),
            payload: to_ijson(&payload).unwrap(),
            recovering: false,
            handoff: DurableHandoffSignal::new(),
        }
    }
    fn create(root: &Path, agents: &Path, path: &Path, key: &str) -> String {
        endpoint::ManagementStore::open_light(root)
            .unwrap()
            .create_session(endpoint::SessionCreateOperation {
                rpc_id: key,
                request_sha256: &"a".repeat(64),
                requested_session_id: None,
                workspace_id: None,
                cwd: Some(&path.to_string_lossy()),
                environment: None,
                identity_profile: None,
                preset: None,
                preset_mcp: None,
                preset_models: None,
                user_agent_dir: agents,
                started_at: "2026-10-16T00:00:00.000Z",
                principal: "test",
            })
            .unwrap()
    }
    fn json(value: IJsonValue) -> Value {
        serde_json::to_value(value).unwrap()
    }

    #[test]
    fn rejected_folder_edit_does_not_publish_catalog_or_ledger() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("kernel");
        let agents = temp.path().join("agents");
        let a = temp.path().join("a");
        let b = temp.path().join("b");
        for path in [&agents, &a, &b] {
            fs::create_dir_all(path).unwrap();
        }
        let host =
            ProductionProcessHost::open(&root, "/usr/bin/true", "atomicity", &agents).unwrap();
        let routes = ClientAdminRoutes::new(&root, host).unwrap();
        let session = create(&root, &agents, &a, "create");
        let folder = root.join("threads").join(&session);
        let ledger_before = fs::read(folder.join("main.jsonl")).unwrap();
        let catalog_before = fs::read(root.join("config/folders.json")).unwrap();
        // Force a resolution failure after path validation, with no ledger write.
        fs::write(
            store::session_settings_path(&folder).unwrap(),
            b"invalid settings",
        )
        .unwrap();
        let call = request(
            "add",
            "session.addFolder",
            json!({"sessionId":session,"expectedRevision":1,"path":b}),
        );
        assert!(
            routes
                .execute(&call, &serde_json::to_value(&call.payload).unwrap(), "test")
                .is_err()
        );
        assert_eq!(fs::read(folder.join("main.jsonl")).unwrap(), ledger_before);
        assert_eq!(
            fs::read(root.join("config/folders.json")).unwrap(),
            catalog_before
        );
        assert!(!call.handoff.is_durable());
        fs::remove_file(store::session_settings_path(&folder).unwrap()).unwrap();
        let retry = request(
            "add",
            "session.addFolder",
            json!({"sessionId":session,"expectedRevision":1,"path":b}),
        );
        let result = json(
            routes
                .execute(
                    &retry,
                    &serde_json::to_value(&retry.payload).unwrap(),
                    "test",
                )
                .unwrap(),
        );
        assert_eq!(result["revision"], 2);
        let committed = fs::read(folder.join("main.jsonl")).unwrap();
        let published = fs::read(root.join("config/folders.json")).unwrap();
        drop(routes);
        let host =
            ProductionProcessHost::open(&root, "/usr/bin/true", "atomicity", &agents).unwrap();
        let routes = ClientAdminRoutes::new(&root, host).unwrap();
        let retry = request(
            "add",
            "session.addFolder",
            json!({"sessionId":session,"expectedRevision":1,"path":b}),
        );
        assert_eq!(
            json(
                routes
                    .execute(
                        &retry,
                        &serde_json::to_value(&retry.payload).unwrap(),
                        "test"
                    )
                    .unwrap()
            ),
            result
        );
        assert_eq!(fs::read(folder.join("main.jsonl")).unwrap(), committed);
        assert_eq!(
            fs::read(root.join("config/folders.json")).unwrap(),
            published
        );
    }

    #[test]
    fn phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited()
     {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("kernel");
        let agents = temp.path().join("agents");
        let a = temp.path().join("a");
        let b = temp.path().join("b");
        for path in [&agents, &a, &b] {
            fs::create_dir_all(path).unwrap();
        }
        let a = a.canonicalize().unwrap();
        let b = b.canonicalize().unwrap();
        let host = ProductionProcessHost::open(&root, "/usr/bin/true", "phase7", &agents).unwrap();
        let routes = ClientAdminRoutes::new(&root, host).unwrap();
        let first = create(&root, &agents, &a, "first");
        let second = create(&root, &agents, &a, "second");
        let scope = profile::SessionScope::locate(&root, &first).unwrap();
        assert!(scope.is_detached());
        let workspace = scope.workspace_id().to_owned();
        let mut compatibility = routes.repository.workspace(&workspace).unwrap();
        compatibility.revision += 1;
        compatibility.policy.as_mut().unwrap().network = false;
        compatibility.folders.push(profile::WorkspaceFolder {
            id: "other".into(),
            path: b.to_string_lossy().into_owned(),
        });
        routes
            .repository
            .publish_workspace(compatibility.revision - 1, &compatibility)
            .unwrap();
        let config = scope.config(&routes.repository).unwrap();
        assert!(config.workspace.policy.network);
        assert_eq!(config.workspace.cwd, vec![a.to_string_lossy()]);
        fs::remove_dir_all(root.join("workspaces").join(&workspace)).unwrap();
        assert_eq!(scope.config(&routes.repository).unwrap(), config);
        let add = request(
            "add",
            "session.addFolder",
            json!({"sessionId":first,"expectedRevision":1,"path":b}),
        );
        let result = json(
            routes
                .execute(&add, &serde_json::to_value(&add.payload).unwrap(), "test")
                .unwrap(),
        );
        assert_eq!(result["revision"], 2);
        assert!(add.handoff.is_durable());
        let retry = request(
            "add",
            "session.addFolder",
            json!({"sessionId":first,"expectedRevision":1,"path":b}),
        );
        assert_eq!(
            json(
                routes
                    .execute(
                        &retry,
                        &serde_json::to_value(&retry.payload).unwrap(),
                        "test"
                    )
                    .unwrap()
            ),
            result
        );
        let running = NamedLock::shared(crate::process_host::workspace_quiescence_lock_path(
            &root,
            &format!("session:{first}"),
        ))
        .unwrap();
        assert_eq!(
            json(
                routes
                    .execute(
                        &retry,
                        &serde_json::to_value(&retry.payload).unwrap(),
                        "test"
                    )
                    .unwrap()
            ),
            result
        );
        let blocked = request(
            "busy",
            "session.removeFolder",
            json!({"sessionId":first,"expectedRevision":2,"path":a}),
        );
        assert_eq!(
            routes
                .execute(
                    &blocked,
                    &serde_json::to_value(&blocked.payload).unwrap(),
                    "test"
                )
                .unwrap_err()
                .code,
            "workspace-busy"
        );
        drop(running);
        let conflict = request(
            "add",
            "session.addFolder",
            json!({"sessionId":first,"expectedRevision":1,"path":a}),
        );
        assert_eq!(
            routes
                .execute(
                    &conflict,
                    &serde_json::to_value(&conflict.payload).unwrap(),
                    "test"
                )
                .unwrap_err()
                .code,
            "idempotency-conflict"
        );
        let stale = request(
            "stale",
            "session.removeFolder",
            json!({"sessionId":first,"expectedRevision":1,"path":a}),
        );
        assert_eq!(
            routes
                .execute(
                    &stale,
                    &serde_json::to_value(&stale.payload).unwrap(),
                    "test"
                )
                .unwrap_err()
                .code,
            "stale-revision"
        );
        assert_eq!(
            profile::SessionScope::locate(&root, &first)
                .unwrap()
                .folders(&routes.repository)
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            profile::SessionScope::locate(&root, &second)
                .unwrap()
                .folders(&routes.repository)
                .unwrap()
                .len(),
            1
        );
        let bytes = fs::read(root.join("threads").join(&first).join("main.jsonl")).unwrap();
        assert!(schema::validate_ledger(&bytes, 3).unwrap().read_only);
        assert!(!schema::validate_ledger(&bytes, 4).unwrap().read_only);
        let store = store::ThreadStore::open(&root).unwrap();
        let fork = "018f0000-0000-7000-8000-000000000007";
        store
            .begin_fork("phase7-fork", &first, fork, "2026-10-16T01:00:00.000Z")
            .unwrap();
        store.recover_rewrites().unwrap();
        assert_eq!(
            profile::SessionScope::locate(&root, fork)
                .unwrap()
                .folders(&routes.repository)
                .unwrap()
                .len(),
            2
        );
        let remove = request(
            "remove",
            "session.removeFolder",
            json!({"sessionId":first,"expectedRevision":2,"path":a}),
        );
        routes
            .execute(
                &remove,
                &serde_json::to_value(&remove.payload).unwrap(),
                "test",
            )
            .unwrap();
        let config = profile::SessionScope::locate(&root, &first)
            .unwrap()
            .config(&routes.repository)
            .unwrap();
        assert_eq!(config.execution_cwd(), Some(b.to_str().unwrap()));
        assert_eq!(
            config.workspace.policy.writable_roots,
            vec![b.to_string_lossy()]
        );
        let inventory = routes.process_host.inventory_items(true).unwrap().unwrap();
        let row = inventory
            .iter()
            .find(|row| row.session_id == first)
            .unwrap();
        let paths = routes.management().unwrap().folder_paths().unwrap();
        assert_eq!(row.resolved_cwd(&paths), b.to_string_lossy());
        let current = json(
            routes
                .session_folders(SessionFoldersGet {
                    session_id: first.clone(),
                })
                .unwrap(),
        )["revision"]
            .as_u64()
            .unwrap();
        let last = request(
            "last",
            "session.removeFolder",
            json!({"sessionId":first,"expectedRevision":current,"path":b}),
        );
        assert_eq!(
            routes
                .execute(&last, &serde_json::to_value(&last.payload).unwrap(), "test")
                .unwrap_err()
                .code,
            "workspace-last-folder"
        );
        assert_eq!(
            profile::SessionScope::locate(&root, fork)
                .unwrap()
                .folders(&routes.repository)
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("kernel");
        let agents = temp.path().join("agents");
        let a = temp.path().join("a");
        let b = temp.path().join("b");
        let moved = temp.path().join("moved");
        for path in [&agents, &a, &b, &moved] {
            fs::create_dir_all(path).unwrap();
        }
        let host = ProductionProcessHost::open(&root, "/usr/bin/true", "phase7", &agents).unwrap();
        let routes = ClientAdminRoutes::new(&root, host).unwrap();
        let session = create(&root, &agents, &a, "create");
        for (index, path) in [Path::new("/"), root.as_path()].into_iter().enumerate() {
            let call = request(
                &format!("guard-{index}"),
                "session.addFolder",
                json!({"sessionId":session,"expectedRevision":1,"path":path}),
            );
            assert_eq!(
                routes
                    .execute(&call, &serde_json::to_value(&call.payload).unwrap(), "test")
                    .unwrap_err()
                    .code,
                "workspace-invalid-path"
            );
            assert!(!call.handoff.is_durable());
        }
        let add = request(
            "add-b",
            "session.addFolder",
            json!({"sessionId":session,"expectedRevision":1,"path":b}),
        );
        routes
            .execute(&add, &serde_json::to_value(&add.payload).unwrap(), "test")
            .unwrap();
        let catalog = profile::FolderCatalog::new(&root);
        let before = catalog
            .folders()
            .unwrap()
            .into_iter()
            .find(|folder| folder.path == b.canonicalize().unwrap().to_string_lossy())
            .unwrap();
        let relocate = request(
            "relocate-b",
            "folders.relocate",
            json!({"previousPath":before.path,"path":moved}),
        );
        routes
            .execute(
                &relocate,
                &serde_json::to_value(&relocate.payload).unwrap(),
                "test",
            )
            .unwrap();
        routes
            .execute(
                &relocate,
                &serde_json::to_value(&relocate.payload).unwrap(),
                "test",
            )
            .unwrap();
        let after = catalog
            .folders()
            .unwrap()
            .into_iter()
            .find(|folder| folder.id == before.id)
            .unwrap();
        assert_eq!(after.path, moved.canonicalize().unwrap().to_string_lossy());
        assert!(
            !profile::FolderAuthorityStore::new(&root)
                .status(&moved)
                .unwrap()
                .trusted
        );
        let config = profile::SessionScope::locate(&root, &session)
            .unwrap()
            .config(&routes.repository)
            .unwrap();
        assert!(config.workspace.cwd.contains(&after.path));
        assert!(config.workspace.policy.writable_roots.contains(&after.path));
        assert!(!config.workspace.cwd.contains(&before.path));
    }
}
