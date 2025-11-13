use async_trait::async_trait;
use joinerror::OptionExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::NotFound};
use moss_fs::FileSystem;
use sapic_base::theme::types::{
    ColorThemeInfo,
    primitives::{ThemeId, ThemeMode},
};
use sapic_platform::theme::loader::ThemeLoader;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

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

pub struct AppThemeRegistry {
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
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            themes: RwLock::new(HashMap::new()),
        })
    }
}

pub struct ThemeService {
    loader: ThemeLoader,
    registry: Arc<dyn ThemeRegistry>,
}

impl ThemeService {
    pub async fn new<R: AppRuntime>(
        app_delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
        registry: Arc<dyn ThemeRegistry>,
    ) -> joinerror::Result<Self> {
        Ok(Self {
            registry,
            loader: ThemeLoader::new(fs, app_delegate.resource_dir().join("policies/theme.rego")),
        })
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
                        order: None, // FIXME
                        source: item.path,
                        is_default: None, // FIXME
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

        let theme = self.loader.load(&item.path).await?;

        // TODO: apply color theme token overrides

        let css = sapic_base::theme::convert(&theme).await?;

        Ok(css)
    }
}
