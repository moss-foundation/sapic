use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_project::registries::{
    GlobalResourceStatusRegistry, resource_statuses::ResourceStatusRegistryItem,
};
use sapic_base::extension::{contribution::ContributionKey, types::LoadedExtensionInfo};
use sapic_runtime::extension_point::ExtensionPoint;
use serde_json::Value as JsonValue;

const RESOURCE_STATUSES_KEY: ContributionKey = ContributionKey::new("resource_statuses");

pub struct ResourceStatusesExtensionPoint;

impl ResourceStatusesExtensionPoint {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for ResourceStatusesExtensionPoint {
    fn key(&self) -> ContributionKey {
        RESOURCE_STATUSES_KEY
    }

    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        _: &LoadedExtensionInfo,
        data: JsonValue,
    ) -> joinerror::Result<()> {
        if !data.is_array() {
            joinerror::bail!("resource statuses contribution must be an array");
        }

        let statuses: Vec<ResourceStatusRegistryItem> = serde_json::from_value(data)?;
        app_delegate
            .global::<GlobalResourceStatusRegistry>()
            .register(statuses)
            .await;

        Ok(())
    }
}
