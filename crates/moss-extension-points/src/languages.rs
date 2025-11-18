use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_language::{
    contribution::LanguageContributionDecl,
    registry::{GlobalLanguageRegistry, LanguageRegistryItem},
};
use sapic_base::extension::{contribution::ContributionKey, types::LoadedExtensionInfo};
use sapic_runtime::extension_point::ExtensionPoint;
use serde_json::Value as JsonValue;

const LANGUAGES_KEY: ContributionKey = ContributionKey::new("languages");

pub struct LanguageExtensionPoint;

impl LanguageExtensionPoint {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for LanguageExtensionPoint {
    fn key(&self) -> ContributionKey {
        LANGUAGES_KEY
    }

    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &LoadedExtensionInfo,
        contribution: JsonValue,
    ) -> joinerror::Result<()> {
        if !contribution.is_array() {
            joinerror::bail!("languages contribution must be an array");
        }

        let languages: Vec<LanguageContributionDecl> = serde_json::from_value(contribution)?;
        let items = languages
            .into_iter()
            .map(|entry| LanguageRegistryItem {
                display_name: entry.display_name,
                code: entry.code,
                direction: entry.direction,
                path: info.source.join(entry.path),
            })
            .collect();

        app_delegate
            .global::<GlobalLanguageRegistry>()
            .register(items)
            .await;

        Ok(())
    }
}
