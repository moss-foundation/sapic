pub mod theme_registry;
pub mod theme_service;

use crate::theme::theme_registry::ThemeRegistryItem;
use async_trait::async_trait;
use sapic_base::theme::{manifest::ThemeFile, types::primitives::ThemeId};
use sapic_core::context::AnyAsyncContext;
use std::{collections::HashMap, path::Path, sync::Arc};

pub type DynThemeLoader = Arc<dyn ThemeLoader>;
pub type DynThemeRegistry = Arc<dyn ThemeRegistry>;

#[async_trait]
pub trait ThemeLoader: Send + Sync {
    async fn load(&self, ctx: &dyn AnyAsyncContext, path: &Path) -> joinerror::Result<ThemeFile>;
}

#[async_trait]
pub trait ThemeRegistry: Send + Sync {
    async fn register(&self, items: Vec<ThemeRegistryItem>);
    async fn get(&self, identifier: &ThemeId) -> Option<ThemeRegistryItem>;
    async fn list(&self) -> HashMap<ThemeId, ThemeRegistryItem>;
}
