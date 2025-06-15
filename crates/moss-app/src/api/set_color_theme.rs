use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{app::App, models::operations::SetColorThemeInput};

impl<R: TauriRuntime> App<R> {
    pub async fn set_color_theme(&self, input: SetColorThemeInput) -> OperationResult<()> {
        // TODO: this implementation is not good enough, we need revisit it, and refactor it
        let mut theme_descriptor_lock = self.preferences.theme.write().await;
        *theme_descriptor_lock = Some(input.theme_info);

        Ok(())
    }
}
