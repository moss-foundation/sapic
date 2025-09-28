mod conversion;
pub mod install;
mod models;
mod validation;

use async_trait::async_trait;
use joinerror::{OptionExt, ResultExt};
use moss_addon::ExtensionInfo;
use moss_app_delegate::AppDelegate;
use moss_applib::{
    AppRuntime,
    errors::{Internal, NotFound},
};
use moss_contrib::ContributionKey;
use moss_fs::{FileSystem, FsResultExt};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    extension::ExtensionPoint,
    models::{
        primitives::{ThemeId, ThemeMode},
        types::ColorThemeInfo,
    },
};

#[derive(Deserialize, Debug)]
pub struct ThemeContributionEntry {
    id: ThemeId,
    label: String,
    mode: ThemeMode,
    path: PathBuf,
}

pub struct ThemeExtensionPoint {}

impl ThemeExtensionPoint {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for ThemeExtensionPoint {
    fn key(&self) -> ContributionKey {
        ContributionKey::Themes
    }

    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &ExtensionInfo,
        data: JsonValue,
    ) -> joinerror::Result<()> {
        if !data.is_array() {
            joinerror::bail!("themes contribution must be an array");
        }

        let themes: Vec<ThemeContributionEntry> = serde_json::from_value(data)?;
        let items = themes
            .into_iter()
            .map(|entry| ThemeRegistryItem {
                id: entry.id,
                display_name: entry.label,
                mode: entry.mode,
                path: info.source.join(entry.path),
            })
            .collect();

        app_delegate
            .global::<GlobalThemeRegistry>()
            .register(items)
            .await;

        Ok(())
    }
}

#[async_trait]
pub trait ThemeRegistry: Send + Sync {
    async fn register(&self, items: Vec<ThemeRegistryItem>);
    async fn get(&self, identifier: &ThemeId) -> Option<ThemeRegistryItem>;
    async fn list(&self) -> HashMap<ThemeId, ThemeRegistryItem>;
}

#[derive(Debug, Clone)]
pub struct ThemeRegistryItem {
    pub id: ThemeId,
    pub display_name: String,
    pub mode: ThemeMode,
    pub path: PathBuf,
}

struct AppThemeRegistry {
    themes: RwLock<HashMap<ThemeId, ThemeRegistryItem>>,
}

#[async_trait]
impl ThemeRegistry for AppThemeRegistry {
    async fn register(&self, items: Vec<ThemeRegistryItem>) {
        self.themes
            .write()
            .await
            .extend(items.into_iter().map(|item| (item.id.clone(), item)));
    }

    async fn get(&self, identifier: &ThemeId) -> Option<ThemeRegistryItem> {
        self.themes.read().await.get(identifier).cloned()
    }

    async fn list(&self) -> HashMap<ThemeId, ThemeRegistryItem> {
        self.themes.read().await.clone()
    }
}

impl AppThemeRegistry {
    pub fn new() -> Self {
        Self {
            themes: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Deref, Clone)]
struct GlobalThemeRegistry(Arc<dyn ThemeRegistry>);

impl dyn ThemeRegistry {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn ThemeRegistry> {
        delegate.global::<GlobalThemeRegistry>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(delegate: &AppDelegate<R>, v: Arc<dyn ThemeRegistry>) {
        delegate.set_global(GlobalThemeRegistry(v));
    }
}

pub struct ThemeService {
    themes_dir: PathBuf,
    fs: Arc<dyn FileSystem>,
    registry: Arc<dyn ThemeRegistry>,
}

impl ThemeService {
    pub async fn new(fs: Arc<dyn FileSystem>, themes_dir: PathBuf) -> joinerror::Result<Self> {
        Ok(Self {
            themes_dir,
            fs,
            registry: Arc::new(AppThemeRegistry::new()),
        })
    }

    pub fn registry(&self) -> Arc<dyn ThemeRegistry> {
        self.registry.clone()
    }

    pub async fn themes(&self) -> HashMap<ThemeId, ColorThemeInfo> {
        let themes = self.registry.list().await;
        themes
            .into_iter()
            .map(|(id, item)| {
                (
                    id,
                    ColorThemeInfo {
                        identifier: item.id,
                        display_name: item.display_name,
                        mode: item.mode,
                        order: None,
                        source: item.path,
                        is_default: None,
                    },
                )
            })
            .collect()
    }

    pub async fn read(&self, id: &ThemeId) -> joinerror::Result<String> {
        let item = self
            .registry
            .get(id)
            .await
            .ok_or_join_err_with::<NotFound>(|| format!("theme with id `{}` not found", id))?;

        let mut rdr = self
            .fs
            .open_file(&self.themes_dir.join(item.path.clone()))
            .await
            .join_err_with::<Internal>(|| {
                format!("failed to open theme file `{}`", item.path.display())
            })?;

        let mut buf = String::new();
        rdr.read_to_string(&mut buf)
            .join_err::<Internal>("failed to read theme file")?;

        Ok(buf)
    }
}
