use async_trait::async_trait;
use sapic_base::configuration::ConfigurationModel;
use sapic_system::configuration::{SettingsStore, configuration_registry::ConfigurationRegistry};
use serde_json::Value as JsonValue;
use std::sync::Arc;

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

pub struct AppSettingsStorage {
    registry: Arc<dyn ConfigurationRegistry>,
    defaults: ConfigurationModel,
    user_settings: Arc<dyn SettingsStore>,
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
            return self.user_settings.update_value(key, value).await;
        }

        if scope.is_workspace() {
            unimplemented!()
        }

        Ok(())
    }
    async fn remove_value(
        &self,
        _scope: &SettingScope,
        _key: &str,
    ) -> joinerror::Result<Option<JsonValue>> {
        unimplemented!()
    }
    async fn batch_update_value(
        &self,
        _scope: &SettingScope,
        _values: &[(&str, JsonValue)],
    ) -> joinerror::Result<()> {
        unimplemented!()
    }
    async fn batch_get_value(
        &self,
        _scope: &SettingScope,
        _keys: &[&str],
    ) -> joinerror::Result<Vec<(String, Option<JsonValue>)>> {
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
