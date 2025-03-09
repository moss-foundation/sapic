use anyhow::{anyhow, Result};
use moss_app::service::AppService;
use moss_db::encrypted_bincode_store::EncryptedBincodeStore;
use moss_fs::ports::FileSystem;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::models::{
    operations::{GetTranslationsInput, ListLocalesOutput},
    primitives::LocaleId,
    types::LocaleDescriptor,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MockVault {}

const LOCALES_REGISTRY_FILE: &str = "locales.json";

pub struct LocaleService {
    locales_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    v_store: EncryptedBincodeStore<'static, &'static str, MockVault>,
    locales: OnceCell<HashMap<LocaleId, LocaleDescriptor>>,
}

impl LocaleService {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        locales_dir: PathBuf,
        v_store: EncryptedBincodeStore<'static, &'static str, MockVault>,
    ) -> Self {
        Self {
            locales_dir,
            fs,
            v_store,
            locales: OnceCell::new(),
        }
    }

    async fn locales(&self) -> Result<&HashMap<LocaleId, LocaleDescriptor>> {
        self.locales
            .get_or_try_init(|| async move {
                let descriptors = self.parse_registry_file().await?;
                let result = descriptors
                    .into_iter()
                    .map(|item| (item.identifier.clone(), item))
                    .collect::<HashMap<LocaleId, LocaleDescriptor>>();

                Ok::<HashMap<LocaleId, LocaleDescriptor>, anyhow::Error>(result)
            })
            .await
    }

    async fn parse_registry_file(&self) -> Result<Vec<LocaleDescriptor>> {
        let reader = self
            .fs
            .open_file(&self.locales_dir.join(LOCALES_REGISTRY_FILE))
            .await?;

        Ok(serde_json::from_reader(reader)?)
    }

    async fn read_translations_from_file(
        &self,
        language: &str,
        namespace: &str,
    ) -> Result<JsonValue> {
        let full_path = self
            .locales_dir
            .join(language)
            .join(format!("{namespace}.json"));

        if !full_path.exists() {
            return Err(anyhow!("File '{}' does not exist", full_path.display()));
        }

        if !full_path.is_file() {
            return Err(anyhow!("Path '{}' is not a file", full_path.display()));
        }

        let reader = self.fs.open_file(&full_path).await?;

        Ok(serde_json::from_reader(reader)?)
    }
}

impl LocaleService {
    pub async fn list_locales(&self) -> Result<ListLocalesOutput> {
        let locales = self.locales().await?;

        Ok(ListLocalesOutput {
            contents: locales.into_iter().map(|(_, item)| item).cloned().collect(),
        })
    }

    pub async fn get_translations(&self, input: &GetTranslationsInput) -> Result<JsonValue> {
        let translations = self
            .read_translations_from_file(&input.language, &input.namespace)
            .await?;

        Ok(translations)
    }
}

impl AppService for LocaleService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &(dyn std::any::Any + Send) {
        self
    }
}
