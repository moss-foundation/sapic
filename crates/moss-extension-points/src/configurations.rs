use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_extension::{ExtensionInfo, ExtensionPoint, contribution::ContributionKey};
use serde_json::Value as JsonValue;

pub struct ConfigurationExtensionPoint;

impl ConfigurationExtensionPoint {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

const CONFIGURATIONS_KEY: ContributionKey = ContributionKey::new("configurations");

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for ConfigurationExtensionPoint {
    fn key(&self) -> ContributionKey {
        CONFIGURATIONS_KEY
    }

    #[allow(unused_variables)]
    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &ExtensionInfo,
        data: JsonValue,
    ) -> joinerror::Result<()> {
        unimplemented!()
    }
}
