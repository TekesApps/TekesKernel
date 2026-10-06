//! Production providerAdmin.v1 and workspacePolicy.v1 endpoint routes.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use endpoint::{DurableHandoffProof, EndpointHostCall, MethodClass, RpcDurableIdentity};
use profile::{
    ConfigRepository, Model, Provider, ProvidersConfig, SettingsConfig, WorkspacePolicy,
};
use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use store::{AtomicPublisher, NamedLock};

use crate::endpoint_host::{ProductionEndpointRoutes, ProductionRouteFailure};
use crate::process_host::ProductionProcessHost;

pub const PROVIDER_ADMIN_METHODS: [(&str, MethodClass); 9] = [
    ("providers.list", MethodClass::ReadOnly),
    ("providers.verify", MethodClass::ReadOnly),
    ("providers.connections", MethodClass::ReadOnly),
    ("providers.connection.save", MethodClass::Mutation),
    ("providers.connection.delete", MethodClass::Mutation),
    ("providers.profiles", MethodClass::ReadOnly),
    ("providers.profile.save", MethodClass::Mutation),
    ("providers.profile.delete", MethodClass::Mutation),
    ("providers.profile.default", MethodClass::Mutation),
];

pub const WORKSPACE_POLICY_METHODS: [(&str, MethodClass); 2] = [
    ("workspace.policy.get", MethodClass::ReadOnly),
    ("workspace.policy.set", MethodClass::Mutation),
];

pub struct ClientAdminRoutes {
    provider_admin: bool,
    repository: ConfigRepository,
    process_host: Arc<ProductionProcessHost>,
    mutation_gate: Mutex<()>,
    journal: AdminMutationJournal,
}

impl ClientAdminRoutes {
    pub fn new(
        root: impl AsRef<std::path::Path>,
        process_host: Arc<ProductionProcessHost>,
    ) -> Result<Self, ProductionRouteFailure> {
        Self::open(root.as_ref(), process_host, true)
    }

    pub fn for_application(
        root: &std::path::Path,
        process_host: Arc<ProductionProcessHost>,
    ) -> Result<Self, ProductionRouteFailure> {
        Self::open(root, process_host, false)
    }

    fn open(
        root: &std::path::Path,
        process_host: Arc<ProductionProcessHost>,
        provider_admin: bool,
    ) -> Result<Self, ProductionRouteFailure> {
        let routes = Self {
            provider_admin,
            repository: ConfigRepository::open(root).map_err(internal_profile)?,
            process_host,
            mutation_gate: Mutex::new(()),
            journal: AdminMutationJournal::open(root).map_err(internal_store)?,
        };
        routes.recover_pending_intents()?;
        Ok(routes)
    }

    /// Returns the two independently negotiable administration capability
    /// groups. Both wrappers share the same durable authority and mutation
    /// journal; advertisement is nevertheless all-or-none within each group.
    #[must_use]
    pub fn routes(self: &Arc<Self>) -> Vec<Arc<dyn ProductionEndpointRoutes>> {
        let mut routes: Vec<Arc<dyn ProductionEndpointRoutes>> =
            vec![Arc::new(AdminCapabilityRoutes::new(
                "workspacePolicy.v1",
                &WORKSPACE_POLICY_METHODS,
                Arc::clone(self),
            ))];
        if self.provider_admin {
            routes.push(Arc::new(AdminCapabilityRoutes::new(
                "providerAdmin.v1",
                &PROVIDER_ADMIN_METHODS,
                Arc::clone(self),
            )));
        }
        routes
    }

    fn recover_pending_intents(&self) -> Result<(), ProductionRouteFailure> {
        for record in self.journal.records()? {
            // Application launch configuration wins over abandoned standalone edits.
            if !self.provider_admin && record.authority == "config/providers.json" {
                continue;
            }
            if record.phase == AdminPhase::Committed {
                continue;
            }
            if self.journal.authority_digest(&record.authority)?.as_deref()
                != Some(&record.desired_sha256)
            {
                match record.authority.as_str() {
                    "config/providers.json" => {
                        let desired = ProvidersConfig::decode(&record.desired)
                            .map_err(map_profile_provider)?;
                        self.repository
                            .publish_providers_checked(record.expected_revision, &desired)
                            .map_err(map_profile_provider)?;
                    }
                    "config/settings.json" => {
                        let desired = SettingsConfig::decode(&record.desired)
                            .map_err(map_profile_provider)?;
                        self.repository
                            .publish_settings_checked(record.expected_revision, &desired)
                            .map_err(map_profile_provider)?;
                    }
                    authority
                        if authority.starts_with("workspaces/") && authority.ends_with(".json") =>
                    {
                        let desired = profile::WorkspaceConfig::decode(&record.desired)
                            .map_err(map_policy)?;
                        let previous = self
                            .repository
                            .resolve(&desired.id)
                            .map_err(internal_profile)?;
                        let policy = desired.policy.clone().unwrap_or_default();
                        let published = self
                            .repository
                            .publish_workspace_policy(&desired.id, record.expected_revision, policy)
                            .map_err(map_policy)?;
                        if published.canonical_bytes().map_err(map_policy)? != record.desired {
                            return Err(internal(
                                "recovered workspace publication differs from intent",
                            ));
                        }
                        self.process_host
                            .workspace_policy_published(&desired.id, &previous)
                            .map_err(internal_daemon)?;
                    }
                    _ => {
                        return Err(internal(
                            "config mutation intent names an unknown authority",
                        ));
                    }
                }
            }
            match record.authority.as_str() {
                "config/providers.json" | "config/settings.json" => self
                    .process_host
                    .refresh_live_credentials()
                    .map_err(internal_daemon)?,
                authority if authority.starts_with("workspaces/") => {
                    let desired =
                        profile::WorkspaceConfig::decode(&record.desired).map_err(map_policy)?;
                    self.process_host.workspace_policy_recovered(&desired.id);
                }
                _ => {}
            }
            self.journal.commit_record(&record.rpc_id)?;
        }
        Ok(())
    }

    fn connections(&self) -> Result<IJsonValue, ProductionRouteFailure> {
        let config = self.repository.providers().map_err(internal_profile)?;
        let connections = config
            .providers
            .iter()
            .map(|provider| {
                let readiness = self.connection_readiness(provider);
                connection_view(provider, readiness)
            })
            .collect::<Vec<_>>();
        to_ijson(&json!({"format":1,"providersRevision":config.revision,"connections":connections}))
    }

    fn profiles(&self) -> Result<IJsonValue, ProductionRouteFailure> {
        let config = self.repository.providers().map_err(internal_profile)?;
        let settings = self.repository.settings().map_err(internal_profile)?;
        let mut profiles = Vec::new();
        for provider in &config.providers {
            for model in &provider.models {
                profiles.push(self.profile_view(provider, model));
            }
        }
        let mut result = json!({
            "format":1,
            "providersRevision":config.revision,
            "settingsRevision":settings.revision,
            "profiles":profiles,
        });
        if let (Some(connection_id), Some(exact_sku)) =
            (settings.default_provider, settings.default_model)
        {
            result
                .as_object_mut()
                .expect("provider profile result")
                .insert(
                    "default".to_owned(),
                    json!({"connectionId":connection_id,"exactSku":exact_sku}),
                );
        }
        to_ijson(&result)
    }

    fn connection_readiness(&self, provider: &Provider) -> Readiness {
        if provider::endpoint_origin(&provider.endpoint).is_err() {
            return Readiness::unavailable("invalid-endpoint");
        }
        let proof_verified = match provider::configured_route_is_verified(provider) {
            Ok(value) => value,
            Err(provider::DialectError::UnknownDialect(_))
            | Err(provider::DialectError::UnprovedProfile(_)) => {
                return Readiness::unavailable("dialect-unproved");
            }
            Err(_) => return Readiness::unavailable("route-mismatch"),
        };
        match self
            .process_host
            .credential_is_ready(provider.credential_key.as_deref())
        {
            Ok(true) if proof_verified => Readiness::Ready,
            Ok(true) => Readiness::unverified("no-exact-proof"),
            Ok(false) | Err(_) => Readiness::unavailable("credential-unavailable"),
        }
    }

    fn profile_readiness(&self, provider: &Provider, model: &Model) -> Readiness {
        if provider::endpoint_origin(&provider.endpoint).is_err() {
            return Readiness::unavailable("invalid-endpoint");
        }
        match provider::resolve_profile(provider, model) {
            Ok(profile) => match self
                .process_host
                .credential_is_ready(provider.credential_key.as_deref())
            {
                Ok(true) if profile.proof_verified => Readiness::Ready,
                Ok(true) => Readiness::unverified("no-exact-proof"),
                Ok(false) | Err(_) => Readiness::unavailable("credential-unavailable"),
            },
            Err(provider::DialectError::UnprovedProfile(_)) => {
                Readiness::unavailable("dialect-unproved")
            }
            Err(_) => Readiness::unavailable("route-mismatch"),
        }
    }

    fn profile_view(&self, provider: &Provider, model: &Model) -> Value {
        let provider_name = provider.name.as_deref().unwrap_or(&provider.id);
        let mut value = json!({
            "connectionId":provider.id,
            "provider":{"id":provider.id,"name":provider_name},
            "modelProfileId":model.profile,
            "exactSku":model.id,
            "enabled":model.enabled,
            "contextWindowTokens":model.context_window_tokens,
            "compactTriggerTokens":model.compact_trigger_tokens,
            "readiness":self.profile_readiness(provider, model),
        });
        if let Ok(profile) = provider::resolve_profile(provider, model) {
            if !profile.reasoning_efforts().is_empty() {
                value["reasoning"] = json!({
                    "efforts":profile.reasoning_efforts().iter().map(|effort| json!({
                        "id":effort,
                        "name":effort,
                    })).collect::<Vec<_>>(),
                    "defaultEffort":profile.default_reasoning_effort(),
                });
            }
        }
        value
    }

    fn provider_list(&self) -> Result<IJsonValue, ProductionRouteFailure> {
        let proofs = provider::advertised_dialect_proofs().map_err(|error| {
            failure(
                "dialect-unproved",
                "Provider proof registry is unavailable",
                json!({"reason":error.to_string()}),
            )
        })?;
        let proofs = proofs.into_iter().map(|proof| json!({
            "proofId":proof.proof_id,
            "target":target(&proof.protocol_family, &proof.dialect_id, &proof.model_profile_id,
                &proof.endpoint_owner, &proof.gateway_translation, &proof.exact_sku,
                &proof.evidence_revision),
        })).collect::<Vec<_>>();
        to_ijson(&json!({"format":1,"proofs":proofs}))
    }

    fn verify(&self, input: VerifyRequest) -> Result<IJsonValue, ProductionRouteFailure> {
        let config = self.repository.providers().map_err(internal_profile)?;
        let provider = find_provider(&config, &input.connection_id)?;
        let model = find_model(provider, &input.exact_sku)?;
        let resolved = provider::resolve_profile(provider, model).map_err(map_dialect)?;
        if !self
            .process_host
            .credential_is_ready(provider.credential_key.as_deref())
            .map_err(internal_daemon)?
        {
            return Err(failure(
                "credential-unavailable",
                "Provider credential is unavailable",
                json!({"connectionId":input.connection_id}),
            ));
        }
        let route = &resolved.target.route;
        to_ijson(&json!({"format":1,"verified":true,
            "proofVerified":resolved.proof_verified,"credentialReady":true,
            "target":target(&resolved.target.protocol_family, &resolved.target.dialect_id,
                &resolved.target.model_profile_id, &route.endpoint_owner,
                &route.gateway_translation, &route.exact_sku, &route.evidence_revision)}))
    }

    fn save_connection(
        &self,
        request: &EndpointHostCall,
        input: SaveConnectionRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let mut config = self.repository.providers().map_err(internal_profile)?;
        if config.revision != input.expected_providers_revision {
            return Err(stale(input.expected_providers_revision, config.revision));
        }
        let mut candidate = Provider::from(&input.connection);
        if let Some(existing) = config
            .providers
            .iter()
            .find(|provider| provider.id == candidate.id)
        {
            candidate.models = existing.models.clone();
        }
        // Administration is the durable configuration authority. Exact proof controls the
        // verified badge, not whether a known dialect configuration may execute best-effort.
        match config
            .providers
            .iter()
            .position(|provider| provider.id == candidate.id)
        {
            Some(index) => config.providers[index] = candidate.clone(),
            None => config.providers.push(candidate.clone()),
        }
        config.revision = next_revision(config.revision)?;
        let result = to_ijson(&json!({"format":1,"providersRevision":config.revision,
            "connection":connection_view(&candidate, self.connection_readiness(&candidate))}))?;
        let desired = config.canonical_bytes().map_err(map_profile_provider)?;
        if let AdminBegin::Completed(result) = self.journal.begin(
            request,
            "config/providers.json",
            input.expected_providers_revision,
            config.revision,
            &desired,
            &result,
        )? {
            mark_handoff(request)?;
            return Ok(result);
        }
        if let Err(error) = self
            .repository
            .publish_providers_checked(input.expected_providers_revision, &config)
        {
            self.journal.abort(request)?;
            return Err(map_profile_provider(error));
        }
        self.process_host
            .refresh_live_credentials()
            .map_err(internal_daemon)?;
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }

    fn delete_connection(
        &self,
        request: &EndpointHostCall,
        input: DeleteConnectionRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let mut config = self.repository.providers().map_err(internal_profile)?;
        if config.revision != input.expected_providers_revision {
            return Err(stale(input.expected_providers_revision, config.revision));
        }
        let index = config
            .providers
            .iter()
            .position(|provider| provider.id == input.connection_id)
            .ok_or_else(|| {
                failure(
                    "route-mismatch",
                    "Provider connection is absent",
                    json!({"connectionId":input.connection_id}),
                )
            })?;
        if !config.providers[index].models.is_empty() {
            return Err(failure(
                "provider-in-use",
                "Provider connection still owns profiles",
                json!({"connectionId":input.connection_id}),
            ));
        }
        config.providers.remove(index);
        config.revision = next_revision(config.revision)?;
        let result =
            to_ijson(&json!({"format":1,"providersRevision":config.revision,"deleted":true}))?;
        let desired = config.canonical_bytes().map_err(map_profile_provider)?;
        self.journal.begin(
            request,
            "config/providers.json",
            input.expected_providers_revision,
            config.revision,
            &desired,
            &result,
        )?;
        if let Err(error) = self
            .repository
            .publish_providers_checked(input.expected_providers_revision, &config)
        {
            self.journal.abort(request)?;
            return Err(map_provider_in_use(error));
        }
        self.process_host
            .refresh_live_credentials()
            .map_err(internal_daemon)?;
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }

    fn save_profile(
        &self,
        request: &EndpointHostCall,
        input: SaveProfileRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let mut config = self.repository.providers().map_err(internal_profile)?;
        if config.revision != input.expected_providers_revision {
            return Err(stale(input.expected_providers_revision, config.revision));
        }
        let provider = config
            .providers
            .iter_mut()
            .find(|provider| provider.id == input.profile.connection_id)
            .ok_or_else(|| {
                failure(
                    "route-mismatch",
                    "Provider connection is absent",
                    json!({"connectionId":input.profile.connection_id}),
                )
            })?;
        let model = Model::from(&input.profile);
        // An unproved model remains executable through its configured dialect and is surfaced as
        // unverified until exact route evidence is added.
        match provider
            .models
            .iter()
            .position(|value| value.id == model.id)
        {
            Some(index) => provider.models[index] = model.clone(),
            None => provider.models.push(model.clone()),
        }
        let provider_copy = provider.clone();
        config.revision = next_revision(config.revision)?;
        let result = to_ijson(&json!({"format":1,"providersRevision":config.revision,
            "profile":self.profile_view(&provider_copy, &model)}))?;
        let desired = config.canonical_bytes().map_err(map_profile_provider)?;
        self.journal.begin(
            request,
            "config/providers.json",
            input.expected_providers_revision,
            config.revision,
            &desired,
            &result,
        )?;
        if let Err(error) = self
            .repository
            .publish_providers_checked(input.expected_providers_revision, &config)
        {
            self.journal.abort(request)?;
            return Err(map_profile_save(error));
        }
        self.process_host
            .refresh_live_credentials()
            .map_err(internal_daemon)?;
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }

    fn delete_profile(
        &self,
        request: &EndpointHostCall,
        input: DeleteProfileRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let mut config = self.repository.providers().map_err(internal_profile)?;
        if config.revision != input.expected_providers_revision {
            return Err(stale(input.expected_providers_revision, config.revision));
        }
        let provider = config
            .providers
            .iter_mut()
            .find(|provider| provider.id == input.connection_id)
            .ok_or_else(|| {
                failure(
                    "route-mismatch",
                    "Provider connection is absent",
                    json!({"connectionId":input.connection_id}),
                )
            })?;
        let index = provider
            .models
            .iter()
            .position(|model| model.id == input.exact_sku)
            .ok_or_else(|| {
                failure(
                    "route-mismatch",
                    "Provider profile is absent",
                    json!({"exactSku":input.exact_sku}),
                )
            })?;
        provider.models.remove(index);
        config.revision = next_revision(config.revision)?;
        let result =
            to_ijson(&json!({"format":1,"providersRevision":config.revision,"deleted":true}))?;
        let desired = config.canonical_bytes().map_err(map_profile_provider)?;
        self.journal.begin(
            request,
            "config/providers.json",
            input.expected_providers_revision,
            config.revision,
            &desired,
            &result,
        )?;
        if let Err(error) = self
            .repository
            .publish_providers_checked(input.expected_providers_revision, &config)
        {
            self.journal.abort(request)?;
            return Err(map_profile_in_use(error));
        }
        self.process_host
            .refresh_live_credentials()
            .map_err(internal_daemon)?;
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }

    fn set_default(
        &self,
        request: &EndpointHostCall,
        input: DefaultProfileRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let providers = self.repository.providers().map_err(internal_profile)?;
        let provider = find_provider(&providers, &input.connection_id)?;
        let model = find_model(provider, &input.exact_sku)?;
        provider::resolve_profile(provider, model).map_err(map_dialect)?;
        let mut settings = self.repository.settings().map_err(internal_profile)?;
        if settings.revision != input.expected_settings_revision {
            return Err(stale(input.expected_settings_revision, settings.revision));
        }
        settings.revision = next_revision(settings.revision)?;
        settings.default_provider = Some(input.connection_id.clone());
        settings.default_model = Some(input.exact_sku.clone());
        let result = default_result(settings.revision, &input)?;
        let desired = settings.canonical_bytes().map_err(map_profile_provider)?;
        self.journal.begin(
            request,
            "config/settings.json",
            input.expected_settings_revision,
            settings.revision,
            &desired,
            &result,
        )?;
        if let Err(error) = self
            .repository
            .publish_settings_checked(input.expected_settings_revision, &settings)
        {
            self.journal.abort(request)?;
            return Err(map_profile_route(error));
        }
        self.process_host
            .refresh_live_credentials()
            .map_err(internal_daemon)?;
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }

    fn get_policy(&self, input: PolicyGetRequest) -> Result<IJsonValue, ProductionRouteFailure> {
        let workspace = self
            .repository
            .workspace(&input.workspace_id)
            .map_err(map_workspace)?;
        to_ijson(&json!({"format":1,"revision":workspace.revision,
            "policy":workspace.policy.unwrap_or_default()}))
    }

    fn set_policy(
        &self,
        request: &EndpointHostCall,
        input: PolicySetRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let workspace = self
            .repository
            .workspace(&input.workspace_id)
            .map_err(map_workspace)?;
        if workspace.revision != input.expected_revision {
            return Err(stale(input.expected_revision, workspace.revision));
        }
        self.process_host
            .validate_workspace_policy_candidate(&input.workspace_id, &input.policy)
            .map_err(|error| {
                failure(
                    "policy-escalation",
                    "Workspace policy exceeds an authority ceiling",
                    json!({"reason":error.to_string()}),
                )
            })?;
        let previous = self
            .repository
            .resolve(&input.workspace_id)
            .map_err(internal_profile)?;
        let mut desired_workspace = workspace.clone();
        desired_workspace.revision = next_revision(desired_workspace.revision)?;
        desired_workspace.policy = Some(input.policy.clone());
        let desired = desired_workspace.canonical_bytes().map_err(map_policy)?;
        let result = policy_result(desired_workspace.revision, &input.policy)?;
        self.journal.begin(
            request,
            &format!("workspaces/{}/workspace.json", input.workspace_id),
            input.expected_revision,
            desired_workspace.revision,
            &desired,
            &result,
        )?;
        let published = match self.repository.publish_workspace_policy(
            &input.workspace_id,
            input.expected_revision,
            input.policy.clone(),
        ) {
            Ok(value) => value,
            Err(error) => {
                self.journal.abort(request)?;
                return Err(map_policy(error));
            }
        };
        self.process_host
            .workspace_policy_published(&input.workspace_id, &previous)
            .map_err(internal_daemon)?;
        if published.revision != desired_workspace.revision {
            return Err(internal(
                "workspace policy publication revision disagrees with intent",
            ));
        }
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }
}

struct AdminCapabilityRoutes {
    id: &'static str,
    methods: &'static [(&'static str, MethodClass)],
    authority: Arc<ClientAdminRoutes>,
}

impl AdminCapabilityRoutes {
    fn new(
        id: &'static str,
        methods: &'static [(&'static str, MethodClass)],
        authority: Arc<ClientAdminRoutes>,
    ) -> Self {
        Self {
            id,
            methods,
            authority,
        }
    }

    fn class(&self, method: &str) -> Option<MethodClass> {
        self.methods
            .iter()
            .find_map(|(name, class)| (*name == method).then_some(*class))
    }
}

impl ProductionEndpointRoutes for AdminCapabilityRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        self.methods
            .iter()
            .map(|(name, _)| (*name).to_owned())
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        self.class(method)
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        if self.class(operation).is_none() {
            return Err(failure(
                "unsupported-capability",
                "Client administration capability is unavailable",
                json!({"capability":self.id,"operation":operation}),
            ));
        }
        self.authority
            .validate_extension_payload(operation, payload)
    }

    fn extension_failure_is_exact(
        &self,
        operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        self.class(operation).is_some()
            && self
                .authority
                .extension_failure_is_exact(operation, failure)
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        if self.class(&request.operation).is_none() {
            return Err(failure(
                "unsupported-capability",
                "Client administration capability is unavailable",
                json!({"capability":self.id,"operation":request.operation}),
            ));
        }
        self.authority.execute(request, payload, principal)
    }
}

impl ProductionEndpointRoutes for ClientAdminRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        PROVIDER_ADMIN_METHODS
            .into_iter()
            .chain(WORKSPACE_POLICY_METHODS)
            .map(|(name, _)| name.to_owned())
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        Some(match method {
            "providers.list"
            | "providers.verify"
            | "providers.connections"
            | "providers.profiles"
            | "workspace.policy.get" => MethodClass::ReadOnly,
            "providers.connection.save"
            | "providers.connection.delete"
            | "providers.profile.save"
            | "providers.profile.delete"
            | "providers.profile.default"
            | "workspace.policy.set" => MethodClass::Mutation,
            _ => return None,
        })
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        match operation {
            "providers.list" | "providers.connections" | "providers.profiles" => {
                parse::<Empty>(payload).map(|_| ())
            }
            "providers.verify" => parse::<VerifyRequest>(payload).map(|_| ()),
            "providers.connection.save" => parse::<SaveConnectionRequest>(payload).map(|_| ()),
            "providers.connection.delete" => parse::<DeleteConnectionRequest>(payload).map(|_| ()),
            "providers.profile.save" => parse::<SaveProfileRequest>(payload).map(|_| ()),
            "providers.profile.delete" => parse::<DeleteProfileRequest>(payload).map(|_| ()),
            "providers.profile.default" => parse::<DefaultProfileRequest>(payload).map(|_| ()),
            "workspace.policy.get" => parse::<PolicyGetRequest>(payload).map(|_| ()),
            "workspace.policy.set" => parse::<PolicySetRequest>(payload).map(|_| ()),
            _ => Err(failure(
                "unsupported-capability",
                "Client administration method is unavailable",
                json!({"operation":operation}),
            )),
        }
    }

    fn extension_failure_is_exact(
        &self,
        _operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        matches!(
            failure.code.as_str(),
            "bad-request"
                | "unsupported-capability"
                | "internal"
                | "idempotency-conflict"
                | "stale-revision"
                | "dialect-unproved"
                | "route-mismatch"
                | "credential-unavailable"
                | "provider-in-use"
                | "profile-in-use"
                | "workspace-not-found"
                | "policy-invalid"
                | "policy-escalation"
        )
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        match request.operation.as_str() {
            "providers.list" => self.provider_list(),
            "providers.verify" => self.verify(parse(payload)?),
            "providers.connections" => self.connections(),
            "providers.connection.save" => self.save_connection(request, parse(payload)?),
            "providers.connection.delete" => self.delete_connection(request, parse(payload)?),
            "providers.profiles" => self.profiles(),
            "providers.profile.save" => self.save_profile(request, parse(payload)?),
            "providers.profile.delete" => self.delete_profile(request, parse(payload)?),
            "providers.profile.default" => self.set_default(request, parse(payload)?),
            "workspace.policy.get" => self.get_policy(parse(payload)?),
            "workspace.policy.set" => self.set_policy(request, parse(payload)?),
            _ => Err(failure(
                "unsupported-capability",
                "Client administration method is unavailable",
                json!({"operation":request.operation}),
            )),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Empty {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct VerifyRequest {
    connection_id: String,
    exact_sku: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Connection {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    protocol_family: String,
    dialect_id: String,
    endpoint_owner: String,
    gateway_translation: String,
    evidence_revision: String,
    endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_id: Option<String>,
}

impl From<&Provider> for Connection {
    fn from(value: &Provider) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            protocol_family: value.adapter.clone(),
            dialect_id: value.dialect.clone(),
            endpoint_owner: value.endpoint_owner.clone(),
            gateway_translation: value.gateway_translation.clone(),
            evidence_revision: value.evidence_revision.clone(),
            endpoint: value.endpoint.clone(),
            credential_id: value.credential_key.clone(),
        }
    }
}
impl From<&Connection> for Provider {
    fn from(value: &Connection) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            adapter: value.protocol_family.clone(),
            dialect: value.dialect_id.clone(),
            endpoint_owner: value.endpoint_owner.clone(),
            gateway_translation: value.gateway_translation.clone(),
            evidence_revision: value.evidence_revision.clone(),
            endpoint: value.endpoint.clone(),
            credential_key: value.credential_id.clone(),
            models: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SaveConnectionRequest {
    expected_providers_revision: u64,
    connection: Connection,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DeleteConnectionRequest {
    expected_providers_revision: u64,
    connection_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ModelProfile {
    connection_id: String,
    model_profile_id: String,
    exact_sku: String,
    enabled: bool,
    context_window_tokens: u64,
    compact_trigger_tokens: u64,
}
impl From<(&String, &Model)> for ModelProfile {
    fn from((connection, value): (&String, &Model)) -> Self {
        Self {
            connection_id: connection.clone(),
            model_profile_id: value.profile.clone(),
            exact_sku: value.id.clone(),
            enabled: value.enabled,
            context_window_tokens: value.context_window_tokens,
            compact_trigger_tokens: value.compact_trigger_tokens,
        }
    }
}
impl From<&ModelProfile> for Model {
    fn from(value: &ModelProfile) -> Self {
        Self {
            id: value.exact_sku.clone(),
            profile: value.model_profile_id.clone(),
            enabled: value.enabled,
            context_window_tokens: value.context_window_tokens,
            compact_trigger_tokens: value.compact_trigger_tokens,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SaveProfileRequest {
    expected_providers_revision: u64,
    profile: ModelProfile,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DeleteProfileRequest {
    expected_providers_revision: u64,
    connection_id: String,
    exact_sku: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DefaultProfileRequest {
    expected_settings_revision: u64,
    connection_id: String,
    exact_sku: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PolicyGetRequest {
    workspace_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PolicySetRequest {
    workspace_id: String,
    expected_revision: u64,
    policy: WorkspacePolicy,
}

#[derive(Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
enum Readiness {
    Ready,
    Unverified { reason: &'static str },
    Unavailable { reason: &'static str },
}
impl Readiness {
    fn unverified(reason: &'static str) -> Self {
        Self::Unverified { reason }
    }
    fn unavailable(reason: &'static str) -> Self {
        Self::Unavailable { reason }
    }
}

fn parse<T: for<'de> Deserialize<'de>>(payload: &Value) -> Result<T, ProductionRouteFailure> {
    serde_json::from_value(payload.clone()).map_err(|error| {
        failure(
            "bad-request",
            "Invalid client administration request",
            json!({"reason":error.to_string()}),
        )
    })
}
fn to_ijson(value: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
    IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(value).map_err(|error| internal(error.to_string()))?,
    )
    .map_err(|error| internal(error.to_string()))
}
fn failure(code: &str, message: &str, details: Value) -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        code,
        message,
        to_ijson(&details).unwrap_or_else(|_| IJsonValue::parse_str("{}").expect("I-JSON")),
    )
}
fn internal(message: impl ToString) -> ProductionRouteFailure {
    failure(
        "internal",
        "Client administration failed",
        json!({"reason":message.to_string()}),
    )
}
fn internal_profile(error: profile::ProfileError) -> ProductionRouteFailure {
    internal(error)
}
fn internal_daemon(error: crate::host_runtime::DaemonError) -> ProductionRouteFailure {
    internal(error)
}
fn stale(expected: u64, actual: u64) -> ProductionRouteFailure {
    failure(
        "stale-revision",
        "Configuration revision is stale",
        json!({"expected":expected,"actual":actual}),
    )
}
fn next_revision(value: u64) -> Result<u64, ProductionRouteFailure> {
    value
        .checked_add(1)
        .filter(|value| *value <= 9_007_199_254_740_991)
        .ok_or_else(|| {
            failure(
                "bad-request",
                "Configuration revision is exhausted",
                json!({}),
            )
        })
}
fn find_provider<'a>(
    config: &'a ProvidersConfig,
    id: &str,
) -> Result<&'a Provider, ProductionRouteFailure> {
    config
        .providers
        .iter()
        .find(|provider| provider.id == id)
        .ok_or_else(|| {
            failure(
                "route-mismatch",
                "Provider connection is absent",
                json!({"connectionId":id}),
            )
        })
}
fn find_model<'a>(provider: &'a Provider, id: &str) -> Result<&'a Model, ProductionRouteFailure> {
    provider
        .models
        .iter()
        .find(|model| model.id == id)
        .ok_or_else(|| {
            failure(
                "route-mismatch",
                "Provider profile is absent",
                json!({"exactSku":id}),
            )
        })
}
fn map_dialect(error: provider::DialectError) -> ProductionRouteFailure {
    match error {
        provider::DialectError::UnprovedProfile(_) => failure(
            "dialect-unproved",
            "Provider dialect proof is unavailable",
            json!({"reason":error.to_string()}),
        ),
        _ => failure(
            "route-mismatch",
            "Provider route does not match its proof",
            json!({"reason":error.to_string()}),
        ),
    }
}
fn map_profile_provider(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        profile::ProfileError::InvalidReference { .. } => failure(
            "provider-in-use",
            "Provider configuration is referenced",
            json!({"reason":error.to_string()}),
        ),
        _ => failure(
            "route-mismatch",
            "Provider configuration is invalid",
            json!({"reason":error.to_string()}),
        ),
    }
}
fn map_profile_in_use(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        profile::ProfileError::InvalidReference { .. } => failure(
            "profile-in-use",
            "Provider profile is referenced",
            json!({"reason":error.to_string()}),
        ),
        _ => failure(
            "route-mismatch",
            "Provider configuration is invalid",
            json!({"reason":error.to_string()}),
        ),
    }
}
fn map_provider_in_use(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        profile::ProfileError::InvalidReference { .. } => failure(
            "provider-in-use",
            "Provider connection is referenced",
            json!({"reason":error.to_string()}),
        ),
        _ => failure(
            "route-mismatch",
            "Provider configuration is invalid",
            json!({"reason":error.to_string()}),
        ),
    }
}
fn map_profile_save(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        profile::ProfileError::InvalidReference { .. } => failure(
            "profile-in-use",
            "Provider profile is referenced",
            json!({"reason":error.to_string()}),
        ),
        _ => failure(
            "route-mismatch",
            "Provider configuration is invalid",
            json!({"reason":error.to_string()}),
        ),
    }
}
fn map_profile_route(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        _ => failure(
            "route-mismatch",
            "Provider default is invalid",
            json!({"reason":error.to_string()}),
        ),
    }
}
fn map_workspace(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::Io(ref io) if io.kind() == std::io::ErrorKind::NotFound => {
            failure("workspace-not-found", "Workspace is absent", json!({}))
        }
        _ => internal_profile(error),
    }
}
fn map_policy(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        profile::ProfileError::Io(ref io) if io.kind() == std::io::ErrorKind::NotFound => {
            failure("workspace-not-found", "Workspace is absent", json!({}))
        }
        profile::ProfileError::InvalidReference { .. } => failure(
            "policy-escalation",
            "Workspace policy exceeds an authority ceiling",
            json!({"reason":error.to_string()}),
        ),
        _ => failure(
            "policy-invalid",
            "Workspace policy is invalid",
            json!({"reason":error.to_string()}),
        ),
    }
}
fn target(
    family: &str,
    dialect: &str,
    profile: &str,
    owner: &str,
    gateway: &str,
    sku: &str,
    revision: &str,
) -> Value {
    json!({"protocolFamily":family,"dialectId":dialect,"modelProfileId":profile,"endpointOwner":owner,"gatewayTranslation":gateway,"exactSku":sku,"evidenceRevision":revision})
}
fn connection_view(provider: &Provider, readiness: Readiness) -> Value {
    let c = Connection::from(provider);
    let mut value = json!({"id":c.id,"protocolFamily":c.protocol_family,"dialectId":c.dialect_id,"endpointOwner":c.endpoint_owner,"gatewayTranslation":c.gateway_translation,"evidenceRevision":c.evidence_revision,"endpoint":c.endpoint,"readiness":readiness});
    if let Some(name) = c.name {
        value
            .as_object_mut()
            .expect("connection object")
            .insert("name".to_owned(), Value::String(name));
    }
    if let Some(id) = c.credential_id {
        value
            .as_object_mut()
            .expect("connection object")
            .insert("credentialId".to_owned(), Value::String(id));
    }
    value
}
fn default_result(
    revision: u64,
    input: &DefaultProfileRequest,
) -> Result<IJsonValue, ProductionRouteFailure> {
    to_ijson(
        &json!({"format":1,"settingsRevision":revision,"default":{"connectionId":input.connection_id,"exactSku":input.exact_sku}}),
    )
}
fn policy_result(
    revision: u64,
    policy: &WorkspacePolicy,
) -> Result<IJsonValue, ProductionRouteFailure> {
    to_ijson(&json!({"format":1,"revision":revision,"policy":policy}))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AdminPhase {
    Prepared,
    Committed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdminRecord {
    format: u64,
    rpc_id: String,
    operation: String,
    request_sha256: String,
    authority: String,
    expected_revision: u64,
    next_revision: u64,
    desired_sha256: String,
    desired: Vec<u8>,
    phase: AdminPhase,
    result: IJsonValue,
}

enum AdminBegin {
    Execute,
    Completed(IJsonValue),
}

/// The durable config-admin rpc carrier directory under the authority root.
const CONFIG_ADMIN_DIR: &str = "config-admin";
/// The name it carried before the rename; retired in place on first open.
const LEGACY_CONFIG_ADMIN_DIR: &str = "config-admin-v1";

struct AdminMutationJournal {
    authority_root: PathBuf,
    root: PathBuf,
    lock: PathBuf,
}

impl AdminMutationJournal {
    fn open(authority_root: &Path) -> Result<Self, store::StoreError> {
        let admin_root =
            store::retire_legacy_name(authority_root, CONFIG_ADMIN_DIR, LEGACY_CONFIG_ADMIN_DIR)?;
        let root = admin_root.join("rpc");
        fs::create_dir_all(&root)?;
        let lock = admin_root.join("lock");
        if !lock.exists() {
            AtomicPublisher::replace(&lock, b"config-admin\n")?;
        }
        Ok(Self {
            authority_root: authority_root.to_path_buf(),
            root,
            lock,
        })
    }

    fn begin(
        &self,
        request: &EndpointHostCall,
        authority: &str,
        expected_revision: u64,
        next_revision: u64,
        desired_bytes: &[u8],
        result: &IJsonValue,
    ) -> Result<AdminBegin, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let request_sha256 = request_digest(request)?;
        let path = self.record_path(&request.rpc_id);
        if let Some(mut old) = read_record(&path)? {
            if old.rpc_id != request.rpc_id
                || old.operation != request.operation
                || old.request_sha256 != request_sha256
            {
                return Err(failure(
                    "idempotency-conflict",
                    "rpcId was already used for another config mutation",
                    json!({"rpcId":request.rpc_id}),
                ));
            }
            if old.authority != authority
                || old.expected_revision != expected_revision
                || old.next_revision != next_revision
                || old.desired_sha256 != sha256(desired_bytes)
                || old.desired != desired_bytes
                || old.result != *result
            {
                return Err(failure(
                    "idempotency-conflict",
                    "rpcId config intent disagrees with its durable record",
                    json!({"rpcId":request.rpc_id}),
                ));
            }
            if old.phase == AdminPhase::Prepared
                && self.authority_digest(&old.authority)?.as_deref() == Some(&old.desired_sha256)
            {
                old.phase = AdminPhase::Committed;
                publish_record(&path, &old)?;
            }
            return Ok(if old.phase == AdminPhase::Committed {
                AdminBegin::Completed(old.result)
            } else {
                AdminBegin::Execute
            });
        }
        let record = AdminRecord {
            format: 1,
            rpc_id: request.rpc_id.clone(),
            operation: request.operation.clone(),
            request_sha256,
            authority: authority.to_owned(),
            expected_revision,
            next_revision,
            desired_sha256: sha256(desired_bytes),
            desired: desired_bytes.to_vec(),
            phase: AdminPhase::Prepared,
            result: result.clone(),
        };
        publish_record(&path, &record)?;
        Ok(AdminBegin::Execute)
    }

    fn recover(
        &self,
        request: &EndpointHostCall,
    ) -> Result<Option<IJsonValue>, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(&request.rpc_id);
        let Some(mut record) = read_record(&path)? else {
            return Ok(None);
        };
        if record.rpc_id != request.rpc_id
            || record.operation != request.operation
            || record.request_sha256 != request_digest(request)?
        {
            return Err(failure(
                "idempotency-conflict",
                "rpcId was already used for another config mutation",
                json!({"rpcId":request.rpc_id}),
            ));
        }
        if record.phase == AdminPhase::Prepared
            && self.authority_digest(&record.authority)?.as_deref() == Some(&record.desired_sha256)
        {
            record.phase = AdminPhase::Committed;
            publish_record(&path, &record)?;
        }
        Ok((record.phase == AdminPhase::Committed).then_some(record.result))
    }

    fn commit(&self, request: &EndpointHostCall) -> Result<IJsonValue, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(&request.rpc_id);
        let mut record =
            read_record(&path)?.ok_or_else(|| internal("config mutation intent is missing"))?;
        if record.request_sha256 != request_digest(request)?
            || record.operation != request.operation
        {
            return Err(failure(
                "idempotency-conflict",
                "rpcId config intent disagrees with request",
                json!({"rpcId":request.rpc_id}),
            ));
        }
        if self.authority_digest(&record.authority)?.as_deref() != Some(&record.desired_sha256) {
            return Err(internal(
                "published config bytes disagree with durable intent",
            ));
        }
        record.phase = AdminPhase::Committed;
        publish_record(&path, &record)?;
        Ok(record.result)
    }

    fn records(&self) -> Result<Vec<AdminRecord>, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let mut entries = fs::read_dir(&self.root)
            .map_err(internal_io)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(internal_io)?;
        entries.sort_by_key(fs::DirEntry::file_name);
        entries
            .into_iter()
            .map(|entry| {
                read_record(&entry.path())?.ok_or_else(|| internal("config journal entry vanished"))
            })
            .collect()
    }

    fn commit_record(&self, rpc_id: &str) -> Result<(), ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(rpc_id);
        let mut record =
            read_record(&path)?.ok_or_else(|| internal("config mutation intent is missing"))?;
        if self.authority_digest(&record.authority)?.as_deref() != Some(&record.desired_sha256) {
            return Err(internal(
                "published config bytes disagree with durable intent",
            ));
        }
        record.phase = AdminPhase::Committed;
        publish_record(&path, &record)
    }

    fn abort(&self, request: &EndpointHostCall) -> Result<(), ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(&request.rpc_id);
        let Some(record) = read_record(&path)? else {
            return Ok(());
        };
        if record.phase == AdminPhase::Committed
            || self.authority_digest(&record.authority)?.as_deref() == Some(&record.desired_sha256)
        {
            return Err(internal("cannot abort a published config mutation"));
        }
        fs::remove_file(&path).map_err(internal_io)?;
        fs::File::open(&self.root)
            .and_then(|directory| directory.sync_all())
            .map_err(internal_io)
    }

    fn authority_digest(&self, authority: &str) -> Result<Option<String>, ProductionRouteFailure> {
        if authority.starts_with('/')
            || authority
                .split('/')
                .any(|part| matches!(part, "" | "." | ".."))
        {
            return Err(internal("config mutation authority path is invalid"));
        }
        let path = self.authority_root.join(authority);
        match fs::read(path) {
            Ok(bytes) => Ok(Some(sha256(&bytes))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(internal_io(error)),
        }
    }

    fn record_path(&self, rpc_id: &str) -> PathBuf {
        self.root
            .join(format!("{}.json", sha256(rpc_id.as_bytes())))
    }
}

fn request_digest(request: &EndpointHostCall) -> Result<String, ProductionRouteFailure> {
    let bytes = serde_json_canonicalizer::to_vec(
        &json!({"method":request.operation,"payload":request.payload}),
    )
    .map_err(|error| internal(error.to_string()))?;
    Ok(sha256(&bytes))
}
fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read_record(path: &Path) -> Result<Option<AdminRecord>, ProductionRouteFailure> {
    match fs::read(path) {
        Ok(bytes) => {
            if !bytes.ends_with(b"\n") || bytes[..bytes.len().saturating_sub(1)].contains(&b'\n') {
                return Err(internal("config mutation record framing is invalid"));
            }
            let value: AdminRecord =
                serde_json::from_slice(&bytes).map_err(|error| internal(error.to_string()))?;
            let mut canonical = serde_json_canonicalizer::to_vec(&value)
                .map_err(|error| internal(error.to_string()))?;
            canonical.push(b'\n');
            if canonical != bytes {
                return Err(internal("config mutation record is noncanonical"));
            }
            Ok(Some(value))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(internal_io(error)),
    }
}
fn publish_record(path: &Path, record: &AdminRecord) -> Result<(), ProductionRouteFailure> {
    let mut bytes =
        serde_json_canonicalizer::to_vec(record).map_err(|error| internal(error.to_string()))?;
    bytes.push(b'\n');
    AtomicPublisher::replace(path, &bytes).map_err(internal_store)
}
fn internal_store(error: store::StoreError) -> ProductionRouteFailure {
    internal(error)
}
fn internal_io(error: std::io::Error) -> ProductionRouteFailure {
    internal(error)
}
fn mark_handoff(request: &EndpointHostCall) -> Result<(), ProductionRouteFailure> {
    request
        .handoff
        .mark_handed_off(DurableHandoffProof {
            delivery: request.rpc_id.clone(),
            durable_identity: Some(RpcDurableIdentity {
                kind: CONFIG_ADMIN_DIR.to_owned(),
                id: request.rpc_id.clone(),
                seq: None,
            }),
        })
        .map_err(|error| internal(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use endpoint::DurableHandoffSignal;
    use profile::WorkspaceConfig;
    use tempfile::TempDir;

    fn fixture() -> (TempDir, Arc<ProductionProcessHost>, ClientAdminRoutes) {
        let root = TempDir::new().expect("temp root");
        let agent = root.path().join("agent");
        fs::create_dir_all(&agent).expect("agent root");
        let host = ProductionProcessHost::open(root.path(), "/usr/bin/true", "test-build", &agent)
            .expect("process host");
        let repository = ConfigRepository::open(root.path()).expect("config repository");
        repository
            .publish_workspace(
                0,
                &WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "workspace-1".to_owned(),
                    name: "Workspace".to_owned(),
                    cwd: vec![root.path().to_string_lossy().into_owned()],
                    folders: Vec::new(),
                    policy: Some(WorkspacePolicy::default()),
                },
            )
            .expect("workspace");
        let routes = ClientAdminRoutes::new(root.path(), Arc::clone(&host)).expect("routes");
        (root, host, routes)
    }

    fn call(rpc_id: &str, operation: &str, payload: Value, recovering: bool) -> EndpointHostCall {
        EndpointHostCall {
            rpc_id: rpc_id.to_owned(),
            operation: operation.to_owned(),
            payload: to_ijson(&payload).expect("payload"),
            recovering,
            handoff: DurableHandoffSignal::new(),
        }
    }

    fn openai_connection() -> Value {
        json!({
            "id":"openai-main",
            "name":"OpenAI",
            "protocolFamily":"responses",
            "dialectId":"openai_responses_v1",
            "endpointOwner":"openai",
            "gatewayTranslation":"direct",
            "evidenceRevision":"openai-2026-08-01",
            "endpoint":"https://api.openai.com/v1"
        })
    }

    #[test]
    fn provider_profile_projection_uses_exact_model_reasoning_capabilities() {
        let (_root, _host, routes) = fixture();
        let provider = Provider {
            id: "deepseek-main".to_owned(),
            name: Some("DeepSeek".to_owned()),
            adapter: "responses".to_owned(),
            dialect: "deepseek_responses_v1".to_owned(),
            endpoint_owner: "deepseek".to_owned(),
            gateway_translation: "direct".to_owned(),
            evidence_revision:
                "deepseek-direct-responses-v4-2026-07-31+function-json-schema-strict-v1".to_owned(),
            endpoint: "https://api.deepseek.com".to_owned(),
            credential_key: None,
            models: vec![Model {
                id: "deepseek-v4-flash".to_owned(),
                profile: "deepseek_responses_v1:deepseek-v4-flash".to_owned(),
                enabled: true,
                context_window_tokens: 128_000,
                compact_trigger_tokens: 96_000,
            }],
        };

        let value = routes.profile_view(&provider, &provider.models[0]);

        assert_eq!(
            value["reasoning"]["efforts"],
            json!([
                {"id":"low","name":"low"},
                {"id":"high","name":"high"},
                {"id":"max","name":"max"},
            ])
        );
        assert_eq!(value["reasoning"]["defaultEffort"], "high");
    }

    #[test]
    fn provider_admin_publishes_exact_identity_and_reacks_from_journal() {
        let (_root, _host, routes) = fixture();
        let save = call(
            "rpc-connection",
            "providers.connection.save",
            json!({"expectedProvidersRevision":0,"connection":openai_connection()}),
            false,
        );
        let first = routes
            .execute(
                &save,
                &serde_json::to_value(&save.payload).expect("payload value"),
                "uid:1",
            )
            .expect("save");
        assert!(save.handoff.is_durable());
        let retry = call(
            "rpc-connection",
            "providers.connection.save",
            json!({"expectedProvidersRevision":0,"connection":openai_connection()}),
            true,
        );
        let repeated = routes
            .execute(
                &retry,
                &serde_json::to_value(&retry.payload).expect("payload value"),
                "uid:1",
            )
            .expect("re-ack");
        assert_eq!(first, repeated);
        assert!(retry.handoff.is_durable());

        let listed = routes.connections().expect("connections");
        let value = serde_json::to_value(listed).expect("list value");
        assert_eq!(value["connections"][0]["readiness"]["state"], "ready");
        assert_eq!(value["connections"][0]["name"], "OpenAI");
        assert_eq!(value["connections"][0]["protocolFamily"], "responses");
    }

    #[test]
    fn exact_proof_verify_and_reference_protection_share_one_config_authority() {
        let (_root, _host, routes) = fixture();
        let save_connection = call(
            "rpc-proof-connection",
            "providers.connection.save",
            json!({"expectedProvidersRevision":0,"connection":openai_connection()}),
            false,
        );
        routes
            .execute(
                &save_connection,
                &serde_json::to_value(&save_connection.payload).expect("payload"),
                "uid:1",
            )
            .expect("connection");

        let save_profile = call(
            "rpc-proof-profile",
            "providers.profile.save",
            json!({"expectedProvidersRevision":1,"profile":{
                "connectionId":"openai-main",
                "modelProfileId":"openai_responses_v1:gpt-5",
                "exactSku":"gpt-5",
                "enabled":true,
                "contextWindowTokens":200000,
                "compactTriggerTokens":180000
            }}),
            false,
        );
        routes
            .execute(
                &save_profile,
                &serde_json::to_value(&save_profile.payload).expect("payload"),
                "uid:1",
            )
            .expect("profile");

        let profiles =
            serde_json::to_value(routes.profiles().expect("profiles")).expect("profile result");
        assert_eq!(profiles["profiles"][0]["provider"]["name"], "OpenAI");

        let proofs =
            serde_json::to_value(routes.provider_list().expect("proofs")).expect("proof result");
        assert_eq!(proofs["proofs"][0]["proofId"], "proof-openai_responses_v1");
        let verified = serde_json::to_value(
            routes
                .verify(VerifyRequest {
                    connection_id: "openai-main".to_owned(),
                    exact_sku: "gpt-5".to_owned(),
                })
                .expect("verify"),
        )
        .expect("verify result");
        assert_eq!(verified["verified"], true);
        assert_eq!(verified["target"]["exactSku"], "gpt-5");

        let select_default = call(
            "rpc-proof-default",
            "providers.profile.default",
            json!({"expectedSettingsRevision":0,"connectionId":"openai-main","exactSku":"gpt-5"}),
            false,
        );
        routes
            .execute(
                &select_default,
                &serde_json::to_value(&select_default.payload).expect("payload"),
                "uid:1",
            )
            .expect("default");
        let delete = call(
            "rpc-proof-delete",
            "providers.profile.delete",
            json!({"expectedProvidersRevision":2,"connectionId":"openai-main","exactSku":"gpt-5"}),
            false,
        );
        let error = routes
            .execute(
                &delete,
                &serde_json::to_value(&delete.payload).expect("payload"),
                "uid:1",
            )
            .expect_err("referenced profile must not delete");
        assert_eq!(error.code, "profile-in-use");
        assert_eq!(
            routes.repository.providers().expect("providers").revision,
            2
        );
    }

    #[test]
    fn unproved_configured_route_is_usable_and_distinct_from_verified() {
        let (_root, _host, routes) = fixture();
        let connection = json!({
            "id":"example-cloudflare-openai-responses-v1",
            "protocolFamily":"responses",
            "dialectId":"openai_responses_v1",
            "endpointOwner":"cloudflare",
            "gatewayTranslation":"router",
            "evidenceRevision":"legacy-example-v2",
            "endpoint":"https://api.cloudflare.com/client/v4/accounts/example/ai/v1",
            "credentialId":null
        });
        let save_connection = call(
            "rpc-legacy-connection",
            "providers.connection.save",
            json!({"expectedProvidersRevision":0,"connection":connection}),
            false,
        );
        routes
            .execute(
                &save_connection,
                &serde_json::to_value(&save_connection.payload).expect("payload"),
                "uid:1",
            )
            .expect("legacy connection is configurable");
        let save_profile = call(
            "rpc-legacy-profile",
            "providers.profile.save",
            json!({"expectedProvidersRevision":1,"profile":{
                "connectionId":"example-cloudflare-openai-responses-v1",
                "modelProfileId":"openai_responses_v1:openai/gpt-5.6-luna",
                "exactSku":"openai/gpt-5.6-luna",
                "enabled":true,
                "contextWindowTokens":200000,
                "compactTriggerTokens":180000
            }}),
            false,
        );
        routes
            .execute(
                &save_profile,
                &serde_json::to_value(&save_profile.payload).expect("payload"),
                "uid:1",
            )
            .expect("legacy profile is configurable");

        let connections = serde_json::to_value(routes.connections().expect("connections"))
            .expect("connections value");
        assert_eq!(
            connections["connections"][0]["readiness"]["state"],
            "unverified"
        );
        assert_eq!(
            connections["connections"][0]["readiness"]["reason"],
            "no-exact-proof"
        );
        let profiles =
            serde_json::to_value(routes.profiles().expect("profiles")).expect("profiles value");
        assert_eq!(profiles["profiles"][0]["readiness"]["state"], "unverified");
        let configured = serde_json::to_value(
            routes
                .verify(VerifyRequest {
                    connection_id: "example-cloudflare-openai-responses-v1".to_owned(),
                    exact_sku: "openai/gpt-5.6-luna".to_owned(),
                })
                .expect("configured route validation"),
        );
        let configured = configured.expect("configured route value");
        assert_eq!(configured["verified"], true);
        assert_eq!(configured["proofVerified"], false);

        let select_default = call(
            "rpc-unverified-default",
            "providers.profile.default",
            json!({"expectedSettingsRevision":0,
                "connectionId":"example-cloudflare-openai-responses-v1",
                "exactSku":"openai/gpt-5.6-luna"}),
            false,
        );
        routes
            .execute(
                &select_default,
                &serde_json::to_value(&select_default.payload).expect("payload"),
                "uid:1",
            )
            .expect("unverified configured route can become default");
    }

    #[test]
    fn policy_replacement_can_relax_local_default_below_ceiling() {
        let (_root, _host, routes) = fixture();
        let request = call(
            "rpc-policy",
            "workspace.policy.set",
            json!({"workspaceId":"workspace-1","expectedRevision":1,"policy":{
                "network":false,"allowed_tools":["read"],"writable_roots":[]
            }}),
            false,
        );
        let payload = serde_json::to_value(&request.payload).expect("payload value");
        let result = routes
            .execute(&request, &payload, "uid:1")
            .expect("policy set");
        let result = serde_json::to_value(result).expect("result value");
        assert_eq!(result["revision"], 2);
        assert_eq!(result["policy"]["allowed_tools"], json!(["read"]));
        assert!(request.handoff.is_durable());
    }

    #[test]
    fn journal_rejects_wrong_bytes_for_the_same_rpc_id() {
        let (_root, _host, routes) = fixture();
        let request = call(
            "rpc-conflict",
            "workspace.policy.set",
            json!({"a":1}),
            false,
        );
        let desired = b"{\"format\":1,\"revision\":2}\n";
        let result = to_ijson(&json!({"format":1})).expect("result");
        assert!(matches!(
            routes.journal.begin(
                &request,
                "workspaces/workspace-1/workspace.json",
                1,
                2,
                desired,
                &result
            ),
            Ok(AdminBegin::Execute)
        ));
        let changed = call("rpc-conflict", "workspace.policy.set", json!({"a":2}), true);
        let error = routes
            .journal
            .recover(&changed)
            .expect_err("wrong bytes reject");
        assert_eq!(error.code, "idempotency-conflict");
    }

    #[test]
    fn crash_before_publish_is_completed_before_an_unrelated_rpc() {
        let (_root, _host, routes) = fixture();
        let connection = Connection {
            id: "openai-main".to_owned(),
            name: Some("OpenAI".to_owned()),
            protocol_family: "responses".to_owned(),
            dialect_id: "openai_responses_v1".to_owned(),
            endpoint_owner: "openai".to_owned(),
            gateway_translation: "direct".to_owned(),
            evidence_revision: "openai-2026-08-01".to_owned(),
            endpoint: "https://api.openai.com/v1".to_owned(),
            credential_id: None,
        };
        let desired = ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![Provider::from(&connection)],
            web_search: None,
        };
        let interrupted = call(
            "rpc-interrupted-provider",
            "providers.connection.save",
            json!({"expectedProvidersRevision":0,"connection":openai_connection()}),
            false,
        );
        let result = to_ijson(&json!({"format":1,"providersRevision":1,"connection":connection_view(&desired.providers[0],Readiness::Ready)})).expect("result");
        assert!(matches!(
            routes.journal.begin(
                &interrupted,
                "config/providers.json",
                0,
                1,
                &desired.canonical_bytes().expect("desired"),
                &result,
            ),
            Ok(AdminBegin::Execute)
        ));

        let unrelated = call(
            "rpc-unrelated-policy",
            "workspace.policy.set",
            json!({"workspaceId":"workspace-1","expectedRevision":1,"policy":{
                "network":false,"allowed_tools":["read"],"writable_roots":[]
            }}),
            false,
        );
        let payload = serde_json::to_value(&unrelated.payload).expect("payload value");
        routes
            .execute(&unrelated, &payload, "uid:1")
            .expect("unrelated succeeds after recovery");
        assert_eq!(routes.repository.providers().expect("providers"), desired);
        let record = routes
            .journal
            .records()
            .expect("records")
            .into_iter()
            .find(|record| record.rpc_id == "rpc-interrupted-provider")
            .expect("provider record");
        assert_eq!(record.phase, AdminPhase::Committed);
    }

    #[test]
    fn startup_closes_policy_crashes_on_both_sides_of_the_side_effect() {
        for side_effect_ran in [false, true] {
            let (root, host, routes) = fixture();
            let request = call(
                if side_effect_ran {
                    "rpc-policy-after-side-effect"
                } else {
                    "rpc-policy-before-side-effect"
                },
                "workspace.policy.set",
                json!({"workspaceId":"workspace-1","expectedRevision":1,"policy":{
                    "network":false,"allowed_tools":["read"],"writable_roots":[]
                }}),
                false,
            );
            let mut desired = routes
                .repository
                .workspace("workspace-1")
                .expect("workspace");
            let previous = routes.repository.resolve("workspace-1").expect("previous");
            desired.revision = 2;
            desired.policy = Some(WorkspacePolicy {
                allowed_tools: vec!["read".to_owned()],
                ..WorkspacePolicy::default()
            });
            let result =
                policy_result(2, desired.policy.as_ref().expect("policy")).expect("result");
            routes
                .journal
                .begin(
                    &request,
                    "workspaces/workspace-1/workspace.json",
                    1,
                    2,
                    &desired.canonical_bytes().expect("desired"),
                    &result,
                )
                .expect("prepare");
            routes
                .repository
                .publish_workspace_policy("workspace-1", 1, desired.policy.clone().expect("policy"))
                .expect("publish");
            if side_effect_ran {
                host.workspace_policy_published("workspace-1", &previous)
                    .expect("side effect");
            }
            drop(routes);
            let recovered =
                ClientAdminRoutes::new(root.path(), Arc::clone(&host)).expect("startup recovery");
            let record = recovered
                .journal
                .records()
                .expect("records")
                .into_iter()
                .find(|record| record.rpc_id == request.rpc_id)
                .expect("policy record");
            assert_eq!(record.phase, AdminPhase::Committed);
            assert_eq!(
                recovered
                    .repository
                    .workspace("workspace-1")
                    .expect("workspace"),
                desired
            );
        }
    }
}
