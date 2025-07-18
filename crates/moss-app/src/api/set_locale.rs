use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{app::App, models::operations::SetLocaleInput};

impl<R: AppRuntime> App<R> {
    pub async fn set_locale(
        &self,
        _ctx: &R::AsyncContext,
        input: &SetLocaleInput,
    ) -> OperationResult<()> {
        // TODO: this implementation is not good enough, we need revisit it, and refactor it
        let mut locale_lock = self.preferences.locale.write().await;
        *locale_lock = Some(input.locale_info.clone());

        Ok(())
    }
}
