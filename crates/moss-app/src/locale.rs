mod registry;

use moss_applib::errors::Internal;
use moss_fs::{FileSystem, FsResultExt};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    locale::registry::LocaleRegistryItem,
    models::{primitives::LocaleId, types::LocaleInfo},
};

pub(crate) const LOCALES_REGISTRY_FILE: &str = "locales.json";

struct ServiceState {
    locales: HashMap<LocaleId, LocaleRegistryItem>,
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

        let parsed: Vec<LocaleRegistryItem> = serde_json::from_reader(rdr)?;
        let locales = parsed
            .into_iter()
            .map(|item| (item.identifier.clone(), item))
            .collect::<HashMap<LocaleId, LocaleRegistryItem>>();

        Ok(Self {
            locales_dir,
            fs,
            state: RwLock::new(ServiceState { locales }),
        })
    }

    pub async fn get_locale(&self, identifier: &LocaleId) -> Option<LocaleInfo> {
        let state = self.state.read().await;
        state
            .locales
            .get(identifier)
            .cloned()
            .map(|item| LocaleInfo {
                identifier: item.identifier,
                display_name: item.display_name,
                code: item.code,
                direction: item.direction,
                order: item.order,
                is_default: item.is_default,
            })
    }

    pub async fn locales(&self) -> HashMap<LocaleId, LocaleInfo> {
        let state = self.state.read().await;
        state
            .locales
            .clone()
            .into_iter()
            .map(|(id, item)| {
                (
                    id,
                    LocaleInfo {
                        identifier: item.identifier,
                        display_name: item.display_name,
                        code: item.code,
                        direction: item.direction,
                        order: item.order,
                        is_default: item.is_default,
                    },
                )
            })
            .collect()
    }

    pub async fn get_namespace(&self, code: &str, ns: &str) -> joinerror::Result<JsonValue> {
        let abs_path = self.locales_dir.join(code).join(format!("{ns}.json"));

        let rdr = self
            .fs
            .open_file(&abs_path)
            .await
            .join_err_with::<Internal>(|| {
                format!("failed to open locale file `{}`", abs_path.display())
            })?;

        let parsed: JsonValue = serde_json::from_reader(rdr)?;

        Ok(parsed)
    }
}
