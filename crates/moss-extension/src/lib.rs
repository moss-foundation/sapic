pub mod contribution;
pub mod manifest;
pub mod scanner;

use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use serde_json::Value as JsonValue;
use std::path::PathBuf;

use crate::contribution::ContributionKey;

pub struct ExtensionInfo {
    pub source: PathBuf,
}

#[async_trait]
pub trait ExtensionPoint<R: AppRuntime>: Send + Sync + 'static {
    fn key(&self) -> ContributionKey;
    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &ExtensionInfo,
        contribution: JsonValue,
    ) -> joinerror::Result<()>;
}
