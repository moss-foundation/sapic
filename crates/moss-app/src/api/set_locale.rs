use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{app::App, models::operations::SetLocaleInput};

impl<R: TauriRuntime> App<R> {
    pub async fn set_locale(&self, input: SetLocaleInput) -> OperationResult<()> {
        // TODO: this implementation is not good enough, we need revisit it, and refactor it
        let mut locale_lock = self.preferences.locale.write().await;
        *locale_lock = Some(input.locale_info);

        Ok(())
    }
}
