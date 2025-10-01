use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_extension::{ExtensionInfo, ExtensionPoint, contribution::ContributionKey};
use moss_theme::{
    contribution::ThemeContributionDecl,
    registry::{GlobalThemeRegistry, ThemeRegistryItem},
};
use serde_json::Value as JsonValue;

const THEMES_KEY: ContributionKey = ContributionKey::new("themes");

pub struct ThemeExtensionPoint;

impl ThemeExtensionPoint {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for ThemeExtensionPoint {
    fn key(&self) -> ContributionKey {
        THEMES_KEY
    }

    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &ExtensionInfo,
        data: JsonValue,
    ) -> joinerror::Result<()> {
        if !data.is_array() {
            joinerror::bail!("themes contribution must be an array");
        }

        let themes: Vec<ThemeContributionDecl> = serde_json::from_value(data)?;
        let items = themes
            .into_iter()
            .map(|entry| ThemeRegistryItem {
                id: entry.id,
                display_name: entry.label,
                mode: entry.mode,
                path: info.source.join(entry.path),
            })
            .collect();

        app_delegate
            .global::<GlobalThemeRegistry>()
            .register(items)
            .await;

        Ok(())
    }
}
