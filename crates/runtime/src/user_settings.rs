use async_trait::async_trait;
use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use sapic_platform::configuration::ConfigurationBackend;
use sapic_system::configuration::SettingsStore;
use serde_json::{Map, Value as JsonValue};
use std::sync::Arc;

const SETTINGS_FILE: &str = "settings.json";

pub struct UserSettingsService {
    backend: ConfigurationBackend,
}

impl UserSettingsService {
    pub async fn new<R: AppRuntime>(
        delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            backend: ConfigurationBackend::new(
                fs,
                delegate.user_dir().join("user").join(SETTINGS_FILE),
            )
            .await?,
        })
    }
}

#[async_trait]
impl SettingsStore for UserSettingsService {
    async fn values(&self) -> Map<String, JsonValue> {
        self.backend.values().await
    }

    async fn get_value(&self, key: &str) -> Option<JsonValue> {
        self.backend.get_value(key).await
    }

    async fn update_value(&self, key: &str, value: JsonValue) -> joinerror::Result<()> {
        self.backend
            .update_value(key, value)
            .await
            .join_err_with::<()>(|| format!("failed to update value for key: {}", key))
    }

    async fn remove_value(&self, _key: &str) -> joinerror::Result<Option<JsonValue>> {
        unimplemented!()
    }
}
