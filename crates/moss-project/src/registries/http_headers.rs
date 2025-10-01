use async_trait::async_trait;
use joinerror::ResultExt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::contribution::RegisterHttpHeadersContribution;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpHeaderRegistryItem {
    pub name: String,
    pub value: String,
    pub protected: bool,
    pub disabled: bool,
    pub description: Option<String>,
}

#[async_trait]
pub trait HttpHeaderRegistry: Send + Sync {
    async fn register(&self, items: Vec<HttpHeaderRegistryItem>);
}

pub struct AppHttpHeaderRegistry {
    headers: RwLock<HashMap<String, HttpHeaderRegistryItem>>,
}

#[async_trait]
impl HttpHeaderRegistry for AppHttpHeaderRegistry {
    async fn register(&self, items: Vec<HttpHeaderRegistryItem>) {
        self.headers
            .write()
            .await
            .extend(items.into_iter().map(|item| (item.name.clone(), item)));
    }
}

impl AppHttpHeaderRegistry {
    pub fn new() -> joinerror::Result<Arc<Self>> {
        let mut headers = HashMap::new();
        for contrib in inventory::iter::<RegisterHttpHeadersContribution>() {
            let decl: Vec<HttpHeaderRegistryItem> = serde_json::from_str(contrib.0)
                .join_err_with::<()>(|| {
                    format!("failed to parse included http headers: {}", contrib.0)
                })?;

            headers.extend(decl.into_iter().map(|item| (item.name.clone(), item)));
        }

        Ok(Self {
            headers: RwLock::new(headers),
        }
        .into())
    }
}
