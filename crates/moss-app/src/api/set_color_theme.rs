use moss_applib::AppRuntime;

use crate::{app::App, models::operations::SetColorThemeInput};

impl<R: AppRuntime> App<R> {
    pub async fn set_color_theme(
        &self,
        _ctx: &R::AsyncContext,
        input: &SetColorThemeInput,
    ) -> joinerror::Result<()> {
        // TODO: this implementation is not good enough, we need revisit it, and refactor it
        let mut theme_descriptor_lock = self.preferences.theme.write().await;
        *theme_descriptor_lock = Some(input.theme_info.clone());

        Ok(())
    }
}
