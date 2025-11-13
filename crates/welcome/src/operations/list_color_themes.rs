use moss_api::contracts::theme::ListColorThemesOutput;
use moss_applib::AppRuntime;

use crate::WelcomeWindow;

impl<R: AppRuntime> WelcomeWindow<R> {
    pub async fn list_color_themes(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListColorThemesOutput> {
        let themes = self.color_theme_ops.themes().await?;

        Ok(ListColorThemesOutput(
            themes.values().cloned().collect::<Vec<_>>(),
        ))
    }
}
