use moss_applib::AppRuntime;

use crate::{app::App, models::operations::ListLocalesOutput};

impl<R: AppRuntime> App<R> {
    pub async fn list_locales(
        &self,
        _ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListLocalesOutput> {
        let locales = self.locale_service.locales().await?;

        Ok(ListLocalesOutput(
            locales.into_iter().map(|(_, item)| item).cloned().collect(),
        ))
    }
}
