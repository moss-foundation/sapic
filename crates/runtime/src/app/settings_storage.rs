use async_trait::async_trait;
use rustc_hash::{FxHashMap, FxHashSet};
use sapic_base::configuration::ConfigurationModel;
use sapic_core::{
    context::AnyAsyncContext,
    subscription::{EventEmitter, EventMarker},
};
use sapic_system::configuration::{SettingsStore, configuration_registry::ConfigurationRegistry};
use serde_json::Value as JsonValue;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum SettingScope {
    User,
    Workspace(String),
}

impl SettingScope {
    pub fn is_user(&self) -> bool {
        matches!(self, SettingScope::User)
    }
    pub fn is_workspace(&self) -> bool {
        matches!(self, SettingScope::Workspace(_))
    }
}

#[async_trait]
pub trait SettingsStorage: Send + Sync {
    async fn values(&self, scope: &SettingScope) -> Vec<(String, JsonValue)>;
    async fn get_value(
        &self,
        scope: &SettingScope,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>>;
    async fn update_value(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: &SettingScope,
        key: &str,
        value: JsonValue,
    ) -> joinerror::Result<()>;
    async fn remove_value(
        &self,
        scope: &SettingScope,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>>;

    async fn batch_update_value(
        &self,
        scope: &SettingScope,
        values: &[(&str, JsonValue)],
    ) -> joinerror::Result<()>;
    async fn batch_get_value(
        &self,
        scope: &SettingScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;
    async fn batch_remove_value(
        &self,
        scope: &SettingScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>>;
}

#[derive(Debug, Clone)]
pub struct OnDidChangeSettings {
    pub scope: SettingScope,
    pub affected_keys: FxHashSet<String>,
    pub changes: FxHashMap<String, JsonValue>,
}

impl EventMarker for OnDidChangeSettings {}

pub struct AppSettingsStorage {
    registry: Arc<dyn ConfigurationRegistry>,
    defaults: ConfigurationModel,
    user_settings: Arc<dyn SettingsStore>,

    #[allow(unused)]
    on_did_change_configuration_emitter: EventEmitter<OnDidChangeSettings>,
}

impl AppSettingsStorage {
    pub fn new(
        registry: Arc<dyn ConfigurationRegistry>,
        user_settings: Arc<dyn SettingsStore>,
    ) -> Self {
        let defaults = registry.defaults();
        Self {
            registry,
            defaults: ConfigurationModel {
                keys: defaults.keys().map(|key| key.to_string()).collect(),
                contents: defaults
                    .into_iter()
                    .map(|(key, value)| (key.to_string(), value))
                    .collect(),
            },
            user_settings,
            on_did_change_configuration_emitter: EventEmitter::<OnDidChangeSettings>::new(),
        }
    }
}

#[async_trait]
impl SettingsStorage for AppSettingsStorage {
    async fn values(&self, scope: &SettingScope) -> Vec<(String, JsonValue)> {
        if scope.is_user() {
            return self
                .defaults
                .merge(&self.user_settings.values().await)
                .values();
        }

        if scope.is_workspace() {
            unimplemented!()
        }

        vec![]
    }

    async fn get_value(
        &self,
        scope: &SettingScope,
        key: &str,
    ) -> joinerror::Result<Option<JsonValue>> {
        if scope.is_user() {
            return Ok(self
                .defaults
                .merge(&self.user_settings.values().await)
                .get(key)
                .cloned());
        }

        if scope.is_workspace() {
            unimplemented!()
        }

        Ok(None)
    }
    async fn update_value(
        &self,
        ctx: &dyn AnyAsyncContext,
        scope: &SettingScope,
        key: &str,
        value: JsonValue,
    ) -> joinerror::Result<()> {
        if !self.registry.is_parameter_known(key) {
            tracing::warn!("parameter '{}' is unknown", key);
        } else {
            self.registry.validate_parameter(key, &value)?;
        }

        if scope.is_user() {
            return self.user_settings.update_value(ctx, key, value).await;
        }

        if scope.is_workspace() {
            unimplemented!()
        }

        Ok(self
            .on_did_change_configuration_emitter
            .fire(OnDidChangeSettings {
                scope: scope.clone(),
                affected_keys: FxHashSet::from_iter([key.to_string()]),
                changes: FxHashMap::from_iter([(key.to_string(), value)]),
            })
            .await)
    }

    async fn remove_value(
        &self,
        _scope: &SettingScope,
        _key: &str,
    ) -> joinerror::Result<Option<JsonValue>> {
        unimplemented!()
    }

    async fn batch_get_value(
        &self,
        scope: &SettingScope,
        keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        let mut result = vec![];

        if scope.is_user() {
            let settings = self.defaults.merge(&self.user_settings.values().await);

            for key in keys {
                result.push((key.to_string(), settings.get(key).cloned()));
            }
        }

        Ok(result)
    }

    async fn batch_update_value(
        &self,
        _scope: &SettingScope,
        _values: &[(&str, JsonValue)],
    ) -> joinerror::Result<()> {
        unimplemented!()
    }

    async fn batch_remove_value(
        &self,
        _scope: &SettingScope,
        _keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
        unimplemented!()
    }
}
