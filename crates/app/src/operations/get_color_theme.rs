use moss_api::contracts::theme::{GetColorThemeInput, GetColorThemeOutput};
use moss_applib::AppRuntime;

use crate::App;

impl<R: AppRuntime> App<R> {
    pub async fn get_color_theme(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetColorThemeInput,
    ) -> joinerror::Result<GetColorThemeOutput> {
        let css_content = self.services.theme_service.read(&input.id).await?;

        Ok(GetColorThemeOutput { css_content })
    }
}
