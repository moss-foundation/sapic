use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_extension::{ExtensionInfo, ExtensionPoint, contribution::ContributionKey};
use moss_locale::{
    contribution::LocaleContributionDecl,
    registry::{GlobalLocaleRegistry, LocaleRegistryItem},
};
use serde_json::Value as JsonValue;

const LOCALES_KEY: ContributionKey = ContributionKey::new("locales");

pub struct LocaleExtensionPoint;

impl LocaleExtensionPoint {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for LocaleExtensionPoint {
    fn key(&self) -> ContributionKey {
        LOCALES_KEY
    }

    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &ExtensionInfo,
        contribution: JsonValue,
    ) -> joinerror::Result<()> {
        if !contribution.is_array() {
            joinerror::bail!("locales contribution must be an array");
        }

        let locales: Vec<LocaleContributionDecl> = serde_json::from_value(contribution)?;
        let items = locales
            .into_iter()
            .map(|entry| LocaleRegistryItem {
                identifier: entry.identifier,
                display_name: entry.display_name,
                code: entry.code,
                direction: entry.direction,
                path: info.source.join(entry.path),
            })
            .collect();

        app_delegate
            .global::<GlobalLocaleRegistry>()
            .register(items)
            .await;

        Ok(())
    }
}
