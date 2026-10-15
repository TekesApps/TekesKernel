# tekes-supervisor::phase5_tests

[Package atlas](index.md) · [Source](../../src/phase5_tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::phase5_tests::wait_until](../../src/phase5_tests.rs#L12) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::server](../../src/phase5_tests.rs#L19) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::script](../../src/phase5_tests.rs#L44) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::save](../../src/phase5_tests.rs#L70) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::workspace](../../src/phase5_tests.rs#L81) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts](../../src/phase5_tests.rs#L99) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke](../../src/phase5_tests.rs#L164) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::project_aliases_share_owner_but_session_scope_never_imports_extra_folders](../../src/phase5_tests.rs#L298) | function_item | `private` | test;  |
| [tekes-supervisor::phase5_tests::credential_generation_changes_pool_identity_without_reviving_the_old_peer](../../src/phase5_tests.rs#L379) | function_item | `private` | test;  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `MemorySecretStore` | `provider::MemorySecretStore` | `private` |
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `fs` | `std::fs` | `private` |
| `Path` | `std::path::Path` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Duration` | `std::time::Duration` | `private` |
| `Instant` | `std::time::Instant` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `wait_until` | `Instant::now` | [13](../../src/phase5_tests.rs#L13) | external-constructor-callback-or-unresolved |
| `wait_until` | `Duration::from_secs` | [13](../../src/phase5_tests.rs#L13) | external-constructor-callback-or-unresolved |
| `wait_until` | `condition` | [14](../../src/phase5_tests.rs#L14) | external-constructor-callback-or-unresolved |
| `wait_until` | `std::thread::sleep` | [16](../../src/phase5_tests.rs#L16) | external-constructor-callback-or-unresolved |
| `wait_until` | `Duration::from_millis` | [16](../../src/phase5_tests.rs#L16) | external-constructor-callback-or-unresolved |
| `server` | `workspace.into` | [23](../../src/phase5_tests.rs#L23) | receiver-type-required |
| `server` | `"fixture".into` | [25](../../src/phase5_tests.rs#L25) | receiver-type-required |
| `server` | `Some` | [33](../../src/phase5_tests.rs#L33) | external-constructor-callback-or-unresolved |
| `server` | `project.display().to_string` | [33](../../src/phase5_tests.rs#L33) | receiver-type-required |
| `server` | `project.display` | [33](../../src/phase5_tests.rs#L33) | receiver-type-required |
| `server` | `BTreeMap::new` | [34](../../src/phase5_tests.rs#L34) | external-constructor-callback-or-unresolved |
| `script` | `project.join` | [45](../../src/phase5_tests.rs#L45) | receiver-type-required |
| `script` | `fs::write(&script,r#"import os,json,sys,time,subprocess if os.path.exists('descendant'):  child=subprocess.Popen(['/bin/sleep','30'])  with open('child-pid','w') as f:f.write(str(child.pid)) with open('starts','a') as f:f.write(str(os.getpid())+'\n') for line in sys.stdin:  r=json.loads(line)  if 'id' not in r:continue  m=r['method']  if m=='initialize':out={'protocolVersion':'2025-11-25','capabilities':{'tools':{},'resources':{}},'serverInfo':{'name':'fixture','version':'1'}}  elif m=='tools/list':out={'tools':[{'name':'ping','description':'ping','annotations':{'readOnlyHint':True},'inputSchema':{'type':'object','properties':{},'required':[],'additionalProperties':False}}]}  elif m=='tools/call':   with open('calls','a') as f:f.write('call\n')   if os.path.exists('block'):    open('entered','w').close()    while not os.path.exists('release'):time.sleep(.01)   out={'content':[{'type':'text','text':'ok'}]}  elif m=='resources/list':out={'resources':[{'uri':'fixture://one','name':'one'}]}  elif m=='resources/read':out={'contents':[{'uri':'fixture://one','text':'ok'}]}  else:raise RuntimeError(m)  print(json.dumps({'jsonrpc':'2.0','id':r['id'],'result':out}),flush=True) "#).unwrap` | [46](../../src/phase5_tests.rs#L46) | receiver-type-required |
| `script` | `fs::write` | [46](../../src/phase5_tests.rs#L46) | external-constructor-callback-or-unresolved |
| `save` | `runtime         .mutate(             op,             &McpManagementMutation::Save {                 server,                 credential_fields: BTreeMap::new(),             },         )         .unwrap` | [71](../../src/phase5_tests.rs#L71) | receiver-type-required |
| `save` | `runtime         .mutate` | [71](../../src/phase5_tests.rs#L71) | receiver-type-required |
| `save` | `BTreeMap::new` | [76](../../src/phase5_tests.rs#L76) | external-constructor-callback-or-unresolved |
| `workspace` | `profile::ConfigRepository::open(state)         .unwrap()         .publish_workspace(             0,             &profile::WorkspaceConfig {                 format: 1,                 revision: 1,                 id: name.into(),                 name: name.into(),                 cwd: paths,                 folders: vec![],                 policy: Some(profile::WorkspacePolicy::default()),             },         )         .unwrap` | [82](../../src/phase5_tests.rs#L82) | receiver-type-required |
| `workspace` | `profile::ConfigRepository::open(state)         .unwrap()         .publish_workspace` | [82](../../src/phase5_tests.rs#L82) | receiver-type-required |
| `workspace` | `profile::ConfigRepository::open(state)         .unwrap` | [82](../../src/phase5_tests.rs#L82) | receiver-type-required |
| `workspace` | `profile::ConfigRepository::open` | [82](../../src/phase5_tests.rs#L82) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L814) |
| `workspace` | `name.into` | [89](../../src/phase5_tests.rs#L89), [90](../../src/phase5_tests.rs#L90) | receiver-type-required |
| `workspace` | `Some` | [93](../../src/phase5_tests.rs#L93) | external-constructor-callback-or-unresolved |
| `workspace` | `profile::WorkspacePolicy::default` | [93](../../src/phase5_tests.rs#L93) | external-constructor-callback-or-unresolved |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `tempfile::tempdir().unwrap` | [100](../../src/phase5_tests.rs#L100) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `tempfile::tempdir` | [100](../../src/phase5_tests.rs#L100) | external-constructor-callback-or-unresolved |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `root.path().join` | [101](../../src/phase5_tests.rs#L101), [102](../../src/phase5_tests.rs#L102) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `root.path` | [101](../../src/phase5_tests.rs#L101), [102](../../src/phase5_tests.rs#L102) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `fs::create_dir(&project).unwrap` | [103](../../src/phase5_tests.rs#L103) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `fs::create_dir` | [103](../../src/phase5_tests.rs#L103) | external-constructor-callback-or-unresolved |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `project.canonicalize().unwrap` | [104](../../src/phase5_tests.rs#L104) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `project.canonicalize` | [104](../../src/phase5_tests.rs#L104) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `workspace` | [105](../../src/phase5_tests.rs#L105), [106](../../src/phase5_tests.rs#L106) | [tekes-supervisor::phase5_tests::workspace](../../src/phase5_tests.rs#L81) |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `script` | [107](../../src/phase5_tests.rs#L107) | external-constructor-callback-or-unresolved |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `McpRuntime::open(state.join("config"), Arc::new(MemorySecretStore::new())).unwrap` | [109](../../src/phase5_tests.rs#L109) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `McpRuntime::open` | [109](../../src/phase5_tests.rs#L109) | external-constructor-callback-or-unresolved |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `state.join` | [109](../../src/phase5_tests.rs#L109) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `Arc::new` | [109](../../src/phase5_tests.rs#L109) | external-constructor-callback-or-unresolved |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `MemorySecretStore::new` | [109](../../src/phase5_tests.rs#L109) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `server` | [110](../../src/phase5_tests.rs#L110) | [tekes-supervisor::phase5_tests::server](../../src/phase5_tests.rs#L19) |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `first.clone` | [111](../../src/phase5_tests.rs#L111), [113](../../src/phase5_tests.rs#L113) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `"b".into` | [112](../../src/phase5_tests.rs#L112) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `save` | [113](../../src/phase5_tests.rs#L113), [114](../../src/phase5_tests.rs#L114), [149](../../src/phase5_tests.rs#L149) | [tekes-supervisor::phase5_tests::save](../../src/phase5_tests.rs#L70) |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `alias.clone` | [114](../../src/phase5_tests.rs#L114) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `runtime.prepare_workspace("a").unwrap` | [119](../../src/phase5_tests.rs#L119), [141](../../src/phase5_tests.rs#L141), [150](../../src/phase5_tests.rs#L150) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `runtime.prepare_workspace` | [119](../../src/phase5_tests.rs#L119), [120](../../src/phase5_tests.rs#L120), [141](../../src/phase5_tests.rs#L141), [150](../../src/phase5_tests.rs#L150) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `runtime.prepare_workspace("b").unwrap` | [120](../../src/phase5_tests.rs#L120) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `worker_control::ToolControl::new(         "018f0000-0000-7000-8000-000000000001",         "018f0000-0000-7000-8000-000000000001",         1,         "call",         b.catalog.tools[0].name.clone(),         schema::IJsonValue::parse(b"{}").unwrap(),     )     .unwrap` | [131](../../src/phase5_tests.rs#L131) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `worker_control::ToolControl::new` | [131](../../src/phase5_tests.rs#L131) | [worker-control::durable::ToolControl::new](../../../worker-control/src/durable.rs#L203) |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `b.catalog.tools[0].name.clone` | [136](../../src/phase5_tests.rs#L136) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `schema::IJsonValue::parse(b"{}").unwrap` | [137](../../src/phase5_tests.rs#L137) | receiver-type-required |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `schema::IJsonValue::parse` | [137](../../src/phase5_tests.rs#L137) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts` | `drop` | [140](../../src/phase5_tests.rs#L140), [142](../../src/phase5_tests.rs#L142) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `tempfile::tempdir().unwrap` | [165](../../src/phase5_tests.rs#L165) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `tempfile::tempdir` | [165](../../src/phase5_tests.rs#L165) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `root.path().join` | [166](../../src/phase5_tests.rs#L166), [167](../../src/phase5_tests.rs#L167) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `root.path` | [166](../../src/phase5_tests.rs#L166), [167](../../src/phase5_tests.rs#L167) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::create_dir(&project).unwrap` | [168](../../src/phase5_tests.rs#L168) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::create_dir` | [168](../../src/phase5_tests.rs#L168) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `project.canonicalize().unwrap` | [169](../../src/phase5_tests.rs#L169) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `project.canonicalize` | [169](../../src/phase5_tests.rs#L169) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `workspace` | [170](../../src/phase5_tests.rs#L170) | [tekes-supervisor::phase5_tests::workspace](../../src/phase5_tests.rs#L81) |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `script` | [171](../../src/phase5_tests.rs#L171) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `profile::FolderAuthorityStore::new` | [172](../../src/phase5_tests.rs#L172) | [profile::folder_authority::FolderAuthorityStore::new](../../../profile/src/folder_authority.rs#L86) |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.request("request", 0, &project).unwrap` | [173](../../src/phase5_tests.rs#L173) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.request` | [173](../../src/phase5_tests.rs#L173), [271](../../src/phase5_tests.rs#L271) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `server` | [174](../../src/phase5_tests.rs#L174) | [tekes-supervisor::phase5_tests::server](../../src/phase5_tests.rs#L19) |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `Some` | [175](../../src/phase5_tests.rs#L175), [198](../../src/phase5_tests.rs#L198) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `request.folder.id.clone` | [175](../../src/phase5_tests.rs#L175) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `Arc::new` | [176](../../src/phase5_tests.rs#L176), [177](../../src/phase5_tests.rs#L177), [196](../../src/phase5_tests.rs#L196) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `McpRuntime::open(state.join("config"), Arc::new(MemorySecretStore::new())).unwrap` | [177](../../src/phase5_tests.rs#L177) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `McpRuntime::open` | [177](../../src/phase5_tests.rs#L177) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `state.join` | [177](../../src/phase5_tests.rs#L177), [198](../../src/phase5_tests.rs#L198) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `MemorySecretStore::new` | [177](../../src/phase5_tests.rs#L177), [196](../../src/phase5_tests.rs#L196) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `save` | [179](../../src/phase5_tests.rs#L179) | [tekes-supervisor::phase5_tests::save](../../src/phase5_tests.rs#L70) |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `config.clone` | [179](../../src/phase5_tests.rs#L179) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.grant_from_host("grant", 1, "request").unwrap` | [191](../../src/phase5_tests.rs#L191) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.grant_from_host` | [191](../../src/phase5_tests.rs#L191), [272](../../src/phase5_tests.rs#L272) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::write(project.join("descendant"), b"").unwrap` | [192](../../src/phase5_tests.rs#L192) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::write` | [192](../../src/phase5_tests.rs#L192), [205](../../src/phase5_tests.rs#L205), [229](../../src/phase5_tests.rs#L229) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `project.join` | [192](../../src/phase5_tests.rs#L192), [205](../../src/phase5_tests.rs#L205), [216](../../src/phase5_tests.rs#L216), [229](../../src/phase5_tests.rs#L229), [248](../../src/phase5_tests.rs#L248), [261](../../src/phase5_tests.rs#L261), [285](../../src/phase5_tests.rs#L285) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `runtime.prepare_workspace("a").unwrap` | [193](../../src/phase5_tests.rs#L193), [286](../../src/phase5_tests.rs#L286) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `runtime.prepare_workspace` | [193](../../src/phase5_tests.rs#L193), [286](../../src/phase5_tests.rs#L286) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `McpRegistryStore::new` | [198](../../src/phase5_tests.rs#L198) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `super::server_pool_key(&config, None, &secrets).unwrap` | [201](../../src/phase5_tests.rs#L201), [273](../../src/phase5_tests.rs#L273) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `super::server_pool_key` | [201](../../src/phase5_tests.rs#L201), [273](../../src/phase5_tests.rs#L273) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `runtime.broker.handle` | [204](../../src/phase5_tests.rs#L204) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::write(project.join("block"), b"").unwrap` | [205](../../src/phase5_tests.rs#L205) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `broker.clone` | [206](../../src/phase5_tests.rs#L206), [217](../../src/phase5_tests.rs#L217) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `old_key.clone` | [207](../../src/phase5_tests.rs#L207), [218](../../src/phase5_tests.rs#L218) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::thread::spawn` | [208](../../src/phase5_tests.rs#L208), [219](../../src/phase5_tests.rs#L219) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `first.call_tool` | [209](../../src/phase5_tests.rs#L209) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `schema::IJsonValue::parse(b"{}").unwrap` | [212](../../src/phase5_tests.rs#L212), [223](../../src/phase5_tests.rs#L223) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `schema::IJsonValue::parse` | [212](../../src/phase5_tests.rs#L212), [223](../../src/phase5_tests.rs#L223) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `McpCancellationToken::default` | [213](../../src/phase5_tests.rs#L213), [224](../../src/phase5_tests.rs#L224) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `wait_until` | [216](../../src/phase5_tests.rs#L216), [239](../../src/phase5_tests.rs#L239), [252](../../src/phase5_tests.rs#L252), [262](../../src/phase5_tests.rs#L262) | [tekes-supervisor::phase5_tests::wait_until](../../src/phase5_tests.rs#L12) |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `project.join("entered").exists` | [216](../../src/phase5_tests.rs#L216) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `second.call_tool` | [220](../../src/phase5_tests.rs#L220) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.revoke("revoke", 2, &request.folder.id).unwrap` | [227](../../src/phase5_tests.rs#L227) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.revoke` | [227](../../src/phase5_tests.rs#L227) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::write(project.join("release"), b"").unwrap` | [229](../../src/phase5_tests.rs#L229) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.effects_drained(&request.folder.id).unwrap` | [239](../../src/phase5_tests.rs#L239) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.effects_drained` | [239](../../src/phase5_tests.rs#L239) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::read_to_string(project.join("starts"))         .unwrap()         .trim()         .to_owned` | [248](../../src/phase5_tests.rs#L248) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::read_to_string(project.join("starts"))         .unwrap()         .trim` | [248](../../src/phase5_tests.rs#L248) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::read_to_string(project.join("starts"))         .unwrap` | [248](../../src/phase5_tests.rs#L248) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::read_to_string` | [248](../../src/phase5_tests.rs#L248), [261](../../src/phase5_tests.rs#L261) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", &pid])             .stdout(std::process::Stdio::null())             .stderr(std::process::Stdio::null())             .status()             .unwrap()             .success` | [253](../../src/phase5_tests.rs#L253) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", &pid])             .stdout(std::process::Stdio::null())             .stderr(std::process::Stdio::null())             .status()             .unwrap` | [253](../../src/phase5_tests.rs#L253) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", &pid])             .stdout(std::process::Stdio::null())             .stderr(std::process::Stdio::null())             .status` | [253](../../src/phase5_tests.rs#L253) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", &pid])             .stdout(std::process::Stdio::null())             .stderr` | [253](../../src/phase5_tests.rs#L253) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", &pid])             .stdout` | [253](../../src/phase5_tests.rs#L253) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args` | [253](../../src/phase5_tests.rs#L253), [263](../../src/phase5_tests.rs#L263) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new` | [253](../../src/phase5_tests.rs#L253), [263](../../src/phase5_tests.rs#L263) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Stdio::null` | [255](../../src/phase5_tests.rs#L255), [256](../../src/phase5_tests.rs#L256), [265](../../src/phase5_tests.rs#L265), [266](../../src/phase5_tests.rs#L266) | external-constructor-callback-or-unresolved |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::read_to_string(project.join("child-pid")).unwrap` | [261](../../src/phase5_tests.rs#L261) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", child_pid.trim()])             .stdout(std::process::Stdio::null())             .stderr(std::process::Stdio::null())             .status()             .unwrap()             .success` | [263](../../src/phase5_tests.rs#L263) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", child_pid.trim()])             .stdout(std::process::Stdio::null())             .stderr(std::process::Stdio::null())             .status()             .unwrap` | [263](../../src/phase5_tests.rs#L263) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", child_pid.trim()])             .stdout(std::process::Stdio::null())             .stderr(std::process::Stdio::null())             .status` | [263](../../src/phase5_tests.rs#L263) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", child_pid.trim()])             .stdout(std::process::Stdio::null())             .stderr` | [263](../../src/phase5_tests.rs#L263) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `std::process::Command::new("/bin/kill")             .args(["-0", child_pid.trim()])             .stdout` | [263](../../src/phase5_tests.rs#L263) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `child_pid.trim` | [264](../../src/phase5_tests.rs#L264) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.request("r2", 3, &project).unwrap` | [271](../../src/phase5_tests.rs#L271) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `authority.grant_from_host("g2", 4, "r2").unwrap` | [272](../../src/phase5_tests.rs#L272) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::remove_file(project.join("descendant")).unwrap` | [285](../../src/phase5_tests.rs#L285) | receiver-type-required |
| `project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke` | `fs::remove_file` | [285](../../src/phase5_tests.rs#L285) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `tempfile::tempdir().unwrap` | [299](../../src/phase5_tests.rs#L299) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `tempfile::tempdir` | [299](../../src/phase5_tests.rs#L299) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `root.path().join` | [300](../../src/phase5_tests.rs#L300), [301](../../src/phase5_tests.rs#L301), [302](../../src/phase5_tests.rs#L302) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `root.path` | [300](../../src/phase5_tests.rs#L300), [301](../../src/phase5_tests.rs#L301), [302](../../src/phase5_tests.rs#L302) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `fs::create_dir(&one).unwrap` | [303](../../src/phase5_tests.rs#L303) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `fs::create_dir` | [303](../../src/phase5_tests.rs#L303), [304](../../src/phase5_tests.rs#L304) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `fs::create_dir(&two).unwrap` | [304](../../src/phase5_tests.rs#L304) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `one.canonicalize().unwrap` | [305](../../src/phase5_tests.rs#L305) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `one.canonicalize` | [305](../../src/phase5_tests.rs#L305) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `two.canonicalize().unwrap` | [306](../../src/phase5_tests.rs#L306) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `two.canonicalize` | [306](../../src/phase5_tests.rs#L306) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `script` | [307](../../src/phase5_tests.rs#L307) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `workspace` | [308](../../src/phase5_tests.rs#L308), [313](../../src/phase5_tests.rs#L313) | [tekes-supervisor::phase5_tests::workspace](../../src/phase5_tests.rs#L81) |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `profile::FolderAuthorityStore::new` | [314](../../src/phase5_tests.rs#L314) | [profile::folder_authority::FolderAuthorityStore::new](../../../profile/src/folder_authority.rs#L86) |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `Vec::new` | [315](../../src/phase5_tests.rs#L315) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `[&one, &two].iter().enumerate` | [316](../../src/phase5_tests.rs#L316) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `[&one, &two].iter` | [316](../../src/phase5_tests.rs#L316) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `authority.registry_revision().unwrap` | [317](../../src/phase5_tests.rs#L317) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `authority.registry_revision` | [317](../../src/phase5_tests.rs#L317) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `authority.request(&req, rev, path).unwrap` | [319](../../src/phase5_tests.rs#L319) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `authority.request` | [319](../../src/phase5_tests.rs#L319) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `folders.push` | [320](../../src/phase5_tests.rs#L320) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `authority                 .grant_from_host(&format!("g{index}"), rev + 1, &req)                 .unwrap` | [321](../../src/phase5_tests.rs#L321) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `authority                 .grant_from_host` | [321](../../src/phase5_tests.rs#L321) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `McpRuntime::open(state.join("config"), Arc::new(MemorySecretStore::new())).unwrap` | [328](../../src/phase5_tests.rs#L328) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `McpRuntime::open` | [328](../../src/phase5_tests.rs#L328) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `state.join` | [328](../../src/phase5_tests.rs#L328) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `Arc::new` | [328](../../src/phase5_tests.rs#L328) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `MemorySecretStore::new` | [328](../../src/phase5_tests.rs#L328) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `server` | [329](../../src/phase5_tests.rs#L329), [333](../../src/phase5_tests.rs#L333) | [tekes-supervisor::phase5_tests::server](../../src/phase5_tests.rs#L19) |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `Some` | [330](../../src/phase5_tests.rs#L330), [335](../../src/phase5_tests.rs#L335) | external-constructor-callback-or-unresolved |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `folders[0].id.clone` | [330](../../src/phase5_tests.rs#L330) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `first.clone` | [331](../../src/phase5_tests.rs#L331), [336](../../src/phase5_tests.rs#L336) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `"b".into` | [332](../../src/phase5_tests.rs#L332) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `"second".into` | [334](../../src/phase5_tests.rs#L334) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `folders[1].id.clone` | [335](../../src/phase5_tests.rs#L335) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `save` | [336](../../src/phase5_tests.rs#L336), [337](../../src/phase5_tests.rs#L337), [338](../../src/phase5_tests.rs#L338), [372](../../src/phase5_tests.rs#L372) | [tekes-supervisor::phase5_tests::save](../../src/phase5_tests.rs#L70) |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `second.clone` | [338](../../src/phase5_tests.rs#L338) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `profile::ConfigRepository::open(&state)         .unwrap()         .resolve("a")         .unwrap` | [339](../../src/phase5_tests.rs#L339) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `profile::ConfigRepository::open(&state)         .unwrap()         .resolve` | [339](../../src/phase5_tests.rs#L339) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `profile::ConfigRepository::open(&state)         .unwrap` | [339](../../src/phase5_tests.rs#L339) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `profile::ConfigRepository::open` | [339](../../src/phase5_tests.rs#L339) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L814) |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `runtime.prepare_for_config(&config).unwrap` | [344](../../src/phase5_tests.rs#L344), [361](../../src/phase5_tests.rs#L361), [373](../../src/phase5_tests.rs#L373) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `runtime.prepare_for_config` | [344](../../src/phase5_tests.rs#L344), [361](../../src/phase5_tests.rs#L361), [373](../../src/phase5_tests.rs#L373) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `config.workspace.cwd.push` | [360](../../src/phase5_tests.rs#L360) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `two.display().to_string` | [360](../../src/phase5_tests.rs#L360) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `two.display` | [360](../../src/phase5_tests.rs#L360) | receiver-type-required |
| `project_aliases_share_owner_but_session_scope_never_imports_extra_folders` | `"fixture".into` | [371](../../src/phase5_tests.rs#L371) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `tempfile::tempdir().unwrap` | [380](../../src/phase5_tests.rs#L380) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `tempfile::tempdir` | [380](../../src/phase5_tests.rs#L380) | external-constructor-callback-or-unresolved |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `root.path().join` | [381](../../src/phase5_tests.rs#L381), [382](../../src/phase5_tests.rs#L382) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `root.path` | [381](../../src/phase5_tests.rs#L381), [382](../../src/phase5_tests.rs#L382) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `fs::create_dir(&project).unwrap` | [383](../../src/phase5_tests.rs#L383) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `fs::create_dir` | [383](../../src/phase5_tests.rs#L383) | external-constructor-callback-or-unresolved |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `project.canonicalize().unwrap` | [384](../../src/phase5_tests.rs#L384) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `project.canonicalize` | [384](../../src/phase5_tests.rs#L384) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `workspace` | [385](../../src/phase5_tests.rs#L385) | [tekes-supervisor::phase5_tests::workspace](../../src/phase5_tests.rs#L81) |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `script` | [386](../../src/phase5_tests.rs#L386) | external-constructor-callback-or-unresolved |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `Arc::new` | [387](../../src/phase5_tests.rs#L387) | external-constructor-callback-or-unresolved |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `MemorySecretStore::new` | [387](../../src/phase5_tests.rs#L387) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `secrets         .publish(             "fixture-key",             provider::SecretRecord::Active {                 generation: 1,                 material: "synthetic-one".into(),             },         )         .unwrap` | [388](../../src/phase5_tests.rs#L388) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `secrets         .publish` | [388](../../src/phase5_tests.rs#L388), [419](../../src/phase5_tests.rs#L419) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `"synthetic-one".into` | [393](../../src/phase5_tests.rs#L393) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `McpRuntime::open(state.join("config"), secrets.clone()).unwrap` | [397](../../src/phase5_tests.rs#L397) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `McpRuntime::open` | [397](../../src/phase5_tests.rs#L397) | external-constructor-callback-or-unresolved |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `state.join` | [397](../../src/phase5_tests.rs#L397) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `secrets.clone` | [397](../../src/phase5_tests.rs#L397) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `server` | [398](../../src/phase5_tests.rs#L398) | [tekes-supervisor::phase5_tests::server](../../src/phase5_tests.rs#L19) |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime         .mutate(             "save-key",             &McpManagementMutation::Save {                 server: config.clone(),                 credential_fields: [(                     "environment.FIXTURE".into(),                     McpCredentialFieldOperation::Replace {                         credential_id: "fixture-key".into(),                     },                 )]                 .into(),             },         )         .unwrap` | [399](../../src/phase5_tests.rs#L399) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime         .mutate` | [399](../../src/phase5_tests.rs#L399) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `config.clone` | [403](../../src/phase5_tests.rs#L403) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `[(                     "environment.FIXTURE".into(),                     McpCredentialFieldOperation::Replace {                         credential_id: "fixture-key".into(),                     },                 )]                 .into` | [404](../../src/phase5_tests.rs#L404) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `"environment.FIXTURE".into` | [405](../../src/phase5_tests.rs#L405) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `"fixture-key".into` | [407](../../src/phase5_tests.rs#L407) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime.get_server(&config.reference).unwrap().unwrap` | [414](../../src/phase5_tests.rs#L414) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime.get_server(&config.reference).unwrap` | [414](../../src/phase5_tests.rs#L414) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime.get_server` | [414](../../src/phase5_tests.rs#L414) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime.secret_access` | [415](../../src/phase5_tests.rs#L415) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `server_pool_key(&configured, None, &access).unwrap` | [416](../../src/phase5_tests.rs#L416), [428](../../src/phase5_tests.rs#L428) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `server_pool_key` | [416](../../src/phase5_tests.rs#L416), [428](../../src/phase5_tests.rs#L428) | external-constructor-callback-or-unresolved |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime.prepare_workspace("a").unwrap` | [417](../../src/phase5_tests.rs#L417), [442](../../src/phase5_tests.rs#L442) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `runtime.prepare_workspace` | [417](../../src/phase5_tests.rs#L417), [442](../../src/phase5_tests.rs#L442) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `secrets         .publish(             "fixture-key",             provider::SecretRecord::Active {                 generation: 2,                 material: "synthetic-two".into(),             },         )         .unwrap` | [419](../../src/phase5_tests.rs#L419) | receiver-type-required |
| `credential_generation_changes_pool_identity_without_reviving_the_old_peer` | `"synthetic-two".into` | [424](../../src/phase5_tests.rs#L424) | receiver-type-required |
