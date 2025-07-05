use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App, models::operations::ListColorThemesOutput, services::theme_service::ThemeService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn list_color_themes(&self) -> OperationResult<ListColorThemesOutput> {
        let theme_service = self.services.get::<ThemeService>();
        let themes = theme_service.themes().await?;

        Ok(ListColorThemesOutput(
            themes.into_iter().map(|(_, item)| item).cloned().collect(),
        ))
    }
}
