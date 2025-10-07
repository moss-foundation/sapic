use async_trait::async_trait;
use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::primitives::{Direction, LocaleId};

#[async_trait]
pub trait LocaleRegistry: Send + Sync {
    async fn register(&self, items: Vec<LocaleRegistryItem>);
    async fn get(&self, id: &LocaleId) -> Option<LocaleRegistryItem>;
    async fn list(&self) -> HashMap<LocaleId, LocaleRegistryItem>;
}

#[derive(Debug, Clone)]
pub struct LocaleRegistryItem {
    pub identifier: LocaleId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<Direction>,
    pub path: PathBuf,
}

pub struct AppLocaleRegistry {
    // The frontend always uses the language code to fetch localization
    locales: RwLock<HashMap<LocaleId, LocaleRegistryItem>>,
}

#[async_trait]
impl LocaleRegistry for AppLocaleRegistry {
    async fn register(&self, items: Vec<LocaleRegistryItem>) {
        self.locales.write().await.extend(
            items
                .into_iter()
                .map(|item| (item.identifier.clone(), item)),
        )
    }

    async fn get(&self, id: &LocaleId) -> Option<LocaleRegistryItem> {
        self.locales.read().await.get(id).cloned()
    }

    async fn list(&self) -> HashMap<LocaleId, LocaleRegistryItem> {
        self.locales.read().await.clone()
    }
}

impl AppLocaleRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            locales: RwLock::new(HashMap::new()),
        })
    }
}

#[derive(Deref, Clone)]
pub struct GlobalLocaleRegistry(Arc<dyn LocaleRegistry>);

impl dyn LocaleRegistry {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn LocaleRegistry> {
        delegate.global::<GlobalLocaleRegistry>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(delegate: &AppDelegate<R>, v: Arc<dyn LocaleRegistry>) {
        delegate.set_global(GlobalLocaleRegistry(v))
    }
}
