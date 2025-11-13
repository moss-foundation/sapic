use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_base::theme::types::{ColorThemeInfo, primitives::ThemeId};
use sapic_system::theme::theme_service::ThemeService;
use std::{collections::HashMap, sync::Arc};

pub struct WelcomeColorThemeOps {
    color_theme_service: Arc<ThemeService>,
}

impl WelcomeColorThemeOps {
    pub fn new(color_theme_service: Arc<ThemeService>) -> Self {
        Self {
            color_theme_service,
        }
    }

    pub async fn read(&self, id: &ThemeId) -> joinerror::Result<String> {
        self.color_theme_service.read(id).await
    }

    pub async fn themes(&self) -> joinerror::Result<HashMap<ThemeId, ColorThemeInfo>> {
        Ok(self.color_theme_service.themes().await)
    }
}
