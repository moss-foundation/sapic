use moss_applib::AppRuntime;

use crate::{
    app::App,
    models::operations::{GetColorThemeInput, GetColorThemeOutput},
};

impl<R: AppRuntime> App<R> {
    pub async fn get_color_theme(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetColorThemeInput,
    ) -> joinerror::Result<GetColorThemeOutput> {
        let css_content = self.theme_service.read(&input.id).await?;

        Ok(GetColorThemeOutput { css_content })
    }
}
