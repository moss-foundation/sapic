use moss_applib::AppRuntime;
use sapic_ipc::contracts::theme::ListColorThemesOutput;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn list_color_themes(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListColorThemesOutput> {
        let themes = self.services.theme_service.themes().await;

        Ok(ListColorThemesOutput(
            themes.values().cloned().collect::<Vec<_>>(),
        ))
    }
}
