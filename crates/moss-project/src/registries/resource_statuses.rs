use async_trait::async_trait;
use joinerror::ResultExt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::contribution::RegisterResourceStatusesContribution;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceStatusRegistryItem {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub resources: Vec<String>,
}

#[async_trait]
pub trait ResourceStatusRegistry: Send + Sync {
    async fn register(&self, items: Vec<ResourceStatusRegistryItem>);
}

pub struct AppResourceStatusRegistry {
    statuses: RwLock<HashMap<String, ResourceStatusRegistryItem>>,
}

#[async_trait]
impl ResourceStatusRegistry for AppResourceStatusRegistry {
    async fn register(&self, items: Vec<ResourceStatusRegistryItem>) {
        self.statuses
            .write()
            .await
            .extend(items.into_iter().map(|item| (item.name.clone(), item)));
    }
}

impl AppResourceStatusRegistry {
    pub fn new() -> joinerror::Result<Arc<Self>> {
        let mut statuses = HashMap::new();
        for contrib in inventory::iter::<RegisterResourceStatusesContribution>() {
            let decl: Vec<ResourceStatusRegistryItem> = serde_json::from_str(contrib.0)
                .join_err_with::<()>(|| {
                    format!("failed to parse included resource statuses: {}", contrib.0)
                })?;

            statuses.extend(decl.into_iter().map(|item| (item.name.clone(), item)));
        }

        Ok(Self {
            statuses: RwLock::new(statuses),
        }
        .into())
    }
}
