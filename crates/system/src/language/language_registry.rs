use async_trait::async_trait;
use sapic_base::language::types::primitives::{LanguageCode, LanguageDirection};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::language::LanguagePackRegistry;

#[derive(Debug, Clone)]
pub struct LanguageRegistryItem {
    pub display_name: String,
    pub code: LanguageCode,
    pub direction: Option<LanguageDirection>,
    pub path: PathBuf,
}

pub struct AppLanguagePackRegistry {
    // The frontend always uses the language code to fetch localization
    languages: RwLock<HashMap<LanguageCode, LanguageRegistryItem>>,
}

impl AppLanguagePackRegistry {
    pub fn new() -> Arc<Self> {
        Self {
            languages: RwLock::new(HashMap::new()),
        }
        .into()
    }
}

#[async_trait]
impl LanguagePackRegistry for AppLanguagePackRegistry {
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
