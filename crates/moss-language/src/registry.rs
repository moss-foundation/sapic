use async_trait::async_trait;
use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::primitives::{Direction, LanguageId};

#[async_trait]
pub trait LanguageRegistry: Send + Sync {
    async fn register(&self, items: Vec<LanguageRegistryItem>);
    async fn get(&self, id: &LanguageId) -> Option<LanguageRegistryItem>;
    async fn list(&self) -> HashMap<LanguageId, LanguageRegistryItem>;
}

#[derive(Debug, Clone)]
pub struct LanguageRegistryItem {
    pub identifier: LanguageId,
    pub display_name: String,
    pub code: String,
    pub direction: Option<Direction>,
    pub path: PathBuf,
}

pub struct AppLanguageRegistry {
    // The frontend always uses the language code to fetch localization
    locales: RwLock<HashMap<LanguageId, LanguageRegistryItem>>,
}

#[async_trait]
impl LanguageRegistry for AppLanguageRegistry {
    async fn register(&self, items: Vec<LanguageRegistryItem>) {
        self.locales.write().await.extend(
            items
                .into_iter()
                .map(|item| (item.identifier.clone(), item)),
        )
    }

    async fn get(&self, id: &LanguageId) -> Option<LanguageRegistryItem> {
        self.locales.read().await.get(id).cloned()
    }

    async fn list(&self) -> HashMap<LanguageId, LanguageRegistryItem> {
        self.locales.read().await.clone()
    }
}

impl AppLanguageRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            locales: RwLock::new(HashMap::new()),
        })
    }
}

#[derive(Deref, Clone)]
pub struct GlobalLanguageRegistry(Arc<dyn LanguageRegistry>);

impl dyn LanguageRegistry {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn LanguageRegistry> {
        delegate.global::<GlobalLanguageRegistry>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(delegate: &AppDelegate<R>, v: Arc<dyn LanguageRegistry>) {
        delegate.set_global(GlobalLanguageRegistry(v))
    }
}
