use moss_applib::AppRuntime;

use crate::{app::App, models::operations::SetLocaleInput};

// DEPRECATED
impl<R: AppRuntime> App<R> {
    pub async fn set_locale(
        &self,
        _ctx: &R::AsyncContext,
        input: &SetLocaleInput,
    ) -> joinerror::Result<()> {
        // TODO: this implementation is not good enough, we need revisit it, and refactor it
        // let mut locale_lock = self.preferences.locale.write().await;
        // *locale_lock = Some(input.locale_info.clone());

        Ok(())
    }
}
