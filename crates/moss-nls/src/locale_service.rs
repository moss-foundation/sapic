use anyhow::{anyhow, Result};
use moss_app::service_pool::AppService;
use moss_fs::ports::FileSystem;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::models::{
    operations::{GetTranslationsInput, ListLocalesOutput},
    primitives::LocaleId,
    types::LocaleDescriptor,
};

const LOCALES_REGISTRY_FILE: &str = "locales.json";

pub struct LocaleService {
    locales_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    locales: OnceCell<HashMap<LocaleId, LocaleDescriptor>>,
    default_locale: OnceCell<LocaleDescriptor>,
}

impl LocaleService {
    pub fn new(fs: Arc<dyn FileSystem>, locales_dir: PathBuf) -> Self {
        Self {
            locales_dir,
            fs,
            locales: Default::default(),
            default_locale: Default::default(),
        }
    }

    pub async fn default_locale(&self) -> Result<&LocaleDescriptor> {
        self.default_locale
            .get_or_try_init(|| async move {
                let locales = self.locales().await?;
                let default_locale = locales
                    .values()
                    .find(|locale| locale.is_default.unwrap_or(false))
                    .cloned();

                Ok::<LocaleDescriptor, anyhow::Error>(
                    default_locale.unwrap_or(
                        locales
                            .values()
                            .next() // We take the first theme as the default theme if no default theme is found
                            .expect("The app must have at least one theme")
                            .clone(),
                    ),
                )
            })
            .await
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

impl AppService for LocaleService {}
