use joinerror::{OptionExt, ResultExt};
use moss_applib::errors::Internal;
use moss_fs::{FileSystem, FsResultExt};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::{primitives::LocaleId, types::LocaleInfo};

const LOCALES_REGISTRY_FILE: &str = "locales.json";

struct ServiceState {
    locales: HashMap<LocaleId, LocaleInfo>,
    default_locale: LocaleInfo,
}

pub struct LocaleService {
    locales_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    state: RwLock<ServiceState>,
}

impl LocaleService {
    pub async fn new(fs: Arc<dyn FileSystem>, locales_dir: PathBuf) -> joinerror::Result<Self> {
        let rdr = fs
            .open_file(&locales_dir.join(LOCALES_REGISTRY_FILE))
            .await?;

        let parsed: Vec<LocaleInfo> = serde_json::from_reader(rdr)?;
        let locales = parsed
            .into_iter()
            .map(|item| (item.identifier.clone(), item))
            .collect::<HashMap<LocaleId, LocaleInfo>>();

        let default_locale = if let Some(locale) = locales
            .values()
            .find(|locale| locale.is_default.unwrap_or(false))
            .cloned()
        {
            locale
        } else {
            locales
                .values()
                .next() // We take the first locale as the default locale if no default locale is found
                .ok_or_join_err::<()>("the app must have at least one locale")?
                .clone()
        };

        Ok(Self {
            locales_dir,
            fs,
            state: RwLock::new(ServiceState {
                locales,
                default_locale,
            }),
        })
    }

    pub async fn default_locale(&self) -> LocaleInfo {
        let state = self.state.read().await;
        state.default_locale.clone()
    }

    pub async fn locales(&self) -> HashMap<LocaleId, LocaleInfo> {
        let state = self.state.read().await;
        state.locales.clone()
    }

    pub async fn read_translations_from_file(
        &self,
        language: &str,
        namespace: &str,
    ) -> joinerror::Result<JsonValue> {
        let full_path = self
            .locales_dir
            .join(language)
            .join(format!("{namespace}.json"));

        let rdr = self
            .fs
            .open_file(&full_path)
            .await
            .join_err_with::<Internal>(|| {
                format!("failed to open locale file `{}`", full_path.display())
            })?;

        Ok(serde_json::from_reader(rdr)
            .join_err::<Internal>("failed to parse locale file as JSON")?)
    }
}
