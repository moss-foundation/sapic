use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{app::App, models::operations::ListColorThemesOutput};

impl<R: AppRuntime> App<R> {
    pub async fn list_color_themes(
        &self,
        _ctx: &R::AsyncContext,
    ) -> OperationResult<ListColorThemesOutput> {
        let themes = self.theme_service.themes().await?;

        Ok(ListColorThemesOutput(
            themes.into_iter().map(|(_, item)| item).cloned().collect(),
        ))
    }
}
