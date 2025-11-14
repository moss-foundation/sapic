use async_trait::async_trait;
use sapic_base::theme::types::primitives::{ThemeId, ThemeMode};
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
