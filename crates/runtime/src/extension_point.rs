use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_base::extension::{contribution::ContributionKey, types::LoadedExtensionInfo};
use serde_json::Value as JsonValue;

#[async_trait]
pub trait ExtensionPoint<R: AppRuntime>: Send + Sync + 'static {
    fn key(&self) -> ContributionKey;
    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &LoadedExtensionInfo,
        contribution: JsonValue,
    ) -> joinerror::Result<()>;
}
