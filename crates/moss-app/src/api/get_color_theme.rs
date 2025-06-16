use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    models::operations::{GetColorThemeInput, GetColorThemeOutput},
    services::theme_service::ThemeService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn get_color_theme(
        &self,
        input: &GetColorThemeInput,
    ) -> OperationResult<GetColorThemeOutput> {
        let theme_service = self.service::<ThemeService>();
        let themes = theme_service.themes().await?;

        if let Some(descriptor) = themes.get(&input.id) {
            let css_content = {
                let mut reader = self
                    .fs
                    .open_file(&theme_service.themes_dir.join(descriptor.source.clone()))
                    .await?;

                let mut content = String::new();
                reader.read_to_string(&mut content).map_err(|e| {
                    moss_common::api::OperationError::Internal(format!(
                        "failed to read theme file: {}",
                        e
                    ))
                })?;

                content
            };

            Ok(GetColorThemeOutput { css_content })
        } else {
            Err(moss_common::api::OperationError::NotFound(format!(
                "theme with id `{}` was not found",
                input.id
            )))
        }
    }
}
