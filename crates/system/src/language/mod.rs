pub mod language_registry;
pub mod language_service;

use async_trait::async_trait;
use sapic_base::language::types::primitives::LanguageCode;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::Path};

use crate::language::language_registry::LanguageRegistryItem;

#[async_trait]
pub trait LanguagePackRegistry: Send + Sync {
    async fn register(&self, items: Vec<LanguageRegistryItem>);
    async fn get(&self, code: &LanguageCode) -> Option<LanguageRegistryItem>;
    async fn list(&self) -> HashMap<LanguageCode, LanguageRegistryItem>;
}

#[async_trait]
pub trait LanguagePackLoader: Send + Sync {
    async fn load_namespace(&self, path: &Path, namespace: &str) -> joinerror::Result<JsonValue>;
}
