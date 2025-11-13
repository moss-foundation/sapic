use async_trait::async_trait;
use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::primitives::{ThemeId, ThemeMode};

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

#[derive(Deref, Clone)]
pub struct GlobalThemeRegistry(Arc<dyn ThemeRegistry>);

impl dyn ThemeRegistry {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn ThemeRegistry> {
        delegate.global::<GlobalThemeRegistry>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(delegate: &AppDelegate<R>, v: Arc<dyn ThemeRegistry>) {
        delegate.set_global(GlobalThemeRegistry(v));
    }
}
