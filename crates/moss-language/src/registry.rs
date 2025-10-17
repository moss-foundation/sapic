use async_trait::async_trait;
use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::models::primitives::{LanguageCode, LanguageDirection};

#[async_trait]
pub trait LanguageRegistry: Send + Sync {
    async fn register(&self, items: Vec<LanguageRegistryItem>);
    async fn get(&self, code: &LanguageCode) -> Option<LanguageRegistryItem>;
    async fn list(&self) -> HashMap<LanguageCode, LanguageRegistryItem>;
}

#[derive(Debug, Clone)]
pub struct LanguageRegistryItem {
    pub display_name: String,
    pub code: LanguageCode,
    pub direction: Option<LanguageDirection>,
    pub path: PathBuf,
}

pub struct AppLanguageRegistry {
    // The frontend always uses the language code to fetch localization
    languages: RwLock<HashMap<LanguageCode, LanguageRegistryItem>>,
}

#[async_trait]
impl LanguageRegistry for AppLanguageRegistry {
    async fn register(&self, items: Vec<LanguageRegistryItem>) {
        self.languages
            .write()
            .await
            .extend(items.into_iter().map(|item| (item.code.clone(), item)))
    }

    async fn get(&self, code: &LanguageCode) -> Option<LanguageRegistryItem> {
        self.languages.read().await.get(code).cloned()
    }

    async fn list(&self) -> HashMap<LanguageCode, LanguageRegistryItem> {
        self.languages.read().await.clone()
    }
}

impl AppLanguageRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            languages: RwLock::new(HashMap::new()),
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
