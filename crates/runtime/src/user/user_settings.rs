use async_trait::async_trait;
use joinerror::ResultExt;
use moss_fs::FileSystem;
use sapic_platform::configuration::FsSettingsStorage;
use sapic_system::configuration::SettingsStore;
use serde_json::{Map, Value as JsonValue};
use std::{path::PathBuf, sync::Arc};

const SETTINGS_FILE: &str = "settings.json";

pub struct UserSettingsService {
    storage: Arc<dyn SettingsStore>,
}

impl UserSettingsService {
    pub async fn new(abs_path: PathBuf, fs: Arc<dyn FileSystem>) -> joinerror::Result<Self> {
        Ok(Self {
            storage: Arc::new(FsSettingsStorage::new(fs, abs_path.join(SETTINGS_FILE)).await?),
        })
    }
}

#[async_trait]
impl SettingsStore for UserSettingsService {
    async fn values(&self) -> Map<String, JsonValue> {
        self.storage.values().await
    }

    async fn get_value(&self, key: &str) -> Option<JsonValue> {
        self.storage.get_value(key).await
    }

    async fn update_value(&self, key: &str, value: JsonValue) -> joinerror::Result<()> {
        self.storage
            .update_value(key, value)
            .await
            .join_err_with::<()>(|| format!("failed to update value for key: {}", key))
    }

    async fn remove_value(&self, _key: &str) -> joinerror::Result<Option<JsonValue>> {
        unimplemented!()
    }
}
