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

        // let themes = self.theme_service.themes().await?;

        // if let Some(descriptor) = themes.get(&input.id) {
        //     let css_content = {
        //         let mut reader = self
        //             .fs
        //             .open_file(
        //                 &self
        //                     .theme_service
        //                     .themes_dir
        //                     .join(descriptor.source.clone()),
        //             )
        //             .await?;

        //         let mut content = String::new();
        //         reader
        //             .read_to_string(&mut content)
        //             .join_err::<()>("failed to read theme file")?;

        //         content
        //     };

        //     Ok(GetColorThemeOutput { css_content })
        // } else {
        //     Err(Error::new::<()>(format!(
        //         "theme with id `{}` not found",
        //         input.id
        //     )))
        // }
    }
}
