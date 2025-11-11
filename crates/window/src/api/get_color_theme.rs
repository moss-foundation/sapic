use moss_applib::AppRuntime;

use crate::{
    models::operations::{GetColorThemeInput, GetColorThemeOutput},
    window::Window,
};

impl<R: AppRuntime> Window<R> {
    pub async fn get_color_theme(
        &self,
        _ctx: &R::AsyncContext,
        input: &GetColorThemeInput,
    ) -> joinerror::Result<GetColorThemeOutput> {
        let css_content = self.theme_service.read(&input.id).await?;

        Ok(GetColorThemeOutput { css_content })
    }
}
