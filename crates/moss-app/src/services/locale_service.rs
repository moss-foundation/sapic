use anyhow::{Result, anyhow};
use moss_applib::ServiceMarker;
use moss_fs::FileSystem;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::OnceCell;

use crate::models::{primitives::LocaleId, types::LocaleInfo};

const LOCALES_REGISTRY_FILE: &str = "locales.json";

pub struct LocaleService {
    locales_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    locales: OnceCell<HashMap<LocaleId, LocaleInfo>>,
    default_locale: OnceCell<LocaleInfo>,
}

impl ServiceMarker for LocaleService {}

impl LocaleService {
    pub fn new(fs: Arc<dyn FileSystem>, locales_dir: PathBuf) -> Self {
        Self {
            locales_dir,
            fs,
            locales: Default::default(),
            default_locale: Default::default(),
        }
    }

    pub async fn default_locale(&self) -> Result<&LocaleInfo> {
        self.default_locale
            .get_or_try_init(|| async move {
                let locales = self.locales().await?;
                let default_locale = locales
                    .values()
                    .find(|locale| locale.is_default.unwrap_or(false))
                    .cloned();

                Ok::<LocaleInfo, anyhow::Error>(
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
    pub(crate) async fn locales(&self) -> Result<&HashMap<LocaleId, LocaleInfo>> {
        self.locales
            .get_or_try_init(|| async move {
                let descriptors = self.parse_registry_file().await?;
                let result = descriptors
                    .into_iter()
                    .map(|item| (item.identifier.clone(), item))
                    .collect::<HashMap<LocaleId, LocaleInfo>>();

                Ok::<HashMap<LocaleId, LocaleInfo>, anyhow::Error>(result)
            })
            .await
    }

    async fn parse_registry_file(&self) -> Result<Vec<LocaleInfo>> {
        let reader = self
            .fs
            .open_file(&self.locales_dir.join(LOCALES_REGISTRY_FILE))
            .await?;

        Ok(serde_json::from_reader(reader)?)
    }

    pub(crate) async fn read_translations_from_file(
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
